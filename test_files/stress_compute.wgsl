enable f16;

struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    color: vec4<f32>,
    lifetime: f32,
    mass: f32,
}

struct SimParams {
    dt: f32,
    gravity: vec3<f32>,
    bounds_min: vec3<f32>,
    bounds_max: vec3<f32>,
    damping: f32,
    particle_count: u32,
    attractor_count: u32,
}

struct Attractor {
    position: vec3<f32>,
    strength: f32,
    radius: f32,
}

struct GridCell {
    particle_indices: array<u32,16>,
    count: atomic<u32>,
}

@group(0) @binding(0) var<storage,read_write> particles: array<Particle>;
@group(0) @binding(1) var<uniform> params: SimParams;
@group(0) @binding(2) var<storage,read> attractors: array<Attractor>;
@group(0) @binding(3) var<storage,read_write> grid: array<GridCell>;
@group(0) @binding(4) var<storage,read_write> debug_output: array<vec4<f32>>;

const GRID_SIZE: u32 = 64u;
const MAX_NEIGHBORS: u32 = 32u;
const_assert GRID_SIZE <= 256u;
override WORKGROUP_SIZE: u32 = 256u;

var<workgroup> shared_positions: array<vec3<f32>,256>;
var<workgroup> shared_velocities: array<vec3<f32>,256>;

fn hash_position(pos: vec3<f32>) -> u32 {
    let grid_pos = vec3<u32>(
        (pos - params.bounds_min) / (params.bounds_max - params.bounds_min) * f32(GRID_SIZE)
    );
    return (grid_pos.x * GRID_SIZE * GRID_SIZE + grid_pos.y * GRID_SIZE + grid_pos.z) % (GRID_SIZE * GRID_SIZE * GRID_SIZE);
}

fn compute_force(p: ptr<function,Particle>, attractor_pos: vec3<f32>, strength: f32, radius: f32) -> vec3<f32> {
    let dir = attractor_pos - (*p).position;
    let dist = max(length(dir), 0.001);
    if dist > radius { return vec3<f32>(0.0); }
    let falloff = 1.0 - (dist / radius);
    return normalize(dir) * strength * falloff * falloff;
}

fn apply_bounds(p: ptr<function,Particle>) {
    if (*p).position.x < params.bounds_min.x {
        (*p).position.x = params.bounds_min.x;
        (*p).velocity.x = abs((*p).velocity.x) * 0.5;
    } else if (*p).position.x > params.bounds_max.x {
        (*p).position.x = params.bounds_max.x;
        (*p).velocity.x = -abs((*p).velocity.x) * 0.5;
    }
    if (*p).position.y < params.bounds_min.y {
        (*p).position.y = params.bounds_min.y;
        (*p).velocity.y = abs((*p).velocity.y) * 0.5;
    } else if (*p).position.y > params.bounds_max.y {
        (*p).position.y = params.bounds_max.y;
        (*p).velocity.y = -abs((*p).velocity.y) * 0.5;
    }
    if (*p).position.z < params.bounds_min.z {
        (*p).position.z = params.bounds_min.z;
        (*p).velocity.z = abs((*p).velocity.z) * 0.5;
    } else if (*p).position.z > params.bounds_max.z {
        (*p).position.z = params.bounds_max.z;
        (*p).velocity.z = -abs((*p).velocity.z) * 0.5;
    }
}

fn integrate(p: ptr<function,Particle>, force: vec3<f32>) {
    let acceleration = force / (*p).mass + params.gravity;
    (*p).velocity = ((*p).velocity + acceleration * params.dt) * params.damping;
    (*p).position = (*p).position + (*p).velocity * params.dt;
    (*p).lifetime = (*p).lifetime - params.dt;
    let speed = length((*p).velocity);
    let t = clamp(speed / 10.0, 0.0, 1.0);
    (*p).color = mix(vec4<f32>(0.2, 0.4, 1.0, 1.0), vec4<f32>(1.0, 0.3, 0.1, 1.0), t);
    (*p).color.a = clamp((*p).lifetime / 5.0, 0.0, 1.0);
}

@compute @workgroup_size(256) fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(local_invocation_id) local_id: vec3<u32>,
    @builtin(workgroup_id) wg_id: vec3<u32>
) {
    let idx = global_id.x;
    if idx >= params.particle_count { return; }
    var p = particles[idx];

    shared_positions[local_id.x] = p.position;
    shared_velocities[local_id.x] = p.velocity;
    workgroupBarrier();

    var total_force = vec3<f32>(0.0);
    for (var i: u32 = 0u; i < params.attractor_count; i += 1u) {
        total_force += compute_force(&p, attractors[i].position, attractors[i].strength, attractors[i].radius);
    }

    for (var i: u32 = 0u; i < 256u; i += 1u) {
        if i == local_id.x { continue; }
        let neighbor_pos = shared_positions[i];
        let diff = p.position - neighbor_pos;
        let dist = length(diff);
        if dist < 0.5 && dist > 0.001 {
            let repulsion = normalize(diff) * (0.5 - dist) * 2.0;
            total_force += repulsion;
        }
    }

    workgroupBarrier();
    integrate(&p, total_force);
    apply_bounds(&p);

    let cell = hash_position(p.position);
    let slot = atomicAdd(&grid[cell].count, 1u);
    if slot < 16u {
        grid[cell].particle_indices[slot] = idx;
    }

    particles[idx] = p;
    debug_output[idx] = vec4<f32>(total_force, length(total_force));
}
