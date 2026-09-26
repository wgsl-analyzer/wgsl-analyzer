//! Browser transport for the LSP server replacing [`Connection::stdio`] for emscripten.
//!
//! The host provides [`lsp_next_message`] and [`lsp_send_message`] in an emscripten JS library,
//! which `build.rs` links in from the path in `WGSL_ANALYZER_JS_LIBRARY`. For wgsl-analyzer-web,
//! that is `js/wgsl-analyzer-web/emscripten/library.js`, which `cargo xtask build-web` sets.
//!
//! Both are called from pthreads, so the library has to mark them `__proxy: 'sync'` to run them
//! on the main runtime thread. `lsp_next_message` also has to be `__async: true`, so it can
//! resolve a promise to the message instead of returning it.
//!
//! This builds only for `wasm32-unknown-emscripten`, with the flags and the rebuilt `std` that
//! `.cargo/config.toml` describes under `[target.wasm32-unknown-emscripten]`.

use std::{
    ffi::{CStr, c_char, c_void},
    io, thread,
};

use crossbeam_channel::{Receiver, Sender};
use lsp_server::{Connection, Message};
use serde::Serialize;

// Provided by the host's JS library, see the module documentation.
unsafe extern "C" {
    /// Blocks until the host has a message, and returns its NUL-terminated body, allocated with
    /// `malloc`. Null if that allocation failed.
    safe fn lsp_next_message() -> *mut c_char;
    /// Hands one NUL-terminated message body to the host, which copies it before returning.
    fn lsp_send_message(body: *const c_char);
    fn free(pointer: *mut c_void);
}

fn parse(body: &CStr) -> io::Result<Message> {
    let body = body.to_str().map_err(io::Error::other)?;
    Ok(serde_json::from_str(body)?)
}

/// Forwards the host's messages to `sender`, until the host sends `exit`.
fn receive(sender: &Sender<Message>) {
    loop {
        let body = lsp_next_message();
        if body.is_null() {
            tracing::error!("cannot allocate the next LSP message");
            std::process::abort();
        }
        // SAFETY: `body` is a NUL-terminated string, per `lsp_next_message`'s contract.
        let message = parse(unsafe { CStr::from_ptr(body) });
        // SAFETY: `body` was allocated with `malloc`, and is not used anymore.
        unsafe {
            free(body.cast());
        }

        let message = match message {
            Ok(message) => message,
            Err(error) => {
                tracing::error!("dropping a malformed LSP message: {error}");
                continue;
            },
        };
        let is_exit = matches!(&message, Message::Notification(notification) if notification.method == "exit");
        if sender.send(message).is_err() || is_exit {
            return;
        }
    }
}

/// Hands each message to the host, one call per message.
///
/// Returns once the server has dropped its sender and every message has been delivered.
fn send(receiver: &Receiver<Message>) -> io::Result<()> {
    #[derive(Serialize)]
    struct JsonRpc<'message> {
        jsonrpc: &'static str,
        #[serde(flatten)]
        message: &'message Message,
    }

    let mut body = Vec::new();
    for message in receiver {
        body.clear();
        serde_json::to_writer(
            &mut body,
            &JsonRpc {
                jsonrpc: "2.0",
                message: &message,
            },
        )?;
        // serde_json escapes U+0000, so this is the only NUL.
        body.push(0);
        // SAFETY: `body` is NUL-terminated and outlives the call.
        unsafe {
            lsp_send_message(body.as_ptr().cast());
        }
    }
    Ok(())
}

/// Counterpart of [`lsp_server::IoThreads`], which is not public.
pub struct IoThreads {
    writer: thread::JoinHandle<io::Result<()>>,
}

impl IoThreads {
    pub fn join(self) -> io::Result<()> {
        match self.writer.join() {
            Ok(result) => result,
            Err(payload) => std::panic::resume_unwind(payload),
        }
    }
}

/// Counterpart of [`Connection::stdio`], reading from [`lsp_next_message`] and writing to
/// [`lsp_send_message`].
pub fn connection() -> (Connection, IoThreads) {
    let (transport_socket, language_server_socket) = Connection::memory();

    // Not joined: it may be waiting in `lsp_next_message` when the main loop ends, and exiting
    // ends it.
    thread::Builder::new()
        .name("LspServerReceiver".to_owned())
        .spawn(move || receive(&transport_socket.sender))
        .unwrap();
    let writer = thread::Builder::new()
        .name("LspServerSender".to_owned())
        .spawn(move || send(&transport_socket.receiver))
        .unwrap();

    (language_server_socket, IoThreads { writer })
}
