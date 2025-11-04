use std::{fmt, sync::OnceLock};

use base_db::input::SourceRootId;
use hir::diagnostics::{DiagnosticsConfig, NagaVersion};
use hir_ty::ty::pretty::TypeVerbosity;
use ide::{
    HoverConfig, HoverDocFormat, MemoryLayoutHoverRenderKind,
    inlay_hints::{self, StructLayoutHints},
};
use ide_completion::{CompletionConfig, CompletionFieldsToResolve};
use itertools::Itertools as _;
use rustc_hash::FxHashMap;
use semver::Version;
use serde::{Deserialize, Serialize};
use stdx::format_to_accumulator;
use triomphe::Arc;
use vfs::AbsPathBuf;

// use ide::{
//     AssistConfig, CallHierarchyConfig, CallableSnippets, CompletionConfig,
//     CompletionFieldsToResolve, DiagnosticsConfig, ExprFillDefaultMode, GenericParameterHints,
//     HighlightConfig, HighlightRelatedConfig, HoverConfig, HoverDocFormat, InlayFieldsToResolve,
//     InlayHintsConfig, JoinLinesConfig, MemoryLayoutHoverConfig, MemoryLayoutHoverRenderKind,
//     Snippet, SnippetScope, SourceRootId,
// };
// use ide_db::{
//     imports::insert_use::{ImportGranularity, InsertUseConfig, PrefixKind},
//     SnippetCap,
// };
use crate::lsp::capabilities::ClientCapabilities;

// Defines the server-side configuration of the wgsl-analyzer. We generate *parts* of VS Code's
// `package.json` config from this. Run `cargo test` to re-generate that file.
//
// However, editor specific config, which the server doesn't know about, should be specified
// directly in `package.json`.
//
// To deprecate an option by replacing it with another name use `new_name` | `old_name` so that we
// keep parsing the old name.
config_data! {
    /// Configs that apply on a workspace-wide scope. There are two levels at which a global
    /// configuration can be provided:
    /// 1. Client-specific settings (e.g. `settings.json` in VS Code).
    /// 2. A user-wide configuration file in this tool's config directory.
    /// A config value is resolved by traversing the "config tree" from the most specific scope
    /// upward (nearest-first principle). The first level that specifies a value wins.
    global: struct GlobalDefaultConfigData <- GlobalConfigInput -> {
        /// Number of worker threads used to warm caches when a project opens.
        /// Use `0` to let the server choose automatically based on the machine.
        cachePriming_numThreads: NumThreads = NumThreads::default(),

        /// Controls whether to show naga's parsing errors.
        diagnostics_nagaParsingErrors: bool = true,
        /// Controls whether to show naga's validation errors.
        diagnostics_nagaValidationErrors: bool = true,
        /// Naga version used for validation.
        diagnostics_nagaVersion: NagaVersionConfig = NagaVersionConfig::default(),
        /// Controls whether to show type errors.
        diagnostics_typeErrors: bool = true,

        /// Whether to show inlay hints.
        inlayHints_enabled: bool = true,
        /// Whether to show inlay hints for the names of function parameters.
        inlayHints_parameterHints: bool = true,
        /// Whether to render leading colons for type hints, and trailing colons for parameter hints.
        inlayHints_renderColons: bool = true,
        /// Whether to show inlay hints for the layout of struct fields.
        inlayHints_structLayoutHints: bool = false,
        /// Whether to show inlay hints for types of variable declarations.
        inlayHints_typeHints: bool = true,
        /// Verbosity of type hints: `"full"`, `"compact"`, or `"inner"`.
        inlayHints_typeVerbosity: InlayHintsTypeVerbosity = InlayHintsTypeVerbosity::default(),

        /// Number of worker threads for the main analysis loop.
        /// `null` lets the server choose automatically.
        numThreads: Option<NumThreads> = None,

        /// Enable logging of VS Code extensions itself.
        /// This settings is now deprecated.
        /// Log level is now controlled by the [Developer: Set Log Level...](command:workbench.action.setLogLevel) command. You can set the log level for the current session and also the default log level from there. This is also available by clicking the gear icon on the OUTPUT tab when wgsl-analyzer Client is visible or by passing the --log wgsl-analyzer.wgsl-analyzer:debug parameter to VS Code.
        trace_extension: bool = false,
        /// Server trace verbosity.
        /// One of: `"off"`, `"messages"`, or `"verbose"`.
        trace_server: TraceServer = TraceServer::Off,
    }
}

#[derive(
    Clone, Copy, Debug, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum NagaVersionConfig {
    #[serde(rename = "0.14")]
    Naga14,
    #[serde(rename = "0.19")]
    Naga19,
    #[serde(rename = "0.22")]
    #[default]
    Naga22,
    #[serde(rename = "main")]
    NagaMain,
}

#[derive(
    Clone, Copy, Debug, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "lowercase")]
pub enum InlayHintsTypeVerbosity {
    /// `ref<uniform, f32, read_write>`
    Full,
    #[default]
    /// `ref<f32>`
    Compact,
    /// `f32`
    Inner,
}

