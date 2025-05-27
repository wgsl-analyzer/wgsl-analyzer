//! Read both stdout and stderr of child without deadlocks.
//!
//! <https://github.com/rust-lang/cargo/blob/905af549966f23a9288e9993a85d1249a5436556/crates/cargo-util/src/read2.rs>
//! <https://github.com/rust-lang/cargo/blob/58a961314437258065e23cb6316dfc121d96fb71/crates/cargo-util/src/process_builder.rs#L231>

use std::{
    io,
    process::{ChildStderr, ChildStdout, Command, Output, Stdio},
};

use crate::JodChild;

#[inline]
pub fn streaming_output(
    out: ChildStdout,
    error: ChildStderr,
    on_stdout_line: &mut dyn FnMut(&str),
    on_stderr_line: &mut dyn FnMut(&str),
    on_eof: &mut dyn FnMut(),
) -> io::Result<(Vec<u8>, Vec<u8>)> {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();

    implementation::read2(out, error, &mut |is_out, data, eof| {
        let index = if eof {
            data.len()
        } else {
            match data.iter().rposition(|&byte| byte == b'\n') {
                Some(i) => i + 1,
                None => return,
            }
        };
        {
            // scope for new_lines
            let new_lines = {
                let dst = if is_out { &mut stdout } else { &mut stderr };
                let start = dst.len();
                let data = data.drain(..index);
                dst.extend(data);
                &dst[start..]
            };
            for line in String::from_utf8_lossy(new_lines).lines() {
                if is_out {
                    on_stdout_line(line);
                } else {
                    on_stderr_line(line);
                }
            }
            if eof {
                on_eof();
            }
        }
    })?;
    Ok((stdout, stderr))
}

/// # Panics
///
/// Panics if `cmd` is not configured to have `stdout` and `stderr` as `piped`.
#[inline]
pub fn spawn_with_streaming_output(
    mut cmd: Command,
    on_stdout_line: &mut dyn FnMut(&str),
    on_stderr_line: &mut dyn FnMut(&str),
) -> io::Result<Output> {
    let cmd = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null());

    let mut child = JodChild(cmd.spawn()?);
    let (stdout, stderr) = streaming_output(
        child.stdout.take().unwrap(),
        child.stderr.take().unwrap(),
        on_stdout_line,
        on_stderr_line,
        &mut || (),
    )?;
    let status = child.wait()?;
    Ok(Output {
        status,
        stdout,
        stderr,
    })
}

#[cfg(unix)]
mod implementation {
    use std::{
        io::{self, prelude::*},
        mem,
        os::unix::prelude::*,
        process::{ChildStderr, ChildStdout},
    };

    /// Reads from both stdout and stderr pipes of a child process concurrently,
    /// using non-blocking I/O and `poll(2)` to multiplex between them.
    ///
    /// The provided `data` callback is invoked repeatedly with accumulated output
    /// until both pipes have reached EOF.
    pub(crate) fn read2(
        mut out_pipe: ChildStdout,
        mut error_pipe: ChildStderr,
        data: &mut dyn FnMut(bool, &mut Vec<u8>, bool),
    ) -> io::Result<()> {
        // SAFETY: We assume `as_raw_fd()` returns valid file descriptors.
        // Setting them to non-blocking mode is safe as long as they are valid.
        unsafe {
            libc::fcntl(out_pipe.as_raw_fd(), libc::F_SETFL, libc::O_NONBLOCK);
        }
        // SAFETY: We assume `as_raw_fd()` returns valid file descriptors.
        // Setting them to non-blocking mode is safe as long as they are valid.
        unsafe {
            libc::fcntl(error_pipe.as_raw_fd(), libc::F_SETFL, libc::O_NONBLOCK);
        }
        let mut out_done = false;
        let mut error_done = false;
        let mut out = Vec::new();
        let mut error = Vec::new();

        // SAFETY: `pollfd` is a plain old data type and zero initialization is valid.
        let mut fds: [libc::pollfd; 2] = unsafe { mem::zeroed() };
        fds[0].fd = out_pipe.as_raw_fd();
        fds[0].events = libc::POLLIN;
        fds[1].fd = error_pipe.as_raw_fd();
        fds[1].events = libc::POLLIN;
        let mut nfds = 2;
        let mut errfd = 1;

        while nfds > 0 {
            // SAFETY: `fds` points to two properly initialized pollfd structs.
            // `poll` will block until one of the fds becomes ready.
            let return_code = unsafe { libc::poll(fds.as_mut_ptr(), nfds, -1) };
            if return_code == -1 {
                let error = io::Error::last_os_error();
                if error.kind() == io::ErrorKind::Interrupted {
                    continue;
                }
                return Err(error);
            }

            // Handles partial reads and filtering out non-fatal WouldBlock errors
            let handle = |result: io::Result<_>| match result {
                Ok(_) => Ok(true),
                Err(error) => {
                    if error.kind() == io::ErrorKind::WouldBlock {
                        Ok(false)
                    } else {
                        Err(error)
                    }
                },
            };

            if !error_done && fds[errfd].revents != 0 && handle(error_pipe.read_to_end(&mut error))?
            {
                error_done = true;
                nfds -= 1;
            }
            data(false, &mut error, error_done);

            if !out_done && fds[0].revents != 0 && handle(out_pipe.read_to_end(&mut out))? {
                out_done = true;
                fds[0].fd = error_pipe.as_raw_fd(); // Switch to monitor stderr only
                errfd = 0;
                nfds -= 1;
            }
            data(true, &mut out, out_done);
        }

        Ok(())
    }
}

