use std::{default, env, fmt, iter, ops::Not, sync::OnceLock};

use base_db::input::SourceRootId;
use hir::diagnostics::{DiagnosticsConfig, NagaVersion};
use hir_ty::ty::pretty::TypeVerbosity;
use ide::{
    HoverConfig, HoverDocFormat, MemoryLayoutHoverRenderKind,
    inlay_hints::{self, StructLayoutHints},
};
use ide_completion::{CompletionConfig, CompletionFieldsToResolve};
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
use crate::{
    diagnostics::DiagnosticsMapConfig,
    lsp::{
        capabilities::ClientCapabilities,
        extensions::{WorkspaceSymbolSearchKind, WorkspaceSymbolSearchScope},
    },
};
use camino::{Utf8Path, Utf8PathBuf};
use itertools::Itertools as _;
use rustc_hash::{FxHashMap, FxHashSet};
use semver::Version;
use serde::{
    Deserialize, Serialize,
    de::{DeserializeOwned, Error},
};
use stdx::format_to_acc;
use triomphe::Arc;
use vfs::{AbsPath, AbsPathBuf, VfsPath};

#[derive(Default, Clone, Debug, Deserialize)]
pub struct TraceConfig {
    pub extension: bool,
    pub server: bool,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum InlayHintsTypeVerbosity {
    Full, // ref<uniform, f32, read_write>,
    #[default]
    Compact, // ref<f32>,
    Inner, // f32
}

#[derive(Clone, Debug)]
struct ClientInfo {
    name: String,
    version: Option<Version>,
}

#[derive(Clone, Debug)]
pub struct Config {
    data: ConfigData,
    /// The workspace roots as registered by the LSP client
    workspace_roots: Vec<AbsPathBuf>,
    capabilities: ClientCapabilities,
    root_path: AbsPathBuf,
    // snippets: Vec<Snippet>,
    client_info: Option<ClientInfo>,
    diagnostics_enable: bool,

    // default_config: &'static DefaultConfigData,
    // /// Config node that obtains its initial value during the server initialization and
    // /// by receiving a `lsp_types::notification::DidChangeConfiguration`.
    // client_config: (FullConfigInput, ConfigErrors),

    // /// Config node whose values apply to **every** Rust project.
    // user_config: Option<(GlobalWorkspaceLocalConfigInput, ConfigErrors)>,

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
    pub const fn data(&self) -> &ConfigData {
        &self.data
    }

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
pub struct ConfigData {
    pub custom_imports: FxHashMap<String, String>,
    pub shader_defs: FxHashSet<String>,
    pub trace: TraceConfig,
    pub inlay_hints: InlayHintsConfig,
    pub diagnostics: DiagnosticsConfig,

