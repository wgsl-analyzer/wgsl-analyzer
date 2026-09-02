//! [`Pool`] implements a basic custom thread pool
//! inspired by the [`threadpool` crate](http://docs.rs/threadpool).
//! When you spawn a task you specify a thread intent
//! so the pool can schedule it to run on a thread with that intent.
//! wgsl-analyzer uses this to prioritize work based on latency requirements.
//!
//! The thread pool is implemented entirely using
//! the threading utilities in [`crate::thread`].

use std::{
    marker::PhantomData,
    panic::{self, UnwindSafe},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use crossbeam_channel::{Receiver, Sender};
use crossbeam_utils::sync::WaitGroup;

use crate::thread::{Builder, JoinHandle, ThreadIntent};

pub struct Pool {
    // `_handles` is never read: the field is present
    // only for its `Drop` impl.

    // The worker threads exit once the channel closes;
    // make sure to keep `job_sender` above `handles`
    // so that the channel is actually closed
    // before we join the worker threads!
    job_sender: Sender<Job>,
    _handles: Box<[JoinHandle]>,
    extant_tasks: Arc<AtomicUsize>,
}

struct Job {
    requested_intent: ThreadIntent,
    function: Box<dyn FnOnce() + Send + UnwindSafe + 'static>,
}

impl Pool {
    /// # Panics
    ///
    /// Panics if job panics.
    #[must_use]
    pub fn new(threads: usize) -> Self {
        const INITIAL_INTENT: ThreadIntent = ThreadIntent::Worker;

        let (job_sender, job_receiver) = crossbeam_channel::unbounded();
        let extant_tasks = Arc::new(AtomicUsize::new(0));

        let mut handles = Vec::with_capacity(threads);
        for index in 0..threads {
            let handle = Builder::new(INITIAL_INTENT, format!("Worker{index}"))
                .allow_leak(true)
                .spawn({
                    let extant_tasks = Arc::clone(&extant_tasks);
                    let job_receiver: Receiver<Job> = job_receiver.clone();
                    move || {
                        let mut current_intent = INITIAL_INTENT;
                        for job in job_receiver {
                            if job.requested_intent != current_intent {
                                job.requested_intent.apply_to_current_thread();
                                current_intent = job.requested_intent;
                            }
                            // discard the panic, we should have logged the backtrace already
                            drop(panic::catch_unwind(job.function));
                            extant_tasks.fetch_sub(1, Ordering::SeqCst);
                        }
                    }
                })
                .expect("failed to spawn thread");

            handles.push(handle);
        }

        Self {
            job_sender,
            _handles: handles.into_boxed_slice(),
            extant_tasks,
        }
    }

    /// # Panics
    ///
    /// Panics if job panics.
    pub fn spawn<Function>(
        &self,
        intent: ThreadIntent,
        function: Function,
    ) where
        Function: FnOnce() + Send + UnwindSafe + 'static,
    {
        #[expect(clippy::semicolon_if_nothing_returned, reason = "thin wrapper")]
        let boxed_function = Box::new(move || {
            if cfg!(debug_assertions) {
                intent.assert_is_used_on_current_thread();
            }
            function()
        });

        let job = Job {
            requested_intent: intent,
            function: boxed_function,
        };
        self.extant_tasks.fetch_add(1, Ordering::SeqCst);
        self.job_sender.send(job).unwrap();
    }

    pub fn scoped<'pool, 'scope, Function, Result>(
        &'pool self,
        function: Function,
    ) -> Result
    where
        Function: FnOnce(&Scope<'pool, 'scope>) -> Result,
    {
        let wait_group = WaitGroup::new();
        let scope = Scope {
            pool: self,
            wg: wait_group,
            _marker: PhantomData,
        };
        let result = function(&scope);
        scope.wg.wait();
        result
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.extant_tasks.load(Ordering::SeqCst)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct Scope<'pool, 'scope> {
    pool: &'pool Pool,
    wg: WaitGroup,
    _marker: PhantomData<fn(&'scope ()) -> &'scope ()>,
}

impl<'scope> Scope<'_, 'scope> {
    pub fn spawn<F>(
        &self,
        intent: ThreadIntent,
        function: F,
    ) where
        F: 'scope + FnOnce() + Send + UnwindSafe,
    {
        let wg = self.wg.clone();
        let boxed_function = Box::new(move || {
            if cfg!(debug_assertions) {
                intent.assert_is_used_on_current_thread();
            }
            function();
            drop(wg);
        });

        let job = Job {
            requested_intent: intent,
            // SAFETY: leaking is inherently safe
            function: unsafe {
                std::mem::transmute::<
                    Box<dyn 'scope + FnOnce() + Send + UnwindSafe>,
                    Box<dyn 'static + FnOnce() + Send + UnwindSafe>,
                >(boxed_function)
            },
        };
        self.pool.extant_tasks.fetch_add(1, Ordering::SeqCst);
        self.pool.job_sender.send(job).unwrap();
    }
}
