mod lower;
pub mod scope;

use std::sync::Arc;

use either::Either;
use la_arena::{Arena, ArenaMap, Idx};
use rustc_hash::FxHashMap;
use syntax::{ast, ptr::AstPtr};

use crate::{
    db::{DefDatabase, DefWithBodyId, Lookup},
    expr::{Expr, ExprId, Statement, StatementId},
    module_data::Name,
    HasSource,
};

pub type BindingId = Idx<Binding>;
/// Local or parameter
#[derive(Debug, PartialEq, Eq)]
pub struct Binding {
    pub name: Name,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Body {
    pub exprs: Arena<Expr>,
    pub statements: Arena<Statement>,
    pub bindings: Arena<Binding>,

    // for global declarations
    pub main_binding: Option<BindingId>,
    // for functions
    pub params: Vec<BindingId>,

    pub root: Option<Either<StatementId, ExprId>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct SyntheticSyntax;

/// An item body together with the mapping from syntax nodes to HIR expression
/// IDs. This is needed to go from e.g. a position in a file to the HIR
/// expression containing it; but for type inference etc., we want to operate on
/// a structure that is agnostic to the actual positions of expressions in the
/// file, so that we don't recompute types whenever some whitespace is typed.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct BodySourceMap {
    expr_map: FxHashMap<AstPtr<ast::Expr>, ExprId>,
    expr_map_back: ArenaMap<ExprId, Result<AstPtr<ast::Expr>, SyntheticSyntax>>,

    stmt_map: FxHashMap<AstPtr<ast::Statement>, StatementId>,
    stmt_map_back: ArenaMap<StatementId, Result<AstPtr<ast::Statement>, SyntheticSyntax>>,

    binding_map: FxHashMap<AstPtr<ast::Binding>, BindingId>,
    binding_map_back: ArenaMap<BindingId, Result<AstPtr<ast::Binding>, SyntheticSyntax>>,
}

impl Body {
    pub fn body_query(db: &dyn DefDatabase, def: DefWithBodyId) -> Arc<Body> {
        db.body_with_source_map(def).0
    }

    pub fn body_with_source_map_query(
        db: &dyn DefDatabase,
        def: DefWithBodyId,
    ) -> (Arc<Body>, Arc<BodySourceMap>) {
        let file_id = def.file_id(db);
        let (body, source_map) = match def {
            DefWithBodyId::Function(id) => {
                let location = id.lookup(db);
                let src = location.source(db);
                let params = src.value.param_list();
                let body = src.value.body();

                lower::lower_function_body(db, file_id, params, body)
            }
            DefWithBodyId::GlobalVariable(id) => {
                let location = id.lookup(db);
                let src = location.source(db);

                lower::lower_global_var_decl(db, file_id, src.value)
            }
            DefWithBodyId::GlobalConstant(id) => {
                let location = id.lookup(db);
                let src = location.source(db);

                lower::lower_global_constant_decl(db, file_id, src.value)
            }
        };

        (Arc::new(body), Arc::new(source_map))
    }
}

impl BodySourceMap {
    pub fn lookup_expr(&self, source: &AstPtr<ast::Expr>) -> Option<ExprId> {
        self.expr_map.get(source).copied()
    }
    pub fn lookup_statement(&self, source: &AstPtr<ast::Statement>) -> Option<StatementId> {
        self.stmt_map.get(source).copied()
    }
    pub fn lookup_binding(&self, source: &AstPtr<ast::Binding>) -> Option<BindingId> {
        self.binding_map.get(source).copied()
    }

    pub fn binding_to_source(
        &self,
        binding: BindingId,
    ) -> Result<&AstPtr<ast::Binding>, &SyntheticSyntax> {
        self.binding_map_back[binding].as_ref()
    }
    pub fn expr_to_source(&self, expr: ExprId) -> Result<&AstPtr<ast::Expr>, &SyntheticSyntax> {
        self.expr_map_back[expr].as_ref()
    }
    pub fn stmt_to_source(
        &self,
        stmt: StatementId,
    ) -> Result<&AstPtr<ast::Statement>, &SyntheticSyntax> {
        self.stmt_map_back[stmt].as_ref()
    }
}
