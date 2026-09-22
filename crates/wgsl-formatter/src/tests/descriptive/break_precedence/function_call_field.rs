use expect_test::expect;

use crate::test_util::check_sweep;

#[expect(clippy::too_many_lines, reason = "It is a long string too.")]
#[test]
fn sweep_function_call_with_field_expression() {
    check_sweep(
        "
        fn main() {
            cool.field[cool.index] = path::fun::thing(bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff).fffffffffffffffffffffffffff.xxxxxxxxxxxxx.zzzzzzzzzzzz);
        }",
        expect![[r#"
            // max_length: 159---------------------------------------------------------------------------------------------------------------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff).fffffffffffffffffffffffffff.xxxxxxxxxxxxx.zzzzzzzzzzzz);
            }

            // max_length: 151-------------------------------------------------------------------------------------------------------------------------------------
            fn main() {
            	cool.field[cool.index] =
            		path::fun::thing(bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff).fffffffffffffffffffffffffff.xxxxxxxxxxxxx.zzzzzzzzzzzz);
            }

            // max_length: 130----------------------------------------------------------------------------------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(
            			bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff).fffffffffffffffffffffffffff.xxxxxxxxxxxxx.zzzzzzzzzzzz,
            		);
            }

            // max_length: 116--------------------------------------------------------------------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(
            			bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff).fffffffffffffffffffffffffff.xxxxxxxxxxxxx
            				.zzzzzzzzzzzz,
            		);
            }

            // max_length: 102------------------------------------------------------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(
            			bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff).fffffffffffffffffffffffffff
            				.xxxxxxxxxxxxx.zzzzzzzzzzzz,
            		);
            }

            // max_length: 88-----------------------------------------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(
            			bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff)
            				.fffffffffffffffffffffffffff.xxxxxxxxxxxxx.zzzzzzzzzzzz,
            		);
            }

            // max_length: 71------------------------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(
            			bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff)
            				.fffffffffffffffffffffffffff.xxxxxxxxxxxxx
            				.zzzzzzzzzzzz,
            		);
            }

            // max_length: 60-------------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(
            			bbb_bbbb_bbbbb(
            				cccccc.cccccccccccccc,
            				ffffffffff,
            			).fffffffffffffffffffffffffff.xxxxxxxxxxxxx
            				.zzzzzzzzzzzz,
            		);
            }

            // max_length: 54-------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(
            			bbb_bbbb_bbbbb(
            				cccccc.cccccccccccccc,
            				ffffffffff,
            			).fffffffffffffffffffffffffff
            				.xxxxxxxxxxxxx.zzzzzzzzzzzz,
            		);
            }

            // max_length: 45----------------------------
            fn main() {
            	cool.field[cool.index] =
            		path::fun::thing(
            			bbb_bbbb_bbbbb(
            				cccccc.cccccccccccccc,
            				ffffffffff,
            			).fffffffffffffffffffffffffff
            				.xxxxxxxxxxxxx.zzzzzzzzzzzz,
            		);
            }

            // max_length: 43--------------------------
            fn main() {
            	cool.field[cool.index] =
            		path::fun::thing(
            			bbb_bbbb_bbbbb(
            				cccccc.cccccccccccccc,
            				ffffffffff,
            			).fffffffffffffffffffffffffff
            				.xxxxxxxxxxxxx
            				.zzzzzzzzzzzz,
            		);
            }

            // max_length: 40-----------------------
            fn main() {
            	cool.field[cool.index] =
            		path::fun::thing(
            			bbb_bbbb_bbbbb(
            				cccccc.cccccccccccccc,
            				ffffffffff,
            			)
            				.fffffffffffffffffffffffffff
            				.xxxxxxxxxxxxx
            				.zzzzzzzzzzzz,
            		);
            }

            // max_length: 37--------------------
            fn main() {
            	cool.field[cool.index] =
            		path::fun::thing(
            			bbb_bbbb_bbbbb(
            				cccccc
            					.cccccccccccccc,
            				ffffffffff,
            			)
            				.fffffffffffffffffffffffffff
            				.xxxxxxxxxxxxx
            				.zzzzzzzzzzzz,
            		);
            }

            // max_length: 27----------
            fn main() {
            	cool.field[
            		cool.index
            	] = path::fun::thing(
            			bbb_bbbb_bbbbb(
            				cccccc
            					.cccccccccccccc,
            				ffffffffff,
            			)
            				.fffffffffffffffffffffffffff
            				.xxxxxxxxxxxxx
            				.zzzzzzzzzzzz,
            		);
            }

            // max_length: 24-------
            fn main() {
            	cool.field[
            		cool.index
            	] =
            		path::fun::thing(
            			bbb_bbbb_bbbbb(
            				cccccc
            					.cccccccccccccc,
            				ffffffffff,
            			)
            				.fffffffffffffffffffffffffff
            				.xxxxxxxxxxxxx
            				.zzzzzzzzzzzz,
            		);
            }

            // max_length: 17
            fn main() {
            	cool.field[
            		cool
            			.index
            	] =
            		path::fun::thing(
            			bbb_bbbb_bbbbb(
            				cccccc
            					.cccccccccccccc,
            				ffffffffff,
            			)
            				.fffffffffffffffffffffffffff
            				.xxxxxxxxxxxxx
            				.zzzzzzzzzzzz,
            		);
            }

            // max_length: 14
            fn main() {
            	cool
            		.field[
            		cool
            			.index
            	] =
            		path::fun::thing(
            			bbb_bbbb_bbbbb(
            				cccccc
            					.cccccccccccccc,
            				ffffffffff,
            			)
            				.fffffffffffffffffffffffffff
            				.xxxxxxxxxxxxx
            				.zzzzzzzzzzzz,
            		);
            }

            // max_length: 10
            fn main(
            ) {
            	cool
            		.field[
            		cool
            			.index
            	] =
            		path::fun::thing(
            			bbb_bbbb_bbbbb(
            				cccccc
            					.cccccccccccccc,
            				ffffffffff,
            			)
            				.fffffffffffffffffffffffffff
            				.xxxxxxxxxxxxx
            				.zzzzzzzzzzzz,
            		);
            }

        "#]],
    );
}
