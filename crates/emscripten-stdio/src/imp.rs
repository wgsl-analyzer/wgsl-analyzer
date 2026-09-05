//! The emscripten half of the stdio bridge.
//!
//! Split out so the crate is an empty shell on every other target.

use std::{
    collections::VecDeque,
    ffi::{c_int, c_void},
    hint::black_box,
    sync::{
        Condvar, Mutex, MutexGuard,
        atomic::{AtomicI32, Ordering},
    },
};

use crate::queue::drain_into;

// ---------------------------------------------------------------------------
// fd 0
// ---------------------------------------------------------------------------

/// Bytes queued for fd 0, plus the end-of-input flag.
struct StdinState {
    /// The raw stream `std::io::stdin` sees, `Content-Length` framing and all,
    /// so nothing above this layer has to change.
    bytes: VecDeque<u8>,
    /// Set by [`lsp_stdin_close`]. Reads report end of input once the queued
    /// bytes run out.
    closed: bool,
}

/// A blocking byte pipe, filled from JavaScript and drained by `read`.
struct StdinPipe {
    state: Mutex<StdinState>,
    ready: Condvar,
}

static STDIN: StdinPipe = StdinPipe {
    state: Mutex::new(StdinState {
        bytes: VecDeque::new(),
        closed: false,
    }),
    ready: Condvar::new(),
};

impl StdinPipe {
    /// Queue bytes for the reader. Returns `false` if stdin is already closed.
    fn push(
        &self,
        bytes: &[u8],
    ) -> bool {
        let mut state = lock(&self.state);
        if state.closed {
            return false;
        }
        state.bytes.extend(bytes);

        // Release before waking, or the reader wakes straight into contention
        // for the lock we are still holding.
        drop(state);
        // notify_all rather than notify_one: waking a single reader is only
        // correct while exactly one thread reads stdin, and that is not a
        // property worth depending on.
        self.ready.notify_all();
        true
    }

    /// Report end of input once the queued bytes run out.
    fn close(&self) {
        lock(&self.state).closed = true;
        self.ready.notify_all();
    }

    /// Wait for bytes, then let `fill` take what it wants. Returns whatever
    /// `fill` reports, or 0 at end of input.
    ///
    /// Both read wrappers come through here, so the POSIX rule lives in one
    /// place: a read waits at most once and returns as soon as it has any data,
    /// even when the caller offered room for more.
    ///
    /// `fill` runs with the queue locked, so it must not call back into this
    /// pipe or touch the filesystem.
    fn read_with(
        &self,
        fill: impl FnOnce(&mut VecDeque<u8>) -> usize,
    ) -> usize {
        let mut state = lock(&self.state);
        while state.bytes.is_empty() && !state.closed {
            state = wait(&self.ready, state);
        }
        fill(&mut state.bytes)
    }
}

// ---------------------------------------------------------------------------
// fd 1
// ---------------------------------------------------------------------------

/// A non-blocking queue, filled by `write` and drained from JavaScript.
struct StdoutPipe {
    queue: Mutex<VecDeque<u8>>,
    /// Counts writes. The host waits on this address for a change instead of
    /// polling; see [`lsp_stdout_signal_ptr`].
    writes: AtomicI32,
}

static STDOUT: StdoutPipe = StdoutPipe {
    queue: Mutex::new(VecDeque::new()),
    writes: AtomicI32::new(0),
};

impl StdoutPipe {
    /// Append `total` bytes through `fill`, then wake the host once.
    ///
    /// This never waits for the host to drain. The caller is `lsp_server`'s
    /// writer thread, and stalling it backs up through a zero-capacity channel
    /// and wedges the server, so the queue is allowed to grow instead.
    fn append(
        &self,
        total: usize,
        fill: impl FnOnce(&mut VecDeque<u8>),
    ) {
        if total == 0 {
            return;
        }
        let mut queue = lock(&self.queue);
        queue.reserve(total);
        fill(&mut queue);
        drop(queue);
        self.wake();
    }

