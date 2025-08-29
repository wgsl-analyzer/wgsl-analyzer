use expect_test::expect;

use crate::test_util::{check, check_tabs};

#[test]
fn leave_matrix_alone() {
    check(
        "
fn main() {
    let x = mat3x3(
        cosR,  0.0, sinR,
        0.0, 1.0, 0.0,
        -sinR, 0.0, cosR,
    );
}",
        expect![["
            fn main() {
                let x = mat3x3(
                    cosR, 0.0, sinR,
                    0.0, 1.0, 0.0,
                    -sinR, 0.0, cosR,
                );
            }"]],
    );
}

#[test]
fn leave_matrix_alone_tabs() {
    check_tabs(
        "
fn main() {
	let x = mat3x3(
		cosR,  0.0, sinR,
		0.0, 1.0, 0.0,
		-sinR, 0.0, cosR,
	);
}",
        expect![["
			fn main() {
				let x = mat3x3(
					cosR, 0.0, sinR,
					0.0, 1.0, 0.0,
					-sinR, 0.0, cosR,
				);
			}"]],
    );
}
