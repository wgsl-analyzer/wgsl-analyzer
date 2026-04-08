use syntax::{
    AstNode as _, HasName as _, HasTemplateParameters as _, SyntaxKind, SyntaxNode, ast,
    ast::SyntaxToken,
};

use crate::{
    FormattingOptions,
    util::{
        create_whitespace, indent_after, indent_before, is_whitespace_with_newline,
        remove_if_whitespace, remove_token, set_whitespace_after, set_whitespace_before,
        set_whitespace_single_after, set_whitespace_single_before,
        trim_whitespace_before_to_newline, whitespace_to_single_around,
    },
};

/// Formats declaration nodes: functions, structs, variables, let/const/override
/// bindings, parameters, return types, and type aliases.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "intentional catch-all dispatcher"
)]
pub(crate) fn format_declaration(
    syntax: &SyntaxNode,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
    match syntax.kind() {
        SyntaxKind::FunctionDeclaration => format_function_declaration(syntax, options),
        SyntaxKind::Parameter => format_parameter(syntax),
        SyntaxKind::ReturnType => format_return_type(syntax),
        SyntaxKind::StructDeclaration => format_struct_declaration(syntax, indentation, options),
        SyntaxKind::StructMember => format_struct_member(syntax),
        SyntaxKind::VariableDeclaration => format_variable_declaration(syntax),
        SyntaxKind::LetDeclaration => format_let_declaration(syntax),
        SyntaxKind::ConstantDeclaration => format_constant_declaration(syntax),
        SyntaxKind::OverrideDeclaration => format_override_declaration(syntax),
        SyntaxKind::TypeAliasDeclaration => format_type_alias_declaration(syntax),
        _ => None,
    }
}

fn format_function_declaration(
    syntax: &SyntaxNode,
    options: &FormattingOptions,
) -> Option<()> {
    let function = ast::FunctionDeclaration::cast(syntax.clone())?;

    trim_whitespace_before_to_newline(&function.fn_token()?);

    set_whitespace_single_after(&function.fn_token()?);
    set_whitespace_single_before(&function.body()?.left_brace_token()?);

    let param_list = function.parameter_list()?;

    remove_if_whitespace(&param_list.left_parenthesis_token()?.prev_token()?); // spellchecker:disable-line

    let has_newline =
        is_whitespace_with_newline(&param_list.left_parenthesis_token()?.next_token()?);

    super::format_param_list(
        param_list.parameters(),
        param_list.parameters().count(),
        has_newline,
        1,
        options.trailing_commas,
        &options.indent_symbol,
    );

    if has_newline {
        set_whitespace_before(
            &param_list.right_parenthesis_token()?,
            create_whitespace("\n"),
        );
    } else {
        remove_if_whitespace(&param_list.right_parenthesis_token()?.prev_token()?); // spellchecker:disable-line
    }
    Some(())
}

fn format_parameter(syntax: &SyntaxNode) -> Option<()> {
    let item = ast::Parameter::cast(syntax.clone())?;
    super::format_colon(item.colon_token().as_ref());
    Some(())
}

fn format_return_type(syntax: &SyntaxNode) -> Option<()> {
    let return_type = ast::ReturnType::cast(syntax.clone())?;
    whitespace_to_single_around(&return_type.arrow_token()?);
    Some(())
}

