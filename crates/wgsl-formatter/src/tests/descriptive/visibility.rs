use expect_test::expect;

use crate::{
    FormattingOptions,
    test_util::{CheckOptions, check, check_comments, check_with_options},
};

#[test]
pub(crate) fn format_comments_around_visibility() {
    check_comments(
        "## public ## var ## < ## private ## > ## a ## : ## private ## ; ##",
        expect![[r#"
            /* 0 */ public /* 1 */ var /* 2 */ </* 3 */ private /* 4 */> /* 5 */ a /* 6 */: /* 7 */ private /* 8 */; /* 9 */
        "#]],
        expect![[r#"
            // 0
            public // 1
            var // 2
                <// 3
                    private, // 4
                > // 5
                a // 6
                : // 7
                private // 8
                ; // 9
        "#]],
    );
}

#[test]
fn format_visibility_before_fn() {
    check(
        "public    fn        main         ( ) {}",
        expect![[r#"
            public fn main() {}
        "#]],
    );
}

#[test]
fn format_visibility_before_struct() {
    check(
        "private    struct   Foo   { a   : u32 }",
        expect![[r#"
            private struct Foo {
                a: u32,
            }
        "#]],
    );
}

#[test]
fn format_visibility_before_var_const_override() {
    check(
        "private   var<private> foo: u32; 
        public   var<private> foo: u32; 
        private            const private   : private    = 0;
        public     const private  :    private = 0;
        private    override    aa   : vec3f   ;
        public   override     aa: vec3f;
        ",
        expect![[r#"
            private var<private> foo: u32;
            public var<private> foo: u32;
            private const private: private = 0;
            public const private: private = 0;
            private override aa: vec3f;
            public override aa: vec3f;
        "#]],
    );
}

#[test]
fn format_visibility_before_struct_with_private_as_name() {
    check(
        "private    struct     private   {    private  :   u32 }",
        expect![[r#"
            private struct private {
                private: u32,
            }
        "#]],
    );
}

#[test]
fn format_visibility_before_type_alias() {
    check(
        "private    alias     a   =  f32;",
        expect![[r#"
            private alias a = f32;
        "#]],
    );
}

#[test]
fn format_visibility_before_import() {
    check(
        "public    import foo  ::   private   ::  {  private  as   private  };",
        expect![[r#"
            public import foo::private::{private as private};
        "#]],
    );
}

#[test]
fn format_long_item_does_not_break_visibility() {
    check_with_options(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        private var long_name_variable_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: u32;",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            private var long_name_variable_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: u32;
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
