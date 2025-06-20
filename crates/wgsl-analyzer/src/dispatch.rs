//! See [`RequestDispatcher`].

use std::{fmt, panic, thread};

use fmt::Debug;
use lsp_server::{ExtractError, Response, ResponseError};
use salsa::Cancelled;
use serde::{Serialize, de::DeserializeOwned};
use stdx::thread::ThreadIntent;

use crate::{
    LspError, Result,
    global_state::{GlobalState, GlobalStateSnapshot},
    main_loop::Task,
};
use crate::{lsp::utilities::is_cancelled, version::version};

/// A visitor for routing a raw JSON request to an appropriate handler function.
///
/// Most requests are read-only and async and are handled on the threadpool
/// (`on` method).
///
/// Some read-only requests are latency sensitive, and are immediately handled
/// on the main loop thread (`on_sync`). These are typically typing-related
/// requests.
///
/// Some requests modify the state, and are run on the main thread to get
/// `&mut` (`on_sync_mut`).
///
/// Read-only requests are wrapped into `catch_unwind` -- they do not modify the
/// state, so it is OK to recover from their failures.
pub(crate) struct RequestDispatcher<'state> {
    pub(crate) request: Option<lsp_server::Request>,
    pub(crate) global_state: &'state mut GlobalState,
}

impl<'global_state> RequestDispatcher<'global_state> {
    pub(crate) const fn new(
        request: Option<lsp_server::Request>,
        global_state: &'global_state mut GlobalState,
    ) -> Self {
        Self {
            request,
            global_state,
        }
    }

    /// Dispatches the request onto the current thread, given full access to
    /// mutable global state. Unlike all other methods here, this one is not
    /// guarded by `catch_unwind`, so, please, do not make bugs :-)
    pub(crate) fn on_sync_mut<R>(
        &mut self,
        function: fn(&mut GlobalState, R::Params) -> anyhow::Result<R::Result>,
    ) -> &mut Self
    where
        R: lsp_types::request::Request,
        R::Params: DeserializeOwned + panic::UnwindSafe + fmt::Debug,
        R::Result: Serialize,
    {
        let Some((request, parameters, panic_context)) = self.parse::<R>() else {
            return self;
        };
        let _guard =
            tracing::info_span!("request", method = ?request.method, "request_id" = ?request.id)
                .entered();
        tracing::debug!(?parameters);
        let result = {
            let _pctx = stdx::panic_context::enter(panic_context);
            function(self.global_state, parameters)
        };
        if let Ok(response) = result_to_response::<R>(request.id, result) {
            self.global_state.respond(response);
        }

        self
    }

    /// Dispatches the request onto the current thread.
    pub(crate) fn on_sync<R>(
        &mut self,
        f: fn(GlobalStateSnapshot, R::Params) -> anyhow::Result<R::Result>,
    ) -> &mut Self
    where
        R: lsp_types::request::Request,
        R::Params: DeserializeOwned + panic::UnwindSafe + fmt::Debug,
        R::Result: Serialize,
    {
        let (req, params, panic_context) = match self.parse::<R>() {
            Some(it) => it,
            None => return self,
        };
        let _guard =
            tracing::info_span!("request", method = ?req.method, "request_id" = ?req.id).entered();
        tracing::debug!(?params);
        let global_state_snapshot = self.global_state.snapshot();

        let result = panic::catch_unwind(move || {
            let _pctx = stdx::panic_context::enter(panic_context);
            f(global_state_snapshot, params)
        });

        if let Ok(response) = thread_result_to_response::<R>(req.id, result) {
            self.global_state.respond(response);
        }

        self
    }

    /// Dispatches a non-latency-sensitive request onto the thread pool. When the VFS is marked not
    /// ready this will return a default constructed [`R::Result`].
    pub(crate) fn on<const ALLOW_RETRYING: bool, R>(
        &mut self,
        function: fn(GlobalStateSnapshot, R::Params) -> anyhow::Result<R::Result>,
    ) -> &mut Self
    where
        R: lsp_types::request::Request<
                Params: DeserializeOwned + panic::UnwindSafe + Send + fmt::Debug,
                Result: Serialize + Default,
            > + 'static,
    {
        if !self.global_state.vfs_done {
            if let Some(lsp_server::Request { id, .. }) =
                self.request.take_if(|it| it.method == R::METHOD)
            {
                self.global_state
                    .respond(lsp_server::Response::new_ok(id, R::Result::default()));
            }
            return self;
        }
        self.on_with_thread_intent::<false, ALLOW_RETRYING, R>(
            ThreadIntent::Worker,
            function,
            Self::content_modified_error,
        )
    }

