#![expect(clippy::too_many_lines, reason = "snapshot test data")]

use expect_test::expect;
use syntax::Capabilities;

use crate::tests::{check_infer, check_infer_with_capabilities};

#[test]
fn all() {
    check_infer(
        "
fn foo() {
    let vec = all(vec2<bool>());
    let one = all(bool());
}
",
        expect![[r#"
            19..22 'vec': bool
            25..42 'all(ve...ol>())': bool
            29..41 'vec2<bool>()': vec2<bool>
            52..55 'one': bool
            58..69 'all(bool())': bool
            62..68 'bool()': bool
        "#]],
    );
}

#[test]
fn any() {
    check_infer(
        "
fn foo() {
    let vec = any(vec2<bool>());
    let one = any(bool());
}
",
        expect![[r#"
            19..22 'vec': bool
            25..42 'any(ve...ol>())': bool
            29..41 'vec2<bool>()': vec2<bool>
            52..55 'one': bool
            58..69 'any(bool())': bool
            62..68 'bool()': bool
        "#]],
    );
}

#[test]
fn select() {
    check_infer_with_capabilities(
        Capabilities {
            shader_int64: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let _bool = select(bool(), bool(), bool());
    let _AbstractInt = select(1, 1, bool());
    let _AbstractFloat = select(1.0, 1.0, bool());
    let _i32 = select(i32(), i32(), bool());
    let _u32 = select(u32(), u32(), bool());
    let _f32 = select(f32(), f32(), bool());
    let _f16 = select(f16(), f16(), bool());
    let _i64 = select(i64(), i64(), bool());
    let _u64 = select(u64(), u64(), bool());
    let _bool_vec = select(vec2<bool>(), vec2<bool>(), bool());
    let _AbstractInt_vec = select(vec2(1), vec2(1), bool());
    let _AbstractFloat_vec = select(vec2(1.0), vec2(1.0), bool());
    let _i32_vec = select(vec2<i32>(), vec2<i32>(), bool());
    let _u32_vec = select(vec2<u32>(), vec2<u32>(), bool());
    let _f32_vec = select(vec2<f32>(), vec2<f32>(), bool());
    let _f16_vec = select(vec2<f16>(), vec2<f16>(), bool());
    let _i64_vec = select(vec2<i64>(), vec2<i64>(), bool());
    let _u64_vec = select(vec2<u64>(), vec2<u64>(), bool());
}
",
        expect![[r#"
            31..36 '_bool': bool
            39..69 'select...ool())': bool
            46..52 'bool()': bool
            54..60 'bool()': bool
            62..68 'bool()': bool
            79..91 '_AbstractInt': i32
            94..114 'select...ool())': integer
            101..102 '1': integer
            104..105 '1': integer
            107..113 'bool()': bool
            124..138 '_AbstractFloat': f32
            141..165 'select...ool())': float
            148..151 '1.0': float
            153..156 '1.0': float
            158..164 'bool()': bool
            175..179 '_i32': i32
            182..210 'select...ool())': i32
            189..194 'i32()': i32
            196..201 'i32()': i32
            203..209 'bool()': bool
            220..224 '_u32': u32
            227..255 'select...ool())': u32
            234..239 'u32()': u32
            241..246 'u32()': u32
            248..254 'bool()': bool
            265..269 '_f32': f32
            272..300 'select...ool())': f32
            279..284 'f32()': f32
            286..291 'f32()': f32
            293..299 'bool()': bool
            310..314 '_f16': f16
            317..345 'select...ool())': f16
            324..329 'f16()': f16
            331..336 'f16()': f16
            338..344 'bool()': bool
            355..359 '_i64': i64
            362..390 'select...ool())': i64
            369..374 'i64()': i64
            376..381 'i64()': i64
            383..389 'bool()': bool
            400..404 '_u64': u64
            407..435 'select...ool())': u64
            414..419 'u64()': u64
            421..426 'u64()': u64
            428..434 'bool()': bool
            445..454 '_bool_vec': vec2<bool>
            457..499 'select...ool())': vec2<bool>
            464..476 'vec2<bool>()': vec2<bool>
            478..490 'vec2<bool>()': vec2<bool>
            492..498 'bool()': bool
            509..525 '_Abstr...nt_vec': vec2<i32>
            528..560 'select...ool())': vec2<integer>
            535..542 'vec2(1)': vec2<integer>
            540..541 '1': integer
            544..551 'vec2(1)': vec2<integer>
            549..550 '1': integer
            553..559 'bool()': bool
            570..588 '_Abstr...at_vec': vec2<f32>
            591..627 'select...ool())': vec2<float>
            598..607 'vec2(1.0)': vec2<float>
            603..606 '1.0': float
            609..618 'vec2(1.0)': vec2<float>
            614..617 '1.0': float
            620..626 'bool()': bool
            637..645 '_i32_vec': vec2<i32>
            648..688 'select...ool())': vec2<i32>
            655..666 'vec2<i32>()': vec2<i32>
            668..679 'vec2<i32>()': vec2<i32>
            681..687 'bool()': bool
            698..706 '_u32_vec': vec2<u32>
            709..749 'select...ool())': vec2<u32>
            716..727 'vec2<u32>()': vec2<u32>
            729..740 'vec2<u32>()': vec2<u32>
            742..748 'bool()': bool
            759..767 '_f32_vec': vec2<f32>
            770..810 'select...ool())': vec2<f32>
            777..788 'vec2<f32>()': vec2<f32>
            790..801 'vec2<f32>()': vec2<f32>
            803..809 'bool()': bool
            820..828 '_f16_vec': vec2<f16>
            831..871 'select...ool())': vec2<f16>
            838..849 'vec2<f16>()': vec2<f16>
            851..862 'vec2<f16>()': vec2<f16>
            864..870 'bool()': bool
            881..889 '_i64_vec': vec2<i64>
            892..932 'select...ool())': vec2<i64>
            899..910 'vec2<i64>()': vec2<i64>
            912..923 'vec2<i64>()': vec2<i64>
            925..931 'bool()': bool
            942..950 '_u64_vec': vec2<u64>
            953..993 'select...ool())': vec2<u64>
            960..971 'vec2<u64>()': vec2<u64>
            973..984 'vec2<u64>()': vec2<u64>
            986..992 'bool()': bool
        "#]],
    );
}
