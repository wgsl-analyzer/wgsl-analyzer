use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check, check_comments};

//TODO(MonaMayrhofer) For now these tests just check for something so that attributes are handled and
// the formatter doesn't crash. More thought should be put into how exactly they shall be formatted

#[test]
pub fn format_attrs_on_struct_members() {
    check(
        "struct VertexOutput {
            @attr(0) @attr(1) position: vec4<f32>,
            @attr(0) @attr(1) uv: vec2<f32>,
        }",
        expect![[r#"
            struct VertexOutput {
                @attr(0)
                @attr(1)
                position: vec4<f32>,
                @attr(0)
                @attr(1)
                uv: vec2<f32>,
            }
        "#]],
    );
}

#[test]
pub fn format_attrs_on_functions() {
    check(
        "@attr(0)
        @attr(1)
        fn main(
        ) {
        }",
        expect![[r#"
            @attr(0)
            @attr(1)
            fn main() {}
        "#]],
    );
}

#[test]
pub fn format_attrs_on_function_return_type() {
    check(
        "
        fn thing(
        ) -> @attr(0) @attr(1) vec4<f32> {
        }",
        expect![[r#"
            fn thing() -> @attr(0)
            @attr(1)
            vec4<f32> {}
        "#]],
    );
}

#[test]
pub fn format_attrs_on_function_parameter() {
    check(
        "
        fn thing(
            @attr(0) @attr(1) position: vec4<f32>,
            @attr(0) @attr(1) uv: vec2<f32>,
        ) -> vec4<f32> {
        }",
        expect![[r#"
            fn thing(
                @attr(0)
                @attr(1)
                position: vec4<f32>,
                @attr(0)
                @attr(1)
                uv: vec2<f32>,
            ) -> vec4<f32> {}
        "#]],
    );
}

#[test]
pub fn format_attrs_on_function_body() {
    //TODO (MonaMayrhofer) This attribute spacing is ugly.
    check(
        "
        fn thing() -> vec4<f32> @attr(0) @attr(1) {
        }",
        expect![[r#"
            fn thing() -> vec4<f32> @attr(0)
            @attr(1)
            {}
        "#]],
    );
}

#[test]
pub fn format_attrs_on_global_variable() {
    check(
        "
        @attr(0) @attr(1) var<uniform> material: CustomMaterial;
        ",
        expect![[r#"
            @attr(0)
            @attr(1)
            var<uniform> material: CustomMaterial;
        "#]],
    );
}

#[test]
pub fn format_attrs_on_override() {
    check(
        "
        @attr(0) @attr(1) override amount: u64 = 0;
        ",
        expect![[r#"
            @attr(0)
            @attr(1)
            override amount: u64 = 0;
        "#]],
    );
}

#[test]
pub fn format_attrs_on_compound_statement() {
    check(
        "
        fn main() {
            @attr(0) @attr(1) {}
            if true @attr(0) @attr(1) {}
        }
        ",
        expect![[r#"
            fn main() {
                @attr(0)
                @attr(1)
                {}
                if true @attr(0)
                @attr(1)
                {}
            }
        "#]],
    );
}

#[test]
pub fn format_attrs_on_nonempty_compound_statement() {
    check(
        "
        fn main() {
            @attr(0) @attr(1) {
                let a = 0;
            }
            if true @attr(0) @attr(1) {
                let a = 0;
            }
        }
        ",
        expect![[r#"
            fn main() {
                @attr(0)
                @attr(1)
                {
                    let a = 0;
                }
                if true @attr(0)
                @attr(1)
                {
                    let a = 0;
                }
            }
        "#]],
    );
}

#[test]
pub fn format_attrs_on_if_statement() {
    check(
        "
        fn main() {
            @attr(0) @attr(1) if true {}
        }
        ",
        expect![[r#"
            fn main() {
                @attr(0)
                @attr(1)
                if true {}
            }
        "#]],
    );
}

#[test]
pub fn format_attrs_on_switch_statement() {
    check(
        "
        fn main() {
            @attr(0) @attr(1) switch a {}
        }
        ",
        expect![[r#"
            fn main() {
                @attr(0)
                @attr(1)
                switch a {}
            }
        "#]],
    );
}

#[test]
pub fn format_attrs_on_switch_statement_body() {
    check(
        "
        fn main() {
            switch a @attr(0) @attr(1) {}
        }
        ",
        expect![[r#"
            fn main() {
                switch a @attr(0)
                @attr(1)
                {}
            }
        "#]],
    );
}

#[test]
pub fn format_attrs_on_loop_statement() {
    check(
        "
        fn main() {
            @attr(0) @attr(1) loop {}
        }
        ",
        expect![[r#"
            fn main() {
                @attr(0)
                @attr(1)
                loop {}
            }
        "#]],
    );
}

#[test]
pub fn format_attrs_on_loop_statement_body() {
    check(
        "
        fn main() {
            loop @attr(0) @attr(1) {}
        }
        ",
        expect![[r#"
            fn main() {
                loop @attr(0)
                @attr(1)
                {}
            }
        "#]],
    );
}

#[test]
pub fn format_attrs_on_for_statement() {
    check(
        "
        fn main() {
            @attr(0) @attr(1) for(var i = 0; i < 10; i++) {}
        }
        ",
        expect![[r#"
            fn main() {
                @attr(0)
                @attr(1)
                for(var i = 0; i < 10; i++) {}
            }
        "#]],
    );
}

#[test]
pub fn format_attrs_on_loop_continuing_block() {
    check(
        "
        fn main() {
        loop{
        continuing
        @attr(0) @attr(1) {}
        }
        }
        ",
        expect![[r#"
            fn main() {
                loop {
                    continuing @attr(0)
                    @attr(1)
                    {}
                }
            }
        "#]],
    );
}

#[test]
pub fn format_attrs_on_while_statement() {
    check(
        "
        fn main() {
        @attr(0) @attr(1) while true {}
        }
        ",
        expect![[r#"
            fn main() {
                @attr(0)
                @attr(1)
                while true {}
            }
        "#]],
    );
}

#[test]
pub fn format_attrs_on_input_statement() {
    assert_out_of_scope(
        "
        @if(THING)
        import the::thing;
        ",
        "This should be supported as soon as the parser supports it.",
    );
}
