mod collector;
mod diagnostics;
mod modules_map;

#[cfg(test)]
mod tests;

pub use collector::collect_module;
pub use diagnostics::{DefDiagnostic, DefDiagnosticKind};
pub use modules_map::{ModuleData, ModulesMap, modules_map_query};