    /// Dispatches a non-latency-sensitive request onto the thread pool. When the VFS is marked not
    /// ready this will return a `default` constructed [`R::Result`].
    pub(crate) fn on_with_vfs_default<R>(
        &mut self,
        function: fn(GlobalStateSnapshot, R::Params) -> anyhow::Result<R::Result>,
        default: impl FnOnce() -> R::Result,
        on_cancelled: fn() -> ResponseError,
    ) -> &mut Self
    where
        R: lsp_types::request::Request<
                Params: DeserializeOwned + panic::UnwindSafe + Send + fmt::Debug,
                Result: Serialize,
            > + 'static,
    {
        if !self.global_state.vfs_done {
            if let Some(lsp_server::Request { id, .. }) =
                self.request.take_if(|it| it.method == R::METHOD)
            {
                self.global_state
                    .respond(lsp_server::Response::new_ok(id, default()));
            }
            return self;
        }
        self.on_with_thread_intent::<false, false, R>(ThreadIntent::Worker, function, on_cancelled)
    }

    /// Formatting requests should never block on waiting a for task thread to open up, editors will wait
    /// on the response and a late formatting update might mess with the document and user.
    /// We can't run this on the main thread though as we invoke rustfmt which may take arbitrary time to complete!
    pub(crate) fn on_fmt_thread<R>(
        &mut self,
        function: fn(GlobalStateSnapshot, R::Params) -> anyhow::Result<R::Result>,
    ) -> &mut Self
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + panic::UnwindSafe + Send + fmt::Debug,
        R::Result: Serialize,
    {
        self.on_with_thread_intent::<true, false, R>(
            ThreadIntent::LatencySensitive,
            function,
            Self::content_modified_error,
        )
    }

    pub(crate) fn finish(&mut self) {
        if let Some(request) = self.request.take() {
            tracing::error!("Unknown request: {:?}", request);
            #[expect(clippy::as_conversions, reason = "valid according to JSON RPC")]
            let response = lsp_server::Response::new_err(
                request.id,
                lsp_server::ErrorCode::MethodNotFound as i32,
                "unknown request".to_owned(),
            );
            self.global_state.respond(response);
        }
    }

    fn on_with_thread_intent<const WGSLFMT: bool, const ALLOW_RETRYING: bool, R>(
        &mut self,
        intent: ThreadIntent,
        function: fn(GlobalStateSnapshot, R::Params) -> anyhow::Result<R::Result>,
        on_cancelled: fn() -> ResponseError,
    ) -> &mut Self
    where
        R: lsp_types::request::Request + 'static,
        R::Params: DeserializeOwned + panic::UnwindSafe + Send + fmt::Debug,
        R::Result: Serialize,
    {
        let Some((request, parameters, panic_context)) = self.parse::<R>() else {
            return self;
        };
        let _guard =
            tracing::info_span!("request", method = ?request.method, "request_id" = ?request.id)
                .entered();
        tracing::debug!(?parameters);

        let world = self.global_state.snapshot();
        if WGSLFMT {
            &mut self.global_state.fmt_pool.handle
        } else {
            &mut self.global_state.task_pool.handle
        }
        .spawn(intent, move || {
            let result = panic::catch_unwind(move || {
                let _pctx = stdx::panic_context::enter(panic_context);
                function(world, parameters)
            });
            match thread_result_to_response::<R>(request.id.clone(), result) {
                Ok(response) => Task::Response(response),
                Err(_cancelled) if ALLOW_RETRYING => Task::Retry(request),
                Err(_cancelled) => {
                    let error = on_cancelled();
                    Task::Response(Response {
                        id: request.id,
                        result: None,
                        error: Some(error),
                    })
                },
            }
        });

        self
    }

