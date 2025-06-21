pub mod cli;
pub mod config;
mod diagnostics;
mod dispatch;
mod global_state;
mod in_memory_documents;
mod line_index;
pub mod lsp;
pub mod main_loop;
mod operation_queue;
mod reload;
mod task_pool;
mod version;

pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

use serde::de::DeserializeOwned;

pub use crate::{lsp::capabilities::server_capabilities, main_loop::main_loop, version::version};

pub fn from_json<T: DeserializeOwned>(
    what: &'static str,
    json: &serde_json::Value,
) -> Result<T> {
    serde_json::from_value(json.clone())
        .map_err(|error| anyhow::anyhow!("Failed to deserialize {}: {}; {}", what, error, json))
}

#[derive(Debug)]
struct LspError {
    code: i32,
    message: String,
}

impl LspError {
    const fn new(
        code: i32,
        message: String,
    ) -> Self {
        Self { code, message }
    }
}

impl std::fmt::Display for LspError {
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "Language Server request failed with {}. ({})",
            self.code, self.message
        )
    }
}

impl std::error::Error for LspError {}

#[doc(hidden)]
macro_rules! try_default_ {
    ($maybe_value:expr $(,)?) => {
        match $maybe_value {
            Some(value) => value,
            None => return Ok(Default::default()),
        }
    };
}
pub(crate) use try_default_ as try_default;

mod handlers {
    // pub mod dispatch;
    pub(crate) mod notification;
    pub(crate) mod request;
}

pub mod tracing {
    pub mod config;
    pub mod json;
    pub use config::Config;
    pub mod hprof;
}
