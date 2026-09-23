use std::{
    ffi::c_int,
};

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
