use std::collections::HashMap;

use serde::Deserialize;

use hir::diagnostics::DiagnosticsConfig;

#[derive(Default, Clone, Debug, Deserialize)]
pub struct TraceConfig {
    pub extension: bool,
    pub server: bool,
}

#[derive(Default, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub show_type_errors: bool,
    pub custom_imports: HashMap<String, String>,
    pub trace: TraceConfig,
}

impl Config {
    fn try_update(&mut self, value: serde_json::Value) -> Result<(), serde_json::Error> {
        *self = serde_json::from_value(value)?;
        Ok(())
    }

    pub fn update(&mut self, value: serde_json::Value) {
        if value.is_null() {
            return;
        }
        if let Err(e) = self.try_update(value.clone()) {
            tracing::error!("Failed to update config: {:?}", e);
            tracing::error!("Received JSON: {}", value.to_string());
        }
    }

    pub fn diagnostics(&self) -> DiagnosticsConfig {
        DiagnosticsConfig {
            show_type_errors: self.show_type_errors,
        }
    }
}