#[cfg(windows)]
mod implementation {
    use core::convert::TryInto as _;
    use std::{
        io,
        os::windows::prelude::*,
        process::{ChildStderr, ChildStdout},
        slice,
    };

    use miow::{
        Overlapped,
        iocp::{CompletionPort, CompletionStatus},
        pipe::NamedPipe,
    };
    use windows_sys::Win32::Foundation::ERROR_BROKEN_PIPE;

    struct Pipe<'buffer> {
        dst: &'buffer mut Vec<u8>,
        overlapped: Overlapped,
        inner: NamedPipe,
        done: bool,
    }

    pub(crate) fn read2(
        out_pipe: ChildStdout,
        error_pipe: ChildStderr,
        data: &mut dyn FnMut(bool, &mut Vec<u8>, bool),
    ) -> io::Result<()> {
        let mut out = Vec::new();
        let mut error = Vec::new();

        let port = CompletionPort::new(1)?;
        port.add_handle(0, &out_pipe)?;
        port.add_handle(1, &error_pipe)?;

        // SAFETY: TODO
        let mut out_pipe = unsafe { Pipe::new(out_pipe, &mut out) };
        // SAFETY: TODO
        let mut error_pipe = unsafe { Pipe::new(error_pipe, &mut error) };

        // SAFETY: TODO
        unsafe {
            out_pipe.read()?;
        }
        // SAFETY: TODO
        unsafe {
            error_pipe.read()?;
        }

        let mut status = [CompletionStatus::zero(), CompletionStatus::zero()];

        while !out_pipe.done || !error_pipe.done {
            for status in port.get_many(&mut status, None)? {
                if status.token() == 0 {
                    // SAFETY: TODO
                    unsafe {
                        out_pipe.complete(status);
                    }
                    data(true, out_pipe.dst, out_pipe.done);
                    // SAFETY: TODO
                    unsafe {
                        out_pipe.read()?;
                    }
                } else {
                    // SAFETY: TODO
                    unsafe {
                        error_pipe.complete(status);
                    }
                    data(false, error_pipe.dst, error_pipe.done);
                    // SAFETY: TODO
                    unsafe {
                        error_pipe.read()?;
                    }
                }
            }
        }

        Ok(())
    }

    impl<'buffer> Pipe<'buffer> {
        /// # Safety
        /// `p` must be a valid [`NamedPipe`].
        unsafe fn new<P: IntoRawHandle>(
            pipe: P,
            dst: &'buffer mut Vec<u8>,
        ) -> Self {
            // SAFETY: `p`is required to be a valid `NamedPipe`.
            let pipe = unsafe { NamedPipe::from_raw_handle(pipe.into_raw_handle()) };
            Pipe {
                dst,
                inner: pipe,
                overlapped: Overlapped::zero(),
                done: false,
            }
        }

        /// # Safety
        /// TODO
        unsafe fn read(&mut self) -> io::Result<()> {
            // SAFETY: TODO
            let dst = unsafe { slice_to_end(self.dst) };
            // SAFETY: TODO
            match unsafe { self.inner.read_overlapped(dst, self.overlapped.raw()) } {
                Ok(_) => Ok(()),
                Err(error) =>
                {
                    #[expect(clippy::cast_possible_wrap, reason = "this is known to be fine")]
                    #[expect(clippy::as_conversions, reason = "this is known to be fine")]
                    if error.raw_os_error() == Some(ERROR_BROKEN_PIPE as i32) {
                        self.done = true;
                        Ok(())
                    } else {
                        Err(error)
                    }
                },
            }
        }

        /// # Safety
        /// `self.dst` must have a capacity of at least `self.dst.len() + status.bytes_transferred()`.
        unsafe fn complete(
            &mut self,
            status: &CompletionStatus,
        ) {
            let new_length = self
                .dst
                .len()
                .checked_add(status.bytes_transferred().try_into().unwrap())
                .unwrap();
            // SAFETY: the responsibility lies on the callsite to make sure that the vec
            // has enough capacity to make this a valid operation.
            unsafe {
                self.dst.set_len(new_length);
            }
            if status.bytes_transferred() == 0 {
                self.done = true;
            }
        }
    }

    /// # Safety
    /// Seems safe enough
    unsafe fn slice_to_end(vector: &mut Vec<u8>) -> &mut [u8] {
        if vector.capacity() == 0 {
            vector.reserve(16);
        }
        if vector.capacity() == vector.len() {
            vector.reserve(1);
        }
        // SAFETY: invariants hold because this gets a pointer within the Vec's capacity
        let data = unsafe { vector.as_mut_ptr().add(vector.len()) };
        let length = vector.capacity() - vector.len();
        // SAFETY: invariants have been checked and tested
        unsafe { slice::from_raw_parts_mut(data, length) }
    }
}

#[cfg(target_arch = "wasm32")]
mod implementation {
    use std::{
        io,
        process::{ChildStderr, ChildStdout},
    };

    pub(crate) fn read2(
        _out_pipe: ChildStdout,
        _err_pipe: ChildStderr,
        _data: &mut dyn FnMut(bool, &mut Vec<u8>, bool),
    ) -> io::Result<()> {
        panic!("no processes on wasm")
    }
}
