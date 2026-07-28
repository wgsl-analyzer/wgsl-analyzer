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
fn not_constructible() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
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
    let reference = ref<function, u32, read>();
    let tex = texture_storage_2d<rgba16float, write>();
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
            279..303 'signed...atomic': [error]
            306..319 'atomic<i32>()': [error]
            329..355 'unsign...atomic': [error]
            358..371 'atomic<u32>()': [error]
            382..391 'structure': Foo
            394..399 'Foo()': Foo
            410..417 'pointer': [error]
            420..446 'ptr<fu...ead>()': [error]
            456..465 'reference': [error]
            472..480 'function': [error]
            482..485 'u32': [error]
            487..491 'read': [error]
            504..507 'tex': [error]
            510..550 'textur...ite>()': [error]
            79..92 'array<bool>()': type `array<bool>` is not constructible
            128..140 'array<i32>()': type `array<i32>` is not constructible
            178..190 'array<u32>()': type `array<u32>` is not constructible
            217..229 'array<f32>()': type `array<f32>` is not constructible
            256..268 'array<f16>()': type `array<f16>` is not constructible
            306..319 'atomic<i32>()': type `atomic<i32>` is not constructible
            306..319 'atomic<i32>()': type `atomic<i32>` is not constructible
            358..371 'atomic<u32>()': type `atomic<u32>` is not constructible
            358..371 'atomic<u32>()': type `atomic<u32>` is not constructible
            394..399 'Foo()': type `Foo` is not constructible
            420..446 'ptr<fu...ead>()': type `ptr<function, u32, read>` is not constructible
            420..446 'ptr<fu...ead>()': type `ptr<function, u32, read>` is not constructible
            [EditionedFileId(Id(1c00))] ExpectedLoweredKind { expression: Idx::<Expression>(19), expected: Variable, actual: Enumerant, path: Path(ModPath("function")) } in Body
            [EditionedFileId(Id(1c00))] AssignmentNotAReference { left_side: Idx::<Expression>(19), actual: Type(2802) } in Body
            [EditionedFileId(Id(1c00))] ExpectedLoweredKind { expression: Idx::<Expression>(21), expected: Variable, actual: Type, path: Path(ModPath("u32")) } in Body
            [EditionedFileId(Id(1c00))] AssignmentNotAReference { left_side: Idx::<Expression>(21), actual: Type(2802) } in Body
            [EditionedFileId(Id(1c00))] ExpectedLoweredKind { expression: Idx::<Expression>(23), expected: Variable, actual: Enumerant, path: Path(ModPath("read")) } in Body
            [EditionedFileId(Id(1c00))] AssignmentNotAReference { left_side: Idx::<Expression>(23), actual: Type(2802) } in Body
            [EditionedFileId(Id(1c00))] AssignmentNotAReference { left_side: Idx::<Expression>(25), actual: Type(2802) } in Body
            510..550 'textur...ite>()': type `texture_storage_2d<rgba16float,write>` is not constructible
            510..550 'textur...ite>()': type `texture_storage_2d<rgba16float,write>` is not constructible
        "#]],
    );
}
