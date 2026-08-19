// #![expect(non_snake_case, reason = "name based on WGSL builtins")]

use expect_test::expect;
use syntax::Capabilities;

use crate::tests::{check_infer, check_infer_with_capabilities};

#[test]
fn zero_value_constructors() {
    check_infer(
        "
enable f16;
struct Foo { bar: bool }
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

    let float_32_matrix = mat2x2<f32>();
    let float_16_matrix = mat2x2<f16>();

    let boolean_array = array<bool, 1>();
    let signed_integer_32_array = array<i32, 1>();
    let unsigned_integer_32_array = array<u32, 1>();
    let float_32_array = array<f32, 1>();
    let float_16_array = array<f16, 1>();

    let structure = Foo();

    // AbstractInt and AbstractFloat do not have builtin functions to access the zero values
}
",
        expect![[r#"
            56..63 'boolean': bool
            66..72 'bool()': bool
            82..99 'signed...ger_32': i32
            102..107 'i32()': i32
            117..136 'unsign...ger_32': u32
            139..144 'u32()': u32
            154..162 'float_32': f32
            165..170 'f32()': f32
            180..188 'float_16': f16
            191..196 'f16()': f16
            207..221 'boolean_vector': vec2<bool>
            224..236 'vec2<bool>()': vec2<bool>
            246..270 'signed...vector': vec2<i32>
            273..284 'vec2<i32>()': vec2<i32>
            294..320 'unsign...vector': vec2<u32>
            323..334 'vec2<u32>()': vec2<u32>
            344..359 'float_32_vector': vec2<f32>
            362..373 'vec2<f32>()': vec2<f32>
            383..398 'float_16_vector': vec2<f16>
            401..412 'vec2<f16>()': vec2<f16>
            423..442 'abstra...vector': vec2<i32>
            445..451 'vec2()': vec2<integer>
            462..477 'float_32_matrix': mat2x2<f32>
            480..493 'mat2x2<f32>()': mat2x2<f32>
            503..518 'float_16_matrix': mat2x2<f16>
            521..534 'mat2x2<f16>()': mat2x2<f16>
            545..558 'boolean_array': array<bool, 1>
            561..577 'array<..., 1>()': array<bool, 1>
            587..610 'signed..._array': array<i32, 1>
            613..628 'array<i32, 1>()': array<i32, 1>
            638..663 'unsign..._array': array<u32, 1>
            666..681 'array<u32, 1>()': array<u32, 1>
            691..705 'float_32_array': array<f32, 1>
            708..723 'array<f32, 1>()': array<f32, 1>
            733..747 'float_16_array': array<f16, 1>
            750..765 'array<f16, 1>()': array<f16, 1>
            776..785 'structure': Foo
            788..793 'Foo()': Foo
        "#]],
    );
}

#[test]
fn naga() {
    check_infer_with_capabilities(
        Capabilities {
            shader_int64: true,
            ..Default::default()
        },
        "
fn foo() {
    let signed_integer_64 = i64();
    let unsigned_integer_64 = u64();

    let signed_integer_64_vector = vec2<i64>();
    let unsigned_integer_64_vector = vec2<u64>();

    let signed_integer_64_array = array<i64, 1>();
    let unsigned_integer_64_array = array<u64, 1>();
}
",
        expect![[r#"
            19..36 'signed...ger_64': i64
            39..44 'i64()': i64
            54..73 'unsign...ger_64': u64
            76..81 'u64()': u64
            92..116 'signed...vector': vec2<i64>
            119..130 'vec2<i64>()': vec2<i64>
            140..166 'unsign...vector': vec2<u64>
            169..180 'vec2<u64>()': vec2<u64>
            191..214 'signed..._array': array<i64, 1>
            217..232 'array<i64, 1>()': array<i64, 1>
            242..267 'unsign..._array': array<u64, 1>
            270..285 'array<u64, 1>()': array<u64, 1>
        "#]],
    );
}

#[test]
fn not_constructible() {
    check_infer(
        "
enable f16;
struct Foo { bar: atomic<u32> }
fn foo() {
    let boolean_array = array<bool>();
    let signed_integer_32_array = array<i32>();
    let unsigned_integer_32_array = array<u32>();
    let float_32_array = array<f32>();
    let float_16_array = array<f16>();

    let signed_integer_32_atomic = atomic<i32>();
    let unsigned_integer_32_atomic = atomic<u32>();

    let structure = Foo();

    let pointer = ptr<function, u32, read>();
    // ref doesn't even parse
    // let reference = ref<function, u32, read>();
    let tex = texture_storage_2d<rgba16float, write>();

    let _array = array();
    let _atomic = atomic();
    let pointer = ptr();
    // ref doesn't even parse
    // let reference = ref();
    let tex = texture_storage_2d();
}
",
        expect![[r#"
            63..76 'boolean_array': array<bool>
            79..92 'array<bool>()': array<bool>
            102..125 'signed..._array': array<i32>
            128..140 'array<i32>()': array<i32>
            150..175 'unsign..._array': array<u32>
            178..190 'array<u32>()': array<u32>
            200..214 'float_32_array': array<f32>
            217..229 'array<f32>()': array<f32>
            239..253 'float_16_array': array<f16>
            256..268 'array<f16>()': array<f16>
            279..303 'signed...atomic': atomic<i32>
            306..319 'atomic<i32>()': atomic<i32>
            329..355 'unsign...atomic': atomic<u32>
            358..371 'atomic<u32>()': atomic<u32>
            382..391 'structure': Foo
            394..399 'Foo()': Foo
            410..417 'pointer': ptr<function, u32, read>
            420..446 'ptr<fu...ead>()': ptr<function, u32, read>
            537..540 'tex': texture_storage_2d<rgba16float,write>
            543..583 'textur...ite>()': texture_storage_2d<rgba16float,write>
            594..600 '_array': array<[error]>
            603..610 'array()': array<[error]>
            620..627 '_atomic': atomic<[error]>
            630..638 'atomic()': atomic<[error]>
            648..655 'pointer': [error]
            658..663 'ptr()': [error]
            733..736 'tex': [error]
            739..759 'textur...e_2d()': [error]
            79..92 'array<bool>()': type `array<bool>` is not constructible
            128..140 'array<i32>()': type `array<i32>` is not constructible
            178..190 'array<u32>()': type `array<u32>` is not constructible
            217..229 'array<f32>()': type `array<f32>` is not constructible
            256..268 'array<f16>()': type `array<f16>` is not constructible
            306..319 'atomic<i32>()': type `atomic<i32>` is not constructible
            358..371 'atomic<u32>()': type `atomic<u32>` is not constructible
            394..399 'Foo()': type `Foo` is not constructible
            420..446 'ptr<fu...ead>()': type `ptr<function, u32, read>` is not constructible
            543..583 'textur...ite>()': type `texture_storage_2d<rgba16float,write>` is not constructible
            603..610 'array()': no overload of function `array` found that takes no arguments
            630..638 'atomic()': expected 1 template arguments, but got 0
            630..638 'atomic()': missing template argument, expected a type
            658..663 'ptr()': expected 2 to 3 template arguments, but got 0
            658..663 'ptr()': missing template argument, expected an enum
            739..759 'textur...e_2d()': expected 1 to 2 template arguments, but got 0
            739..759 'textur...e_2d()': missing template argument, expected an enum
        "#]],
    );
}

#[test]
fn not_constructible_no_template() {
    check_infer(
        "
fn foo() {
    let structure = array();
}
",
        expect![[r#"
            19..28 'structure': array<[error]>
            31..38 'array()': array<[error]>
            31..38 'array()': no overload of function `array` found that takes no arguments
        "#]],
    );
}

#[test]
fn not_constructible_type_expectation() {
    check_infer(
        "
fn foo() {
    const b: array<u32, 3> = array();
}
",
        expect![[r#"
            21..22 'b': array<u32, 3>
            40..47 'array()': array<[error]>
            40..47 'array()': no overload of function `array` found that takes no arguments
        "#]],
    );
}
