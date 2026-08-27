#![expect(non_snake_case, reason = "named after builtin functions")]

use expect_test::expect;

use crate::tests::check_infer;

#[test]
fn rayQueryInitialize() {
    check_infer(
        "
//enable wgpu_ray_query;
//enable wgpu_ray_query_vertex_return;
var a_s: acceleration_structure;
var r_q: ray_query;
fn foo() {
    let r_d = todo();
    rayQueryInitialize(&r_q, a_s, r_d);

}
        ",
        expect![[r#"
            68..71 'a_s': ref<handle, acceleration_structure, read>
            101..104 'r_q': ref<handle, ray_query, read>
            136..139 'r_d': [error]
            142..148 'todo()': [error]
            154..188 'rayQue..., r_d)': [error]
            173..177 '&r_q': [error]
            174..177 'r_q': ref<handle, ray_query, read>
            179..182 'a_s': ref<handle, acceleration_structure, read>
            184..187 'r_d': [error]
            142..148 'todo()': `todo` not found in scope
            173..177 '&r_q': cannot create a pointer in `handle` address space
        "#]],
    );
}

// rayQueryInitialize(rq: ptr<function, ray_query>, acceleration_structure: acceleration_structure, ray_desc: RayDesc)
// rayQueryInitialize(rq: ptr<function, ray_query<vertex_return>>, acceleration_structure: acceleration_structure<vertex_return>, ray_desc: RayDesc)
