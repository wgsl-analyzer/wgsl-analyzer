#![expect(clippy::too_many_lines, reason = "snapshot test data")]

use expect_test::expect;
use syntax::ExtensionsConfig;

use crate::tests::check_infer;

#[test]
fn array() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    array<i32, 1>(1);
    array(1);
}
",
        expect![[r#"
            15..31 'array<... 1>(1)': array<i32, 1>
            29..30 '1': integer
            37..45 'array(1)': array<integer, 1>
            43..44 '1': integer
        "#]],
    );
}

#[test]
fn bool() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    bool(1);
}
",
        expect![[r#"
            15..22 'bool(1)': bool
            20..21 '1': integer
        "#]],
    );
}

#[test]
fn f16() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    f16(1);
}
",
        expect![[r#"
            27..33 'f16(1)': f16
            31..32 '1': integer
        "#]],
    );
}

#[test]
fn f32() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    f32(1);
}
",
        expect![[r#"
            15..21 'f32(1)': f32
            19..20 '1': integer
        "#]],
    );
}

#[test]
fn i32() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    i32(1);
}
",
        expect![[r#"
            15..21 'i32(1)': i32
            19..20 '1': integer
        "#]],
    );
}

#[test]
fn mat2x2() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let abs_abs_abs_abs = mat2x2(1, 2, 3, 4);
    let f16_f16_f16_f16 = mat2x2(1h, 2h, 3h, 4h);
    let f32_f32_f32_f32 = mat2x2(1f, 2f, 3f, 4f);
    let __f16__f16_f16_f16_f16 = mat2x2<f16>(1h, 2h, 3h, 4h);
    let __f32__f32_f32_f32_f32 = mat2x2<f32>(1f, 2f, 3f, 4f);

    let vec2_vec2 = mat2x2(vec2(1, 2), vec2(3, 4));
    let vec2h_vec2h = mat2x2(vec2h(1h, 2h), vec2h(3, 4));
    let vec2f_vec2f = mat2x2(vec2(1f, 2f), vec2f(3, 4));
    let __f16__vec2h_vec2h = mat2x2<f16>(vec2h(1, 2), vec2h(3, 4));
    let __f32__vec2f_vec2f = mat2x2<f32>(vec2f(1, 2), vec2f(3, 4));

    let __f16__mat2x2 = mat2x2<f16>(abs_abs_abs_abs);
    let __f16__mat2x2h = mat2x2<f16>(f16_f16_f16_f16);
    let __f16__mat2x2f = mat2x2<f16>(f32_f32_f32_f32);
    let __f32__mat2x2 = mat2x2<f32>(abs_abs_abs_abs);
    let __f32__mat2x2h = mat2x2<f32>(f16_f16_f16_f16);
    let __f32__mat2x2f = mat2x2<f32>(f32_f32_f32_f32);
    let mat2x2_identity = mat2x2(abs_abs_abs_abs);
    let mat2x2h_identity = mat2x2(f16_f16_f16_f16);
    let mat2x2f_identity = mat2x2(f32_f32_f32_f32);
}
",
        expect![[r#"
            19..34 'abs_abs_abs_abs': mat2x2<f32>
            37..55 'mat2x2... 3, 4)': mat2x2<float>
            44..45 '1': integer
            47..48 '2': integer
            50..51 '3': integer
            53..54 '4': integer
            65..80 'f16_f16_f16_f16': mat2x2<f16>
            83..105 'mat2x2...h, 4h)': mat2x2<f16>
            90..92 '1h': f16
            94..96 '2h': f16
            98..100 '3h': f16
            102..104 '4h': f16
            115..130 'f32_f32_f32_f32': mat2x2<f32>
            133..155 'mat2x2...f, 4f)': mat2x2<f32>
            140..142 '1f': f32
            144..146 '2f': f32
            148..150 '3f': f32
            152..154 '4f': f32
            165..187 '__f16_...16_f16': mat2x2<f16>
            190..217 'mat2x2...h, 4h)': mat2x2<f16>
            202..204 '1h': f16
            206..208 '2h': f16
            210..212 '3h': f16
            214..216 '4h': f16
            227..249 '__f32_...32_f32': mat2x2<f32>
            252..279 'mat2x2...f, 4f)': mat2x2<f32>
            264..266 '1f': f32
            268..270 '2f': f32
            272..274 '3f': f32
            276..278 '4f': f32
            290..299 'vec2_vec2': mat2x2<f32>
            302..332 'mat2x2...3, 4))': mat2x2<float>
            309..319 'vec2(1, 2)': vec2<integer>
            314..315 '1': integer
            317..318 '2': integer
            321..331 'vec2(3, 4)': vec2<integer>
            326..327 '3': integer
            329..330 '4': integer
            342..353 'vec2h_vec2h': mat2x2<f16>
            356..390 'mat2x2...3, 4))': mat2x2<f16>
            363..376 'vec2h(1h, 2h)': vec2<f16>
            369..371 '1h': f16
            373..375 '2h': f16
            378..389 'vec2h(3, 4)': vec2<f16>
            384..385 '3': integer
            387..388 '4': integer
            400..411 'vec2f_vec2f': mat2x2<f32>
            414..447 'mat2x2...3, 4))': mat2x2<f32>
            421..433 'vec2(1f, 2f)': vec2<f32>
            426..428 '1f': f32
            430..432 '2f': f32
            435..446 'vec2f(3, 4)': vec2<f32>
            441..442 '3': integer
            444..445 '4': integer
            457..475 '__f16_..._vec2h': mat2x2<f16>
            478..515 'mat2x2...3, 4))': mat2x2<f16>
            490..501 'vec2h(1, 2)': vec2<f16>
            496..497 '1': integer
            499..500 '2': integer
            503..514 'vec2h(3, 4)': vec2<f16>
            509..510 '3': integer
            512..513 '4': integer
            525..543 '__f32_..._vec2f': mat2x2<f32>
            546..583 'mat2x2...3, 4))': mat2x2<f32>
            558..569 'vec2f(1, 2)': vec2<f32>
            564..565 '1': integer
            567..568 '2': integer
            571..582 'vec2f(3, 4)': vec2<f32>
            577..578 '3': integer
            580..581 '4': integer
            594..607 '__f16__mat2x2': mat2x2<f16>
            610..638 'mat2x2...s_abs)': mat2x2<f16>
            622..637 'abs_abs_abs_abs': mat2x2<f32>
            648..662 '__f16__mat2x2h': mat2x2<f16>
            665..693 'mat2x2...6_f16)': mat2x2<f16>
            677..692 'f16_f16_f16_f16': mat2x2<f16>
            703..717 '__f16__mat2x2f': mat2x2<f16>
            720..748 'mat2x2...2_f32)': mat2x2<f16>
            732..747 'f32_f32_f32_f32': mat2x2<f32>
            758..771 '__f32__mat2x2': mat2x2<f32>
            774..802 'mat2x2...s_abs)': mat2x2<f32>
            786..801 'abs_abs_abs_abs': mat2x2<f32>
            812..826 '__f32__mat2x2h': mat2x2<f32>
            829..857 'mat2x2...6_f16)': mat2x2<f32>
            841..856 'f16_f16_f16_f16': mat2x2<f16>
            867..881 '__f32__mat2x2f': mat2x2<f32>
            884..912 'mat2x2...2_f32)': mat2x2<f32>
            896..911 'f32_f32_f32_f32': mat2x2<f32>
            922..937 'mat2x2_identity': mat2x2<f32>
            940..963 'mat2x2...s_abs)': mat2x2<f32>
            947..962 'abs_abs_abs_abs': mat2x2<f32>
            973..989 'mat2x2...entity': mat2x2<f16>
            992..1015 'mat2x2...6_f16)': mat2x2<f16>
            999..1014 'f16_f16_f16_f16': mat2x2<f16>
            1025..1041 'mat2x2...entity': mat2x2<f32>
            1044..1067 'mat2x2...2_f32)': mat2x2<f32>
            1051..1066 'f32_f32_f32_f32': mat2x2<f32>
        "#]],
    );
}
#[test]
fn mat2x3() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let abs_abs_abs_abs_abs_abs = mat2x3(1, 2, 3, 4, 5, 6);
    let f16_f16_f16_f16_f16_f16 = mat2x3(1h, 2h, 3h, 4h, 5h, 6h);
    let f32_f32_f32_f32_f32_f32 = mat2x3(1f, 2f, 3f, 4f, 5f, 6f);
    let __f16__f16_f16_f16_f16_f16_f16 = mat2x3<f16>(1h, 2h, 3h, 4h, 5h, 6h);
    let __f32__f32_f32_f32_f32_f32_f32 = mat2x3<f32>(1f, 2f, 3f, 4f, 5f, 6f);

    let vec3_vec3 = mat2x3(vec3(1, 2, 3), vec3(4, 5, 6));
    let vec3h_vec3h = mat2x3(vec3h(1h, 2h, 3h), vec3h(4, 5, 6));
    let vec3f_vec3f = mat2x3(vec3(1f, 2f, 3f), vec3f(4, 5, 6));
    let __f16__vec3h_vec3h = mat2x3<f16>(vec3h(1, 2, 3), vec3h(4, 5, 6));
    let __f32__vec3f_vec3f = mat2x3<f32>(vec3f(1, 2, 3), vec3f(4, 5, 6));

    let __f16__mat2x3 = mat2x3<f16>(abs_abs_abs_abs_abs_abs);
    let __f16__mat2x3h = mat2x3<f16>(f16_f16_f16_f16_f16_f16);
    let __f16__mat2x3f = mat2x3<f16>(f32_f32_f32_f32_f32_f32);
    let __f32__mat2x3 = mat2x3<f32>(abs_abs_abs_abs_abs_abs);
    let __f32__mat2x3h = mat2x3<f32>(f16_f16_f16_f16_f16_f16);
    let __f32__mat2x3f = mat2x3<f32>(f32_f32_f32_f32_f32_f32);
    let mat2x3_identity = mat2x3(abs_abs_abs_abs_abs_abs);
    let mat2x3h_identity = mat2x3(f16_f16_f16_f16_f16_f16);
    let mat2x3f_identity = mat2x3(f32_f32_f32_f32_f32_f32);
}
",
        expect![[r#"
            19..42 'abs_ab...bs_abs': mat2x3<f32>
            45..69 'mat2x3... 5, 6)': mat2x3<float>
            52..53 '1': integer
            55..56 '2': integer
            58..59 '3': integer
            61..62 '4': integer
            64..65 '5': integer
            67..68 '6': integer
            79..102 'f16_f1...16_f16': mat2x3<f16>
            105..135 'mat2x3...h, 6h)': mat2x3<f16>
            112..114 '1h': f16
            116..118 '2h': f16
            120..122 '3h': f16
            124..126 '4h': f16
            128..130 '5h': f16
            132..134 '6h': f16
            145..168 'f32_f3...32_f32': mat2x3<f32>
            171..201 'mat2x3...f, 6f)': mat2x3<f32>
            178..180 '1f': f32
            182..184 '2f': f32
            186..188 '3f': f32
            190..192 '4f': f32
            194..196 '5f': f32
            198..200 '6f': f32
            211..241 '__f16_...16_f16': mat2x3<f16>
            244..279 'mat2x3...h, 6h)': mat2x3<f16>
            256..258 '1h': f16
            260..262 '2h': f16
            264..266 '3h': f16
            268..270 '4h': f16
            272..274 '5h': f16
            276..278 '6h': f16
            289..319 '__f32_...32_f32': mat2x3<f32>
            322..357 'mat2x3...f, 6f)': mat2x3<f32>
            334..336 '1f': f32
            338..340 '2f': f32
            342..344 '3f': f32
            346..348 '4f': f32
            350..352 '5f': f32
            354..356 '6f': f32
            368..377 'vec3_vec3': mat2x3<f32>
            380..416 'mat2x3...5, 6))': mat2x3<float>
            387..400 'vec3(1, 2, 3)': vec3<integer>
            392..393 '1': integer
            395..396 '2': integer
            398..399 '3': integer
            402..415 'vec3(4, 5, 6)': vec3<integer>
            407..408 '4': integer
            410..411 '5': integer
            413..414 '6': integer
            426..437 'vec3h_vec3h': mat2x3<f16>
            440..481 'mat2x3...5, 6))': mat2x3<f16>
            447..464 'vec3h(...h, 3h)': vec3<f16>
            453..455 '1h': f16
            457..459 '2h': f16
            461..463 '3h': f16
            466..480 'vec3h(4, 5, 6)': vec3<f16>
            472..473 '4': integer
            475..476 '5': integer
            478..479 '6': integer
            491..502 'vec3f_vec3f': mat2x3<f32>
            505..545 'mat2x3...5, 6))': mat2x3<f32>
            512..528 'vec3(1...f, 3f)': vec3<f32>
            517..519 '1f': f32
            521..523 '2f': f32
            525..527 '3f': f32
            530..544 'vec3f(4, 5, 6)': vec3<f32>
            536..537 '4': integer
            539..540 '5': integer
            542..543 '6': integer
            555..573 '__f16_..._vec3h': mat2x3<f16>
            576..619 'mat2x3...5, 6))': mat2x3<f16>
            588..602 'vec3h(1, 2, 3)': vec3<f16>
            594..595 '1': integer
            597..598 '2': integer
            600..601 '3': integer
            604..618 'vec3h(4, 5, 6)': vec3<f16>
            610..611 '4': integer
            613..614 '5': integer
            616..617 '6': integer
            629..647 '__f32_..._vec3f': mat2x3<f32>
            650..693 'mat2x3...5, 6))': mat2x3<f32>
            662..676 'vec3f(1, 2, 3)': vec3<f32>
            668..669 '1': integer
            671..672 '2': integer
            674..675 '3': integer
            678..692 'vec3f(4, 5, 6)': vec3<f32>
            684..685 '4': integer
            687..688 '5': integer
            690..691 '6': integer
            704..717 '__f16__mat2x3': mat2x3<f16>
            720..756 'mat2x3...s_abs)': mat2x3<f16>
            732..755 'abs_ab...bs_abs': mat2x3<f32>
            766..780 '__f16__mat2x3h': mat2x3<f16>
            783..819 'mat2x3...6_f16)': mat2x3<f16>
            795..818 'f16_f1...16_f16': mat2x3<f16>
            829..843 '__f16__mat2x3f': mat2x3<f16>
            846..882 'mat2x3...2_f32)': mat2x3<f16>
            858..881 'f32_f3...32_f32': mat2x3<f32>
            892..905 '__f32__mat2x3': mat2x3<f32>
            908..944 'mat2x3...s_abs)': mat2x3<f32>
            920..943 'abs_ab...bs_abs': mat2x3<f32>
            954..968 '__f32__mat2x3h': mat2x3<f32>
            971..1007 'mat2x3...6_f16)': mat2x3<f32>
            983..1006 'f16_f1...16_f16': mat2x3<f16>
            1017..1031 '__f32__mat2x3f': mat2x3<f32>
            1034..1070 'mat2x3...2_f32)': mat2x3<f32>
            1046..1069 'f32_f3...32_f32': mat2x3<f32>
            1080..1095 'mat2x3_identity': mat2x3<f32>
            1098..1129 'mat2x3...s_abs)': mat2x3<f32>
            1105..1128 'abs_ab...bs_abs': mat2x3<f32>
            1139..1155 'mat2x3...entity': mat2x3<f16>
            1158..1189 'mat2x3...6_f16)': mat2x3<f16>
            1165..1188 'f16_f1...16_f16': mat2x3<f16>
            1199..1215 'mat2x3...entity': mat2x3<f32>
            1218..1249 'mat2x3...2_f32)': mat2x3<f32>
            1225..1248 'f32_f3...32_f32': mat2x3<f32>
        "#]],
    );
}