#[derive(
    Clone, Copy, Debug, Serialize, Deserialize, Default, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
#[serde(rename_all = "lowercase")]
pub enum TraceServer {
    #[default]
    Off,
    Messages,
    Verbose,
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
struct ClientInfo {
    name: String,
    version: Option<Version>,
}

#[derive(Clone, Debug)]
pub struct Config {
    /// The workspace roots as registered by the LSP client
    workspace_roots: Vec<AbsPathBuf>,
    capabilities: ClientCapabilities,
    root_path: AbsPathBuf,
    // snippets: Vec<Snippet>,
    client_info: Option<ClientInfo>,
    diagnostics_enable: bool,

    default: &'static DefaultConfigData,
    /// Config node that obtains its initial value during the server initialization and
    /// by receiving a `lsp_types::notification::DidChangeConfiguration`.
    client: (FullConfigInput, ConfigErrors),

    /// Config node whose values apply to **every** wgsl project.
    user: Option<(GlobalWorkspaceLocalConfigInput, ConfigErrors)>,

    // /// Clone of the value that is stored inside a `GlobalState`.
    // source_root_parent_map: Arc<FxHashMap<SourceRootId, SourceRootId>>,
    /// Use case : It is an error to have an empty value for `check_command`.
    /// Since it is a `global` command at the moment, its final value can only be determined by
    /// traversing through `global` configs and the `client` config. However the non-null value constraint
    /// is config level agnostic, so this requires an independent error storage
    validation_errors: ConfigErrors,

    detached_files: Vec<AbsPathBuf>,
    wgslfmt_override_command: Option<Vec<String>>,
    wgslfmt_extra_args: Vec<String>,
    wgslfmt_range_formatting_enable: bool,
}

impl Config {
    #[must_use]
    #[expect(
        clippy::unused_self,
        reason = "TODO: See https://github.com/wgsl-analyzer/wgsl-analyzer/issues/26"
    )]
    pub const fn discover_workspace_config(&self) -> Option<&DiscoverWorkspaceConfig> {
        None
    }

    #[must_use]
    pub const fn publish_diagnostics(
        &self,
        source_root: Option<SourceRootId>,
    ) -> bool {
        self.diagnostics_enable
    }
}

// Delegate capability fetching methods
impl std::ops::Deref for Config {
    type Target = ClientCapabilities;

