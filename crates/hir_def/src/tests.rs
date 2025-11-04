use base_db::{FileId, change::Change};
use expect_test::{Expect, expect};
use triomphe::Arc;
use vfs::VfsPath;

use crate::{database::DefDatabase as _, test_db::TestDatabase};

pub(crate) fn single_file_db(source: &str) -> (TestDatabase, FileId) {
    let mut database = TestDatabase::default();
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
    expect.assert_eq(&crate::module_data::pretty::pretty_print_module(
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
            // FileAstId { id: Idx::<SyntaxNodePointer>(0) }
            fn test;
            // FileAstId { id: Idx::<SyntaxNodePointer>(1) }
            fn test2;
            // FileAstId { id: Idx::<SyntaxNodePointer>(2) }
            fn error;
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
            // FileAstId { id: Idx::<SyntaxNodePointer>(0) }
            fn test;
            // FileAstId { id: Idx::<SyntaxNodePointer>(1) }
            var tex_sampled = _;
            // FileAstId { id: Idx::<SyntaxNodePointer>(2) }
            var tex_sampled_cube_array = _;
            // FileAstId { id: Idx::<SyntaxNodePointer>(3) }
            var tex_storage = _;
            // FileAstId { id: Idx::<SyntaxNodePointer>(4) }
            var tex_depth = _;
            // FileAstId { id: Idx::<SyntaxNodePointer>(5) }
            var tex_external = _;
            // FileAstId { id: Idx::<SyntaxNodePointer>(6) }
            var tex_depth_multisampled = _;
            // FileAstId { id: Idx::<SyntaxNodePointer>(7) }
            var x = _;
            // FileAstId { id: Idx::<SyntaxNodePointer>(8) }
            var y = _;
            // FileAstId { id: Idx::<SyntaxNodePointer>(9) }
            var z = _;
            // FileAstId { id: Idx::<SyntaxNodePointer>(10) }
            var z = _;
            // FileAstId { id: Idx::<SyntaxNodePointer>(11) }
            struct Test { ... }
        "]],
    );
}
