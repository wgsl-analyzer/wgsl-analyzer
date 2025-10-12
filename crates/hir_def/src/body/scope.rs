use std::{iter, ops::Index};

use either::Either;
use la_arena::{Arena, Idx};
use rustc_hash::FxHashMap;
use triomphe::Arc;

use super::{BindingId, Body};
use crate::{
    database::{DefDatabase, DefinitionWithBodyId},
    expression::{ExpressionId, Statement, StatementId, SwitchCaseSelector},
    module_data::Name,
};

pub type ScopeId = Idx<ScopeData>;

#[derive(Debug, PartialEq, Eq)]
pub struct ExprScopes {
    scopes: Arena<ScopeData>,
    pub(crate) scope_by_expression: FxHashMap<ExpressionId, ScopeId>,
    scope_by_statement: FxHashMap<StatementId, ScopeId>,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct ScopeData {
    parent: Option<ScopeId>,
    pub(crate) entries: Vec<ScopeEntry>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ScopeEntry {
    pub name: Name,
    pub binding: BindingId,
}

impl Index<ScopeId> for ExprScopes {
    type Output = ScopeData;

    fn index(
        &self,
        index: ScopeId,
    ) -> &Self::Output {
        &self.scopes[index]
    }
}

impl ExprScopes {
    pub fn expression_scopes_query(
        database: &dyn DefDatabase,
        definition: DefinitionWithBodyId,
    ) -> Arc<Self> {
        let body = database.body(definition);
        Arc::new(Self::new(&body))
    }

    #[must_use]
    pub fn new(body: &Body) -> Self {
        let mut scopes = Self {
            scopes: Arena::default(),
            scope_by_expression: FxHashMap::default(),
            scope_by_statement: FxHashMap::default(),
        };

        let root = scopes.root_scope();

        scopes.add_param_bindings(body, root, &body.parameters);

        if let Some(statement) = body.root {
            match statement {
                Either::Left(statement) => {
                    compute_statement_scopes(statement, body, &mut scopes, root);
                },
                Either::Right(expression) => {
                    compute_expression_scopes(expression, body, &mut scopes, root);
                },
            }
        }

        scopes
    }

    #[must_use]
    pub fn scope_for_expression(
        &self,
        expression: ExpressionId,
    ) -> Option<ScopeId> {
        self.scope_by_expression.get(&expression).copied()
    }

    #[must_use]
    pub fn scope_for_statement(
        &self,
        statement: StatementId,
    ) -> Option<ScopeId> {
        self.scope_by_statement.get(&statement).copied()
    }

    pub fn scope_chain(
        &self,
        scope: Option<ScopeId>,
    ) -> impl Iterator<Item = ScopeId> + '_ {
        iter::successors(scope, move |&scope| self.scopes[scope].parent)
    }

    #[must_use]
    pub fn entries(
        &self,
        scope: ScopeId,
    ) -> &[ScopeEntry] {
        &self.scopes[scope].entries
    }

    #[must_use]
    pub fn resolve_name_in_scope(
        &self,
        scope: ScopeId,
        name: &Name,
    ) -> Option<&ScopeEntry> {
        self.scope_chain(Some(scope))
            .find_map(|scope| self.entries(scope).iter().find(|entry| entry.name == *name))
    }

    fn root_scope(&mut self) -> ScopeId {
        self.scopes.alloc(ScopeData::default())
    }

    fn set_scope_expression(
        &mut self,
        expression: ExpressionId,
        scope: ScopeId,
    ) {
        self.scope_by_expression.insert(expression, scope);
    }

    fn set_scope_statement(
        &mut self,
        statement: StatementId,
        scope: ScopeId,
    ) {
        self.scope_by_statement.insert(statement, scope);
    }

    fn add_param_bindings(
        &mut self,
        body: &Body,
        root: Idx<ScopeData>,
        parameters: &[BindingId],
    ) {
        for parameter in parameters {
            self.add_binding(body, *parameter, root);
        }
    }

    fn add_binding(
        &mut self,
        body: &Body,
        binding_id: BindingId,
        scope: ScopeId,
    ) {
        let binding = &body.bindings[binding_id];
        let entry = ScopeEntry {
            name: binding.name.clone(),
            binding: binding_id,
        };
        self.scopes[scope].entries.push(entry);
    }

    fn new_block_scope(
        &mut self,
        parent: ScopeId,
    ) -> ScopeId {
        self.scopes.alloc(ScopeData {
            parent: Some(parent),
            entries: vec![],
        })
    }
}

fn compute_compound_statement_scopes(
    statements: &[StatementId],
    body: &Body,
    scopes: &mut ExprScopes,
    mut scope: ScopeId,
) {
    for statement in statements {
        scope = compute_statement_scopes(*statement, body, scopes, scope);
    }
}

#[expect(clippy::too_many_lines, reason = "TODO")]
fn compute_statement_scopes(
    statement_id: StatementId,
    body: &Body,
    scopes: &mut ExprScopes,
    scope: ScopeId,
) -> ScopeId {
    scopes.set_scope_statement(statement_id, scope);

    let statement = &body.statements[statement_id];

    match statement {
        Statement::Compound { statements } => {
            let new_scope = scopes.new_block_scope(scope);
            scopes.set_scope_statement(statement_id, new_scope);
            compute_compound_statement_scopes(statements, body, scopes, new_scope);
        },
        Statement::Variable {
            binding_id,
            initializer,
            ..
        }
        | Statement::Const {
            binding_id,
            initializer,
            ..
        }
        | Statement::Let {
            binding_id,
            initializer,
            ..
        } => {
            if let Some(init) = initializer {
                compute_expression_scopes(*init, body, scopes, scope);
            }
            let scope = scopes.new_block_scope(scope);
            scopes.add_binding(body, *binding_id, scope);
            return scope;
        },
        Statement::Assignment {
            left_side,
            right_side,
        }
        | Statement::CompoundAssignment {
            left_side,
            right_side,
            ..
        } => {
            compute_expression_scopes(*left_side, body, scopes, scope);
            compute_expression_scopes(*right_side, body, scopes, scope);
        },
        Statement::PhonyAssignment { right_side } => {
            compute_expression_scopes(*right_side, body, scopes, scope);
        },
        Statement::IncrDecr { expression, .. } | Statement::Expression { expression } => {
            compute_expression_scopes(*expression, body, scopes, scope);
        },
        Statement::If {
            condition,
            block,
            else_if_blocks,
            else_block,
        } => {
            compute_expression_scopes(*condition, body, scopes, scope);
            compute_statement_scopes(*block, body, scopes, scope);
            for else_if_block in else_if_blocks {
                compute_statement_scopes(*else_if_block, body, scopes, scope);
            }
            if let Some(else_block) = else_block {
                compute_statement_scopes(*else_block, body, scopes, scope);
            }
        },
        Statement::Switch {
            expression,
            case_blocks,
        } => {
            compute_expression_scopes(*expression, body, scopes, scope);

            for (selectors, case) in case_blocks {
                for selector in selectors {
                    if let SwitchCaseSelector::Expression(selector) = selector {
                        compute_expression_scopes(*selector, body, scopes, scope);
                    }
                }

                let case_scope = scopes.new_block_scope(scope);
                compute_statement_scopes(*case, body, scopes, case_scope);
            }
        },
        Statement::For {
            initializer,
            condition,
            continuing_part,
            block,
        } => {
            let mut new_scope = scope;
            if let Some(init) = initializer {
                new_scope = compute_statement_scopes(*init, body, scopes, new_scope);
            }
            if let Some(condition) = condition {
                compute_expression_scopes(*condition, body, scopes, new_scope);
            }
            if let Some(cont) = continuing_part {
                // Variables produced in the continuing block are not used
                compute_statement_scopes(*cont, body, scopes, new_scope);
            }
            compute_statement_scopes(*block, body, scopes, new_scope);
        },
        Statement::While { condition, block } => {
            compute_expression_scopes(*condition, body, scopes, scope);
            compute_statement_scopes(*block, body, scopes, scope);
        },
        Statement::Return { expression } => {
            if let Some(expression) = expression {
                compute_expression_scopes(*expression, body, scopes, scope);
            }
        },
        Statement::Assert { expression } => {
            compute_expression_scopes(*expression, body, scopes, scope)
        },
        Statement::BreakIf { condition } => {
            compute_expression_scopes(*condition, body, scopes, scope);
        },
        Statement::Missing | Statement::Discard | Statement::Break | Statement::Continue => {},
        Statement::Continuing { block } | Statement::Loop { body: block } => {
            compute_statement_scopes(*block, body, scopes, scope);
        },
    }
    scope
}

fn compute_expression_scopes(
    expression: ExpressionId,
    body: &Body,
    scopes: &mut ExprScopes,
    scope: ScopeId,
) {
    scopes.set_scope_expression(expression, scope);
    body.store.exprs[expression].walk_child_expressions(|child| {
        compute_expression_scopes(child, body, scopes, scope);
    });
}
