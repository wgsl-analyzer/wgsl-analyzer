use base_db::{FileId, FileRange, TextRange};
use hir::{Field, HasSource, Semantics};
use hir_def::{data::FieldId, module_data::Name, InFile};
use hir_ty::{
    layout::{FieldLayout, LayoutAddressSpace},
    ty::{
        pretty::{pretty_type_with_verbosity, TypeVerbosity},
        FunctionType, TyKind,
    },
};
use rowan::NodeOrToken;
use smol_str::SmolStr;
use syntax::{ast, AstNode, HasName, SyntaxNode};

use crate::RootDatabase;

#[derive(Clone, Debug)]
pub struct InlayHintsConfig {
    pub enabled: bool,
    pub type_hints: bool,
    pub parameter_hints: bool,
    pub struct_layout_hints: Option<StructLayoutHints>,
    pub type_verbosity: TypeVerbosity,
}

#[derive(Clone, Copy, Debug)]
pub enum StructLayoutHints {
    Offset,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InlayKind {
    TypeHint,
    ParameterHint,
    StructLayoutHint,
}

#[derive(Debug)]
pub struct InlayHint {
    pub range: TextRange,
    pub kind: InlayKind,
    pub label: SmolStr,
}

pub(crate) fn inlay_hints(
    db: &RootDatabase,
    file_id: FileId,
    range_limit: Option<FileRange>,
    config: &InlayHintsConfig,
) -> Vec<InlayHint> {
    let sema = Semantics::new(db);
    let file = sema.parse(file_id);

    let mut hints = Vec::new();

    if let Some(range_limit) = range_limit {
        let range_limit = range_limit.range;
        match file.syntax().covering_element(range_limit) {
            NodeOrToken::Token(_) => return hints,
            NodeOrToken::Node(n) => {
                for node in n
                    .descendants()
                    .filter(|descendant| range_limit.contains_range(descendant.text_range()))
                {
                    get_hints(&mut hints, file_id, &sema, config, node);
                }

                get_struct_layout_hints(&mut hints, file_id, &sema, config);
            }
        }
    } else {
        for node in file.syntax().descendants() {
            get_hints(&mut hints, file_id, &sema, config, node);
        }

        get_struct_layout_hints(&mut hints, file_id, &sema, config);
    }

    hints
}

fn get_struct_layout_hints(
    hints: &mut Vec<InlayHint>,
    file_id: FileId,
    sema: &Semantics,
    config: &InlayHintsConfig,
) -> Option<()> {
    let display_kind = config.struct_layout_hints?;

    let module_info = sema.db.module_info(file_id.into());

    for strukt in module_info.structs() {
        let strukt = sema.db.intern_struct(InFile::new(file_id.into(), strukt));
        let fields = sema.db.field_types(strukt);

        let address_space = LayoutAddressSpace::Storage;

        hir_ty::layout::struct_member_layout(
            &fields,
            sema.db,
            address_space,
            |field, field_layout| {
                let FieldLayout {
                    offset,
                    align: _,
                    size: _,
                } = field_layout;
                let field = Field {
                    id: FieldId { strukt, field },
                };

                let source = field.source(sema.db.upcast())?.value;

                // this is only necessary, because the field syntax nodes include the whitespace to the next line...
                let actual_last_token = std::iter::successors(
                    source.syntax().last_token(),
                    rowan::SyntaxToken::prev_token,
                )
                .skip_while(|token| token.kind().is_trivia())
                .next()?;
                let range = TextRange::new(
                    source.syntax().text_range().start(),
                    actual_last_token.text_range().end(),
                );

                hints.push(InlayHint {
                    range,
                    kind: InlayKind::StructLayoutHint,
                    label: match display_kind {
                        StructLayoutHints::Offset => format!("{offset}").into(),
                    },
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
    sema: &Semantics,
    config: &InlayHintsConfig,
    node: SyntaxNode,
) -> Option<()> {
    if let Some(expr) = ast::Expr::cast(node.clone()) {
        match &expr {
            ast::Expr::FunctionCall(function_call_expr) => {
                if !config.parameter_hints {
                    return None;
                }
                let container = sema.find_container(file_id.into(), &node)?;

                let analyzed = sema.analyze(container);
                let function_ty = analyzed
                    .type_of_expr(&function_call_expr.expr()?)?
                    .kind(sema.db);

                let parameter_exprs = function_call_expr.params()?.args();

                let ty = match function_ty {
                    TyKind::Function(ty) => ty,
                    TyKind::BuiltinFnOverload(builtin, overload_id) => builtin
                        .lookup(sema.db)
                        .overload(overload_id)
                        .ty
                        .kind(sema.db)
                        .as_function()
                        .expect("builtin type should be function"),
                    _ => return None,
                };

                let param_hints = ty
                    .parameter_names()
                    .zip(parameter_exprs)
                    .filter(|&(name, _)| !Name::is_missing(name))
                    .filter(|(param_name, expr)| {
                        !should_hide_param_name_hint(&ty, param_name, expr)
                    })
                    .map(|(param_name, expr)| InlayHint {
                        range: expr.syntax().text_range(),
                        kind: InlayKind::ParameterHint,
                        label: param_name.into(),
                    });

                hints.extend(param_hints);
            }
            _ => {}
        }
    } else if let Some((binding, ty)) = ast::VariableStatement::cast(node.clone())
        .and_then(|stmt| Some((stmt.binding()?, stmt.ty())))
        .or_else(|| {
            ast::GlobalConstantDecl::cast(node.clone())
                .and_then(|stmt| Some((stmt.binding()?, stmt.ty())))
        })
        .or_else(|| {
            ast::GlobalVariableDecl::cast(node.clone())
                .and_then(|stmt| Some((stmt.binding()?, stmt.ty())))
        })
    {
        if !config.type_hints {
            return None;
        }
        if ty.is_none() {
            let container = sema.find_container(file_id.into(), &node)?;
            let ty = sema.analyze(container).type_of_binding(&binding)?;

            let label = pretty_type_with_verbosity(sema.db, ty, config.type_verbosity);
            hints.push(InlayHint {
                range: binding.name()?.ident_token()?.text_range(),
                kind: InlayKind::TypeHint,
                label: label.into(),
            });
        }
    }

    Some(())
}

// taken from https://github.com/rust-analyzer/rust-analyzer/blob/7308b3ef413cad8c211e239d32c9fab29ae2e664/crates/ide/src/inlay_hints.rs#L422

fn should_hide_param_name_hint(ty: &FunctionType, param_name: &str, expr: &ast::Expr) -> bool {
    is_argument_similar_to_param_name(expr, param_name)
        || (ty.parameters.len() == 1 && is_obvious_param(param_name))
}

fn is_argument_similar_to_param_name(expr: &ast::Expr, param_name: &str) -> bool {
    let argument = match get_string_representation(expr) {
        Some(argument) => argument,
        None => return false,
    };

    // std is honestly too panic happy...
    let str_split_at = |str: &str, at| str.is_char_boundary(at).then(|| argument.split_at(at));

    let param_name = param_name.trim_start_matches('_');
    let argument = argument.trim_start_matches('_');

    match str_split_at(argument, param_name.len()) {
        Some((prefix, rest)) if prefix.eq_ignore_ascii_case(param_name) => {
            return rest.is_empty() || rest.starts_with('_');
        }
        _ => (),
    }
    match argument
        .len()
        .checked_sub(param_name.len())
        .and_then(|at| str_split_at(argument, at))
    {
        Some((rest, suffix)) if param_name.eq_ignore_ascii_case(suffix) => {
            return rest.is_empty() || rest.ends_with('_');
        }
        _ => (),
    }

    // mixed camelCase/snake_case
    if compare_ignore_case_convention(argument, param_name) {
        return true;
    }

    false
}

fn is_obvious_param(param_name: &str) -> bool {
    let is_obvious_param_name = matches!(param_name, "predicate" | "value");
    param_name.len() == 1 || is_obvious_param_name
}

fn compare_ignore_case_convention(argument: &str, param_name: &str) -> bool {
    argument
        .chars()
        .filter(|&c| c != '_')
        .zip(param_name.chars().filter(|&c| c != '_'))
        .all(|(a, b)| a.eq_ignore_ascii_case(&b))
}

fn get_string_representation(expr: &ast::Expr) -> Option<String> {
    match expr {
        ast::Expr::PathExpr(expr) => Some(expr.name_ref()?.text().as_str().to_string()),
        ast::Expr::PrefixExpr(expr) => get_string_representation(&expr.expr()?),
        ast::Expr::FieldExpr(expr) => Some(expr.name_ref()?.text().as_str().to_string()),
        _ => None,
    }
}
