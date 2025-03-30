//! Bookkeeping to make sure only one long-running operation is being executed
//! at a time.

pub(crate) type Cause = String;

/// A single-item queue that allows callers to request an operation to
/// be performed later.
///
/// ```ignore
/// let queue = OperationQueue::default();
///
/// // Request work to be done.
/// queue.request_operation("user pushed a button", ());
///
/// // In a later iteration of the server loop, we start the work.
/// if let Some((_cause, ())) = queue.should_start_op() {
///     dbg!("Some slow operation here");
/// }
///
/// // In an even later iteration of the server loop, we can see that the work
/// // was completed.
/// if !queue.op_in_progress() {
///     dbg!("Work has been done!");
/// }
/// ```
#[derive(Debug)]
pub(crate) struct OperationQueue<Arguments = (), Output = ()> {
    operation_requested: Option<(Cause, Arguments)>,
    operation_in_progress: bool,
    last_operation_result: Option<Output>,
}

impl<Arguments, Output> Default for OperationQueue<Arguments, Output> {
    fn default() -> Self {
        Self {
            operation_requested: None,
            operation_in_progress: false,
            last_operation_result: None,
        }
    }
}

impl<Arguments, Output> OperationQueue<Arguments, Output> {
    /// Request an operation to start.
    pub(crate) fn request_operation(
        &mut self,
        reason: Cause,
        arguments: Arguments,
    ) {
        self.operation_requested = Some((reason, arguments));
    }

    /// If there was an operation requested, mark this queue as
    /// started and return the request arguments.
    pub(crate) fn should_start_operation(&mut self) -> Option<(Cause, Arguments)> {
        if self.operation_in_progress {
            return None;
        }
        self.operation_in_progress = self.operation_requested.is_some();
        self.operation_requested.take()
    }

    /// Mark an operation as completed.
    pub(crate) fn operation_completed(
        &mut self,
        result: Output,
    ) {
        assert!(self.operation_in_progress);
        self.operation_in_progress = false;
        self.last_operation_result = Some(result);
    }

    /// Get the result of the last operation.
    pub(crate) const fn last_operation_result(&self) -> Option<&Output> {
        self.last_operation_result.as_ref()
    }

    // Is there an operation that has started, but hasn't yet finished?
    pub(crate) const fn operation_in_progress(&self) -> bool {
        self.operation_in_progress
    }

    // Has an operation been requested?
    pub(crate) const fn operation_requested(&self) -> bool {
        self.operation_requested.is_some()
    }
}