#[test]
fn mat2x4() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let abs_abs_abs_abs_abs_abs_abs_abs = mat2x4(1, 2, 3, 4, 5, 6, 7, 8);
    let f16_f16_f16_f16_f16_f16_f16_f16 = mat2x4(1h, 2h, 3h, 4h, 5h, 6h, 7h, 8h);
    let f32_f32_f32_f32_f32_f32_f32_f32 = mat2x4(1f, 2f, 3f, 4f, 5f, 6f, 7f, 8f);
    let __f16__f16_f16_f16_f16_f16_f16_f16_f16 = mat2x4<f16>(1h, 2h, 3h, 4h, 5h, 6h, 7h, 8h);
    let __f32__f32_f32_f32_f32_f32_f32_f32_f32 = mat2x4<f32>(1f, 2f, 3f, 4f, 5f, 6f, 7f, 8f);

    let vec4_vec4 = mat2x4(vec4(1, 2, 3, 4), vec4(5, 6, 7, 8));
    let vec4h_vec4h = mat2x4(vec4h(1h, 2h, 3h, 4h), vec4h(5, 6, 7, 8));
    let vec4f_vec4f = mat2x4(vec4(1f, 2f, 3f, 4f), vec4f(5, 6, 7, 8));
    let __f16__vec4h_vec4h = mat2x4<f16>(vec4h(1, 2, 3, 4), vec4h(5, 6, 7, 8));
    let __f32__vec4f_vec4f = mat2x4<f32>(vec4f(1, 2, 3, 4), vec4f(5, 6, 7, 8));

    let __f16__mat2x4 = mat2x4<f16>(abs_abs_abs_abs_abs_abs_abs_abs);
    let __f16__mat2x4h = mat2x4<f16>(f16_f16_f16_f16_f16_f16_f16_f16);
    let __f16__mat2x4f = mat2x4<f16>(f32_f32_f32_f32_f32_f32_f32_f32);
    let __f32__mat2x4 = mat2x4<f32>(abs_abs_abs_abs_abs_abs_abs_abs);
    let __f32__mat2x4h = mat2x4<f32>(f16_f16_f16_f16_f16_f16_f16_f16);
    let __f32__mat2x4f = mat2x4<f32>(f32_f32_f32_f32_f32_f32_f32_f32);
    let mat2x4_identity = mat2x4(abs_abs_abs_abs_abs_abs_abs_abs);
    let mat2x4h_identity = mat2x4(f16_f16_f16_f16_f16_f16_f16_f16);
    let mat2x4f_identity = mat2x4(f32_f32_f32_f32_f32_f32_f32_f32);
}
",
        expect![[r#"
            19..50 'abs_ab...bs_abs': mat2x4<f32>
            53..83 'mat2x4... 7, 8)': mat2x4<float>
            60..61 '1': integer
            63..64 '2': integer
            66..67 '3': integer
            69..70 '4': integer
            72..73 '5': integer
            75..76 '6': integer
            78..79 '7': integer
            81..82 '8': integer
            93..124 'f16_f1...16_f16': mat2x4<f16>
            127..165 'mat2x4...h, 8h)': mat2x4<f16>
            134..136 '1h': f16
            138..140 '2h': f16
            142..144 '3h': f16
            146..148 '4h': f16
            150..152 '5h': f16
            154..156 '6h': f16
            158..160 '7h': f16
            162..164 '8h': f16
            175..206 'f32_f3...32_f32': mat2x4<f32>
            209..247 'mat2x4...f, 8f)': mat2x4<f32>
            216..218 '1f': f32
            220..222 '2f': f32
            224..226 '3f': f32
            228..230 '4f': f32
            232..234 '5f': f32
            236..238 '6f': f32
            240..242 '7f': f32
            244..246 '8f': f32
            257..295 '__f16_...16_f16': mat2x4<f16>
            298..341 'mat2x4...h, 8h)': mat2x4<f16>
            310..312 '1h': f16
            314..316 '2h': f16
            318..320 '3h': f16
            322..324 '4h': f16
            326..328 '5h': f16
            330..332 '6h': f16
            334..336 '7h': f16
            338..340 '8h': f16
            351..389 '__f32_...32_f32': mat2x4<f32>
            392..435 'mat2x4...f, 8f)': mat2x4<f32>
            404..406 '1f': f32
            408..410 '2f': f32
            412..414 '3f': f32
            416..418 '4f': f32
            420..422 '5f': f32
            424..426 '6f': f32
            428..430 '7f': f32
            432..434 '8f': f32
            446..455 'vec4_vec4': mat2x4<f32>
            458..500 'mat2x4...7, 8))': mat2x4<float>
            465..481 'vec4(1... 3, 4)': vec4<integer>
            470..471 '1': integer
            473..474 '2': integer
            476..477 '3': integer
            479..480 '4': integer
            483..499 'vec4(5... 7, 8)': vec4<integer>
            488..489 '5': integer
            491..492 '6': integer
            494..495 '7': integer
            497..498 '8': integer
            510..521 'vec4h_vec4h': mat2x4<f16>
            524..572 'mat2x4...7, 8))': mat2x4<f16>
            531..552 'vec4h(...h, 4h)': vec4<f16>
            537..539 '1h': f16
            541..543 '2h': f16
            545..547 '3h': f16
            549..551 '4h': f16
            554..571 'vec4h(... 7, 8)': vec4<f16>
            560..561 '5': integer
            563..564 '6': integer
            566..567 '7': integer
            569..570 '8': integer
            582..593 'vec4f_vec4f': mat2x4<f32>
            596..643 'mat2x4...7, 8))': mat2x4<f32>
            603..623 'vec4(1...f, 4f)': vec4<f32>
            608..610 '1f': f32
            612..614 '2f': f32
            616..618 '3f': f32
            620..622 '4f': f32
            625..642 'vec4f(... 7, 8)': vec4<f32>
            631..632 '5': integer
            634..635 '6': integer
            637..638 '7': integer
            640..641 '8': integer
            653..671 '__f16_..._vec4h': mat2x4<f16>
            674..723 'mat2x4...7, 8))': mat2x4<f16>
            686..703 'vec4h(... 3, 4)': vec4<f16>
            692..693 '1': integer
            695..696 '2': integer
            698..699 '3': integer
            701..702 '4': integer
            705..722 'vec4h(... 7, 8)': vec4<f16>
            711..712 '5': integer
            714..715 '6': integer
            717..718 '7': integer
            720..721 '8': integer
            733..751 '__f32_..._vec4f': mat2x4<f32>
            754..803 'mat2x4...7, 8))': mat2x4<f32>
            766..783 'vec4f(... 3, 4)': vec4<f32>
            772..773 '1': integer
            775..776 '2': integer
            778..779 '3': integer
            781..782 '4': integer
            785..802 'vec4f(... 7, 8)': vec4<f32>
            791..792 '5': integer
            794..795 '6': integer
            797..798 '7': integer
            800..801 '8': integer
            814..827 '__f16__mat2x4': mat2x4<f16>
            830..874 'mat2x4...s_abs)': mat2x4<f16>
            842..873 'abs_ab...bs_abs': mat2x4<f32>
            884..898 '__f16__mat2x4h': mat2x4<f16>
            901..945 'mat2x4...6_f16)': mat2x4<f16>
            913..944 'f16_f1...16_f16': mat2x4<f16>
            955..969 '__f16__mat2x4f': mat2x4<f16>
            972..1016 'mat2x4...2_f32)': mat2x4<f16>
            984..1015 'f32_f3...32_f32': mat2x4<f32>
            1026..1039 '__f32__mat2x4': mat2x4<f32>
            1042..1086 'mat2x4...s_abs)': mat2x4<f32>
            1054..1085 'abs_ab...bs_abs': mat2x4<f32>
            1096..1110 '__f32__mat2x4h': mat2x4<f32>
            1113..1157 'mat2x4...6_f16)': mat2x4<f32>
            1125..1156 'f16_f1...16_f16': mat2x4<f16>
            1167..1181 '__f32__mat2x4f': mat2x4<f32>
            1184..1228 'mat2x4...2_f32)': mat2x4<f32>
            1196..1227 'f32_f3...32_f32': mat2x4<f32>
            1238..1253 'mat2x4_identity': mat2x4<f32>
            1256..1295 'mat2x4...s_abs)': mat2x4<f32>
            1263..1294 'abs_ab...bs_abs': mat2x4<f32>
            1305..1321 'mat2x4...entity': mat2x4<f16>
            1324..1363 'mat2x4...6_f16)': mat2x4<f16>
            1331..1362 'f16_f1...16_f16': mat2x4<f16>
            1373..1389 'mat2x4...entity': mat2x4<f32>
            1392..1431 'mat2x4...2_f32)': mat2x4<f32>
            1399..1430 'f32_f3...32_f32': mat2x4<f32>
        "#]],
    );
}