    /// Move up to `dst.len()` bytes out. Never blocks.
    fn pop_into(
        &self,
        dst: &mut [u8],
    ) -> usize {
        drain_into(&mut lock(&self.queue), dst)
    }

    /// Publish that output is waiting, waking an `Atomics.waitAsync` waiter.
    fn wake(&self) {
        self.writes.fetch_add(1, Ordering::Release);

        // SAFETY: the address of a `static AtomicI32` is valid, four-byte
        // aligned and lives for the whole program. Waking every waiter is
        // correct because the host keeps at most one.
        unsafe {
            core::arch::wasm32::memory_atomic_notify(
                (&raw const self.writes).cast_mut().cast::<i32>(),
                u32::MAX,
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

/// Take a lock, ignoring poisoning.
///
/// Callers arrive through `extern "C"` functions, where unwinding is undefined
/// behavior. Poisoning buys nothing either: a panic while holding one of these
/// locks cannot leave a byte queue in a state worth protecting.
fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// [`lock`] for condition variable waits.
fn wait<'guard, T>(
    condvar: &Condvar,
    guard: MutexGuard<'guard, T>,
) -> MutexGuard<'guard, T> {
    condvar
        .wait(guard)
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Set `errno` and return the POSIX failure value.
///
/// These wrappers are C entry points, so a caller reads `errno` after a `-1`.
/// Rust's own `cvt` does exactly that, and a stale `EINTR` left in the slot
/// would turn one bad argument into an endless retry.
fn fail(error: c_int) -> isize {
    // SAFETY: `__errno_location` returns this thread's errno slot.
    unsafe {
        *libc::__errno_location() = error;
    }
    -1
}

/// POSIX return value for a transfer of `count` bytes.
///
/// No slice is longer than `isize::MAX`, so this never saturates in practice.
fn transferred(count: usize) -> isize {
    isize::try_from(count).unwrap_or(isize::MAX)
}

/// View a C buffer as a slice.
///
/// A zero length yields an empty slice without touching `ptr`, matching what
/// POSIX allows callers to pass.
///
/// # Safety
///
/// `ptr` must point to `len` readable bytes.
unsafe fn as_slice<'bytes>(
    ptr: *const u8,
    len: usize,
) -> Result<&'bytes [u8], c_int> {
    if len == 0 {
        return Ok(&[]);
    }
    if ptr.is_null() {
        return Err(libc::EFAULT);
    }
    if len > isize::MAX as usize {
        return Err(libc::EINVAL);
    }
    // SAFETY: the caller promises `len` readable bytes at `ptr`.
    Ok(unsafe { std::slice::from_raw_parts(ptr, len) })
}

/// [`as_slice`] for a writable buffer.
///
/// # Safety
///
/// `ptr` must point to `len` writable bytes.
unsafe fn as_slice_mut<'bytes>(
    ptr: *mut u8,
    len: usize,
) -> Result<&'bytes mut [u8], c_int> {
    if len == 0 {
        return Ok(&mut []);
    }
    if ptr.is_null() {
        return Err(libc::EFAULT);
    }
    if len > isize::MAX as usize {
        return Err(libc::EINVAL);
    }
    // SAFETY: the caller promises `len` writable bytes at `ptr`.
    Ok(unsafe { std::slice::from_raw_parts_mut(ptr, len) })
}

/// Validate an iovec array and total its lengths.
///
/// Both vectored wrappers check up front rather than as they go, so a bad
/// argument can never surface as a short transfer or, worse, as end of input.
///
/// # Safety
///
/// `iov` must point to `iovcnt` readable `iovec` values.
unsafe fn checked_iovecs<'iov>(
    iov: *const libc::iovec,
    iovcnt: c_int,
) -> Result<(&'iov [libc::iovec], usize), c_int> {
    let count = usize::try_from(iovcnt).map_err(|_| libc::EINVAL)?;
    if count == 0 {
        return Ok((&[], 0));
    }
    if iov.is_null() {
        return Err(libc::EFAULT);
    }
    // SAFETY: the caller promises `iovcnt` readable iovec values at `iov`.
    let iovecs = unsafe { std::slice::from_raw_parts(iov, count) };

    let mut total: usize = 0;
    for entry in iovecs {
        if entry.iov_len != 0 && entry.iov_base.is_null() {
            return Err(libc::EFAULT);
        }
        total = match total.checked_add(entry.iov_len) {
            Some(sum) if sum <= isize::MAX as usize => sum,
            _ => return Err(libc::EINVAL),
        };
    }
    Ok((iovecs, total))
}

