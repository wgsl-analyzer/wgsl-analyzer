use base_db::{FilePosition, FileRange, RangeInfo, SourceDatabase as _};
use hir::{Definition, HirDatabase as _, Semantics};
use hir_def::{database::DefDatabase as _, item_tree::Name};
use hir_ty::{
    builtins::Builtin,
    infer::ResolvedCall,
    ty::pretty::{pretty_fn, pretty_type},
};
use ide_db::RootDatabase;
use syntax::{AstNode as _, SyntaxKind, ast};

use crate::{NavigationTarget, helpers, markup::Markup};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverConfig {
    pub links_in_hover: bool,
    pub memory_layout: Option<MemoryLayoutHoverConfig>,
    pub documentation: bool,
    pub keywords: bool,
    pub format: HoverDocFormat,
    pub max_fields_count: Option<usize>,
    pub max_enum_variants_count: Option<usize>,
    pub max_substitution_type_length: SubstitutionTypeLength,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubstitutionTypeLength {
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

/// Contains the results when hovering over an item.
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
    let semantics = &Semantics::new(database);
    let file_id = database.editioned_file_id(file_range.file_id);
    let file = database.parse(file_id).tree();
    let token = file.syntax().token_at_offset(file_range.range.start());

    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "infeasible to list all cases"
    )]
    let token = helpers::pick_best_token(token, |token| match token {
        SyntaxKind::Identifier => 2,
        kind if kind.is_trivia() => 0,
        _ => 1,
    })?;

    let range = token.text_range();

    // Try resolving as a user-defined definition first
    if let Some(definition) = Definition::from_token(semantics, file_id.into(), &token) {
        if let Some(markup_text) = definition.hover_text(database) {
            let mut hover_content = String::new();

            // Add doc comments above the code block if present
            if let Some(doc) = definition.doc_comments(database) {
                hover_content.push_str(&doc);
                hover_content.push_str("\n\n---\n\n");
            }

            hover_content.push_str(&format!("```wgsl\n{markup_text}\n```"));

            return Some(RangeInfo::new(
                range,
                HoverResult {
                    markup: Markup::from(hover_content),
                    actions: Vec::new(),
                },
            ));
        }
    }

    // Fall back to expression type hover (e.g., `.x` on a vec3 — swizzle access)
    if let Some(parent) = token.parent() {
        if let Some(field_expr) = ast::FieldExpression::cast(parent.clone()) {
            let expr = ast::Expression::FieldExpression(field_expr);
            let container = semantics.find_container(file_id.into(), expr.syntax())?;
            let analyzer = semantics.analyze(container.as_def_with_body_id()?);
            if let Some(ty) = analyzer.type_of_expression(&expr) {
                let markup_text = pretty_type(database, ty);
                return Some(RangeInfo::new(
                    range,
                    HoverResult {
                        markup: Markup::fenced_block(&markup_text),
                        actions: Vec::new(),
                    },
                ));
            }
        }
    }

    // Check if hovering over an attribute name (e.g., @group, @binding, @vertex)
    if token.kind() == SyntaxKind::Identifier {
        if let Some(parent) = token.parent() {
            if ast::Attribute::cast(parent).is_some() {
                if let Some(description) = attribute_description(token.text()) {
                    return Some(RangeInfo::new(
                        range,
                        HoverResult {
                            markup: Markup::from(description),
                            actions: Vec::new(),
                        },
                    ));
                }
            }
        }
    }

    // Fall back to builtin lookup for functions like abs, dot, clamp, etc.
    if token.kind() == SyntaxKind::Identifier {
        let name = Name::from(token.text());
        if let Some(builtin) = Builtin::for_name(database, &name) {
            // Try to resolve the specific overload if this is a call site
            if let Some(markup_text) =
                try_resolve_call_at_token(semantics, file_id.into(), &token, database)
            {
                return Some(RangeInfo::new(
                    range,
                    HoverResult {
                        markup: Markup::fenced_block(&markup_text),
                        actions: Vec::new(),
                    },
                ));
            }

            // Fall back: try exact overload match first (all args present & matching)
            let arg_types = collect_call_arg_types(semantics, file_id.into(), &token, database);
            if let Some(overload) = builtin.exact_overload(database, &arg_types) {
                let function = overload.r#type.lookup(database);
                let markup_text = pretty_fn(database, &function);
                return Some(RangeInfo::new(
                    range,
                    HoverResult {
                        markup: Markup::fenced_block(&markup_text),
                        actions: Vec::new(),
                    },
                ));
            }

            // Otherwise show all matching overloads sorted by relevance
            let matching = builtin.matching_overloads(database, &arg_types);
            let mut lines = Vec::new();
            for (_, overload) in &matching {
                let function = overload.r#type.lookup(database);
                lines.push(pretty_fn(database, &function));
            }
            if !lines.is_empty() {
                let markup_text = lines.join("\n");
                return Some(RangeInfo::new(
                    range,
                    HoverResult {
                        markup: Markup::fenced_block(&markup_text),
                        actions: Vec::new(),
                    },
                ));
            }
        }
    }

    None
}

