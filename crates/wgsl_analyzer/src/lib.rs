mod capabilities;
pub mod config;
mod diagnostics;
mod dispatch;
mod from_proto;
mod global_state;
mod handlers;
mod line_index;
mod lsp_ext;
mod lsp_utils;
pub mod main_loop;
mod task_pool;
mod to_proto;

pub use capabilities::server_capabilities;

pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

use serde::de::DeserializeOwned;

pub fn from_json<T: DeserializeOwned>(
    what: &'static str,
    json: serde_json::Value,
) -> Result<T> {
    let res = serde_json::from_value(json.clone())
        .map_err(|e| anyhow::anyhow!("Failed to deserialize {}: {}; {}", what, e, json))?;
    Ok(res)
}

#[derive(Debug)]
struct LspError {
    code: i32,
    message: String,
}

impl LspError {
    #[allow(dead_code)]
    fn new(
        code: i32,
        message: String,
    ) -> LspError {
        LspError { code, message }
    }
}

impl std::fmt::Display for LspError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "Language Server request failed with {}. ({})",
            self.code, self.message
        )
    }
}

impl std::error::Error for LspError {}
