use expect_test::expect;

use crate::test_util::check_sweep;

#[expect(clippy::too_many_lines, reason = "It is a long string too.")]
#[test]
fn sweep_function_call_mat4x4_simple() {
    check_sweep(
        "
        fn main() {
            let a = mat4x4<f32>(0.0,0.01,0.001,0.0001,11.0,11.01,11.001,11.0001,222.0,222.01,222.001,222.0001,3333.0,3333.01,3333.001,3333.0001)
        }",
        expect![[r#"
            // max_length: 143-----------------------------------------------------------------------------------------------------------------------------
            fn main() {
            	let a = mat4x4<f32>(
            			0.0, 0.01, 0.001, 0.0001,
            			11.0, 11.01, 11.001, 11.0001,
            			222.0, 222.01, 222.001, 222.0001,
            			3333.0, 3333.01, 3333.001, 3333.0001,
            		);
            }

            // max_length: 48-------------------------------
            fn main() {
            	let a = mat4x4<f32>(
            			0.0, 0.01, 0.001, 0.0001,
            			11.0, 11.01, 11.001, 11.0001,
            			222.0, 222.01, 222.001, 222.0001,
            			3333.0,
            			3333.01,
            			3333.001,
            			3333.0001,
            		);
            }

            // max_length: 44---------------------------
            fn main() {
            	let a = mat4x4<f32>(
            			0.0, 0.01, 0.001, 0.0001,
            			11.0, 11.01, 11.001, 11.0001,
            			222.0,
            			222.01,
            			222.001,
            			222.0001,
            			3333.0,
            			3333.01,
            			3333.001,
            			3333.0001,
            		);
            }

            // max_length: 40-----------------------
            fn main() {
            	let a = mat4x4<f32>(
            			0.0, 0.01, 0.001, 0.0001,
            			11.0,
            			11.01,
            			11.001,
            			11.0001,
            			222.0,
            			222.01,
            			222.001,
            			222.0001,
            			3333.0,
            			3333.01,
            			3333.001,
            			3333.0001,
            		);
            }

            // max_length: 36-------------------
            fn main() {
            	let a = mat4x4<f32>(
            			0.0,
            			0.01,
            			0.001,
            			0.0001,
            			11.0,
            			11.01,
            			11.001,
            			11.0001,
            			222.0,
            			222.01,
            			222.001,
            			222.0001,
            			3333.0,
            			3333.01,
            			3333.001,
            			3333.0001,
            		);
            }

            // max_length: 23------
            fn main() {
            	let a =
            		mat4x4<f32>(
            			0.0,
            			0.01,
            			0.001,
            			0.0001,
            			11.0,
            			11.01,
            			11.001,
            			11.0001,
            			222.0,
            			222.01,
            			222.001,
            			222.0001,
            			3333.0,
            			3333.01,
            			3333.001,
            			3333.0001,
            		);
            }

            // max_length: 19--
            fn main() {
            	let a = mat4x4<
            			f32,
            		>(
            			0.0,
            			0.01,
            			0.001,
            			0.0001,
            			11.0,
            			11.01,
            			11.001,
            			11.0001,
            			222.0,
            			222.01,
            			222.001,
            			222.0001,
            			3333.0,
            			3333.01,
            			3333.001,
            			3333.0001,
            		);
            }

            // max_length: 18-
            fn main() {
            	let a =
            		mat4x4<
            			f32,
            		>(
            			0.0,
            			0.01,
            			0.001,
            			0.0001,
            			11.0,
            			11.01,
            			11.001,
            			11.0001,
            			222.0,
            			222.01,
            			222.001,
            			222.0001,
            			3333.0,
            			3333.01,
            			3333.001,
            			3333.0001,
            		);
            }

            // max_length: 10
            fn main(
            ) {
            	let a =
            		mat4x4<
            			f32,
            		>(
            			0.0,
            			0.01,
            			0.001,
            			0.0001,
            			11.0,
            			11.01,
            			11.001,
            			11.0001,
            			222.0,
            			222.01,
            			222.001,
            			222.0001,
            			3333.0,
            			3333.01,
            			3333.001,
            			3333.0001,
            		);
            }

        "#]],
    );
}

#[expect(clippy::too_many_lines, reason = "It is a long string too.")]
#[test]
fn sweep_function_call_mat2x2_nested() {
    check_sweep(
        "
        fn main() {
            let a = mat2x2<f32>(cos(buu + baa), -sin(baa-guu), sin(buu+baa), cos(baaaa));
        }",
        expect![[r#"
            // max_length: 88-----------------------------------------------------------------------
            fn main() {
            	let a = mat2x2<f32>(
            			cos(buu + baa), -sin(baa - guu),
            			sin(buu + baa), cos(baaaa),
            		);
            }

            // max_length: 43--------------------------
            fn main() {
            	let a = mat2x2<f32>(
            			cos(buu + baa),
            			-sin(baa - guu),
            			sin(buu + baa), cos(baaaa),
            		);
            }

            // max_length: 38---------------------
            fn main() {
            	let a = mat2x2<f32>(
            			cos(buu + baa),
            			-sin(baa - guu),
            			sin(buu + baa),
            			cos(baaaa),
            		);
            }

            // max_length: 27----------
            fn main() {
            	let a = mat2x2<f32>(
            			cos(buu + baa),
            			-sin(
            				baa - guu,
            			),
            			sin(buu + baa),
            			cos(baaaa),
            		);
            }

            // max_length: 26---------
            fn main() {
            	let a = mat2x2<f32>(
            			cos(
            				buu + baa,
            			),
            			-sin(
            				baa - guu,
            			),
            			sin(
            				buu + baa,
            			),
            			cos(baaaa),
            		);
            }

            // max_length: 25--------
            fn main() {
            	let a = mat2x2<f32>(
            			cos(
            				buu
            				+ baa,
            			),
            			-sin(
            				baa
            				- guu,
            			),
            			sin(
            				buu
            				+ baa,
            			),
            			cos(baaaa),
            		);
            }

            // max_length: 23------
            fn main() {
            	let a =
            		mat2x2<f32>(
            			cos(
            				buu
            				+ baa,
            			),
            			-sin(
            				baa
            				- guu,
            			),
            			sin(
            				buu
            				+ baa,
            			),
            			cos(baaaa),
            		);
            }

            // max_length: 22-----
            fn main() {
            	let a =
            		mat2x2<f32>(
            			cos(
            				buu
            				+ baa,
            			),
            			-sin(
            				baa
            				- guu,
            			),
            			sin(
            				buu
            				+ baa,
            			),
            			cos(
            				baaaa,
            			),
            		);
            }

            // max_length: 19--
            fn main() {
            	let a = mat2x2<
            			f32,
            		>(
            			cos(
            				buu
            				+ baa,
            			),
            			-sin(
            				baa
            				- guu,
            			),
            			sin(
            				buu
            				+ baa,
            			),
            			cos(
            				baaaa,
            			),
            		);
            }

            // max_length: 18-
            fn main() {
            	let a =
            		mat2x2<
            			f32,
            		>(
            			cos(
            				buu
            				+ baa,
            			),
            			-sin(
            				baa
            				- guu,
            			),
            			sin(
            				buu
            				+ baa,
            			),
            			cos(
            				baaaa,
            			),
            		);
            }

            // max_length: 10
            fn main(
            ) {
            	let a =
            		mat2x2<
            			f32,
            		>(
            			cos(
            				buu
            				+ baa,
            			),
            			-sin(
            				baa
            				- guu,
            			),
            			sin(
            				buu
            				+ baa,
            			),
            			cos(
            				baaaa,
            			),
            		);
            }

        "#]],
    );
}
