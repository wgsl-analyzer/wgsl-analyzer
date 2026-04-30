use expect_test::expect;

use crate::test_util::{check_range, strip_leading_indentation};

#[test]
fn format_range_in_for_initializer() {
    check_range(
        &strip_leading_indentation(
            "
            fn    main()
            {
                for(var #|# i      =  #|# 0; i         < 7;  i+=                  1) {}
            }
            ",
        ),
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
        &strip_leading_indentation(
            "
            fn    main()
            {
                for(var i      =  0; i #|#         < #|#7;  i+=                  1) {}
            }
            ",
        ),
        expect![[r#"
            fn    main()
            {
                for(var i      =  0; i < 7;  i+=                  1) {}
            }
        "#]],
    );
}

#[test]
fn format_range_in_for_continuing() {
    check_range(
        &strip_leading_indentation(
            "
            fn    main()
            {
                for(var i      =  0; i   < 7;  i+=  #|#      #|#           1) {}
            }
            ",
        ),
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
        &strip_leading_indentation(
            "
            fn          main()

            {
                for(var #|# i      =  0; i < 7;  i #|# +=                  1) {}
            }
            ",
        ),
        expect![[r#"
            fn          main()

            {
                for(var i = 0; i < 7; i += 1) {}
            }
        "#]],
    );
}