#[test]
fn mat3x2() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let abs_abs_abs_abs_abs_abs = mat3x2(1, 2, 3, 4, 5, 6);
    let f16_f16_f16_f16_f16_f16 = mat3x2(1h, 2h, 3h, 4h, 5h, 6h);
    let f32_f32_f32_f32_f32_f32 = mat3x2(1f, 2f, 3f, 4f, 5f, 6f);
    let __f16__f16_f16_f16_f16_f16_f16 = mat3x2<f16>(1h, 2h, 3h, 4h, 5h, 6h);
    let __f32__f32_f32_f32_f32_f32_f32 = mat3x2<f32>(1f, 2f, 3f, 4f, 5f, 6f);

    let vec2_vec2_vec2 = mat3x2(vec2(1, 2), vec2(3, 4), vec2(5, 6));
    let vec2h_vec2h_vec2h = mat3x2(vec2h(1h, 2h), vec2h(3, 4), vec2h(5, 6));
    let vec2f_vec2f_vec2f = mat3x2(vec2(1f, 2f), vec2f(3, 4), vec2f(5, 6));
    let __f16__vec2h_vec2h_vec2h = mat3x2<f16>(vec2h(1, 2), vec2h(3, 4), vec2h(5, 6));
    let __f32__vec2f_vec2f_vec2f = mat3x2<f32>(vec2f(1, 2), vec2f(3, 4), vec2f(5, 6));

    let __f16__mat3x2 = mat3x2<f16>(abs_abs_abs_abs_abs_abs);
    let __f16__mat3x2h = mat3x2<f16>(f16_f16_f16_f16_f16_f16);
    let __f16__mat3x2f = mat3x2<f16>(f32_f32_f32_f32_f32_f32);
    let __f32__mat3x2 = mat3x2<f32>(abs_abs_abs_abs_abs_abs);
    let __f32__mat3x2h = mat3x2<f32>(f16_f16_f16_f16_f16_f16);
    let __f32__mat3x2f = mat3x2<f32>(f32_f32_f32_f32_f32_f32);
    let mat3x2_identity = mat3x2(abs_abs_abs_abs_abs_abs);
    let mat3x2h_identity = mat3x2(f16_f16_f16_f16_f16_f16);
    let mat3x2f_identity = mat3x2(f32_f32_f32_f32_f32_f32);
}
",
        expect![[r#"
            19..42 'abs_ab...bs_abs': mat3x2<f32>
            45..69 'mat3x2... 5, 6)': mat3x2<float>
            52..53 '1': integer
            55..56 '2': integer
            58..59 '3': integer
            61..62 '4': integer
            64..65 '5': integer
            67..68 '6': integer
            79..102 'f16_f1...16_f16': mat3x2<f16>
            105..135 'mat3x2...h, 6h)': mat3x2<f16>
            112..114 '1h': f16
            116..118 '2h': f16
            120..122 '3h': f16
            124..126 '4h': f16
            128..130 '5h': f16
            132..134 '6h': f16
            145..168 'f32_f3...32_f32': mat3x2<f32>
            171..201 'mat3x2...f, 6f)': mat3x2<f32>
            178..180 '1f': f32
            182..184 '2f': f32
            186..188 '3f': f32
            190..192 '4f': f32
            194..196 '5f': f32
            198..200 '6f': f32
            211..241 '__f16_...16_f16': mat3x2<f16>
            244..279 'mat3x2...h, 6h)': mat3x2<f16>
            256..258 '1h': f16
            260..262 '2h': f16
            264..266 '3h': f16
            268..270 '4h': f16
            272..274 '5h': f16
            276..278 '6h': f16
            289..319 '__f32_...32_f32': mat3x2<f32>
            322..357 'mat3x2...f, 6f)': mat3x2<f32>
            334..336 '1f': f32
            338..340 '2f': f32
            342..344 '3f': f32
            346..348 '4f': f32
            350..352 '5f': f32
            354..356 '6f': f32
            368..382 'vec2_vec2_vec2': mat3x2<f32>
            385..427 'mat3x2...5, 6))': mat3x2<float>
            392..402 'vec2(1, 2)': vec2<integer>
            397..398 '1': integer
            400..401 '2': integer
            404..414 'vec2(3, 4)': vec2<integer>
            409..410 '3': integer
            412..413 '4': integer
            416..426 'vec2(5, 6)': vec2<integer>
            421..422 '5': integer
            424..425 '6': integer
            437..454 'vec2h_..._vec2h': mat3x2<f16>
            457..504 'mat3x2...5, 6))': mat3x2<f16>
            464..477 'vec2h(1h, 2h)': vec2<f16>
            470..472 '1h': f16
            474..476 '2h': f16
            479..490 'vec2h(3, 4)': vec2<f16>
            485..486 '3': integer
            488..489 '4': integer
            492..503 'vec2h(5, 6)': vec2<f16>
            498..499 '5': integer
            501..502 '6': integer
            514..531 'vec2f_..._vec2f': mat3x2<f32>
            534..580 'mat3x2...5, 6))': mat3x2<f32>
            541..553 'vec2(1f, 2f)': vec2<f32>
            546..548 '1f': f32
            550..552 '2f': f32
            555..566 'vec2f(3, 4)': vec2<f32>
            561..562 '3': integer
            564..565 '4': integer
            568..579 'vec2f(5, 6)': vec2<f32>
            574..575 '5': integer
            577..578 '6': integer
            590..614 '__f16_..._vec2h': mat3x2<f16>
            617..667 'mat3x2...5, 6))': mat3x2<f16>
            629..640 'vec2h(1, 2)': vec2<f16>
            635..636 '1': integer
            638..639 '2': integer
            642..653 'vec2h(3, 4)': vec2<f16>
            648..649 '3': integer
            651..652 '4': integer
            655..666 'vec2h(5, 6)': vec2<f16>
            661..662 '5': integer
            664..665 '6': integer
            677..701 '__f32_..._vec2f': mat3x2<f32>
            704..754 'mat3x2...5, 6))': mat3x2<f32>
            716..727 'vec2f(1, 2)': vec2<f32>
            722..723 '1': integer
            725..726 '2': integer
            729..740 'vec2f(3, 4)': vec2<f32>
            735..736 '3': integer
            738..739 '4': integer
            742..753 'vec2f(5, 6)': vec2<f32>
            748..749 '5': integer
            751..752 '6': integer
            765..778 '__f16__mat3x2': mat3x2<f16>
            781..817 'mat3x2...s_abs)': mat3x2<f16>
            793..816 'abs_ab...bs_abs': mat3x2<f32>
            827..841 '__f16__mat3x2h': mat3x2<f16>
            844..880 'mat3x2...6_f16)': mat3x2<f16>
            856..879 'f16_f1...16_f16': mat3x2<f16>
            890..904 '__f16__mat3x2f': mat3x2<f16>
            907..943 'mat3x2...2_f32)': mat3x2<f16>
            919..942 'f32_f3...32_f32': mat3x2<f32>
            953..966 '__f32__mat3x2': mat3x2<f32>
            969..1005 'mat3x2...s_abs)': mat3x2<f32>
            981..1004 'abs_ab...bs_abs': mat3x2<f32>
            1015..1029 '__f32__mat3x2h': mat3x2<f32>
            1032..1068 'mat3x2...6_f16)': mat3x2<f32>
            1044..1067 'f16_f1...16_f16': mat3x2<f16>
            1078..1092 '__f32__mat3x2f': mat3x2<f32>
            1095..1131 'mat3x2...2_f32)': mat3x2<f32>
            1107..1130 'f32_f3...32_f32': mat3x2<f32>
            1141..1156 'mat3x2_identity': mat3x2<f32>
            1159..1190 'mat3x2...s_abs)': mat3x2<f32>
            1166..1189 'abs_ab...bs_abs': mat3x2<f32>
            1200..1216 'mat3x2...entity': mat3x2<f16>
            1219..1250 'mat3x2...6_f16)': mat3x2<f16>
            1226..1249 'f16_f1...16_f16': mat3x2<f16>
            1260..1276 'mat3x2...entity': mat3x2<f32>
            1279..1310 'mat3x2...2_f32)': mat3x2<f32>
            1286..1309 'f32_f3...32_f32': mat3x2<f32>
        "#]],
    );
}

#[test]
fn mat3x3() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let abs_abs_abs_abs_abs_abs_abs_abs_abs = mat3x3(1, 2, 3, 4, 5, 6, 7, 8, 9);
    let f16_f16_f16_f16_f16_f16_f16_f16_f16 = mat3x3(1h, 2h, 3h, 4h, 5h, 6h, 7h, 8h, 9h);
    let f32_f32_f32_f32_f32_f32_f32_f32_f32 = mat3x3(1f, 2f, 3f, 4f, 5f, 6f, 7f, 8f, 9f);
    let __f16__f16_f16_f16_f16_f16_f16_f16_f16_f16 = mat3x3<f16>(1h, 2h, 3h, 4h, 5h, 6h, 7h, 8h, 9h);
    let __f32__f32_f32_f32_f32_f32_f32_f32_f32_f32 = mat3x3<f32>(1f, 2f, 3f, 4f, 5f, 6f, 7f, 8f, 9f);

    let vec3_vec3_vec3 = mat3x3(vec3(1, 2, 3), vec3(4, 5, 6), vec3(7, 8, 9));
    let vec3h_vec3h_vec3h = mat3x3(vec3h(1h, 2h, 3h), vec3h(4, 5, 6), vec3h(7, 8, 9));
    let vec3f_vec3f_vec3f = mat3x3(vec3(1f, 2f, 3f), vec3f(4, 5, 6), vec3f(7, 8, 9));
    let __f16__vec3h_vec3h_vec3h = mat3x3<f16>(vec3h(1, 2, 3), vec3h(4, 5, 6), vec3h(7, 8, 9));
    let __f32__vec3f_vec3f_vec3f = mat3x3<f32>(vec3f(1, 2, 3), vec3f(4, 5, 6), vec3f(7, 8, 9));

    let __f16__mat3x3 = mat3x3<f16>(abs_abs_abs_abs_abs_abs_abs_abs_abs);
    let __f16__mat3x3h = mat3x3<f16>(f16_f16_f16_f16_f16_f16_f16_f16_f16);
    let __f16__mat3x3f = mat3x3<f16>(f32_f32_f32_f32_f32_f32_f32_f32_f32);
    let __f32__mat3x3 = mat3x3<f32>(abs_abs_abs_abs_abs_abs_abs_abs_abs);
    let __f32__mat3x3h = mat3x3<f32>(f16_f16_f16_f16_f16_f16_f16_f16_f16);
    let __f32__mat3x3f = mat3x3<f32>(f32_f32_f32_f32_f32_f32_f32_f32_f32);
    let mat3x3_identity = mat3x3(abs_abs_abs_abs_abs_abs_abs_abs_abs);
    let mat3x3h_identity = mat3x3(f16_f16_f16_f16_f16_f16_f16_f16_f16);
    let mat3x3f_identity = mat3x3(f32_f32_f32_f32_f32_f32_f32_f32_f32);
}
",
        expect![[r#"
            19..54 'abs_ab...bs_abs': mat3x3<f32>
            57..90 'mat3x3... 8, 9)': mat3x3<float>
            64..65 '1': integer
            67..68 '2': integer
            70..71 '3': integer
            73..74 '4': integer
            76..77 '5': integer
            79..80 '6': integer
            82..83 '7': integer
            85..86 '8': integer
            88..89 '9': integer
            100..135 'f16_f1...16_f16': mat3x3<f16>
            138..180 'mat3x3...h, 9h)': mat3x3<f16>
            145..147 '1h': f16
            149..151 '2h': f16
            153..155 '3h': f16
            157..159 '4h': f16
            161..163 '5h': f16
            165..167 '6h': f16
            169..171 '7h': f16
            173..175 '8h': f16
            177..179 '9h': f16
            190..225 'f32_f3...32_f32': mat3x3<f32>
            228..270 'mat3x3...f, 9f)': mat3x3<f32>
            235..237 '1f': f32
            239..241 '2f': f32
            243..245 '3f': f32
            247..249 '4f': f32
            251..253 '5f': f32
            255..257 '6f': f32
            259..261 '7f': f32
            263..265 '8f': f32
            267..269 '9f': f32
            280..322 '__f16_...16_f16': mat3x3<f16>
            325..372 'mat3x3...h, 9h)': mat3x3<f16>
            337..339 '1h': f16
            341..343 '2h': f16
            345..347 '3h': f16
            349..351 '4h': f16
            353..355 '5h': f16
            357..359 '6h': f16
            361..363 '7h': f16
            365..367 '8h': f16
            369..371 '9h': f16
            382..424 '__f32_...32_f32': mat3x3<f32>
            427..474 'mat3x3...f, 9f)': mat3x3<f32>
            439..441 '1f': f32
            443..445 '2f': f32
            447..449 '3f': f32
            451..453 '4f': f32
            455..457 '5f': f32
            459..461 '6f': f32
            463..465 '7f': f32
            467..469 '8f': f32
            471..473 '9f': f32
            485..499 'vec3_vec3_vec3': mat3x3<f32>
            502..553 'mat3x3...8, 9))': mat3x3<float>
            509..522 'vec3(1, 2, 3)': vec3<integer>
            514..515 '1': integer
            517..518 '2': integer
            520..521 '3': integer
            524..537 'vec3(4, 5, 6)': vec3<integer>
            529..530 '4': integer
            532..533 '5': integer
            535..536 '6': integer
            539..552 'vec3(7, 8, 9)': vec3<integer>
            544..545 '7': integer
            547..548 '8': integer
            550..551 '9': integer
            563..580 'vec3h_..._vec3h': mat3x3<f16>
            583..640 'mat3x3...8, 9))': mat3x3<f16>
            590..607 'vec3h(...h, 3h)': vec3<f16>
            596..598 '1h': f16
            600..602 '2h': f16
            604..606 '3h': f16
            609..623 'vec3h(4, 5, 6)': vec3<f16>
            615..616 '4': integer
            618..619 '5': integer
            621..622 '6': integer
            625..639 'vec3h(7, 8, 9)': vec3<f16>
            631..632 '7': integer
            634..635 '8': integer
            637..638 '9': integer
            650..667 'vec3f_..._vec3f': mat3x3<f32>
            670..726 'mat3x3...8, 9))': mat3x3<f32>
            677..693 'vec3(1...f, 3f)': vec3<f32>
            682..684 '1f': f32
            686..688 '2f': f32
            690..692 '3f': f32
            695..709 'vec3f(4, 5, 6)': vec3<f32>
            701..702 '4': integer
            704..705 '5': integer
            707..708 '6': integer
            711..725 'vec3f(7, 8, 9)': vec3<f32>
            717..718 '7': integer
            720..721 '8': integer
            723..724 '9': integer
            736..760 '__f16_..._vec3h': mat3x3<f16>
            763..822 'mat3x3...8, 9))': mat3x3<f16>
            775..789 'vec3h(1, 2, 3)': vec3<f16>
            781..782 '1': integer
            784..785 '2': integer
            787..788 '3': integer
            791..805 'vec3h(4, 5, 6)': vec3<f16>
            797..798 '4': integer
            800..801 '5': integer
            803..804 '6': integer
            807..821 'vec3h(7, 8, 9)': vec3<f16>
            813..814 '7': integer
            816..817 '8': integer
            819..820 '9': integer
            832..856 '__f32_..._vec3f': mat3x3<f32>
            859..918 'mat3x3...8, 9))': mat3x3<f32>
            871..885 'vec3f(1, 2, 3)': vec3<f32>
            877..878 '1': integer
            880..881 '2': integer
            883..884 '3': integer
            887..901 'vec3f(4, 5, 6)': vec3<f32>
            893..894 '4': integer
            896..897 '5': integer
            899..900 '6': integer
            903..917 'vec3f(7, 8, 9)': vec3<f32>
            909..910 '7': integer
            912..913 '8': integer
            915..916 '9': integer
            929..942 '__f16__mat3x3': mat3x3<f16>
            945..993 'mat3x3...s_abs)': mat3x3<f16>
            957..992 'abs_ab...bs_abs': mat3x3<f32>
            1003..1017 '__f16__mat3x3h': mat3x3<f16>
            1020..1068 'mat3x3...6_f16)': mat3x3<f16>
            1032..1067 'f16_f1...16_f16': mat3x3<f16>
            1078..1092 '__f16__mat3x3f': mat3x3<f16>
            1095..1143 'mat3x3...2_f32)': mat3x3<f16>
            1107..1142 'f32_f3...32_f32': mat3x3<f32>
            1153..1166 '__f32__mat3x3': mat3x3<f32>
            1169..1217 'mat3x3...s_abs)': mat3x3<f32>
            1181..1216 'abs_ab...bs_abs': mat3x3<f32>
            1227..1241 '__f32__mat3x3h': mat3x3<f32>
            1244..1292 'mat3x3...6_f16)': mat3x3<f32>
            1256..1291 'f16_f1...16_f16': mat3x3<f16>
            1302..1316 '__f32__mat3x3f': mat3x3<f32>
            1319..1367 'mat3x3...2_f32)': mat3x3<f32>
            1331..1366 'f32_f3...32_f32': mat3x3<f32>
            1377..1392 'mat3x3_identity': mat3x3<f32>
            1395..1438 'mat3x3...s_abs)': mat3x3<f32>
            1402..1437 'abs_ab...bs_abs': mat3x3<f32>
            1448..1464 'mat3x3...entity': mat3x3<f16>
            1467..1510 'mat3x3...6_f16)': mat3x3<f16>
            1474..1509 'f16_f1...16_f16': mat3x3<f16>
            1520..1536 'mat3x3...entity': mat3x3<f32>
            1539..1582 'mat3x3...2_f32)': mat3x3<f32>
            1546..1581 'f32_f3...32_f32': mat3x3<f32>
        "#]],
    );
}

