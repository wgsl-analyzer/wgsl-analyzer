use std::collections::{HashMap, HashSet};

use hir::diagnostics;
use hir_ty::ty::pretty::TypeVerbosity;
use ide::inlay_hints;
use ide::inlay_hints::StructLayoutHints;
use serde::Deserialize;

use crate::line_index::OffsetEncoding;

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

#[derive(Default, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub custom_imports: HashMap<String, String>,
    pub shader_defs: HashSet<String>,
    pub trace: TraceConfig,
    pub inlay_hints: InlayHintsConfig,
    pub diagnostics: DiagnosticsConfig,
}

#[derive(Default, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticsConfig {
    pub type_errors: bool,
    pub naga_parsing_errors: bool,
    pub naga_validation_errors: bool,
    pub naga_version: NagaVersion,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub enum NagaVersion {
    #[serde(rename = "0.14")]
    Naga14,
    #[serde(rename = "0.19")]
    #[default]
    Naga19,
    #[serde(rename = "0.22")]
    Naga22,
    #[serde(rename = "main")]
    NagaMain,
}

impl Config {
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
    pub const fn diagnostics(&self) -> diagnostics::DiagnosticsConfig {
        diagnostics::DiagnosticsConfig {
            type_errors: self.diagnostics.type_errors,
            naga_parsing_errors: self.diagnostics.naga_parsing_errors,
            naga_validation_errors: self.diagnostics.naga_validation_errors,
            naga_version: match self.diagnostics.naga_version {
                NagaVersion::Naga14 => diagnostics::NagaVersion::Naga14,
                NagaVersion::Naga19 => diagnostics::NagaVersion::Naga19,
                NagaVersion::Naga22 => diagnostics::NagaVersion::Naga22,
                NagaVersion::NagaMain => diagnostics::NagaVersion::NagaMain,
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

    #[inline]
    #[must_use]
    pub const fn offset_encoding(&self) -> OffsetEncoding {
        OffsetEncoding::Utf8 // TODO do we need to check whether it is supported?
    }
}