fn format_struct_declaration(
    syntax: &SyntaxNode,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
    let r#struct = ast::StructDeclaration::cast(syntax.clone())?;

    trim_whitespace_before_to_newline(&r#struct.struct_token()?);

    let name = r#struct.name()?;
    whitespace_to_single_around(&name.ident_token()?);

    let body = r#struct.body()?;
    let l_brace = body.left_brace_token()?;
    let r_brace = body.right_brace_token()?;
    let has_fields = body
        .fields()
        .any(|field| field.syntax().text_range().len() > 0.into());
    if has_fields {
        indent_after(&l_brace, indentation + 1, options)?;
        for field in body.fields() {
            let first = field.syntax().first_token()?;
            indent_before(&first, indentation + 1, options)?;
        }
        indent_before(&r_brace, indentation, options)?;
    } else {
        set_whitespace_after(&l_brace, create_whitespace(""));
    }
    Some(())
}

fn format_struct_member(syntax: &SyntaxNode) -> Option<()> {
    let item = ast::StructMember::cast(syntax.clone())?;
    super::format_colon(item.colon_token().as_ref());
    if let Some(last) = item.syntax().last_token() {
        let mut tok = last.next_token()?;
        while tok.kind().is_whitespace() {
            let next = tok.next_token()?;
            remove_token(&tok);
            tok = next;
        }
    }
    Some(())
}

fn format_variable_declaration(syntax: &SyntaxNode) -> Option<()> {
    let statement = ast::VariableDeclaration::cast(syntax.clone())?;
    if let Some(tmpl) = statement.template_parameters() {
        super::format_template_angles(&tmpl);
        if let Some(right_angle) = tmpl.right_angle_token() {
            set_whitespace_single_after(&right_angle);
        }
    } else {
        set_whitespace_single_after(&statement.var_token()?);
    }
    super::format_colon(statement.colon().as_ref());
    whitespace_to_single_around(&statement.equal_token()?);
    Some(())
}

fn format_let_declaration(syntax: &SyntaxNode) -> Option<()> {
    let statement = ast::LetDeclaration::cast(syntax.clone())?;
    set_whitespace_single_after(&statement.let_token()?);
    super::format_colon(statement.colon().as_ref());
    whitespace_to_single_around(&statement.equal_token()?);
    Some(())
}

fn format_constant_declaration(syntax: &SyntaxNode) -> Option<()> {
    let statement = ast::ConstantDeclaration::cast(syntax.clone())?;
    set_whitespace_single_after(&statement.constant_token()?);
    super::format_colon(statement.colon().as_ref());
    whitespace_to_single_around(&statement.equal_token()?);
    Some(())
}

fn format_override_declaration(syntax: &SyntaxNode) -> Option<()> {
    let statement = ast::OverrideDeclaration::cast(syntax.clone())?;
    set_whitespace_single_after(&statement.override_token()?);
    super::format_colon(statement.colon().as_ref());
    whitespace_to_single_around(&statement.equal_token()?);
    Some(())
}

fn format_type_alias_declaration(syntax: &SyntaxNode) -> Option<()> {
    let statement = ast::TypeAliasDeclaration::cast(syntax.clone())?;
    set_whitespace_single_after(&statement.alias_token()?);
    whitespace_to_single_around(&statement.equal_token()?);
    Some(())
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::format::tests::check;

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
    fn format_pointer_params() {
        check(
            "fn inc(p:ptr<function,f32>) { *p=*p+1.0; }",
            expect![["fn inc(p: ptr<function,f32>) { *p = *p + 1.0; }"]],
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
    fn format_variable_type() {
        check(
            "fn main() {var x   : u32=0;}",
            expect!["fn main() { var x: u32 = 0; }"],
        );
    }

    #[test]
    fn format_garbled_struct() {
        check(
            "struct\nA\n{\na\n:\ni32\n,\nb\n:\nu32\n,\n}",
            expect![[r#"
            struct A {
                a: i32,
                b: u32,
            }"#]],
        );
    }

    #[test]
    fn format_garbled_fn() {
        check(
            "fn\nmain\n(\n)\n{\n}",
            expect![[r#"
            fn main(
            ) {
            }"#]],
        );
    }

    #[test]
    fn format_garbled_fn_with_params() {
        check(
            "fn\nmain\n(\na\n:\ni32\n,\nb\n:\nu32\n)\n{\n}",
            expect![[r#"
            fn main(
                a: i32,
                b: u32
            ) {
            }"#]],
        );
    }

    #[test]
    fn format_struct_member_with_builtin_attribute() {
        check(
            "struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
}",
            expect![[r#"
            struct VertexOutput {
                @builtin(position) pos: vec4<f32>,
                @location(0) color: vec4<f32>,
            }"#]],
        );
    }

    #[test]
    fn format_empty_struct_extra_spaces() {
        check("struct  Empty  {  }", expect![["struct Empty {}"]]);
    }

    #[test]
    fn format_multiple_attributes_on_struct_member() {
        check(
            "struct V {
@location(0)@interpolate(flat)color:  vec4<f32>,
}",
            expect![[r#"
                struct V {
                    @location(0)@interpolate(flat)color: vec4<f32>,
                }"#]],
        );
    }

    #[test]
    fn format_param_comment_before_comma() {
        check(
            "fn main(a: f32 /* comment */ , b: f32) {}",
            expect![["fn main(a: f32/* comment */, b: f32) {}"]],
        );
    }

    #[test]
    fn format_let_without_type_annotation() {
        check(
            "fn a() { let   x   =   1; }",
            expect!["fn a() { let x = 1; }"],
        );
    }

    #[test]
    fn format_var_without_initializer() {
        check(
            "fn a() { var   x  :  u32; }",
            expect!["fn a() { var x: u32; }"],
        );
    }
}
