#![allow(dead_code)]

use base_db::FilePosition;
use completions::Completions;
use context::CompletionContext;
use hir::HirDatabase;

pub mod completions;
mod context;
pub mod item;
mod patterns;

pub fn completions(
	db: &dyn HirDatabase,
	position: FilePosition,
) -> Option<Completions> {
	let mut acc = Completions::default();

	let ctx = CompletionContext::new(db, position)?;
	completions::import::complete_import(&mut acc, &ctx);
	completions::dot::complete_dot(&mut acc, &ctx);
	completions::expr::complete_names_in_scope(&mut acc, &ctx);

	Some(acc)
}