#[test]
fn mat3x4() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs = mat3x4(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    let f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16 = mat3x4(1h, 2h, 3h, 4h, 5h, 6h, 7h, 8h, 9h, 10h, 11h, 12h);
    let f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32 = mat3x4(1f, 2f, 3f, 4f, 5f, 6f, 7f, 8f, 9f, 10f, 11f, 12f);
    let __f16__f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16 = mat3x4<f16>(1h, 2h, 3h, 4h, 5h, 6h, 7h, 8h, 9h, 10h, 11h, 12h);
    let __f32__f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32 = mat3x4<f32>(1f, 2f, 3f, 4f, 5f, 6f, 7f, 8f, 9f, 10f, 11f, 12f);

    let vec4_vec4_vec4 = mat3x4(vec4(1, 2, 3, 4), vec4(5, 6, 7, 8), vec4(9, 10, 11, 12));
    let vec4h_vec4h_vec4h = mat3x4(vec4h(1h, 2h, 3h, 4h), vec4h(5, 6, 7, 8), vec4h(9, 10, 11, 12));
    let vec4f_vec4f_vec4f = mat3x4(vec4(1f, 2f, 3f, 4f), vec4f(5, 6, 7, 8), vec4f(9, 10, 11, 12));
    let __f16__vec4h_vec4h_vec4h = mat3x4<f16>(vec4h(1, 2, 3, 4), vec4h(5, 6, 7, 8), vec4h(9, 10, 11, 12));
    let __f32__vec4f_vec4f_vec4f = mat3x4<f32>(vec4f(1, 2, 3, 4), vec4f(5, 6, 7, 8), vec4f(9, 10, 11, 12));

    let __f16__mat3x4 = mat3x4<f16>(abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs);
    let __f16__mat3x4h = mat3x4<f16>(f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16);
    let __f16__mat3x4f = mat3x4<f16>(f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32);
    let __f32__mat3x4 = mat3x4<f32>(abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs);
    let __f32__mat3x4h = mat3x4<f32>(f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16);
    let __f32__mat3x4f = mat3x4<f32>(f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32);
    let mat3x4_identity = mat3x4(abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs);
    let mat3x4h_identity = mat3x4(f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16);
    let mat3x4f_identity = mat3x4(f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32);
}
",
        expect![[r#"
            19..66 'abs_ab...bs_abs': mat3x4<f32>
            69..114 'mat3x4...1, 12)': mat3x4<float>
            76..77 '1': integer
            79..80 '2': integer
            82..83 '3': integer
            85..86 '4': integer
            88..89 '5': integer
            91..92 '6': integer
            94..95 '7': integer
            97..98 '8': integer
            100..101 '9': integer
            103..105 '10': integer
            107..109 '11': integer
            111..113 '12': integer
            124..171 'f16_f1...16_f16': mat3x4<f16>
            174..231 'mat3x4..., 12h)': mat3x4<f16>
            181..183 '1h': f16
            185..187 '2h': f16
            189..191 '3h': f16
            193..195 '4h': f16
            197..199 '5h': f16
            201..203 '6h': f16
            205..207 '7h': f16
            209..211 '8h': f16
            213..215 '9h': f16
            217..220 '10h': f16
            222..225 '11h': f16
            227..230 '12h': f16
            241..288 'f32_f3...32_f32': mat3x4<f32>
            291..348 'mat3x4..., 12f)': mat3x4<f32>
            298..300 '1f': f32
            302..304 '2f': f32
            306..308 '3f': f32
            310..312 '4f': f32
            314..316 '5f': f32
            318..320 '6f': f32
            322..324 '7f': f32
            326..328 '8f': f32
            330..332 '9f': f32
            334..337 '10f': f32
            339..342 '11f': f32
            344..347 '12f': f32
            358..412 '__f16_...16_f16': mat3x4<f16>
            415..477 'mat3x4..., 12h)': mat3x4<f16>
            427..429 '1h': f16
            431..433 '2h': f16
            435..437 '3h': f16
            439..441 '4h': f16
            443..445 '5h': f16
            447..449 '6h': f16
            451..453 '7h': f16
            455..457 '8h': f16
            459..461 '9h': f16
            463..466 '10h': f16
            468..471 '11h': f16
            473..476 '12h': f16
            487..541 '__f32_...32_f32': mat3x4<f32>
            544..606 'mat3x4..., 12f)': mat3x4<f32>
            556..558 '1f': f32
            560..562 '2f': f32
            564..566 '3f': f32
            568..570 '4f': f32
            572..574 '5f': f32
            576..578 '6f': f32
            580..582 '7f': f32
            584..586 '8f': f32
            588..590 '9f': f32
            592..595 '10f': f32
            597..600 '11f': f32
            602..605 '12f': f32
            617..631 'vec4_vec4_vec4': mat3x4<f32>
            634..697 'mat3x4..., 12))': mat3x4<float>
            641..657 'vec4(1... 3, 4)': vec4<integer>
            646..647 '1': integer
            649..650 '2': integer
            652..653 '3': integer
            655..656 '4': integer
            659..675 'vec4(5... 7, 8)': vec4<integer>
            664..665 '5': integer
            667..668 '6': integer
            670..671 '7': integer
            673..674 '8': integer
            677..696 'vec4(9...1, 12)': vec4<integer>
            682..683 '9': integer
            685..687 '10': integer
            689..691 '11': integer
            693..695 '12': integer
            707..724 'vec4h_..._vec4h': mat3x4<f16>
            727..797 'mat3x4..., 12))': mat3x4<f16>
            734..755 'vec4h(...h, 4h)': vec4<f16>
            740..742 '1h': f16
            744..746 '2h': f16
            748..750 '3h': f16
            752..754 '4h': f16
            757..774 'vec4h(... 7, 8)': vec4<f16>
            763..764 '5': integer
            766..767 '6': integer
            769..770 '7': integer
            772..773 '8': integer
            776..796 'vec4h(...1, 12)': vec4<f16>
            782..783 '9': integer
            785..787 '10': integer
            789..791 '11': integer
            793..795 '12': integer
            807..824 'vec4f_..._vec4f': mat3x4<f32>
            827..896 'mat3x4..., 12))': mat3x4<f32>
            834..854 'vec4(1...f, 4f)': vec4<f32>
            839..841 '1f': f32
            843..845 '2f': f32
            847..849 '3f': f32
            851..853 '4f': f32
            856..873 'vec4f(... 7, 8)': vec4<f32>
            862..863 '5': integer
            865..866 '6': integer
            868..869 '7': integer
            871..872 '8': integer
            875..895 'vec4f(...1, 12)': vec4<f32>
            881..882 '9': integer
            884..886 '10': integer
            888..890 '11': integer
            892..894 '12': integer
            906..930 '__f16_..._vec4h': mat3x4<f16>
            933..1004 'mat3x4..., 12))': mat3x4<f16>
            945..962 'vec4h(... 3, 4)': vec4<f16>
            951..952 '1': integer
            954..955 '2': integer
            957..958 '3': integer
            960..961 '4': integer
            964..981 'vec4h(... 7, 8)': vec4<f16>
            970..971 '5': integer
            973..974 '6': integer
            976..977 '7': integer
            979..980 '8': integer
            983..1003 'vec4h(...1, 12)': vec4<f16>
            989..990 '9': integer
            992..994 '10': integer
            996..998 '11': integer
            1000..1002 '12': integer
            1014..1038 '__f32_..._vec4f': mat3x4<f32>
            1041..1112 'mat3x4..., 12))': mat3x4<f32>
            1053..1070 'vec4f(... 3, 4)': vec4<f32>
            1059..1060 '1': integer
            1062..1063 '2': integer
            1065..1066 '3': integer
            1068..1069 '4': integer
            1072..1089 'vec4f(... 7, 8)': vec4<f32>
            1078..1079 '5': integer
            1081..1082 '6': integer
            1084..1085 '7': integer
            1087..1088 '8': integer
            1091..1111 'vec4f(...1, 12)': vec4<f32>
            1097..1098 '9': integer
            1100..1102 '10': integer
            1104..1106 '11': integer
            1108..1110 '12': integer
            1123..1136 '__f16__mat3x4': mat3x4<f16>
            1139..1199 'mat3x4...s_abs)': mat3x4<f16>
            1151..1198 'abs_ab...bs_abs': mat3x4<f32>
            1209..1223 '__f16__mat3x4h': mat3x4<f16>
            1226..1286 'mat3x4...6_f16)': mat3x4<f16>
            1238..1285 'f16_f1...16_f16': mat3x4<f16>
            1296..1310 '__f16__mat3x4f': mat3x4<f16>
            1313..1373 'mat3x4...2_f32)': mat3x4<f16>
            1325..1372 'f32_f3...32_f32': mat3x4<f32>
            1383..1396 '__f32__mat3x4': mat3x4<f32>
            1399..1459 'mat3x4...s_abs)': mat3x4<f32>
            1411..1458 'abs_ab...bs_abs': mat3x4<f32>
            1469..1483 '__f32__mat3x4h': mat3x4<f32>
            1486..1546 'mat3x4...6_f16)': mat3x4<f32>
            1498..1545 'f16_f1...16_f16': mat3x4<f16>
            1556..1570 '__f32__mat3x4f': mat3x4<f32>
            1573..1633 'mat3x4...2_f32)': mat3x4<f32>
            1585..1632 'f32_f3...32_f32': mat3x4<f32>
            1643..1658 'mat3x4_identity': mat3x4<f32>
            1661..1716 'mat3x4...s_abs)': mat3x4<f32>
            1668..1715 'abs_ab...bs_abs': mat3x4<f32>
            1726..1742 'mat3x4...entity': mat3x4<f16>
            1745..1800 'mat3x4...6_f16)': mat3x4<f16>
            1752..1799 'f16_f1...16_f16': mat3x4<f16>
            1810..1826 'mat3x4...entity': mat3x4<f32>
            1829..1884 'mat3x4...2_f32)': mat3x4<f32>
            1836..1883 'f32_f3...32_f32': mat3x4<f32>
        "#]],
    );
}

