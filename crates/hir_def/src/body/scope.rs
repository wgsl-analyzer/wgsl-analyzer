use std::{
	ops::Index,
	sync::Arc,
};

use either::Either;
use la_arena::{
	Arena,
	Idx,
};
use rustc_hash::FxHashMap;

use crate::{
	db::{
		DefDatabase,
		DefWithBodyId,
	},
	expr::{
		ExprId,
		Statement,
		StatementId,
	},
	module_data::Name,
};

use super::{
	BindingId,
	Body,
};

pub type ScopeId = Idx<ScopeData>;

#[derive(Debug, PartialEq, Eq)]
pub struct ExprScopes {
	scopes: Arena<ScopeData>,
	pub scope_by_expr: FxHashMap<ExprId, ScopeId>,
	scope_by_stmt: FxHashMap<StatementId, ScopeId>,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct ScopeData {
	parent: Option<ScopeId>,
	pub entries: Vec<ScopeEntry>,
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
	pub fn expr_scopes_query(
		db: &dyn DefDatabase,
		def: DefWithBodyId,
	) -> Arc<ExprScopes> {
		let body = db.body(def);
		Arc::new(ExprScopes::new(&body))
	}

	pub fn new(body: &Body) -> ExprScopes {
		let mut scopes = ExprScopes {
			scopes: Arena::default(),
			scope_by_expr: FxHashMap::default(),
			scope_by_stmt: FxHashMap::default(),
		};

		let root = scopes.root_scope();

		scopes.add_param_bindings(body, root, &body.params);

		if let Some(stmt) = body.root {
			match stmt {
				Either::Left(stmt) => {
					let _ = compute_statement_scopes(stmt, body, &mut scopes, root);
				},
				Either::Right(expr) => compute_expr_scopes(expr, body, &mut scopes, root),
			}
		}

		scopes
	}

	pub fn scope_for_expr(
		&self,
		expr: ExprId,
	) -> Option<ScopeId> {
		self.scope_by_expr.get(&expr).copied()
	}
	pub fn scope_for_statement(
		&self,
		stmt: StatementId,
	) -> Option<ScopeId> {
		self.scope_by_stmt.get(&stmt).copied()
	}

	pub fn scope_chain(
		&self,
		scope: Option<ScopeId>,
	) -> impl Iterator<Item = ScopeId> + '_ {
		std::iter::successors(scope, move |&scope| self.scopes[scope].parent)
	}

	pub fn entries(
		&self,
		scope: ScopeId,
	) -> &[ScopeEntry] {
		&self.scopes[scope].entries
	}

	pub fn resolve_name_in_scope(
		&self,
		scope: ScopeId,
		name: &Name,
	) -> Option<&ScopeEntry> {
		self.scope_chain(Some(scope))
			.find_map(|scope| self.entries(scope).iter().find(|it| it.name == *name))
	}

	fn root_scope(&mut self) -> ScopeId {
		self.scopes.alloc(ScopeData::default())
	}

	fn set_scope_expr(
		&mut self,
		expr: ExprId,
		scope: ScopeId,
	) {
		self.scope_by_expr.insert(expr, scope);
	}
	fn set_scope_stmt(
		&mut self,
		stmt: StatementId,
		scope: ScopeId,
	) {
		self.scope_by_stmt.insert(stmt, scope);
	}

