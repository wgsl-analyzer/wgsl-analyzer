use expect_test::expect;
use syntax::Capabilities;

use crate::tests::{check_infer, check_infer_with_capabilities};

#[test]
fn type_alias_in_struct() {
    check_infer(
        "
        alias Foo = u32;
        struct S { x: Foo }

        fn foo() {
            let a = S(5);
            let b = a.x + 10u;
        }
        ",
        expect![[r#"
            57..58 'a': S
            61..65 'S(5)': S
            63..64 '5': integer
            75..76 'b': u32
            79..80 'a': S
            79..82 'a.x': u32
            79..88 'a.x + 10u': u32
            85..88 '10u': u32
        "#]],
    );
}

#[test]
fn field_expression_on_error_type() {
    check_infer(
        "
        fn foo() {
            let x = Nonsense();
            let a = x.nonsense;
        }
        ",
        expect![[r#"
            19..20 'x': [error]
            23..33 'Nonsense()': [error]
            43..44 'a': [error]
            47..48 'x': [error]
            47..57 'x.nonsense': [error]
            23..33 'Nonsense()': `Nonsense` not found in scope
        "#]],
    );
}

#[test]
fn index_expression_on_error_type() {
    check_infer(
        "
        fn foo() {
            let x = Nonsense();
            let a = x[0];
        }
        ",
        expect![[r#"
            19..20 'x': [error]
            23..33 'Nonsense()': [error]
            43..44 'a': [error]
            47..48 'x': [error]
            47..51 'x[0]': [error]
            49..50 '0': integer
            23..33 'Nonsense()': `Nonsense` not found in scope
        "#]],
    );
}

#[test]
fn ident_expression_infers_ref() {
    check_infer(
        "
        struct Bar { baz: u32 }

        fn foo() {
            var in_memory = Bar(5);
            let value = in_memory.baz + 10u;
        }
        ",
        expect![[r#"
            44..53 'in_memory': ref<function, Bar, read_write>
            56..62 'Bar(5)': Bar
            60..61 '5': integer
            72..77 'value': u32
            80..89 'in_memory': ref<function, Bar, read_write>
            80..93 'in_memory.baz': ref<function, u32, read_write>
            80..99 'in_mem... + 10u': u32
            96..99 '10u': u32
        "#]],
    );
}

#[test]
fn automatic_ptr_dereference() {
    check_infer(
        "
        struct MyData {
            alpha: f32,
            beta: f32,
        }

        @group(0) @binding(1)
        var<storage, read_write> mybuff: array<MyData>;

        fn my_op(index: u32) {
            mybuff[index].alpha = 1.0;
            let data = &mybuff[index];
            data.alpha = 1.0;
        }
        ",
        expect![[r#"
            97..103 'mybuff': ref<storage, array<MyData>, read_write>
            130..135 'index': u32
            148..154 'mybuff': ref<storage, array<MyData>, read_write>
            148..161 'mybuff[index]': ref<storage, MyData, read_write>
            148..167 'mybuff....alpha': ref<storage, f32, read_write>
            155..160 'index': u32
            170..173 '1.0': float
            183..187 'data': ptr<storage, MyData, read_write>
            190..204 '&mybuff[index]': ptr<storage, MyData, read_write>
            191..197 'mybuff': ref<storage, array<MyData>, read_write>
            191..204 'mybuff[index]': ref<storage, MyData, read_write>
            198..203 'index': u32
            210..214 'data': ptr<storage, MyData, read_write>
            210..220 'data.alpha': ref<storage, f32, read_write>
            223..226 '1.0': float
        "#]],
    );
}

#[test]
fn ptr_deref_is_ref() {
    check_infer(
        "
        fn foo() {
            var v = vec2(1, 2);
            let p = &v;
            p.x = 2;
        }
        ",
        expect![[r#"
            19..20 'v': ref<function, vec2<i32>, read_write>
            23..33 'vec2(1, 2)': vec2<integer>
            28..29 '1': integer
            31..32 '2': integer
            43..44 'p': ptr<function, vec2<i32>, read_write>
            47..49 '&v': ptr<function, vec2<i32>, read_write>
            48..49 'v': ref<function, vec2<i32>, read_write>
            55..56 'p': ptr<function, vec2<i32>, read_write>
            55..58 'p.x': ref<function, i32, read_write>
            61..62 '2': integer
        "#]],
    );
}

#[test]
fn vec_x_is_ref() {
    check_infer(
        "
        fn foo() {
            var v = vec2(1, 2);
            v.x = v.y;
        }
        ",
        expect![[r#"
            19..20 'v': ref<function, vec2<i32>, read_write>
            23..33 'vec2(1, 2)': vec2<integer>
            28..29 '1': integer
            31..32 '2': integer
            39..40 'v': ref<function, vec2<i32>, read_write>
            39..42 'v.x': ref<function, i32, read_write>
            45..46 'v': ref<function, vec2<i32>, read_write>
            45..48 'v.y': ref<function, i32, read_write>
        "#]],
    );
}

#[test]
fn vec_field_is_not_ref() {
    check_infer(
        "
        fn foo() {
            let not_ref = vec2(1, 2).x;
        }
        ",
        expect![[r#"
            19..26 'not_ref': i32
            29..39 'vec2(1, 2)': vec2<integer>
            29..41 'vec2(1, 2).x': integer
            34..35 '1': integer
            37..38 '2': integer
        "#]],
    );
}

#[test]
fn struct_field_is_not_ref() {
    check_infer(
        "
        struct Bar { baz: u32 }
        fn foo() {
            let not_ref = Bar(0).baz;
        }
        ",
        expect![[r#"
            43..50 'not_ref': u32
            53..59 'Bar(0)': Bar
            53..63 'Bar(0).baz': u32
            57..58 '0': integer
        "#]],
    );
}

#[test]
fn no_such_field_on_struct_ref() {
    check_infer(
        "
        struct Bar { baz: u32 }
        fn foo() {
            var bar = Bar(0);
            let xx = bar.bazzzzz;
        }
        ",
        expect![[r#"
            43..46 'bar': ref<function, Bar, read_write>
            49..55 'Bar(0)': Bar
            53..54 '0': integer
            65..67 'xx': [error]
            70..73 'bar': ref<function, Bar, read_write>
            70..81 'bar.bazzzzz': ref<function, [error], read_write>
            70..81 'bar.bazzzzz': no such field `bazzzzz` on type `ref<function, Bar, read_write>`
        "#]],
    );
}

#[test]
fn no_such_field_on_struct_ptr() {
    check_infer(
        "
        struct Bar { baz: u32 }
        fn foo() {
            var bar = Bar(0);
            let bar_ptr = &bar;
            let x = bar_ptr.bazzz;
        }
        ",
        expect![[r#"
            43..46 'bar': ref<function, Bar, read_write>
            49..55 'Bar(0)': Bar
            53..54 '0': integer
            65..72 'bar_ptr': ptr<function, Bar, read_write>
            75..79 '&bar': ptr<function, Bar, read_write>
            76..79 'bar': ref<function, Bar, read_write>
            89..90 'x': [error]
            93..100 'bar_ptr': ptr<function, Bar, read_write>
            93..106 'bar_ptr.bazzz': ref<function, [error], read_write>
            93..106 'bar_ptr.bazzz': no such field `bazzz` on type `ptr<function, Bar, read_write>`
        "#]],
    );
}

#[test]
fn store_type_must_be_storable() {
    check_infer(
        "
        fn foo() {
            var bar = 1;
            var bar_ptr = &bar;
        }
        ",
        expect![[r#"
            19..22 'bar': ref<function, i32, read_write>
            25..26 '1': integer
            36..43 'bar_ptr': ref<function, [error], read_write>
            46..50 '&bar': ptr<function, i32, read_write>
            47..50 'bar': ref<function, i32, read_write>
            46..50 '&bar': expected storable type but got `ptr<function, i32, read_write>`
        "#]],
    );
}

#[test]
fn no_such_field_on_struct() {
    check_infer(
        "
        struct Bar { baz: u32 }
        fn foo() {
            let xx = Bar(0).bazzzzz;
        }
        ",
        expect![[r#"
            43..45 'xx': [error]
            48..54 'Bar(0)': Bar
            48..62 'Bar(0).bazzzzz': [error]
            52..53 '0': integer
            48..62 'Bar(0).bazzzzz': no such field `bazzzzz` on type `Bar`
        "#]],
    );
}

#[test]
fn no_such_field_on_vec() {
    check_infer(
        "
        fn foo() {
            let xyz = vec2(0, 0).xyz;
        }
        ",
        expect![[r#"
            19..22 'xyz': [error]
            25..35 'vec2(0, 0)': vec2<integer>
            25..39 'vec2(0, 0).xyz': [error]
            30..31 '0': integer
            33..34 '0': integer
            25..39 'vec2(0, 0).xyz': no such field `xyz` on type `vec2<integer>`
        "#]],
    );
}

#[test]
fn no_such_field_on_vec_ref() {
    check_infer(
        "
        fn foo() {
            var v = vec2(0, 0);
            let xyz = v.xyz;
        }
        ",
        expect![[r#"
            19..20 'v': ref<function, vec2<i32>, read_write>
            23..33 'vec2(0, 0)': vec2<integer>
            28..29 '0': integer
            31..32 '0': integer
            43..46 'xyz': [error]
            49..50 'v': ref<function, vec2<i32>, read_write>
            49..54 'v.xyz': [error]
            49..54 'v.xyz': no such field `xyz` on type `ref<function, vec2<i32>, read_write>`
        "#]],
    );
}

#[test]
fn no_such_field_on_vec_ptr() {
    check_infer(
        "
        fn foo() {
            var v = vec2(0, 0);
            let v_ptr = &v;
            let xyz = v_ptr.xyz;
        }
        ",
        expect![[r#"
            19..20 'v': ref<function, vec2<i32>, read_write>
            23..33 'vec2(0, 0)': vec2<integer>
            28..29 '0': integer
            31..32 '0': integer
            43..48 'v_ptr': ptr<function, vec2<i32>, read_write>
            51..53 '&v': ptr<function, vec2<i32>, read_write>
            52..53 'v': ref<function, vec2<i32>, read_write>
            63..66 'xyz': [error]
            69..74 'v_ptr': ptr<function, vec2<i32>, read_write>
            69..78 'v_ptr.xyz': [error]
            69..78 'v_ptr.xyz': no such field `xyz` on type `ptr<function, vec2<i32>, read_write>`
        "#]],
    );
}

#[test]
fn address_of_not_reference() {
    check_infer(
        "
        fn foo() {
            let x = 1;
            let x_ptr = &x;
        }
        ",
        expect![[r#"
            19..20 'x': i32
            23..24 '1': integer
            34..39 'x_ptr': [error]
            42..44 '&x': [error]
            43..44 'x': i32
            [EditionedFileId(Id(300))] WgslError { expression: Idx::<Expression>(1), message: "cannot use unary operator `&` on type `i32`" } in Body
        "#]],
    );
}

#[test]
fn component_reference_from_a_composite_reference() {
    // From example in spec: <https://www.w3.org/TR/WGSL/#example-5aaac12b>
    check_infer(
        "
struct S {
    age: i32,
    weight: f32
}
var<private> person: S;
// Elsewhere, 'person' denotes the reference to the memory underlying the variable,
// and will have type ref<private,S,read_write>.

fn f() {
    var uv: vec2<f32>;
    // For the remainder of this function body, 'uv' denotes the reference
    // to the memory underlying the variable, and will have type
    // ref<function,vec2<f32>,read_write>.

    // Evaluate the left-hand side of the assignment:
    //   Evaluate 'uv.x' to yield a reference:
    //   1. First evaluate 'uv', yielding a reference to the memory for
    //      the 'uv' variable. The result has type ref<function,vec2<f32>,read_write>.
    //   2. Then apply the '.x' vector access phrase, yielding a reference to
    //      the memory for the first component of the vector pointed at by the
    //      reference value from the previous step.
    //      The result has type ref<function,f32,read_write>.
    // Evaluating the right-hand side of the assignment yields the f32 value 1.0.
    // Store the f32 value 1.0 into the storage memory locations referenced by uv.x.
    uv.x = 1.0;

    // Evaluate the left-hand side of the assignment:
    //   Evaluate 'uv[1]' to yield a reference:
    //   1. First evaluate 'uv', yielding a reference to the memory for
    //      the 'uv' variable. The result has type ref<function,vec2<f32>,read_write>.
    //   2. Then apply the '[1]' array index phrase, yielding a reference to
    //      the memory for second component of the vector referenced from
    //      the previous step.  The result has type ref<function,f32,read_write>.
    // Evaluating the right-hand side of the assignment yields the f32 value 2.0.
    // Store the f32 value 2.0 into the storage memory locations referenced by uv[1].
    uv[1] = 2.0;

    var m: mat3x2<f32>;
    // When evaluating 'm[2]':
    // 1. First evaluate 'm', yielding a reference to the memory for
    //    the 'm' variable. The result has type ref<function,mat3x2<f32>,read_write>.
    // 2. Then apply the '[2]' array index phrase, yielding a reference to
    //    the memory for the third column vector pointed at by the reference
    //    value from the previous step.
    //    Therefore the 'm[2]' expression has type ref<function,vec2<f32>,read_write>.
    // The 'let' declaration is for type vec2<f32>, so the declaration
    // statement requires the initializer to be of type vec2<f32>.
    // The Load Rule applies (because no other type rule can apply), and
    // the evaluation of the initializer yields the vec2<f32> value loaded
    // from the memory locations referenced by 'm[2]' at the time the declaration
    // is executed.
    let p_m_col2: vec2<f32> = m[2];

    var A: array<i32,5>;
    // When evaluating 'A[4]'
    // 1. First evaluate 'A', yielding a reference to the memory for
    //    the 'A' variable. The result has type ref<function,array<i32,5>,read_write>.
    // 2. Then apply the '[4]' array index phrase, yielding a reference to
    //    the memory for the fifth element of the array referenced by
    //    the reference value from the previous step.
    //    The result value has type ref<function,i32,read_write>.
    // The let-declaration requires the right-hand-side to be of type i32.
    // The Load Rule applies (because no other type rule can apply), and
    // the evaluation of the initializer yields the i32 value loaded from
    // the memory locations referenced by 'A[4]' at the time the declaration
    // is executed.
    let A_4_value: i32 = A[4];

    // When evaluating 'person.weight'
    // 1. First evaluate 'person', yielding a reference to the memory for
    //    the 'person' variable declared at module scope.
    //    The result has type ref<private,S,read_write>.
    // 2. Then apply the '.weight' member access phrase, yielding a reference to
    //    the memory for the second member of the memory referenced by
    //    the reference value from the previous step.
    //    The result has type ref<private,f32,read_write>.
    // The let-declaration requires the right-hand-side to be of type f32.
    // The Load Rule applies (because no other type rule can apply), and
    // the evaluation of the initializer yields the f32 value loaded from
    // the memory locations referenced by 'person.weight' at the time the
    // declaration is executed.
    let person_weight: f32 = person.weight;

    // Alternatively, references can also be formed from pointers using
    // the same syntax.

    let uv_ptr = &uv;
    // For the remainder of this function body, 'uv_ptr' denotes a pointer
    // to the memory underlying 'uv', and will have the type
    // ptr<function,vec2<f32>,read_write>.

    // Evaluate the left-hand side of the assignment:
    //   Evaluate '*uv_ptr' to yield a reference:
    //   1. First evaluate 'uv_ptr', yielding a pointer to the memory for
    //      the 'uv' variable. The result has type ptr<function,vec2<f32>,read_write>.
    //   2. Then apply the indirection expression operator, yielding a
    //      reference to memory for 'uv'.
    // Evaluating the right-hand side of the assignment yields the vec2<f32> value (1.0, 2.0).
    // Store the value (1.0, 2.0) into the storage memory locations referenced by uv.
    *uv_ptr = vec2f(1.0, 2.0);

    // Evaluate the left-hand side of the assignment:
    //   Evaluate 'uv_ptr.x' to yield a reference:
    //   1. First evaluate 'uv_ptr', yielding a pointer to the memory for
    //      the 'uv' variable. The result has type ptr<function,vec2<f32>,read_write>.
    //   2. Then apply the '.x' vector access phrase, yielding a reference to
    //      the memory for the first component of the vector pointed at by the
    //      reference value from the previous step.
    //      The result has type ref<function,f32,read_write>.
    // Evaluating the right-hand side of the assignment yields the f32 value 1.0.
    // Store the f32 value 1.0 into the storage memory locations referenced by uv.x.
    uv_ptr.x = 1.0;

    // Evaluate the left-hand side of the assignment:
    //   Evaluate 'uv_ptr[1]' to yield a reference:
    //   1. First evaluate 'uv_ptr', yielding a pointer to the memory for
    //      the 'uv' variable. The result has type ptr<function,vec2<f32>,read_write>.
    //   2. Then apply the '[1]' array index phrase, yielding a reference to
    //      the memory for second component of the vector referenced from
    //      the previous step.  The result has type ref<function,f32,read_write>.
    // Evaluating the right-hand side of the assignment yields the f32 value 2.0.
    // Store the f32 value 2.0 into the storage memory locations referenced by uv[1].
    uv_ptr[1] = 2.0;

    let m_ptr = &m;
    // When evaluating 'm_ptr[2]':
    // 1. First evaluate 'm_ptr', yielding a pointer to the memory for
    //    the 'm' variable. The result has type ptr<function,mat3x2<f32>,read_write>.
    // 2. Then apply the '[2]' array index phrase, yielding a reference to
    //    the memory for the third column vector pointed at by the reference
    //    value from the previous step.
    //    Therefore the 'm[2]' expression has type ref<function,vec2<f32>,read_write>.
    // The 'let' declaration is for type vec2<f32>, so the declaration
    // statement requires the initializer to be of type vec2<f32>.
    // The Load Rule applies (because no other type rule can apply), and
    // the evaluation of the initializer yields the vec2<f32> value loaded
    // from the memory locations referenced by 'm[2]' at the time the declaration
    // is executed.
    let p_m_col2: vec2<f32> = m_ptr[2];

    let A_ptr = &A;
    // When evaluating 'A[4]'
    // 1. First evaluate 'A', yielding a pointer to the memory for
    //    the 'A' variable. The result has type ptr<function,array<i32,5>,read_write>.
    // 2. Then apply the '[4]' array index phrase, yielding a reference to
    //    the memory for the fifth element of the array referenced by
    //    the reference value from the previous step.
    //    The result value has type ref<function,i32,read_write>.
    // The let-declaration requires the right-hand-side to be of type i32.
    // The Load Rule applies (because no other type rule can apply), and
    // the evaluation of the initializer yields the i32 value loaded from
    // the memory locations referenced by 'A[4]' at the time the declaration
    // is executed.
    let A_4_value: i32 = A_ptr[4];

    let person_ptr = &person;
    // When evaluating 'person.weight'
    // 1. First evaluate 'person_ptr', yielding a pointer to the memory for
    //    the 'person' variable declared at module scope.
    //    The result has type ptr<private,S,read_write>.
    // 2. Then apply the '.weight' member access phrase, yielding a reference to
    //    the memory for the second member of the memory referenced by
    //    the reference value from the previous step.
    //    The result has type ref<private,f32,read_write>.
    // The let-declaration requires the right-hand-side to be of type f32.
    // The Load Rule applies (because no other type rule can apply), and
    // the evaluation of the initializer yields the f32 value loaded from
    // the memory locations referenced by 'person.weight' at the time the
    // declaration is executed.
    let person_weight: f32 = person_ptr.weight;
}
",
        expect![[r#"
            56..62 'person': ref<private, S, read_write>
            218..220 'uv': ref<function, vec2<f32>, read_write>
            1119..1121 'uv': ref<function, vec2<f32>, read_write>
            1119..1123 'uv.x': ref<function, f32, read_write>
            1126..1129 '1.0': float
            1798..1800 'uv': ref<function, vec2<f32>, read_write>
            1798..1803 'uv[1]': ref<function, f32, read_write>
            1801..1802 '1': integer
            1806..1809 '2.0': float
            1820..1821 'm': ref<function, mat3x2<f32>, read_write>
            2697..2705 'p_m_col2': vec2<f32>
            2719..2720 'm': ref<function, mat3x2<f32>, read_write>
            2719..2723 'm[2]': ref<function, vec2<f32>, read_write>
            2721..2722 '2': integer
            2734..2735 'A': ref<function, array<i32, 5>, read_write>
            3529..3538 'A_4_value': i32
            3546..3547 'A': ref<function, array<i32, 5>, read_write>
            3546..3550 'A[4]': ref<function, i32, read_write>
            3548..3549 '4': integer
            4382..4395 'person_weight': f32
            4403..4409 'person': ref<private, S, read_write>
            4403..4416 'person.weight': ref<private, f32, read_write>
            4524..4530 'uv_ptr': ptr<function, vec2<f32>, read_write>
            4533..4536 '&uv': ptr<function, vec2<f32>, read_write>
            4534..4536 'uv': ref<function, vec2<f32>, read_write>
            5281..5288 '*uv_ptr': ref<function, vec2<f32>, read_write>
            5282..5288 'uv_ptr': ptr<function, vec2<f32>, read_write>
            5291..5306 'vec2f(1.0, 2.0)': vec2<f32>
            5297..5300 '1.0': float
            5302..5305 '2.0': float
            6017..6023 'uv_ptr': ptr<function, vec2<f32>, read_write>
            6017..6025 'uv_ptr.x': ref<function, f32, read_write>
            6028..6031 '1.0': float
            6706..6712 'uv_ptr': ptr<function, vec2<f32>, read_write>
            6706..6715 'uv_ptr[1]': ref<function, f32, read_write>
            6713..6714 '1': integer
            6718..6721 '2.0': float
            6732..6737 'm_ptr': ptr<function, mat3x2<f32>, read_write>
            6740..6742 '&m': ptr<function, mat3x2<f32>, read_write>
            6741..6742 'm': ref<function, mat3x2<f32>, read_write>
            7611..7619 'p_m_col2': vec2<f32>
            7633..7638 'm_ptr': ptr<function, mat3x2<f32>, read_write>
            7633..7641 'm_ptr[2]': ref<function, vec2<f32>, read_write>
            7639..7640 '2': integer
            7652..7657 'A_ptr': ptr<function, array<i32, 5>, read_write>
            7660..7662 '&A': ptr<function, array<i32, 5>, read_write>
            7661..7662 'A': ref<function, array<i32, 5>, read_write>
            8440..8449 'A_4_value': i32
            8457..8462 'A_ptr': ptr<function, array<i32, 5>, read_write>
            8457..8465 'A_ptr[4]': ref<function, i32, read_write>
            8463..8464 '4': integer
            8476..8486 'person_ptr': ptr<private, S, read_write>
            8489..8496 '&person': ptr<private, S, read_write>
            8490..8496 'person': ref<private, S, read_write>
            9329..9342 'person_weight': f32
            9350..9360 'person_ptr': ptr<private, S, read_write>
            9350..9367 'person...weight': ref<private, f32, read_write>
        "#]],
    );
}

#[test]
fn vec_xy_is_not_ref() {
    check_infer(
        "
        fn foo() {
            var v = vec2(1, 2);
            v.xy = v.yx;
        }
        ",
        expect![[r#"
            19..20 'v': ref<function, vec2<i32>, read_write>
            23..33 'vec2(1, 2)': vec2<integer>
            28..29 '1': integer
            31..32 '2': integer
            39..40 'v': ref<function, vec2<i32>, read_write>
            39..43 'v.xy': vec2<i32>
            46..47 'v': ref<function, vec2<i32>, read_write>
            46..50 'v.yx': vec2<i32>
            39..43 'v.xy': cannot assign to non-reference `vec2<i32>`
        "#]],
    );
}

#[test]
fn struct_constructor_is_empty() {
    check_infer(
        "
        struct S { u: u32, a: array<f32, 3> };

        fn foo() {
            let s = S();
        }
        ",
        expect![[r#"
            59..60 's': S
            63..66 'S()': S
        "#]],
    );
}

#[test]
fn struct_constructor_is_correct() {
    check_infer(
        "
        struct S { u: u32, a: array<f32, 3> };

        fn foo() {
            let s = S(1u, array<f32, 3>(1.0, 2.0, 3.0));
        }
        ",
        expect![[r#"
            59..60 's': S
            63..98 'S(1u, ... 3.0))': S
            65..67 '1u': u32
            69..97 'array<..., 3.0)': array<f32, 3>
            83..86 '1.0': float
            88..91 '2.0': float
            93..96 '3.0': float
        "#]],
    );
}

#[test]
fn struct_constructor_unrefs() {
    check_infer(
        "
        struct S { u: u32, a: array<f32, 3> };

        fn foo() {
            var u = 1u;
            var a = array<f32, 3>(1.0, 2.0, 3.0);
            let s = S(u, a);
        }
        ",
        expect![[r#"
            59..60 'u': ref<function, u32, read_write>
            63..65 '1u': u32
            75..76 'a': ref<function, array<f32, 3>, read_write>
            79..107 'array<..., 3.0)': array<f32, 3>
            93..96 '1.0': float
            98..101 '2.0': float
            103..106 '3.0': float
            117..118 's': S
            121..128 'S(u, a)': S
            123..124 'u': ref<function, u32, read_write>
            126..127 'a': ref<function, array<f32, 3>, read_write>
        "#]],
    );
}

#[test]
fn struct_constructor_not_enough_args() {
    check_infer(
        "
        struct S { u: u32, a: array<f32, 3> };

        fn foo() {
            let s = S(1u);
        }
        ",
        expect![[r#"
            59..60 's': [error]
            63..68 'S(1u)': [error]
            65..67 '1u': u32
            63..68 'S(1u)': expected `2` arguments, but received `1`
        "#]],
    );
}

#[test]
fn struct_constructor_incorrect_types() {
    check_infer(
        "
        struct S { u: u32, a: array<f32, 3> };

        fn foo() {
            let s = S(1.0f, vec3f(1.0, 2.0, 3.0));
        }
        ",
        expect![[r#"
            59..60 's': [error]
            63..92 'S(1.0f... 3.0))': [error]
            65..69 '1.0f': f32
            71..91 'vec3f(..., 3.0)': vec3<f32>
            77..80 '1.0': float
            82..85 '2.0': float
            87..90 '3.0': float
            65..69 '1.0f': expected u32 but got f32
            71..91 'vec3f(..., 3.0)': expected array<f32, 3> but got vec3<f32>
        "#]],
    );
}

#[test]
fn const_array() {
    check_infer(
        "
        const a: array<f32, 1> = array(1);
        const b = array(1,2,3);
        ",
        expect![[r#"
            6..7 'a': array<f32, 1>
            25..33 'array(1)': array<integer, 1>
            31..32 '1': integer
            41..42 'b': array<integer, 3>
            45..57 'array(1,2,3)': array<integer, 3>
            51..52 '1': integer
            53..54 '2': integer
            55..56 '3': integer
        "#]],
    );
}

#[test]
fn const_vec() {
    check_infer(
        "
        const a: vec3<u32> = vec3(1);
        const b = vec2f();
        const c = vec2();
        ",
        expect![[r#"
            6..7 'a': vec3<u32>
            21..28 'vec3(1)': vec3<integer>
            26..27 '1': integer
            36..37 'b': vec2<f32>
            40..47 'vec2f()': vec2<f32>
            55..56 'c': vec2<integer>
            59..65 'vec2()': vec2<integer>
        "#]],
    );
}

#[test]
fn const_array_of_vec() {
    check_infer(
        "
        const pos = array(vec2(1.0,  1.0), vec2(1.0, -1.0));
        const pos_explicit = array<vec2f, 1>(vec2(-1.0, -1.0));
        ",
        expect![[r#"
            6..9 'pos': array<vec2<float>, 2>
            12..51 'array(...-1.0))': array<vec2<float>, 2>
            18..33 'vec2(1.0,  1.0)': vec2<float>
            23..26 '1.0': float
            29..32 '1.0': float
            35..50 'vec2(1.0, -1.0)': vec2<float>
            40..43 '1.0': float
            45..49 '-1.0': float
            46..49 '1.0': float
            59..71 'pos_explicit': array<vec2<f32>, 1>
            74..107 'array<...-1.0))': array<vec2<f32>, 1>
            90..106 'vec2(-... -1.0)': vec2<float>
            95..99 '-1.0': float
            96..99 '1.0': float
            101..105 '-1.0': float
            102..105 '1.0': float
        "#]],
    );
}

#[test]
fn const_u32_as_array_size() {
    check_infer(
        "
        const maxLayers = 12u;
        var layers: array<f32, maxLayers>;
        ",
        expect![[r#"
            6..15 'maxLayers': u32
            18..21 '12u': u32
            27..33 'layers': ref<handle, [error], read>
            46..55 'maxLayers': unexpected template argument, expected a `u32` or a `i32` greater than `0`
        "#]],
    );
}

#[test]
fn multiply_with_minus_one() {
    check_infer(
        r#"
    const x: i32 = 1;
    const y = x * -1;
        "#,
        expect![[r#"
            6..7 'x': i32
            15..16 '1': integer
            24..25 'y': i32
            28..29 'x': i32
            28..34 'x * -1': i32
            32..34 '-1': integer
            33..34 '1': integer
        "#]],
    );
}

#[test]
fn var_array() {
    check_infer(
        "
        @group(0) @binding(0) var<storage, read_write> data: array<f32>;
        ",
        expect![[r#"
            47..51 'data': ref<storage, array<f32>, read_write>
        "#]],
    );
}

#[test]
fn break_if_bool() {
    check_infer(
        "
        fn foo() {
            let a = 3;
            loop { continuing { break if a > 2; } }
        }
        ",
        expect![[r#"
            19..20 'a': i32
            23..24 '3': integer
            59..60 'a': i32
            59..64 'a > 2': bool
            63..64 '2': integer
        "#]],
    );
}

#[test]
fn abstract_number_for_const() {
    check_infer(
        "
const some_integer = 1;
const some_i32: i32 = 1;
        ",
        expect![[r#"
            6..18 'some_integer': integer
            21..22 '1': integer
            30..38 'some_i32': i32
            46..47 '1': integer
        "#]],
    );
}

#[test]
fn assign_abstract_number() {
    check_infer(
        "
var i32_from_type : i32 = 3;

fn main() {
let some_i32 = 2;
let some_u32: u32 = 2;
var i32_from_type : i32 = 3;
var f32_promotion : f32 = 5;
}
        ",
        expect![[r#"
            4..17 'i32_from_type': ref<handle, i32, read>
            26..27 '3': integer
            46..54 'some_i32': i32
            57..58 '2': integer
            64..72 'some_u32': u32
            80..81 '2': integer
            87..100 'i32_from_type': ref<function, i32, read_write>
            109..110 '3': integer
            116..129 'f32_promotion': ref<function, f32, read_write>
            138..139 '5': integer
        "#]],
    );
}

#[test]
fn negate_abstract_number() {
    check_infer(
        "
const a = -4;
const b: f32 = -3.5;
        ",
        expect![[r#"
            6..7 'a': integer
            10..12 '-4': integer
            11..12 '4': integer
            20..21 'b': f32
            29..33 '-3.5': float
            30..33 '3.5': float
        "#]],
    );
}

#[test]
fn add_abstract_integers() {
    check_infer(
        "
fn main() {
var u32_expr1 = 6 + 1u;
var u32_expr2 = 1u + (1 + 2);
}
    ",
        expect![[r#"
            16..25 'u32_expr1': ref<function, u32, read_write>
            28..29 '6': integer
            28..34 '6 + 1u': u32
            32..34 '1u': u32
            40..49 'u32_expr2': ref<function, u32, read_write>
            52..54 '1u': u32
            52..64 '1u + (1 + 2)': u32
            58..59 '1': integer
            58..63 '1 + 2': integer
            62..63 '2': integer
        "#]],
    );
}

#[test]
fn add_abstract_floats() {
    check_infer(
        "
fn main() {
let f32_promotion1 = 1.0 + 2 + 3;
let f32_promotion2 = 2 + 1.0 + 3;
let f32_promotion3 = 1f + ((2 + 3) + 4);
let f32_promotion4 = ((2 + (3 + 1f)) + 4);
}
    ",
        expect![[r#"
            16..30 'f32_promotion1': f32
            33..36 '1.0': float
            33..40 '1.0 + 2': float
            33..44 '1.0 + 2 + 3': float
            39..40 '2': integer
            43..44 '3': integer
            50..64 'f32_promotion2': f32
            67..68 '2': integer
            67..74 '2 + 1.0': float
            67..78 '2 + 1.0 + 3': float
            71..74 '1.0': float
            77..78 '3': integer
            84..98 'f32_promotion3': f32
            101..103 '1f': f32
            101..119 '1f + (...) + 4)': f32
            107..118 '(2 + 3) + 4': integer
            108..109 '2': integer
            108..113 '2 + 3': integer
            112..113 '3': integer
            117..118 '4': integer
            125..139 'f32_promotion4': f32
            143..161 '(2 + (...)) + 4': f32
            144..145 '2': integer
            144..156 '2 + (3 + 1f)': f32
            149..150 '3': integer
            149..155 '3 + 1f': f32
            153..155 '1f': f32
            160..161 '4': integer
        "#]],
    );
}

#[test]
fn call_with_abstract_numbers() {
    check_infer(
        "
fn main() {
let i32_clamp = clamp(1, -5, 5);
let u32_clamp = clamp(5, 0, 1u);
let f32_clamp = clamp(0, 1f, 1);
}
    ",
        expect![[r#"
            16..25 'i32_clamp': i32
            28..43 'clamp(1, -5, 5)': integer
            34..35 '1': integer
            37..39 '-5': integer
            38..39 '5': integer
            41..42 '5': integer
            49..58 'u32_clamp': u32
            61..76 'clamp(5, 0, 1u)': u32
            67..68 '5': integer
            70..71 '0': integer
            73..75 '1u': u32
            82..91 'f32_clamp': f32
            94..109 'clamp(0, 1f, 1)': f32
            100..101 '0': integer
            103..105 '1f': f32
            107..108 '1': integer
        "#]],
    );
}

#[test]
fn call_user_defined_with_abstract_numbers() {
    check_infer(
        "
fn make_one(x: f32) -> u32 {
  return 1u;
}

fn main() {
    let a = make_one(0.333);
}


",
        expect![[r#"
            12..13 'x': f32
            38..40 '1u': u32
            65..66 'a': u32
            69..84 'make_one(0.333)': u32
            78..83 '0.333': float
        "#]],
    );
}

#[test]
fn vec_constructors() {
    check_infer(
        "
const a = vec3(1f, 2f, 3f);
fn main() {
let b = vec4(vec3f(1f), 1f);
}
    ",
        expect![[r#"
            6..7 'a': vec3<f32>
            10..26 'vec3(1...f, 3f)': vec3<f32>
            15..17 '1f': f32
            19..21 '2f': f32
            23..25 '3f': f32
            44..45 'b': vec4<f32>
            48..67 'vec4(v...), 1f)': vec4<f32>
            53..62 'vec3f(1f)': vec3<f32>
            59..61 '1f': f32
            64..66 '1f': f32
        "#]],
    );
}

#[test]
fn texture_storage_2d_template() {
    check_infer(
        "
var framebuffer : texture_storage_2d<rgba16float, write>;
    ",
        expect![[r#"
            4..15 'framebuffer': ref<handle, texture_storage_2d<rgba16float,write>, read>
        "#]],
    );
}

#[test]
fn global_assert_statement_correct() {
    check_infer(
        "
        const a = 29;
        const_assert 27 < a;
    ",
        expect![[r#"
            6..7 'a': integer
            10..12 '29': integer
            27..29 '27': integer
            27..33 '27 < a': bool
            32..33 'a': integer
        "#]],
    );
}

#[test]
fn global_assert_statement_wrong() {
    check_infer(
        "
        const a = 29;
        const_assert 27 + a;
    ",
        expect![[r#"
            6..7 'a': integer
            10..12 '29': integer
            27..29 '27': integer
            27..33 '27 + a': integer
            32..33 'a': integer
            27..33 '27 + a': expected bool but got integer
        "#]],
    );
}

#[test]
fn global_var_function_address_space_error() {
    check_infer(
        "var<function> not_allowed_at_module_level: u32;",
        expect![[r#"
            14..41 'not_al..._level': ref<function, u32, read_write>
            4..12 'function': unexpected template argument `function`
        "#]],
    );
}

#[test]
fn no_crash_on_hex_int() {
    // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/826
    check_infer(
        "
fn f() {
    let i2 = 0u;
    let p0 = (i2 >> 0u) & 0xf;
}
",
        expect![[r#"
            17..19 'i2': u32
            22..24 '0u': u32
            34..36 'p0': u32
            39..55 '(i2 >>... & 0xf': u32
            40..42 'i2': u32
            40..48 'i2 >> 0u': u32
            46..48 '0u': u32
            52..55 '0xf': integer
        "#]],
    );
}

#[test]
fn array_index_is_i32() {
    check_infer(
        "
        const index = 1i;
        const arr = array<i32, 3>(1, 2, 3);
        const a = arr[index];
        ",
        expect![[r#"
            6..11 'index': i32
            14..16 '1i': i32
            24..27 'arr': array<i32, 3>
            30..52 'array<... 2, 3)': array<i32, 3>
            44..45 '1': integer
            47..48 '2': integer
            50..51 '3': integer
            60..61 'a': i32
            64..67 'arr': array<i32, 3>
            64..74 'arr[index]': i32
            68..73 'index': i32
        "#]],
    );
}

#[test]
fn array_index_is_u32() {
    check_infer(
        "
        const index = 1u;
        const arr = array<i32, 3>(1, 2, 3);
        const a = arr[index];
        ",
        expect![[r#"
            6..11 'index': u32
            14..16 '1u': u32
            24..27 'arr': array<i32, 3>
            30..52 'array<... 2, 3)': array<i32, 3>
            44..45 '1': integer
            47..48 '2': integer
            50..51 '3': integer
            60..61 'a': i32
            64..67 'arr': array<i32, 3>
            64..74 'arr[index]': i32
            68..73 'index': u32
        "#]],
    );
}

#[test]
fn array_index_is_abstract_int() {
    check_infer(
        "
        const index = 1;
        const arr = array<i32, 3>(1, 2, 3);
        const a = arr[index];
        ",
        expect![[r#"
            6..11 'index': integer
            14..15 '1': integer
            23..26 'arr': array<i32, 3>
            29..51 'array<... 2, 3)': array<i32, 3>
            43..44 '1': integer
            46..47 '2': integer
            49..50 '3': integer
            59..60 'a': i32
            63..66 'arr': array<i32, 3>
            63..73 'arr[index]': i32
            67..72 'index': integer
        "#]],
    );
}

#[test]
fn array_index_is_not_f32() {
    check_infer(
        "
        const index = 1.0f;
        const arr = array<i32, 3>(1, 2, 3);
        const a = arr[index];
        ",
        expect![[r#"
            6..11 'index': f32
            14..18 '1.0f': f32
            26..29 'arr': array<i32, 3>
            32..54 'array<... 2, 3)': array<i32, 3>
            46..47 '1': integer
            49..50 '2': integer
            52..53 '3': integer
            62..63 'a': i32
            66..69 'arr': array<i32, 3>
            66..76 'arr[index]': i32
            70..75 'index': f32
            70..75 'index': expected i32 or u32 but got f32
        "#]],
    );
}

#[test]
fn array_index_is_ref_i32() {
    check_infer(
        "
        fn test(arr: array<i32>) {
            var index = 1i;
            const a = arr[index];
        }
        ",
        expect![[r#"
            8..11 'arr': array<i32>
            35..40 'index': ref<function, i32, read_write>
            43..45 '1i': i32
            57..58 'a': i32
            61..64 'arr': array<i32>
            61..71 'arr[index]': i32
            65..70 'index': ref<function, i32, read_write>
        "#]],
    );
}

#[test]
fn array_index_is_not_ref_f32() {
    check_infer(
        "
        fn test(arr: array<i32>) {
            var index = 1.0f;
            const a = arr[index];
        }
        ",
        expect![[r#"
            8..11 'arr': array<i32>
            35..40 'index': ref<function, f32, read_write>
            43..47 '1.0f': f32
            59..60 'a': i32
            63..66 'arr': array<i32>
            63..73 'arr[index]': i32
            67..72 'index': ref<function, f32, read_write>
            67..72 'index': expected i32 or u32 but got f32
        "#]],
    );
}

#[test]
fn array_index_is_not_abstract_float() {
    check_infer(
        "
        const index = 1.0;
        const arr = array<i32, 3>(1, 2, 3);
        const a = arr[index];
        ",
        expect![[r#"
            6..11 'index': float
            14..17 '1.0': float
            25..28 'arr': array<i32, 3>
            31..53 'array<... 2, 3)': array<i32, 3>
            45..46 '1': integer
            48..49 '2': integer
            51..52 '3': integer
            61..62 'a': i32
            65..68 'arr': array<i32, 3>
            65..75 'arr[index]': i32
            69..74 'index': float
            69..74 'index': expected i32 or u32 but got float
        "#]],
    );
}

#[test]
fn array_index_is_not_bool() {
    check_infer(
        "
        const index = true;
        const arr = array<i32, 3>(1, 2, 3);
        const a = arr[index];
        ",
        expect![[r#"
            6..11 'index': bool
            14..18 'true': bool
            26..29 'arr': array<i32, 3>
            32..54 'array<... 2, 3)': array<i32, 3>
            46..47 '1': integer
            49..50 '2': integer
            52..53 '3': integer
            62..63 'a': i32
            66..69 'arr': array<i32, 3>
            66..76 'arr[index]': i32
            70..75 'index': bool
            70..75 'index': expected i32 or u32 but got bool
        "#]],
    );
}

#[test]
fn vec_index_is_int() {
    check_infer(
        "
        const index = 1i;
        const vec = vec3<f32>(1.0, 2.0, 3.0);
        const a = vec[index];
        ",
        expect![[r#"
            6..11 'index': i32
            14..16 '1i': i32
            24..27 'vec': vec3<f32>
            30..54 'vec3<f..., 3.0)': vec3<f32>
            40..43 '1.0': float
            45..48 '2.0': float
            50..53 '3.0': float
            62..63 'a': f32
            66..69 'vec': vec3<f32>
            66..76 'vec[index]': f32
            70..75 'index': i32
        "#]],
    );
}

#[test]
fn vec_index_is_not_f32() {
    check_infer(
        "
        const index = 1.0f;
        const vec = vec3<f32>(1.0, 2.0, 3.0);
        const a = vec[index];
        ",
        expect![[r#"
            6..11 'index': f32
            14..18 '1.0f': f32
            26..29 'vec': vec3<f32>
            32..56 'vec3<f..., 3.0)': vec3<f32>
            42..45 '1.0': float
            47..50 '2.0': float
            52..55 '3.0': float
            64..65 'a': f32
            68..71 'vec': vec3<f32>
            68..78 'vec[index]': f32
            72..77 'index': f32
            72..77 'index': expected i32 or u32 but got f32
        "#]],
    );
}

#[test]
fn mat_index_is_int() {
    check_infer(
        "
        const index = 1i;
        const mat = mat2x2<f32>(1.0, 2.0, 3.0, 4.0);
        const a = mat[index][0];
        ",
        expect![[r#"
            6..11 'index': i32
            14..16 '1i': i32
            24..27 'mat': mat2x2<f32>
            30..61 'mat2x2..., 4.0)': mat2x2<f32>
            42..45 '1.0': float
            47..50 '2.0': float
            52..55 '3.0': float
            57..60 '4.0': float
            69..70 'a': f32
            73..76 'mat': mat2x2<f32>
            73..83 'mat[index]': vec2<f32>
            73..86 'mat[index][0]': f32
            77..82 'index': i32
            84..85 '0': integer
        "#]],
    );
}

#[test]
fn concretize_matrix() {
    check_infer(
        "
        fn foo() {
            let x = bar(mat2x2(0, 0, 0, 0));
        }
        fn bar(baz: mat2x2<f32>) -> u32 { return 0; }
        ",
        expect![[r#"
            19..20 'x': u32
            23..46 'bar(ma...0, 0))': u32
            27..45 'mat2x2... 0, 0)': mat2x2<float>
            34..35 '0': integer
            37..38 '0': integer
            40..41 '0': integer
            43..44 '0': integer
            57..60 'baz': mat2x2<f32>
            91..92 '0': integer
        "#]],
    );
}

#[test]
fn mat_index_i_is_not_f32() {
    check_infer(
        "
        const index = 1.0f;
        const mat = mat2x2<f32>(1.0, 2.0, 3.0, 4.0);
        const a = mat[index][0];
        ",
        expect![[r#"
            6..11 'index': f32
            14..18 '1.0f': f32
            26..29 'mat': mat2x2<f32>
            32..63 'mat2x2..., 4.0)': mat2x2<f32>
            44..47 '1.0': float
            49..52 '2.0': float
            54..57 '3.0': float
            59..62 '4.0': float
            71..72 'a': f32
            75..78 'mat': mat2x2<f32>
            75..85 'mat[index]': vec2<f32>
            75..88 'mat[index][0]': f32
            79..84 'index': f32
            86..87 '0': integer
            79..84 'index': expected i32 or u32 but got f32
        "#]],
    );
}

#[test]
fn mat_index_j_is_not_f32() {
    check_infer(
        "
        const index = 1.0f;
        const mat = mat2x2<f32>(1.0, 2.0, 3.0, 4.0);
        const a = mat[0][index];
        ",
        expect![[r#"
            6..11 'index': f32
            14..18 '1.0f': f32
            26..29 'mat': mat2x2<f32>
            32..63 'mat2x2..., 4.0)': mat2x2<f32>
            44..47 '1.0': float
            49..52 '2.0': float
            54..57 '3.0': float
            59..62 '4.0': float
            71..72 'a': f32
            75..78 'mat': mat2x2<f32>
            75..81 'mat[0]': vec2<f32>
            75..88 'mat[0][index]': f32
            79..80 '0': integer
            82..87 'index': f32
            82..87 'index': expected i32 or u32 but got f32
        "#]],
    );
}

#[test]
fn naga_shader_int64() {
    check_infer_with_capabilities(
        Capabilities {
            shader_int64: true,
            ..Default::default()
        },
        "
fn foo(bar: i64, baz: u64) {}
",
        expect![[r#"
            7..10 'bar': i64
            17..20 'baz': u64
        "#]],
    );
}

#[test]
fn no_builtin_overload() {
    check_infer(
        "
        var x = 1f + mat2x2f();
        ",
        expect![[r#"
            4..5 'x': ref<handle, [error], read>
            8..10 '1f': f32
            8..22 '1f + mat2x2f()': [error]
            13..22 'mat2x2f()': mat2x2<f32>
            [EditionedFileId(Id(300))] WgslError { expression: Idx::<Expression>(2), message: "cannot use binary operator `+` with operands `f32` and `mat2x2<f32>`" } in Body
        "#]],
    );
}

#[test]
fn deref_not_a_pointer() {
    check_infer(
        "
        var x = *1f;
        ",
        expect![[r#"
            4..5 'x': ref<handle, [error], read>
            8..11 '*1f': [error]
            9..11 '1f': f32
            [EditionedFileId(Id(300))] WgslError { expression: Idx::<Expression>(0), message: "cannot use unary operator `*` on type `f32`" } in Body
        "#]],
    );
}

#[test]
fn no_constructor() {
    check_infer(
        "
        var x = vec2f(1, 2, 3);
        ",
        expect![[r#"
            4..5 'x': ref<handle, [error], read>
            8..22 'vec2f(1, 2, 3)': [error]
            14..15 '1': integer
            17..18 '2': integer
            20..21 '3': integer
            8..22 'vec2f(1, 2, 3)': no constructor for builtin `op_vec2_constructor` of type `vec2<f32>` with parameters `integer, integer, integer`
        "#]],
    );
}

#[test]
fn add_refs_and_ptrs() {
    check_infer(
        "
        struct MyData {
            a: u32,
            b: u32,
        }

        @group(0) @binding(9)
        var<storage, read_write> MyBuff: array<MyData>;

        fn MyFn(index: u32) -> u32 {
            let data = &MyBuff[index];

            var t = data.a;
            return t + data.b;
        }

        fn foo() {
            var a_ref = 1;
            var b_ref = 1;

            let a_ptr = &a_ref;
            let b_ptr = &b_ref;

            let test1 = a_ref + b_ref;
            let test2 = a_ptr + b_ptr;
            let test3 = a_ptr + b_ref;
        }
        ",
        expect![[r#"
            90..96 'MyBuff': ref<storage, array<MyData>, read_write>
            122..127 'index': u32
            151..155 'data': ptr<storage, MyData, read_write>
            158..172 '&MyBuff[index]': ptr<storage, MyData, read_write>
            159..165 'MyBuff': ref<storage, array<MyData>, read_write>
            159..172 'MyBuff[index]': ref<storage, MyData, read_write>
            166..171 'index': u32
            183..184 't': ref<function, u32, read_write>
            187..191 'data': ptr<storage, MyData, read_write>
            187..193 'data.a': ref<storage, u32, read_write>
            206..207 't': ref<function, u32, read_write>
            206..216 't + data.b': u32
            210..214 'data': ptr<storage, MyData, read_write>
            210..216 'data.b': ref<storage, u32, read_write>
            240..245 'a_ref': ref<function, i32, read_write>
            248..249 '1': integer
            259..264 'b_ref': ref<function, i32, read_write>
            267..268 '1': integer
            279..284 'a_ptr': ptr<function, i32, read_write>
            287..293 '&a_ref': ptr<function, i32, read_write>
            288..293 'a_ref': ref<function, i32, read_write>
            303..308 'b_ptr': ptr<function, i32, read_write>
            311..317 '&b_ref': ptr<function, i32, read_write>
            312..317 'b_ref': ref<function, i32, read_write>
            328..333 'test1': i32
            336..341 'a_ref': ref<function, i32, read_write>
            336..349 'a_ref + b_ref': i32
            344..349 'b_ref': ref<function, i32, read_write>
            359..364 'test2': [error]
            367..372 'a_ptr': ptr<function, i32, read_write>
            367..380 'a_ptr + b_ptr': [error]
            375..380 'b_ptr': ptr<function, i32, read_write>
            390..395 'test3': [error]
            398..403 'a_ptr': ptr<function, i32, read_write>
            398..411 'a_ptr + b_ref': [error]
            406..411 'b_ref': ref<function, i32, read_write>
            [EditionedFileId(Id(300))] WgslError { expression: Idx::<Expression>(11), message: "cannot use binary operator `+` with operands `ptr<function, i32, read_write>` and `ptr<function, i32, read_write>`" } in Body
            [EditionedFileId(Id(300))] WgslError { expression: Idx::<Expression>(14), message: "cannot use binary operator `+` with operands `ptr<function, i32, read_write>` and `i32`" } in Body
        "#]],
    );
}

#[test]
fn unexpected_return_type() {
    check_infer(
        "
        fn foo() {
            return 0;
        }
        ",
        expect![[r#"
            22..23 '0': integer
            22..23 '0': unexpected return value of type `integer` in function with no return type
        "#]],
    );
}

#[test]
fn wrong_return_type() {
    check_infer(
        "
        fn foo() -> bool {
            return 0;
        }
        ",
        expect![[r#"
            30..31 '0': integer
            30..31 '0': expected bool but got integer
        "#]],
    );
}

#[test]
fn shift_operator_inference() {
    check_infer(
        "
        fn bit_repro() {
            let x = 1 << 4;
            let y = 5;
            let z = x & y;
        }
        ",
        expect![[r#"
            25..26 'x': i32
            29..30 '1': integer
            29..35 '1 << 4': integer
            34..35 '4': integer
            45..46 'y': i32
            49..50 '5': integer
            60..61 'z': i32
            64..65 'x': i32
            64..69 'x & y': i32
            68..69 'y': i32
        "#]],
    );
}

#[test]
fn lowering_type_missing_template_arguments() {
    check_infer(
        "
        var x: mat4x4;
        const m: mat4x4 = mat2x2(0, 1, 2, 3);
        ",
        expect![[r#"
            4..5 'x': ref<handle, [error], read>
            7..13 'mat4x4': missing template arguments
            21..22 'm': [error]
            33..51 'mat2x2... 2, 3)': mat2x2<float>
            40..41 '0': integer
            43..44 '1': integer
            46..47 '2': integer
            49..50 '3': integer
            24..30 'mat4x4': missing template arguments
        "#]],
    );
}

#[test]
fn lowering_type_missing_expected_type() {
    check_infer(
        "
        var x: modf;
        ",
        expect![[r#"
            4..5 'x': ref<handle, [error], read>
            7..11 'modf': modf is not a type
        "#]],
    );
}

#[test]
fn to_wgsl_types_builtin_struct() {
    check_infer(
        "
        fn foo() {
            let x = modf(1.0);
            let y = modf(x);
        }
        ",
        expect![[r#"
            19..20 'x': __modf_result_abstract
            23..32 'modf(1.0)': __modf_result_abstract
            28..31 '1.0': float
            42..43 'y': [error]
            46..53 'modf(x)': [error]
            51..52 'x': __modf_result_abstract
            [EditionedFileId(Id(300))] WgslError { expression: Idx::<Expression>(3), message: "`modf` expects a float scalar or vector argument" } in Body
        "#]],
    );
}

#[test]
fn lower_function_as_template_argument() {
    check_infer(
        "
        fn foo() {
            let y = array<foo, 1>(1.0);
        }
        ",
        expect![[r#"
            19..20 'y': array<[error], 1>
            23..41 'array<...>(1.0)': array<[error], 1>
            37..40 '1.0': float
            37..40 '1.0': expected [error] but got float
            29..32 'foo': foo was written, write foo() instead
        "#]],
    );
}

#[test]
fn builtin_struct_not_constructible() {
    check_infer(
        "
        fn foo() {
            let y = __modf_result_abstract(1.0, 0.1);
        }
        ",
        expect![[r#"
            19..20 'y': [error]
            23..55 '__modf..., 0.1)': [error]
            46..49 '1.0': float
            51..54 '0.1': float
            23..55 '__modf..., 0.1)': `__modf_result_abstract` not found in scope
        "#]],
    );
}

#[test]
fn lower_call_uncallable_diagnostic() {
    check_infer(
        "
        fn foo() {
            let y = rgba16float();
        }
        ",
        expect![[r#"
            19..20 'y': [error]
            23..36 'rgba16float()': [error]
            23..36 'rgba16float()': expected function, but got enumerant `rgba16float`
        "#]],
    );
}

#[test]
fn not_convertible() {
    check_infer(
        "
        fn foo() {
            let x = 1i * 1.0f;
        }
        ",
        expect![[r#"
            19..20 'x': [error]
            23..25 '1i': i32
            23..32 '1i * 1.0f': [error]
            28..32 '1.0f': f32
            [EditionedFileId(Id(300))] WgslError { expression: Idx::<Expression>(2), message: "cannot use binary operator `*` with operands `i32` and `f32`" } in Body
        "#]],
    );
}

#[test]
fn sampler_comparison_no_template() {
    check_infer(
        "
        var x: sampler_comparison<wrong>;
        ",
        expect![[r#"
            4..5 'x': ref<handle, sampler_comparison, read>
            26..31 'wrong': `wrong` not found in scope
            26..31 'wrong': unexpected template argument, expected nothing
        "#]],
    );
}

#[test]
fn ptr_template_not_enumerant() {
    check_infer(
        "
        fn foo1(bar1: ptr<rgba8unorm, i32, read_write>) { }
        fn foo2(bar2: ptr<storage, 123i, read_write>) { }
        fn foo3(bar3: ptr<storage, i32, rgba8unorm>) { }
        fn foo4(bar4: ptr<storage, i32>) { }
        ",
        expect![[r#"
            8..12 'bar1': [error]
            18..28 'rgba8unorm': unexpected template argument, expected an address space
            60..64 'bar2': ptr<storage, [error], read_write>
            79..83 '123i': unexpected template argument, expected a type
            110..114 'bar3': [error]
            134..144 'rgba8unorm': unexpected template argument, expected one of: (read, read_write, write)
            159..163 'bar4': ptr<storage, i32, read>
        "#]],
    );
}

#[test]
fn small_fragment_shader() {
    check_infer(
        "
@fragment
fn main(@builtin(position) pos: vec4f) -> @location(0) vec4f {
    var x: f32;
    @if (true)
    {
        var x = 1i;
    }
    return vec4(vec3(x), 1);
}
        ",
        expect![[r#"
            37..40 'pos': vec4<f32>
            81..82 'x': ref<function, f32, read_write>
            122..123 'x': ref<function, i32, read_write>
            126..128 '1i': i32
            147..163 'vec4(v...x), 1)': vec4<i32>
            152..159 'vec3(x)': vec3<i32>
            157..158 'x': ref<function, i32, read_write>
            161..162 '1': integer
            147..163 'vec4(v...x), 1)': expected vec4<f32> but got vec4<i32>
        "#]],
    );
}

#[test]
fn override_declaration() {
    check_infer(
        "
        override LEVELS: f32 = 4.0;
        fn foo() -> f32 { return LEVELS; }
        ",
        expect![[r#"
            9..15 'LEVELS': f32
            23..26 '4.0': float
            53..59 'LEVELS': f32
        "#]],
    );
}

#[test]
fn function_call_argument_type_mismatch() {
    check_infer(
        "
fn foo(x: i32, y: u32) {
    foo(y, x);
}
        ",
        expect![[r#"
            7..8 'x': i32
            15..16 'y': u32
            29..38 'foo(y, x)': [error]
            33..34 'y': u32
            36..37 'x': i32
            33..34 'y': expected i32 but got u32
            36..37 'x': expected u32 but got i32
        "#]],
    );
}

#[test]
fn ident_override_inference() {
    check_infer(
        "
override bar: u32;
fn foo() {
    let x = bar;
}
        ",
        expect![[r#"
            9..12 'bar': u32
            38..39 'x': u32
            42..45 'bar': u32
        "#]],
    );
}

#[test]
fn var_cycle() {
    check_infer(
        "
var x = y;
var y = x;
        ",
        expect![[r#"
            [EditionedFileId(Id(300))] CyclicType { name: Name("x"), range: 0..10 } in Body
            [EditionedFileId(Id(300))] CyclicType { name: Name("y"), range: 11..21 } in Body
        "#]],
    );
}

#[test]
fn struct_cycle() {
    check_infer(
        "
struct Foo { foo: Bar }
struct Bar { foo: Foo }
fn foo() {
    let x = Foo();
}
        ",
        expect![[r#"
            67..68 'x': Foo
            71..76 'Foo()': Foo
            71..76 'Foo()': type `Foo` is not constructible
        "#]],
    );
}

// TODO https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1389
#[test]
fn function_cycle() {
    check_infer(
        "
fn foo() {
    foo();
}
        ",
        expect![[r#"
            15..20 'foo()': [error]
        "#]],
    );
}

#[test]
fn alias_cycle() {
    check_infer(
        "
alias Foo = Bar;
alias Bar = Foo;
        ",
        expect![[r#"
            [EditionedFileId(Id(300))] CyclicType { name: Name("Foo"), range: 0..16 } in Signature
            [EditionedFileId(Id(300))] CyclicType { name: Name("Bar"), range: 17..33 } in Signature
        "#]],
    );
}

#[test]
fn not_a_reference() {
    check_infer(
        "
fn foo() {
    let x = 1;
    x = 2;
    x++;
    x += 1;
}
        ",
        expect![[r#"
            19..20 'x': i32
            23..24 '1': integer
            30..31 'x': i32
            34..35 '2': integer
            41..42 'x': i32
            50..51 'x': i32
            55..56 '1': integer
            30..31 'x': cannot assign to non-reference `i32`
            41..42 'x': cannot assign to non-reference `i32`
            50..51 'x': cannot assign to non-reference `i32`
            55..56 '1': expected [error] but got i32
        "#]],
    );
}

#[test]
fn error_in_template() {
    check_infer(
        "
fn foo() {
    let y = 0;
    let x = sqrt<&y>(y);
}
        ",
        expect![[r#"
            19..20 'y': i32
            23..24 '0': integer
            34..35 'x': [error]
            38..49 'sqrt<&y>(y)': [error]
            47..48 'y': i32
            38..49 'sqrt<&y>(y)': `sqrt` not found in scope
        "#]],
    );
}

#[test]
fn construct_templated_but_argument_is_error_no_second_diagnostic() {
    check_infer(
        "
fn foo() {
    let y = &0;
    let x = vec2<f32>(y);
}
        ",
        expect![[r#"
            19..20 'y': [error]
            23..25 '&0': [error]
            24..25 '0': integer
            35..36 'x': [error]
            39..51 'vec2<f32>(y)': [error]
            49..50 'y': [error]
            [EditionedFileId(Id(300))] WgslError { expression: Idx::<Expression>(0), message: "cannot use unary operator `&` on type `AbstractInt`" } in Body
            39..51 'vec2<f32>(y)': no constructor for builtin `op_vec2_constructor` of type `vec2<f32>` with parameters `[error]`
        "#]],
    );
}

#[test]
fn construct_untemplated_but_argument_is_error_no_second_diagnostic() {
    check_infer(
        "
fn foo() {
    let y = &0;
    let x = vec2(y);
}
        ",
        expect![[r#"
            19..20 'y': [error]
            23..25 '&0': [error]
            24..25 '0': integer
            35..36 'x': [error]
            39..46 'vec2(y)': [error]
            44..45 'y': [error]
            [EditionedFileId(Id(300))] WgslError { expression: Idx::<Expression>(0), message: "cannot use unary operator `&` on type `AbstractInt`" } in Body
            39..46 'vec2(y)': no constructor for builtin `op_vec2_constructor` of type `vec2<[error]>` with parameters `[error]`
        "#]],
    );
}

#[test]
fn matrix_no_constructor() {
    check_infer(
        "
fn foo() {
    let y = mat2x2f(true);
}
        ",
        expect![[r#"
            19..20 'y': [error]
            23..36 'mat2x2f(true)': [error]
            31..35 'true': bool
            23..36 'mat2x2f(true)': no constructor for builtin `op_mat2x2_constructor` of type `mat2x2<f32>` with parameters `bool`
        "#]],
    );
}

#[test]
fn vector_no_constructor() {
    check_infer(
        "
fn foo() {
    let y = vec2(true, true, true);
}
        ",
        expect![[r#"
            19..20 'y': [error]
            23..45 'vec2(t... true)': [error]
            28..32 'true': bool
            34..38 'true': bool
            40..44 'true': bool
            23..45 'vec2(t... true)': no constructor for builtin `op_vec2_constructor` of type `vec2<[error]>` with parameters `bool, bool, bool`
        "#]],
    );
}

#[test]
fn array_zero_arguments() {
    check_infer(
        "
fn foo() {
    let y = array();
}
        ",
        expect![[r#"
            19..20 'y': array<[error]>
            23..30 'array()': array<[error]>
            23..30 'array()': type `array<[error]>` is not constructible
        "#]],
    );
}

#[test]
fn vector_zero_arguments() {
    check_infer(
        "
fn foo() {
    let y = vec2();
}
        ",
        expect![[r#"
            19..20 'y': vec2<i32>
            23..29 'vec2()': vec2<integer>
        "#]],
    );
}

#[test]
fn matrix_zero_arguments() {
    check_infer(
        "
fn foo() {
    let y = mat2x2();
}
        ",
        expect![[r#"
            19..20 'y': [error]
            23..31 'mat2x2()': [error]
            23..31 'mat2x2()': expected `1` arguments, but received `0`
        "#]],
    );
}

#[test]
fn matrix_missing_template_no_constructor() {
    check_infer(
        "
fn foo() {
    let y = mat2x2(true);
}
        ",
        expect![[r#"
            19..20 'y': [error]
            23..35 'mat2x2(true)': [error]
            30..34 'true': bool
            23..35 'mat2x2(true)': no constructor for builtin `op_mat2x2_constructor` of type `mat2x2<[error]>` with parameters `bool`
        "#]],
    );
}

#[test]
fn array_templated_not_convertible() {
    check_infer(
        "
fn foo() {
    let y = array<bool, 1>(1);
}
        ",
        expect![[r#"
            19..20 'y': array<bool, 1>
            23..40 'array<... 1>(1)': array<bool, 1>
            38..39 '1': integer
            38..39 '1': expected bool but got integer
        "#]],
    );
}

#[test]
fn array_templated_wrong_number_arguments() {
    check_infer(
        "
fn foo() {
    let y = array<i32, 1>(1, 2);
}
        ",
        expect![[r#"
            19..20 'y': array<i32, 1>
            23..42 'array<...(1, 2)': array<i32, 1>
            37..38 '1': integer
            40..41 '2': integer
            23..42 'array<...(1, 2)': expected `1` arguments, but received `2`
        "#]],
    );
}

#[test]
fn array_untemplated_not_convertible() {
    check_infer(
        "
fn foo() {
    let y = array(bool, 1);
}
        ",
        expect![[r#"
            19..20 'y': array<[error], 2>
            23..37 'array(bool, 1)': array<[error], 2>
            29..33 'bool': [error]
            35..36 '1': integer
            29..33 'bool': expected variable, but got type `bool`
            35..36 '1': expected [error] but got integer
        "#]],
    );
}

#[test]
fn array_untemplated_wrong_number_arguments() {
    check_infer(
        "
fn foo() {
    let y = array(1, 2);
}
        ",
        expect![[r#"
            19..20 'y': array<i32, 2>
            23..34 'array(1, 2)': array<integer, 2>
            29..30 '1': integer
            32..33 '2': integer
        "#]],
    );
}

#[test]
fn atomic_assignment() {
    check_infer(
        "
struct Foo {
    y: atomic<u32>,
}
var<storage, read_write> x: array<Foo>;
fn foo() {
    x[0].y = 0u;
}
        ",
        expect![[r#"
            60..61 'x': ref<storage, array<Foo>, read_write>
            90..91 'x': ref<storage, array<Foo>, read_write>
            90..94 'x[0]': ref<storage, Foo, read_write>
            90..96 'x[0].y': ref<storage, atomic<u32>, read_write>
            92..93 '0': integer
            99..101 '0u': u32
            99..101 '0u': expected atomic<u32> but got u32
        "#]],
    );
}

#[test]
fn atomic_assignment2() {
    check_infer(
        "
@fragment
fn shade_it() -> @location(0) vec4<f32> {
  _ = &buf;
  atomicStore(&buf.counter, 1u);
  return vec4<f32>();
}

struct BufferContents {
    counter: atomic<u32>,
    data: array<vec4<f32>>
}

@group(0) @binding(0) var<storage, read_write> buf: BufferContents;

        ",
        expect![[r#"
            58..62 '&buf': ptr<storage, BufferContents, read_write>
            59..62 'buf': ref<storage, BufferContents, read_write>
            66..95 'atomic...r, 1u)': [error]
            78..90 '&buf.counter': ptr<storage, atomic<u32>, read_write>
            79..82 'buf': ref<storage, BufferContents, read_write>
            79..90 'buf.counter': ref<storage, atomic<u32>, read_write>
            92..94 '1u': u32
            106..117 'vec4<f32>()': vec4<f32>
            249..252 'buf': ref<storage, BufferContents, read_write>
        "#]],
    );
}
