pub mod lower;

use crate::{
    expression::{Expression, ExpressionId},
    type_ref::TypeReference,
};
use la_arena::{Arena, ArenaMap};
use rustc_hash::{FxHashMap, FxHashSet};
use syntax::{ast, pointer::AstPointer};

#[derive(PartialEq, Eq, Debug)]
pub struct SyntheticSyntax;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct ExpressionStore {
    pub exprs: Arena<Expression>,
    // Maybe add types here https://github.com/rust-lang/rust-analyzer/blob/e10fa9393ea2df4067e2258c9b8132244e415964/crates/hir-def/src/expr_store.rs#L121

    // TODO: Get rid of this (move the checks to the syntax tree)
    pub parenthesis_expressions: FxHashSet<ExpressionId>,
}

#[derive(Default, Debug, Eq)]
pub struct ExpressionSourceMap {
    expression_map: FxHashMap<AstPointer<ast::Expression>, ExpressionId>,
    expression_map_back:
        ArenaMap<ExpressionId, Result<AstPointer<ast::Expression>, SyntheticSyntax>>,
}

impl PartialEq for ExpressionSourceMap {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        // we only need to compare one of the two mappings
        // as the other is a reverse mapping and thus will compare
        // the same as normal mapping
        let Self {
            expression_map: _,
            expression_map_back,
        } = self;

        *expression_map_back == other.expression_map_back
    }
}

/// The body of an item (function, const etc.).
#[derive(Debug, Eq, PartialEq, Default)]
pub struct ExpressionStoreBuilder {
    pub exprs: Arena<Expression>,
    pub parenthesis_expressions: FxHashSet<ExpressionId>,

    expression_map: FxHashMap<AstPointer<ast::Expression>, ExpressionId>,
    expression_map_back:
        ArenaMap<ExpressionId, Result<AstPointer<ast::Expression>, SyntheticSyntax>>,
}

impl ExpressionSourceMap {
    #[must_use]
    pub fn lookup_expression(
        &self,
        source: &AstPointer<ast::Expression>,
    ) -> Option<ExpressionId> {
        self.expression_map.get(source).copied()
    }

    pub fn expression_to_source(
        &self,
        expression: ExpressionId,
    ) -> Result<&AstPointer<ast::Expression>, &SyntheticSyntax> {
        self.expression_map_back[expression].as_ref()
    }
}