// ---------------------------------------------------------------------------
// Exported to JavaScript
// ---------------------------------------------------------------------------

/// See [`crate::force_link`].
pub(crate) fn force_link() {
    black_box(lsp_stdout_signal_ptr());
}

/// Queue bytes for the server to read, already LSP-framed by the host.
///
/// Copies before returning, so the caller may reuse its buffer immediately.
/// Returns `0` on success, `-1` for an invalid pointer, `-2` if stdin is
/// closed.
///
/// # Safety
///
/// `ptr` must point to `len` readable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lsp_stdin_push(
    ptr: *const u8,
    len: usize,
) -> c_int {
    // SAFETY: the caller promises `len` readable bytes at `ptr`.
    let Ok(bytes) = (unsafe { as_slice(ptr, len) }) else {
        return -1;
    };
    if STDIN.push(bytes) { 0 } else { -2 }
}

/// Close stdin, so a blocked read finishes instead of hanging at shutdown.
///
/// Queued bytes stay readable; the reader sees end of input once it drains
/// them.
#[unsafe(no_mangle)]
pub extern "C" fn lsp_stdin_close() {
    STDIN.close();
}

/// Address of the counter the host waits on for output.
///
/// This is a `static`, so the address is stable for the life of the program and
/// aligned for a JavaScript `Int32Array`.
#[unsafe(no_mangle)]
pub extern "C" fn lsp_stdout_signal_ptr() -> *const i32 {
    (&raw const STDOUT.writes).cast::<i32>()
}

/// Move up to `capacity` bytes of the server's output into `dst`.
///
/// Returns the number copied, or `-1` for an invalid pointer. Zero means
/// nothing is pending. Never blocks, because the host calls this from the
/// runtime thread's event loop.
///
/// # Safety
///
/// `dst` must point to `capacity` writable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lsp_stdout_pop(
    dst: *mut u8,
    capacity: usize,
) -> isize {
    // SAFETY: the caller promises `capacity` writable bytes at `dst`.
    let Ok(destination) = (unsafe { as_slice_mut(dst, capacity) }) else {
        return -1;
    };
    transferred(STDOUT.pop_into(destination))
}

// ---------------------------------------------------------------------------
// The wrappers, and the libc entry points they displace
// ---------------------------------------------------------------------------

// SAFETY: each signature matches the libc function the linker binds it to.
// `--wrap=read` renames the original `read` to `__real_read`, and so on for the
// other three.
unsafe extern "C" {
    #[link_name = "__real_read"]
    fn real_read(
        fd: c_int,
        buf: *mut c_void,
        count: usize,
    ) -> isize;

    #[link_name = "__real_readv"]
    fn real_readv(
        fd: c_int,
        iov: *const libc::iovec,
        iovcnt: c_int,
    ) -> isize;

    #[link_name = "__real_write"]
    fn real_write(
        fd: c_int,
        buf: *const c_void,
        count: usize,
    ) -> isize;

    #[link_name = "__real_writev"]
    fn real_writev(
        fd: c_int,
        iov: *const libc::iovec,
        iovcnt: c_int,
    ) -> isize;
}

/// Intercepts libc `read`. Handles fd 0 and forwards every other descriptor.
///
/// # Safety
///
/// Same contract as `read(2)`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_read(
    fd: c_int,
    buf: *mut c_void,
    count: usize,
) -> isize {
    if fd != libc::STDIN_FILENO {
        // SAFETY: forwarded unchanged, with the exact libc ABI.
        return unsafe { real_read(fd, buf, count) };
    }
    // SAFETY: the caller promises `count` writable bytes at `buf`.
    let destination = match unsafe { as_slice_mut(buf.cast::<u8>(), count) } {
        Ok(destination) => destination,
        Err(error) => return fail(error),
    };
    transferred(STDIN.read_with(|queue| drain_into(queue, destination)))
}

