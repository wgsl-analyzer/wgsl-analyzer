pub mod lower;

use std::ops::Index;

use crate::{
    expression::{Expression, ExpressionId},
    type_specifier::{TypeSpecifier, TypeSpecifierId},
};
use la_arena::{Arena, ArenaMap};
use rustc_hash::{FxHashMap, FxHashSet};
use syntax::{ast, pointer::AstPointer};

#[derive(PartialEq, Eq, Debug)]
pub struct SyntheticSyntax;

/// An arena with expressions.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct ExpressionStore {
    pub exprs: Arena<Expression>,
    pub types: Arena<TypeSpecifier>,
    /// Used for signatures and for bodies.
    /// For example, a `const foo: vec3<f32> = vec3f(1,2,3);` will have two stores.
    /// One for the `const foo: vec3<f32>` part and another one for `vec3f(1,2,3);`.
    /// Separating them gives us more fine grained incrementality.
    pub store_source: ExpressionStoreSource,

    // TODO: Get rid of this (move the checks to the syntax tree)
    pub parenthesis_expressions: FxHashSet<ExpressionId>,
}

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum ExpressionStoreSource {
    #[default]
    Body,
    Signature,
}

impl Index<ExpressionId> for ExpressionStore {
    type Output = Expression;

    #[inline]
    fn index(
        &self,
        expr: ExpressionId,
    ) -> &Expression {
        &self.exprs[expr]
    }
}

impl Index<TypeSpecifierId> for ExpressionStore {
    type Output = TypeSpecifier;

    #[inline]
    fn index(
        &self,
        expr: TypeSpecifierId,
    ) -> &TypeSpecifier {
        &self.types[expr]
    }
}

#[derive(Default, Debug, Eq)]
pub struct ExpressionSourceMap {
    expression_map: FxHashMap<AstPointer<ast::Expression>, ExpressionId>,
    expression_map_back:
        ArenaMap<ExpressionId, Result<AstPointer<ast::Expression>, SyntheticSyntax>>,
    type_map: FxHashMap<AstPointer<ast::TypeSpecifier>, TypeSpecifierId>,
    type_map_back:
        ArenaMap<TypeSpecifierId, Result<AstPointer<ast::TypeSpecifier>, SyntheticSyntax>>,
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
            type_map: _,
            type_map_back,
        } = self;

        *expression_map_back == other.expression_map_back && *type_map_back == other.type_map_back
    }
}

/// The body of an item (function, const etc.).
#[derive(Debug, Eq, PartialEq, Default)]
pub struct ExpressionStoreBuilder {
    exprs: Arena<Expression>,
    types: Arena<TypeSpecifier>,
    store_source: ExpressionStoreSource,
    parenthesis_expressions: FxHashSet<ExpressionId>,

    expression_map: FxHashMap<AstPointer<ast::Expression>, ExpressionId>,
    expression_map_back:
        ArenaMap<ExpressionId, Result<AstPointer<ast::Expression>, SyntheticSyntax>>,
    type_map: FxHashMap<AstPointer<ast::TypeSpecifier>, TypeSpecifierId>,
    type_map_back:
        ArenaMap<TypeSpecifierId, Result<AstPointer<ast::TypeSpecifier>, SyntheticSyntax>>,
}

impl ExpressionStoreBuilder {
    #[must_use]
    pub fn finish(self) -> (ExpressionStore, ExpressionSourceMap) {
        let Self {
            mut exprs,
            mut types,
            store_source,
            mut parenthesis_expressions,
            mut expression_map,
            mut expression_map_back,
            mut type_map,
            mut type_map_back,
        } = self;
        exprs.shrink_to_fit();
        types.shrink_to_fit();
        parenthesis_expressions.shrink_to_fit();
        expression_map.shrink_to_fit();
        expression_map_back.shrink_to_fit();
        type_map.shrink_to_fit();
        type_map_back.shrink_to_fit();
        (
            ExpressionStore {
                exprs,
                types,
                store_source,
                parenthesis_expressions,
            },
            ExpressionSourceMap {
                expression_map,
                expression_map_back,
                type_map,
                type_map_back,
            },
        )
    }
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
    #[must_use]
    pub fn lookup_type_specifier(
        &self,
        source: &AstPointer<ast::TypeSpecifier>,
    ) -> Option<TypeSpecifierId> {
        self.type_map.get(source).copied()
    }

    pub fn type_specifier_to_source(
        &self,
        expression: TypeSpecifierId,
    ) -> Result<&AstPointer<ast::TypeSpecifier>, &SyntheticSyntax> {
        self.type_map_back[expression].as_ref()
    }
}