    fn deref(&self) -> &Self::Target {
        &self.capabilities
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoverWorkspaceConfig {
    pub command: Vec<String>,
    pub progress_label: String,
    pub files_to_watch: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintsConfig {
    pub render_colons: bool,
    pub enabled: bool,
    pub type_hints: bool,
    pub parameter_hints: bool,
    pub struct_layout_hints: bool,
    pub type_verbosity: InlayHintsTypeVerbosity,
}

impl Default for InlayHintsConfig {
    fn default() -> Self {
        Self {
            render_colons: true,
            enabled: true,
            type_hints: true,
            parameter_hints: true,
            struct_layout_hints: false,
            type_verbosity: InlayHintsTypeVerbosity::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlayFieldsToResolve {
    pub resolve_text_edits: bool,
    pub resolve_hint_tooltip: bool,
    pub resolve_label_tooltip: bool,
    pub resolve_label_location: bool,
    pub resolve_label_command: bool,
}

#[derive(Clone, Debug, Default)]
pub struct ConfigErrors(Vec<Arc<ConfigErrorInner>>);

impl ConfigErrors {
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for ConfigErrors {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let errors = self
            .0
            .iter()
            .format_with("\n", |inner, formatter| match &**inner {
                ConfigErrorInner::Json { config_key, error } => {
                    formatter(config_key)?;
                    formatter(&": ")?;
                    formatter(error)
                },
                // ConfigErrorInner::Toml { config_key: key, error: e } => {
                //     formatter(key)?;
                //     formatter(&": ")?;
                //     formatter(e)
                // }
                ConfigErrorInner::ParseError { reason } => formatter(reason),
                _ => formatter(&""),
            });
        write!(
            formatter,
            "invalid config value{}:\n{errors}",
            if self.0.len() == 1 { "" } else { "s" }
        )
    }
}

impl std::error::Error for ConfigErrors {}

impl Config {
    #[must_use]
    pub fn new(
        root_path: AbsPathBuf,
        caps: lsp_types::ClientCapabilities,
        workspace_roots: Vec<AbsPathBuf>,
        client_info: Option<lsp_types::ClientInfo>,
    ) -> Self {
        static DEFAULT_CONFIG_DATA: OnceLock<&'static DefaultConfigData> = OnceLock::new();

        Self {
            workspace_roots,
            // discovered_projects_from_filesystem: Vec::new(),
            // discovered_projects_from_command: Vec::new(),
            capabilities: ClientCapabilities::new(caps),
            // snippets: Default::default(),
            root_path,
            client_info: client_info.map(|client_info| ClientInfo {
                name: client_info.name,
                version: client_info
                    .version
                    .as_deref()
                    .map(Version::parse)
                    .and_then(Result::ok),
            }),
            diagnostics_enable: true,
            default: DEFAULT_CONFIG_DATA.get_or_init(|| Box::leak(Box::default())),
            client: (FullConfigInput::default(), ConfigErrors(vec![])),
            // source_root_parent_map: Arc::new(FxHashMap::default()),
            user: None,
            validation_errors: ConfigErrors::default(),
            detached_files: Vec::default(),
            // watoml_file: Default::default(),
            wgslfmt_override_command: None,
            wgslfmt_extra_args: vec![],
            wgslfmt_range_formatting_enable: false,
        }
    }

    #[expect(
        clippy::unused_self,
        clippy::needless_pass_by_ref_mut,
        reason = "TODO: See https://github.com/wgsl-analyzer/wgsl-analyzer/issues/26"
    )]
    pub const fn rediscover_workspaces(&mut self) {
        // let discovered = vec![];
        // tracing::info!("discovered projects: {:?}", discovered);
        // if discovered.is_empty() {
        //     tracing::error!("failed to find any projects in {:?}", &self.workspace_roots);
        // }
        // self.discovered_projects_from_filesystem = discovered;
    }

    #[must_use]
    pub fn json_schema() -> serde_json::Value {
        let mut schema = FullConfigInput::json_schema();

        fn sort_objects_by_field(json: &mut serde_json::Value) {
            if let serde_json::Value::Object(object) = json {
                let old = std::mem::take(object);
                old.into_iter()
                    .sorted_by(|(key, _), (key2, _)| key.cmp(key2))
                    .for_each(|(key, mut value)| {
                        sort_objects_by_field(&mut value);
                        object.insert(key, value);
                    });
            }
        }
        sort_objects_by_field(&mut schema);
        schema
    }

    fn apply_change_with_sink(
        &self,
        change: ConfigChange,
    ) -> (Self, bool) {
        let mut config = self.clone();
        let mut should_update = false;

        if let Some(mut json) = change.client_config {
            tracing::info!("updating WGSL config from JSON: {:#}", json);

            if json.is_null() || json.as_object().is_some_and(serde_json::Map::is_empty) {
                return (config, should_update);
            }

            let mut json_errors = Vec::<(String, serde_json::Error)>::new();
            let input = FullConfigInput::from_json(json, &mut json_errors);

            // Stash parsed values and any field-level errors so `apply_change` can surface them.
            config.client = (
                input,
                ConfigErrors(
                    json_errors
                        .into_iter()
                        .map(|(config_key, error)| {
                            Arc::new(ConfigErrorInner::Json { config_key, error })
                        })
                        .collect(),
                ),
            );

            // If the client changed anything, let the caller rebuild derived state (hints, hovers, etc.)
            should_update = true;
        }

        (config, should_update)
    }

    /// Given `change` this generates a new `Config`, thereby collecting errors of type `ConfigError`.
    /// If there are changes that have global/client level effect, the last component of the return type
    /// will be set to `true`, which should be used by the `GlobalState` to update itself.
    #[must_use]
    pub fn apply_change(
        &self,
        change: ConfigChange,
    ) -> (Self, ConfigErrors, bool) {
        let (config, should_update) = self.apply_change_with_sink(change);
        let errors = ConfigErrors(
            config
                .client
                .1
                .0
                .iter()
                .chain(
                    config
                        .user
                        .as_ref()
                        .into_iter()
                        .flat_map(|pair| pair.1.0.iter()),
                )
                .chain(config.validation_errors.0.iter())
                .cloned()
                .collect(),
        );
        (config, errors, should_update)
    }

    #[must_use]
    pub const fn root_path(&self) -> &AbsPathBuf {
        &self.root_path
    }

    #[must_use]
    pub const fn capabilities(&self) -> &ClientCapabilities {
        &self.capabilities
    }

    #[must_use]
    pub fn prime_caches_number_of_threads(&self) -> usize {
        match self.cachePriming_numThreads() {
            NumThreads::Concrete(0) | NumThreads::Physical => num_cpus::get_physical(),
            NumThreads::Concrete(number) => *number,
            NumThreads::Logical => num_cpus::get(),
        }
    }

    #[must_use]
    pub fn main_loop_number_of_threads(&self) -> usize {
        match self.numThreads() {
            Some(NumThreads::Concrete(0) | NumThreads::Physical) | None => num_cpus::get_physical(),
            Some(NumThreads::Concrete(number)) => *number,
            Some(NumThreads::Logical) => num_cpus::get(),
        }
    }

    #[must_use]
    pub fn completion(
        &self,
        source_root: Option<SourceRootId>,
    ) -> CompletionConfig {
        let client_capability_fields = self.completion_resolve_support_properties();
        CompletionConfig {
            enable_postfix_completions: false,
            enable_imports_on_the_fly: false,
            enable_self_on_the_fly: false,
            enable_auto_iter: false,
            enable_auto_await: false,
            enable_private_editable: false,
            enable_term_search: false,
            term_search_fuel: 400,
            full_function_signatures: false,
            callable: None,
            add_semicolon_to_unit: false,
            // pub snippet_cap: Option<SnippetCap>,
            // pub insert_use: InsertUseConfig,
            prefer_no_std: false,
            prefer_prelude: false,
            prefer_absolute: false,
            // pub snippets: Vec<Snippet>,
            limit: None,
            fields_to_resolve: CompletionFieldsToResolve::from_client_capabilities(
                &client_capability_fields,
            ),
            exclude_flyimport: <_>::default(),
        }
    }

    #[must_use]
    pub fn hover_actions(&self) -> HoverActionsConfig {
        let enable = self.capabilities.hover_actions();
        HoverActionsConfig {
            implementations: enable,
            references: enable,
            run: enable,
            debug: enable,
            update_test: enable,
            goto_type_def: enable,
        }
    }

    #[must_use]
    #[expect(
        clippy::unused_self,
        reason = "TODO: See https://github.com/wgsl-analyzer/wgsl-analyzer/issues/362"
    )]
    pub fn hover(&self) -> HoverConfig {
        let mem_kind = |kind| match kind {
            MemoryLayoutHoverRenderKindDef::Both => MemoryLayoutHoverRenderKind::Both,
            MemoryLayoutHoverRenderKindDef::Decimal => MemoryLayoutHoverRenderKind::Decimal,
            MemoryLayoutHoverRenderKindDef::Hexadecimal => MemoryLayoutHoverRenderKind::Hexadecimal,
        };
        HoverConfig {
            links_in_hover: false,
            memory_layout: None,
            documentation: false,
            format: {
                if false {
                    HoverDocFormat::Markdown
                } else {
                    HoverDocFormat::PlainText
                }
            },
            keywords: false,
            max_fields_count: None,
            max_enum_variants_count: None,
            max_subst_ty_len: ide::SubstTyLen::LimitTo(20),
        }
    }

    #[must_use]
    pub fn inlay_hints(&self) -> inlay_hints::InlayHintsConfig {
        let client_capability_fields = self.inlay_hint_resolve_support_properties();
        inlay_hints::InlayHintsConfig {
            render_colons: *self.inlayHints_renderColons(),
            enabled: *self.inlayHints_enabled(),
            type_hints: *self.inlayHints_typeHints(),
            parameter_hints: *self.inlayHints_parameterHints(),
            struct_layout_hints: self
                .inlayHints_structLayoutHints()
                .then_some(StructLayoutHints::Offset),
            type_verbosity: match self.inlayHints_typeVerbosity() {
                InlayHintsTypeVerbosity::Full => TypeVerbosity::Full,
                InlayHintsTypeVerbosity::Compact => TypeVerbosity::Compact,
                InlayHintsTypeVerbosity::Inner => TypeVerbosity::Inner,
            },
            fields_to_resolve: ide::inlay_hints::InlayFieldsToResolve::from_client_capabilities(
                &client_capability_fields,
            ),
        }
    }

    #[must_use]
    #[expect(
        clippy::unused_self,
        reason = "TODO: See https://github.com/wgsl-analyzer/wgsl-analyzer/issues/363"
    )]
    pub const fn completion_hide_deprecated(&self) -> bool {
        false
    }

