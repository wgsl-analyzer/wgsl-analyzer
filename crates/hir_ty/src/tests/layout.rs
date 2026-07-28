use base_db::{EditionedFileId, Intern, Lookup};
use expect_test::{Expect, expect};
use hir_def::{
    database::{DefDatabase, ModuleDefinitionId},
    expression_store::ExpressionSourceMap,
};
use salsa::Durability;
use std::fmt::Write;
use syntax::{AstNode, ExtensionsConfig, SyntaxNode};
use test_fixture::WithFixture;
use vfs::FileId;
use wgsl_types::syntax::AddressSpace;

use crate::{
    database::HirDatabase,
    diagnostics::{InferenceDiagnostic, InferenceDiagnosticKind},
    layout::{FieldLayout, struct_member_layout},
    test_db::TestDatabase,
    tests::{ellipsize, module_definitions, text_range_start},
    ty::{
        Type, TypeKind,
        pretty::{TypeVerbosity, pretty_type_with_verbosity},
    },
};

#[expect(clippy::needless_pass_by_value, reason = "Matches expect! macro")]
fn check_layout(
    extensions: ExtensionsConfig,
    wa_fixture: &str,
    expect: Expect,
) {
    let mut database = TestDatabase::with_files(wa_fixture);
    database.set_extensions_with_durability(extensions, Durability::MEDIUM);
    let mut buffer = String::new();
    LayoutPrinter::new(
        &database,
        EditionedFileId::from_file(&database, FileId::from_raw(0)),
    )
    .infer_layout(&mut buffer);
    buffer.truncate(buffer.trim_end().len());
    buffer.push('\n');
    expect.assert_eq(&buffer);
}

struct LayoutPrinter<'database> {
    database: &'database TestDatabase,
    file_id: EditionedFileId,
    root: SyntaxNode,
}

impl<'db> LayoutPrinter<'db> {
    fn new(
        database: &'db TestDatabase,
        file_id: EditionedFileId,
    ) -> Self {
        let root = file_id.parse(database).syntax();
        Self {
            database,
            file_id,
            root,
        }
    }

    fn infer_layout(
        &self,
        buffer: &mut String,
    ) {
        let module_info = self.database.item_tree(self.file_id);
        let mut definitions = module_definitions(self.database, self.file_id, &module_info);
        definitions.sort_by_key(|definition| text_range_start(*definition, self.database));
        for definition in definitions {
            match definition {
                ModuleDefinitionId::Function(id) => (),
                ModuleDefinitionId::GlobalVariable(id) => (),
                ModuleDefinitionId::GlobalConstant(id) => (),
                ModuleDefinitionId::GlobalAssertStatement(id) => (),
                ModuleDefinitionId::Override(id) => (),
                ModuleDefinitionId::Module(_) => (),
                ModuleDefinitionId::Struct(id) => {
                    let (signature, map) = self.database.struct_data(id);
                    let (fields, diagnostics) = &*self.database.field_types(id);
                    assert!(diagnostics.is_empty());
                    let mut fields_output = vec![];
                    let Some((align, size)) = struct_member_layout(
                        &fields,
                        self.database,
                        AddressSpace::Storage,
                        |field_data, field_type, field_layout| {
                            fields_output.push((
                                signature.field_data(&field_data),
                                field_type,
                                field_layout.offset,
                                field_layout.align,
                                field_layout.size,
                            ));
                        },
                    ) else {
                        panic!("unable to calculate layout for struct {id:?}");
                    };
                    let name = signature.name.as_str();
                    let before = format!("struct {} {{", name);
                    let spaces = 47 - before.len();
                    writeln!(
                        buffer,
                        "{}{}//             align({})  size({})",
                        before, " ".repeat(spaces), align, size
                    );
                    for field_output in fields_output {
                        let name = field_output.0.map(|x| x.name.as_str()).unwrap();
                        let r#type = pretty_type_with_verbosity(
                            self.database,
                            field_output.1,
                            TypeVerbosity::Full,
                        );
                        let before = format!("    {}: {},", name, r#type);
                        let spaces = 47 - before.len();
                        writeln!(
                            buffer,
                            "{}{}// offset({})  align({})  size({})",
                            before,
                            " ".repeat(spaces),
                            field_output.2,
                            field_output.3,
                            field_output.4,
                        );
                    }
                    writeln!(buffer, "}}");
                },
                ModuleDefinitionId::TypeAlias(id) => (),
            }
        }
    }
}

#[test]
fn example_layout_of_structures_using_implicit_member_sizes_and_alignments() {
    check_layout(
        ExtensionsConfig::default(),
        "
            struct A {                                     //             align(8)  size(24)
                u: f32,                                    // offset(0)   align(4)  size(4)
                v: f32,                                    // offset(4)   align(4)  size(4)
                w: vec2<f32>,                              // offset(8)   align(8)  size(8)
                x: f32                                     // offset(16)  align(4)  size(4)
                // -- implicit struct size padding --      // offset(20)            size(4)
            }

            struct B {                                     //             align(16) size(160)
                a: vec2<f32>,                              // offset(0)   align(8)  size(8)
                // -- implicit member alignment padding -- // offset(8)             size(8)
                b: vec3<f32>,                              // offset(16)  align(16) size(12)
                c: f32,                                    // offset(28)  align(4)  size(4)
                d: f32,                                    // offset(32)  align(4)  size(4)
                // -- implicit member alignment padding -- // offset(36)            size(4)
                e: A,                                      // offset(40)  align(8)  size(24)
                f: vec3<f32>,                              // offset(64)  align(16) size(12)
                // -- implicit member alignment padding -- // offset(76)            size(4)
                g: array<A, 3>,    // element stride 24       offset(80)  align(8)  size(72)
                h: i32                                     // offset(152) align(4)  size(4)
                // -- implicit struct size padding --      // offset(156)           size(4)
            }
            ",
        expect![[r#"
            struct A {                                     //             align(8)  size(24)
                u: f32,                                    // offset(0)  align(4)  size(4)
                v: f32,                                    // offset(4)  align(4)  size(4)
                w: vec2<f32>,                              // offset(8)  align(8)  size(8)
                x: f32,                                    // offset(16)  align(4)  size(4)
            }
            struct B {                                     //             align(16)  size(80)
                a: vec2<f32>,                              // offset(0)  align(8)  size(8)
                b: vec3<f32>,                              // offset(8)  align(16)  size(16)
                c: f32,                                    // offset(32)  align(4)  size(4)
                d: f32,                                    // offset(36)  align(4)  size(4)
                e: A,                                      // offset(40)  align(8)  size(8)
                f: vec3<f32>,                              // offset(48)  align(16)  size(16)
                g: array<A, 3>,                            // offset(64)  align(8)  size(8)
                h: i32,                                    // offset(72)  align(4)  size(4)
            }
        "#]],
    );
}
