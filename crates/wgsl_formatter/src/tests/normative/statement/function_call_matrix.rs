use expect_test::expect;

use crate::test_util::check;

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
        1.0
        // Breaky
        , 1.0,
        sinR,
        0.0, 1.0, 0.0,
        -sinR, 0.0,
    );
}",
        expect![[r#"
            fn main() {
                let x = mat3x3(
                        cosR, 1.0, // Breaky
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
                        cosR, do_thing(
                            1.0, // Breaky
                        ), 1.0,
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
    check(
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
                        cosR, do_thing(
                            aaaaaaaaaaaaaaaaaaa + bbbbbbbbbbbbbbbbbbb + cccccccccccccccc
                            + dddddddddddddddd + eeeeeeeeeeee,
                        ), 1.0,
                        sinR, 1.0, 0.0,
                        0.0, -sinR, 0.0,
                    );
            }
        "#]],
    );
}

#[test]
fn format_matrix_break_mat_lines_if_forced_multiline_by_line_length() {
    // I don't think this looks too bad - but there was no big discussion on how we should handle this
    check(
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
    );
}

#[test]
fn format_matrix_dont_if_incorrect_number_of_args_vecs() {
    check(
        "
fn main() {
    let x = mat3x3(
    vec3(0.0, 0.0, 0.0),
    vec3(0.0, 0.0, 0.0),
    vec3(0.0, 0.0, 0.0),
    );
}",
        expect![[r#"
            fn main() {
                let x = mat3x3(
                        vec3(0.0, 0.0, 0.0),
                        vec3(0.0, 0.0, 0.0),
                        vec3(0.0, 0.0, 0.0),
                    );
            }
        "#]],
    );
}
