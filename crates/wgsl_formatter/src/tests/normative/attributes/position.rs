use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check};

#[test]
pub fn format_attribute_offset_size_align_are_grouped() {
    // @align @size @offset should not be on the same line as the field,
    // but on the line beforehand (on the same line, but on separate lines to other fields)
    //
    // They are on separate lines to be consistent if all 3 are given, and if all 3 are given, putting
    // them on the same line as the field would threaten to get pretty long, make git diffs weird and
    // pose questions about which attribute group should be privileged enough to share the line with the field.
    check(
        "struct VertexOutput {
            @align(7)
            @size(9)
            @offset(28)
            a: u32,
            @align(7)
            @location(1)
            @size(9)
            @offset(28)
            b: u32,
        }",
        expect![[r#"
            struct VertexOutput {
                @offset(28) @align(7) @size(9)
                a: u32,
                @location(1)
                @offset(28) @align(7) @size(9)
                b: u32,
            }
        "#]],
    );
}

#[test]
pub fn format_attribute_group_binding_are_grouped() {
    // @group @binding should be on the line before the binding,
    // sharing the same line, but on separate lines to other fields
    check(
        "
        @blaa(off)
        @binding(1)
        @group(0)
        var<storage> a: b;
        ",
        expect![[r#"
            @blaa(off)
            @group(0) @binding(1)
            var<storage> a: b;
        "#]],
    );
}

#[test]
pub fn format_attribute_workgroup_size_compute() {
    // @compute @workgroup_size should be on the line before the fn,
    // sharing the same line, but on separate lines to other fields
    //
    // They should be ordered @compute @workgroup_size
    check(
        "
        @workgroup_size(1,1,1)
        @compute
        fn main() {
        }
        ",
        expect![[r#"
            @compute @workgroup_size(1, 1, 1)
            fn main() {}
        "#]],
    );
}

#[test]
pub fn format_attribute_const_is_inline_with_function() {
    // Following the WGSL spec, we keep @const inlined with the function
    check(
        "
        @bla()
        @const
        @blo()
        fn thing() {}
        ",
        expect![[r#"
            @bla()
            @blo()
            @const fn thing() {}
        "#]],
    );
}

#[test]
pub fn format_attribute_must_use_is_inline_with_function() {
    // Following the WGSL spec, we keep @must_use inlined with the function
    check(
        "
        @bla()
        @must_use
        @blo()
        fn thing() {}
        ",
        expect![[r#"
            @bla()
            @blo()
            @must_use fn thing() {}
        "#]],
    );
}

#[test]
pub fn format_attribute_const_must_use_order() {
    // Following the WGSL spec, we order @const before @must_use
    check(
        "
        @must_use
        @const
        fn thing() {}
        ",
        expect![[r#"
            @const @must_use fn thing() {}
        "#]],
    );
}

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
            fn thing() -> @attr(0) @attr(1) vec4<f32> {}
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
pub fn format_attrs_on_function_body_singleline() {
    check(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn thing() -> vec4<f32> @attr(0) @attr(1) @diagnostic(bla, off) {
        }",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn thing() -> vec4<f32> @attr(0) @attr(1) @diagnostic(bla, off) {}
        "#]],
    );
}

#[test]
pub fn format_attrs_on_function_body_multiline() {
    check(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn thing() -> vec4<f32> @aaaaaaaa(3) @bbbbbbb(1) @ccccccccccccc(4,3) @dddddddddd(28) @eeeeeeeeeeee(11,11,11) {
        }",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            fn thing() -> vec4<f32> @aaaaaaaa(3) @bbbbbbb(1) @ccccccccccccc(4, 3)
            @dddddddddd(28) @eeeeeeeeeeee(11, 11, 11) {}
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
                switch a @attr(0) @attr(1) {}
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

//TODO Look at these cases a bit more closely
#[test]
#[ignore = "TODO Parser Error?"]
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
                    continuing @attr(0) @attr(1) {}
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
        @attr(0) @attr(1)
        while true {}
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
pub fn format_attrs_on_import_statement() {
    check(
        "
        @if(THING)
        import the::thing;
        ",
        expect![[r#"
            @if(THING)
            import the::thing;
        "#]],
    );
}
#[test]
pub fn format_all_attribute_order() {
    check(
        "
        @const
        @align(1)
        @binding(1)
        @blend_src(1)
        @builtin(position)
        @group(1)
        @id(1)
        @interpolate(flat)
        @invariant
        @location(0)
        @must_use
        @size(1)
        @workgroup_size(1,2,3)
        @vertex
        @fragment
        @compute
        fn a() {}
        ",
        expect![[r#"
            @blend_src(1)
            @id(1)
            @interpolate(flat)
            @invariant
            @location(0)
            @align(1) @size(1)
            @group(1) @binding(1)
            @compute @workgroup_size(1, 2, 3)
            @fragment
            @vertex
            @const @must_use @builtin(position) fn a() {}
        "#]],
    );
}

#[test]
pub fn format_attribute_suboptimal_comment_positioning_1() {
    // This test exists to demonstrate pretty suboptimal positioning of comments between attributes and functions
    // The issue in question is that comments get placed differently depending on if the attribute is an inline attribute (@must_use) or a
    // non-inline attribute (@fragment).
    //
    // Ideally we could have "// Hello" placed at the same position no matter what attribute is preceding it.
    //
    // This is a tradeoff, because I don't think these cases will occur very frequently, and even if they do, i doubt this will annoy people.
    // Supporting this without breaking other things would introduce a nontrivial amount of complexity into the code
    // because of how the formatter currently works (the comments are attached to the function as trivia, just as the attributes.)
    // A starting point of how to support this would be "trivia on trivia" so that comments can be attached to the attribute as trivia, which in turn
    // is then attached to the function - and as such we can "always put the comment after the attribute" and only after handling the succeeding trivia of the
    // attribute decide "do we need a newline or a space before the item the attribute is attached to".
    //
    // This behavior can be changed for the better.
    check(
        "
        @must_use //Hello
        fn a() {}

        @fragment //Hello
        fn b() {}
        ",
        expect![[r#"
            @must_use //Hello
            fn a() {}

            @fragment
            //Hello
            fn b() {}
        "#]],
    );
}

#[test]
pub fn format_attribute_suboptimal_comment_positioning_2() {
    // This test exists to demonstrate pretty suboptimal positioning of comments between attributes and functions
    // The issue in question is that comments get placed differently depending on if the attribute is an inline attribute (@must_use) or a
    // non-inline attribute (@fragment).
    //
    // Ideally we could have "/* Hello */" placed at the same position no matter what attribute is preceding it.
    // That position would probably be on the same line as the attribute (for consistency), but on a different line than the function (in order
    // to preserve the intent of "there was a newline after the block comment in the source")
    //
    // This is a tradeoff, because I don't think these cases will occur very frequently, and even if they do, i doubt this will annoy people.
    // Supporting this without breaking other things would introduce a nontrivial amount of complexity into the code
    // because of how the formatter currently works (the comments are attached to the function as trivia, just as the attributes.)
    // A starting point of how to support this would be "trivia on trivia" so that comments can be attached to the attribute as trivia, which in turn
    // is then attached to the function - and as such we can "always put the comment after the attribute" and only after handling the succeeding trivia of the
    // attribute decide "do we need a newline or a space before the item the attribute is attached to".
    //
    // This behavior can be changed for the better.
    check(
        "
        @must_use /* Hello */
        fn a() {}

        @fragment /* Hello */
        fn b() {}
        ",
        expect![[r#"
            @must_use /* Hello */
            fn a() {}

            @fragment
            /* Hello */
            fn b() {}
        "#]],
    );
}

#[test]
pub fn format_attribute_comment_positioning() {
    // This test exists to demonstrate optimal behavior of block comments between attributes and functions.
    // In contrast to the suboptimal behavior demonstrated in `format_attribute_suboptimal_comment_positioning_1` and `format_attribute_suboptimal_comment_positioning_2`
    // this behavior seems optimal and consistent with the behavior of comments across the formatter.
    //
    // In this case the comment is attached to the function (and not to the attribute) and as such sticks to the function.
    //
    // This case is the one we will probably see the most in the wild for example in cases where people have an attribute, but temporarily comment it out.
    // In these cases we really do not want the comment shifting around.
    //
    // This behavior should not be changed without good reason.
    check(
        "
        @must_use /* @some_inline_attr */ fn a() {}

        @fragment
        /* @some_inline_attr */ fn b() {}

        @fragment
        /* @some_noninline_attr */
        fn c() {}

        @fragment
        // @some_noninline_attr
        fn d() {}
        ",
        expect![[r#"
            @must_use /* @some_inline_attr */ fn a() {}

            @fragment
            /* @some_inline_attr */ fn b() {}

            @fragment
            /* @some_noninline_attr */
            fn c() {}

            @fragment
            // @some_noninline_attr
            fn d() {}
        "#]],
    );
}