#[test]
fn mat4x2() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let abs_abs_abs_abs_abs_abs_abs_abs = mat4x2(1, 2, 3, 4, 5, 6, 7, 8);
    let f16_f16_f16_f16_f16_f16_f16_f16 = mat4x2(1h, 2h, 3h, 4h, 5h, 6h, 7h, 8h);
    let f32_f32_f32_f32_f32_f32_f32_f32 = mat4x2(1f, 2f, 3f, 4f, 5f, 6f, 7f, 8f);
    let __f16__f16_f16_f16_f16_f16_f16_f16_f16 = mat4x2<f16>(1h, 2h, 3h, 4h, 5h, 6h, 7h, 8h);
    let __f32__f32_f32_f32_f32_f32_f32_f32_f32 = mat4x2<f32>(1f, 2f, 3f, 4f, 5f, 6f, 7f, 8f);

    let vec2_vec2_vec2_vec2 = mat4x2(vec2(1, 2), vec2(3, 4), vec2(5, 6), vec2(7, 8));
    let vec2h_vec2h_vec2h_vec2h = mat4x2(vec2h(1h, 2h), vec2h(3, 4), vec2h(5, 6), vec2h(7, 8));
    let vec2f_vec2f_vec2f_vec2f = mat4x2(vec2(1f, 2f), vec2f(3, 4), vec2f(5, 6), vec2f(7, 8));
    let __f16__vec2h_vec2h_vec2h_vec2h = mat4x2<f16>(vec2h(1, 2), vec2h(3, 4), vec2h(5, 6), vec2h(7, 8));
    let __f32__vec2f_vec2f_vec2f_vec2f = mat4x2<f32>(vec2f(1, 2), vec2f(3, 4), vec2f(5, 6), vec2f(7, 8));

    let __f16__mat4x2 = mat4x2<f16>(abs_abs_abs_abs_abs_abs_abs_abs);
    let __f16__mat4x2h = mat4x2<f16>(f16_f16_f16_f16_f16_f16_f16_f16);
    let __f16__mat4x2f = mat4x2<f16>(f32_f32_f32_f32_f32_f32_f32_f32);
    let __f32__mat4x2 = mat4x2<f32>(abs_abs_abs_abs_abs_abs_abs_abs);
    let __f32__mat4x2h = mat4x2<f32>(f16_f16_f16_f16_f16_f16_f16_f16);
    let __f32__mat4x2f = mat4x2<f32>(f32_f32_f32_f32_f32_f32_f32_f32);
    let mat4x2_identity = mat4x2(abs_abs_abs_abs_abs_abs_abs_abs);
    let mat4x2h_identity = mat4x2(f16_f16_f16_f16_f16_f16_f16_f16);
    let mat4x2f_identity = mat4x2(f32_f32_f32_f32_f32_f32_f32_f32);
}
",
        expect![[r#"
            19..50 'abs_ab...bs_abs': mat4x2<f32>
            53..83 'mat4x2... 7, 8)': mat4x2<float>
            60..61 '1': integer
            63..64 '2': integer
            66..67 '3': integer
            69..70 '4': integer
            72..73 '5': integer
            75..76 '6': integer
            78..79 '7': integer
            81..82 '8': integer
            93..124 'f16_f1...16_f16': mat4x2<f16>
            127..165 'mat4x2...h, 8h)': mat4x2<f16>
            134..136 '1h': f16
            138..140 '2h': f16
            142..144 '3h': f16
            146..148 '4h': f16
            150..152 '5h': f16
            154..156 '6h': f16
            158..160 '7h': f16
            162..164 '8h': f16
            175..206 'f32_f3...32_f32': mat4x2<f32>
            209..247 'mat4x2...f, 8f)': mat4x2<f32>
            216..218 '1f': f32
            220..222 '2f': f32
            224..226 '3f': f32
            228..230 '4f': f32
            232..234 '5f': f32
            236..238 '6f': f32
            240..242 '7f': f32
            244..246 '8f': f32
            257..295 '__f16_...16_f16': mat4x2<f16>
            298..341 'mat4x2...h, 8h)': mat4x2<f16>
            310..312 '1h': f16
            314..316 '2h': f16
            318..320 '3h': f16
            322..324 '4h': f16
            326..328 '5h': f16
            330..332 '6h': f16
            334..336 '7h': f16
            338..340 '8h': f16
            351..389 '__f32_...32_f32': mat4x2<f32>
            392..435 'mat4x2...f, 8f)': mat4x2<f32>
            404..406 '1f': f32
            408..410 '2f': f32
            412..414 '3f': f32
            416..418 '4f': f32
            420..422 '5f': f32
            424..426 '6f': f32
            428..430 '7f': f32
            432..434 '8f': f32
            446..465 'vec2_v...2_vec2': mat4x2<f32>
            468..522 'mat4x2...7, 8))': mat4x2<float>
            475..485 'vec2(1, 2)': vec2<integer>
            480..481 '1': integer
            483..484 '2': integer
            487..497 'vec2(3, 4)': vec2<integer>
            492..493 '3': integer
            495..496 '4': integer
            499..509 'vec2(5, 6)': vec2<integer>
            504..505 '5': integer
            507..508 '6': integer
            511..521 'vec2(7, 8)': vec2<integer>
            516..517 '7': integer
            519..520 '8': integer
            532..555 'vec2h_..._vec2h': mat4x2<f16>
            558..618 'mat4x2...7, 8))': mat4x2<f16>
            565..578 'vec2h(1h, 2h)': vec2<f16>
            571..573 '1h': f16
            575..577 '2h': f16
            580..591 'vec2h(3, 4)': vec2<f16>
            586..587 '3': integer
            589..590 '4': integer
            593..604 'vec2h(5, 6)': vec2<f16>
            599..600 '5': integer
            602..603 '6': integer
            606..617 'vec2h(7, 8)': vec2<f16>
            612..613 '7': integer
            615..616 '8': integer
            628..651 'vec2f_..._vec2f': mat4x2<f32>
            654..713 'mat4x2...7, 8))': mat4x2<f32>
            661..673 'vec2(1f, 2f)': vec2<f32>
            666..668 '1f': f32
            670..672 '2f': f32
            675..686 'vec2f(3, 4)': vec2<f32>
            681..682 '3': integer
            684..685 '4': integer
            688..699 'vec2f(5, 6)': vec2<f32>
            694..695 '5': integer
            697..698 '6': integer
            701..712 'vec2f(7, 8)': vec2<f32>
            707..708 '7': integer
            710..711 '8': integer
            723..753 '__f16_..._vec2h': mat4x2<f16>
            756..819 'mat4x2...7, 8))': mat4x2<f16>
            768..779 'vec2h(1, 2)': vec2<f16>
            774..775 '1': integer
            777..778 '2': integer
            781..792 'vec2h(3, 4)': vec2<f16>
            787..788 '3': integer
            790..791 '4': integer
            794..805 'vec2h(5, 6)': vec2<f16>
            800..801 '5': integer
            803..804 '6': integer
            807..818 'vec2h(7, 8)': vec2<f16>
            813..814 '7': integer
            816..817 '8': integer
            829..859 '__f32_..._vec2f': mat4x2<f32>
            862..925 'mat4x2...7, 8))': mat4x2<f32>
            874..885 'vec2f(1, 2)': vec2<f32>
            880..881 '1': integer
            883..884 '2': integer
            887..898 'vec2f(3, 4)': vec2<f32>
            893..894 '3': integer
            896..897 '4': integer
            900..911 'vec2f(5, 6)': vec2<f32>
            906..907 '5': integer
            909..910 '6': integer
            913..924 'vec2f(7, 8)': vec2<f32>
            919..920 '7': integer
            922..923 '8': integer
            936..949 '__f16__mat4x2': mat4x2<f16>
            952..996 'mat4x2...s_abs)': mat4x2<f16>
            964..995 'abs_ab...bs_abs': mat4x2<f32>
            1006..1020 '__f16__mat4x2h': mat4x2<f16>
            1023..1067 'mat4x2...6_f16)': mat4x2<f16>
            1035..1066 'f16_f1...16_f16': mat4x2<f16>
            1077..1091 '__f16__mat4x2f': mat4x2<f16>
            1094..1138 'mat4x2...2_f32)': mat4x2<f16>
            1106..1137 'f32_f3...32_f32': mat4x2<f32>
            1148..1161 '__f32__mat4x2': mat4x2<f32>
            1164..1208 'mat4x2...s_abs)': mat4x2<f32>
            1176..1207 'abs_ab...bs_abs': mat4x2<f32>
            1218..1232 '__f32__mat4x2h': mat4x2<f32>
            1235..1279 'mat4x2...6_f16)': mat4x2<f32>
            1247..1278 'f16_f1...16_f16': mat4x2<f16>
            1289..1303 '__f32__mat4x2f': mat4x2<f32>
            1306..1350 'mat4x2...2_f32)': mat4x2<f32>
            1318..1349 'f32_f3...32_f32': mat4x2<f32>
            1360..1375 'mat4x2_identity': mat4x2<f32>
            1378..1417 'mat4x2...s_abs)': mat4x2<f32>
            1385..1416 'abs_ab...bs_abs': mat4x2<f32>
            1427..1443 'mat4x2...entity': mat4x2<f16>
            1446..1485 'mat4x2...6_f16)': mat4x2<f16>
            1453..1484 'f16_f1...16_f16': mat4x2<f16>
            1495..1511 'mat4x2...entity': mat4x2<f32>
            1514..1553 'mat4x2...2_f32)': mat4x2<f32>
            1521..1552 'f32_f3...32_f32': mat4x2<f32>
        "#]],
    );
}

