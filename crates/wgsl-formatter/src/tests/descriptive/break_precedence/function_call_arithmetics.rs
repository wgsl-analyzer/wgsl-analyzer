use expect_test::expect;

use crate::test_util::check_sweep;

#[expect(clippy::too_many_lines, reason = "It is a long string too.")]
#[test]
fn sweep_function_call_with_arithmetics() {
    check_sweep(
        "
        fn main() {
            a = fun_thing(111 + 2222 + 3333 * 44 / (18+19)) + 1.0 + 2.0 + 33.28198742 + (sin(1284.0 * thing.thing) / THE_FACTOR) * 27.0
        }",
        expect![[r#"
            // max_length: 134--------------------------------------------------------------------------------------------------------------------
            fn main() {
            	a = fun_thing(111 + 2222 + 3333 * 44 / (18 + 19)) + 1.0 + 2.0 + 33.28198742 + (sin(1284.0 * thing.thing) / THE_FACTOR) * 27.0;
            }

            // max_length: 129---------------------------------------------------------------------------------------------------------------
            fn main() {
            	a = fun_thing(111 + 2222 + 3333 * 44 / (18 + 19)) + 1.0 + 2.0 + 33.28198742
            		+ (sin(1284.0 * thing.thing) / THE_FACTOR) * 27.0;
            }

            // max_length: 78-------------------------------------------------------------
            fn main() {
            	a =
            		fun_thing(111 + 2222 + 3333 * 44 / (18 + 19)) + 1.0 + 2.0
            		+ 33.28198742 + (sin(1284.0 * thing.thing) / THE_FACTOR) * 27.0;
            }

            // max_length: 71------------------------------------------------------
            fn main() {
            	a =
            		fun_thing(111 + 2222 + 3333 * 44 / (18 + 19)) + 1.0 + 2.0
            		+ 33.28198742
            		+ (sin(1284.0 * thing.thing) / THE_FACTOR) * 27.0;
            }

            // max_length: 64-----------------------------------------------
            fn main() {
            	a =
            		fun_thing(111 + 2222 + 3333 * 44 / (18 + 19)) + 1.0
            		+ 2.0 + 33.28198742
            		+ (sin(1284.0 * thing.thing) / THE_FACTOR) * 27.0;
            }

            // max_length: 58-----------------------------------------
            fn main() {
            	a =
            		fun_thing(111 + 2222 + 3333 * 44 / (18 + 19))
            		+ 1.0 + 2.0 + 33.28198742
            		+ (sin(1284.0 * thing.thing) / THE_FACTOR) * 27.0;
            }

            // max_length: 57----------------------------------------
            fn main() {
            	a =
            		fun_thing(111 + 2222 + 3333 * 44 / (18 + 19))
            		+ 1.0 + 2.0 + 33.28198742
            		+ (sin(1284.0 * thing.thing) / THE_FACTOR)
            		* 27.0;
            }

            // max_length: 52-----------------------------------
            fn main() {
            	a = fun_thing(
            			111 + 2222 + 3333 * 44 / (18 + 19),
            		) + 1.0 + 2.0 + 33.28198742
            		+ (sin(1284.0 * thing.thing) / THE_FACTOR)
            		* 27.0;
            }

            // max_length: 49--------------------------------
            fn main() {
            	a = fun_thing(
            			111 + 2222 + 3333 * 44 / (18 + 19),
            		) + 1.0 + 2.0 + 33.28198742
            		+ (sin(1284.0 * thing.thing)
            			/ THE_FACTOR) * 27.0;
            }

            // max_length: 46-----------------------------
            fn main() {
            	a = fun_thing(
            			111 + 2222
            			+ 3333 * 44 / (18 + 19),
            		) + 1.0 + 2.0 + 33.28198742
            		+ (sin(1284.0 * thing.thing)
            			/ THE_FACTOR) * 27.0;
            }

            // max_length: 35------------------
            fn main() {
            	a = fun_thing(
            			111 + 2222
            			+ 3333 * 44
            			/ (18 + 19),
            		) + 1.0 + 2.0 + 33.28198742
            		+ (sin(
            				1284.0
            				* thing.thing,
            			) / THE_FACTOR) * 27.0;
            }

            // max_length: 34-----------------
            fn main() {
            	a = fun_thing(
            			111 + 2222
            			+ 3333 * 44
            			/ (18 + 19),
            		) + 1.0 + 2.0
            		+ 33.28198742 + (sin(
            				1284.0
            				* thing.thing,
            			) / THE_FACTOR)
            		* 27.0;
            }

            // max_length: 29------------
            fn main() {
            	a = fun_thing(
            			111 + 2222
            			+ 3333 * 44
            			/ (18 + 19),
            		) + 1.0 + 2.0
            		+ 33.28198742 + (sin(
            				1284.0
            				* thing
            					.thing,
            			) / THE_FACTOR)
            		* 27.0;
            }

            // max_length: 28-----------
            fn main() {
            	a = fun_thing(
            			111 + 2222
            			+ 3333 * 44
            			/ (18 + 19),
            		) + 1.0 + 2.0
            		+ 33.28198742
            		+ (sin(
            				1284.0
            				* thing
            					.thing,
            			) / THE_FACTOR)
            		* 27.0;
            }

            // max_length: 26---------
            fn main() {
            	a = fun_thing(
            			111 + 2222
            			+ 3333 * 44
            			/ (18 + 19),
            		) + 1.0 + 2.0
            		+ 33.28198742
            		+ (sin(
            				1284.0
            				* thing
            					.thing,
            			)
            			/ THE_FACTOR)
            		* 27.0;
            }

            // max_length: 23------
            fn main() {
            	a = fun_thing(
            			111 + 2222
            			+ 3333 * 44
            			/ (18
            				+ 19),
            		) + 1.0 + 2.0
            		+ 33.28198742
            		+ (sin(
            				1284.0
            				* thing
            					.thing,
            			)
            			/ THE_FACTOR)
            		* 27.0;
            }

            // max_length: 22-----
            fn main() {
            	a = fun_thing(
            			111 + 2222
            			+ 3333
            			* 44
            			/ (18
            				+ 19),
            		) + 1.0 + 2.0
            		+ 33.28198742
            		+ (sin(
            				1284.0
            				* thing
            					.thing,
            			)
            			/ THE_FACTOR)
            		* 27.0;
            }

            // max_length: 21----
            fn main() {
            	a = fun_thing(
            			111
            			+ 2222
            			+ 3333
            			* 44
            			/ (18
            				+ 19),
            		) + 1.0 + 2.0
            		+ 33.28198742
            		+ (sin(
            				1284.0
            				* thing
            					.thing,
            			)
            			/ THE_FACTOR)
            		* 27.0;
            }

            // max_length: 20---
            fn main() {
            	a = fun_thing(
            			111
            			+ 2222
            			+ 3333
            			* 44
            			/ (18
            				+ 19),
            		) + 1.0
            		+ 2.0
            		+ 33.28198742
            		+ (sin(
            				1284.0
            				* thing
            					.thing,
            			)
            			/ THE_FACTOR)
            		* 27.0;
            }

            // max_length: 17
            fn main() {
            	a =
            		fun_thing(
            			111
            			+ 2222
            			+ 3333
            			* 44
            			/ (18
            				+ 19),
            		) + 1.0
            		+ 2.0
            		+ 33.28198742
            		+ (sin(
            				1284.0
            				* thing
            					.thing,
            			)
            			/ THE_FACTOR)
            		* 27.0;
            }

            // max_length: 14
            fn main() {
            	a =
            		fun_thing(
            			111
            			+ 2222
            			+ 3333
            			* 44
            			/ (18
            				+ 19),
            		)
            		+ 1.0
            		+ 2.0
            		+ 33.28198742
            		+ (sin(
            				1284.0
            				* thing
            					.thing,
            			)
            			/ THE_FACTOR)
            		* 27.0;
            }

            // max_length: 10
            fn main(
            ) {
            	a =
            		fun_thing(
            			111
            			+ 2222
            			+ 3333
            			* 44
            			/ (18
            				+ 19),
            		)
            		+ 1.0
            		+ 2.0
            		+ 33.28198742
            		+ (sin(
            				1284.0
            				* thing
            					.thing,
            			)
            			/ THE_FACTOR)
            		* 27.0;
            }

        "#]],
    );
}
