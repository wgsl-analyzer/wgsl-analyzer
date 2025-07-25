//! A utility module for working with threads that automatically joins threads upon drop
//! and abstracts over operating system quality of service (`QoS`) APIs
//! through the concept of a “thread intent”.
//!
//! The intent of a thread is frozen at thread creation time,
//! i.e. there is no API to change the intent of a thread once it has been spawned.
//!
//! As a system, wgsl-analyzer should have the property that
//! old manual scheduling APIs are replaced entirely by `QoS`.
//! To maintain this invariant, we panic when it is clear that
//! old scheduling APIs have been used.
//!
//! Moreover, we also want to ensure that every thread has an intent set explicitly
//! to force a decision about its importance to the system.
//! Thus, [`ThreadIntent`] has no default value
//! and every entry point to creating a thread requires a [`ThreadIntent`] upfront.

use std::fmt;

mod intent;
mod pool;

pub use intent::ThreadIntent;
pub use pool::Pool;

/// # Panics
///
/// Panics if failed to spawn the thread.
pub fn spawn<Function, T>(
    intent: ThreadIntent,
    function: Function,
) -> JoinHandle<T>
where
    Function: (FnOnce() -> T) + Send + 'static,
    T: Send + 'static,
{
    Builder::new(intent)
        .spawn(function)
        .expect("failed to spawn thread")
}

pub struct Builder {
    intent: ThreadIntent,
    inner: jod_thread::Builder,
    allow_leak: bool,
}

impl Builder {
    #[must_use]
    pub fn new(intent: ThreadIntent) -> Self {
        Self {
            intent,
            inner: jod_thread::Builder::new(),
            allow_leak: false,
        }
    }

    #[must_use]
    pub fn name(
        self,
        name: String,
    ) -> Self {
        Self {
            inner: self.inner.name(name),
            ..self
        }
    }

    #[must_use]
    pub fn stack_size(
        self,
        size: usize,
    ) -> Self {
        Self {
            inner: self.inner.stack_size(size),
            ..self
        }
    }

    #[must_use]
    pub fn allow_leak(
        self,
        allow_leak: bool,
    ) -> Self {
        Self { allow_leak, ..self }
    }

    pub fn spawn<Function, T>(
        self,
        function: Function,
    ) -> std::io::Result<JoinHandle<T>>
    where
        Function: (FnOnce() -> T) + Send + 'static,
        T: Send + 'static,
    {
        let inner_handle = self.inner.spawn(move || {
            self.intent.apply_to_current_thread();
            function()
        })?;

        Ok(JoinHandle {
            inner: Some(inner_handle),
            allow_leak: self.allow_leak,
        })
    }
}

pub struct JoinHandle<T = ()> {
    // `inner` is an `Option` so that we can
    // take ownership of the contained `JoinHandle`.
    inner: Option<jod_thread::JoinHandle<T>>,
    allow_leak: bool,
}

impl<T> JoinHandle<T> {
    #[must_use]
    /// # Panics
    ///
    /// If there is no job
    pub fn join(mut self) -> T {
        self.inner.take().unwrap().join()
    }
}

impl<T> Drop for JoinHandle<T> {
    fn drop(&mut self) {
        if !self.allow_leak {
            return;
        }
        if let Some(join_handle) = self.inner.take() {
            join_handle.detach();
        }
    }
}

impl<T> fmt::Debug for JoinHandle<T> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.pad("JoinHandle { .. }")
    }
}
