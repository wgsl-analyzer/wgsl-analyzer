use std::sync::Arc;

use base_db::{change::Change, FileId};
use expect_test::{expect, Expect};
use hir_def::db::DefDatabase;

use crate::RootDatabase;

fn single_file_db(source: &str) -> (RootDatabase, FileId) {
    let mut db = RootDatabase::new();
    let mut change = Change::new();
    let file_id = FileId(0);
    change.change_file(file_id, Some(Arc::new(source.to_string())));
    db.apply_change(change);

    (db, file_id)
}

fn check_item_tree(source: &str, expect: Expect) {
    let (db, file_id) = single_file_db(source);

    let module_info = db.module_info(file_id.into());
    expect.assert_eq(&hir_def::module_data::pretty::pretty_print_module(
        &db,
        &module_info,
    ));
}

#[test]
fn simple_item_tree() {
    check_item_tree(
        r#"
fn test(a: f32) {}
fn test2(b: vec3<u32>, c: vec4<test>) {}
fn error(d: ?) {}
"#,
        expect![[r#"
            fn test(f32);
            fn test2(vec3<u32>, vec4<test>);
            fn error([error]);
        "#]],
    );
}

#[test]
fn item_tree_types() {
    check_item_tree(
        r#"
fn test(a: texture_2d<f32>) {}

let tex_sampled: texture_2d<f32>;
let tex_sampled_cube_array: texture_cube_array<f32>;
let tex_storage: texture_storage_2d<rgba8unorm, read_write>;
let tex_depth: texture_depth;
let tex_external: texture_external;
let tex_depth_multisampled: texture_depth_multisampled_2d;

let x: sampler_comparison;
let y: atomic<u32>;
let z: array<path, 10>;
let z: array<path, COUNT>;

struct Test {
    a: f32,
    b: vec3<f32>;
};
"#,
        expect![[r#"
            fn test(texture_2d<f32>);
            let tex_sampled: texture_2d<f32>;
            let tex_sampled_cube_array: texture_cube_array<f32>;
            let tex_storage: texture_storage_2d<rgba8unorm, read_write>;
            let tex_depth: texture_depth;
            let tex_external: texture_external;
            let tex_depth_multisampled: texture_depth_multisampled_2d;
            let x: sampler_comparison;
            let y: atomic<u32>;
            let z: array<path, 10>;
            let z: array<path, COUNT>;
            struct Test {
                a: f32;
                b: vec3<f32>;
            };
        "#]],
    );
}
