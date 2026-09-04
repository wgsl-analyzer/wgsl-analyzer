//! Browser stdio bridge for `wasm32-unknown-emscripten`.
//!
//! This crate lets a language server keep using
//! [`lsp_server::Connection::stdio`], `std::io::stdin` and `std::io::stdout`
//! unchanged inside a Web Worker. Everything browser-specific happens below
//! Rust's `std::io` on libc level.
//!
//! The linker redirects four calls, `read`, `readv`, `write` and `writev`, so
//! `read` reaches `__wrap_read` while the original stays available as
//! `__real_read`. Only fd 0 and fd 1 are handled here. Everything else is
//! forwarded untouched, so the filesystem behaves as emscripten intends.
//!
//! ```text
//! JavaScript ──lsp_stdin_push──► queue ──► __wrap_read(0)  ──► std::io::stdin
//! JavaScript ◄──lsp_stdout_pop── queue ◄── __wrap_write(1) ◄── std::io::stdout
//! ```
//!
//! ## Why emscripten's own stdin cannot do this
//!
//! The build uses `WasmFS`, whose stdin is `StdinFile::read` in
//! `system/lib/wasmfs/special_files.cpp`:
//!
//! ```text
//! for (size_t i = 0; i < len; i++) {
//!   auto c = _wasmfs_stdin_get_char();
//!   if (c < 0) return i;
//!   buf[i] = c;
//! }
//! ```
//!
//! From this arise 3 issues:
//!
//! It asks for one byte at a time, up to whatever `len` the caller passed, and
//! nothing tells it a message has ended. A blocking implementation therefore
//! parks on the byte *after* the last one available, part way through a read,
//! before a single byte reaches Rust.
//!
//! A non-blocking implementation is worse. Reporting "nothing yet" makes the
//! loop return `i`, so an empty queue at the start of a read returns zero, and
//! zero from `read` is end of file. The server's reader would shut down quietly
//! rather than wait.
//!
//! `_wasmfs_stdin_get_char` is also not proxied, so it runs on whichever thread
//! called `read`, inside *that* worker's JavaScript context. Pthread workers
//! start with no arguments, so the `Module.stdin` installed on the main thread
//! does not exist there at all.
//!
//! Keeping the queues in wasm sidesteps these issues. They live in shared memory
//! rather than in one worker's JavaScript heap, so the host can fill stdin from
//! the runtime thread while the reader blocks on a [`Condvar`] on its own
//! thread, which for `lsp_server` is `LspServerReader`.
//!
//! Stdout has a smaller version of the same problem. `WasmFS` routes it through
//! `emscripten_out` and flushes on newlines, and an LSP frame ends in JSON with
//! no trailing newline, so message bodies would sit in the buffer until the
//! next header arrived.
//!
//! One consequence worth remembering: whoever calls `read` on fd 0 is the
//! thread that blocks.
//!
//! On targets other than emscripten this crate is nothing but a no-op
//! [`force_link`].
//!
//! [`lsp_server::Connection::stdio`]: https://docs.rs/lsp-server/latest/lsp_server/struct.Connection.html
//! [`Condvar`]: std::sync::Condvar

// `memory_atomic_notify` is the only way to wake a JavaScript
// `Atomics.waitAsync` waiter, and it is still unstable. The emscripten build
// already requires nightly because it rebuilds `std` with `-Zbuild-std`, so
// this costs nothing there, and the attribute is absent everywhere else.
#![cfg_attr(target_os = "emscripten", feature(stdarch_wasm_atomic_wait))]

#[cfg(target_os = "emscripten")]
mod imp;
mod queue;
