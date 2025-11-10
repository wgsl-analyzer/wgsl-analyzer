//! Read `.wesl/config.toml` as a JSON object
use paths::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashMap;

use crate::{ManifestPath, utf8_stdout};

pub(crate) type WeslConfigFile = serde_json::Map<String, serde_json::Value>;

pub(crate) fn read(
    manifest: &ManifestPath,
    extra_env: &FxHashMap<String, Option<String>>,
) -> Option<WeslConfigFile> {
    None
}
