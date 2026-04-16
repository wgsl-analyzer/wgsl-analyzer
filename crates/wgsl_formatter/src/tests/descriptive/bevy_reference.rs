use expect_test::expect;

use crate::test_util::check;

#[test]
fn format_bevy_function_1() {
    // We do not follow existing format
    // but break it into multiline https://discord.com/channels/1289346613185351722/1487480096909426698
    check(
        "fn directional_light(light: DirectionalLight, roughness: f32, NdotV: f32, normal: vec3<f32>, view: vec3<f32>, R: vec3<f32>, F0: vec3<f32>, diffuseColor: vec3<f32>) -> vec3<f32> {}",
        expect![[r#"
            fn directional_light(
                light: DirectionalLight,
                roughness: f32,
                NdotV: f32,
                normal: vec3<f32>,
                view: vec3<f32>,
                R: vec3<f32>,
                F0: vec3<f32>,
                diffuseColor: vec3<f32>,
            ) -> vec3<f32> {}
        "#]],
    );
}

#[test]
fn format_bevy_function_2() {
    // We do not follow existing format
    // but break it into multiline https://discord.com/channels/1289346613185351722/1487480096909426698
    check(
        "fn specular(f0: vec3<f32>, roughness: f32, h: vec3<f32>, NoV: f32, NoL: f32,
              NoH: f32, LoH: f32, specularIntensity: f32) -> vec3<f32> {}",
        expect![[r#"
            fn specular(
                f0: vec3<f32>,
                roughness: f32,
                h: vec3<f32>,
                NoV: f32,
                NoL: f32,
                NoH: f32,
                LoH: f32,
                specularIntensity: f32,
            ) -> vec3<f32> {}
        "#]],
    );
}
