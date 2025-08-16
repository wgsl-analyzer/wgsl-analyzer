fn fn_sig_args_generic(b: array<array<array<array<array<array<array<u32, 10>, 1>, 5>, >, 2>, 3>, 4>, 5>,)) -> vec4f {}

fn fn_sig_ret_generic() ->  b: array<array<array<array<array<array<array<u32, 10>, 1>, 5>, >, 2>, 3>, 4>, 5> {}

fn super_long_fn_name_that_is_very_long_and_has_many_parts_and_might_be_too_long_to_comprehend() {}

@compute @workgroup_size(1, 1, 1) fn fn_sig_attributes(
    @location(1) thing: vec3f,
    @location(2) thing: vec3f,
    @location(3) thing: vec3f,
    @builtin(position) thing: vec3f,
) -> @location(0) vec3f{}
