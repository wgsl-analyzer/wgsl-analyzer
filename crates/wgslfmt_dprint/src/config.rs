use dprint_core::{
    configuration::{
        ConfigKeyMap, ConfigurationDiagnostic, GlobalConfiguration, NewLineKind,
        get_nullable_value, get_unknown_property_diagnostics,
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

    // if let Some(trailing_commas) =
    //     get_nullable_value::<Policy>(&mut config, "trailingCommas", &mut diagnostics)
    // {
    //     options.trailing_commas = trailing_commas;
    // }

    // We follow
    // https://github.com/dprint/dprint-plugin-typescript/blob/main/src/configuration/resolve_config.rs
    // for the naming of similar options

    if let Some(max_line_width) = get_nullable_value(&mut config, "lineWidth", &mut diagnostics) {
        options.max_line_width = max_line_width;
    }
    if let Some(indent_width) = get_nullable_value(&mut config, "indentWidth", &mut diagnostics) {
        options.indent_width = indent_width;
    }
    if let Some(use_tabs) = get_nullable_value(&mut config, "useTabs", &mut diagnostics) {
        options.indent_style = if use_tabs {
            wgsl_formatter::IndentStyle::Tabs
        } else {
            wgsl_formatter::IndentStyle::Spaces
        };
    }
    if let Some(new_line_kind) = get_nullable_value(&mut config, "newLineKind", &mut diagnostics) {
        options.line_break_style = new_line_kind;
    }

    diagnostics.extend(get_unknown_property_diagnostics(config));

    PluginResolveConfigurationResult {
        config: options,
        diagnostics,
        file_matching: FileMatchingInfo {
            file_extensions: vec!["wgsl".into(), "wesl".into()],
            file_names: vec![],
        },
    }
}
