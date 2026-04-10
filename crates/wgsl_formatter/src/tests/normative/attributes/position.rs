use expect_test::expect;

use crate::test_util::{assert_out_of_scope, check, check_comments};

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

//TODO(MonaMayrhofer) For now these tests below just check for something so that attributes are handled and
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
