//! Like [`std::time::Instant`], but for memory.
//!
//! Measures the total size of all currently allocated objects.
use std::fmt;

use cfg_if::cfg_if;

#[derive(Copy, Clone)]
pub struct MemoryUsage {
    pub allocated: Bytes,
}

impl fmt::Display for MemoryUsage {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.allocated.fmt(formatter)
    }
}

impl std::ops::Sub for MemoryUsage {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self {
        Self {
            allocated: self.allocated - rhs.allocated,
        }
    }
}

impl MemoryUsage {
    /// Get the current memory usage.
    ///
    /// # Panics
    ///
    /// Panics if using jemalloc targeting msvc and advance fails.
    #[must_use]
    pub fn now() -> Self {
        cfg_if! {
            if #[cfg(all(feature = "jemalloc", not(target_env = "msvc")))] {
                jemalloc_ctl::epoch::advance().unwrap();
                Self {
                    allocated: Bytes(i32::try_from(jemalloc_ctl::stats::allocated::read().unwrap()).unwrap().try_into().unwrap()),
                }
            } else if #[cfg(all(target_os = "linux", target_env = "gnu"))] {
                memusage_linux()
            } else if #[cfg(windows)] {
                // There doesn't seem to be an API for determining heap usage, so we try to
                // approximate that by using the Commit Charge value.

                use windows_sys::Win32::System::{Threading::GetCurrentProcess, ProcessStatus::{PROCESS_MEMORY_COUNTERS, GetProcessMemoryInfo}};
                use std::mem::MaybeUninit;

                // SAFETY: Windows API safety is undocumented.
                let proc = unsafe { GetCurrentProcess() };
                let mut mem_counters = MaybeUninit::uninit();
                let cb = size_of::<PROCESS_MEMORY_COUNTERS>();
                // SAFETY: Windows API safety is undocumented.
                let ret = unsafe { GetProcessMemoryInfo(proc, mem_counters.as_mut_ptr(), cb as u32) };
                assert!(ret != 0);

                // SAFETY: mem_counters is initialized by GetProcessMemoryInfo.
                let usage = unsafe { mem_counters.assume_init().PagefileUsage };
                Self { allocated: Bytes(usage as isize) }
            } else {
                Self { allocated: Bytes(0) }
            }
        }
    }
}

#[cfg(all(target_os = "linux", target_env = "gnu", not(feature = "jemalloc")))]
#[expect(clippy::as_conversions, reason = "no better way")]
fn memusage_linux() -> MemoryUsage {
    // Linux/glibc has 2 APIs for allocator introspection that we can use: mallinfo and mallinfo2.
    // mallinfo uses `int` fields and cannot handle memory usage exceeding 2 GB.
    // mallinfo2 is very recent, so its presence needs to be detected at runtime.
    // Both are abysmally slow.

    use std::sync::atomic::{AtomicUsize, Ordering};

    static MALLINFO2: AtomicUsize = AtomicUsize::new(1);

    let mut mallinfo2 = MALLINFO2.load(Ordering::Relaxed);
    if mallinfo2 == 1 {
        // SAFETY: undocumented safety
        mallinfo2 = unsafe { libc::dlsym(libc::RTLD_DEFAULT, c"mallinfo2".as_ptr()) } as usize;
        // NB: races don't matter here, since they'll always store the same value
        MALLINFO2.store(mallinfo2, Ordering::Relaxed);
    }

    if mallinfo2 == 0 {
        // mallinfo2 does not exist, use mallinfo.
        // SAFETY: undocumented safety
        let allocation = isize::try_from(unsafe { libc::mallinfo() }.uordblks).unwrap();
        MemoryUsage {
            allocated: Bytes(allocation),
        }
    } else {
        // SAFETY: tested and correct
        let mallinfo2: extern "C" fn() -> libc::mallinfo2 =
            unsafe { std::mem::transmute(mallinfo2) };
        let allocation = isize::try_from(mallinfo2().uordblks).unwrap();
        MemoryUsage {
            allocated: Bytes(allocation),
        }
    }
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Bytes(isize);

impl Bytes {
    #[must_use]
    pub const fn new(bytes: isize) -> Self {
        Self(bytes)
    }
}

impl Bytes {
    #[must_use]
    #[expect(
        clippy::integer_division_remainder_used,
        reason = "not a security issue"
    )]
    #[expect(clippy::integer_division, reason = "precision loss is acceptable")]
    pub const fn megabytes(self) -> isize {
        self.0 / 0x0400 / 0x0400
    }
}

impl fmt::Display for Bytes {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let bytes = self.0;
        let mut value = bytes;
        let mut suffix = "b";
        if value.abs() > 0x1000 {
            value /= 0x0400;
            suffix = "kb";
            if value.abs() > 0x1000 {
                value /= 0x0400;
                suffix = "mb";
            }
        }
        formatter.pad(&format!("{value}{suffix}"))
    }
}

impl std::ops::AddAssign<usize> for Bytes {
    fn add_assign(
        &mut self,
        rhs: usize,
    ) {
        self.0 = self.0.checked_add_unsigned(rhs).unwrap();
    }
}

impl std::ops::Sub for Bytes {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self {
        Self(self.0.checked_sub(rhs.0).unwrap())
    }
}
