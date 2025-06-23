//! Like `std::time::Instant`, but also measures memory & CPU cycles.

#![expect(clippy::print_stderr, reason = "this is a debugging utility")]

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
    pub fn start() -> Self {
        #[cfg(all(target_os = "linux", not(target_env = "ohos")))]
        let counter = {
            // When debugging rust-analyzer using rr, the perf-related syscalls cause it to abort.
            // We allow disabling perf by setting the env var `WA_DISABLE_PERF`.

            use std::sync::OnceLock;
            static PERF_ENABLED: OnceLock<bool> = OnceLock::new();

            if *PERF_ENABLED.get_or_init(|| std::env::var_os("WA_DISABLE_PERF").is_none()) {
                let mut counter = perf_event::Builder::new()
                    .build()
                    .map_err(|error| eprintln!("Failed to create perf counter: {error}"))
                    .ok();
                if let Some(counter) = &mut counter
                    && let Err(error) = counter.enable()
                {
                    eprintln!("Failed to start perf counter: {error}");
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

    pub fn elapsed(&mut self) -> StopWatchSpan {
        let time = self.time.elapsed();

        #[cfg(all(target_os = "linux", not(target_env = "ohos")))]
        let instructions = self.counter.as_mut().and_then(|counter| {
            counter
                .read()
                .map_err(|error| eprintln!("Failed to read perf counter: {error}"))
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
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{:.2}", self.time.as_millis())?;

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
            write!(f, ", {value}{suffix}instr")?;
        }
        write!(f, ", {}", self.memory)?;
        Ok(())
    }
}
