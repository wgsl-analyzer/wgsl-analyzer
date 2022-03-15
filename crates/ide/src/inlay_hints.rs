use base_db::{FileId, FileRange, TextRange};
use hir::Semantics;
use hir_def::module_data::Name;
use hir_ty::ty::{
    pretty::{pretty_type_with_verbosity, TypeVerbosity},
    TyKind,
};
use rowan::NodeOrToken;
use smol_str::SmolStr;
use syntax::{ast, AstNode, HasName, SyntaxNode};

use crate::RootDatabase;

#[derive(Clone, Debug)]
pub struct InlayHintsConfig {
    pub enabled: bool,
    pub type_verbosity: TypeVerbosity,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InlayKind {
    TypeHint,
    ParameterHint,
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
    let file = file.syntax();

    let mut hints = Vec::new();

    if let Some(range_limit) = range_limit {
        let range_limit = range_limit.range;
        match file.covering_element(range_limit) {
            NodeOrToken::Token(_) => return hints,
            NodeOrToken::Node(n) => {
                for node in n
                    .descendants()
                    .filter(|descendant| range_limit.contains_range(descendant.text_range()))
                {
                    get_hints(&mut hints, file_id, &sema, config, node);
                }
            }
        }
    } else {
        for node in file.descendants() {
            get_hints(&mut hints, file_id, &sema, config, node);
        }
    }

    hints
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
