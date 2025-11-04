//! Like `std::time::Instant`, but also measures memory & CPU cycles.

#![cfg_attr(
    all(target_os = "linux", not(target_env = "ohos")),
    expect(clippy::print_stderr, reason = "this is a debugging utility")
)]

use std::{
    fmt,
    time::{Duration, Instant},
};

use crate::MemoryUsage;

pub struct StopWatch {
    time: Instant,
    #[cfg(all(target_os = "linux", not(target_env = "ohos")))]
    counter: Option<perf_event::Counter>,
    memory: MemoryUsage,
}

pub struct StopWatchSpan {
    pub time: Duration,
    pub instructions: Option<u64>,
    pub memory: MemoryUsage,
}

impl StopWatch {
    #[must_use]
    pub fn start() -> Self {
        #[cfg(all(target_os = "linux", not(target_env = "ohos")))]
        let counter = {
            // When debugging wgsl-analyzer using rr, the performance-related syscalls cause it to abort.
            // We allow disabling performance by setting the environment variable `WA_DISABLE_PERFORMANCE`.

            use std::sync::OnceLock;
            static PERFORMANCE_ENABLED: OnceLock<bool> = OnceLock::new();

            if *PERFORMANCE_ENABLED
                .get_or_init(|| std::env::var_os("WA_DISABLE_PERFORMANCE").is_none())
            {
                let mut counter = perf_event::Builder::new()
                    .build()
                    .map_err(|error| eprintln!("Failed to create performance counter: {error}"))
                    .ok();
                if let Some(counter) = &mut counter
                    && let Err(error) = counter.enable()
                {
                    eprintln!("Failed to start performance counter: {error}");
                }
                counter
            } else {
                None
            }
        };
        let memory = MemoryUsage::now();
        let time = Instant::now();
        Self {
            time,
            #[cfg(all(target_os = "linux", not(target_env = "ohos")))]
            counter,
            memory,
        }
    }

    #[cfg_attr(
        not(all(target_os = "linux", not(target_env = "ohos"))),
        expect(clippy::needless_pass_by_ref_mut, reason = "platform differences")
    )]
    pub fn elapsed(&mut self) -> StopWatchSpan {
        let time = self.time.elapsed();

        #[cfg(all(target_os = "linux", not(target_env = "ohos")))]
        let instructions = self.counter.as_mut().and_then(|counter| {
            counter
                .read()
                .map_err(|error| eprintln!("Failed to read performance counter: {error}"))
                .ok()
        });
        #[cfg(all(target_os = "linux", target_env = "ohos"))]
        let instructions = None;
        #[cfg(not(target_os = "linux"))]
        let instructions = None;

        let memory = MemoryUsage::now() - self.memory;
        StopWatchSpan {
            time,
            instructions,
            memory,
        }
    }
}

impl fmt::Display for StopWatchSpan {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "{:.2}", self.time.as_millis())?;

        if let Some(instructions) = self.instructions {
            let (value, suffix) = if instructions > 10_000_000_000 {
                (instructions.saturating_div(1_000_000_000), "g")
            } else if instructions > 10_000_000 {
                (instructions.saturating_div(1_000_000), "m")
            } else if instructions > 10_000 {
                (instructions.saturating_div(1_000), "k")
            } else {
                (instructions, "")
            };
            write!(formatter, ", {value}{suffix}instr")?;
        }
        write!(formatter, ", {}", self.memory)?;
        Ok(())
    }
}
