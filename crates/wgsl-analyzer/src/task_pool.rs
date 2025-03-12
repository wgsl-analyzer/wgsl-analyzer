//! A thin wrapper around [`stdx::thread::Pool`] which threads a sender through spawned jobs.
//! It is used in [`crate::global_state::GlobalState`] throughout the main loop.

use std::panic::UnwindSafe;

use crossbeam_channel::Sender;
use stdx::thread::{Pool, ThreadIntent};

use crate::main_loop::QueuedTask;

pub(crate) struct TaskPool<T> {
    sender: Sender<T>,
    pool: Pool,
}

impl<T> TaskPool<T> {
    pub(crate) fn new_with_threads(
        sender: Sender<T>,
        threads: usize,
    ) -> Self {
        Self {
            sender,
            pool: Pool::new(threads),
        }
    }

    pub(crate) fn spawn<F>(
        &self,
        intent: ThreadIntent,
        task: F,
    ) where
        F: FnOnce() -> T + Send + UnwindSafe + 'static,
        T: Send + 'static,
    {
        self.pool.spawn(intent, {
            let sender = self.sender.clone();
            move || sender.send(task()).unwrap()
        });
    }

    pub(crate) fn spawn_with_sender<F>(
        &self,
        intent: ThreadIntent,
        task: F,
    ) where
        F: FnOnce(Sender<T>) + Send + UnwindSafe + 'static,
        T: Send + 'static,
    {
        self.pool.spawn(intent, {
            let sender = self.sender.clone();
            move || task(sender)
        });
    }

    pub(crate) fn length(&self) -> usize {
        self.pool.length()
    }
}

/// `TaskQueue`, like its name suggests, queues tasks.
///
/// This should only be used if a task must run after [`GlobalState::process_changes`]
/// has been called.
pub(crate) struct TaskQueue {
    pub(crate) sender: crossbeam_channel::Sender<QueuedTask>,
    pub(crate) receiver: crossbeam_channel::Receiver<QueuedTask>,
}
