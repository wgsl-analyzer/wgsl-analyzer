//! The emscripten half of the stdio bridge.
//!
//! Split out so the crate is an empty shell on every other target.

use std::{
    collections::VecDeque,
    ffi::{c_int, c_void},
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
