use std::collections::HashMap;

use hir::diagnostics::DiagnosticsConfig;

#[derive(Default, Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub show_type_errors: bool,
    pub custom_imports: HashMap<String, String>,
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
        if let Err(e) = self.try_update(value) {
            tracing::error!("failed to update config: {:?}", e);
        }
    }

    pub fn diagnostics(&self) -> DiagnosticsConfig {
        DiagnosticsConfig {
            show_type_errors: self.show_type_errors,
        }
    }
}