    #[must_use]
    pub fn wgslfmt(
        &self,
        source_root_id: Option<SourceRootId>,
    ) -> WgslfmtConfig {
        match &self.wgslfmt_override_command {
            Some(arguments) if !arguments.is_empty() => {
                let mut arguments = arguments.clone();
                let command = arguments.remove(0);
                WgslfmtConfig::CustomCommand { command, arguments }
            },
            Some(_) | None => WgslfmtConfig::Wgslfmt {
                extra_arguments: self.wgslfmt_extra_args.clone(),
                enable_range_formatting: self.wgslfmt_range_formatting_enable,
            },
        }
    }

    #[must_use]
    pub fn diagnostics(
        &self,
        source_root: Option<SourceRootId>,
    ) -> DiagnosticsConfig {
        DiagnosticsConfig {
            enabled: true,
            type_errors: *self.diagnostics_typeErrors(),
            naga_parsing_errors: *self.diagnostics_nagaParsingErrors(),
            naga_validation_errors: *self.diagnostics_nagaValidationErrors(),
            naga_version: match self.diagnostics_nagaVersion() {
                NagaVersionConfig::Naga14 => NagaVersion::Naga14,
                NagaVersionConfig::Naga19 => NagaVersion::Naga19,
                NagaVersionConfig::Naga22 => NagaVersion::Naga22,
                NagaVersionConfig::NagaMain => NagaVersion::NagaMain,
            },
        }
    }

