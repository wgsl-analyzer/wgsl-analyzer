// #![expect(non_snake_case, reason = "name based on WGSL builtins")]

use expect_test::expect;
use syntax::ExtensionsConfig;

use crate::tests::check_infer;

#[test]
fn zero_value_constructors() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let boolean = bool();
    let signed_integer_32 = i32();
    let unsigned_integer_32 = u32();
    let float_32 = f32();
    let float_16 = f16();

    let boolean_vector = vec2<bool>();
    let signed_integer_32_vector = vec2<i32>();
    let unsigned_integer_32_vector = vec2<u32>();
    let float_32_vector = vec2<f32>();
    let float_16_vector = vec2<f16>();

    let abstract_int_vector = vec2();

    let boolean_matrix = mat2x2<bool>();
    let signed_integer_32_matrix = mat2x2<i32>();
    let unsigned_integer_32_matrix = mat2x2<u32>();
    let float_32_matrix = mat2x2<f32>();
    let float_16_matrix = mat2x2<f16>();

    let boolean_array = array<bool>();
    let signed_integer_32_array = array<i32>();
    let unsigned_integer_32_array = array<u32>();
    let float_32_array = array<f32>();
    let float_16_array = array<f16>();

    struct Foo { bar: bool }
    let structure = Foo();

    // AbstractInt and AbstractFloat do not have builtin functions to access the zero values
}
",
        expect![[r#"
            31..38 'boolean': bool
            41..47 'bool()': bool
            57..74 'signed...ger_32': i32
            77..82 'i32()': i32
            92..111 'unsign...ger_32': u32
            114..119 'u32()': u32
            129..137 'float_32': f32
            140..145 'f32()': f32
            155..163 'float_16': f16
            166..171 'f16()': f16
            182..196 'boolean_vector': vec2<bool>
            199..211 'vec2<bool>()': vec2<bool>
            221..245 'signed...vector': vec2<i32>
            248..259 'vec2<i32>()': vec2<i32>
            269..295 'unsign...vector': vec2<u32>
            298..309 'vec2<u32>()': vec2<u32>
            319..334 'float_32_vector': vec2<f32>
            337..348 'vec2<f32>()': vec2<f32>
            358..373 'float_16_vector': vec2<f16>
            376..387 'vec2<f16>()': vec2<f16>
            398..417 'abstra...vector': vec2<i32>
            420..426 'vec2()': vec2<integer>
            437..451 'boolean_matrix': mat2x2<[error]>
            454..468 'mat2x2<bool>()': mat2x2<[error]>
            478..502 'signed...matrix': mat2x2<[error]>
            505..518 'mat2x2<i32>()': mat2x2<[error]>
            528..554 'unsign...matrix': mat2x2<[error]>
            557..570 'mat2x2<u32>()': mat2x2<[error]>
            580..595 'float_32_matrix': mat2x2<f32>
            598..611 'mat2x2<f32>()': mat2x2<f32>
            621..636 'float_16_matrix': mat2x2<f16>
            639..652 'mat2x2<f16>()': mat2x2<f16>
            663..676 'boolean_array': array<bool>
            679..692 'array<bool>()': array<bool>
            702..725 'signed..._array': array<i32>
            728..740 'array<i32>()': array<i32>
            750..775 'unsign..._array': array<u32>
            778..790 'array<u32>()': array<u32>
            800..814 'float_32_array': array<f32>
            817..829 'array<f32>()': array<f32>
            839..853 'float_16_array': array<f16>
            856..868 'array<f16>()': array<f16>
            461..465 'bool': unexpected template argument, expected one of: f32 or f16
            512..515 'i32': unexpected template argument, expected one of: f32 or f16
            564..567 'u32': unexpected template argument, expected one of: f32 or f16
        "#]],
    );
}

#[test]
fn not_constructible() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
enable f16;
fn foo() {
    let boolean_matrix = mat2x2<bool>();
    let signed_integer_32_matrix = mat2x2<i32>();
    let unsigned_integer_32_matrix = mat2x2<u32>();
    let float_32_matrix = mat2x2<f32>();
    let float_16_matrix = mat2x2<f16>();

    let boolean_array = array<bool>();
    let signed_integer_32_array = array<i32>();
    let unsigned_integer_32_array = array<u32>();
    let float_32_array = array<f32>();
    let float_16_array = array<f16>();

    struct Foo { bar: bool }
    let structure = Foo();

    // AbstractInt and AbstractFloat do not have builtin functions to access the zero values
}
",
        expect![[r#"
            31..45 'boolean_matrix': mat2x2<[error]>
            48..62 'mat2x2<bool>()': mat2x2<[error]>
            72..96 'signed...matrix': mat2x2<[error]>
            99..112 'mat2x2<i32>()': mat2x2<[error]>
            122..148 'unsign...matrix': mat2x2<[error]>
            151..164 'mat2x2<u32>()': mat2x2<[error]>
            174..189 'float_32_matrix': mat2x2<f32>
            192..205 'mat2x2<f32>()': mat2x2<f32>
            215..230 'float_16_matrix': mat2x2<f16>
            233..246 'mat2x2<f16>()': mat2x2<f16>
            257..270 'boolean_array': array<bool>
            273..286 'array<bool>()': array<bool>
            296..319 'signed..._array': array<i32>
            322..334 'array<i32>()': array<i32>
            344..369 'unsign..._array': array<u32>
            372..384 'array<u32>()': array<u32>
            394..408 'float_32_array': array<f32>
            411..423 'array<f32>()': array<f32>
            433..447 'float_16_array': array<f16>
            450..462 'array<f16>()': array<f16>
            55..59 'bool': unexpected template argument, expected one of: f32 or f16
            106..109 'i32': unexpected template argument, expected one of: f32 or f16
            158..161 'u32': unexpected template argument, expected one of: f32 or f16
        "#]],
    );
}
