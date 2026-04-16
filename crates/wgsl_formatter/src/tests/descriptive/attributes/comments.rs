use expect_test::expect;

use crate::test_util::check_comments;

#[test]
pub fn format_comments_in_attrs_on_struct_members() {
    check_comments(
        "struct VertexOutput { ##
        ## @attr(0) ## @attr(1) ## position ## : vec4<f32>,
        ## @attr(0) ## @attr(1) ## uv ## : vec2<f32>,
        }",
        expect![[r#"
            struct VertexOutput {
                /* 0 */ /* 1 */
                @attr(0) /* 2 */
                @attr(1) /* 3 */
                position: /* 4 */ vec4<f32>,
                /* 5 */
                @attr(0) /* 6 */
                @attr(1) /* 7 */
                uv: /* 8 */ vec2<f32>,
            }
        "#]],
        expect![[r#"
            struct VertexOutput {
                // 0
                // 1
                @attr(0) // 2
                @attr(1) // 3
                position: // 4
                vec4<f32>,
                // 5
                @attr(0) // 6
                @attr(1) // 7
                uv: // 8
                vec2<f32>,
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_attrs_on_functions() {
    check_comments(
        "##@attr(0)##@attr(1)##fn##main(
        ) {
        }",
        expect![[r#"
            /* 0 */
            @attr(0) /* 1 */
            @attr(1) /* 2 */
            fn /* 3 */ main() {}
        "#]],
        expect![[r#"
            // 0
            @attr(0) // 1
            @attr(1) // 2
            fn // 3
            main() {}
        "#]],
    );
}

#[test]
pub fn format_comments_in_attrs_on_function_return_type() {
    check_comments(
        "
        fn thing(
        ) ## -> ## @attr(0) ## @attr(1) ## vec4<f32> ## {
        }",
        expect![[r#"
            fn thing() /* 0 */ -> /* 1 */ @attr(0) /* 2 */ @attr(1) /* 3 */ vec4<
                f32,
            > /* 4 */ {}
        "#]],
        expect![[r#"
            fn thing() // 0
            -> // 1
            @attr(0) // 2
            @attr(1) // 3
            vec4<f32> // 4
            {}
        "#]],
    );
}

#[test]
pub fn format_comments_in_attrs_on_function_parameter() {
    check_comments(
        "
        fn thing ## ( ##
        ## @attr(0) ## @attr(1) ## position: vec4<f32>,
        ## @attr(0) ## @attr(1) ## uv: vec2<f32>,
        ) -> vec4<f32> {
        }",
        expect![[r#"
            fn thing /* 0 */ (
                /* 1 */ /* 2 */
                @attr(0) /* 3 */
                @attr(1) /* 4 */
                position: vec4<f32>,
                /* 5 */
                @attr(0) /* 6 */
                @attr(1) /* 7 */
                uv: vec2<f32>,
            ) -> vec4<f32> {}
        "#]],
        expect![[r#"
            fn thing // 0
            (
                // 1
                // 2
                @attr(0) // 3
                @attr(1) // 4
                position: vec4<f32>,
                // 5
                @attr(0) // 6
                @attr(1) // 7
                uv: vec2<f32>,
            ) -> vec4<f32> {}
        "#]],
    );
}

#[test]
pub fn format_comments_in_attrs_on_function_body() {
    check_comments(
        "
        fn thing() -> ## vec4<f32> ## @attr(0) ## @attr(1) ## { ##
        }",
        expect![[r#"
            fn thing() -> /* 0 */ vec4<f32> /* 1 */ @attr(0) /* 2 */
            @attr(1) /* 3 */
            {
                /* 4 */
            }
        "#]],
        expect![[r#"
            fn thing() -> // 0
            vec4<f32> // 1
            @attr(0) // 2
            @attr(1) // 3
            {
                // 4
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_attrs_on_global_variable() {
    check_comments(
        "
        ## @attr(0) ## @attr(1) ## var<uniform> ## material: CustomMaterial;
        ",
        expect![[r#"
            /* 0 */
            @attr(0) /* 1 */
            @attr(1) /* 2 */
            var<uniform> /* 3 */ material: CustomMaterial;
        "#]],
        expect![[r#"
            // 0
            @attr(0) // 1
            @attr(1) // 2
            var<uniform> // 3
                material: CustomMaterial;
        "#]],
    );
}

#[test]
pub fn format_comments_in_attrs_on_override() {
    check_comments(
        "
        ## @attr(0) ## @attr(1) ## override amount: u64 = 0;
        ",
        expect![[r#"
            /* 0 */
            @attr(0) /* 1 */
            @attr(1) /* 2 */
            override amount: u64 = 0;
        "#]],
        expect![[r#"
            // 0
            @attr(0) // 1
            @attr(1) // 2
            override amount: u64 = 0;
        "#]],
    );
}

#[test]
pub fn format_comments_in_attrs_on_compound_statement() {
    check_comments(
        "
        fn main() {
            ## @attr(0) ## @attr(1) ## { ## }
            ## if ## true ## @attr(0) ## @attr(1) ## { ## }
        }
        ",
        expect![[r#"
            fn main() {
                /* 0 */
                @attr(0) /* 1 */
                @attr(1) /* 2 */
                {
                    /* 3 */
                }
                /* 4 */
                if /* 5 */ true /* 6 */ @attr(0) /* 7 */
                @attr(1) /* 8 */
                {
                    /* 9 */
                }
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                @attr(0) // 1
                @attr(1) // 2
                {
                    // 3
                }
                // 4
                if // 5
                true // 6
                @attr(0) // 7
                @attr(1) // 8
                {
                    // 9
                }
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_attrs_on_if_statement() {
    check_comments(
        "
        fn main() {
            ## @attr(0) ## @attr(1) ## if ## true {}
        }
        ",
        expect![[r#"
            fn main() {
                /* 0 */
                @attr(0) /* 1 */
                @attr(1) /* 2 */
                if /* 3 */ true {}
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                @attr(0) // 1
                @attr(1) // 2
                if // 3
                true {}
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_attrs_on_switch_statement_and_body() {
    check_comments(
        "
        fn main() {
            ## @attr(0) ## @attr(1) ## switch a ## @attr(0) ## @attr(1) ## { ## }
        }
        ",
        expect![[r#"
            fn main() {
                /* 0 */
                @attr(0) /* 1 */
                @attr(1) /* 2 */
                switch a /* 3 */ @attr(0) /* 4 */ @attr(1) /* 5 */ { /* 6 */ }
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                @attr(0) // 1
                @attr(1) // 2
                switch a // 3
                @attr(0) // 4
                @attr(1) // 5
                { // 6
                }
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_attrs_on_loop_statement_and_body() {
    check_comments(
        "
        fn main() {
            ## @attr(0) ## @attr(1) ## loop ## @attr(0) ## @attr(1) ## { ## }
        }
        ",
        expect![[r#"
            fn main() {
                /* 0 */
                @attr(0) /* 1 */
                @attr(1) /* 2 */
                loop /* 3 */ @attr(0) /* 4 */
                @attr(1) /* 5 */
                {
                    /* 6 */
                }
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                @attr(0) // 1
                @attr(1) // 2
                loop // 3
                @attr(0) // 4
                @attr(1) // 5
                {
                    // 6
                }
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_attrs_on_for_statement() {
    check_comments(
        "
        fn main() {
            ## @attr(0) ## @attr(1) ## for ## (var i = 0; i < 10; i++) {}
        }
        ",
        expect![[r#"
            fn main() {
                /* 0 */
                @attr(0) /* 1 */
                @attr(1) /* 2 */
                for /* 3 */ (var i = 0; i < 10; i++) {}
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                @attr(0) // 1
                @attr(1) // 2
                for // 3
                (var i = 0; i < 10; i++) {}
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_attrs_on_loop_continuing_block() {
    check_comments(
        "
        fn main() {
        loop{
        ## continuing ## @attr(0) ## @attr(1) ## { ## }
        }
        }
        ",
        expect![[r#"
            fn main() {
                loop {
                    /* 0 */
                    continuing /* 1 */ @attr(0) /* 2 */
                    @attr(1) /* 3 */
                    {
                        /* 4 */
                    }
                }
            }
        "#]],
        expect![[r#"
            fn main() {
                loop {
                    // 0
                    continuing // 1
                    @attr(0) // 2
                    @attr(1) // 3
                    {
                        // 4
                    }
                }
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_attrs_on_while_statement() {
    check_comments(
        "
        fn main() {
        ## @attr(0) ## @attr(1) ## while ## true {}
        }
        ",
        expect![[r#"
            fn main() {
                /* 0 */
                @attr(0) /* 1 */
                @attr(1) /* 2 */
                while /* 3 */ true {}
            }
        "#]],
        expect![[r#"
            fn main() {
                // 0
                @attr(0) // 1
                @attr(1) // 2
                while // 3
                true {}
            }
        "#]],
    );
}

#[test]
pub fn format_comments_in_interpolate_attr() {
    check_comments(
        "
        ## @ ## interpolate ## ( ## flat ## , ## either ## ) ##
        override a: usize = 0;
        ",
        expect![[r#"
            /* 0 */
            @ /* 1 */ interpolate /* 2 */ ( /* 3 */ flat /* 4 */ , /* 5 */ either /* 6 */ ) /* 7 */
            override a: usize = 0;
        "#]],
        expect![[r#"
            // 0
            @ // 1
            interpolate // 2
            ( // 3
            flat // 4
            , // 5
            either // 6
            ) // 7
            override a: usize = 0;
        "#]],
    );
}