    #[must_use]
    #[expect(
        clippy::unnecessary_wraps,
        clippy::unused_self,
        reason = "Intended to be refactored into config macro"
    )]
    pub fn typing_trigger_chars(&self) -> Option<String> {
        Some("=.".to_owned())
    }

    // VSCode is our reference implementation, so we allow ourselves to work around issues by
    // special casing certain versions
    #[must_use]
    pub fn visual_studio_code_version(&self) -> Option<&Version> {
        let client_info = self
            .client_info
            .as_ref()
            .filter(|client_info| client_info.name.starts_with("Visual Studio Code"))?;
        client_info.version.as_ref()
    }

    #[must_use]
    pub fn client_is_helix(&self) -> bool {
        self.client_info
            .as_ref()
            .is_some_and(|client_info| client_info.name == "helix")
    }

    #[must_use]
    pub fn client_is_neovim(&self) -> bool {
        self.client_info
            .as_ref()
            .is_some_and(|client_info| client_info.name == "Neovim")
    }
}

#[derive(Default, Debug)]
pub struct ConfigChange {
    user_config: Option<Arc<str>>,
    client_config: Option<serde_json::Value>,
    source_map: Option<Arc<FxHashMap<SourceRootId, SourceRootId>>>,
}

#[derive(Copy, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum NumThreads {
    #[default]
    Physical,
    Logical,
    #[serde(untagged)]
    Concrete(usize),
}

impl ConfigChange {
    /// # Panics
    ///
    /// Panics if double writing
    pub fn change_user_config(
        &mut self,
        content: Option<Arc<str>>,
    ) {
        assert!(self.user_config.is_none()); // Otherwise it is a double write.
        self.user_config = content;
    }

    pub fn change_client_config(
        &mut self,
        change: serde_json::Value,
    ) {
        self.client_config = Some(change);
    }

    /// # Panics
    ///
    /// Panics if double writing
    pub fn change_source_root_parent_map(
        &mut self,
        source_root_map: Arc<FxHashMap<SourceRootId, SourceRootId>>,
    ) {
        assert!(self.source_map.is_none());
        self.source_map = Some(source_root_map);
    }
}

