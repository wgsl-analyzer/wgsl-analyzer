struct Particle {
    position: vec3<f32>,
    padding1: f32,
    velocity: vec3<f32>,
    padding2: f32,
    color: vec4<f32>,
}

struct SimParams {
    delta_time: f32,
    gravity: f32,
    num_particles: u32,
    color_mode: u32,
    mouse_force: f32,
    mouse_radius: f32,
    // mouse_position: vec3<f32>,
    mouse_position_x: f32,
    mouse_position_y: f32,
    mouse_position_z: f32,
    is_mouse_dragging: u32,
}

@group(0) @binding(0)
var<storage, read_write> particles: array<Particle>;

@group(0) @binding(1)
var<uniform> params: SimParams;

@compute @workgroup_size(128)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    if index >= params.num_particles {
        return;
    }

    var particle = particles[index];

    particle.velocity.y -= params.gravity * params.delta_time;

    if params.is_mouse_dragging > 0u {
        let mouse_pos_3d = vec3<f32>(
                params.mouse_position_x,
                params.mouse_position_y,
                params.mouse_position_z,
            );

        let dir = mouse_pos_3d - particle.position;
        let dist = length(dir);

        if dist < params.mouse_radius * 2.0 {
            // Make the force stronger overall and more dramatic for closer particles
            let force_factor = pow(1.0 - dist / (params.mouse_radius * 2.0), 2.0) * 2.0;
            let force = normalize(dir) * params.mouse_force * force_factor;
            particle.velocity += force * params.delta_time;
        }
    }

    particle.position += particle.velocity * params.delta_time;

    let bounds = 500.0;

    if particle.position.x < -bounds {
        particle.position.x = -bounds;
        particle.velocity.x = abs(particle.velocity.x) * 0.5;
    } else if particle.position.x > bounds {
        particle.position.x = bounds;
        particle.velocity.x = -abs(particle.velocity.x) * 0.5;
    }

    if particle.position.y < -bounds {
        particle.position.y = -bounds;
        particle.velocity.y = abs(particle.velocity.y) * 0.5;
    } else if particle.position.y > bounds {
        particle.position.y = bounds;
        particle.velocity.y = -abs(particle.velocity.y) * 0.5;
    }

    if particle.position.z < -bounds {
        particle.position.z = -bounds;
        particle.velocity.z = abs(particle.velocity.z) * 0.5;
    } else if particle.position.z > bounds {
        particle.position.z = bounds;
        particle.velocity.z = -abs(particle.velocity.z) * 0.5;
    }

    // Apply damping
    particle.velocity *= 0.99;

    if params.color_mode == 1u {
        // Velocity-based coloring
        let speed = length(particle.velocity);
        let norm_speed = min(speed / 5.0, 1.0);
        particle.color = vec4<f32>(norm_speed, 0.5 - norm_speed * 0.5, 1.0 - norm_speed, 1.0);
    } else if params.color_mode == 2u {
        // Position-based coloring
        let norm_pos = (particle.position / bounds + 1.0) * 0.5;
        particle.color = vec4<f32>(norm_pos.x, norm_pos.y, norm_pos.z, 1.0);
    }

    particles[index] = particle;
}
