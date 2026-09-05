use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{CheckOptions, check, check_with_options},
};

#[test]
fn format_nonsquare_matrix_as_cols_rows() {
    // https://gpuweb.github.io/gpuweb/wgsl/#matrix-types
    // "matCxR<T> 	Matrix of C columns and R rows..."
    check(
        "
fn main() {
    let z = mat2x4<f32>(a,b,c,d,g,h,i,j);
}",
        expect![[r#"
            fn main() {
                let z = mat2x4<f32>(
                        a, b,
                        c, d,
                        g, h,
                        i, j,
                    );
            }
        "#]],
    );
}

#[test]
fn format_matrix_if_correct_number_of_args() {
    check(
        "
fn main() {
    let x = mat3x3(
        cosR,  0.0, sinR,
        0.0, 1.0, 0.0,
        -sinR, 0.0, cosR,
    );
}",
        expect![[r#"
            fn main() {
                let x = mat3x3(
                        cosR, 0.0, sinR,
                        0.0, 1.0, 0.0,
                        -sinR, 0.0, cosR,
                    );
            }
        "#]],
    );
}

#[test]
fn format_matrix_dont_if_incorrect_number_of_args_random() {
    check(
        "
fn main() {
    let x = mat3x3(
        cosR,  0.0, sinR,
        0.0, 1.0, 0.0,
        -sinR, 0.0,
    );
}",
        expect![[r#"
            fn main() {
                let x = mat3x3(cosR, 0.0, sinR, 0.0, 1.0, 0.0, -sinR, 0.0);
            }
        "#]],
    );
}

#[test]
fn format_matrix_if_forced_multiline_by_comment() {
    // I don't think this looks too bad - but there was no big discussion on how we should handle this
    check(
        "
fn main() {
    let x = mat3x3(
        cosR,
        1.0, // Breaky
        1.0,
        sinR,
        0.0, 1.0, 0.0,
        -sinR, 0.0,
    );
}",
        expect![[r#"
            fn main() {
                let x = mat3x3(
                        cosR,
                        1.0, // Breaky
                        1.0,
                        sinR, 0.0, 1.0,
                        0.0, -sinR, 0.0,
                    );
            }
        "#]],
    );
}

#[test]
fn format_matrix_if_forced_multiline_by_nesting() {
    // I don't think this looks too bad - but there was no big discussion on how we should handle this
    check(
        "
fn main() {
    let x = mat3x3(
        cosR, do_thing(
        1.0 // Breaky
        ), 1.0,
        sinR,
        0.0, 1.0, 0.0,
        -sinR, 0.0,
    );
}",
        expect![[r#"
            fn main() {
                let x = mat3x3(
                        cosR,
                        do_thing(
                            1.0, // Breaky
                        ),
                        1.0,
                        sinR, 0.0, 1.0,
                        0.0, -sinR, 0.0,
                    );
            }
        "#]],
    );
}

#[test]
fn format_matrix_break_inner_first_if_forced_multiline_by_line_length() {
    // I don't think this looks too bad - but there was no big discussion on how we should handle this
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
            let x = mat3x3(
                cosR, do_thing(aaaaaaaaaaaaaaaaaaa+bbbbbbbbbbbbbbbbbbb+cccccccccccccccc+dddddddddddddddd+eeeeeeeeeeee), 1.0,
                sinR,
                1.0, 0.0, 0.0,
                -sinR, 0.0,
            );
        }",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                let x = mat3x3(
                        cosR,
                        do_thing(
                            aaaaaaaaaaaaaaaaaaa + bbbbbbbbbbbbbbbbbbb + cccccccccccccccc
                            + dddddddddddddddd + eeeeeeeeeeee,
                        ),
                        1.0,
                        sinR, 1.0, 0.0,
                        0.0, -sinR, 0.0,
                    );
            }
        "#]],
        &FormattingOptions {
            max_line_width: 80,
            ..Default::default()
        }
        .into(),
    );
}

#[test]
fn format_matrix_break_mat_lines_if_forced_multiline_by_line_length() {
    // I don't think this looks too bad - but there was no big discussion on how we should handle this
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
fn main() {
    let x = mat3x3(
        cosR, this_is_a_very_long_name_that_causes_this_matrix_line_not_to_fit_into_the_max_length, 1.0,
        sinR,
        1.0, 0.0, 0.0,
        -sinR, 0.0,
    );
}",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn main() {
                let x = mat3x3(
                        cosR,
                        this_is_a_very_long_name_that_causes_this_matrix_line_not_to_fit_into_the_max_length,
                        1.0,
                        sinR, 1.0, 0.0,
                        0.0, -sinR, 0.0,
                    );
            }
        "#]],
        &CheckOptions {
            assert_line_width: None,
            formatting: FormattingOptions {
                max_line_width: 80,
                ..Default::default()
            },
            ..Default::default()
        },
    );
}

#[test]
fn format_matrix_if_number_of_args_equals_rows() {
    check(
        "
fn main() {
    let x = mat2x4<f32>(
    vec2(0.0, 0.0),
    vec2(0.0, 0.0),
    vec2(0.0, 0.0),
    vec2(0.0, 0.0),
    );
}",
        expect![[r#"
            fn main() {
                let x = mat2x4<f32>(
                        vec2(0.0, 0.0),
                        vec2(0.0, 0.0),
                        vec2(0.0, 0.0),
                        vec2(0.0, 0.0),
                    );
            }
        "#]],
    );
}

#[test]
fn format_matrix_dont_if_incorrect_number_of_args_equals_rows() {
    check(
        "
fn main() {
    let x = mat2x4<f32>(
    0.0, 0.0,
    1.0, 1.0,
    0.1
    );
}",
        expect![[r#"
            fn main() {
                let x = mat2x4<f32>(0.0, 0.0, 1.0, 1.0, 0.1);
            }
        "#]],
    );
}

#[test]
fn format_matrix_dont_if_incorrect_number_of_args_vecs_long() {
    check(
        "
        const RGB2YIQ: mat3x3<f32> = mat3x3<f32>(
                vec3f(0.300, 0.5900, 0.1100),
                vec3f(0.599,-0.2773,-0.3217,),
                vec3f(0.213, -0.5251, 0.3121),
            );
",
        expect![[r#"
            const RGB2YIQ: mat3x3<f32> = mat3x3<f32>(
                    vec3f(0.300, 0.5900, 0.1100),
                    vec3f(0.599, -0.2773, -0.3217),
                    vec3f(0.213, -0.5251, 0.3121),
                );
        "#]],
    );
}
