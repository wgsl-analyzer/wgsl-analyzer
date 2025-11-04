#![expect(clippy::use_debug, reason = "debug formatting in unit tests is ok")]

use crate::{
    FileAstId,
    module_data::{ModuleInfo, ModuleItem},
};
use std::fmt::Write as _;

#[must_use]
pub fn pretty_print_module(module: &ModuleInfo) -> String {
    let mut buffer = String::new();
    for &item in module.items() {
        write_pretty_module_item(item, module, &mut buffer);
        buffer.push('\n');
    }
    buffer
}

fn write_pretty_module_item(
    item: ModuleItem,
    module: &ModuleInfo,
    buffer: &mut String,
) {
    match item {
        ModuleItem::Function(id) => {
            let function = &module.data[id.index];
            print_ast_id(buffer, function.ast_id);
            _ = write!(buffer, "fn {};", function.name.0);
        },
        ModuleItem::Struct(id) => {
            let r#struct = &module.data[id.index];
            print_ast_id(buffer, r#struct.ast_id);
            _ = write!(buffer, "struct {} {{ ... }}", r#struct.name.0);
        },
        ModuleItem::GlobalVariable(id) => {
            let variable = &module.data[id.index];
            print_ast_id(buffer, variable.ast_id);
            _ = write!(buffer, "var {} = _;", &variable.name.0);
        },
        ModuleItem::GlobalConstant(id) => {
            let constant = &module.data[id.index];
            print_ast_id(buffer, constant.ast_id);
            _ = write!(buffer, "const {} = _;", &constant.name.0);
        },
        ModuleItem::Override(id) => {
            let override_declaration = &module.data[id.index];
            print_ast_id(buffer, override_declaration.ast_id);
            _ = write!(buffer, "override {} = _;", &override_declaration.name.0);
        },
        ModuleItem::TypeAlias(id) => {
            let type_alias = &module.data[id.index];
            print_ast_id(buffer, type_alias.ast_id);
            _ = write!(buffer, "alias {} = _;", &type_alias.name.0);
        },
    }
}

fn print_ast_id<T: syntax::AstNode>(
    buffer: &mut String,
    ast_id: FileAstId<T>,
) {
    writeln!(buffer, "// {ast_id:?}");
}

fn trim_in_place(
    string: &mut String,
    pat: &str,
) {
    let new_length = string.trim_end_matches(pat).len();
    string.truncate(new_length);
}
