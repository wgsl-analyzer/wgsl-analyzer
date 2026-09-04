use expect_test::expect;

use crate::tests::check_infer;

#[test]
fn ray_tracing_pipeline() {
    check_infer(
        "
// enable wgpu_ray_tracing_pipeline;

struct HitCounters {
    hit_num: u32,
    selected_hit: u32,
}

var<ray_payload> hit_num: HitCounters;

@group(0) @binding(0)
var acc_struct: acceleration_structure;

@ray_generation
fn ray_gen_main(@builtin(ray_invocation_id) id: vec3<u32>, @builtin(num_ray_invocations) num_invocations: vec3<u32>) {
    hit_num = HitCounters();
    let shift = vec3<f32>(id) / vec3<f32>(num_invocations);
    let ray_shift = (vec3(shift.x, 0.0, shift.y) * 2.0) - 1.0;
    traceRay(acc_struct, RayDesc(RAY_FLAG_NONE, 0xff, 0.01, 100.0, vec3(0.0), vec3(0.0, 1.0, 0.0) + ray_shift), &hit_num);
}

var<incoming_ray_payload> incoming_hit_num: HitCounters;

@miss
@incoming_payload(incoming_hit_num)
fn miss(@builtin(world_ray_origin) origin: vec3<f32>, @builtin(world_ray_direction) dir: vec3<f32>, @builtin(ray_t_min) t_min: f32) {}

@any_hit
@incoming_payload(incoming_hit_num)
fn any_hit_main(@builtin(instance_custom_data) data: u32, @builtin(geometry_index) geo_idx: u32, @builtin(ray_t_current_max) max: f32, @builtin(hit_kind) kind: u32) {
    incoming_hit_num.hit_num++;
    incoming_hit_num.selected_hit = data;
}

@closest_hit
@incoming_payload(incoming_hit_num)
fn closest_hit_main(@builtin(object_ray_origin) origin: vec3<f32>, @builtin(object_ray_direction) dir: vec3<f32>, @builtin(object_to_world) obj_to_world: mat4x3<f32>, @builtin(world_to_object) world_to_obj: mat4x3<f32>) {}
",
        expect![[r#"
            120..127 'hit_num': ref<ray_payload, HitCounters, read_write>
            169..179 'acc_struct': ref<handle, acceleration_structure, read>
            266..268 'id': vec3<u32>
            311..326 'num_invocations': vec3<u32>
            345..352 'hit_num': ref<ray_payload, HitCounters, read_write>
            355..368 'HitCounters()': HitCounters
            378..383 'shift': vec3<f32>
            386..399 'vec3<f32>(id)': vec3<f32>
            386..428 'vec3<f...tions)': vec3<f32>
            396..398 'id': vec3<u32>
            402..428 'vec3<f...tions)': vec3<f32>
            412..427 'num_invocations': vec3<u32>
            438..447 'ray_shift': vec3<f32>
            450..491 '(vec3(... - 1.0': vec3<f32>
            451..478 'vec3(s...ift.y)': vec3<f32>
            451..484 'vec3(s... * 2.0': vec3<f32>
            456..461 'shift': vec3<f32>
            456..463 'shift.x': f32
            465..468 '0.0': float
            470..475 'shift': vec3<f32>
            470..477 'shift.y': f32
            481..484 '2.0': float
            488..491 '1.0': float
            497..614 'traceR...t_num)': [error]
            506..516 'acc_struct': ref<handle, acceleration_structure, read>
            518..603 'RayDes...shift)': RayDesc
            526..539 'RAY_FLAG_NONE': u32
            541..545 '0xff': integer
            547..551 '0.01': float
            553..558 '100.0': float
            560..569 'vec3(0.0)': vec3<float>
            565..568 '0.0': float
            571..590 'vec3(0..., 0.0)': vec3<float>
            571..602 'vec3(0..._shift': vec3<f32>
            576..579 '0.0': float
            581..584 '1.0': float
            586..589 '0.0': float
            593..602 'ray_shift': vec3<f32>
            605..613 '&hit_num': ptr<ray_payload, HitCounters, read_write>
            606..613 'hit_num': ref<ray_payload, HitCounters, read_write>
            645..661 'incomi...it_num': ref<incoming_ray_payload, HitCounters, read_write>
            754..760 'origin': vec3<f32>
            803..806 'dir': vec3<f32>
            839..844 't_min': f32
            947..951 'data': u32
            983..990 'geo_idx': u32
            1025..1028 'max': f32
            1054..1058 'kind': u32
            1071..1087 'incomi...it_num': ref<incoming_ray_payload, HitCounters, read_write>
            1071..1095 'incomi...it_num': ref<incoming_ray_payload, u32, read_write>
            1103..1119 'incomi...it_num': ref<incoming_ray_payload, HitCounters, read_write>
            1103..1132 'incomi...ed_hit': ref<incoming_ray_payload, u32, read_write>
            1135..1139 'data': u32
            1241..1247 'origin': vec3<f32>
            1291..1294 'dir': vec3<f32>
            1333..1345 'obj_to_world': mat4x3<f32>
            1386..1398 'world_to_obj': mat4x3<f32>
        "#]],
    );
}

#[test]
fn decl() {
    check_infer(
        "
fn foo() {
    let x = RAY_FLAG_NONE;
}
        ",
        expect![[r#"
            19..20 'x': u32
            23..36 'RAY_FLAG_NONE': u32
        "#]],
    );
}
