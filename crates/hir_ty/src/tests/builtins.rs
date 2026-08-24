#![expect(non_snake_case, reason = "name based on WGSL builtins")]

mod array;
mod atomic;
mod bit_reinterpretation;
mod derivative;
mod logical;
mod numeric;
mod numeric_invalid;
mod operators;
mod value_constructor;
mod zero_value;

use expect_test::expect;

use crate::tests::check_infer;

#[test]
fn textureTypes() {
    check_infer(
        "
        var _texture_depth_multisampled_2d: texture_depth_multisampled_2d;
        var _texture_external: texture_external;
        var _texture_depth_2d: texture_depth_2d;
        var _texture_depth_2d_array: texture_depth_2d_array;
        var _texture_depth_cube: texture_depth_cube;
        var _texture_depth_cube_array: texture_depth_cube_array;
        var _sampler: sampler;
        var _sampler_comparison: sampler_comparison;

        // generators
        var _texture_1d: texture_1d<f32>;
        var _texture_2d: texture_2d<f32>;
        var _texture_2d_array: texture_2d_array<f32>;
        var _texture_3d: texture_3d<f32>;
        var _texture_cube: texture_cube<f32>;
        var _texture_cube_array: texture_cube_array<f32>;

        var _texture_multisampled_2d: texture_multisampled_2d<f32>;

        var _texture_storage_1d: texture_storage_1d<rgba8unorm, read_write>;
        var _texture_storage_2d: texture_storage_2d<rgba8unorm, read_write>;
        var _texture_storage_2d_array: texture_storage_2d_array<rgba8unorm, read_write>;
        var _texture_storage_3d: texture_storage_3d<rgba8unorm, read_write>;
",
        expect![[r#"
            4..34 '_textu...led_2d': ref<handle, texture_depth_multisampled_2d, read>
            71..88 '_textu...ternal': ref<handle, texture_external, read>
            112..129 '_textu...pth_2d': ref<handle, texture_depth_2d, read>
            153..176 '_textu..._array': ref<handle, texture_depth_2d_array, read>
            206..225 '_textu...h_cube': ref<handle, texture_depth_cube, read>
            251..276 '_textu..._array': ref<handle, texture_depth_cube_array, read>
            308..316 '_sampler': ref<handle, sampler, read>
            331..350 '_sampl...arison': ref<handle, sampler_comparison, read>
            391..402 '_texture_1d': ref<handle, texture_1d<f32>, read>
            425..436 '_texture_2d': ref<handle, texture_2d<f32>, read>
            459..476 '_textu..._array': ref<handle, texture_2d_array<f32>, read>
            505..516 '_texture_3d': ref<handle, texture_3d<f32>, read>
            539..552 '_texture_cube': ref<handle, texture_cube<f32>, read>
            577..596 '_textu..._array': ref<handle, texture_cube_array<f32>, read>
            628..652 '_textu...led_2d': ref<handle, texture_multisampled_2d<f32>, read>
            689..708 '_textu...age_1d': ref<handle, texture_storage_1d<rgba8unorm,read_write>, read>
            758..777 '_textu...age_2d': ref<handle, texture_storage_2d<rgba8unorm,read_write>, read>
            827..852 '_textu..._array': ref<handle, texture_storage_2d_array<rgba8unorm,read_write>, read>
            908..927 '_textu...age_3d': ref<handle, texture_storage_3d<rgba8unorm,read_write>, read>
        "#]],
    );
}

#[test]
fn textureSampleBaseClampToEdge() {
    check_infer(
        "
var x: texture_2d<f32>;
var y: texture_external;
var s: sampler;

fn foo() {
    let a: vec4<f32> = textureSampleBaseClampToEdge(x, s, vec2(0.0, 0.0));
    let b: vec4<f32> = textureSampleBaseClampToEdge(y, s, vec2(0.0, 0.0));
}
",
        expect![[r#"
            4..5 'x': ref<handle, texture_2d<f32>, read>
            28..29 'y': ref<handle, texture_external, read>
            53..54 's': ref<handle, sampler, read>
            85..86 'a': vec4<f32>
            100..150 'textur... 0.0))': vec4<f32>
            129..130 'x': ref<handle, texture_2d<f32>, read>
            132..133 's': ref<handle, sampler, read>
            135..149 'vec2(0.0, 0.0)': vec2<float>
            140..143 '0.0': float
            145..148 '0.0': float
            160..161 'b': vec4<f32>
            175..225 'textur... 0.0))': vec4<f32>
            204..205 'y': ref<handle, texture_external, read>
            207..208 's': ref<handle, sampler, read>
            210..224 'vec2(0.0, 0.0)': vec2<float>
            215..218 '0.0': float
            220..223 '0.0': float
        "#]],
    );
}
