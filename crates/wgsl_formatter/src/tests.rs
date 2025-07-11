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
    let syntax = syntax::parse(before.trim_start())
        .syntax()
        .clone_for_update();
    format_recursive(&syntax, options);
    eprintln!("{syntax:#?}");

    let new = syntax.to_string();
    after.assert_eq(&new);

    // Check for idempotence
    let syntax = syntax::parse(new.trim_start()).syntax().clone_for_update();
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
    x+y*z;
}",
        expect![["
                fn main() {
                    x + y * z;
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