#[test]
fn mat4x3() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs = mat4x3(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    let f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16 = mat4x3(1h, 2h, 3h, 4h, 5h, 6h, 7h, 8h, 9h, 10h, 11h, 12h);
    let f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32 = mat4x3(1f, 2f, 3f, 4f, 5f, 6f, 7f, 8f, 9f, 10f, 11f, 12f);
    let __f16__f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16 = mat4x3<f16>(1h, 2h, 3h, 4h, 5h, 6h, 7h, 8h, 9h, 10h, 11h, 12h);
    let __f32__f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32 = mat4x3<f32>(1f, 2f, 3f, 4f, 5f, 6f, 7f, 8f, 9f, 10f, 11f, 12f);

    let vec3_vec3_vec3_vec3 = mat4x3(vec3(1, 2, 3), vec3(4, 5, 6), vec3(7, 8, 9), vec3(10, 11, 12));
    let vec3h_vec3h_vec3h_vec3h = mat4x3(vec3h(1h, 2h, 3h), vec3h(4, 5, 6), vec3h(7, 8, 9), vec3h(10, 11, 12));
    let vec3f_vec3f_vec3f_vec3f = mat4x3(vec3(1f, 2f, 3f), vec3f(4, 5, 6), vec3f(7, 8, 9), vec3f(10, 11, 12));
    let __f16__vec3h_vec3h_vec3h_vec3h = mat4x3<f16>(vec3h(1, 2, 3), vec3h(4, 5, 6), vec3h(7, 8, 9), vec3h(10, 11, 12));
    let __f32__vec3f_vec3f_vec3f_vec3f = mat4x3<f32>(vec3f(1, 2, 3), vec3f(4, 5, 6), vec3f(7, 8, 9), vec3f(10, 11, 12));

    let __f16__mat4x3 = mat4x3<f16>(abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs);
    let __f16__mat4x3h = mat4x3<f16>(f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16);
    let __f16__mat4x3f = mat4x3<f16>(f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32);
    let __f32__mat4x3 = mat4x3<f32>(abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs);
    let __f32__mat4x3h = mat4x3<f32>(f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16);
    let __f32__mat4x3f = mat4x3<f32>(f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32);
    let mat4x3_identity = mat4x3(abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs);
    let mat4x3h_identity = mat4x3(f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16);
    let mat4x3f_identity = mat4x3(f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32);
}
",
        expect![[r#"
            19..66 'abs_ab...bs_abs': mat4x3<f32>
            69..114 'mat4x3...1, 12)': mat4x3<float>
            76..77 '1': integer
            79..80 '2': integer
            82..83 '3': integer
            85..86 '4': integer
            88..89 '5': integer
            91..92 '6': integer
            94..95 '7': integer
            97..98 '8': integer
            100..101 '9': integer
            103..105 '10': integer
            107..109 '11': integer
            111..113 '12': integer
            124..171 'f16_f1...16_f16': mat4x3<f16>
            174..231 'mat4x3..., 12h)': mat4x3<f16>
            181..183 '1h': f16
            185..187 '2h': f16
            189..191 '3h': f16
            193..195 '4h': f16
            197..199 '5h': f16
            201..203 '6h': f16
            205..207 '7h': f16
            209..211 '8h': f16
            213..215 '9h': f16
            217..220 '10h': f16
            222..225 '11h': f16
            227..230 '12h': f16
            241..288 'f32_f3...32_f32': mat4x3<f32>
            291..348 'mat4x3..., 12f)': mat4x3<f32>
            298..300 '1f': f32
            302..304 '2f': f32
            306..308 '3f': f32
            310..312 '4f': f32
            314..316 '5f': f32
            318..320 '6f': f32
            322..324 '7f': f32
            326..328 '8f': f32
            330..332 '9f': f32
            334..337 '10f': f32
            339..342 '11f': f32
            344..347 '12f': f32
            358..412 '__f16_...16_f16': mat4x3<f16>
            415..477 'mat4x3..., 12h)': mat4x3<f16>
            427..429 '1h': f16
            431..433 '2h': f16
            435..437 '3h': f16
            439..441 '4h': f16
            443..445 '5h': f16
            447..449 '6h': f16
            451..453 '7h': f16
            455..457 '8h': f16
            459..461 '9h': f16
            463..466 '10h': f16
            468..471 '11h': f16
            473..476 '12h': f16
            487..541 '__f32_...32_f32': mat4x3<f32>
            544..606 'mat4x3..., 12f)': mat4x3<f32>
            556..558 '1f': f32
            560..562 '2f': f32
            564..566 '3f': f32
            568..570 '4f': f32
            572..574 '5f': f32
            576..578 '6f': f32
            580..582 '7f': f32
            584..586 '8f': f32
            588..590 '9f': f32
            592..595 '10f': f32
            597..600 '11f': f32
            602..605 '12f': f32
            617..636 'vec3_v...3_vec3': mat4x3<f32>
            639..708 'mat4x3..., 12))': mat4x3<float>
            646..659 'vec3(1, 2, 3)': vec3<integer>
            651..652 '1': integer
            654..655 '2': integer
            657..658 '3': integer
            661..674 'vec3(4, 5, 6)': vec3<integer>
            666..667 '4': integer
            669..670 '5': integer
            672..673 '6': integer
            676..689 'vec3(7, 8, 9)': vec3<integer>
            681..682 '7': integer
            684..685 '8': integer
            687..688 '9': integer
            691..707 'vec3(1...1, 12)': vec3<integer>
            696..698 '10': integer
            700..702 '11': integer
            704..706 '12': integer
            718..741 'vec3h_..._vec3h': mat4x3<f16>
            744..820 'mat4x3..., 12))': mat4x3<f16>
            751..768 'vec3h(...h, 3h)': vec3<f16>
            757..759 '1h': f16
            761..763 '2h': f16
            765..767 '3h': f16
            770..784 'vec3h(4, 5, 6)': vec3<f16>
            776..777 '4': integer
            779..780 '5': integer
            782..783 '6': integer
            786..800 'vec3h(7, 8, 9)': vec3<f16>
            792..793 '7': integer
            795..796 '8': integer
            798..799 '9': integer
            802..819 'vec3h(...1, 12)': vec3<f16>
            808..810 '10': integer
            812..814 '11': integer
            816..818 '12': integer
            830..853 'vec3f_..._vec3f': mat4x3<f32>
            856..931 'mat4x3..., 12))': mat4x3<f32>
            863..879 'vec3(1...f, 3f)': vec3<f32>
            868..870 '1f': f32
            872..874 '2f': f32
            876..878 '3f': f32
            881..895 'vec3f(4, 5, 6)': vec3<f32>
            887..888 '4': integer
            890..891 '5': integer
            893..894 '6': integer
            897..911 'vec3f(7, 8, 9)': vec3<f32>
            903..904 '7': integer
            906..907 '8': integer
            909..910 '9': integer
            913..930 'vec3f(...1, 12)': vec3<f32>
            919..921 '10': integer
            923..925 '11': integer
            927..929 '12': integer
            941..971 '__f16_..._vec3h': mat4x3<f16>
            974..1052 'mat4x3..., 12))': mat4x3<f16>
            986..1000 'vec3h(1, 2, 3)': vec3<f16>
            992..993 '1': integer
            995..996 '2': integer
            998..999 '3': integer
            1002..1016 'vec3h(4, 5, 6)': vec3<f16>
            1008..1009 '4': integer
            1011..1012 '5': integer
            1014..1015 '6': integer
            1018..1032 'vec3h(7, 8, 9)': vec3<f16>
            1024..1025 '7': integer
            1027..1028 '8': integer
            1030..1031 '9': integer
            1034..1051 'vec3h(...1, 12)': vec3<f16>
            1040..1042 '10': integer
            1044..1046 '11': integer
            1048..1050 '12': integer
            1062..1092 '__f32_..._vec3f': mat4x3<f32>
            1095..1173 'mat4x3..., 12))': mat4x3<f32>
            1107..1121 'vec3f(1, 2, 3)': vec3<f32>
            1113..1114 '1': integer
            1116..1117 '2': integer
            1119..1120 '3': integer
            1123..1137 'vec3f(4, 5, 6)': vec3<f32>
            1129..1130 '4': integer
            1132..1133 '5': integer
            1135..1136 '6': integer
            1139..1153 'vec3f(7, 8, 9)': vec3<f32>
            1145..1146 '7': integer
            1148..1149 '8': integer
            1151..1152 '9': integer
            1155..1172 'vec3f(...1, 12)': vec3<f32>
            1161..1163 '10': integer
            1165..1167 '11': integer
            1169..1171 '12': integer
            1184..1197 '__f16__mat4x3': mat4x3<f16>
            1200..1260 'mat4x3...s_abs)': mat4x3<f16>
            1212..1259 'abs_ab...bs_abs': mat4x3<f32>
            1270..1284 '__f16__mat4x3h': mat4x3<f16>
            1287..1347 'mat4x3...6_f16)': mat4x3<f16>
            1299..1346 'f16_f1...16_f16': mat4x3<f16>
            1357..1371 '__f16__mat4x3f': mat4x3<f16>
            1374..1434 'mat4x3...2_f32)': mat4x3<f16>
            1386..1433 'f32_f3...32_f32': mat4x3<f32>
            1444..1457 '__f32__mat4x3': mat4x3<f32>
            1460..1520 'mat4x3...s_abs)': mat4x3<f32>
            1472..1519 'abs_ab...bs_abs': mat4x3<f32>
            1530..1544 '__f32__mat4x3h': mat4x3<f32>
            1547..1607 'mat4x3...6_f16)': mat4x3<f32>
            1559..1606 'f16_f1...16_f16': mat4x3<f16>
            1617..1631 '__f32__mat4x3f': mat4x3<f32>
            1634..1694 'mat4x3...2_f32)': mat4x3<f32>
            1646..1693 'f32_f3...32_f32': mat4x3<f32>
            1704..1719 'mat4x3_identity': mat4x3<f32>
            1722..1777 'mat4x3...s_abs)': mat4x3<f32>
            1729..1776 'abs_ab...bs_abs': mat4x3<f32>
            1787..1803 'mat4x3...entity': mat4x3<f16>
            1806..1861 'mat4x3...6_f16)': mat4x3<f16>
            1813..1860 'f16_f1...16_f16': mat4x3<f16>
            1871..1887 'mat4x3...entity': mat4x3<f32>
            1890..1945 'mat4x3...2_f32)': mat4x3<f32>
            1897..1944 'f32_f3...32_f32': mat4x3<f32>
        "#]],
    );
}

#[test]
fn mat4x4() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs = mat4x4(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
    let f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16 = mat4x4(1h, 2h, 3h, 4h, 5h, 6h, 7h, 8h, 9h, 10h, 11h, 12h, 13h, 14h, 15h, 16h);
    let f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32 = mat4x4(1f, 2f, 3f, 4f, 5f, 6f, 7f, 8f, 9f, 10f, 11f, 12f, 13f, 14f, 15f, 16f);
    let __f16__f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16 = mat4x4<f16>(1h, 2h, 3h, 4h, 5h, 6h, 7h, 8h, 9h, 10h, 11h, 12h, 13h, 14h, 15h, 16h);
    let __f32__f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32 = mat4x4<f32>(1f, 2f, 3f, 4f, 5f, 6f, 7f, 8f, 9f, 10f, 11f, 12f, 13f, 14f, 15f, 16f);

    let vec4_vec4_vec4_vec4 = mat4x4(vec4(1, 2, 3, 4), vec4(5, 6, 7, 8), vec4(9, 10, 11, 12), vec4(13, 14, 15, 16));
    let vec4h_vec4h_vec4h_vec4h = mat4x4(vec4h(1h, 2h, 3h, 4h), vec4h(5, 6, 7, 8), vec4h(9, 10, 11, 12), vec4h(13, 14, 15, 16));
    let vec4f_vec4f_vec4f_vec4f = mat4x4(vec4(1f, 2f, 3f, 4f), vec4f(5, 6, 7, 8), vec4f(9, 10, 11, 12), vec4f(13, 14, 15, 16));
    let __f16__vec4h_vec4h_vec4h_vec4h = mat4x4<f16>(vec4h(1, 2, 3, 4), vec4h(5, 6, 7, 8), vec4h(9, 10, 11, 12), vec4h(13, 14, 15, 16));
    let __f32__vec4f_vec4f_vec4f_vec4f = mat4x4<f32>(vec4f(1, 2, 3, 4), vec4f(5, 6, 7, 8), vec4f(9, 10, 11, 12), vec4f(13, 14, 15, 16));

    let __f16__mat4x4 = mat4x4<f16>(abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs);
    let __f16__mat4x4h = mat4x4<f16>(f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16);
    let __f16__mat4x4f = mat4x4<f16>(f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32);
    let __f32__mat4x4 = mat4x4<f32>(abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs);
    let __f32__mat4x4h = mat4x4<f32>(f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16);
    let __f32__mat4x4f = mat4x4<f32>(f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32);
    let mat4x4_identity = mat4x4(abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs_abs);
    let mat4x4h_identity = mat4x4(f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16_f16);
    let mat4x4f_identity = mat4x4(f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32);
}
",
        expect![[r#"
            19..82 'abs_ab...bs_abs': mat4x4<f32>
            85..146 'mat4x4...5, 16)': mat4x4<float>
            92..93 '1': integer
            95..96 '2': integer
            98..99 '3': integer
            101..102 '4': integer
            104..105 '5': integer
            107..108 '6': integer
            110..111 '7': integer
            113..114 '8': integer
            116..117 '9': integer
            119..121 '10': integer
            123..125 '11': integer
            127..129 '12': integer
            131..133 '13': integer
            135..137 '14': integer
            139..141 '15': integer
            143..145 '16': integer
            156..219 'f16_f1...16_f16': mat4x4<f16>
            222..299 'mat4x4..., 16h)': mat4x4<f16>
            229..231 '1h': f16
            233..235 '2h': f16
            237..239 '3h': f16
            241..243 '4h': f16
            245..247 '5h': f16
            249..251 '6h': f16
            253..255 '7h': f16
            257..259 '8h': f16
            261..263 '9h': f16
            265..268 '10h': f16
            270..273 '11h': f16
            275..278 '12h': f16
            280..283 '13h': f16
            285..288 '14h': f16
            290..293 '15h': f16
            295..298 '16h': f16
            309..372 'f32_f3...32_f32': mat4x4<f32>
            375..452 'mat4x4..., 16f)': mat4x4<f32>
            382..384 '1f': f32
            386..388 '2f': f32
            390..392 '3f': f32
            394..396 '4f': f32
            398..400 '5f': f32
            402..404 '6f': f32
            406..408 '7f': f32
            410..412 '8f': f32
            414..416 '9f': f32
            418..421 '10f': f32
            423..426 '11f': f32
            428..431 '12f': f32
            433..436 '13f': f32
            438..441 '14f': f32
            443..446 '15f': f32
            448..451 '16f': f32
            462..532 '__f16_...16_f16': mat4x4<f16>
            535..617 'mat4x4..., 16h)': mat4x4<f16>
            547..549 '1h': f16
            551..553 '2h': f16
            555..557 '3h': f16
            559..561 '4h': f16
            563..565 '5h': f16
            567..569 '6h': f16
            571..573 '7h': f16
            575..577 '8h': f16
            579..581 '9h': f16
            583..586 '10h': f16
            588..591 '11h': f16
            593..596 '12h': f16
            598..601 '13h': f16
            603..606 '14h': f16
            608..611 '15h': f16
            613..616 '16h': f16
            627..697 '__f32_...32_f32': mat4x4<f32>
            700..782 'mat4x4..., 16f)': mat4x4<f32>
            712..714 '1f': f32
            716..718 '2f': f32
            720..722 '3f': f32
            724..726 '4f': f32
            728..730 '5f': f32
            732..734 '6f': f32
            736..738 '7f': f32
            740..742 '8f': f32
            744..746 '9f': f32
            748..751 '10f': f32
            753..756 '11f': f32
            758..761 '12f': f32
            763..766 '13f': f32
            768..771 '14f': f32
            773..776 '15f': f32
            778..781 '16f': f32
            793..812 'vec4_v...4_vec4': mat4x4<f32>
            815..900 'mat4x4..., 16))': mat4x4<float>
            822..838 'vec4(1... 3, 4)': vec4<integer>
            827..828 '1': integer
            830..831 '2': integer
            833..834 '3': integer
            836..837 '4': integer
            840..856 'vec4(5... 7, 8)': vec4<integer>
            845..846 '5': integer
            848..849 '6': integer
            851..852 '7': integer
            854..855 '8': integer
            858..877 'vec4(9...1, 12)': vec4<integer>
            863..864 '9': integer
            866..868 '10': integer
            870..872 '11': integer
            874..876 '12': integer
            879..899 'vec4(1...5, 16)': vec4<integer>
            884..886 '13': integer
            888..890 '14': integer
            892..894 '15': integer
            896..898 '16': integer
            910..933 'vec4h_..._vec4h': mat4x4<f16>
            936..1029 'mat4x4..., 16))': mat4x4<f16>
            943..964 'vec4h(...h, 4h)': vec4<f16>
            949..951 '1h': f16
            953..955 '2h': f16
            957..959 '3h': f16
            961..963 '4h': f16
            966..983 'vec4h(... 7, 8)': vec4<f16>
            972..973 '5': integer
            975..976 '6': integer
            978..979 '7': integer
            981..982 '8': integer
            985..1005 'vec4h(...1, 12)': vec4<f16>
            991..992 '9': integer
            994..996 '10': integer
            998..1000 '11': integer
            1002..1004 '12': integer
            1007..1028 'vec4h(...5, 16)': vec4<f16>
            1013..1015 '13': integer
            1017..1019 '14': integer
            1021..1023 '15': integer
            1025..1027 '16': integer
            1039..1062 'vec4f_..._vec4f': mat4x4<f32>
            1065..1157 'mat4x4..., 16))': mat4x4<f32>
            1072..1092 'vec4(1...f, 4f)': vec4<f32>
            1077..1079 '1f': f32
            1081..1083 '2f': f32
            1085..1087 '3f': f32
            1089..1091 '4f': f32
            1094..1111 'vec4f(... 7, 8)': vec4<f32>
            1100..1101 '5': integer
            1103..1104 '6': integer
            1106..1107 '7': integer
            1109..1110 '8': integer
            1113..1133 'vec4f(...1, 12)': vec4<f32>
            1119..1120 '9': integer
            1122..1124 '10': integer
            1126..1128 '11': integer
            1130..1132 '12': integer
            1135..1156 'vec4f(...5, 16)': vec4<f32>
            1141..1143 '13': integer
            1145..1147 '14': integer
            1149..1151 '15': integer
            1153..1155 '16': integer
            1167..1197 '__f16_..._vec4h': mat4x4<f16>
            1200..1294 'mat4x4..., 16))': mat4x4<f16>
            1212..1229 'vec4h(... 3, 4)': vec4<f16>
            1218..1219 '1': integer
            1221..1222 '2': integer
            1224..1225 '3': integer
            1227..1228 '4': integer
            1231..1248 'vec4h(... 7, 8)': vec4<f16>
            1237..1238 '5': integer
            1240..1241 '6': integer
            1243..1244 '7': integer
            1246..1247 '8': integer
            1250..1270 'vec4h(...1, 12)': vec4<f16>
            1256..1257 '9': integer
            1259..1261 '10': integer
            1263..1265 '11': integer
            1267..1269 '12': integer
            1272..1293 'vec4h(...5, 16)': vec4<f16>
            1278..1280 '13': integer
            1282..1284 '14': integer
            1286..1288 '15': integer
            1290..1292 '16': integer
            1304..1334 '__f32_..._vec4f': mat4x4<f32>
            1337..1431 'mat4x4..., 16))': mat4x4<f32>
            1349..1366 'vec4f(... 3, 4)': vec4<f32>
            1355..1356 '1': integer
            1358..1359 '2': integer
            1361..1362 '3': integer
            1364..1365 '4': integer
            1368..1385 'vec4f(... 7, 8)': vec4<f32>
            1374..1375 '5': integer
            1377..1378 '6': integer
            1380..1381 '7': integer
            1383..1384 '8': integer
            1387..1407 'vec4f(...1, 12)': vec4<f32>
            1393..1394 '9': integer
            1396..1398 '10': integer
            1400..1402 '11': integer
            1404..1406 '12': integer
            1409..1430 'vec4f(...5, 16)': vec4<f32>
            1415..1417 '13': integer
            1419..1421 '14': integer
            1423..1425 '15': integer
            1427..1429 '16': integer
            1442..1455 '__f16__mat4x4': mat4x4<f16>
            1458..1534 'mat4x4...s_abs)': mat4x4<f16>
            1470..1533 'abs_ab...bs_abs': mat4x4<f32>
            1544..1558 '__f16__mat4x4h': mat4x4<f16>
            1561..1637 'mat4x4...6_f16)': mat4x4<f16>
            1573..1636 'f16_f1...16_f16': mat4x4<f16>
            1647..1661 '__f16__mat4x4f': mat4x4<f16>
            1664..1740 'mat4x4...2_f32)': mat4x4<f16>
            1676..1739 'f32_f3...32_f32': mat4x4<f32>
            1750..1763 '__f32__mat4x4': mat4x4<f32>
            1766..1842 'mat4x4...s_abs)': mat4x4<f32>
            1778..1841 'abs_ab...bs_abs': mat4x4<f32>
            1852..1866 '__f32__mat4x4h': mat4x4<f32>
            1869..1945 'mat4x4...6_f16)': mat4x4<f32>
            1881..1944 'f16_f1...16_f16': mat4x4<f16>
            1955..1969 '__f32__mat4x4f': mat4x4<f32>
            1972..2048 'mat4x4...2_f32)': mat4x4<f32>
            1984..2047 'f32_f3...32_f32': mat4x4<f32>
            2058..2073 'mat4x4_identity': mat4x4<f32>
            2076..2147 'mat4x4...s_abs)': mat4x4<f32>
            2083..2146 'abs_ab...bs_abs': mat4x4<f32>
            2157..2173 'mat4x4...entity': mat4x4<f16>
            2176..2247 'mat4x4...6_f16)': mat4x4<f16>
            2183..2246 'f16_f1...16_f16': mat4x4<f16>
            2257..2273 'mat4x4...entity': mat4x4<f32>
            2276..2347 'mat4x4...2_f32)': mat4x4<f32>
            2283..2346 'f32_f3...32_f32': mat4x4<f32>
        "#]],
    );
}

