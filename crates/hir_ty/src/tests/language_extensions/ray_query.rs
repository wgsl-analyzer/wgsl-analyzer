#![expect(non_snake_case, reason = "named after builtin functions")]

use expect_test::expect;

use crate::tests::check_infer;

// TODO support wgpu_ray_query
#[test]
fn rayQueryInitialize() {
    check_infer(
        "
//enable wgpu_ray_query_vertex_return;
var x: acceleration_structure;
fn foo() {
    let z = rayQueryInitialize(1, x, 2);
}
        ",
        expect![[r#"
            43..44 'x': ref<handle, acceleration_structure, read>
            89..90 'z': [error]
            93..120 'rayQue... x, 2)': [error]
            112..113 '1': integer
            115..116 'x': ref<handle, acceleration_structure, read>
            118..119 '2': integer
            93..120 'rayQue... x, 2)': `rayQueryInitialize` expects a pointer to `ray_query`, an acceleration structure and a `RayDesc` argument
        "#]],
    );
}
