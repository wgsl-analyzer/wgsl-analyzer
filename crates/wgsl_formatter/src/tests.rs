#![expect(clippy::print_stderr, reason = "useful in tests")]
#![expect(clippy::print_stdout, reason = "useful in tests")]
#![expect(clippy::use_debug, reason = "useful in tests")]

use std::{ffi::OsString, panic, path::Path};

use expect_test::{Expect, ExpectFile, expect, expect_file};

use crate::{FormattingOptions, format_str};

trait ExpectAssertEq {
    fn assert_eq(
        &self,
        other: &str,
    );
}

impl ExpectAssertEq for Expect {
    fn assert_eq(
        &self,
        other: &str,
    ) {
        self.assert_eq(other);
    }
}

impl ExpectAssertEq for ExpectFile {
    fn assert_eq(
        &self,
        other: &str,
    ) {
        self.assert_eq(other);
    }
}

#[expect(clippy::needless_pass_by_value, reason = "intentional API")]
fn check(
    before: &str,
    after: impl ExpectAssertEq,
) {
    check_with_options(before, &after, &FormattingOptions::default());
}

#[expect(clippy::needless_pass_by_value, reason = "intentional API")]
fn check_tabs(
    before: &str,
    after: impl ExpectAssertEq,
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
    after: &impl ExpectAssertEq,
    options: &FormattingOptions,
) {
    let new = format_str(before, options);

    after.assert_eq(&new);

    // Check for idempotence
    let syntax = syntax::parse(new.trim_start()).syntax().clone_for_update();
    let new_second = format_str(before, options);
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
#[ignore = "formatter is known to be broken"]
fn format_fn_header_comma_oneline() {
    check(
        "fn main(a: b , c: d ,)  -> f32   {}",
        expect![["fn main(a: b, c: d) -> f32 {}"]],
    );
}

#[test]
#[ignore = "formatter is known to be broken"]
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
#[ignore = "formatter is known to be broken"]
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
#[ignore = "formatter is known to be broken"]
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
#[ignore = "formatter is known to be broken"]
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
#[ignore = "formatter is known to be broken"]
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
#[ignore = "formatter is known to be broken"]
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
#[ignore = "formatter is known to be broken"]
fn format_expression_bitcast() {
    check(
        "fn main() { bitcast   <  vec4<u32>  >  ( x+5 ) }",
        expect!["fn main() { bitcast<vec4<u32>>(x + 5) }"],
    );
}

#[test]
#[ignore = "formatter is known to be broken"]
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
#[ignore = "formatter is known to be broken"]
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
fn smoke_tests() {
    let source_path = Path::new("./tests/smoke_tests/source");
    let output_path = Path::new("./tests/smoke_tests/output");

    //At this stage, the old formatter is not idempotent on all the tests.
    // But we want to achieve feature parity first, so we focus on the cases
    // where the old formatter works first, and generate tests that the new formatter works with.
    let skip = [OsString::from("old_formatter_idempotence.wgsl")];

    for entry in std::fs::read_dir(source_path).expect("smoke test source directory should exist") {
        let entry = entry.expect("smoke test source directory should be traversable");
        let smoke_test_source_path = entry.path();
        if smoke_test_source_path.is_file() {
            let smoke_test_name = smoke_test_source_path
                .file_name()
                .expect("smoke_test source should be a normal file.");

            if skip.iter().any(|it| it == smoke_test_name) {
                continue;
            }

            let smoke_test_output_path = std::env::current_dir()
                .unwrap()
                .join(output_path)
                .join(smoke_test_name);

            let source = std::fs::read_to_string(smoke_test_source_path)
                .expect("source file should be a readable text file.");

            check(&source, expect_file![smoke_test_output_path]);
        } else {
            panic!("Expected smoke_test directory to not have subdirectories.")
        }
    }
}
