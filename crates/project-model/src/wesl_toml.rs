use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeslToml {
    /// WESL edition (required).
    pub edition: String,
    pub package_manager: Option<String>,
    #[serde(default = "default_root")]
    pub root: String,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub dependencies: BTreeMap<String, WeslDependency>,
}

fn default_root() -> String {
    "./shaders/".to_string()
}

#[derive(Deserialize)]
pub struct WeslDependency {
    pub package: Option<String>,
    pub path: Option<String>,
}