#[test]
fn vec2() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let vec2_abstract_float = vec2(1.0);
    let vec2_abstract_integer = vec2(1);
}
",
        expect![[r#"
            19..38 'vec2_a..._float': vec2<f32>
            41..50 'vec2(1.0)': vec2<float>
            46..49 '1.0': float
            60..81 'vec2_a...nteger': vec2<i32>
            84..91 'vec2(1)': vec2<integer>
            89..90 '1': integer
        "#]],
    );
}

// op_vec2_constructor<bool>(e: bool) -> vec2<bool>
// op_vec2_constructor<i32>(e: i32) -> vec2<i32>
// op_vec2_constructor<u32>(e: u32) -> vec2<u32>
// op_vec2_constructor<f32>(e: f32) -> vec2<f32>
// op_vec2_constructor<f16>(e: f16) -> vec2<f16>
// op_vec2_constructor(e: bool) -> vec2<bool>
// op_vec2_constructor(e: AbstractFloat) -> vec2<AbstractFloat>
// op_vec2_constructor(e: AbstractInt) -> vec2<AbstractInt>
// op_vec2_constructor(e: i32) -> vec2<i32>
// op_vec2_constructor(e: u32) -> vec2<u32>
// op_vec2_constructor(e: f32) -> vec2<f32>
// op_vec2_constructor(e: f16) -> vec2<f16>

// op_vec2_constructor<bool>(e: vec2<bool>) -> vec2<bool>
// op_vec2_constructor<i32>(e: vec2<i32>) -> vec2<i32>
// op_vec2_constructor<u32>(e: vec2<u32>) -> vec2<u32>
// op_vec2_constructor<f32>(e: vec2<f32>) -> vec2<f32>
// op_vec2_constructor<f16>(e: vec2<f16>) -> vec2<f16>
// op_vec2_constructor(e: vec2<bool>) -> vec2<bool>
// op_vec2_constructor(e: vec2<AbstractFloat>) -> vec2<AbstractFloat>
// op_vec2_constructor(e: vec2<AbstractInt>) -> vec2<AbstractInt>
// op_vec2_constructor(e: vec2<i32>) -> vec2<i32>
// op_vec2_constructor(e: vec2<u32>) -> vec2<u32>
// op_vec2_constructor(e: vec2<f32>) -> vec2<f32>
// op_vec2_constructor(e: vec2<f16>) -> vec2<f16>

// op_vec2_constructor<bool>(e1: bool, e2: bool) -> vec2<bool>
// op_vec2_constructor<i32>(e1: i32, e2: i32) -> vec2<i32>
// op_vec2_constructor<u32>(e1: u32, e2: u32) -> vec2<u32>
// op_vec2_constructor<f32>(e1: f32, e2: f32) -> vec2<f32>
// op_vec2_constructor<f16>(e1: f16, e2: f16) -> vec2<f16>
// op_vec2_constructor(e1: bool, e2: bool) -> vec2<bool>
// op_vec2_constructor(e1: AbstractFloat, e2: AbstractFloat) -> vec2<AbstractFloat>
// op_vec2_constructor(e1: AbstractInt, e2: AbstractInt) -> vec2<AbstractInt>
// op_vec2_constructor(e1: i32, e2: i32) -> vec2<i32>
// op_vec2_constructor(e1: u32, e2: u32) -> vec2<u32>
// op_vec2_constructor(e1: f32, e2: f32) -> vec2<f32>
// op_vec2_constructor(e1: f16, e2: f16) -> vec2<f16>

// // parameterless is handled in Rust code

// op_vec3_constructor<bool>(e: bool) -> vec3<bool>
// op_vec3_constructor<i32>(e: i32) -> vec3<i32>
// op_vec3_constructor<u32>(e: u32) -> vec3<u32>
// op_vec3_constructor<f32>(e: f32) -> vec3<f32>
// op_vec3_constructor<f16>(e: f16) -> vec3<f16>
// op_vec3_constructor(e: bool) -> vec3<bool>
// op_vec3_constructor(e: AbstractFloat) -> vec3<AbstractFloat>
// op_vec3_constructor(e: AbstractInt) -> vec3<AbstractInt>
// op_vec3_constructor(e: i32) -> vec3<i32>
// op_vec3_constructor(e: u32) -> vec3<u32>
// op_vec3_constructor(e: f32) -> vec3<f32>
// op_vec3_constructor(e: f16) -> vec3<f16>

// op_vec3_constructor<bool>(e: vec3<bool>) -> vec3<bool>
// op_vec3_constructor<i32>(e: vec3<i32>) -> vec3<i32>
// op_vec3_constructor<u32>(e: vec3<u32>) -> vec3<u32>
// op_vec3_constructor<f32>(e: vec3<f32>) -> vec3<f32>
// op_vec3_constructor<f16>(e: vec3<f16>) -> vec3<f16>
// op_vec3_constructor(e: vec3<bool>) -> vec3<bool>
// op_vec3_constructor(e: vec3<AbstractFloat>) -> vec3<AbstractFloat>
// op_vec3_constructor(e: vec3<AbstractInt>) -> vec3<AbstractInt>
// op_vec3_constructor(e: vec3<i32>) -> vec3<i32>
// op_vec3_constructor(e: vec3<u32>) -> vec3<u32>
// op_vec3_constructor(e: vec3<f32>) -> vec3<f32>
// op_vec3_constructor(e: vec3<f16>) -> vec3<f16>

// op_vec3_constructor<bool>(e1: bool, e2: bool, e3: bool) -> vec3<bool>
// op_vec3_constructor<i32>(e1: i32, e2: i32, e3: i32) -> vec3<i32>
// op_vec3_constructor<u32>(e1: u32, e2: u32, e3: u32) -> vec3<u32>
// op_vec3_constructor<f32>(e1: f32, e2: f32, e3: f32) -> vec3<f32>
// op_vec3_constructor<f16>(e1: f16, e2: f16, e3: f16) -> vec3<f16>
// op_vec3_constructor(e1: bool, e2: bool, e3: bool) -> vec3<bool>
// op_vec3_constructor(e1: AbstractFloat, e2: AbstractFloat, e3: AbstractFloat) -> vec3<AbstractFloat>
// op_vec3_constructor(e1: AbstractInt, e2: AbstractInt, e3: AbstractInt) -> vec3<AbstractInt>
// op_vec3_constructor(e1: i32, e2: i32, e3: i32) -> vec3<i32>
// op_vec3_constructor(e1: u32, e2: u32, e3: u32) -> vec3<u32>
// op_vec3_constructor(e1: f32, e2: f32, e3: f32) -> vec3<f32>
// op_vec3_constructor(e1: f16, e2: f16, e3: f16) -> vec3<f16>

// op_vec3_constructor<bool>(v1: vec2<bool>, e1: bool) -> vec3<bool>
// op_vec3_constructor<i32>(v1: vec2<i32>, e1: i32) -> vec3<i32>
// op_vec3_constructor<u32>(v1: vec2<u32>, e1: u32) -> vec3<u32>
// op_vec3_constructor<f32>(v1: vec2<f32>, e1: f32) -> vec3<f32>
// op_vec3_constructor<f16>(v1: vec2<f16>, e1: f16) -> vec3<f16>
// op_vec3_constructor(v1: vec2<bool>, e1: bool) -> vec3<bool>
// op_vec3_constructor(v1: vec2<AbstractFloat>, e1: AbstractFloat) -> vec3<AbstractFloat>
// op_vec3_constructor(v1: vec2<AbstractInt>, e1: AbstractInt) -> vec3<AbstractInt>
// op_vec3_constructor(v1: vec2<i32>, e1: i32) -> vec3<i32>
// op_vec3_constructor(v1: vec2<u32>, e1: u32) -> vec3<u32>
// op_vec3_constructor(v1: vec2<f32>, e1: f32) -> vec3<f32>
// op_vec3_constructor(v1: vec2<f16>, e1: f16) -> vec3<f16>

