mod lower;
pub mod scope;

use std::sync::Arc;

use either::Either;
use la_arena::{Arena, ArenaMap, Idx};
use rustc_hash::{FxHashMap, FxHashSet};
use syntax::{ast, pointer::AstPointer};

use crate::{
    HasSource,
    db::{DefDatabase, DefinitionWithBodyId, Lookup},
    expression::{Expression, ExpressionId, Statement, StatementId},
    module_data::Name,
};

pub type BindingId = Idx<Binding>;
/// Local or parameter
#[derive(Debug, PartialEq, Eq)]
pub struct Binding {
    pub name: Name,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Body {
    pub exprs: Arena<Expression>,
    pub statements: Arena<Statement>,
    pub bindings: Arena<Binding>,
    pub parenthesis_expressions: FxHashSet<ExpressionId>,

    // for global declarations
    pub main_binding: Option<BindingId>,
    // for functions
    pub parameters: Vec<BindingId>,

    pub root: Option<Either<StatementId, ExpressionId>>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct SyntheticSyntax;

/// An item body together with the mapping from syntax nodes to HIR expression
/// IDs. This is needed to go from e.g. a position in a file to the HIR
/// expression containing it; but for type inference etc., we want to operate on
/// a structure that is agnostic to the actual positions of expressions in the
/// file, so that we do not recompute types whenever some whitespace is typed.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct BodySourceMap {
    expression_map: FxHashMap<AstPointer<ast::Expr>, ExpressionId>,
    expression_map_back: ArenaMap<ExpressionId, Result<AstPointer<ast::Expr>, SyntheticSyntax>>,

    statement_map: FxHashMap<AstPointer<ast::Statement>, StatementId>,
    statement_map_back: ArenaMap<StatementId, Result<AstPointer<ast::Statement>, SyntheticSyntax>>,

    binding_map: FxHashMap<AstPointer<ast::Binding>, BindingId>,
    binding_map_back: ArenaMap<BindingId, Result<AstPointer<ast::Binding>, SyntheticSyntax>>,
}

impl Body {
    pub fn body_query(
        db: &dyn DefDatabase,
        def: DefinitionWithBodyId,
    ) -> Arc<Body> {
        db.body_with_source_map(def).0
    }

    pub fn body_with_source_map_query(
        db: &dyn DefDatabase,
        def: DefinitionWithBodyId,
    ) -> (Arc<Body>, Arc<BodySourceMap>) {
        let file_id = def.file_id(db);
        let (body, source_map) = match def {
            DefinitionWithBodyId::Function(id) => {
                let location = id.lookup(db);
                let source = location.source(db);
                let parameters = source.value.parameter_list();
                let body = source.value.body();

                lower::lower_function_body(db, file_id, parameters, body)
            },
            DefinitionWithBodyId::GlobalVariable(id) => {
                let location = id.lookup(db);
                let source = location.source(db);

                lower::lower_global_var_declaration(db, file_id, source.value)
            },
            DefinitionWithBodyId::GlobalConstant(id) => {
                let location = id.lookup(db);
                let source = location.source(db);

                lower::lower_global_constant_declaration(db, file_id, source.value)
            },
            DefinitionWithBodyId::Override(id) => {
                let location = id.lookup(db);
                let source = location.source(db);

                lower::lower_override_declaration(db, file_id, source.value)
            },
        };

        (Arc::new(body), Arc::new(source_map))
    }
}

impl BodySourceMap {
    pub fn lookup_expression(
        &self,
        source: &AstPointer<ast::Expr>,
    ) -> Option<ExpressionId> {
        self.expression_map.get(source).copied()
    }

    pub fn lookup_statement(
        &self,
        source: &AstPointer<ast::Statement>,
    ) -> Option<StatementId> {
        self.statement_map.get(source).copied()
    }

    pub fn lookup_binding(
        &self,
        source: &AstPointer<ast::Binding>,
    ) -> Option<BindingId> {
        self.binding_map.get(source).copied()
    }

    pub fn binding_to_source(
        &self,
        binding: BindingId,
    ) -> Result<&AstPointer<ast::Binding>, &SyntheticSyntax> {
        self.binding_map_back[binding].as_ref()
    }

    pub fn expression_to_source(
        &self,
        expression: ExpressionId,
    ) -> Result<&AstPointer<ast::Expr>, &SyntheticSyntax> {
        self.expression_map_back[expression].as_ref()
    }

    pub fn statement_to_source(
        &self,
        statement: StatementId,
    ) -> Result<&AstPointer<ast::Statement>, &SyntheticSyntax> {
        self.statement_map_back[statement].as_ref()
    }
}
