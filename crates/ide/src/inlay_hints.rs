use std::fmt;

use ast::Expression as AstExpression;
use base_db::{FileId, FileRange, TextRange};
use hir::{Field, HasSource as _, Semantics};
use hir_def::{InFile, data::FieldId, module_data::Name};
use hir_ty::{
    function::FunctionDetails,
    infer::ResolvedCall,
    layout::{FieldLayout, LayoutAddressSpace},
    ty::pretty::{TypeVerbosity, pretty_type_with_verbosity},
};
use ide_db::text_edit::TextEdit;
use itertools::Itertools as _;
use rowan::NodeOrToken;
use rustc_hash::FxHashSet;
use smallvec::{SmallVec, smallvec};
use std::mem;
use syntax::{AstChildren, AstNode as _, HasName as _, SyntaxNode, ast};

use crate::RootDatabase;

pub struct InlayHintsConfig {
    pub render_colons: bool,
    pub enabled: bool,
    pub type_hints: bool,
    pub parameter_hints: bool,
    pub struct_layout_hints: Option<StructLayoutHints>,
    pub type_verbosity: TypeVerbosity,
    pub fields_to_resolve: InlayFieldsToResolve,
}

#[derive(Clone, Copy, Debug)]
pub enum StructLayoutHints {
    Offset,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InlayKind {
    Type,
    Parameter,
    StructLayout,
}

#[derive(Debug, Hash)]
pub enum InlayHintPosition {
    Before,
    After,
}

#[derive(Debug)]
pub struct InlayHint {
    /// The text range this inlay hint applies to.
    pub range: TextRange,
    pub position: InlayHintPosition,
    pub pad_left: bool,
    pub pad_right: bool,
    /// The kind of this inlay hint.
    pub kind: InlayKind,
    /// The actual label to show in the inlay hint.
    pub label: InlayHintLabel,
    /// Text edit to apply when "accepting" this inlay hint.
    pub text_edit: Option<LazyProperty<TextEdit>>,
    /// Range to recompute inlay hints when trying to resolve for this hint. If this is none, the
    /// hint does not support resolving.
    pub resolve_parent: Option<TextRange>,
}

/// A type signaling that a value is either computed, or is available for computation.
#[derive(Clone, Debug)]
pub enum LazyProperty<T> {
    Computed(T),
    Lazy,
}

impl<T> LazyProperty<T> {
    #[inline]
    pub fn computed(self) -> Option<T> {
        match self {
            Self::Computed(value) => Some(value),
            Self::Computed(_) | Self::Lazy => None,
        }
    }

    #[inline]
    pub const fn is_lazy(&self) -> bool {
        matches!(self, Self::Lazy)
    }
}

impl std::hash::Hash for InlayHint {
    #[inline]
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self.range.hash(state);
        self.position.hash(state);
        self.pad_left.hash(state);
        self.pad_right.hash(state);
        // self.kind.hash(state);
        self.label.hash(state);
        mem::discriminant(&self.text_edit).hash(state);
    }
}

impl InlayHint {
    fn closing_paren_after(
        kind: InlayKind,
        range: TextRange,
    ) -> Self {
        Self {
            range,
            kind,
            label: InlayHintLabel::from(")"),
            text_edit: None,
            position: InlayHintPosition::After,
            pad_left: false,
            pad_right: false,
            resolve_parent: None,
        }
    }
}
#[derive(Debug, Hash)]
pub enum InlayTooltip {
    String(String),
    Markdown(String),
}

#[derive(Default, Hash)]
pub struct InlayHintLabel {
    pub parts: SmallVec<[InlayHintLabelPart; 1]>,
}

impl InlayHintLabel {
    #[inline]
    pub fn simple<Stringy: Into<String>>(
        stringy: Stringy,
        tooltip: Option<LazyProperty<InlayTooltip>>,
        linked_location: Option<LazyProperty<FileRange>>,
    ) -> Self {
        Self {
            parts: smallvec![InlayHintLabelPart {
                text: stringy.into(),
                linked_location,
                tooltip
            }],
        }
    }

    #[inline]
    pub fn prepend_str(
        &mut self,
        string: &str,
    ) {
        match &mut *self.parts {
            [
                InlayHintLabelPart {
                    text,
                    linked_location: None,
                    tooltip: None,
                },
                ..,
            ] => text.insert_str(0, string),
            _ => self.parts.insert(
                0,
                InlayHintLabelPart {
                    text: string.into(),
                    linked_location: None,
                    tooltip: None,
                },
            ),
        }
    }

