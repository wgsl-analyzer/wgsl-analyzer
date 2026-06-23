//! The type system. We currently use this to infer types for completion, hover
//! information and various assists.

pub mod builtins;
pub mod database;
pub mod function;
pub mod infer;
pub mod layout;
pub mod ty;
pub mod validate;

pub mod diagnostics;
pub mod lower;
#[cfg(test)]
mod test_db;
#[cfg(test)]
mod tests;
