use std::fmt::Write as _;

use super::ImportValue;
use crate::{
    database::DefDatabase,
    module_data::{ModuleInfo, ModuleItem},
};

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
            for parameter in function.parameters.clone().map(|index| &module.data[index]) {
                let r#type = database.lookup_intern_type_ref(parameter.r#type);
                _ = write!(buffer, "{}, ", &r#type);
            }
            trim_in_place(buffer, ", ");
            _ = write!(buffer, ")");
        },
        ModuleItem::Struct(id) => {
            let r#struct = &module.data[id.index];
            _ = writeln!(buffer, "struct {} {{", r#struct.name.0);
            for field in r#struct.fields.clone() {
                let field = &module.data[field];
                let r#type = database.lookup_intern_type_ref(field.r#type);
                _ = writeln!(buffer, "    {}: {};", field.name.0, r#type);
            }
            _ = write!(buffer, "}}");
        },
        ModuleItem::GlobalVariable(var) => {
            let var = &module.data[var.index];
            let r#type = var
                .r#type
                .map(|r#type| database.lookup_intern_type_ref(r#type));
            _ = write!(buffer, "var {}", &var.name.0);
            if let Some(r#type) = r#type {
                _ = write!(buffer, ": {type}");
            }
        },
        ModuleItem::GlobalConstant(var) => {
            let constant = &module.data[var.index];
            let r#type = constant
                .r#type
                .map(|r#type| database.lookup_intern_type_ref(r#type));
            _ = write!(buffer, "let {}", &constant.name.0);
            if let Some(r#type) = r#type {
                _ = write!(buffer, ": {type}");
            }
        },
        ModuleItem::Override(var) => {
            let override_decl = &module.data[var.index];
            let r#type = override_decl
                .r#type
                .map(|r#type| database.lookup_intern_type_ref(r#type));
            _ = write!(buffer, "override {}", &override_decl.name.0);
            if let Some(r#type) = r#type {
                _ = write!(buffer, ": {type}");
            }
        },
        ModuleItem::Import(import) => {
            let import = &module.data[import.index];
            _ = match &import.value {
                ImportValue::Path(path) => write!(buffer, "#import \"{path}\""),
                ImportValue::Custom(key) => write!(buffer, "#import {key}"),
            };
        },
        ModuleItem::TypeAlias(type_alias) => {
            let type_alias = &module.data[type_alias.index];
            let name = &type_alias.name.0;
            let r#type = database.lookup_intern_type_ref(type_alias.r#type);
            _ = write!(buffer, "type {name} = {type};");
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
