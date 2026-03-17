use expect_test::expect;
use parser::{Edition, ParseEntryPoint};
use rowan::{TextLen as _, TextRange, TextSize};

use crate::{FormattingOptions, test_util::check_range};
use indoc::indoc;

#[test]
fn format_range_in_for_initializer() {
    check_range(
        indoc! {"
            fn    main()
            {
                for(var #|# i      =  #|# 0; i         < 7;  i+=                  1) {}
            }
        "},
        expect![[r#"
            fn    main()
            {
                for(var i = 0; i         < 7;  i+=                  1) {}
            }
        "#]],
    );
}

#[test]
fn format_range_in_for_condition() {
    check_range(
        indoc! {"
            fn    main()
            {
                for(var i      =  0; i #|#         < #|#7;  i+=                  1) {}
            }
        "},
        expect![[r#"
            fn    main()
            {
                for(var i      =  0; i < 7;  i+=                  1) {}
            }
        "#]],
    );
}

#[test]
fn format_range_in_for_contiuing() {
    check_range(
        indoc! {"
            fn    main()
            {
                for(var i      =  0; i   < 7;  i+=  #|#      #|#           1) {}
            }
        "},
        expect![[r#"
            fn    main()
            {
                for(var i      =  0; i   < 7;  i += 1) {}
            }
        "#]],
    );
}

#[test]
fn format_range_in_for_statement() {
    check_range(
        indoc! {"
            fn          main()

            {
                for(var #|# i      =  0; i < 7;  i #|# +=                  1) {}
            }
        "},
        expect![[r#"
            fn          main()

            {
                for(var i = 0; i < 7; i += 1) {}
            }
        "#]],
    );
}
