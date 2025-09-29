use triomphe::Arc;

use base_db::{FileId, change::Change};
use expect_test::{Expect, expect};
use hir_def::database::DefDatabase as _;
use vfs::VfsPath;

use crate::RootDatabase;

fn single_file_db(source: &str) -> (RootDatabase, FileId) {
    let mut database = RootDatabase::new(None);
    let mut change = Change::new();
    let file_id = FileId::from_raw(0);
    change.change_file(
        file_id,
        Some(Arc::new(source.to_owned())),
        VfsPath::new_virtual_path("/".into()),
    );
    database.apply_change(change);

    (database, file_id)
}

#[expect(clippy::needless_pass_by_value, reason = "matches expect! macro")]
fn check_item_tree(
    source: &str,
    expect: Expect,
) {
    let (database, file_id) = single_file_db(source);

    let module_info = database.module_info(file_id.into());
    expect.assert_eq(&hir_def::module_data::pretty::pretty_print_module(
        &database,
        &module_info,
    ));
}

#[test]
fn simple_item_tree() {
    check_item_tree(
        "
fn test(a: f32) {}

fn test2(b: vec3<u32>, c: vec4<test>) {}

fn error(d: ?) {}
",
        expect![["
            fn test(f32);
            fn test2(vec3<u32>, vec4<test>);
            fn error([error]);
        "]],
    );
}

#[test]
fn item_tree_types() {
    check_item_tree(
        "
fn test(a: texture_2d<f32>) {}

var tex_sampled: texture_2d<f32>;
var tex_sampled_cube_array: texture_cube_array<f32>;
var tex_storage: texture_storage_2d<rgba8unorm, read_write>;
var tex_depth: texture_depth;
var tex_external: texture_external;
var tex_depth_multisampled: texture_depth_multisampled_2d;

var x: sampler_comparison;
var y: atomic<u32>;
var z: array<path, 10>;
var z: array<path, COUNT>;

struct Test {
    a: f32,
    b: vec3<f32>;
};
",
        expect![["
            fn test(texture_2d<f32>);
            var tex_sampled: texture_2d<f32>;
            var tex_sampled_cube_array: texture_cube_array<f32>;
            var tex_storage: texture_storage_2d<rgba8unorm, read_write>;
            var tex_depth: texture_depth;
            var tex_external: texture_external;
            var tex_depth_multisampled: texture_depth_multisampled_2d;
            var x: sampler_comparison;
            var y: atomic<u32>;
            var z: array<path, 10>;
            var z: array<path, COUNT>;
            struct Test {
                a: f32;
                b: vec3<f32>;
            };
        "]],
    );
}
