//! Browser transport for the LSP server replacing [`Connection::stdio`] for emscripten.
//!
//! The host pushes each message body in through [`lsp_push_message`], and receives the server's
//! output through the function it registers with [`lsp_set_on_message`].
//!
//! This builds only for `wasm32-unknown-emscripten`, with the flags and the rebuilt `std` that
//! `.cargo/config.toml` describes under `[target.wasm32-unknown-emscripten]`.

use std::{
    ffi::{CStr, c_char, c_void},
    io,
    sync::{LazyLock, OnceLock},
    thread,
};

use crossbeam_channel::{Receiver, Sender, unbounded};
use lsp_server::{Connection, Message};
use serde::Serialize;

/// `em_proxying_queue` from `<emscripten/proxying.h>`.
#[repr(C)]
struct ProxyingQueue {
    _opaque: [u8; 0],
}

/// A function the host added with `addFunction(…, "vp")` on the main runtime thread.
///
/// It receives one NUL-terminated message body, valid only during the call, and may only
/// be called on that thread.
type OnMessage = unsafe extern "C" fn(*mut c_void);

// Provided by emscripten's system library, `<emscripten/proxying.h>` and
// `<emscripten/threading.h>`.
unsafe extern "C" {
    safe fn em_proxying_queue_create() -> *mut ProxyingQueue;
    safe fn emscripten_main_runtime_thread_id() -> *mut c_void;
    /// Runs `func(arg)` on `target_thread` and returns once it has returned, or `false` if the
    /// task could not be queued.
    fn emscripten_proxy_sync(
        queue: *mut ProxyingQueue,
        target_thread: *mut c_void,
        func: OnMessage,
        arg: *mut c_void,
    ) -> bool;
}

/// Messages from the host, waiting for the main loop.
static INPUT: LazyLock<(Sender<Message>, Receiver<Message>)> = LazyLock::new(unbounded);

/// Set once, by [`lsp_set_on_message`].
static ON_MESSAGE: OnceLock<OnMessage> = OnceLock::new();

/// Queues one complete message body for the server.
///
/// # Safety
///
/// `message` points to a NUL-terminated string that stays valid for the duration of the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lsp_push_message(message: *const c_char) {
    // SAFETY: guaranteed by the caller.
    let body = unsafe { CStr::from_ptr(message) };
    let message = match parse(body) {
        Ok(message) => message,
        Err(error) => {
            tracing::error!("dropping a malformed LSP message: {error}");
            return;
        },
    };
    // the input receiver is static, so it is never dropped
    INPUT.0.send(message).unwrap();
}

/// Registers the function that receives the server's messages. The host calls this once, before
/// `callMain`.
///
/// # Safety
///
/// `on_message` is a `"vp"` function added with `addFunction` on the main runtime thread.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lsp_set_on_message(on_message: Option<OnMessage>) {
    ON_MESSAGE
        .set(on_message.expect("_lsp_set_on_message needs a function"))
        .expect("_lsp_set_on_message called twice");
}

fn parse(body: &CStr) -> io::Result<Message> {
    let body = body.to_str().map_err(io::Error::other)?;
    Ok(serde_json::from_str(body)?)
}

/// Hands each message to `on_message`, one call per message.
///
/// Returns once the server has dropped its sender and every message has been delivered.
fn write(
    on_message: OnMessage,
    receiver: &Receiver<Message>,
) -> io::Result<()> {
    #[derive(Serialize)]
    struct JsonRpc<'message> {
        jsonrpc: &'static str,
        #[serde(flatten)]
        message: &'message Message,
    }

    // Private, so its tasks run only from the host's event loop, never inside a futex wait there.
    // Never destroyed, because destroying frees it at once, even under a task still in flight.
    let queue = em_proxying_queue_create();
    if queue.is_null() {
        return Err(io::Error::other("cannot create proxying queue"));
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

        // SAFETY: `queue` is never destroyed. `on_message` may run on the main runtime thread, per
        // `lsp_set_on_message`'s contract, and `body` outlives the call, which returns only once
        // `on_message` has.
        let delivered = unsafe {
            emscripten_proxy_sync(
                queue,
                emscripten_main_runtime_thread_id(),
                on_message,
                body.as_mut_ptr().cast(),
            )
        };
        if !delivered {
            return Err(io::Error::other("failed to queue proxied on_message task"));
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

/// Counterpart of [`Connection::stdio`], reading from [`lsp_push_message`] and writing to the
/// function registered with [`lsp_set_on_message`].
pub fn connection() -> (Connection, IoThreads) {
    let on_message = *ON_MESSAGE
        .get()
        .expect("the host must call `_lsp_set_on_message` before `callMain`");
    // Unbounded, so a busy host never stalls the main loop.
    let (sender, receiver) = unbounded::<Message>();
    let writer = thread::Builder::new()
        .name("LspServerWriter".to_owned())
        .spawn(move || write(on_message, &receiver))
        .unwrap();

    let connection = Connection {
        sender,
        receiver: INPUT.1.clone(),
    };
    (connection, IoThreads { writer })
}
