use std::fmt::Write;

use crate::{
    db::DefDatabase,
    module_data::{ModuleInfo, ModuleItem},
};

use super::ImportValue;

pub fn pretty_print_module(db: &dyn DefDatabase, module: &ModuleInfo) -> String {
    let mut f = String::new();

    for item in module.items() {
        write_pretty_module_item(item, module, &mut f, db);
        f.push_str(";\n");
    }

    f
}

pub fn pretty_module_item(item: &ModuleItem, module: &ModuleInfo, db: &dyn DefDatabase) -> String {
    let mut f = String::new();
    write_pretty_module_item(item, module, &mut f, db);
    f
}

fn write_pretty_module_item(
    item: &ModuleItem,
    module: &ModuleInfo,
    f: &mut String,
    db: &dyn DefDatabase,
) {
    match *item {
        ModuleItem::Function(id) => {
            let function = &module.data[id.index];

            let _ = write!(f, "fn {}(", function.name.0);
            for param in function
                .params
                .clone()
                .map(|idx| &module.data[idx])
            {
                let ty = db.lookup_intern_type_ref(param.ty);
                let _ = write!(f, "{}, ", &ty);
            }
            trim_in_place(f, ", ");
            let _ = write!(f, ")");
        }
        ModuleItem::Struct(id) => {
            let strukt = &module.data[id.index];
            let _ = writeln!(f, "struct {} {{", strukt.name.0);
            for field in strukt.fields.clone() {
                let field = &module.data[field];
                let ty = db.lookup_intern_type_ref(field.ty);
                let _ = writeln!(f, "    {}: {};", field.name.0, ty);
            }
            let _ = write!(f, "}}");
        }
        ModuleItem::GlobalVariable(var) => {
            let var = &module.data[var.index];
            let ty = var.ty.map(|ty| db.lookup_intern_type_ref(ty));
            let _ = write!(f, "var {}", &var.name.0);
            if let Some(ty) = ty {
                let _ = write!(f, ": {}", ty);
            }
        }
        ModuleItem::GlobalConstant(var) => {
            let constant = &module.data[var.index];
            let ty = constant.ty.map(|ty| db.lookup_intern_type_ref(ty));
            let _ = write!(f, "let {}", &constant.name.0);
            if let Some(ty) = ty {
                let _ = write!(f, ": {}", ty);
            }
        }
        ModuleItem::Override(var) => {
            let override_decl = &module.data[var.index];
            let ty = override_decl.ty.map(|ty| db.lookup_intern_type_ref(ty));
            let _ = write!(f, "override {}", &override_decl.name.0);
            if let Some(ty) = ty {
                let _ = write!(f, ": {}", ty);
            }
        }
        ModuleItem::Import(import) => {
            let import = &module.data[import.index];
            let _ = match &import.value {
                ImportValue::Path(path) => write!(f, "#import \"{}\"", path),
                ImportValue::Custom(key) => write!(f, "#import {}", key),
            };
        }
        ModuleItem::TypeAlias(type_alias) => {
            let type_alias = &module.data[type_alias.index];
            let name = &type_alias.name.0;
            let ty = db.lookup_intern_type_ref(type_alias.ty);
            let _ = write!(f, "type {} = {};", name, ty);
        }
    }
}

fn trim_in_place(s: &mut String, pat: &str) {
    let new_len = s.trim_end_matches(pat).len();
    s.truncate(new_len);
}