    #[inline]
    pub fn append_str(
        &mut self,
        string: &str,
    ) {
        match &mut *self.parts {
            [
                ..,
                InlayHintLabelPart {
                    text,
                    linked_location: None,
                    tooltip: None,
                },
            ] => text.push_str(string),
            _ => self.parts.push(InlayHintLabelPart {
                text: string.into(),
                linked_location: None,
                tooltip: None,
            }),
        }
    }

    #[inline]
    pub fn append_part(
        &mut self,
        part: InlayHintLabelPart,
    ) {
        if part.linked_location.is_none() && part.tooltip.is_none() {
            if let Some(InlayHintLabelPart {
                text,
                linked_location: None,
                tooltip: None,
            }) = self.parts.last_mut()
            {
                text.push_str(&part.text);
                return;
            }
        }
        self.parts.push(part);
    }
}

impl From<String> for InlayHintLabel {
    #[inline]
    fn from(value: String) -> Self {
        Self {
            parts: smallvec![InlayHintLabelPart {
                text: value,
                linked_location: None,
                tooltip: None
            }],
        }
    }
}

impl From<&str> for InlayHintLabel {
    #[inline]
    fn from(value: &str) -> Self {
        Self {
            parts: smallvec![InlayHintLabelPart {
                text: value.into(),
                linked_location: None,
                tooltip: None
            }],
        }
    }
}

impl fmt::Display for InlayHintLabel {
    #[inline]
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.parts.iter().map(|part| &part.text).format(""))
    }
}

impl fmt::Debug for InlayHintLabel {
    #[inline]
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_list().entries(&self.parts).finish()
    }
}

pub struct InlayHintLabelPart {
    pub text: String,
    /// Source location represented by this label part. The client will use this to fetch the part's
    /// hover tooltip, and Ctrl+Clicking the label part will navigate to the definition the location
    /// refers to (not necessarily the location itself).
    /// When setting this, no tooltip must be set on the containing hint, or VS Code will display
    /// them both.
    pub linked_location: Option<LazyProperty<FileRange>>,
    /// The tooltip to show when hovering over the inlay hint, this may invoke other actions like
    /// hover requests to show.
    pub tooltip: Option<LazyProperty<InlayTooltip>>,
}

impl std::hash::Hash for InlayHintLabelPart {
    #[inline]
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self.text.hash(state);
        self.linked_location.is_some().hash(state);
        self.tooltip.is_some().hash(state);
    }
}

impl fmt::Debug for InlayHintLabelPart {
    #[inline]
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self {
                text,
                linked_location: None,
                tooltip: None | Some(LazyProperty::Lazy),
            } => text.fmt(f),
            Self {
                text,
                linked_location,
                tooltip,
            } => f
                .debug_struct("InlayHintLabelPart")
                .field("text", text)
                .field("linked_location", linked_location)
                .field(
                    "tooltip",
                    &tooltip.as_ref().map_or("", |property| match property {
                        LazyProperty::Computed(
                            InlayTooltip::String(text) | InlayTooltip::Markdown(text),
                        ) => text,
                        LazyProperty::Lazy => "",
                    }),
                )
                .finish(),
        }
    }
}

pub(crate) fn inlay_hints(
    database: &RootDatabase,
    file_id: FileId,
    range_limit: Option<TextRange>,
    config: &InlayHintsConfig,
) -> Vec<InlayHint> {
    let sema = Semantics::new(database);
    let file = sema.parse(file_id);

    let mut hints = Vec::new();

    if let Some(range_limit) = range_limit {
        match file.syntax().covering_element(range_limit) {
            NodeOrToken::Token(_) => return hints,
            NodeOrToken::Node(n) => {
                for node in n
                    .descendants()
                    .filter(|descendant| range_limit.contains_range(descendant.text_range()))
                {
                    get_hints(&mut hints, file_id, &sema, config, &node);
                }

                get_struct_layout_hints(&mut hints, file_id, &sema, config);
            },
        }
    } else {
        for node in file.syntax().descendants() {
            get_hints(&mut hints, file_id, &sema, config, &node);
        }

        get_struct_layout_hints(&mut hints, file_id, &sema, config);
    }

    hints
}

