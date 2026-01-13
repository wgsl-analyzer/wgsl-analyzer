#![expect(clippy::use_debug, reason = "debug formatting in unit tests is ok")]

use std::fmt::Write as _;

use crate::{
    FileAstId,
    item_tree::{ImportTree, ItemTree, ModuleItem},
    mod_path::PathKind,
};

#[must_use]
pub fn pretty_print_item_tree(module: &ItemTree) -> String {
    let mut buffer = String::new();
    for &item in module.items() {
        write_pretty_module_item(item, module, &mut buffer);
        buffer.push('\n');
    }
    buffer
}

fn write_pretty_module_item(
    item: ModuleItem,
    module: &ItemTree,
    buffer: &mut String,
) {
    match item {
        ModuleItem::ImportStatement(id) => {
            let import_statement = &module[id.index];
            print_ast_id(buffer, import_statement.ast_id);
            _ = write!(buffer, "import");
            write_pretty_relative_import(import_statement.kind, buffer);
            write_pretty_import_tree(&import_statement.tree, buffer);
            _ = write!(buffer, ";");
        },
        ModuleItem::Function(id) => {
            let function = &module[id.index];
            print_ast_id(buffer, function.ast_id);
            _ = write!(buffer, "fn {};", function.name.0);
        },
        ModuleItem::Struct(id) => {
            let r#struct = &module[id.index];
            print_ast_id(buffer, r#struct.ast_id);
            _ = write!(buffer, "struct {} {{ ... }}", r#struct.name.0);
        },
        ModuleItem::GlobalVariable(id) => {
            let variable = &module[id.index];
            print_ast_id(buffer, variable.ast_id);
            _ = write!(buffer, "var {} = _;", &variable.name.0);
        },
        ModuleItem::GlobalConstant(id) => {
            let constant = &module[id.index];
            print_ast_id(buffer, constant.ast_id);
            _ = write!(buffer, "const {} = _;", &constant.name.0);
        },
        ModuleItem::Override(id) => {
            let override_declaration = &module[id.index];
            print_ast_id(buffer, override_declaration.ast_id);
            _ = write!(buffer, "override {} = _;", &override_declaration.name.0);
        },
        ModuleItem::TypeAlias(id) => {
            let type_alias = &module[id.index];
            print_ast_id(buffer, type_alias.ast_id);
            _ = write!(buffer, "alias {} = _;", &type_alias.name.0);
        },
        ModuleItem::GlobalAssertStatement(_) => {
            _ = write!(buffer, "const_assert _;");
        },
    }
}

fn write_pretty_relative_import(
    path_kind: PathKind,
    buffer: &mut String,
) {
    match path_kind {
        PathKind::Super(count) => {
            if count == 0 {
                _ = write!(buffer, "self::");
            } else {
                for _ in 0..count {
                    _ = write!(buffer, "self::");
                }
            }
        },
        PathKind::Package => {
            _ = write!(buffer, "package::");
        },
        PathKind::Plain => {},
    }
}

fn write_pretty_import_tree(
    import_tree: &ImportTree,
    buffer: &mut String,
) {
    match import_tree {
        ImportTree::Path { name, item } => {
            _ = write!(buffer, "{}::", name.as_str());
            write_pretty_import_tree(&item, buffer);
        },
        ImportTree::Item { name, alias: None } => {
            _ = write!(buffer, "{}", name.as_str());
        },
        ImportTree::Item {
            name,
            alias: Some(alias),
        } => {
            _ = write!(buffer, "{} as {}", name.as_str(), alias.as_str());
        },
        ImportTree::Collection { list } => {
            _ = write!(buffer, "{{");
            for item in list {
                write_pretty_import_tree(item, buffer);
            }
            _ = write!(buffer, "}}");
        },
    }
}

fn print_ast_id<T: syntax::AstNode>(
    buffer: &mut String,
    ast_id: FileAstId<T>,
) {
    writeln!(buffer, "// {ast_id:?}").unwrap();
}

fn trim_in_place(
    string: &mut String,
    pat: &str,
) {
    let new_length = string.trim_end_matches(pat).len();
    string.truncate(new_length);
}
