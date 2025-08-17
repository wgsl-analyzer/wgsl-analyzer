use base_db::{FilePosition, FileRange, RangeInfo, SourceDatabase as _};
use hir::Semantics;
use hir_def::InFile;
use ide_db::RootDatabase;
use syntax::{AstNode as _, ast};

use crate::{NavigationTarget, markup::Markup};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverConfig {
    pub links_in_hover: bool,
    pub memory_layout: Option<MemoryLayoutHoverConfig>,
    pub documentation: bool,
    pub keywords: bool,
    pub format: HoverDocFormat,
    pub max_fields_count: Option<usize>,
    pub max_enum_variants_count: Option<usize>,
    pub max_subst_ty_len: SubstTyLen,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubstTyLen {
    Unlimited,
    LimitTo(usize),
    Hide,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MemoryLayoutHoverConfig {
    pub size: Option<MemoryLayoutHoverRenderKind>,
    pub offset: Option<MemoryLayoutHoverRenderKind>,
    pub alignment: Option<MemoryLayoutHoverRenderKind>,
    pub padding: Option<MemoryLayoutHoverRenderKind>,
    pub niches: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MemoryLayoutHoverRenderKind {
    Decimal,
    Hexadecimal,
    Both,
}

/// Contains the results when hovering over an item
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct HoverResult {
    pub markup: Markup,
    pub actions: Vec<HoverAction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HoverDocFormat {
    Markdown,
    PlainText,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum HoverAction {
    Implementation(FilePosition),
    Reference(FilePosition),
    GoToType(Vec<HoverGotoTypeData>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct HoverGotoTypeData {
    pub mod_path: String,
    pub navigation_target: NavigationTarget,
}

// Feature: Hover
//
// Shows additional information, like the type of an expression or the documentation for a definition when "focusing" code.
// Focusing is usually hovering with a mouse, but can also be triggered with a shortcut.
pub(crate) fn hover(
    database: &RootDatabase,
    file_range: FileRange,
    _config: &HoverConfig,
) -> Option<RangeInfo<HoverResult>> {
    let _semantics = &Semantics::new(database);
    let _file = database.parse(file_range.file_id).tree();
    None
}