fn get_struct_layout_hints(
    hints: &mut Vec<InlayHint>,
    file_id: FileId,
    sema: &Semantics<'_>,
    config: &InlayHintsConfig,
) -> Option<()> {
    let display_kind = config.struct_layout_hints?;

    let module_info = sema.database.module_info(file_id.into());

    for r#struct in module_info.structs() {
        let r#struct = sema
            .database
            .intern_struct(InFile::new(file_id.into(), r#struct));
        let fields = sema.database.field_types(r#struct);

        let address_space = if sema
            .database
            .struct_is_used_in_uniform(r#struct, file_id.into())
        {
            LayoutAddressSpace::Uniform
        } else {
            LayoutAddressSpace::Storage
        };

        hir_ty::layout::struct_member_layout(
            &fields,
            sema.database,
            address_space,
            |field, field_layout| {
                let FieldLayout { offset, .. } = field_layout;
                let field = Field {
                    id: FieldId { r#struct, field },
                };

                let source = field.source(sema.database)?.value;

                // this is only necessary, because the field syntax nodes include the whitespace to the next line...
                let actual_last_token = std::iter::successors(
                    source.syntax().last_token(),
                    rowan::SyntaxToken::prev_token, // spellchecker:disable-line
                )
                .find(|token| !token.kind().is_trivia())?;
                let range = TextRange::new(
                    source.syntax().text_range().start(),
                    actual_last_token.text_range().end(),
                );

                hints.push(InlayHint {
                    range,
                    position: InlayHintPosition::After,
                    pad_left: false,
                    pad_right: false,
                    kind: InlayKind::StructLayout,
                    label: match display_kind {
                        StructLayoutHints::Offset => format!("{offset}").into(),
                    },
                    text_edit: None,
                    resolve_parent: Some(range),
                });

                Some(())
            },
        );
    }

    Some(())
}

fn get_hints(
    hints: &mut Vec<InlayHint>,
    file_id: FileId,
    sema: &Semantics<'_>,
    config: &InlayHintsConfig,
    node: &SyntaxNode,
) -> Option<()> {
    if let Some(expression) = AstExpression::cast(node.clone()) {
        match &expression {
            AstExpression::FunctionCall(function_call_expression) => {
                if !config.parameter_hints {
                    return None;
                }
                function_hints(
                    sema,
                    file_id,
                    node,
                    &expression,
                    function_call_expression.parameters()?.arguments(),
                    hints,
                )?;
            },
            AstExpression::TypeInitializer(type_initialiser_expression) => {
                if !config.parameter_hints {
                    return None;
                }
                // Show hints for the built-in initializers.
                // `vec4(xyz: val1, w: val2)` could also be
                // `vec4(xy: val1, zw: val2)` without hints
                function_hints(
                    sema,
                    file_id,
                    node,
                    &expression,
                    type_initialiser_expression.arguments()?.arguments(),
                    hints,
                )?;
            },
            AstExpression::InfixExpression(_)
            | AstExpression::PrefixExpression(_)
            | AstExpression::Literal(_)
            | AstExpression::ParenthesisExpression(_)
            | AstExpression::FieldExpression(_)
            | AstExpression::IndexExpression(_)
            | AstExpression::PathExpression(_)
            | AstExpression::BitcastExpression(_)
            | AstExpression::InvalidFunctionCall(_) => {},
        }
    } else if let Some((binding, r#type)) = ast::VariableStatement::cast(node.clone())
        .and_then(|statement| Some((statement.binding()?, statement.ty())))
        .or_else(|| {
            let statement = ast::GlobalConstantDeclaration::cast(node.clone())?;
            Some((statement.binding()?, statement.ty()))
        })
        .or_else(|| {
            let statement = ast::GlobalVariableDeclaration::cast(node.clone())?;
            Some((statement.binding()?, statement.ty()))
        })
    {
        if !config.type_hints {
            return None;
        }
        if r#type.is_none() {
            let container = sema.find_container(file_id.into(), node)?;
            let r#type = sema.analyze(container).type_of_binding(&binding)?;

            let label = pretty_type_with_verbosity(sema.database, r#type, config.type_verbosity);
            hints.push(InlayHint {
                range: binding.name()?.ident_token()?.text_range(),
                position: InlayHintPosition::After,
                pad_left: !config.render_colons,
                pad_right: false,
                kind: InlayKind::Type,
                label: label.into(),
                text_edit: None,
                resolve_parent: None,
            });
        }
    }

    Some(())
}

fn function_hints(
    sema: &Semantics<'_>,
    file_id: FileId,
    node: &SyntaxNode,
    expression: &AstExpression,
    parameter_expressions: AstChildren<AstExpression>,
    hints: &mut Vec<InlayHint>,
) -> Option<()> {
    let container = sema.find_container(file_id.into(), node)?;
    let analyzed = sema.analyze(container);
    let expression = analyzed.expression_id(expression)?;
    let resolved = analyzed.infer.call_resolution(expression)?;
    let func = match resolved {
        ResolvedCall::Function(func) => func.lookup(analyzed.database),
        ResolvedCall::OtherTypeInitializer(_) => return None,
    };
    let param_hints = func
        .parameter_names()
        .zip(parameter_expressions)
        .filter(|&(name, _)| !Name::is_missing(name))
        .filter(|(param_name, expression)| {
            !should_hide_param_name_hint(&func, param_name, expression)
        })
        .map(|(param_name, expression)| InlayHint {
            range: expression.syntax().text_range(),
            position: InlayHintPosition::After,
            pad_left: false,
            pad_right: false,
            kind: InlayKind::Parameter,
            label: param_name.into(),
            text_edit: None,
            resolve_parent: None,
        });
    hints.extend(param_hints);
    Some(())
}

// taken from https://github.com/rust-lang/rust-analyzer/blob/7308b3ef413cad8c211e239d32c9fab29ae2e664/crates/ide/src/inlay_hints.rs#L422

fn should_hide_param_name_hint(
    func: &FunctionDetails,
    param_name: &str,
    expression: &AstExpression,
) -> bool {
    is_argument_similar_to_param_name(expression, param_name)
        || (func.parameters.len() == 1 && is_obvious_parameter(param_name))
}

fn is_argument_similar_to_param_name(
    expression: &AstExpression,
    param_name: &str,
) -> bool {
    let Some(argument) = get_string_representation(expression) else {
        return false;
    };

    // std is honestly too panic happy...
    let str_split_at = |str: &str, at| str.is_char_boundary(at).then(|| argument.split_at(at));

    let param_name = param_name.trim_start_matches('_');
    let argument = argument.trim_start_matches('_');

    match str_split_at(argument, param_name.len()) {
        Some((prefix, rest)) if prefix.eq_ignore_ascii_case(param_name) => {
            return rest.is_empty() || rest.starts_with('_');
        },
        _ => (),
    }
    match argument
        .len()
        .checked_sub(param_name.len())
        .and_then(|at| str_split_at(argument, at))
    {
        Some((rest, suffix)) if param_name.eq_ignore_ascii_case(suffix) => {
            return rest.is_empty() || rest.ends_with('_');
        },
        _ => (),
    }

    // mixed camelCase/snake_case
    if compare_ignore_case_convention(argument, param_name) {
        return true;
    }

    false
}

fn is_obvious_parameter(param_name: &str) -> bool {
    let is_obvious_param_name = matches!(param_name, "predicate" | "value");
    param_name.len() == 1 || is_obvious_param_name
}

fn compare_ignore_case_convention(
    argument: &str,
    param_name: &str,
) -> bool {
    argument
        .chars()
        .filter(|&character| character != '_')
        .zip(param_name.chars().filter(|&character| character != '_'))
        .all(|(argument, param_name)| argument.eq_ignore_ascii_case(&param_name))
}

fn get_string_representation(expression: &AstExpression) -> Option<String> {
    match expression {
        AstExpression::PathExpression(expression) => {
            Some(expression.name_ref()?.text().as_str().to_owned())
        },
        AstExpression::PrefixExpression(expression) => {
            get_string_representation(&expression.expression()?)
        },
        AstExpression::FieldExpression(expression) => {
            Some(expression.name_ref()?.text().as_str().to_owned())
        },
        AstExpression::InfixExpression(_)
        | AstExpression::Literal(_)
        | AstExpression::ParenthesisExpression(_)
        | AstExpression::FunctionCall(_)
        | AstExpression::TypeInitializer(_)
        | AstExpression::IndexExpression(_)
        | AstExpression::BitcastExpression(_)
        | AstExpression::InvalidFunctionCall(_) => None,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InlayFieldsToResolve {
    pub resolve_text_edits: bool,
    pub resolve_hint_tooltip: bool,
    pub resolve_label_tooltip: bool,
    pub resolve_label_location: bool,
    pub resolve_label_command: bool,
}

impl InlayFieldsToResolve {
    #[must_use]
    #[inline]
    pub fn from_client_capabilities(client_capability_fields: &FxHashSet<&str>) -> Self {
        Self {
            resolve_text_edits: client_capability_fields.contains("textEdits"),
            resolve_hint_tooltip: client_capability_fields.contains("tooltip"),
            resolve_label_tooltip: client_capability_fields.contains("label.tooltip"),
            resolve_label_location: client_capability_fields.contains("label.location"),
            resolve_label_command: client_capability_fields.contains("label.command"),
        }
    }

    #[must_use]
    #[inline]
    pub const fn empty() -> Self {
        Self {
            resolve_text_edits: false,
            resolve_hint_tooltip: false,
            resolve_label_tooltip: false,
            resolve_label_location: false,
            resolve_label_command: false,
        }
    }
}
