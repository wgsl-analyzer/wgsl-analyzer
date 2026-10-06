#import my_module;
#import my_other_module as mod2;

#import my_third_mod::{my_func, my_const}

#import my_package::{
    first_module::{item_one as item, item_two},
    second_module::submodule,
}

#import bevy_pbr::lighting as Lighting

#define USER_NUMBER 42

override fn Lighting::point_light (world_position: vec3<f32>) -> vec3<f32> {
    let original = Lighting::point_light(world_position);
    let quantized = vec3<u32>(original * 3.0);
    return vec3<f32>(quantized) / 3.0;
}

fn get_number() -> f32 {
    #ifdef BIG_NUMBER
        return 999.0;
    #else if USER_NUMBER > 1
        return f32(#USER_NUMBER);
    #else
        return 0.999;
    #endif
}

fn main() -> f32 {
    let x = my_module::my_func();
    let y = mod2::my_other_func();
    let z = my_func(my_const);

    let new_item = item + item_two + submodule::subitem + my_package::third_module::item;

    return x * y;
}
