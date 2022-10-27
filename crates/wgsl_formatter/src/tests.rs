use crate::{format_recursive, FormattingOptions};
use expect_test::{expect, Expect};

fn check(before: &str, after: Expect) {
    check_with_options(before, after, &FormattingOptions::default())
}
fn check_with_options(before: &str, after: Expect, options: &FormattingOptions) {
    let syntax = syntax::parse(before.trim_start())
        .syntax()
        .clone_for_update();
    format_recursive(syntax.clone(), options);

    eprintln!("{:#?}", syntax);

    let new = syntax.to_string();
    after.assert_eq(&new);
}

#[test]
fn format_empty() {
    check("", expect![[""]]);
}

#[test]
fn format_fn_header() {
    check(
        "fn  main ( a :  b )  -> f32   {}",
        expect![[r#"fn main(a: b) -> f32 {}"#]],
    );
}

#[test]
fn format_fn_header_2() {
    check(
        "fn  main ( a :  b,  c : d )  -> f32   {}",
        expect![[r#"fn main(a: b, c: d) -> f32 {}"#]],
    );
}

#[test]
fn format_fn_header_comma_oneline() {
    check(
        "fn main(a: b , c: d ,)  -> f32   {}",
        expect![[r#"fn main(a: b, c: d) -> f32 {}"#]],
    );
}
#[test]
fn format_fn_header_comma_multiline() {
    check(
        "fn main(
                a: b , c: d ,)  -> f32   {}",
        expect![[r#"
            fn main(
                a: b,
                c: d,
            ) -> f32 {}"#]],
    );
}

#[test]
fn format_fn_header_missing_comma() {
    check(
        "fn main(a: b  c: d) {}",
        expect![[r#"fn main(a: b, c: d) {}"#]],
    );
}

#[test]
fn format_fn_header_no_ws() {
    check(
        "fn main(a:b)->f32{}",
        expect![[r#"fn main(a: b) -> f32 {}"#]],
    );
}

#[test]
fn format_fn_newline() {
    check(
        "fn main(
    a:b
)->f32{}",
        expect![[r#"
            fn main(
                a: b
            ) -> f32 {}"#]],
    );
}

#[test]
fn format_fn_newline_2() {
    check(
        "fn main(
    a:b, c:d)->f32{}",
        expect![[r#"
            fn main(
                a: b,
                c: d
            ) -> f32 {}"#]],
    );
}

#[test]
fn format_fn_newline_3() {
    check(
        "fn main(
    a:b,
    c:d
)->f32{}",
        expect![[r#"
            fn main(
                a: b,
                c: d
            ) -> f32 {}"#]],
    );
}

#[test]
fn format_multiple_fns() {
    check(
        "
 fn  main( a:  b )  -> f32   {}
  fn  main( a:  b )  -> f32   {}
",
        expect![[r#"
                fn main(a: b) -> f32 {}
                fn main(a: b) -> f32 {}
            "#]],
    );
}

#[test]
fn format_struct() {
    check(
        "
 struct  Test  {}
",
        expect![[r#"
                struct Test {}
            "#]],
    );
}

#[test]
fn format_bevy_function() {
    check(
            "fn directional_light(light: DirectionalLight, roughness: f32, NdotV: f32, normal: vec3<f32>, view: vec3<f32>, R: vec3<f32>, F0: vec3<f32>, diffuseColor: vec3<f32>) -> vec3<f32> {}",
            expect![["fn directional_light(light: DirectionalLight, roughness: f32, NdotV: f32, normal: vec3<f32>, view: vec3<f32>, R: vec3<f32>, F0: vec3<f32>, diffuseColor: vec3<f32>) -> vec3<f32> {}"]],
        )
}

#[test]
fn format_bevy_function_2() {
    check(
            "fn specular(f0: vec3<f32>, roughness: f32, h: vec3<f32>, NoV: f32, NoL: f32,
              NoH: f32, LoH: f32, specularIntensity: f32) -> vec3<f32> {",
            expect![["fn specular(f0: vec3<f32>, roughness: f32, h: vec3<f32>, NoV: f32, NoL: f32, NoH: f32, LoH: f32, specularIntensity: f32) -> vec3<f32> {"]],
        )
}

#[test]
fn format_if() {
    check(
        "fn main() {
    if(x < 1){}
    if  (  x < 1   )  {}
}",
        expect![[r#"
                fn main() {
                    if (x < 1) {}
                    if (x < 1) {}
                }"#]],
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
        expect![[r#"
                fn main() {
                    if (x < 1) {} else {
                        let a = 3;
                    } else if (x > 2) {}
                }"#]],
    );
}

#[test]
fn format_for() {
    check(
        "fn main() {
    for( var i = 0;i < 100;   i = i + 1  ){}
}",
        expect![[r#"
                fn main() {
                    for (var i = 0; i < 100; i = i + 1) {}
                }"#]],
    );
}

#[test]
fn format_while() {
    check(
        "fn main() {
        while(x < 1){}
        while  (  x < 1   )  {}
    }",
        expect![[r#"
            fn main() {
                while (x < 1) {}
                while (x < 1) {}
            }"#]],
    );
}

#[test]
fn format_function_call() {
    check(
        "fn main() {
    min  (  x,y );
}",
        expect![[r#"
                fn main() {
                    min(x, y);
                }"#]],
    );
}

#[test]
fn format_function_call_newline() {
    check(
        "fn main() {
    min  (  
        x,y );
}",
        expect![[r#"
            fn main() {
                min(
                    x,
                    y
                );
            }"#]],
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
        expect![[r#"
            fn main() {
                if (false) {
                    min(
                        x,
                        y
                    );
                }
            }"#]],
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
        expect![[r#"
            fn main() {
                min(
                    min(
                        1,
                        2,
                    )
                )
            }"#]],
    );
}

#[test]
fn format_function_call_2() {
    check(
        "fn main() {
    vec3  <f32>  (  x,y,z );
}",
        expect![[r#"
                fn main() {
                    vec3<f32>(x, y, z);
                }"#]],
    );
}

#[test]
fn format_infix_expr() {
    check(
        "fn main() {
    x+y*z;
}",
        expect![[r#"
                fn main() {
                    x + y * z;
                }"#]],
    );
}

#[test]
fn format_assignment() {
    check(
        "fn main() {
    x=0;
    y  +=  x + y;
}",
        expect![[r#"
                fn main() {
                    x = 0;
                    y += x + y;
                }"#]],
    );
}

#[test]
fn format_variable() {
    check(
        "fn main() {
    var x=0;
}",
        expect![[r#"
                fn main() {
                    var x = 0;
                }"#]],
    );
}

#[test]
fn format_statement_indent() {
    check(
        "fn main() {
var x=0;
}",
        expect![[r#"
                fn main() {
                    var x = 0;
                }"#]],
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
        expect![[r#"
                fn main() {
                    for () {
                        if (y) {
                            var x = 0;
                        }
                    }
                }"#]],
    );
}

#[test]
fn format_statements_newline() {
    check(
        "fn main() {
let x = 3;

let y = 4;
}",
        expect![[r#"
            fn main() {
                let x = 3;
            
                let y = 4;
            }"#]],
    );
}

#[test]
fn format_expr_shift_right() {
    check(
        "fn main() { let x = 1u >> 3u; }",
        expect![[r#"fn main() { let x = 1u >> 3u; }"#]],
    );
}

#[test]
fn format_expr_shift_left() {
    check(
        "fn main() { let x = 1u << 3u; }",
        expect![[r#"fn main() { let x = 1u << 3u; }"#]],
    );
}

#[test]
fn format_expr_bitcast() {
    check(
        "fn main() { bitcast   <  vec4<u32>  >  ( x+5 ) }",
        expect!["fn main() { bitcast<vec4<u32>>(x + 5) }"],
    );
}
