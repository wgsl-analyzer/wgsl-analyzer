use dprint_core::{
    configuration::{
        ConfigKeyMap, ConfigurationDiagnostic, GlobalConfiguration, get_nullable_value,
        get_unknown_property_diagnostics,
    },
    plugins::{FileMatchingInfo, PluginResolveConfigurationResult},
};
use wgsl_formatter::{FormattingOptions, Policy};

pub(crate) fn resolve_config(
    mut config: ConfigKeyMap,
    global_config: &GlobalConfiguration,
) -> PluginResolveConfigurationResult<FormattingOptions> {
    let mut diagnostics = Vec::new();

    let mut options = FormattingOptions::default();

    if let Some(trailing_commas) =
        get_nullable_value::<Policy>(&mut config, "trailingCommas", &mut diagnostics)
    {
        options.trailing_commas = trailing_commas;
    }

    if let Some(indent_symbol) =
        get_nullable_value::<String>(&mut config, "indentSymbol", &mut diagnostics)
    {
        options.indent_symbol = indent_symbol;
    } else if global_config.use_tabs == Some(true) {
        options.indent_symbol = "\t".into();
    } else if let Some(indent_width) = global_config.indent_width {
        options.indent_symbol = " ".repeat(indent_width.into());
    }

    diagnostics.extend(get_unknown_property_diagnostics(config));

    PluginResolveConfigurationResult {
        config: options,
        diagnostics,
        file_matching: FileMatchingInfo {
            file_extensions: vec!["wgsl".into()],
            file_names: vec![],
        },
    }
}