// op_vec3_constructor<bool>(e1: bool, v1: vec2<bool>) -> vec3<bool>
// op_vec3_constructor<i32>(e1: i32, v1: vec2<i32>) -> vec3<i32>
// op_vec3_constructor<u32>(e1: u32, v1: vec2<u32>) -> vec3<u32>
// op_vec3_constructor<f32>(e1: f32, v1: vec2<f32>) -> vec3<f32>
// op_vec3_constructor<f16>(e1: f16, v1: vec2<f16>) -> vec3<f16>
// op_vec3_constructor(e1: bool, v1: vec2<bool>) -> vec3<bool>
// op_vec3_constructor(e1: AbstractFloat, v1: vec2<AbstractFloat>) -> vec3<AbstractFloat>
// op_vec3_constructor(e1: AbstractInt, v1: vec2<AbstractInt>) -> vec3<AbstractInt>
// op_vec3_constructor(e1: i32, v1: vec2<i32>) -> vec3<i32>
// op_vec3_constructor(e1: u32, v1: vec2<u32>) -> vec3<u32>
// op_vec3_constructor(e1: f32, v1: vec2<f32>) -> vec3<f32>
// op_vec3_constructor(e1: f16, v1: vec2<f16>) -> vec3<f16>

// // parameterless is handled in Rust code

// op_vec4_constructor<bool>(e: bool) -> vec4<bool>
// op_vec4_constructor<i32>(e: i32) -> vec4<i32>
// op_vec4_constructor<u32>(e: u32) -> vec4<u32>
// op_vec4_constructor<f32>(e: f32) -> vec4<f32>
// op_vec4_constructor<f16>(e: f16) -> vec4<f16>
// op_vec4_constructor(e: bool) -> vec4<bool>
// op_vec4_constructor(e: AbstractFloat) -> vec4<AbstractFloat>
// op_vec4_constructor(e: AbstractInt) -> vec4<AbstractInt>
// op_vec4_constructor(e: i32) -> vec4<i32>
// op_vec4_constructor(e: u32) -> vec4<u32>
// op_vec4_constructor(e: f32) -> vec4<f32>
// op_vec4_constructor(e: f16) -> vec4<f16>

// op_vec4_constructor<bool>(e: vec4<bool>) -> vec4<bool>
// op_vec4_constructor<i32>(e: vec4<i32>) -> vec4<i32>
// op_vec4_constructor<u32>(e: vec4<u32>) -> vec4<u32>
// op_vec4_constructor<f32>(e: vec4<f32>) -> vec4<f32>
// op_vec4_constructor<f16>(e: vec4<f16>) -> vec4<f16>
// op_vec4_constructor(e: vec4<bool>) -> vec4<bool>
// op_vec4_constructor(e: vec4<AbstractFloat>) -> vec4<AbstractFloat>
// op_vec4_constructor(e: vec4<AbstractInt>) -> vec4<AbstractInt>
// op_vec4_constructor(e: vec4<i32>) -> vec4<i32>
// op_vec4_constructor(e: vec4<u32>) -> vec4<u32>
// op_vec4_constructor(e: vec4<f32>) -> vec4<f32>
// op_vec4_constructor(e: vec4<f16>) -> vec4<f16>

// op_vec4_constructor<bool>(e1: bool, e2: bool, e3: bool, e4: bool) -> vec4<bool>
// op_vec4_constructor<i32>(e1: i32, e2: i32, e3: i32, e4: i32) -> vec4<i32>
// op_vec4_constructor<u32>(e1: u32, e2: u32, e3: u32, e4: u32) -> vec4<u32>
// op_vec4_constructor<f32>(e1: f32, e2: f32, e3: f32, e4: f32) -> vec4<f32>
// op_vec4_constructor<f16>(e1: f16, e2: f16, e3: f16, e4: f16) -> vec4<f16>
// op_vec4_constructor(e1: bool, e2: bool, e3: bool, e4: bool) -> vec4<bool>
// op_vec4_constructor(e1: AbstractFloat, e2: AbstractFloat, e3: AbstractFloat, e4: AbstractFloat) -> vec4<AbstractFloat>
// op_vec4_constructor(e1: AbstractInt, e2: AbstractInt, e3: AbstractInt, e4: AbstractInt) -> vec4<AbstractInt>
// op_vec4_constructor(e1: i32, e2: i32, e3: i32, e4: i32) -> vec4<i32>
// op_vec4_constructor(e1: u32, e2: u32, e3: u32, e4: u32) -> vec4<u32>
// op_vec4_constructor(e1: f32, e2: f32, e3: f32, e4: f32) -> vec4<f32>
// op_vec4_constructor(e1: f16, e2: f16, e3: f16, e4: f16) -> vec4<f16>

// op_vec4_constructor<bool>(e1: bool, v1: vec2<bool>, e2: bool) -> vec4<bool>
// op_vec4_constructor<i32>(e1: i32, v1: vec2<i32>, e2: i32) -> vec4<i32>
// op_vec4_constructor<u32>(e1: u32, v1: vec2<u32>, e2: u32) -> vec4<u32>
// op_vec4_constructor<f32>(e1: f32, v1: vec2<f32>, e2: f32) -> vec4<f32>
// op_vec4_constructor<f16>(e1: f16, v1: vec2<f16>, e2: f16) -> vec4<f16>
// op_vec4_constructor(e1: bool, v1: vec2<bool>, e2: bool) -> vec4<bool>
// op_vec4_constructor(e1: AbstractFloat, v1: vec2<AbstractFloat>, e2: AbstractFloat) -> vec4<AbstractFloat>
// op_vec4_constructor(e1: AbstractInt, v1: vec2<AbstractInt>, e2: AbstractInt) -> vec4<AbstractInt>
// op_vec4_constructor(e1: i32, v1: vec2<i32>, e2: i32) -> vec4<i32>
// op_vec4_constructor(e1: u32, v1: vec2<u32>, e2: u32) -> vec4<u32>
// op_vec4_constructor(e1: f32, v1: vec2<f32>, e2: f32) -> vec4<f32>
// op_vec4_constructor(e1: f16, v1: vec2<f16>, e2: f16) -> vec4<f16>

// op_vec4_constructor<bool>(e1: bool, e2: bool, v1: vec2<bool>) -> vec4<bool>
// op_vec4_constructor<i32>(e1: i32, e2: i32, v1: vec2<i32>) -> vec4<i32>
// op_vec4_constructor<u32>(e1: u32, e2: u32, v1: vec2<u32>) -> vec4<u32>
// op_vec4_constructor<f32>(e1: f32, e2: f32, v1: vec2<f32>) -> vec4<f32>
// op_vec4_constructor<f16>(e1: f16, e2: f16, v1: vec2<f16>) -> vec4<f16>
// op_vec4_constructor(e1: bool, e2: bool, v1: vec2<bool>) -> vec4<bool>
// op_vec4_constructor(e1: AbstractFloat, e2: AbstractFloat, v1: vec2<AbstractFloat>) -> vec4<AbstractFloat>
// op_vec4_constructor(e1: AbstractInt, e2: AbstractInt, v1: vec2<AbstractInt>) -> vec4<AbstractInt>
// op_vec4_constructor(e1: i32, e2: i32, v1: vec2<i32>) -> vec4<i32>
// op_vec4_constructor(e1: u32, e2: u32, v1: vec2<u32>) -> vec4<u32>
// op_vec4_constructor(e1: f32, e2: f32, v1: vec2<f32>) -> vec4<f32>
// op_vec4_constructor(e1: f16, e2: f16, v1: vec2<f16>) -> vec4<f16>

// op_vec4_constructor<bool>(v1: vec2<bool>, v2: vec2<bool>) -> vec4<bool>
// op_vec4_constructor<i32>(v1: vec2<i32>, v2: vec2<i32>) -> vec4<i32>
// op_vec4_constructor<u32>(v1: vec2<u32>, v2: vec2<u32>) -> vec4<u32>
// op_vec4_constructor<f32>(v1: vec2<f32>, v2: vec2<f32>) -> vec4<f32>
// op_vec4_constructor<f16>(v1: vec2<f16>, v2: vec2<f16>) -> vec4<f16>
// op_vec4_constructor(v1: vec2<bool>, v2: vec2<bool>) -> vec4<bool>
// op_vec4_constructor(v1: vec2<AbstractFloat>, v2: vec2<AbstractFloat>) -> vec4<AbstractFloat>
// op_vec4_constructor(v1: vec2<AbstractInt>, v2: vec2<AbstractInt>) -> vec4<AbstractInt>
// op_vec4_constructor(v1: vec2<i32>, v2: vec2<i32>) -> vec4<i32>
// op_vec4_constructor(v1: vec2<u32>, v2: vec2<u32>) -> vec4<u32>
// op_vec4_constructor(v1: vec2<f32>, v2: vec2<f32>) -> vec4<f32>
// op_vec4_constructor(v1: vec2<f16>, v2: vec2<f16>) -> vec4<f16>

// op_vec4_constructor<bool>(v1: vec2<bool>, e1: bool, e2: bool) -> vec4<bool>
// op_vec4_constructor<i32>(v1: vec2<i32>, e1: i32, e2: i32) -> vec4<i32>
// op_vec4_constructor<u32>(v1: vec2<u32>, e1: u32, e2: u32) -> vec4<u32>
// op_vec4_constructor<f32>(v1: vec2<f32>, e1: f32, e2: f32) -> vec4<f32>
// op_vec4_constructor<f16>(v1: vec2<f16>, e1: f16, e2: f16) -> vec4<f16>
// op_vec4_constructor(v1: vec2<bool>, e1: bool, e2: bool) -> vec4<bool>
// op_vec4_constructor(v1: vec2<AbstractFloat>, e1: AbstractFloat, e2: AbstractFloat) -> vec4<AbstractFloat>
// op_vec4_constructor(v1: vec2<AbstractInt>, e1: AbstractInt, e2: AbstractInt) -> vec4<AbstractInt>
// op_vec4_constructor(v1: vec2<i32>, e1: i32, e2: i32) -> vec4<i32>
// op_vec4_constructor(v1: vec2<u32>, e1: u32, e2: u32) -> vec4<u32>
// op_vec4_constructor(v1: vec2<f32>, e1: f32, e2: f32) -> vec4<f32>
// op_vec4_constructor(v1: vec2<f16>, e1: f16, e2: f16) -> vec4<f16>

// op_vec4_constructor<bool>(v1: vec3<bool>, e1: bool) -> vec4<bool>
// op_vec4_constructor<i32>(v1: vec3<i32>, e1: i32) -> vec4<i32>
// op_vec4_constructor<u32>(v1: vec3<u32>, e1: u32) -> vec4<u32>
// op_vec4_constructor<f32>(v1: vec3<f32>, e1: f32) -> vec4<f32>
// op_vec4_constructor<f16>(v1: vec3<f16>, e1: f16) -> vec4<f16>
// op_vec4_constructor(v1: vec3<bool>, e1: bool) -> vec4<bool>
// op_vec4_constructor(v1: vec3<AbstractFloat>, e1: AbstractFloat) -> vec4<AbstractFloat>
// op_vec4_constructor(v1: vec3<AbstractInt>, e1: AbstractInt) -> vec4<AbstractInt>
// op_vec4_constructor(v1: vec3<i32>, e1: i32) -> vec4<i32>
// op_vec4_constructor(v1: vec3<u32>, e1: u32) -> vec4<u32>
// op_vec4_constructor(v1: vec3<f32>, e1: f32) -> vec4<f32>
// op_vec4_constructor(v1: vec3<f16>, e1: f16) -> vec4<f16>

// op_vec4_constructor<bool>(e1: bool, v1: vec3<bool>) -> vec4<bool>
// op_vec4_constructor<i32>(e1: i32, v1: vec3<i32>) -> vec4<i32>
// op_vec4_constructor<u32>(e1: u32, v1: vec3<u32>) -> vec4<u32>
// op_vec4_constructor<f32>(e1: f32, v1: vec3<f32>) -> vec4<f32>
// op_vec4_constructor<f16>(e1: f16, v1: vec3<f16>) -> vec4<f16>
// op_vec4_constructor(e1: bool, v1: vec3<bool>) -> vec4<bool>
// op_vec4_constructor(e1: AbstractFloat, v1: vec3<AbstractFloat>) -> vec4<AbstractFloat>
// op_vec4_constructor(e1: AbstractInt, v1: vec3<AbstractInt>) -> vec4<AbstractInt>
// op_vec4_constructor(e1: i32, v1: vec3<i32>) -> vec4<i32>
// op_vec4_constructor(e1: u32, v1: vec3<u32>) -> vec4<u32>
// op_vec4_constructor(e1: f32, v1: vec3<f32>) -> vec4<f32>
// op_vec4_constructor(e1: f16, v1: vec3<f16>) -> vec4<f16>