/// Intercepts libc `readv`.
///
/// Wrapping this is not optional: `is_read_vectored()` is true on emscripten,
/// so Rust's buffered readers really do take this path, and wrapping only
/// `read` would lose data.
///
/// # Safety
///
/// Same contract as `readv(2)`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_readv(
    fd: c_int,
    iov: *const libc::iovec,
    iovcnt: c_int,
) -> isize {
    if fd != libc::STDIN_FILENO {
        // SAFETY: forwarded unchanged, with the exact libc ABI.
        return unsafe { real_readv(fd, iov, iovcnt) };
    }
    // SAFETY: the caller promises `iovcnt` readable iovec values at `iov`.
    let (iovecs, capacity) = match unsafe { checked_iovecs(iov, iovcnt) } {
        Ok(checked) => checked,
        Err(error) => return fail(error),
    };
    // No room means no wait. Blocking on a zero-byte request would hang for
    // good, because no amount of incoming data can ever satisfy it.
    if capacity == 0 {
        return 0;
    }

    transferred(STDIN.read_with(|queue| {
        let mut total = 0;
        for entry in iovecs {
            if queue.is_empty() {
                break;
            }
            // SAFETY: checked_iovecs rejected every null base with a non-zero
            // length, so this buffer is writable for `iov_len` bytes.
            let Ok(destination) =
                (unsafe { as_slice_mut(entry.iov_base.cast::<u8>(), entry.iov_len) })
            else {
                break;
            };
            total += drain_into(queue, destination);
        }
        total
    }))
}

/// Intercepts libc `write`. Captures fd 1 and forwards every other descriptor.
///
/// fd 2 is left alone on purpose, so tracing, panics and other diagnostics keep
/// reaching the browser console. That matches the native convention: stdout
/// carries the protocol, stderr carries everything else. It also means any
/// stray write to stdout corrupts the LSP stream.
///
/// # Safety
///
/// Same contract as `write(2)`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_write(
    fd: c_int,
    buf: *const c_void,
    count: usize,
) -> isize {
    if fd != libc::STDOUT_FILENO {
        // SAFETY: forwarded unchanged, with the exact libc ABI.
        return unsafe { real_write(fd, buf, count) };
    }
    // SAFETY: the caller promises `count` readable bytes at `buf`.
    let bytes = match unsafe { as_slice(buf.cast::<u8>(), count) } {
        Ok(bytes) => bytes,
        Err(error) => return fail(error),
    };
    STDOUT.append(bytes.len(), |queue| queue.extend(bytes));
    transferred(bytes.len())
}

/// Intercepts libc `writev`.
///
/// Rust's `Stdout` is a `LineWriter` and `is_write_vectored()` is true on
/// emscripten, so this is the path most LSP responses take.
///
/// # Safety
///
/// Same contract as `writev(2)`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __wrap_writev(
    fd: c_int,
    iov: *const libc::iovec,
    iovcnt: c_int,
) -> isize {
    if fd != libc::STDOUT_FILENO {
        // SAFETY: forwarded unchanged, with the exact libc ABI.
        return unsafe { real_writev(fd, iov, iovcnt) };
    }
    // SAFETY: the caller promises `iovcnt` readable iovec values at `iov`.
    let (iovecs, total) = match unsafe { checked_iovecs(iov, iovcnt) } {
        Ok(checked) => checked,
        Err(error) => return fail(error),
    };

    STDOUT.append(total, |queue| {
        for entry in iovecs {
            // SAFETY: checked_iovecs rejected every null base with a non-zero
            // length, so this buffer is readable for `iov_len` bytes.
            if let Ok(bytes) = unsafe { as_slice(entry.iov_base.cast::<u8>(), entry.iov_len) } {
                queue.extend(bytes);
            }
        }
    });
    transferred(total)
}