    /// How many worker threads to handle priming caches. The default `0` means to pick automatically.
    pub cache_priming_num_threads: NumThreads,
    /// How many worker threads in the main loop. The default `null` means to pick automatically.
    pub num_threads: Option<NumThreads>,
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

#[derive(Debug)]
pub enum ConfigErrorInner {}

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
        #[expect(clippy::match_single_binding, reason = "wip")]
        #[expect(clippy::uninhabited_references, reason = "wip")]
        let errors = self
            .0
            .iter()
            .format_with("\n", |inner, formatter| match &**inner {
                // ConfigErrorInner::Json { config_key: key, error: e } => {
                //     formatter(key)?;
                //     formatter(&": ")?;
                //     formatter(e)
                // }
                // ConfigErrorInner::Toml { config_key: key, error: e } => {
                //     formatter(key)?;
                //     formatter(&": ")?;
                //     formatter(e)
                // }
                // ConfigErrorInner::ParseError { reason } => formatter(reason),
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
        // static DEFAULT_CONFIG_DATA: OnceLock<&'static DefaultConfigData> = OnceLock::new();

        Self {
            data: ConfigData {
                custom_imports: FxHashMap::default(),
                shader_defs: FxHashSet::default(),
                trace: TraceConfig::default(),
                inlay_hints: InlayHintsConfig::default(),
                diagnostics: DiagnosticsConfig::default(),
                cache_priming_num_threads: NumThreads::Physical,
                num_threads: None,
            },
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
            // client_config: (FullConfigInput::default(), ConfigErrors(vec![])),
            // default_config: DEFAULT_CONFIG_DATA.get_or_init(|| Box::leak(Box::default())),
            // source_root_parent_map: Arc::new(FxHashMap::default()),
            // user_config: None,
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

    /// Given `change` this generates a new `Config`, thereby collecting errors of type `ConfigError`.
    /// If there are changes that have global/client level effect, the last component of the return type
    /// will be set to `true`, which should be used by the `GlobalState` to update itself.
    #[must_use]
    pub fn apply_change(
        &self,
        _change: ConfigChange,
    ) -> (Self, ConfigErrors, bool) {
        let (config, should_update) = (self.clone(), true);

        (config, ConfigErrors(vec![]), should_update)
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
        match self.data.cache_priming_num_threads {
            NumThreads::Concrete(0) | NumThreads::Physical => num_cpus::get_physical(),
            NumThreads::Concrete(number) => number,
            NumThreads::Logical => num_cpus::get(),
        }
    }

    #[must_use]
    pub fn main_loop_number_of_threads(&self) -> usize {
        match self.data.num_threads {
            Some(NumThreads::Concrete(0) | NumThreads::Physical) | None => num_cpus::get_physical(),
            Some(NumThreads::Concrete(number)) => number,
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
            render_colons: self.data.inlay_hints.render_colons,
            enabled: self.data.inlay_hints.enabled,
            type_hints: self.data.inlay_hints.type_hints,
            parameter_hints: self.data.inlay_hints.parameter_hints,
            struct_layout_hints: self
                .data
                .inlay_hints
                .struct_layout_hints
                .then_some(StructLayoutHints::Offset),
            type_verbosity: match self.data.inlay_hints.type_verbosity {
                InlayHintsTypeVerbosity::Full => TypeVerbosity::Full,
                InlayHintsTypeVerbosity::Compact => TypeVerbosity::Compact,
                InlayHintsTypeVerbosity::Inner => TypeVerbosity::Inner,
            },
            fields_to_resolve: ide::inlay_hints::InlayFieldsToResolve::from_client_capabilities(
                &client_capability_fields,
            ),
        }
    }
    fn try_update(
        &mut self,
        value: serde_json::Value,
    ) -> Result<(), serde_json::Error> {
        self.data = serde_json::from_value(value)?;
        Ok(())
    }

    pub fn update(
        &mut self,
        value: &serde_json::Value,
    ) {
        if value.is_null() {
            return;
        }
        if let Err(error) = self.try_update(value.clone()) {
            tracing::error!("Failed to update config: {:?}", error);
            tracing::error!("Received JSON: {}", value.to_string());
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
    pub const fn diagnostics(
        &self,
        source_root: Option<SourceRootId>,
    ) -> DiagnosticsConfig {
        DiagnosticsConfig {
            enabled: true,
            type_errors: self.data.diagnostics.type_errors,
            naga_parsing_errors: self.data.diagnostics.naga_parsing_errors,
            naga_validation_errors: self.data.diagnostics.naga_validation_errors,
            naga_version: match self.data.diagnostics.naga_version {
                NagaVersion::Naga14 => NagaVersion::Naga14,
                NagaVersion::Naga19 => NagaVersion::Naga19,
                NagaVersion::Naga22 => NagaVersion::Naga22,
                NagaVersion::NagaMain => NagaVersion::NagaMain,
            },
        }
    }

    #[must_use]
    pub const fn typing_trigger_chars(&self) -> &'static str {
        "=.+"
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NumThreads {
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