/// Try to resolve the specific function overload at a call site.
/// Walks up from the token to find a `FunctionCall` expression, then uses
/// type inference's `call_resolution` to find the resolved overload.
fn try_resolve_call_at_token(
    semantics: &Semantics<'_>,
    file_id: hir_def::HirFileId,
    token: &syntax::SyntaxToken,
    database: &RootDatabase,
) -> Option<String> {
    // Walk up ancestors: Identifier -> Path -> IdentExpression -> FunctionCall
    let func_call = token.parent_ancestors().find_map(ast::FunctionCall::cast)?;
    let call_expr = ast::Expression::FunctionCall(func_call);

    let container = semantics.find_container(file_id, call_expr.syntax())?;
    let analyzer = semantics.analyze(container.as_def_with_body_id()?);
    let expr_id = analyzer.expression_id(&call_expr)?;
    let resolved = analyzer.infer.call_resolution(expr_id)?;

    match resolved {
        ResolvedCall::Function(fn_id) => {
            let function = fn_id.lookup(database);
            Some(pretty_fn(database, &function))
        },
        ResolvedCall::OtherTypeInitializer(_) => None,
    }
}

/// Collects the inferred types of already-typed arguments at a builtin call site.
/// Returns an empty vec if the token is not inside a function call.
fn collect_call_arg_types(
    semantics: &Semantics<'_>,
    file_id: hir_def::HirFileId,
    token: &syntax::SyntaxToken,
    database: &RootDatabase,
) -> Vec<hir_ty::ty::Type> {
    let func_call = match token.parent_ancestors().find_map(ast::FunctionCall::cast) {
        Some(fc) => fc,
        None => return Vec::new(),
    };
    let arguments = match func_call.parameters() {
        Some(args) => args,
        None => return Vec::new(),
    };
    let container = match semantics.find_container(file_id, func_call.syntax()) {
        Some(c) => c,
        None => return Vec::new(),
    };
    let def = match container.as_def_with_body_id() {
        Some(d) => d,
        None => return Vec::new(),
    };
    let analyzer = semantics.analyze(def);
    arguments
        .arguments()
        .filter_map(|arg| analyzer.type_of_expression(&arg))
        .collect()
}


/// Returns a Markdown description for a WGSL attribute name.
/// See <https://www.w3.org/TR/WGSL/#attributes>.
fn attribute_description(name: &str) -> Option<String> {
    let (syntax, description, spec_anchor) = match name {
        "align" => (
            "@align(expression)",
            "Specifies the byte alignment of a struct member. Must be a power of 2.",
            "align-attr",
        ),
        "binding" => (
            "@binding(expression)",
            "Specifies the binding number of a resource variable in a bind group. Used together with `@group`.",
            "binding-attr",
        ),
        "builtin" => (
            "@builtin(builtin_value)",
            "Specifies that a function parameter or struct member corresponds to a built-in value (e.g., `position`, `vertex_index`, `front_facing`).",
            "builtin-attr",
        ),
        "compute" => (
            "@compute",
            "Declares a function as a compute shader entry point.",
            "compute-attr",
        ),
        "const" => (
            "@const",
            "Declares a function as a const-expression function, meaning it can be evaluated at shader creation time.",
            "const-attr",
        ),
        "diagnostic" => (
            "@diagnostic(severity, rule)",
            "Controls the severity of a diagnostic rule. Severity can be `error`, `warning`, `info`, or `off`.",
            "diagnostic-attr",
        ),
        "fragment" => (
            "@fragment",
            "Declares a function as a fragment shader entry point.",
            "fragment-attr",
        ),
        "group" => (
            "@group(expression)",
            "Specifies the bind group index of a resource variable. Used together with `@binding`.",
            "group-attr",
        ),
        "id" => (
            "@id(expression)",
            "Specifies a numeric identifier for an `override` declaration, used for pipeline-overridable constants.",
            "id-attr",
        ),
        "interpolate" => (
            "@interpolate(type, sampling)",
            "Specifies how user-defined IO is interpolated. Type can be `perspective`, `linear`, or `flat`. Sampling can be `center`, `centroid`, or `sample`.",
            "interpolate-attr",
        ),
        "invariant" => (
            "@invariant",
            "Indicates that the value of a `@builtin(position)` output must be invariant across different shader invocations with the same input.",
            "invariant-attr",
        ),
        "location" => (
            "@location(expression)",
            "Specifies the location number for user-defined IO (inter-stage variables between vertex and fragment shaders).",
            "location-attr",
        ),
        "must_use" => (
            "@must_use",
            "Indicates that the return value of a function must be used by the caller.",
            "must-use-attr",
        ),
        "size" => (
            "@size(expression)",
            "Specifies the byte size of a struct member, including any trailing padding.",
            "size-attr",
        ),
        "vertex" => (
            "@vertex",
            "Declares a function as a vertex shader entry point.",
            "vertex-attr",
        ),
        "workgroup_size" => (
            "@workgroup_size(x, y, z)",
            "Specifies the workgroup dimensions for a compute shader. `y` and `z` default to 1 if omitted.",
            "workgroup-size-attr",
        ),
        _ => return None,
    };

    Some(format!(
        "{description}\n\n---\n\n```wgsl\n{syntax}\n```\n\n[WGSL Spec](https://www.w3.org/TR/WGSL/#{spec_anchor})"
    ))
}