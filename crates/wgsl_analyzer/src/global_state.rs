use std::sync::{Arc, RwLock};
use std::time::Instant;

use base_db::change::Change;
use crossbeam_channel::{unbounded, Receiver, Sender};
use ide::{Analysis, AnalysisHost, Cancellable};
use lsp_types::Url;
use rustc_hash::FxHashMap;
use vfs::{FileId, Vfs};

use crate::config::Config;
use crate::diagnostics::DiagnosticCollection;
use crate::line_index::{LineEndings, LineIndex};
use crate::Result;
use crate::{from_proto, to_proto};
use crate::{main_loop::Task, task_pool::TaskPool};

type ReqHandler = fn(&mut GlobalState, lsp_server::Response);
type ReqQueue = lsp_server::ReqQueue<(String, Instant), ReqHandler>;

// Enforces drop order
pub struct Handle<H, C> {
	pub handle: H,
	pub receiver: C,
}

pub struct GlobalState {
	pub sender: Sender<lsp_server::Message>,
	pub req_queue: ReqQueue,
	pub task_pool: Handle<TaskPool<Task>, Receiver<Task>>,
	pub vfs: Arc<RwLock<(Vfs, FxHashMap<FileId, LineEndings>)>>,
	pub analysis_host: AnalysisHost,
	pub diagnostics: DiagnosticCollection,
	pub config: Arc<Config>,
}

pub struct GlobalStateSnapshot {
	pub vfs: Arc<RwLock<(Vfs, FxHashMap<FileId, LineEndings>)>>,
	pub analysis: Analysis,
	pub config: Arc<Config>,
}

impl GlobalState {
	pub fn new(
		sender: Sender<lsp_server::Message>,
		config: Config,
	) -> Self {
		let task_pool = {
			let (sender, receiver) = unbounded();
			let handle = TaskPool::new(sender);
			Handle { handle, receiver }
		};

		let mut this = Self {
			sender,
			req_queue: ReqQueue::default(),
			task_pool,
			vfs: Arc::new(RwLock::new((Vfs::default(), FxHashMap::default()))),
			analysis_host: AnalysisHost::new(),
			diagnostics: DiagnosticCollection::default(),
			config: Arc::new(Config::default()),
		};
		this.update_configuration(config);
		this
	}

	pub(crate) fn process_changes(&mut self) -> bool {
		let change = {
			let mut change = Change::new();
			let (vfs, line_endings_map) = &mut *self.vfs.write().unwrap();
			let changed_files = vfs.take_changes();
			if changed_files.is_empty() {
				return false;
			}

			for file in changed_files {
				let text = if file.exists() {
					let bytes = vfs.file_contents(file.file_id).to_vec();
					String::from_utf8(bytes).ok().map(|text| {
						let (text, line_endings) = LineEndings::normalize(text);
						line_endings_map.insert(file.file_id, line_endings);
						Arc::new(text)
					})
				} else {
					None
				};
				change.change_file(file.file_id, text);
			}
			change
		};

		self.analysis_host.apply_change(change);
		true
	}

	pub fn snapshot(&self) -> GlobalStateSnapshot {
		GlobalStateSnapshot {
			vfs: Arc::clone(&self.vfs),
			analysis: self.analysis_host.snapshot(),
			config: Arc::clone(&self.config),
		}
	}

	pub(crate) fn send_request<R: lsp_types::request::Request>(
		&mut self,
		params: R::Params,
		handler: ReqHandler,
	) {
		let request = self
			.req_queue
			.outgoing
			.register(R::METHOD.to_owned(), params, handler);
		self.send(request.into());
	}

	pub(crate) fn send_notification<N: lsp_types::notification::Notification>(
		&self,
		params: N::Params,
	) {
		let not = lsp_server::Notification::new(N::METHOD.to_owned(), params);
		self.send(not.into());
	}

	pub(crate) fn register_request(
		&mut self,
		request: &lsp_server::Request,
		request_received: Instant,
	) {
		self.req_queue.incoming.register(
			request.id.clone(),
			(request.method.clone(), request_received),
		);
	}
	pub(crate) fn respond(
		&mut self,
		response: lsp_server::Response,
	) {
		if let Some((method, start)) = self.req_queue.incoming.complete(&response.id) {
			if let Some(err) = &response.error {
				self.show_message(lsp_types::MessageType::ERROR, err.message.clone());
			}

			let duration = start.elapsed();
			tracing::debug!(
				"Handled {} - ({}) in {:0.2?}",
				method,
				response.id,
				duration
			);
			self.send(response.into());
		}
	}
	pub(crate) fn cancel(
		&mut self,
		request_id: lsp_server::RequestId,
	) {
		if let Some(response) = self.req_queue.incoming.cancel(request_id) {
			self.send(response.into());
		}
	}

	fn send(
		&self,
		message: lsp_server::Message,
	) {
		self.sender.send(message).unwrap();
	}
}

impl GlobalStateSnapshot {
	pub(crate) fn url_to_file_id(
		&self,
		url: &Url,
	) -> Result<FileId> {
		url_to_file_id(&self.vfs.read().unwrap().0, url)
	}

	pub(crate) fn file_id_to_url(
		&self,
		id: FileId,
	) -> Url {
		file_id_to_url(&self.vfs.read().unwrap().0, id)
	}

	pub(crate) fn file_line_index(
		&self,
		file_id: FileId,
	) -> Cancellable<LineIndex> {
		let endings = self.vfs.read().unwrap().1[&file_id];
		let index = self.analysis.line_index(file_id)?;
		let res = LineIndex {
			index,
			endings,
			encoding: self.config.offset_encoding(),
		};
		Ok(res)
	}
}

pub fn file_id_to_url(
	vfs: &vfs::Vfs,
	id: FileId,
) -> Url {
	let path = vfs.file_path(id);
	let path = path.as_path().unwrap();
	to_proto::url_from_abs_path(path)
}

pub fn url_to_file_id(
	vfs: &vfs::Vfs,
	url: &Url,
) -> Result<FileId> {
	let path = from_proto::vfs_path(url)?;
	let res = vfs
		.file_id(&path)
		.ok_or_else(|| anyhow::anyhow!("file not found: {}", path))?;
	Ok(res)
}
