//! Browser transport for the LSP server. These are copies of [`lsp_server`] functionality
//! replacing `stdin` and `stdout` with custom implementations for emscripten that hand messages
//! directly to the javascript side.
//!
//! # Compiling
//!
//! This builds only for `wasm32-unknown-emscripten`, and only with following `rustflags`
//! defined in `config.toml`:
//!
//! ```toml
//! # wasm threads: connection() spawns a reader, a writer and a dropper
//! -Ctarget-feature=+atomics
//! -Clink-arg=-pthread
//! # links libc++abi, which provides __cpp_exception / _Unwind_* for panic=unwind
//! -Clink-arg=-sDEFAULT_TO_CXX
//! # main() runs on a pthread, so spawning a thread from it cannot deadlock
//! -Clink-arg=-sPROXY_TO_PTHREAD=1
//! # resolves the lsp_js_read / lsp_js_write externs declared below
//! -Clink-arg=--js-library=crates/wgsl-analyzer/src/bin/emscripten-io.js
//! # installs the Module.lspReadInto / lspWriteFrom that those two call into
//! -Clink-arg=--pre-js=crates/wgsl-analyzer/src/bin/emscripten-io-pre.js
//! # FS seeds the workspace; callMain is what starts the server, given INVOKE_RUN=0
//! -Clink-arg=-sEXPORTED_RUNTIME_METHODS=FS,callMain
//! ```
//!
//! `std` has to be rebuilt alongside it, because the shipped one for this target is
//! `singlethread: true` (no wasm `atomics` feature) and so cannot link with `-pthread`:
//!
//! ```bash
//! cargo +nightly build -Zbuild-std=std,panic_unwind --target wasm32-unknown-emscripten
//! ```

use std::{
    ffi::c_int,
    io::{self, BufReader, Read, Write},
    str, thread,
};

use crossbeam_channel::bounded;
use lsp_server::{Connection, Message};
use tracing::debug;

/// Working buffer size on either side of the JS <-> Rust boundary, 64 KiB.
///
/// Every crossing is a hop to another thread, so this is sized to keep a typical LSP message to
/// one of them in each direction rather than to save memory. It is the reader's buffer size, and
/// the capacity the writer starts with and shrinks back to between messages.
const IO_CAPACITY: usize = 1 << 16;

// Each signature matches the js-library function of the same name in `emscripten-io.js`. Both copy
// through the pointer before returning, so neither retains it.
unsafe extern "C" {
    /// Fill `destination` with up to `capacity` bytes, waiting for a message if none are queued.
    ///
    /// Returns the number of bytes written, `0` once the host has closed the input and the queue is
    /// drained, or a negative value if the host failed.
    fn lsp_js_read(
        destination: *mut u8,
        capacity: usize,
    ) -> c_int;

    /// Hand one complete message body of `length` bytes at `source` to the host.
    ///
    /// Returns the number of bytes accepted, or a negative value if the host failed.
    fn lsp_js_write(
        source: *const u8,
        length: usize,
    ) -> c_int;
}

/// Accumulates one LSP frame and hands the host its body on [`Write::flush`].
struct EmscriptenIoWrite {
    frame: Vec<u8>,
    base_capacity: usize,
}

impl EmscriptenIoWrite {
    fn with_base_capacity(base_capacity: usize) -> Self {
        Self {
            frame: Vec::with_capacity(base_capacity),
            base_capacity,
        }
    }
}

