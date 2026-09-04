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
