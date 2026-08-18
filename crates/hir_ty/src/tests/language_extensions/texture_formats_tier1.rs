#![expect(clippy::too_many_lines, reason = "snapshot tests")]

use expect_test::expect;

use crate::tests::check_infer;

#[test]
fn texture_storage_1d() {
    check_infer(
        "
@group(0) @binding(0)  var t_rgba16unorm   : texture_storage_1d<rgba16unorm, write>;

@compute @workgroup_size(1)
fn main() {
    textureStore(t_rgba16unorm,   vec2i(0), textureLoad(t_rgba16unorm,   vec2i(0)));
}
        ",
        expect![[r#"
            27..40 't_rgba16unorm': ref<handle, texture_storage_1d<rgba16unorm,write>, read>
            130..209 'textur...i(0)))': [error]
            143..156 't_rgba16unorm': ref<handle, texture_storage_1d<rgba16unorm,write>, read>
            160..168 'vec2i(0)': vec2<i32>
            166..167 '0': integer
            170..208 'textur...2i(0))': vec4<f32>
            182..195 't_rgba16unorm': ref<handle, texture_storage_1d<rgba16unorm,write>, read>
            199..207 'vec2i(0)': vec2<i32>
            205..206 '0': integer
        "#]],
    );
}

#[test]
fn texture_storage_2d() {
    check_infer(
        "
@group(0) @binding(0)  var t_rgba16unorm   : texture_storage_2d<rgba16unorm, write>;
@group(0) @binding(1)  var t_rgba16snorm   : texture_storage_2d<rgba16snorm, write>;

@group(0) @binding(2)  var t_rg8unorm      : texture_storage_2d<rg8unorm, write>;
@group(0) @binding(3)  var t_rg8snorm      : texture_storage_2d<rg8snorm, write>;
@group(0) @binding(4)  var t_rg8uint       : texture_storage_2d<rg8uint, write>;
@group(0) @binding(5)  var t_rg8sint       : texture_storage_2d<rg8sint, write>;

@group(0) @binding(6)  var t_rg16unorm     : texture_storage_2d<rg16unorm, write>;
@group(0) @binding(7)  var t_rg16snorm     : texture_storage_2d<rg16snorm, write>;
@group(0) @binding(8)  var t_rg16uint      : texture_storage_2d<rg16uint, write>;
@group(0) @binding(9)  var t_rg16sint      : texture_storage_2d<rg16sint, write>;
@group(0) @binding(10) var t_rg16float     : texture_storage_2d<rg16float, write>;

@group(0) @binding(11) var t_r8unorm       : texture_storage_2d<r8unorm, write>;
@group(0) @binding(12) var t_r8snorm       : texture_storage_2d<r8snorm, write>;
@group(0) @binding(13) var t_r8uint        : texture_storage_2d<r8uint, write>;
@group(0) @binding(14) var t_r8sint        : texture_storage_2d<r8sint, write>;

@group(0) @binding(15) var t_r16unorm      : texture_storage_2d<r16unorm, write>;
@group(0) @binding(16) var t_r16snorm      : texture_storage_2d<r16snorm, write>;
@group(0) @binding(17) var t_r16uint       : texture_storage_2d<r16uint, write>;
@group(0) @binding(18) var t_r16sint       : texture_storage_2d<r16sint, write>;
@group(0) @binding(19) var t_r16float      : texture_storage_2d<r16float, write>;

@group(0) @binding(20) var t_rgb10a2unorm  : texture_storage_2d<rgb10a2unorm, write>;
@group(0) @binding(21) var t_rgb10a2uint   : texture_storage_2d<rgb10a2uint, write>;
// @group(0) @binding(22) var t_rg11b10ufloat : texture_storage_2d<rg11b10ufloat, write>;

@compute @workgroup_size(1)
fn main() {
    textureStore(t_rgba16unorm,   vec2i(0), textureLoad(t_rgba16unorm,   vec2i(0)));
    textureStore(t_rgba16snorm,   vec2i(0), textureLoad(t_rgba16snorm,   vec2i(0)));
    textureStore(t_rg8unorm,      vec2i(0), textureLoad(t_rg8unorm,      vec2i(0)));
    textureStore(t_rg8snorm,      vec2i(0), textureLoad(t_rg8snorm,      vec2i(0)));
    textureStore(t_rg8uint,       vec2i(0), textureLoad(t_rg8uint,       vec2i(0)));
    textureStore(t_rg8sint,       vec2i(0), textureLoad(t_rg8sint,       vec2i(0)));
    textureStore(t_rg16unorm,     vec2i(0), textureLoad(t_rg16unorm,     vec2i(0)));
    textureStore(t_rg16snorm,     vec2i(0), textureLoad(t_rg16snorm,     vec2i(0)));
    textureStore(t_rg16uint,      vec2i(0), textureLoad(t_rg16uint,      vec2i(0)));
    textureStore(t_rg16sint,      vec2i(0), textureLoad(t_rg16sint,      vec2i(0)));
    textureStore(t_rg16float,     vec2i(0), textureLoad(t_rg16float,     vec2i(0)));
    textureStore(t_r8unorm,       vec2i(0), textureLoad(t_r8unorm,       vec2i(0)));
    textureStore(t_r8snorm,       vec2i(0), textureLoad(t_r8snorm,       vec2i(0)));
    textureStore(t_r8uint,        vec2i(0), textureLoad(t_r8uint,        vec2i(0)));
    textureStore(t_r8sint,        vec2i(0), textureLoad(t_r8sint,        vec2i(0)));
    textureStore(t_r16unorm,      vec2i(0), textureLoad(t_r16unorm,      vec2i(0)));
    textureStore(t_r16snorm,      vec2i(0), textureLoad(t_r16snorm,      vec2i(0)));
    textureStore(t_r16uint,       vec2i(0), textureLoad(t_r16uint,       vec2i(0)));
    textureStore(t_r16sint,       vec2i(0), textureLoad(t_r16sint,       vec2i(0)));
    textureStore(t_r16float,      vec2i(0), textureLoad(t_r16float,      vec2i(0)));
    textureStore(t_rgb10a2unorm,  vec2i(0), textureLoad(t_rgb10a2unorm,  vec2i(0)));
    textureStore(t_rgb10a2uint,   vec2i(0), textureLoad(t_rgb10a2uint,   vec2i(0)));
    // textureStore(t_rg11b10ufloat, vec2i(0), textureLoad(t_rg11b10ufloat, vec2i(0)));
}
        ",
        expect![[r#"
            27..40 't_rgba16unorm': ref<handle, texture_storage_2d<rgba16unorm,write>, read>
            112..125 't_rgba16snorm': ref<handle, texture_storage_2d<rgba16snorm,write>, read>
            198..208 't_rg8unorm': ref<handle, texture_storage_2d<rg8unorm,write>, read>
            280..290 't_rg8snorm': ref<handle, texture_storage_2d<rg8snorm,write>, read>
            362..371 't_rg8uint': ref<handle, texture_storage_2d<rg8uint,write>, read>
            443..452 't_rg8sint': ref<handle, texture_storage_2d<rg8sint,write>, read>
            525..536 't_rg16unorm': ref<handle, texture_storage_2d<rg16unorm,write>, read>
            608..619 't_rg16snorm': ref<handle, texture_storage_2d<rg16snorm,write>, read>
            691..701 't_rg16uint': ref<handle, texture_storage_2d<rg16uint,write>, read>
            773..783 't_rg16sint': ref<handle, texture_storage_2d<rg16sint,write>, read>
            855..866 't_rg16float': ref<handle, texture_storage_2d<rg16float,write>, read>
            939..948 't_r8unorm': ref<handle, texture_storage_2d<r8unorm,write>, read>
            1020..1029 't_r8snorm': ref<handle, texture_storage_2d<r8snorm,write>, read>
            1101..1109 't_r8uint': ref<handle, texture_storage_2d<r8uint,write>, read>
            1181..1189 't_r8sint': ref<handle, texture_storage_2d<r8sint,write>, read>
            1262..1272 't_r16unorm': ref<handle, texture_storage_2d<r16unorm,write>, read>
            1344..1354 't_r16snorm': ref<handle, texture_storage_2d<r16snorm,write>, read>
            1426..1435 't_r16uint': ref<handle, texture_storage_2d<r16uint,write>, read>
            1507..1516 't_r16sint': ref<handle, texture_storage_2d<r16sint,write>, read>
            1588..1598 't_r16float': ref<handle, texture_storage_2d<r16float,write>, read>
            1671..1685 't_rgb10a2unorm': ref<handle, texture_storage_2d<rgb10a2unorm,write>, read>
            1757..1770 't_rgb10a2uint': ref<handle, texture_storage_2d<rgb10a2uint,write>, read>
            1950..2029 'textur...i(0)))': [error]
            1963..1976 't_rgba16unorm': ref<handle, texture_storage_2d<rgba16unorm,write>, read>
            1980..1988 'vec2i(0)': vec2<i32>
            1986..1987 '0': integer
            1990..2028 'textur...2i(0))': vec4<f32>
            2002..2015 't_rgba16unorm': ref<handle, texture_storage_2d<rgba16unorm,write>, read>
            2019..2027 'vec2i(0)': vec2<i32>
            2025..2026 '0': integer
            2035..2114 'textur...i(0)))': [error]
            2048..2061 't_rgba16snorm': ref<handle, texture_storage_2d<rgba16snorm,write>, read>
            2065..2073 'vec2i(0)': vec2<i32>
            2071..2072 '0': integer
            2075..2113 'textur...2i(0))': vec4<f32>
            2087..2100 't_rgba16snorm': ref<handle, texture_storage_2d<rgba16snorm,write>, read>
            2104..2112 'vec2i(0)': vec2<i32>
            2110..2111 '0': integer
            2120..2199 'textur...i(0)))': [error]
            2133..2143 't_rg8unorm': ref<handle, texture_storage_2d<rg8unorm,write>, read>
            2150..2158 'vec2i(0)': vec2<i32>
            2156..2157 '0': integer
            2160..2198 'textur...2i(0))': vec4<f32>
            2172..2182 't_rg8unorm': ref<handle, texture_storage_2d<rg8unorm,write>, read>
            2189..2197 'vec2i(0)': vec2<i32>
            2195..2196 '0': integer
            2205..2284 'textur...i(0)))': [error]
            2218..2228 't_rg8snorm': ref<handle, texture_storage_2d<rg8snorm,write>, read>
            2235..2243 'vec2i(0)': vec2<i32>
            2241..2242 '0': integer
            2245..2283 'textur...2i(0))': vec4<f32>
            2257..2267 't_rg8snorm': ref<handle, texture_storage_2d<rg8snorm,write>, read>
            2274..2282 'vec2i(0)': vec2<i32>
            2280..2281 '0': integer
            2290..2369 'textur...i(0)))': [error]
            2303..2312 't_rg8uint': ref<handle, texture_storage_2d<rg8uint,write>, read>
            2320..2328 'vec2i(0)': vec2<i32>
            2326..2327 '0': integer
            2330..2368 'textur...2i(0))': vec4<u32>
            2342..2351 't_rg8uint': ref<handle, texture_storage_2d<rg8uint,write>, read>
            2359..2367 'vec2i(0)': vec2<i32>
            2365..2366 '0': integer
            2375..2454 'textur...i(0)))': [error]
            2388..2397 't_rg8sint': ref<handle, texture_storage_2d<rg8sint,write>, read>
            2405..2413 'vec2i(0)': vec2<i32>
            2411..2412 '0': integer
            2415..2453 'textur...2i(0))': vec4<i32>
            2427..2436 't_rg8sint': ref<handle, texture_storage_2d<rg8sint,write>, read>
            2444..2452 'vec2i(0)': vec2<i32>
            2450..2451 '0': integer
            2460..2539 'textur...i(0)))': [error]
            2473..2484 't_rg16unorm': ref<handle, texture_storage_2d<rg16unorm,write>, read>
            2490..2498 'vec2i(0)': vec2<i32>
            2496..2497 '0': integer
            2500..2538 'textur...2i(0))': vec4<f32>
            2512..2523 't_rg16unorm': ref<handle, texture_storage_2d<rg16unorm,write>, read>
            2529..2537 'vec2i(0)': vec2<i32>
            2535..2536 '0': integer
            2545..2624 'textur...i(0)))': [error]
            2558..2569 't_rg16snorm': ref<handle, texture_storage_2d<rg16snorm,write>, read>
            2575..2583 'vec2i(0)': vec2<i32>
            2581..2582 '0': integer
            2585..2623 'textur...2i(0))': vec4<f32>
            2597..2608 't_rg16snorm': ref<handle, texture_storage_2d<rg16snorm,write>, read>
            2614..2622 'vec2i(0)': vec2<i32>
            2620..2621 '0': integer
            2630..2709 'textur...i(0)))': [error]
            2643..2653 't_rg16uint': ref<handle, texture_storage_2d<rg16uint,write>, read>
            2660..2668 'vec2i(0)': vec2<i32>
            2666..2667 '0': integer
            2670..2708 'textur...2i(0))': vec4<u32>
            2682..2692 't_rg16uint': ref<handle, texture_storage_2d<rg16uint,write>, read>
            2699..2707 'vec2i(0)': vec2<i32>
            2705..2706 '0': integer
            2715..2794 'textur...i(0)))': [error]
            2728..2738 't_rg16sint': ref<handle, texture_storage_2d<rg16sint,write>, read>
            2745..2753 'vec2i(0)': vec2<i32>
            2751..2752 '0': integer
            2755..2793 'textur...2i(0))': vec4<i32>
            2767..2777 't_rg16sint': ref<handle, texture_storage_2d<rg16sint,write>, read>
            2784..2792 'vec2i(0)': vec2<i32>
            2790..2791 '0': integer
            2800..2879 'textur...i(0)))': [error]
            2813..2824 't_rg16float': ref<handle, texture_storage_2d<rg16float,write>, read>
            2830..2838 'vec2i(0)': vec2<i32>
            2836..2837 '0': integer
            2840..2878 'textur...2i(0))': vec4<f32>
            2852..2863 't_rg16float': ref<handle, texture_storage_2d<rg16float,write>, read>
            2869..2877 'vec2i(0)': vec2<i32>
            2875..2876 '0': integer
            2885..2964 'textur...i(0)))': [error]
            2898..2907 't_r8unorm': ref<handle, texture_storage_2d<r8unorm,write>, read>
            2915..2923 'vec2i(0)': vec2<i32>
            2921..2922 '0': integer
            2925..2963 'textur...2i(0))': vec4<f32>
            2937..2946 't_r8unorm': ref<handle, texture_storage_2d<r8unorm,write>, read>
            2954..2962 'vec2i(0)': vec2<i32>
            2960..2961 '0': integer
            2970..3049 'textur...i(0)))': [error]
            2983..2992 't_r8snorm': ref<handle, texture_storage_2d<r8snorm,write>, read>
            3000..3008 'vec2i(0)': vec2<i32>
            3006..3007 '0': integer
            3010..3048 'textur...2i(0))': vec4<f32>
            3022..3031 't_r8snorm': ref<handle, texture_storage_2d<r8snorm,write>, read>
            3039..3047 'vec2i(0)': vec2<i32>
            3045..3046 '0': integer
            3055..3134 'textur...i(0)))': [error]
            3068..3076 't_r8uint': ref<handle, texture_storage_2d<r8uint,write>, read>
            3085..3093 'vec2i(0)': vec2<i32>
            3091..3092 '0': integer
            3095..3133 'textur...2i(0))': vec4<u32>
            3107..3115 't_r8uint': ref<handle, texture_storage_2d<r8uint,write>, read>
            3124..3132 'vec2i(0)': vec2<i32>
            3130..3131 '0': integer
            3140..3219 'textur...i(0)))': [error]
            3153..3161 't_r8sint': ref<handle, texture_storage_2d<r8sint,write>, read>
            3170..3178 'vec2i(0)': vec2<i32>
            3176..3177 '0': integer
            3180..3218 'textur...2i(0))': vec4<i32>
            3192..3200 't_r8sint': ref<handle, texture_storage_2d<r8sint,write>, read>
            3209..3217 'vec2i(0)': vec2<i32>
            3215..3216 '0': integer
            3225..3304 'textur...i(0)))': [error]
            3238..3248 't_r16unorm': ref<handle, texture_storage_2d<r16unorm,write>, read>
            3255..3263 'vec2i(0)': vec2<i32>
            3261..3262 '0': integer
            3265..3303 'textur...2i(0))': vec4<f32>
            3277..3287 't_r16unorm': ref<handle, texture_storage_2d<r16unorm,write>, read>
            3294..3302 'vec2i(0)': vec2<i32>
            3300..3301 '0': integer
            3310..3389 'textur...i(0)))': [error]
            3323..3333 't_r16snorm': ref<handle, texture_storage_2d<r16snorm,write>, read>
            3340..3348 'vec2i(0)': vec2<i32>
            3346..3347 '0': integer
            3350..3388 'textur...2i(0))': vec4<f32>
            3362..3372 't_r16snorm': ref<handle, texture_storage_2d<r16snorm,write>, read>
            3379..3387 'vec2i(0)': vec2<i32>
            3385..3386 '0': integer
            3395..3474 'textur...i(0)))': [error]
            3408..3417 't_r16uint': ref<handle, texture_storage_2d<r16uint,write>, read>
            3425..3433 'vec2i(0)': vec2<i32>
            3431..3432 '0': integer
            3435..3473 'textur...2i(0))': vec4<u32>
            3447..3456 't_r16uint': ref<handle, texture_storage_2d<r16uint,write>, read>
            3464..3472 'vec2i(0)': vec2<i32>
            3470..3471 '0': integer
            3480..3559 'textur...i(0)))': [error]
            3493..3502 't_r16sint': ref<handle, texture_storage_2d<r16sint,write>, read>
            3510..3518 'vec2i(0)': vec2<i32>
            3516..3517 '0': integer
            3520..3558 'textur...2i(0))': vec4<i32>
            3532..3541 't_r16sint': ref<handle, texture_storage_2d<r16sint,write>, read>
            3549..3557 'vec2i(0)': vec2<i32>
            3555..3556 '0': integer
            3565..3644 'textur...i(0)))': [error]
            3578..3588 't_r16float': ref<handle, texture_storage_2d<r16float,write>, read>
            3595..3603 'vec2i(0)': vec2<i32>
            3601..3602 '0': integer
            3605..3643 'textur...2i(0))': vec4<f32>
            3617..3627 't_r16float': ref<handle, texture_storage_2d<r16float,write>, read>
            3634..3642 'vec2i(0)': vec2<i32>
            3640..3641 '0': integer
            3650..3729 'textur...i(0)))': [error]
            3663..3677 't_rgb10a2unorm': ref<handle, texture_storage_2d<rgb10a2unorm,write>, read>
            3680..3688 'vec2i(0)': vec2<i32>
            3686..3687 '0': integer
            3690..3728 'textur...2i(0))': vec4<f32>
            3702..3716 't_rgb10a2unorm': ref<handle, texture_storage_2d<rgb10a2unorm,write>, read>
            3719..3727 'vec2i(0)': vec2<i32>
            3725..3726 '0': integer
            3735..3814 'textur...i(0)))': [error]
            3748..3761 't_rgb10a2uint': ref<handle, texture_storage_2d<rgb10a2uint,write>, read>
            3765..3773 'vec2i(0)': vec2<i32>
            3771..3772 '0': integer
            3775..3813 'textur...2i(0))': vec4<u32>
            3787..3800 't_rgb10a2uint': ref<handle, texture_storage_2d<rgb10a2uint,write>, read>
            3804..3812 'vec2i(0)': vec2<i32>
            3810..3811 '0': integer
        "#]],
    );
}

#[test]
fn texture_storage_2d_array() {
    check_infer(
        "
@group(0) @binding(0) var t_rgba16unorm: texture_storage_2d_array<rgba16unorm, read_write>;

@compute @workgroup_size(1)
fn main() {
    let loaded = textureLoad(t_rgba16unorm, vec2i(0));
    textureStore(t_rgba16unorm, vec2i(0), loaded);
}
        ",
        expect![[r#"
            26..39 't_rgba16unorm': ref<handle, texture_storage_2d_array<rgba16unorm,read_write>, read>
            141..147 'loaded': vec4<f32>
            150..186 'textur...2i(0))': vec4<f32>
            162..175 't_rgba16unorm': ref<handle, texture_storage_2d_array<rgba16unorm,read_write>, read>
            177..185 'vec2i(0)': vec2<i32>
            183..184 '0': integer
            192..237 'textur...oaded)': [error]
            205..218 't_rgba16unorm': ref<handle, texture_storage_2d_array<rgba16unorm,read_write>, read>
            220..228 'vec2i(0)': vec2<i32>
            226..227 '0': integer
            230..236 'loaded': vec4<f32>
        "#]],
    );
}