    fn parse<R>(&mut self) -> Option<(lsp_server::Request, R::Params, String)>
    where
        R: lsp_types::request::Request,
        R::Params: DeserializeOwned + fmt::Debug,
    {
        let request = self.request.take_if(|it| it.method == R::METHOD)?;
        let result = crate::from_json(R::METHOD, &request.params);
        match result {
            Ok(parameters) => {
                let panic_context = format!(
                    "\nversion: {}\nrequest: {} {parameters:#?}",
                    version(),
                    R::METHOD
                );
                Some((request, parameters, panic_context))
            },
            Err(error) => {
                #[expect(clippy::as_conversions, reason = "valid according to JSON RPC")]
                let response = lsp_server::Response::new_err(
                    request.id,
                    lsp_server::ErrorCode::InvalidParams as i32,
                    error.to_string(),
                );
                self.global_state.respond(response);
                None
            },
        }
    }

    fn content_modified_error() -> ResponseError {
        #[expect(clippy::as_conversions, reason = "valid according to JSON RPC")]
        ResponseError {
            code: lsp_server::ErrorCode::ContentModified as i32,
            message: "content modified".to_owned(),
            data: None,
        }
    }
}

#[derive(Debug)]
enum HandlerCancelledError {
    PropagatedPanic,
    Inner(Cancelled),
}

impl std::error::Error for HandlerCancelledError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PropagatedPanic => None,
            Self::Inner(cancelled) => Some(cancelled),
        }
    }
}

impl fmt::Display for HandlerCancelledError {
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait method")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "Cancelled")
    }
}

fn thread_result_to_response<R>(
    id: lsp_server::RequestId,
    result: thread::Result<anyhow::Result<R::Result>>,
) -> Result<lsp_server::Response, HandlerCancelledError>
where
    R: lsp_types::request::Request,
    R::Params: DeserializeOwned,
    R::Result: Serialize,
{
    match result {
        Ok(result) => result_to_response::<R>(id, result),
        Err(panic) => {
            let mut message = "server panicked".to_owned();

            let panic_message = panic
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic.downcast_ref::<&str>().copied());

            if let Some(panic_message) = panic_message {
                message.push_str(": ");
                message.push_str(panic_message);
            }
            #[expect(clippy::as_conversions, reason = "valid according to JSON RPC")]
            Ok(lsp_server::Response::new_err(
                id,
                lsp_server::ErrorCode::InternalError as i32,
                message,
            ))
        },
    }
}

fn result_to_response<R>(
    id: lsp_server::RequestId,
    result: anyhow::Result<R::Result>,
) -> Result<lsp_server::Response, HandlerCancelledError>
where
    R: lsp_types::request::Request,
    R::Params: DeserializeOwned,
    R::Result: Serialize,
{
    let result = match result {
        Ok(response) => lsp_server::Response::new_ok(id, &response),
        Err(error) => match error.downcast::<LspError>() {
            Ok(lsp_error) => lsp_server::Response::new_err(id, lsp_error.code, lsp_error.message),
            Err(error) => match error.downcast::<Cancelled>() {
                Ok(cancelled) => return Err(HandlerCancelledError::Inner(cancelled)),
                #[expect(clippy::as_conversions, reason = "valid according to JSON RPC")]
                Err(error) => lsp_server::Response::new_err(
                    id,
                    lsp_server::ErrorCode::InternalError as i32,
                    error.to_string(),
                ),
            },
        },
    };
    Ok(result)
}

pub(crate) struct NotificationDispatcher<'global_state> {
    pub(crate) not: Option<lsp_server::Notification>,
    pub(crate) global_state: &'global_state mut GlobalState,
}

impl NotificationDispatcher<'_> {
    pub(crate) fn on_sync_mut<N>(
        &mut self,
        function: fn(&mut GlobalState, N::Params) -> Result<()>,
    ) -> &mut Self
    where
        N: lsp_types::notification::Notification + 'static,
        N::Params: DeserializeOwned + Send + 'static,
    {
        let Some(not) = self.not.take() else {
            return self;
        };
        let parameters = match not.extract::<N::Params>(N::METHOD) {
            Ok(it) => it,
            Err(ExtractError::JsonError { method, error }) => {
                panic!("Invalid request\nMethod: {method}\n error: {error}",)
            },
            Err(ExtractError::MethodMismatch(not)) => {
                self.not = Some(not);
                return self;
            },
        };
        if let Err(error) = function(self.global_state, parameters) {
            tracing::error!(handler = %N::METHOD, error = %error, "notification handler failed");
        }
        self
    }

    pub(crate) fn finish(&self) {
        if let Some(not) = &self.not {
            if !not.method.starts_with("$/") {
                tracing::error!("Unhandled notification: {:?}", not);
            }
        }
    }
}
