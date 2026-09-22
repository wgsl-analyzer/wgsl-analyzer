use expect_test::expect;

use crate::test_util::check_sweep;

#[expect(clippy::too_many_lines, reason = "It is a long string too.")]
#[test]
fn sweep_for_with_function_calls() {
    check_sweep(
        "
        fn main() {
            for(var i = initial_value(thing.bla); preciate(i, thing); update(i, b)) {}
        }
        ",
        expect![[r#"
            // max_length: 85--------------------------------------------------------------------
            fn main() {
            	for(var i = initial_value(thing.bla); preciate(i, thing); update(i, b)) {}
            }

            // max_length: 77------------------------------------------------------------
            fn main() {
            	for(
            		var i = initial_value(thing.bla);
            		preciate(i, thing);
            		update(i, b)
            	) {}
            }

            // max_length: 40-----------------------
            fn main() {
            	for(
            		var i =
            			initial_value(thing.bla);
            		preciate(i, thing);
            		update(i, b)
            	) {}
            }

            // max_length: 36-------------------
            fn main() {
            	for(
            		var i = initial_value(
            				thing.bla,
            			);
            		preciate(i, thing);
            		update(i, b)
            	) {}
            }

            // max_length: 29------------
            fn main() {
            	for(
            		var i =
            			initial_value(
            				thing.bla,
            			);
            		preciate(i, thing);
            		update(i, b)
            	) {}
            }

            // max_length: 26---------
            fn main() {
            	for(
            		var i =
            			initial_value(
            				thing.bla,
            			);
            		preciate(
            			i,
            			thing,
            		);
            		update(i, b)
            	) {}
            }

            // max_length: 25--------
            fn main() {
            	for(
            		var i =
            			initial_value(
            				thing
            					.bla,
            			);
            		preciate(
            			i,
            			thing,
            		);
            		update(i, b)
            	) {}
            }

            // max_length: 19--
            fn main() {
            	for(
            		var i =
            			initial_value(
            				thing
            					.bla,
            			);
            		preciate(
            			i,
            			thing,
            		);
            		update(
            			i,
            			b,
            		)
            	) {}
            }

            // max_length: 10
            fn main(
            ) {
            	for(
            		var i =
            			initial_value(
            				thing
            					.bla,
            			);
            		preciate(
            			i,
            			thing,
            		);
            		update(
            			i,
            			b,
            		)
            	) {}
            }

        "#]],
    );
}

#[test]
fn sweep_common_for_statement() {
    check_sweep(
        "
        fn main() {
            for(var i = 0; i < AMOUNT; i++) {}
        }
        ",
        expect![[r#"
            // max_length: 45----------------------------
            fn main() {
            	for(var i = 0; i < AMOUNT; i++) {}
            }

            // max_length: 37--------------------
            fn main() {
            	for(
            		var i = 0;
            		i < AMOUNT;
            		i++
            	) {}
            }

            // max_length: 18-
            fn main() {
            	for(
            		var i = 0;
            		i
            		< AMOUNT;
            		i++
            	) {}
            }

            // max_length: 17
            fn main() {
            	for(
            		var i =
            			0;
            		i
            		< AMOUNT;
            		i++
            	) {}
            }

            // max_length: 10
            fn main(
            ) {
            	for(
            		var i =
            			0;
            		i
            		< AMOUNT;
            		i++
            	) {}
            }

        "#]],
    );
}

#[test]
fn sweep_for_statement_empty_initializer() {
    check_sweep(
        "
        fn main() {
            for(; i < AMOUNT; i++) {}
        }
        ",
        expect![[r#"
            // max_length: 36-------------------
            fn main() {
            	for(; i < AMOUNT; i++) {}
            }

            // max_length: 28-----------
            fn main() {
            	for(
            		;
            		i < AMOUNT;
            		i++
            	) {}
            }

            // max_length: 18-
            fn main() {
            	for(
            		;
            		i
            		< AMOUNT;
            		i++
            	) {}
            }

            // max_length: 10
            fn main(
            ) {
            	for(
            		;
            		i
            		< AMOUNT;
            		i++
            	) {}
            }

        "#]],
    );
}

#[test]
fn sweep_for_statement_empty_everything() {
    check_sweep(
        "
        fn main() {
            for(;;) {}
        }
        ",
        expect![[r#"
            // max_length: 21----
            fn main() {
            	for(;;) {}
            }

            // max_length: 13
            fn main() {
            	for(
            		;
            		;
            	) {}
            }

            // max_length: 10
            fn main(
            ) {
            	for(
            		;
            		;
            	) {}
            }

        "#]],
    );
}
