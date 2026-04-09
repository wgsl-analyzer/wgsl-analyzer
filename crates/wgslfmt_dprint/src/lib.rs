use anyhow::Result;
use dprint_core::{
    configuration::{ConfigKeyMap, GlobalConfiguration},
    plugins::{
        CheckConfigUpdatesMessage, ConfigChange, FormatResult, PluginInfo,
        PluginResolveConfigurationResult, SyncFormatRequest, SyncHostFormatRequest,
        SyncPluginHandler,
    },
};
use wgsl_formatter::{FormattingOptions, format_str};

use crate::config::resolve_config;

mod config;

pub struct WgslPluginHandler;

impl SyncPluginHandler<FormattingOptions> for WgslPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        let version = env!("CARGO_PKG_VERSION").to_owned();
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_owned(),
            version: version.clone(),
            config_key: "wgslfmt".to_owned(),
            help_url: "https://github.com/wgsl-analyzer/wgsl-analyzer".to_owned(),
            config_schema_url: format!(
                "https://plugins.dprint.dev/wgsl-analyzer/wgsl-analyzer/v{version}/dprint_plugin_wgslfmt_schema.json",
            ),
            update_url: Some(
                "https://plugins.dprint.dev/wgsl-analyzer/wgsl-analyzer/latest.json".into(),
            ),
        }
    }

    fn license_text(&mut self) -> String {
        format!(
            "{}\n\n{}",
            include_str!("../../../LICENSE-MIT"),
            include_str!("../../../LICENSE-APACHE"),
        )
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<FormattingOptions> {
        resolve_config(config, global_config)
    }

    fn check_config_updates(
        &self,
        _: CheckConfigUpdatesMessage,
    ) -> Result<Vec<ConfigChange>> {
        Ok(Vec::new())
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<'_, FormattingOptions>,
        _: impl FnMut(SyncHostFormatRequest<'_>) -> FormatResult,
    ) -> FormatResult {
        let config = request.config;

        let formatted = format_str(std::str::from_utf8(&request.file_bytes)?, config);

        Ok(Some(formatted.into_bytes()))
    }
}

#[cfg(target_arch = "wasm32")]
dprint_core::generate_plugin_code!(WgslPluginHandler, WgslPluginHandler, FormattingOptions);
