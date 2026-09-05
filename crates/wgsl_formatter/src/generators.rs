//! The generator functions that emit the formatted code.
//!
//! See [`crate::format`] for entry points to the whole formatter.
//!
//! See [`node::gen_node_with_trivia`] for the dispatcher that decides how to
//! format a given `SyntaxNode` and takes care of trivia and ignore pragmas.

pub mod attributes;
pub mod comments;
pub mod diagnostic_directive;
pub mod directives;
pub mod expressions;
pub mod function_declaration;
pub mod global_compound_declaration;
pub mod name;
pub mod node;
pub mod path;
pub mod source_file;
pub mod statements;
pub mod struct_declaration;
pub mod type_alias_declaration;
pub mod types;
pub mod verbatim;
