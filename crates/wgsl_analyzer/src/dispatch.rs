//! See [`RequestDispatcher`].

use crate::{
	LspError,
	Result,
	global_state::{
		GlobalState,
		GlobalStateSnapshot,
	},
	lsp_utils::is_cancelled,
	main_loop::Task,
};

use fmt::Debug;
use lsp_server::ExtractError;
use serde::{
	Serialize,
	de::DeserializeOwned,
};
use std::{
	fmt,
	panic,
	thread,
};

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
/// Read-only requests are wrapped into `catch_unwind` -- they don't modify the
/// state, so it's OK to recover from their failures.
pub struct RequestDispatcher<'global_state> {
	request: Option<lsp_server::Request>,
	global_state: &'global_state mut GlobalState,
}

impl<'global_state> RequestDispatcher<'global_state> {
	pub fn new(
		request: Option<lsp_server::Request>,
		global_state: &'global_state mut GlobalState,
	) -> Self {
		Self {
			request,
			global_state,
		}
	}

	/// Dispatches the request onto the current thread, given full access to
	/// mutable global state. Unlike all other methods here, this one isn't
	/// guarded by `catch_unwind`, so, please, don't make bugs :-)
	pub(crate) fn on_sync_mut<R>(
		&mut self,
		function: fn(&mut GlobalState, R::Params) -> Result<R::Result>,
	) -> &mut Self
	where
		R: lsp_types::request::Request + 'static,
		R::Params: DeserializeOwned + panic::UnwindSafe + fmt::Debug + 'static,
		R::Result: Serialize + 'static,
	{
		let Some((id, params, _panic_context)) = self.parse::<R>() else {
			return self;
		};

		let result = function(self.global_state, params);
		let response = result_to_response::<R>(id, result);

		self.global_state.respond(response);
		self
	}

	/// Dispatches the request onto the current thread.
	pub(crate) fn on_sync<R>(
		&mut self,
		function: fn(GlobalStateSnapshot, R::Params) -> Result<R::Result>,
	) -> &mut Self
	where
		R: lsp_types::request::Request + 'static,
		R::Params: DeserializeOwned + panic::UnwindSafe + fmt::Debug + 'static,
		R::Result: Serialize + 'static,
	{
		let Some((id, params, _panic_context)) = self.parse::<R>() else {
			return self;
		};
		let global_state_snapshot = self.global_state.snapshot();

		let result = panic::catch_unwind(move || function(global_state_snapshot, params));
		let response = thread_result_to_response::<R>(id, result);

		self.global_state.respond(response);
		self
	}

	/// Dispatches the request onto thread pool
	pub(crate) fn on<R>(
		&mut self,
		function: fn(GlobalStateSnapshot, R::Params) -> Result<R::Result>,
	) -> &mut Self
	where
		R: lsp_types::request::Request + 'static,
		R::Params: DeserializeOwned + panic::UnwindSafe + Send + fmt::Debug + 'static,
		R::Result: Serialize + 'static,
	{
		let Some((id, params, _panic_context)) = self.parse::<R>() else {
			return self;
		};

		self.global_state.task_pool.handle.spawn({
			let world = self.global_state.snapshot();
			move || {
				let result = panic::catch_unwind(move || function(world, params));
				let response = thread_result_to_response::<R>(id, result);
				Task::Response(response)
			}
		});

		self
	}

	pub(crate) fn finish(&mut self) {
		if let Some(req) = self.request.take() {
			tracing::error!("Unknown request: {:?}", req);
			#[expect(clippy::as_conversions, reason = "valid according to JSON RPC")]
			let response = lsp_server::Response::new_err(
				req.id,
				lsp_server::ErrorCode::MethodNotFound as i32,
				"unknown request".to_owned(),
			);
			self.global_state.respond(response);
		}
	}

	fn parse<R>(&mut self) -> Option<(lsp_server::RequestId, R::Params, String)>
	where
		R: lsp_types::request::Request + 'static,
		R::Params: DeserializeOwned + fmt::Debug + 'static,
	{
		let request = match &self.request {
			Some(request) if request.method == R::METHOD => self.request.take().unwrap(),
			_ => return None,
		};

		let result = crate::from_json(R::METHOD, &request.params);
		match result {
			Ok(params) => {
				let panic_context = format!("\nrequest: {} {:#?}", R::METHOD, params);
				Some((request.id, params, panic_context))
			},
			Err(err) => {
				#[expect(clippy::as_conversions, reason = "valid according to JSON RPC")]
				let response = lsp_server::Response::new_err(
					request.id,
					lsp_server::ErrorCode::InvalidParams as i32,
					err.to_string(),
				);
				self.global_state.respond(response);
				None
			},
		}
	}
}

fn thread_result_to_response<R>(
	id: lsp_server::RequestId,
	result: thread::Result<Result<R::Result>>,
) -> lsp_server::Response
where
	R: lsp_types::request::Request + 'static,
	R::Params: DeserializeOwned + 'static,
	R::Result: Serialize + 'static,
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
			};
			#[expect(clippy::as_conversions, reason = "valid according to JSON RPC")]
			lsp_server::Response::new_err(id, lsp_server::ErrorCode::InternalError as i32, message)
		},
	}
}

fn result_to_response<R>(
	id: lsp_server::RequestId,
	result: Result<R::Result>,
) -> lsp_server::Response
where
	R: lsp_types::request::Request + 'static,
	R::Params: DeserializeOwned + 'static,
	R::Result: Serialize + 'static,
{
	match result {
		Ok(resp) => lsp_server::Response::new_ok(id, &resp),
		Err(error) => match error.downcast::<LspError>() {
			Ok(lsp_error) => lsp_server::Response::new_err(id, lsp_error.code, lsp_error.message),
			Err(error) => {
				if is_cancelled(&*error) {
					#[expect(clippy::as_conversions, reason = "valid according to JSON RPC")]
					lsp_server::Response::new_err(
						id,
						lsp_server::ErrorCode::ContentModified as i32,
						"content modified".to_owned(),
					)
				} else {
					#[expect(clippy::as_conversions, reason = "valid according to JSON RPC")]
					lsp_server::Response::new_err(
						id,
						lsp_server::ErrorCode::InternalError as i32,
						error.to_string(),
					)
				}
			},
		},
	}
}

pub struct NotificationDispatcher<'global_state> {
	pub(crate) not: Option<lsp_server::Notification>,
	pub(crate) global_state: &'global_state mut GlobalState,
}

impl<'global_state> NotificationDispatcher<'global_state> {
	pub fn new(
		not: Option<lsp_server::Notification>,
		global_state: &'global_state mut GlobalState,
	) -> Self {
		Self { not, global_state }
	}

	pub(crate) fn on<N>(
		&mut self,
		function: fn(&mut GlobalState, N::Params) -> Result<()>,
	) -> Result<&mut Self>
	where
		N: lsp_types::notification::Notification + 'static,
		N::Params: DeserializeOwned + Send + 'static,
	{
		let Some(not) = self.not.take() else {
			return Ok(self);
		};
		let params = match not.extract::<N::Params>(N::METHOD) {
			Ok(it) => it,
			Err(ExtractError::JsonError { method, error }) => {
				panic!("Invalid request\nMethod: {method}\n error: {error}",)
			},
			Err(ExtractError::MethodMismatch(not)) => {
				self.not = Some(not);
				return Ok(self);
			},
		};
		function(self.global_state, params)?;
		Ok(self)
	}

	pub(crate) fn finish(&self) {
		if let Some(not) = &self.not {
			if !not.method.starts_with("$/") {
				tracing::error!("Unhandled notification: {:?}", not);
			}
		}
	}
}
