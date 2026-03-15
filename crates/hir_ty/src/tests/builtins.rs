#![expect(non_snake_case, reason = "name based on WGSL builtins")]

use expect_test::expect;

use crate::tests::check_infer;

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
            4..5 'x': ref<texture_2d<f32>>
            28..29 'y': ref<texture_external>
            53..54 's': ref<sampler>
            85..86 'a': vec4<f32>
            100..150 'textur... 0.0))': vec4<f32>
            129..130 'x': ref<texture_2d<f32>>
            132..133 's': ref<sampler>
            135..149 'vec2(0.0, 0.0)': vec2<float>
            140..143 '0.0': float
            145..148 '0.0': float
            160..161 'b': vec4<f32>
            175..225 'textur... 0.0))': vec4<f32>
            204..205 'y': ref<texture_external>
            207..208 's': ref<sampler>
            210..224 'vec2(0.0, 0.0)': vec2<float>
            215..218 '0.0': float
            220..223 '0.0': float
        "#]],
    );
}
