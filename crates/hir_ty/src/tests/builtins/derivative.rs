#![expect(clippy::too_many_lines, reason = "snapshot test data")]

use expect_test::expect;
use syntax::ExtensionsConfig;

use crate::tests::check_infer;

#[test]
fn dpdx() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let _f32 = dpdx(f32());
    let _vecN = dpdx(vec2<f32>());
}
",
        expect![[r#"
            19..23 '_f32': f32
            26..37 'dpdx(f32())': f32
            31..36 'f32()': f32
            47..52 '_vecN': vec2<f32>
            55..72 'dpdx(v...32>())': vec2<f32>
            60..71 'vec2<f32>()': vec2<f32>
        "#]],
    );
}

#[test]
fn dpdxCoarse() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let _f32 = dpdxCoarse(f32());
    let _vecN = dpdxCoarse(vec2<f32>());
}
",
        expect![[r#"
            19..23 '_f32': f32
            26..43 'dpdxCo...f32())': f32
            37..42 'f32()': f32
            53..58 '_vecN': vec2<f32>
            61..84 'dpdxCo...32>())': vec2<f32>
            72..83 'vec2<f32>()': vec2<f32>
        "#]],
    );
}

#[test]
fn dpdxFine() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let _f32 = dpdxFine(f32());
    let _vecN = dpdxFine(vec2<f32>());
}
",
        expect![[r#"
            19..23 '_f32': f32
            26..41 'dpdxFine(f32())': f32
            35..40 'f32()': f32
            51..56 '_vecN': vec2<f32>
            59..80 'dpdxFi...32>())': vec2<f32>
            68..79 'vec2<f32>()': vec2<f32>
        "#]],
    );
}

#[test]
fn dpdy() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let _f32 = dpdy(f32());
    let _vecN = dpdy(vec2<f32>());
}
",
        expect![[r#"
            19..23 '_f32': f32
            26..37 'dpdy(f32())': f32
            31..36 'f32()': f32
            47..52 '_vecN': vec2<f32>
            55..72 'dpdy(v...32>())': vec2<f32>
            60..71 'vec2<f32>()': vec2<f32>
        "#]],
    );
}

#[test]
fn dpdyCoarse() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let _f32 = dpdyCoarse(f32());
    let _vecN = dpdyCoarse(vec2<f32>());
}
",
        expect![[r#"
            19..23 '_f32': f32
            26..43 'dpdyCo...f32())': f32
            37..42 'f32()': f32
            53..58 '_vecN': vec2<f32>
            61..84 'dpdyCo...32>())': vec2<f32>
            72..83 'vec2<f32>()': vec2<f32>
        "#]],
    );
}

#[test]
fn dpdyFine() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let _f32 = dpdyFine(f32());
    let _vecN = dpdyFine(vec2<f32>());
}
",
        expect![[r#"
            19..23 '_f32': f32
            26..41 'dpdyFine(f32())': f32
            35..40 'f32()': f32
            51..56 '_vecN': vec2<f32>
            59..80 'dpdyFi...32>())': vec2<f32>
            68..79 'vec2<f32>()': vec2<f32>
        "#]],
    );
}

#[test]
fn fwidth() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let _f32 = fwidth(f32());
    let _vecN = fwidth(vec2<f32>());
}
",
        expect![[r#"
            19..23 '_f32': f32
            26..39 'fwidth(f32())': f32
            33..38 'f32()': f32
            49..54 '_vecN': vec2<f32>
            57..76 'fwidth...32>())': vec2<f32>
            64..75 'vec2<f32>()': vec2<f32>
        "#]],
    );
}

#[test]
fn fwidthCoarse() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let _f32 = fwidthCoarse(f32());
    let _vecN = fwidthCoarse(vec2<f32>());
}
",
        expect![[r#"
            19..23 '_f32': f32
            26..45 'fwidth...f32())': f32
            39..44 'f32()': f32
            55..60 '_vecN': vec2<f32>
            63..88 'fwidth...32>())': vec2<f32>
            76..87 'vec2<f32>()': vec2<f32>
        "#]],
    );
}

#[test]
fn fwidthFine() {
    check_infer(
        ExtensionsConfig::default(),
        "
fn foo() {
    let _f32 = fwidthFine(f32());
    let _vecN = fwidthFine(vec2<f32>());
}
",
        expect![[r#"
            19..23 '_f32': f32
            26..43 'fwidth...f32())': f32
            37..42 'f32()': f32
            53..58 '_vecN': vec2<f32>
            61..84 'fwidth...32>())': vec2<f32>
            72..83 'vec2<f32>()': vec2<f32>
        "#]],
    );
}
