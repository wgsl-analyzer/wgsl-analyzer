use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeslManifest {
    /// WESL edition (required).
    pub edition: String,
    pub package_manager: Option<String>,
    #[serde(default = "default_root")]
    pub root: String,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    #[serde(default)]
    pub dependencies: BTreeMap<String, WeslDependency>,
}

impl Default for WeslManifest {
    fn default() -> Self {
        Self {
            edition: "2026_pre".to_owned(),
            package_manager: None,
            root: default_root(),
            include: None,
            exclude: None,
            dependencies: BTreeMap::new(),
        }
    }
}

impl WeslManifest {
    pub fn from_slice(bytes: &[u8]) -> Result<Self, toml::de::Error> {
        toml::from_slice(bytes)
    }
}

fn default_root() -> String {
    "./shaders".to_owned()
}

#[derive(Deserialize)]
pub struct WeslDependency {
    pub package: Option<String>,
    pub path: Option<String>,
}
