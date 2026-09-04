//! The emscripten half of the stdio bridge.
//!
//! Split out so the crate is an empty shell on every other target.

use std::{
    ffi::{c_int, c_void},
    sync::{
        Condvar, Mutex, MutexGuard,
    },
};

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