#[expect(clippy::renamed_function_params, reason = "abbreviations")]
impl Write for EmscriptenIoWrite {
    fn write(
        &mut self,
        bytes: &[u8],
    ) -> std::io::Result<usize> {
        self.frame.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    /// Hands the accumulated frame to the host, as one call carrying one message body.
    fn flush(&mut self) -> std::io::Result<()> {
        if self.frame.is_empty() {
            return Ok(());
        }

        let result = send_body(&self.frame);

        // Cleared whatever happened: a failed flush must not leave a partial frame for the next
        // message to be appended to.
        self.frame.clear();
        // One outsized response must not pin its peak allocation for the rest of the session.
        self.frame.shrink_to(self.base_capacity);

        result
    }
}

/// Checks that `frame` holds exactly one complete message, then hands the host its body, so no
/// decoding has to happen on the JavaScript side.
fn send_body(frame: &[u8]) -> io::Result<()> {
    // The header Message::write puts in front of every message.
    const CONTENT_LENGTH: &[u8] = b"Content-Length: ";

    // Separator between a frame's header block and its body.
    const HEADER_END: &[u8] = b"\r\n\r\n";

    let header_end = frame
        .windows(HEADER_END.len())
        .position(|window| window == HEADER_END)
        .ok_or_else(|| io::Error::other("the LSP frame has no header terminator"))?;
    let (header, rest) = frame.split_at(header_end);
    let body = rest
        .get(HEADER_END.len()..)
        .ok_or_else(|| io::Error::other("the LSP frame has no body"))?;

    let digits = header
        .strip_prefix(CONTENT_LENGTH)
        .ok_or_else(|| io::Error::other("the LSP frame has no Content-Length header"))?;
    let declared: usize = str::from_utf8(digits)
        .map_err(|_error| io::Error::other("the Content-Length header is not UTF-8"))?
        .parse()
        .map_err(|_error| io::Error::other("the Content-Length header is not a number"))?;
    if declared != body.len() {
        return Err(io::Error::other(
            "the LSP frame was flushed before its body was complete",
        ));
    }

    // SAFETY: `body` is a live slice of `body.len()` readable bytes. The call is proxied
    // synchronously, so the host has finished copying out of it by the time this returns.
    let count = unsafe { lsp_js_write(body.as_ptr(), body.len()) };

    let accepted = usize::try_from(count)
        .map_err(|_error| io::Error::other("the host failed to write LSP output"))?;

    if accepted != body.len() {
        return Err(io::Error::other(
            "the host accepted only part of an LSP message",
        ));
    }
    Ok(())
}

struct EmscriptenIoRead;

#[expect(clippy::renamed_function_params, reason = "abbreviations")]
impl Read for EmscriptenIoRead {
    fn read(
        &mut self,
        destination: &mut [u8],
    ) -> io::Result<usize> {
        if destination.is_empty() {
            return Ok(0);
        }

        // SAFETY: `destination` is uniquely borrowed and writable for `destination.len()` bytes,
        // which is the most the host is allowed to write.
        let count = unsafe { lsp_js_read(destination.as_mut_ptr(), destination.len()) };

        let count = usize::try_from(count)
            .map_err(|_error| io::Error::other("the host failed to read LSP input"))?;
        if count > destination.len() {
            // Overrunning the buffer would break the invariants of whatever wraps this, so treat a
            // host that reports more than it was offered as a failure rather than trusting it.
            return Err(io::Error::other("the host read past the end of the buffer"));
        }
        Ok(count)
    }
}

/// Copy [`lsp_server::IoThreads`], which is not public.
pub struct IoThreads {
    reader: thread::JoinHandle<io::Result<()>>,
    writer: thread::JoinHandle<io::Result<()>>,
    dropper: thread::JoinHandle<()>,
}

impl IoThreads {
    pub fn join(self) -> io::Result<()> {
        match self.reader.join() {
            Ok(result) => result?,
            Err(error) => std::panic::panic_any(error),
        }
        match self.dropper.join() {
            Ok(()) => (),
            Err(error) => {
                std::panic::panic_any(error);
            },
        }
        match self.writer.join() {
            Ok(result) => result,
            Err(error) => {
                std::panic::panic_any(error);
            },
        }
    }
}

/// Copy of [`lsp_server::stdio::stdio_transport`] replacing `stdin` and `stdout` with custom
/// implementations for emscripten that hand messages directly to the javascript side.
pub fn connection() -> (Connection, IoThreads) {
    let (drop_sender, drop_receiver) = bounded::<Message>(0);
    let (writer_sender, writer_receiver) = bounded::<Message>(0);
    let writer = thread::Builder::new()
        .name("LspServerWriter".to_owned())
        .spawn(move || {
            // here EmscriptenIoWrite is used instead of stdout()
            let mut stdout = EmscriptenIoWrite::with_base_capacity(IO_CAPACITY);
            writer_receiver.into_iter().try_for_each(|message| {
                let result = message.write(&mut stdout);
                let _sent = drop_sender.send(message);
                result
            })
        })
        .unwrap();
    let dropper = thread::Builder::new()
        .name("LspMessageDropper".to_owned())
        .spawn(move || drop_receiver.into_iter().for_each(drop))
        .unwrap();
    let (reader_sender, reader_receiver) = bounded::<Message>(0);
    let reader = thread::Builder::new()
        .name("LspServerReader".to_owned())
        .spawn(move || {
            // here EmscriptenIoRead is used instead of stdin()
            let mut stdin = BufReader::with_capacity(IO_CAPACITY, EmscriptenIoRead);
            while let Some(message) = Message::read(&mut stdin)? {
                let is_exit = matches!(&message, Message::Notification(notification) if notification.method == "exit");

                debug!("sending message {message:#?}");
                if let Err(error) = reader_sender.send(message) {
                    return Err(io::Error::other(error));
                }

                if is_exit {
                    break;
                }
            }
            Ok(())
        })
        .unwrap();

    let threads = IoThreads {
        reader,
        writer,
        dropper,
    };

    (
        Connection {
            sender: writer_sender,
            receiver: reader_receiver,
        },
        threads,
    )
}
