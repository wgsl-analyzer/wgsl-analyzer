use expect_test::expect;

use crate::test_util::check;

#[test]
pub fn format_let_decl_keeps_line_comment_in_same_place_before() {
    check(
        "fn main() {

        // Assign 1
        let a = 1;


        }",
        expect![["
            fn main() {
                // Assign 1
                let a = 1;
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_line_comment_in_same_place_after() {
    check(
        "fn main() {

        let a = 1;
        // Assign 1


        }",
        expect![["
            fn main() {
                let a = 1;
                // Assign 1
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_line_comment_in_same_place_empty_line_before() {
    check(
        "fn main() {

        // Assign 1

        let a = 1;


        }",
        expect![["
            fn main() {
                // Assign 1

                let a = 1;
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_line_comment_in_same_place_empty_line_after() {
    check(
        "fn main() {


        let a = 1;

        // Assign 1

        }",
        expect![["
            fn main() {
                let a = 1;

                // Assign 1
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_block_comment_in_same_place_same_line() {
    check(
        "fn main() {

        let a = 1; /* Assign 1 */


        }",
        expect![["
            fn main() {
                let a = 1; /* Assign 1 */
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_block_comment_in_same_place_before() {
    check(
        "fn main() {

        /* Assign 1 */
        let a = 1;


        }",
        expect![["
            fn main() {
                /* Assign 1 */
                let a = 1;
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_block_comment_in_same_place_after() {
    check(
        "fn main() {

        let a = 1;
        /* Assign 1 */


        }",
        expect![["
            fn main() {
                let a = 1;
                /* Assign 1 */
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_block_comment_in_same_place_empty_line_before() {
    check(
        "fn main() {

        /* Assign 1 */

        let a = 1;


        }",
        expect![["
            fn main() {
                /* Assign 1 */

                let a = 1;
            }
        "]],
    );
}

#[test]
pub fn format_let_decl_keeps_block_comment_in_same_place_empty_line_after() {
    check(
        "fn main() {


        let a = 1;

        /* Assign 1 */

        }",
        expect![["
            fn main() {
                let a = 1;

                /* Assign 1 */
            }
        "]],
    );
}
