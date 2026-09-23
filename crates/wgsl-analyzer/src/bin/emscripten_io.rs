use std::{
    ffi::c_int,
    io::{self, BufReader, Read, Write},
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
