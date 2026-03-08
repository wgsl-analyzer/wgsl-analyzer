#![expect(clippy::print_stderr, reason = "useful in tests")]
#![expect(clippy::print_stdout, reason = "useful in tests")]
#![expect(clippy::use_debug, reason = "useful in tests")]

use std::panic;

use expect_test::{Expect, expect};

use crate::{FormattingOptions, format_recursive};

#[expect(clippy::needless_pass_by_value, reason = "intentional API")]
fn check(
    before: &str,
    after: Expect,
) {
    check_with_options(before, &after, &FormattingOptions::default());
}

#[expect(clippy::needless_pass_by_value, reason = "intentional API")]
fn check_tabs(
    before: &str,
    after: Expect,
) {
    let options = FormattingOptions {
        indent_symbol: "\t".to_owned(),
        ..Default::default()
    };
    check_with_options(before, &after, &options);
}

#[track_caller]
fn check_with_options(
    before: &str,
    after: &Expect,
    options: &FormattingOptions,
) {
    let syntax = syntax::parse(before.trim_start(), parser::Edition::Wgsl)
        .syntax()
        .clone_for_update();
    format_recursive(&syntax, options);
    eprintln!("{syntax:#?}");

    let new = syntax.to_string();
    after.assert_eq(&new);

    // Check for idempotence
    let syntax = syntax::parse(new.trim_start(), parser::Edition::Wgsl)
        .syntax()
        .clone_for_update();
    format_recursive(&syntax, options);
    let new_second = syntax.to_string();
    let diff = dissimilar::diff(&new, &new_second);
    let position = panic::Location::caller();
    if new == new_second {
        return;
    }
    println!(
        "\n
\x1b[1m\x1b[91merror\x1b[97m: Formatting Idempotence check failed\x1b[0m
\x1b[1m\x1b[34m-->\x1b[0m {position}
\x1b[1mExpect\x1b[0m:
----
{new}
----

\x1b[1mActual\x1b[0m:
----
{new_second}
----

\x1b[1mDiff\x1b[0m:
----
{}
----
",
        format_chunks(diff)
    );
    // Use resume_unwind instead of panic!() to prevent a backtrace, which is unnecessary noise.
    panic::resume_unwind(Box::new(()));
}

fn format_chunks(chunks: Vec<dissimilar::Chunk<'_>>) -> String {
    let mut buf = String::new();
    for chunk in chunks {
        let formatted = match chunk {
            dissimilar::Chunk::Equal(text) => text.into(),
            dissimilar::Chunk::Delete(text) => format!("\x1b[41m{text}\x1b[0m"),
            dissimilar::Chunk::Insert(text) => format!("\x1b[42m{text}\x1b[0m"),
        };
        buf.push_str(&formatted);
    }
    buf
}

#[test]
fn format_empty() {
    check("", expect![[""]]);
}

#[test]
fn format_fn_header() {
    check(
        "fn  main ( a :  b )  -> f32   {}",
        expect![["fn main(a: b) -> f32 {}"]],
    );
}

#[test]
fn format_fn_header_2() {
    check(
        "fn  main ( a :  b,  c : d )  -> f32   {}",
        expect![["fn main(a: b, c: d) -> f32 {}"]],
    );
}

#[test]
fn format_fn_header_comma_oneline() {
    check(
        "fn main(a: b , c: d ,)  -> f32   {}",
        expect![["fn main(a: b, c: d) -> f32 {}"]],
    );
}

#[test]
fn format_fn_header_comma_multiline() {
    check(
        "fn main(
                a: b , c: d ,)  -> f32   {}",
        expect![["
            fn main(
                a: b, c: d,
            ) -> f32 {}"]],
    );
}

#[test]
fn format_fn_header_missing_comma() {
    check(
        "fn main(a: b  c: d) {}",
        expect![["fn main(a: b, c: d) {}"]],
    );
}

#[test]
fn format_fn_header_no_ws() {
    check("fn main(a:b)->f32{}", expect![["fn main(a: b) -> f32 {}"]]);
}

#[test]
fn format_fn_newline() {
    check(
        "fn main(
    a:b
)->f32{}",
        expect![["
            fn main(
                a: b
            ) -> f32 {}"]],
    );
}

#[test]
fn format_fn_newline_2() {
    check(
        "fn main(
    a:b, c:d)->f32{}",
        expect![["
            fn main(
                a: b, c: d
            ) -> f32 {}"]],
    );
}

#[test]
fn format_fn_newline_3() {
    check(
        "fn main(
    a:b,
    c:d
)->f32{}",
        expect![["
            fn main(
                a: b,
                c: d
            ) -> f32 {}"]],
    );
}

#[test]
fn format_multiple_fns() {
    check(
        "
 fn  main( a:  b )  -> f32   {}
  fn  main( a:  b )  -> f32   {}
",
        expect![["
                fn main(a: b) -> f32 {}
                fn main(a: b) -> f32 {}
            "]],
    );
}

#[test]
fn format_struct() {
    check(
        "
 struct  Test  {}
",
        expect![["
                struct Test {}
            "]],
    );
}

#[test]
fn format_struct_body() {
    check(
        "
        struct  Test
        {  @location(0) x: i32,                    a: i32,
        b: f32,

                }",
        expect![["
            struct Test {
                @location(0) x: i32,
                a: i32,
                b: f32,
            }"]],
    );
}

#[test]
fn format_bevy_function() {
    check(
        "fn directional_light(light: DirectionalLight, roughness: f32, NdotV: f32, normal: vec3<f32>, view: vec3<f32>, R: vec3<f32>, F0: vec3<f32>, diffuseColor: vec3<f32>) -> vec3<f32> {}",
        expect![[
            "fn directional_light(light: DirectionalLight, roughness: f32, NdotV: f32, normal: vec3<f32>, view: vec3<f32>, R: vec3<f32>, F0: vec3<f32>, diffuseColor: vec3<f32>) -> vec3<f32> {}"
        ]],
    );
}

#[test]
fn format_bevy_function_2() {
    check(
        "fn specular(f0: vec3<f32>, roughness: f32, h: vec3<f32>, NoV: f32, NoL: f32,
              NoH: f32, LoH: f32, specularIntensity: f32) -> vec3<f32> {",
        expect![["
                fn specular(f0: vec3<f32>, roughness: f32, h: vec3<f32>, NoV: f32, NoL: f32,
                    NoH: f32, LoH: f32, specularIntensity: f32) -> vec3<f32> {"]],
    );
}

#[test]
fn format_if() {
    check(
        "fn main() {
    if(x < 1){}
    if  (  x < 1   )  {}
}",
        expect![["
            fn main() {
                if x < 1 {}
                if x < 1 {}
            }"]],
    );
}

#[test]
fn format_if_2() {
    check(
        "fn main() {
    if(x < 1){}
    else{
        let a = 3;
    }else     if(  x > 2 ){}
}",
        expect![["
            fn main() {
                if x < 1 {} else {
                    let a = 3;
                } else if x > 2 {}
            }"]],
    );
}

#[test]
fn format_for() {
    check(
        "fn main() {
    for( var i = 0;i < 100;   i = i + 1  ){}
}",
        expect![["
                fn main() {
                    for (var i = 0; i < 100; i = i + 1) {}
                }"]],
    );
}

#[test]
fn format_while() {
    check(
        "fn main() {
        while(x < 1){}
        while  (  x < 1   )  {}
    }",
        expect![["
            fn main() {
                while x < 1 {}
                while x < 1 {}
            }"]],
    );
}

#[test]
fn format_function_call() {
    check(
        "fn main() {
    min  (  x,y );
}",
        expect![["
                fn main() {
                    min(x, y);
                }"]],
    );
}

#[test]
fn format_function_call_newline() {
    check(
        "fn main() {
    min  (
        x,y );
}",
        expect![["
            fn main() {
                min(
                    x, y
                );
            }"]],
    );
}

#[test]
fn format_function_call_newline_indent() {
    check(
        "fn main() {
    if (false) {
        min  (
            x,y );
    }
}",
        expect![["
            fn main() {
                if false {
                    min(
                        x, y
                    );
                }
            }"]],
    );
}

#[test]
fn format_function_call_newline_nested() {
    check(
        "fn main() {
    min(
        min(
            1,
            2,
        )
    )
}",
        expect![["
            fn main() {
                min(
                    min(
                        1,
                        2,
                    )
                )
            }"]],
    );
}

#[test]
fn format_function_call_2() {
    check(
        "fn main() {
    vec3  <f32>  (  x,y,z );
}",
        expect![["
                fn main() {
                    vec3<f32>(x, y, z);
                }"]],
    );
}

#[test]
fn format_infix_expression() {
    check(
        "fn main() {
    let a=x+y*z;
}",
        expect![["
            fn main() {
                let a = x + y * z;
            }"]],
    );
}

#[test]
fn format_assignment() {
    check(
        "fn main() {
    x=0;
    y  +=  x + y;
}",
        expect![["
                fn main() {
                    x = 0;
                    y += x + y;
                }"]],
    );
}

#[test]
fn format_variable() {
    check(
        "fn main() {
    var x=0;
}",
        expect![["
                fn main() {
                    var x = 0;
                }"]],
    );
}

#[test]
fn format_statement_indent() {
    check(
        "fn main() {
var x=0;
}",
        expect![["
                fn main() {
                    var x = 0;
                }"]],
    );
}

#[test]
fn format_variable_type() {
    check(
        "fn main() {var x   : u32=0;}",
        expect!["fn main() {var x: u32 = 0;}"],
    );
}

#[test]
fn format_statement_indent_nested() {
    check(
        "fn main() {
for() {
if(y) {
var x = 0;
}
}
}",
        expect![["
            fn main() {
                for () {
                    if y {
                        var x = 0;
                    }
                }
            }"]],
    );
}

#[test]
fn format_statements_newline() {
    check(
        "fn main() {
let x = 3;

let y = 4;
}",
        expect![["
            fn main() {
                let x = 3;

                let y = 4;
            }"]],
    );
}

#[test]
fn format_expression_shift_right() {
    check(
        "fn main() { let x = 1u >> 3u; }",
        expect![["fn main() { let x = 1u >> 3u; }"]],
    );
}

#[test]
fn format_expression_shift_left() {
    check(
        "fn main() { let x = 1u << 3u; }",
        expect![["fn main() { let x = 1u << 3u; }"]],
    );
}

#[test]
fn format_expression_bitcast() {
    check(
        "fn main() { bitcast   <  vec4<u32>  >  ( x+5 ) }",
        expect!["fn main() { bitcast<vec4<u32>>(x + 5) }"],
    );
}

#[test]
fn leave_matrix_alone() {
    check(
        "
fn main() {
    let x = mat3x3(
        cosR,  0.0, sinR,
        0.0, 1.0, 0.0,
        -sinR, 0.0, cosR,
    );
}",
        expect![["
            fn main() {
                let x = mat3x3(
                    cosR, 0.0, sinR,
                    0.0, 1.0, 0.0,
                    -sinR, 0.0, cosR,
                );
            }"]],
    );
}

#[test]
fn leave_matrix_alone_tabs() {
    check_tabs(
        "
fn main() {
	let x = mat3x3(
		cosR,  0.0, sinR,
		0.0, 1.0, 0.0,
		-sinR, 0.0, cosR,
	);
}",
        expect![["
			fn main() {
				let x = mat3x3(
					cosR, 0.0, sinR,
					0.0, 1.0, 0.0,
					-sinR, 0.0, cosR,
				);
			}"]],
    );
}

#[test]
fn format_compound_assignment_with_comment() {
    // A block comment between LHS and operator should be preserved,
    // with spaces around both the comment and the operator.
    check(
        "fn main() {
    a/*c*/+=1;
}",
        expect![["
            fn main() {
                a /*c*/ += 1;
            }"]],
    );
}

#[test]
fn format_param_comment_before_comma() {
    // A comment between a parameter and its comma should not cause
    // the formatter to insert a duplicate comma.
    check(
        "fn main(a: f32 /* comment */ , b: f32) {}",
        expect![["fn main(a: f32/* comment */, b: f32) {}"]],
    );
}

// ── Edge-case tests ──

#[test]
fn format_all_compound_assignment_operators() {
    check(
        "fn main() { x+=1; y-=2; z*=3; w/=4; a%=5; b&=6; c|=7; d^=8; }",
        expect![["fn main() { x += 1; y -= 2; z *= 3; w /= 4; a %= 5; b &= 6; c |= 7; d ^= 8; }"]],
    );
}

#[test]
fn format_shift_compound_assignment() {
    check(
        "fn main() { x<<=1u; y>>=2u; }",
        expect![["fn main() { x <<= 1u; y >>= 2u; }"]],
    );
}

#[test]
fn format_override_declarations() {
    check("override x:u32=64;", expect![["override x: u32 = 64;"]]);
}

#[test]
fn format_override_no_value() {
    check("override y  :  f32;", expect![["override y: f32;"]]);
}

#[test]
fn format_const_declaration() {
    check("const PI:f32=3.14;", expect![["const PI: f32 = 3.14;"]]);
}

#[test]
fn format_type_alias_spacing() {
    check(
        "alias Mat4=mat4x4<f32>;",
        expect![["alias Mat4 = mat4x4<f32>;"]],
    );
}

#[test]
fn format_global_var_uniform() {
    check(
        "var<uniform>camera:Camera;",
        expect![["var<uniform> camera: Camera;"]],
    );
}

#[test]
fn format_nested_function_calls() {
    check(
        "fn main() { let x=min(max(a,b),clamp(c,0.0,1.0)); }",
        expect![["fn main() { let x = min(max(a, b), clamp(c, 0.0, 1.0)); }"]],
    );
}

#[test]
fn format_parenthesized_expression() {
    check(
        "fn main() { let x=(a+b)*(c+d); }",
        expect![["fn main() { let x = (a + b) * (c + d); }"]],
    );
}

#[test]
fn format_nested_templates() {
    check(
        "fn main() { let x:array<vec3<f32>,4>=array<vec3<f32>,4>(); }",
        expect![["fn main() { let x: array<vec3<f32>,4> = array<vec3<f32>,4>(); }"]],
    );
}

#[test]
fn format_if_else_chain_single_line() {
    check(
        "fn main() { if(x<0){return -1;}else if(x>0){return 1;}else{return 0;} }",
        expect![["fn main() { if x < 0 {return -1;} else if x > 0 {return 1;} else {return 0;} }"]],
    );
}

#[test]
fn format_while_paren_removal() {
    check(
        "fn main() { while(i<10){i+=1;} }",
        expect![["fn main() { while i < 10 {i += 1;} }"]],
    );
}

#[test]
fn format_for_no_spaces() {
    check(
        "fn main() { for(var i=0u;i<10u;i+=1u){} }",
        expect![["fn main() { for (var i = 0u; i < 10u; i += 1u) {} }"]],
    );
}

#[test]
fn format_empty_struct() {
    check("struct Empty {}", expect![["struct Empty {}"]]);
}

#[test]
fn format_struct_trailing_semicolon() {
    check(
        "
struct A {
    x: f32,
};",
        expect![["
            struct A {
                x: f32,
            };"]],
    );
}

#[test]
fn format_struct_member_bad_spacing() {
    check(
        "
struct S {
    x  :  f32  ,
    y  :  u32  ,
}",
        expect![["
            struct S {
                x: f32,
                y: u32,
            }"]],
    );
}

#[test]
fn format_deeply_nested() {
    check(
        "
fn main() {
    if true {
        if true {
            for(var i=0;i<10;i+=1) {
                let x=1;
            }
        }
    }
}",
        expect![["
            fn main() {
                if true {
                    if true {
                        for (var i = 0; i < 10; i += 1) {
                            let x = 1;
                        }
                    }
                }
            }"]],
    );
}

#[test]
fn format_pointer_params() {
    check(
        "fn inc(p:ptr<function,f32>) { *p=*p+1.0; }",
        expect![["fn inc(p: ptr<function,f32>) { *p = *p + 1.0; }"]],
    );
}

#[test]
fn format_return_expression() {
    check(
        "fn main() -> f32 { return  x+y; }",
        expect![["fn main() -> f32 { return x + y; }"]],
    );
}

#[test]
fn format_multiple_statements_blank_line() {
    check(
        "
fn main() {
    let x = 1;

    let y = 2;

    let z = 3;
}",
        expect![["
            fn main() {
                let x = 1;

                let y = 2;

                let z = 3;
            }"]],
    );
}

#[test]
fn format_bitcast_nested() {
    check(
        "fn main() { bitcast<vec4<u32>>(bitcast<u32>(x)); }",
        expect![["fn main() { bitcast<vec4<u32>>(bitcast<u32>(x)); }"]],
    );
}

#[test]
fn format_function_call_many_args() {
    check(
        "fn main() { foo(a,b,c,d,e,f); }",
        expect![["fn main() { foo(a, b, c, d, e, f); }"]],
    );
}

#[test]
fn format_function_call_multiline_trailing_comma() {
    check(
        "fn main() {
    foo(
        a,
        b,
        c,
    );
}",
        expect![["
            fn main() {
                foo(
                    a,
                    b,
                    c,
                );
            }"]],
    );
}

#[test]
fn format_override_extra_spaces() {
    check("override   x:u32=64;", expect![["override x: u32 = 64;"]]);
}

#[test]
fn format_alias_extra_spaces() {
    check(
        "alias   Vec3F=vec3<f32>;",
        expect![["alias Vec3F = vec3<f32>;"]],
    );
}

#[test]
fn format_const_assert_spacing() {
    check(
        "const_assert  MAX<=128u;",
        expect![["const_assert MAX <= 128u;"]],
    );
}

#[test]
fn format_switch_spacing() {
    check(
        "fn main() { switch(x){ case 0u:{return 0u;} default:{return 1u;} } }",
        expect!["fn main() { switch (x) { case 0u: {return 0u;} default: {return 1u;} } }"],
    );
}

#[test]
fn format_loop_brace_spacing() {
    check(
        "fn main() { loop{ x += 1; } }",
        expect![["fn main() { loop { x += 1; } }"]],
    );
}

#[test]
fn format_continuing_brace_spacing() {
    check(
        "fn main() { loop { continuing{ x += 1; } } }",
        expect![["fn main() { loop { continuing { x += 1; } } }"]],
    );
}

#[test]
fn format_loop_continuing_combined() {
    check(
        "fn main() { loop{ continuing{ x+=1; } } }",
        expect![["fn main() { loop { continuing { x += 1; } } }"]],
    );
}

#[test]
fn format_switch_multiline() {
    check(
        "
fn main() {
    switch(x){
        case 0u:{
            return 0u;
        }
        default:{
            return 1u;
        }
    }
}",
        expect![[r#"
            fn main() {
                switch (x) {
                    case 0u: {
                        return 0u;
                    }
                    default: {
                        return 1u;
                    }
                }
            }"#]],
    );
}

#[test]
fn format_loop_multiline() {
    check(
        "
fn main() {
    loop{
        if true {break;}
        continuing{
            x+=1;
        }
    }
}",
        expect![["
            fn main() {
                loop {
                    if true {break;}
                    continuing {
                        x += 1;
                    }
                }
            }"]],
    );
}

#[test]
fn format_var_extra_spaces() {
    check(
        "fn main() { var   x = 1; }",
        expect![["fn main() { var x = 1; }"]],
    );
}

#[test]
fn format_let_extra_spaces() {
    check(
        "fn main() { let   x = 1; }",
        expect![["fn main() { let x = 1; }"]],
    );
}

#[test]
fn format_const_extra_spaces() {
    check(
        "fn main() { const   x = 1; }",
        expect![["fn main() { const x = 1; }"]],
    );
}

#[test]
fn format_var_template_no_space_before_angle() {
    check(
        "var  <uniform>camera: Camera;",
        expect![["var<uniform> camera: Camera;"]],
    );
}

#[test]
fn format_attr_space_between_attrs() {
    check(
        "@group(0)@binding(1) var<storage> data: array<f32>;",
        expect![["@group(0) @binding(1) var<storage> data: array<f32>;"]],
    );
}

#[test]
fn format_attr_space_before_fn() {
    check(
        "@vertex fn vs() -> vec4<f32> { return vec4<f32>(0.0); }",
        expect![["@vertex fn vs() -> vec4<f32> { return vec4<f32>(0.0); }"]],
    );
}

#[test]
fn format_attr_space_before_fn_paren() {
    check(
        "@compute @workgroup_size(64)fn cs_main() {}",
        expect![["@compute @workgroup_size(64) fn cs_main() {}"]],
    );
}

#[test]
fn format_attr_space_before_type() {
    check(
        "fn vs() -> @builtin(position)vec4<f32> { return vec4<f32>(0.0); }",
        expect![["fn vs() -> @builtin(position) vec4<f32> { return vec4<f32>(0.0); }"]],
    );
}

#[test]
fn format_attr_space_before_override() {
    check(
        "@id(1)override threads: u32 = 64;",
        expect![["@id(1) override threads: u32 = 64;"]],
    );
}

#[test]
fn format_phony_assignment_spacing() {
    check("fn main() { _=2; }", expect![["fn main() { _ = 2; }"]]);
}

#[test]
fn format_bare_return_no_space() {
    check("fn main() { return; }", expect![["fn main() { return; }"]]);
}

#[test]
fn format_return_with_expr_spacing() {
    check(
        "fn main() { return  42; }",
        expect![["fn main() { return 42; }"]],
    );
}

#[test]
fn format_nested_blocks() {
    check(
        "
fn nested() {
    {
        {
            var x = 1;
        }
    }
}",
        expect![["
            fn nested() {
                {
                    {
                        var x = 1;
                    }
                }
            }"]],
    );
}

#[test]
fn format_semicolon_spacing() {
    check(
        "fn main() { var x: f32 = 0.0  ; let y = 1u  ; x += 1.0  ; return  vec4<f32>(0.0)  ; }",
        expect![["fn main() { var x: f32 = 0.0; let y = 1u; x += 1.0; return vec4<f32>(0.0); }"]],
    );
}

#[test]
fn format_switch_case_spacing() {
    check(
        "fn a() { switch z { case  0u  :  { break; } case  1u, 2u  :  { x = 1.0; } default  :  { break; } } }",
        expect![[
            "fn a() { switch z { case 0u: { break; } case 1u, 2u: { x = 1.0; } default: { break; } } }"
        ]],
    );
}

#[test]
fn format_empty_struct_extra_spaces() {
    check("struct  Empty  {  }", expect![["struct Empty {}"]]);
}

#[test]
fn format_increment_decrement() {
    check(
        "fn a() { var x = 0; x  ++; x  --; }",
        expect![["fn a() { var x = 0; x++; x--; }"]],
    );
}
