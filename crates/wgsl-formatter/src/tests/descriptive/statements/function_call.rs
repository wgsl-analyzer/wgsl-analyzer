use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{CheckOptions, check, check_comments, check_sweep, check_with_options},
};

#[test]
pub(crate) fn format_function_call_statement() {
    check(
        "fn main() {
        foo();
        }",
        expect![[r#"
            fn main() {
                foo();
            }
        "#]],
    );
}

#[test]
pub(crate) fn format_2_function_call_statements() {
    check(
        "fn main() {
        foo();
        bar();
        }",
        expect![[r#"
            fn main() {
                foo();
                bar();
            }
        "#]],
    );
}

#[test]
pub(crate) fn format_insanely_long_function_call_statement() {
    check_with_options(
        "fn main() {
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        foo_efek_felkj_soiu_flejk_lkjef_aoieu_flkejfalk_lkjeifou_flj_lkjsieuf_flkj_Ljklllefjief();
        }",
        expect![[r#"
            fn main() {
                //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
                foo_efek_felkj_soiu_flejk_lkjef_aoieu_flkejfalk_lkjeifou_flj_lkjsieuf_flkj_Ljklllefjief();
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
pub(crate) fn format_function_call_statement_with_arguments() {
    check(
        "fn main() {
        bla(12, bar(), 1 + vubble);
        }",
        expect![[r#"
            fn main() {
                bla(12, bar(), 1 + vubble);
            }
        "#]],
    );
}

#[test]
pub(crate) fn format_comment_in_function_call_statement() {
    check_comments(
        "fn main() {
        ## bla ## ( ## 12 ## , ## bar ## ( ## ) ## , ## 1 ## + ## vubble ## ) ## ; ##
        }",
        expect![[r#"
            fn main() {
                /* 0 */ bla /* 1 */ (
                    /* 2 */ 12, /* 3 */ /* 4 */
                    bar /* 5 */ (/* 6 */), /* 7 */ /* 8 */
                    1 /* 9 */ + /* 10 */ vubble, /* 11 */
                ) /* 12 */ ; /* 13 */
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                bla // 1
                (
                    // 2
                    12, // 3
                    // 4
                    bar // 5
                    (
                        // 6
                    ), // 7
                    // 8
                    1 // 9
                    + // 10
                    vubble, // 11
                ) // 12
                ; // 13
            }
        "#]],
    );
}

#[test]
fn format_function_call_simple() {
    check(
        "fn main() {
    min  (  x,y );
}",
        expect![[r#"
            fn main() {
                min(x, y);
            }
        "#]],
    );
}

#[expect(clippy::too_many_lines, reason = "It is a long string too.")]
#[test]
fn format_function_call_sweep() {
    check_sweep(
        "
        fn main() {
            cool.field[cool.index] = path::fun::thing(bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff).fffffffffffffffffffffffffff.xxxxxxxxxxxxx.zzzzzzzzzzzz);
        }",
        expect![[r#"
            // max_length: 159 --------------------------------------------------------------------------------------------------------------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff).fffffffffffffffffffffffffff.xxxxxxxxxxxxx.zzzzzzzzzzzz);
            }

            // max_length: 151 ------------------------------------------------------------------------------------------------------------------------------------
            fn main() {
            	cool.field[cool.index] =
            		path::fun::thing(bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff).fffffffffffffffffffffffffff.xxxxxxxxxxxxx.zzzzzzzzzzzz);
            }

            // max_length: 130 ---------------------------------------------------------------------------------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(
            			bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff).fffffffffffffffffffffffffff.xxxxxxxxxxxxx.zzzzzzzzzzzz,
            		);
            }

            // max_length: 116 -------------------------------------------------------------------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(
            			bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff).fffffffffffffffffffffffffff.xxxxxxxxxxxxx
            				.zzzzzzzzzzzz,
            		);
            }

            // max_length: 102 -----------------------------------------------------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(
            			bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff).fffffffffffffffffffffffffff
            				.xxxxxxxxxxxxx.zzzzzzzzzzzz,
            		);
            }

            // max_length: 88 ----------------------------------------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(
            			bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff)
            				.fffffffffffffffffffffffffff.xxxxxxxxxxxxx.zzzzzzzzzzzz,
            		);
            }

            // max_length: 71 -----------------------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(
            			bbb_bbbb_bbbbb(cccccc.cccccccccccccc, ffffffffff)
            				.fffffffffffffffffffffffffff.xxxxxxxxxxxxx
            				.zzzzzzzzzzzz,
            		);
            }

            // max_length: 60 ------------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(
            			bbb_bbbb_bbbbb(
            				cccccc.cccccccccccccc,
            				ffffffffff,
            			).fffffffffffffffffffffffffff.xxxxxxxxxxxxx
            				.zzzzzzzzzzzz,
            		);
            }

            // max_length: 54 ------------------------------------
            fn main() {
            	cool.field[cool.index] = path::fun::thing(
            			bbb_bbbb_bbbbb(
            				cccccc.cccccccccccccc,
            				ffffffffff,
            			).fffffffffffffffffffffffffff
            				.xxxxxxxxxxxxx.zzzzzzzzzzzz,
            		);
            }

            // max_length: 45 ---------------------------
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

            // max_length: 43 -------------------------
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

            // max_length: 40 ----------------------
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

            // max_length: 37 -------------------
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

            // max_length: 27 ---------
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

            // max_length: 24 ------
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