#[derive(Debug, Clone)]
pub enum WgslfmtConfig {
    Wgslfmt {
        extra_arguments: Vec<String>,
        enable_range_formatting: bool,
    },
    CustomCommand {
        command: String,
        arguments: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
enum MemoryLayoutHoverRenderKindDef {
    Decimal,
    Hexadecimal,
    Both,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverActionsConfig {
    pub implementations: bool,
    pub references: bool,
    pub run: bool,
    pub debug: bool,
    pub update_test: bool,
    pub goto_type_def: bool,
}

impl HoverActionsConfig {
    pub const NO_ACTIONS: Self = Self {
        implementations: false,
        references: false,
        run: false,
        debug: false,
        update_test: false,
        goto_type_def: false,
    };

    #[must_use]
    pub const fn any(&self) -> bool {
        self.implementations || self.references || self.runnable() || self.goto_type_def
    }

    #[must_use]
    pub const fn none(&self) -> bool {
        !self.any()
    }

    #[must_use]
    pub const fn runnable(&self) -> bool {
        self.run || self.debug || self.update_test
    }
}

#[derive(Default, Debug, Clone)]
struct DefaultConfigData {
    global: GlobalDefaultConfigData,
    // workspace: WorkspaceDefaultConfigData,
    // local: LocalDefaultConfigData,
    // client: ClientDefaultConfigData,
}

/// All of the config levels, all fields `Option<T>`, to describe fields that are actually set by
/// some JSON blob.
#[derive(Debug, Clone, Default)]
struct FullConfigInput {
    global: GlobalConfigInput,
    // workspace: WorkspaceConfigInput,
    // local: LocalConfigInput,
    // client: ClientConfigInput,
}

impl FullConfigInput {
    fn from_json(
        mut json: serde_json::Value,
        error_sink: &mut Vec<(String, serde_json::Error)>,
    ) -> Self {
        Self {
            global: GlobalConfigInput::from_json(&mut json, error_sink),
            // local: LocalConfigInput::from_json(&mut json, error_sink),
            // client: ClientConfigInput::from_json(&mut json, error_sink),
            // workspace: WorkspaceConfigInput::from_json(&mut json, error_sink),
        }
    }

    fn schema_fields() -> Vec<SchemaField> {
        let mut fields = Vec::new();
        GlobalConfigInput::schema_fields(&mut fields);
        fields.sort_by_key(|&(key, ..)| key);
        fields
            .iter()
            .tuple_windows()
            .for_each(|(field_a, field_b)| {
                assert!(field_a.0 != field_b.0, "{field_a:?} duplicate field");
            });
        fields
    }

    fn json_schema() -> serde_json::Value {
        schema(&Self::schema_fields())
    }

    #[cfg(test)]
    fn manual() -> String {
        manual(&Self::schema_fields())
    }
}

/// All of the config levels, all fields `Option<T>`, to describe fields that are actually set by
/// some JSON blob.
#[derive(Debug, Clone, Default)]
struct GlobalWorkspaceLocalConfigInput {
    global: GlobalConfigInput,
    // local: LocalConfigInput,
    // workspace: WorkspaceConfigInput,
}

fn get_field_json<T: serde::de::DeserializeOwned>(
    json: &mut serde_json::Value,
    error_sink: &mut Vec<(String, serde_json::Error)>,
    field: &'static str,
    alias: Option<&'static str>,
) -> Option<T> {
    // XXX: check alias first, to work around the VS Code issue where it pre-fills defaults
    // instead of sending an empty object.
    alias
        .into_iter()
        .chain(std::iter::once(field))
        .find_map(|field_name| {
            let mut pointer = field_name.replace('_', "/");
            pointer.insert(0, '/');

            let value = json.pointer_mut(&pointer)?;

            match serde_json::from_value(value.take()) {
                Ok(parsed) => Some(parsed),
                Err(error) => {
                    tracing::warn!(
                        "Failed to deserialize config field at {}: {:?}",
                        pointer,
                        error
                    );
                    error_sink.push((pointer, error));
                    None
                },
            }
        })
}

type SchemaField = (&'static str, &'static str, &'static [&'static str], String);

fn schema(fields: &[SchemaField]) -> serde_json::Value {
    let map = fields
        .iter()
        .map(|(field, field_type, doc, default)| {
            let name = field.replace('_', ".");
            let category = name.split_once('.').map_or_else(
                || "wgsl-analyzer".into(),
                |(category, _name)| to_title_case(category),
            );
            let name = format!("wgsl-analyzer.{name}");
            let props = field_props(field, field_type, doc, default);
            serde_json::json!({
                "title": category,
                "properties": {
                    name: props
                }
            })
        })
        .collect::<Vec<_>>();
    map.into()
}

/// Translate a field name to a title case string suitable for use in the category names on the
/// vscode settings page.
///
/// First letter of word should be uppercase, if an uppercase letter is encountered, add a space
/// before it e.g. "fooBar" -> "Foo Bar", "fooBarBaz" -> "Foo Bar Baz", "foo" -> "Foo"
///
/// This likely should be in stdx (or just use heck instead), but it doesn't handle any edge cases
/// and is intentionally simple.
fn to_title_case(string: &str) -> String {
    let mut result = String::with_capacity(string.len());
    let mut chars = string.chars();
    if let Some(first) = chars.next() {
        result.push(first.to_ascii_uppercase());
        for character in chars {
            if character.is_uppercase() {
                result.push(' ');
            }
            result.push(character);
        }
    }
    result
}

#[expect(
    clippy::too_many_lines,
    reason = "Schema mapping table: many simple cases; splitting would obscure structure"
)]
fn field_props(
    field: &str,
    field_type: &str,
    doc: &[&str],
    default: &str,
) -> serde_json::Value {
    let doc = doc_comment_to_string(doc);
    let doc = doc.trim_end_matches('\n');
    assert!(
        doc.ends_with('.') && (doc.starts_with(char::is_uppercase)),
        "bad docs for {field}: {doc:?}"
    );
    let default = default.parse::<serde_json::Value>().unwrap();

    let mut map = serde_json::Map::default();
    macro_rules! set {
        ($($key:literal: $value:tt),*$(,)?) => {{$(
            map.insert($key.into(), serde_json::json!($value));
        )*}};
    }
    set!("markdownDescription": doc);
    set!("default": default);

    match field_type {
        "bool" => set!("type": "boolean"),
        "usize" => set!("type": "integer", "minimum": 0),
        "String" => set!("type": "string"),
        "Vec<String>" | "Vec<Utf8PathBuf>" => set! {
            "type": "array",
            "items": { "type": "string" },
        },
        "FxHashMap<Box<str>, Box<[Box<str>]>>"
        | "FxHashMap<String, String>"
        | "FxHashMap<Box<str>, u16>" => set! {
            "type": "object",
        },
        "FxHashSet<String>" => set! {
            "type": "array",
            "items": { "type": "string" },
            "uniqueItems": true,
        },
        "Option<usize>" => set! {
            "type": ["null", "integer"],
            "minimum": 0,
        },
        "Option<u16>" => set! {
            "type": ["null", "integer"],
            "minimum": 0,
            "maximum": 0xFFFF,
        },
        "Option<bool>" => set! {
            "type": ["null", "boolean"],
        },
        "NumThreads" => set! {
            "anyOf": [
                {
                    "type": "number",
                    "minimum": 0,
                    "maximum": 0x00FF
                },
                {
                    "type": "string",
                    "enum": ["physical", "logical"],
                    "enumDescriptions": [
                        "Use the number of physical cores",
                        "Use the number of logical cores",
                    ],
                },
            ],
        },
        "Option<NumThreads>" => set! {
            "anyOf": [
                {
                    "type": "null"
                },
                {
                    "type": "string",
                    "enum": ["physical", "logical"],
                    "enumDescriptions": [
                        "Use the number of physical cores",
                        "Use the number of logical cores",
                    ],
                },
            ],
        },
        "NagaVersionConfig" => set! {
            "type": "string",
            "enum": ["0.14", "0.19", "0.22", "main"],
            "enumDescriptions": [
                "Naga version 14",
                "Naga version 19",
                "Naga version 22",
                "Version of Naga on main (most recent stable version)"
            ],
        },
        "InlayHintsTypeVerbosity" => set! {
            "type": "string",
            "enum": ["full", "compact", "inner"],
            "enumDescriptions": [
                "`ref<uniform, f32, read_write>`",
                "`ref<f32>`",
                "`f32`"
            ]
        },
        "TraceServer" => set! {
            "type": "string",
            "enum": ["off", "messages", "verbose"],
            "enumDescriptions": [
                "No traces",
                "Error only",
                "Full log"
            ]
        },
        _ => panic!("missing entry for {field_type}: {default} (field {field})"),
    }

    map.into()
}

