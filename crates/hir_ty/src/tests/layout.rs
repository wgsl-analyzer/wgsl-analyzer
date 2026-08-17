use base_db::{CapabilitiesInput, EditionedFileId};
use expect_test::{Expect, expect};
use hir_def::{db::ModuleDefinitionId, item_tree::ItemTree, signature::StructSignature};
use std::fmt::Write as _;
use syntax::Capabilities;
use test_fixture::WithFixture as _;
use wgsl_types::syntax::AddressSpace;

use crate::{
    db::HirDatabase as _,
    layout::struct_member_layout,
    test_db::TestDatabase,
    tests::{module_definitions, text_range_start},
    ty::pretty::{TypeVerbosity, pretty_type_with_verbosity},
};

#[expect(clippy::needless_pass_by_value, reason = "Matches expect! macro")]
fn check_layout(
    capabilities: Capabilities,
    wa_fixture: &str,
    expect: Expect,
) {
    let (mut db, file_id) = TestDatabase::with_single_file(wa_fixture);
    CapabilitiesInput::update_capabilities(&mut db, capabilities);
    let mut buffer = String::new();
    LayoutPrinter::new(&db, EditionedFileId::from_file(&db, file_id.file_id(&db)))
        .infer_layout(&mut buffer);
    buffer.truncate(buffer.trim_end().len());
    buffer.push('\n');
    expect.assert_eq(&buffer);
}

struct LayoutPrinter<'db> {
    db: &'db TestDatabase,
    file_id: EditionedFileId,
}

impl<'db> LayoutPrinter<'db> {
    fn new(
        db: &'db TestDatabase,
        file_id: EditionedFileId,
    ) -> Self {
        Self { db, file_id }
    }

    fn infer_layout(
        &self,
        buffer: &mut String,
    ) {
        let module_info = ItemTree::of(self.db, self.file_id);
        let mut definitions = module_definitions(self.db, self.file_id, module_info);
        definitions.sort_by_key(|definition| text_range_start(*definition, self.db));
        for definition in definitions {
            match definition {
                ModuleDefinitionId::Function(_)
                | ModuleDefinitionId::GlobalVariable(_)
                | ModuleDefinitionId::GlobalConstant(_)
                | ModuleDefinitionId::GlobalAssertStatement(_)
                | ModuleDefinitionId::Override(_)
                | ModuleDefinitionId::TypeAlias(_) => (),
                ModuleDefinitionId::Struct(id) => {
                    let signature = StructSignature::of(self.db, id);
                    let (fields, diagnostics) = &*self.db.field_types(id);
                    assert!(diagnostics.is_empty());
                    let mut fields_output = vec![];
                    let Some((align, size)) = struct_member_layout(
                        fields,
                        self.db,
                        AddressSpace::Storage,
                        |field_data, field_type, field_layout| {
                            fields_output.push((
                                signature.field_data(field_data),
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
                    let before = format!("struct {name} {{");
                    let spaces = " ".repeat(47 - before.len());
                    writeln!(
                        buffer,
                        "{before}{spaces}//             align({align})  size({size})"
                    )
                    .unwrap();
                    for field_output in fields_output {
                        let name = field_output
                            .0
                            .map(|field_data| field_data.name.as_str())
                            .unwrap();
                        let r#type = pretty_type_with_verbosity(
                            self.db,
                            field_output.1,
                            TypeVerbosity::Full,
                        );
                        let before = format!("    {name}: {type},");
                        let spaces = 47 - before.len();
                        writeln!(
                            buffer,
                            "{before}{}// offset({})  align({})  size({})",
                            " ".repeat(spaces),
                            field_output.2,
                            field_output.3,
                            field_output.4,
                        )
                        .unwrap();
                    }
                    writeln!(buffer, "}}").unwrap();
                },
            }
        }
    }
}

#[test]
fn example_layout_of_structures_using_implicit_member_sizes_and_alignments() {
    check_layout(
        Capabilities::default(),
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
            struct B {                                     //             align(16)  size(160)
                a: vec2<f32>,                              // offset(0)  align(8)  size(8)
                b: vec3<f32>,                              // offset(8)  align(16)  size(12)
                c: f32,                                    // offset(32)  align(4)  size(4)
                d: f32,                                    // offset(36)  align(4)  size(4)
                e: A,                                      // offset(40)  align(8)  size(24)
                f: vec3<f32>,                              // offset(64)  align(16)  size(12)
                g: array<A, 3>,                            // offset(80)  align(8)  size(72)
                h: i32,                                    // offset(152)  align(4)  size(4)
            }
        "#]],
    );
}
