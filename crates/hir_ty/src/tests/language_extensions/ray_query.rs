#![expect(non_snake_case, reason = "named after builtin functions")]

use expect_test::expect;

use crate::tests::check_infer;

#[test]
fn rayQueryInitialize() {
    check_infer(
        "
//enable wgpu_ray_query;
var acc_struct: acceleration_structure;
fn foo() {
    var rq: ray_query;
    rayQueryInitialize(&rq, acc_struct, RayDesc(0u, 0xFFu, 0.01, 200.0, vec3(), vec3()));
}
        ",
        expect![[r#"
            29..39 'acc_struct': ref<handle, acceleration_structure, read>
            84..86 'rq': ref<function, ray_query, read_write>
            103..187 'rayQue...c3()))': [error]
            122..125 '&rq': ptr<function, ray_query, read_write>
            123..125 'rq': ref<function, ray_query, read_write>
            127..137 'acc_struct': ref<handle, acceleration_structure, read>
            139..186 'RayDes...ec3())': RayDesc
            147..149 '0u': u32
            151..156 '0xFFu': u32
            158..162 '0.01': float
            164..169 '200.0': float
            171..177 'vec3()': vec3<integer>
            179..185 'vec3()': vec3<integer>
        "#]],
    );
}

#[test]
fn rayQueryInitialize_vertex_return() {
    check_infer(
        "
//enable wgpu_ray_query;
//enable wgpu_ray_query_vertex_return;
var acc_struct: acceleration_structure<vertex_return>;
fn foo() {
    var rq: ray_query<vertex_return>;
    rayQueryInitialize(&rq, acc_struct, RayDesc(0u, 0xFFu, 0.01, 200.0, vec3(), vec3()));
}
        ",
        expect![[r#"
            68..78 'acc_struct': ref<handle, acceleration_structure<vertex_return>, read>
            138..140 'rq': ref<function, ray_query<vertex_return>, read_write>
            172..256 'rayQue...c3()))': [error]
            191..194 '&rq': ptr<function, ray_query<vertex_return>, read_write>
            192..194 'rq': ref<function, ray_query<vertex_return>, read_write>
            196..206 'acc_struct': ref<handle, acceleration_structure<vertex_return>, read>
            208..255 'RayDes...ec3())': RayDesc
            216..218 '0u': u32
            220..225 '0xFFu': u32
            227..231 '0.01': float
            233..238 '200.0': float
            240..246 'vec3()': vec3<integer>
            248..254 'vec3()': vec3<integer>
        "#]],
    );
}

// rayQueryInitialize(rq: ptr<function, ray_query>, acceleration_structure: acceleration_structure, ray_desc: RayDesc)
// rayQueryInitialize(rq: ptr<function, ray_query<vertex_return>>, acceleration_structure: acceleration_structure<vertex_return>, ray_desc: RayDesc)