macro_rules! _default_val {
    ($default:expr, $ty:ty) => {{
        let default_: $ty = $default;
        default_
    }};
}
use _default_val as default_val;

macro_rules! _default_str {
    ($default:expr, $ty:ty) => {{
        let val = default_val!($default, $ty);
        serde_json::to_string_pretty(&val).unwrap()
    }};
}
use _default_str as default_str;

macro_rules! _impl_for_config_data {
    (local, $(
            $(#[doc=$doc:literal])*
            $vis:vis $field:ident : $ty:ty = $default:expr,
        )*
    ) => {
        impl Config {
            $(
                $($doc)*
                #[allow(non_snake_case)]
                $vis fn $field(&self, source_root: Option<SourceRootId>) -> &$ty {
                    let mut source_root = source_root.as_ref();
                    if let Some(v) = self.client_config.0.local.$field.as_ref() {
                        return &v;
                    }

                    if let Some((user_config, _)) = self.user_config.as_ref() {
                        if let Some(v) = user_config.local.$field.as_ref() {
                            return &v;
                        }
                    }

                    &self.default_config.local.$field
                }
            )*
        }
    };
    (workspace, $(
            $(#[doc=$doc:literal])*
            $vis:vis $field:ident : $ty:ty = $default:expr,
        )*
    ) => {
        impl Config {
            $(
                $($doc)*
                #[allow(non_snake_case)]
                $vis fn $field(&self, source_root: Option<SourceRootId>) -> &$ty {
                    let mut source_root = source_root.as_ref();
                    if let Some(v) = self.client_config.0.workspace.$field.as_ref() {
                        return &v;
                    }

                    if let Some((user_config, _)) = self.user_config.as_ref() {
                        if let Some(v) = user_config.workspace.$field.as_ref() {
                            return &v;
                        }
                    }

                    &self.default_config.workspace.$field
                }
            )*
        }
    };
    (global, $(
            $(#[doc=$doc:literal])*
            $vis:vis $field:ident : $ty:ty = $default:expr,
        )*
    ) => {
        #[expect(non_snake_case, reason="Generated accessor mirrors user-facing schema keys.")]
        #[expect(clippy::ref_option, reason="Accessors intentionally return &Option<T> to avoid cloning.")]
        impl Config {
            $(
                $($doc)*
                $vis fn $field(&self) -> &$ty {
                    if let Some(value) = self.client.0.global.$field.as_ref() {
                        return value;
                    }

                    if let Some((user_config, _)) = self.user.as_ref() {
                        if let Some(value) = user_config.global.$field.as_ref() {
                            return value;
                        }
                    }

                    &self.default.global.$field
                }
            )*
        }
    };
    (client, $(
            $(#[doc=$doc:literal])*
            $vis:vis $field:ident : $ty:ty = $default:expr,
       )*
    ) => {
        impl Config {
            $(
                $($doc)*
                #[allow(non_snake_case)]
                $vis fn $field(&self) -> &$ty {
                    if let Some(v) = self.client_config.0.client.$field.as_ref() {
                        return &v;
                    }

                    &self.default_config.client.$field
                }
            )*
        }
    };
}
use _impl_for_config_data as impl_for_config_data;

macro_rules! _config_data {
    // modname is for the tests
    ($(#[doc=$dox:literal])* $modname:ident: struct $name:ident <- $input:ident -> {
        $(
            $(#[doc=$doc:literal])*
            $vis:vis $field:ident $(| $alias:ident)*: $ty:ty = $default:expr,
        )*
    }) => {
        /// Default config values for this grouping.
        #[expect(non_snake_case, reason = "Generated accessor mirrors user-facing schema keys.")]
        #[derive(Debug, Clone)]
        struct $name { $($field: $ty,)* }

        impl_for_config_data!{
            $modname,
            $(
                $vis $field : $ty = $default,
            )*
        }

        /// All fields `Option<T>`, `None` representing fields not set in a particular JSON blob.
        #[expect(non_snake_case, reason = "Fields mirror user-facing camelCase keys.")]
        #[derive(Clone, Default)]
        struct $input { $(
            $field: Option<$ty>,
        )* }

        impl std::fmt::Debug for $input {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut debug_struct = formatter.debug_struct(stringify!($input));
                $(
                    if let Some(val) = self.$field.as_ref() {
                        debug_struct.field(stringify!($field), val);
                    }
                )*
                debug_struct.finish()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name {$(
                    $field: default_val!($default, $ty),
                )*}
            }
        }

        impl $input {
            const FIELDS: &'static [&'static str] = &[$(stringify!($field)),*];

            fn from_json(json: &mut serde_json::Value, error_sink: &mut Vec<(String, serde_json::Error)>) -> Self {
                Self {$(
                    $field: get_field_json(
                        json,
                        error_sink,
                        stringify!($field),
                        None$(.or(Some(stringify!($alias))))*,
                    ),
                )*}
            }

            fn schema_fields(sink: &mut Vec<SchemaField>) {
                sink.extend_from_slice(&[
                    $({
                        let field = stringify!($field);
                        let ty = stringify!($ty);
                        let default = default_str!($default, $ty);

                        (field, ty, &[$($doc),*], default)
                    },)*
                ])
            }
        }

        #[cfg(test)]
        mod $modname {
            #[test]
             fn fields_are_sorted() {
                let fields = super::$input::FIELDS;
                fields.iter().zip(fields.iter().skip(1)).for_each(|(left, right)| {
                    assert!(left <= right, "{} <= {} does not hold", left, right);
                });
             }
         }
    };
}
use _config_data as config_data;

#[derive(Debug)]
pub enum ConfigErrorInner {
    Json {
        config_key: String,
        error: serde_json::Error,
    },
    ParseError {
        reason: String,
    },
}

#[cfg(test)]
fn manual(fields: &[SchemaField]) -> String {
    fields
        .iter()
        .fold(String::new(), |mut acc, (field, _ty, doc, default)| {
            let id = field.replace('_', ".");
            let name = format!("wgsl-analyzer.{id}");
            let doc = doc_comment_to_string(doc);
            if default.contains('\n') {
                format_to_accumulator!(
                    acc,
                    "## {name} \n\nDefault:\n```json\n{default}\n```\n\n{doc}\n"
                )
            } else {
                format_to_accumulator!(acc, "## {name}\n\nDefault: `{default}`\n\n{doc}\n")
            }
        })
}

fn doc_comment_to_string(doc: &[&str]) -> String {
    doc.iter()
        .map(|iterator| iterator.strip_prefix(' ').unwrap_or(iterator))
        .fold(String::new(), |mut out, line| {
            format_to_accumulator!(out, "{line}\n")
        })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use test_utils::{ensure_file_contents, project_root};

    use crate::config::{Config, FullConfigInput};

    #[test]
    fn generate_package_json_config() {
        let config_schema = Config::json_schema();

        let schema = format!("{config_schema:#}");
        let mut schema = schema
            .trim_start_matches('[')
            .trim_end_matches(']')
            .replace("  ", "\t")
            .replace('\n', "\n\t\t")
            .trim_start_matches('\n')
            .trim_end()
            .to_owned();
        schema.push_str(",\n");

        // Transform the asciidoc form link to markdown style.
        //
        // https://link[text] => [text](https://link)
        let url_matches = schema.match_indices("https://");
        let mut url_offsets = url_matches.map(|(index, _)| index).collect::<Vec<usize>>();
        url_offsets.reverse();
        for index in url_offsets {
            let link = &schema[index..];
            // matching on whitespace to ignore normal links
            if let Some(link_end) = link.find([' ', '['])
                && link.chars().nth(link_end) == Some('[')
                && let Some(link_text_end) = link.find(']')
            {
                let link_text = link[link_end..=link_text_end].to_string();

                schema.replace_range(((index + link_end)..=(index + link_text_end)), "");
                schema.insert(index, '(');
                schema.insert(index + link_end + 1, ')');
                schema.insert_str(index, &link_text);
            }
        }

        let package_json_path = project_root().join("editors/code/package.json");
        let mut package_json = fs::read_to_string(&package_json_path).unwrap();

        let start_marker = "\t\t\t\"title\": \"$generated-start\"\n\t\t\t},\n";
        let end_marker = "\t\t\t{\n\t\t\t\t\"title\": \"$generated-end\"\n\t\t\t}\n";

        let start = package_json.find(start_marker).unwrap() + start_marker.len();
        let end = package_json.find(end_marker).unwrap();

        let cleaned_package = remove_ws(&package_json[start..end]);
        let cleaned_schema = remove_ws(&schema);
        if !cleaned_package.contains(&cleaned_schema) {
            package_json.replace_range(start..end, &schema);
            ensure_file_contents(package_json_path.as_std_path(), &package_json);
        }
    }

    #[test]
    fn generate_config_documentation() {
        let docs_path = project_root().join("docs/book/src/configuration_generated.md");
        let expected = FullConfigInput::manual();
        ensure_file_contents(docs_path.as_std_path(), &expected);
    }

    fn remove_ws(text: &str) -> String {
        text.replace(char::is_whitespace, "")
    }
}
