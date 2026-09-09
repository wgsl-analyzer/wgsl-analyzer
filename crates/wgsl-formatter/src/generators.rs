//! The generator functions that emit the formatted code.
//!
//! See [`crate::format`] for entry points to the whole formatter.
//!
//! See [`node::gen_node_with_trivia`] for the dispatcher that decides how to
//! format a given `SyntaxNode` and takes care of trivia and ignore pragmas.

pub(crate) mod attributes;
pub(crate) mod comments;
pub(crate) mod diagnostic_directive;
pub(crate) mod directives;
pub(crate) mod expressions;
pub(crate) mod function_declaration;
pub(crate) mod global_compound_declaration;
pub(crate) mod name;
pub(crate) mod node;
pub(crate) mod path;
pub(crate) mod source_file;
pub(crate) mod statements;
pub(crate) mod struct_declaration;
pub(crate) mod type_alias_declaration;
pub(crate) mod types;
pub(crate) mod verbatim;
