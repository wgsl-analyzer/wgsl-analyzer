mod lower;
pub mod scope;

use either::Either;
use la_arena::{Arena, ArenaMap, Idx};
use rustc_hash::{FxHashMap, FxHashSet};
use syntax::{ast, pointer::AstPointer};
use triomphe::Arc;

use crate::{
    HasSource as _,
    attributes::Attribute,
    database::{DefDatabase, DefinitionWithBodyId, Lookup as _},
    expression::{Expression, ExpressionId, Statement, StatementId},
    expression_store::{ExpressionSourceMap, ExpressionStore, SyntheticSyntax},
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
    pub store: ExpressionStore,
    pub attributes: Arena<Attribute>,
    pub statements: Arena<Statement>,
    pub bindings: Arena<Binding>,

    // for global declarations
    pub main_binding: Option<BindingId>,
    // for functions
    pub parameters: Vec<BindingId>,

    pub root: Option<Either<StatementId, ExpressionId>>,
}

impl std::ops::Deref for Body {
    type Target = ExpressionStore;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.store
    }
}

/// An item body together with the mapping from syntax nodes to HIR expression
/// IDs.
///
/// This is needed to go from e.g. a position in a file to the HIR
/// expression containing it; but for type inference etc., we want to operate on
/// a structure that is agnostic to the actual positions of expressions in the
/// file, so that we do not recompute types whenever some whitespace is typed.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct BodySourceMap {
    expressions: ExpressionSourceMap,

    statement_map: FxHashMap<AstPointer<ast::Statement>, StatementId>,
    statement_map_back: ArenaMap<StatementId, Result<AstPointer<ast::Statement>, SyntheticSyntax>>,

    binding_map: FxHashMap<AstPointer<ast::Name>, BindingId>,
    binding_map_back: ArenaMap<BindingId, Result<AstPointer<ast::Name>, SyntheticSyntax>>,
}

impl Body {
    pub fn body_query(
        database: &dyn DefDatabase,
        definition: DefinitionWithBodyId,
    ) -> Arc<Self> {
        database.body_with_source_map(definition).0
    }

    pub fn body_with_source_map_query(
        database: &dyn DefDatabase,
        definition: DefinitionWithBodyId,
    ) -> (Arc<Self>, Arc<BodySourceMap>) {
        let file_id = definition.file_id(database);
        let (body, source_map) = match definition {
            DefinitionWithBodyId::Function(id) => {
                let location = id.lookup(database);
                let source = location.source(database);
                let parameters = source.value.parameter_list();
                let body = source.value.body();

                lower::lower_function_body(database, file_id, parameters, body)
            },
            DefinitionWithBodyId::GlobalVariable(id) => {
                let location = id.lookup(database);
                let source = location.source(database);

                lower::lower_global_variable_declaration(database, file_id, &source.value)
            },
            DefinitionWithBodyId::GlobalConstant(id) => {
                let location = id.lookup(database);
                let source = location.source(database);

                lower::lower_global_constant_declaration(database, file_id, &source.value)
            },
            DefinitionWithBodyId::Override(id) => {
                let location = id.lookup(database);
                let source = location.source(database);

                lower::lower_override_declaration(database, file_id, &source.value)
            },
        };

        (Arc::new(body), Arc::new(source_map))
    }
}

impl BodySourceMap {
    #[must_use]
    pub fn lookup_expression(
        &self,
        source: &AstPointer<ast::Expression>,
    ) -> Option<ExpressionId> {
        self.expressions.lookup_expression(source)
    }

    #[must_use]
    pub fn lookup_statement(
        &self,
        source: &AstPointer<ast::Statement>,
    ) -> Option<StatementId> {
        self.statement_map.get(source).copied()
    }

    #[must_use]
    pub fn lookup_binding(
        &self,
        source: &AstPointer<ast::Name>,
    ) -> Option<BindingId> {
        self.binding_map.get(source).copied()
    }

    pub fn binding_to_source(
        &self,
        binding: BindingId,
    ) -> Result<&AstPointer<ast::Name>, &SyntheticSyntax> {
        self.binding_map_back[binding].as_ref()
    }

    pub fn expression_to_source(
        &self,
        expression: ExpressionId,
    ) -> Result<&AstPointer<ast::Expression>, &SyntheticSyntax> {
        self.expressions.expression_to_source(expression)
    }

    pub fn statement_to_source(
        &self,
        statement: StatementId,
    ) -> Result<&AstPointer<ast::Statement>, &SyntheticSyntax> {
        self.statement_map_back[statement].as_ref()
    }

    #[must_use]
    pub const fn expression_source_map(&self) -> &ExpressionSourceMap {
        &self.expressions
    }
}
