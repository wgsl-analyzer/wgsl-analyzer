//! <https://github.com/gperftools/gperftools>

use std::{
    ffi::CString,
    os::raw::c_char,
    sync::atomic::{AtomicUsize, Ordering},
};

use paths::AbsPath;

#[link(name = "profiler")]
unsafe extern "C" {
    fn ProfilerStart(fname: *const c_char) -> i32;
    fn ProfilerStop();
}

const OFF: usize = 0;
const ON: usize = 1;
const PENDING: usize = 2;

fn transition(
    current: usize,
    new: usize,
) -> bool {
    static STATE: AtomicUsize = AtomicUsize::new(OFF);

    STATE
        .compare_exchange(current, new, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
}

pub(crate) fn start(path: &AbsPath) {
    assert!(transition(OFF, PENDING), "profiler already started");
    let path = CString::new(path.to_string()).unwrap();
    // SAFETY: TODO
    let code = unsafe { ProfilerStart(path.as_ptr()) } != 0;
    assert!(code, "profiler failed to start");
    assert!(transition(PENDING, ON));
}

pub(crate) fn stop() {
    assert!(transition(ON, PENDING), "profiler is not started");
    // SAFETY: TODO
    unsafe { ProfilerStop() }
    assert!(transition(PENDING, OFF));
}
