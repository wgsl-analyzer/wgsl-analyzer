use expect_test::expect;

use crate::tests::check_infer;

#[test]
#[ignore = "https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1388"]
fn ray_tracing_pipeline() {
    check_infer(
        "
enable wgpu_ray_tracing_pipeline;

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
            84..96 'storage_read': ref<storage, S, read>
            149..159 'storage_rw': ref<storage, S, read_write>
            182..198 'runtim...adonly': u32
            201..232 'arrayL....data)': u32
            213..231 '&stora...d.data': ptr<storage, array<vec4<f32>>, read>
            214..226 'storage_read': ref<storage, S, read>
            214..231 'storag...d.data': ref<storage, array<vec4<f32>>, read>
            242..259 'runtim...dwrite': u32
            262..291 'arrayL....data)': u32
            274..290 '&stora...w.data': ptr<storage, array<vec4<f32>>, read_write>
            275..285 'storage_rw': ref<storage, S, read_write>
            275..290 'storage_rw.data': ref<storage, array<vec4<f32>>, read_write>
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
            19..20 'x': [error]
            23..36 'RAY_FLAG_NONE': [error]
            23..36 'RAY_FLAG_NONE': `RAY_FLAG_NONE` not found in scope
            23..36 'RAY_FLAG_NONE': expected variable, but got type `RAY_FLAG_NONE`
        "#]],
    );
}
