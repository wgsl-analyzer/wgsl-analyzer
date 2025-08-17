use crate::{
    database::DefDatabase,
    module_data::{ModuleInfo, ModuleItem},
};
use std::fmt::Write as _;

pub fn pretty_print_module(
    database: &dyn DefDatabase,
    module: &ModuleInfo,
) -> String {
    let mut buffer = String::new();
    for &item in module.items() {
        write_pretty_module_item(item, module, &mut buffer, database);
        buffer.push_str(";\n");
    }
    buffer
}

pub fn pretty_module_item(
    item: ModuleItem,
    module: &ModuleInfo,
    database: &dyn DefDatabase,
) -> String {
    let mut buffer = String::new();
    write_pretty_module_item(item, module, &mut buffer, database);
    buffer
}

fn write_pretty_module_item(
    item: ModuleItem,
    module: &ModuleInfo,
    buffer: &mut String,
    database: &dyn DefDatabase,
) {
    match item {
        ModuleItem::Function(id) => {
            let function = &module.data[id.index];

            _ = write!(buffer, "fn {}(", function.name.0);
            _ = write!(buffer, ")");
        },
        ModuleItem::Struct(id) => {
            let r#struct = &module.data[id.index];
            _ = writeln!(buffer, "struct {} {{", r#struct.name.0);
            _ = write!(buffer, "}}");
        },
        ModuleItem::GlobalVariable(var) => {
            let var = &module.data[var.index];
            _ = write!(buffer, "var {}", &var.name.0);
        },
        ModuleItem::GlobalConstant(var) => {
            let constant = &module.data[var.index];
            _ = write!(buffer, "let {}", &constant.name.0);
        },
        ModuleItem::Override(var) => {
            let override_decl = &module.data[var.index];
            _ = write!(buffer, "override {}", &override_decl.name.0);
        },
        ModuleItem::TypeAlias(type_alias) => {
            let type_alias = &module.data[type_alias.index];
            let name = &type_alias.name.0;
            _ = write!(buffer, "alias {name}");
        },
    }
}

fn trim_in_place(
    string: &mut String,
    pat: &str,
) {
    let new_length = string.trim_end_matches(pat).len();
    string.truncate(new_length);
}
