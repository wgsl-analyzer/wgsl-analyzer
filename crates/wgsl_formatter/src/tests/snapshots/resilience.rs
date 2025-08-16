use expect_test::expect;

use crate::test_util::check;

#[test]
fn fn_headers_with_some_malformed() {
    check(
        "
        fn  main ( a:  b) -> i32 {}
        fn  main ( a
        fn  main ( a:  b) -> {}
        fn  main ( a:  b) -> i32 {}
        fn  main ( a:
        fn  main ( a:  b) -> i32
        fn  main ( a:  b) -> i32 {}
        ",
        expect![["fn  main ( a "]],
    );
}