	fn add_param_bindings(
		&mut self,
		body: &Body,
		root: Idx<ScopeData>,
		params: &[BindingId],
	) {
		for param in params {
			self.add_binding(body, *param, root);
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

fn compute_statement_scopes(
	stmt_id: StatementId,
	body: &Body,
	scopes: &mut ExprScopes,
	scope: ScopeId,
) -> ScopeId {
	scopes.set_scope_stmt(stmt_id, scope);

	let stmt = &body.statements[stmt_id];

	match stmt {
		Statement::Compound { statements } => {
			let new_scope = scopes.new_block_scope(scope);
			scopes.set_scope_stmt(stmt_id, new_scope);
			compute_compound_statement_scopes(statements, body, scopes, new_scope);
		},
		Statement::VariableStatement {
			binding_id,
			initializer,
			..
		}
		| Statement::ConstStatement {
			binding_id,
			initializer,
			..
		}
		| Statement::LetStatement {
			binding_id,
			initializer,
			..
		} => {
			if let Some(init) = initializer {
				compute_expr_scopes(*init, body, scopes, scope);
			}
			let scope = scopes.new_block_scope(scope);
			scopes.add_binding(body, *binding_id, scope);
			return scope;
		},
		Statement::Assignment { lhs, rhs } => {
			compute_expr_scopes(*lhs, body, scopes, scope);
			compute_expr_scopes(*rhs, body, scopes, scope);
		},
		Statement::CompoundAssignment { lhs, rhs, .. } => {
			compute_expr_scopes(*lhs, body, scopes, scope);
			compute_expr_scopes(*rhs, body, scopes, scope);
		},
		Statement::IncrDecr { expr, .. } => {
			compute_expr_scopes(*expr, body, scopes, scope);
		},
		Statement::If {
			condition,
			block,
			else_if_blocks,
			else_block,
		} => {
			compute_expr_scopes(*condition, body, scopes, scope);
			compute_statement_scopes(*block, body, scopes, scope);
			for else_if_block in else_if_blocks {
				compute_statement_scopes(*else_if_block, body, scopes, scope);
			}
			if let Some(else_block) = else_block {
				compute_statement_scopes(*else_block, body, scopes, scope);
			}
		},
		Statement::Switch {
			expr,
			case_blocks,
			default_block,
		} => {
			compute_expr_scopes(*expr, body, scopes, scope);

			for (selectors, case) in case_blocks {
				for selector in selectors {
					compute_expr_scopes(*selector, body, scopes, scope);
				}

				let case_scope = scopes.new_block_scope(scope);
				compute_statement_scopes(*case, body, scopes, case_scope);
			}

			if let Some(default_block) = default_block {
				let default_scope = scopes.new_block_scope(scope);
				compute_statement_scopes(*default_block, body, scopes, default_scope);
			}
		},
		Statement::For {
			initializer,
			condition,
			continuing_part,
			block,
		} => {
			let mut scope = scope;
			if let Some(init) = initializer {
				scope = compute_statement_scopes(*init, body, scopes, scope);
			}
			if let Some(condition) = condition {
				compute_expr_scopes(*condition, body, scopes, scope);
			}
			if let Some(cont) = continuing_part {
				// Variables produced in the continuing block aren't used
				let _ = compute_statement_scopes(*cont, body, scopes, scope);
			}
			let _ = compute_statement_scopes(*block, body, scopes, scope);
		},
		Statement::While { condition, block } => {
			compute_expr_scopes(*condition, body, scopes, scope);
			compute_statement_scopes(*block, body, scopes, scope);
		},
		Statement::Return { expr } => {
			if let Some(expr) = expr {
				compute_expr_scopes(*expr, body, scopes, scope);
			}
		},
		Statement::Missing | Statement::Discard | Statement::Break | Statement::Continue => {},
		Statement::Continuing { block } => {
			compute_statement_scopes(*block, body, scopes, scope);
		},
		Statement::Expr { expr } => {
			compute_expr_scopes(*expr, body, scopes, scope);
		},
		Statement::Loop { body: block } => {
			let _ = compute_statement_scopes(*block, body, scopes, scope);
		},
	}
	scope
}

fn compute_expr_scopes(
	expr: ExprId,
	body: &Body,
	scopes: &mut ExprScopes,
	scope: ScopeId,
) {
	scopes.set_scope_expr(expr, scope);
	body.exprs[expr].walk_child_exprs(|child| {
		compute_expr_scopes(child, body, scopes, scope);
	});
}
