use std::{default, env, fmt, iter, ops::Not, sync::OnceLock};

use base_db::input::SourceRootId;
use hir::diagnostics::{DiagnosticsConfig, NagaVersion};
use hir_ty::ty::pretty::TypeVerbosity;
use ide::inlay_hints::{self, StructLayoutHints};
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
    line_index::OffsetEncoding,
    lsp::{
        capabilities::ClientCapabilities,
        ext::{WorkspaceSymbolSearchKind, WorkspaceSymbolSearchScope},
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

#[derive(Default, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintsConfig {
    pub enabled: bool,
    pub type_hints: bool,
    pub parameter_hints: bool,
    pub struct_layout_hints: bool,
    pub type_verbosity: InlayHintsTypeVerbosity,
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
    caps: ClientCapabilities,
    root_path: AbsPathBuf,
    // snippets: Vec<Snippet>,
    client_info: Option<ClientInfo>,

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
}

impl Config {
    #[must_use]
    #[inline]
    pub const fn data(&self) -> &ConfigData {
        &self.data
    }
}

// Delegate capability fetching methods
impl std::ops::Deref for Config {
    type Target = ClientCapabilities;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.caps
    }
}

#[derive(Default, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigData {
    pub custom_imports: FxHashMap<String, String>,
    pub shader_defs: FxHashSet<String>,
    pub trace: TraceConfig,
    pub inlay_hints: InlayHintsConfig,
    pub diagnostics: DiagnosticsConfig,
}

#[derive(Debug)]
pub enum ConfigErrorInner {}

#[derive(Clone, Debug, Default)]
pub struct ConfigErrors(Vec<Arc<ConfigErrorInner>>);

impl ConfigErrors {
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for ConfigErrors {
    #[inline]
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait method")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        #[expect(clippy::match_single_binding, reason = "wip")]
        #[expect(clippy::uninhabited_references, reason = "wip")]
        let errors = self
            .0
            .iter()
            .format_with("\n", |inner, formatter| match &**inner {
                // ConfigErrorInner::Json { config_key: key, error: e } => {
                //     f(key)?;
                //     f(&": ")?;
                //     f(e)
                // }
                // ConfigErrorInner::Toml { config_key: key, error: e } => {
                //     f(key)?;
                //     f(&": ")?;
                //     f(e)
                // }
                // ConfigErrorInner::ParseError { reason } => f(reason),
                _ => formatter(&""),
            });
        write!(
            f,
            "invalid config value{}:\n{}",
            if self.0.len() == 1 { "" } else { "s" },
            errors
        )
    }
}

impl std::error::Error for ConfigErrors {}

impl Config {
    #[inline]
    #[must_use]
    pub fn new(
        root_path: AbsPathBuf,
        caps: lsp_types::ClientCapabilities,
        workspace_roots: Vec<AbsPathBuf>,
        client_info: Option<lsp_types::ClientInfo>,
    ) -> Self {
        // static DEFAULT_CONFIG_DATA: OnceLock<&'static DefaultConfigData> = OnceLock::new();

        Self {
            data: ConfigData::default(),
            caps: ClientCapabilities::new(caps),
            // discovered_projects_from_filesystem: Vec::new(),
            // discovered_projects_from_command: Vec::new(),
            root_path,
            // snippets: Default::default(),
            workspace_roots,
            client_info: client_info.map(|it| ClientInfo {
                name: it.name,
                version: it
                    .version
                    .as_deref()
                    .map(Version::parse)
                    .and_then(Result::ok),
            }),
            // client_config: (FullConfigInput::default(), ConfigErrors(vec![])),
            // default_config: DEFAULT_CONFIG_DATA.get_or_init(|| Box::leak(Box::default())),
            // source_root_parent_map: Arc::new(FxHashMap::default()),
            // user_config: None,
            detached_files: Vec::default(),
            validation_errors: ConfigErrors::default(),
            // watoml_file: Default::default(),
        }
    }

    /// Given `change` this generates a new `Config`, thereby collecting errors of type `ConfigError`.
    /// If there are changes that have global/client level effect, the last component of the return type
    /// will be set to `true`, which should be used by the `GlobalState` to update itself.
    #[inline]
    #[must_use]
    pub fn apply_change(
        &self,
        _change: ConfigChange,
    ) -> (Self, ConfigErrors, bool) {
        let (config, should_update) = (self.clone(), true);

        (config, ConfigErrors(vec![]), should_update)
    }

    #[inline]
    #[must_use]
    pub const fn root_path(&self) -> &AbsPathBuf {
        &self.root_path
    }

    #[inline]
    #[must_use]
    pub const fn caps(&self) -> &ClientCapabilities {
        &self.caps
    }

    #[inline]
    #[must_use]
    pub const fn main_loop_num_threads(&self) -> usize {
        1
    }

    #[inline]
    #[must_use]
    pub fn completion(
        &self,
        source_root: Option<SourceRootId>,
    ) -> CompletionConfig<'_> {
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
            exclude_traits: <_>::default(),
        }
    }
}

#[derive(Default, Debug)]
pub struct ConfigChange {
    user_config_change: Option<Arc<str>>,
    client_config_change: Option<serde_json::Value>,
    source_map_change: Option<Arc<FxHashMap<SourceRootId, SourceRootId>>>,
}

impl ConfigChange {
    /// # Panics
    ///
    /// Panics if double writing
    #[inline]
    pub fn change_user_config(
        &mut self,
        content: Option<Arc<str>>,
    ) {
        assert!(self.user_config_change.is_none()); // Otherwise it is a double write.
        self.user_config_change = content;
    }

    #[inline]
    pub fn change_client_config(
        &mut self,
        change: serde_json::Value,
    ) {
        self.client_config_change = Some(change);
    }

    /// # Panics
    ///
    /// Panics if double writing
    #[inline]
    pub fn change_source_root_parent_map(
        &mut self,
        source_root_map: Arc<FxHashMap<SourceRootId, SourceRootId>>,
    ) {
        assert!(self.source_map_change.is_none());
        self.source_map_change = Some(source_root_map);
    }
}

impl ConfigData {
    fn try_update(
        &mut self,
        value: serde_json::Value,
    ) -> Result<(), serde_json::Error> {
        *self = serde_json::from_value(value)?;
        Ok(())
    }

    #[inline]
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

    #[inline]
    #[must_use]
    pub const fn diagnostics(
        &self,
        source_root: Option<SourceRootId>,
    ) -> DiagnosticsConfig {
        DiagnosticsConfig {
            enabled: true,
            type_errors: self.diagnostics.type_errors,
            naga_parsing_errors: self.diagnostics.naga_parsing_errors,
            naga_validation_errors: self.diagnostics.naga_validation_errors,
            naga_version: match self.diagnostics.naga_version {
                NagaVersion::Naga14 => NagaVersion::Naga14,
                NagaVersion::Naga19 => NagaVersion::Naga19,
                NagaVersion::Naga22 => NagaVersion::Naga22,
                NagaVersion::NagaMain => NagaVersion::NagaMain,
            },
        }
    }

    #[inline]
    #[must_use]
    pub fn inlay_hints(&self) -> inlay_hints::InlayHintsConfig {
        inlay_hints::InlayHintsConfig {
            enabled: self.inlay_hints.enabled,
            type_hints: self.inlay_hints.type_hints,
            parameter_hints: self.inlay_hints.parameter_hints,
            struct_layout_hints: self
                .inlay_hints
                .struct_layout_hints
                .then_some(StructLayoutHints::Offset),
            type_verbosity: match self.inlay_hints.type_verbosity {
                InlayHintsTypeVerbosity::Full => TypeVerbosity::Full,
                InlayHintsTypeVerbosity::Compact => TypeVerbosity::Compact,
                InlayHintsTypeVerbosity::Inner => TypeVerbosity::Inner,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum WgslfmtConfig {
    Wgslfmt {
        extra_args: Vec<String>,
        enable_range_formatting: bool,
    },
    CustomCommand {
        command: String,
        args: Vec<String>,
    },
}
