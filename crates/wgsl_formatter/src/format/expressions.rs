use rowan::NodeOrToken;
use syntax::{
    AstNode as _, HasTemplateParameters as _, SyntaxKind, SyntaxNode, ast, ast::SyntaxToken,
};

use crate::FormattingOptions;
use crate::util::{
    remove_if_whitespace, remove_token, remove_whitespace_around_double_colon,
    whitespace_to_single_around,
};

/// Formats expression nodes: identifiers, function calls, infix/prefix
/// operators, indexing, field access, parenthesized expressions, and
/// type specifiers.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "intentional catch-all dispatcher"
)]
pub(crate) fn format_expression(
    syntax: &SyntaxNode,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
    match syntax.kind() {
        SyntaxKind::Path => format_path(syntax),
        SyntaxKind::IdentExpression => format_ident_expression(syntax)?,
        SyntaxKind::FunctionCall => format_function_call(syntax, indentation, options)?,
        SyntaxKind::InfixExpression => format_infix_expression(syntax)?,
        SyntaxKind::IndexExpression => format_index_expression(syntax)?,
        SyntaxKind::FieldExpression => format_field_expression(syntax)?,
        SyntaxKind::PrefixExpression => format_prefix_expression(syntax)?,
        SyntaxKind::ParenthesisExpression => format_parenthesis_expression(syntax)?,
        _ => format_type_specifier(syntax)?,
    }
    Some(())
}

/// WESL fully qualified identifier: `foo :: bar :: baz` → `foo::bar::baz`.
fn format_path(syntax: &SyntaxNode) {
    remove_whitespace_around_double_colon(syntax);
}

/// Formats template angles on identifier expressions (e.g. `vec3 < f32 >` → `vec3<f32>`).
fn format_ident_expression(syntax: &SyntaxNode) -> Option<()> {
    let ident_expression = ast::IdentExpression::cast(syntax.clone())?;
    super::format_template_angles(&ident_expression.template_parameters()?);
    Some(())
}

/// Formats function calls: removes whitespace before `(` and normalizes arguments.
fn format_function_call(
    syntax: &SyntaxNode,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
    let function_call = ast::FunctionCall::cast(syntax.clone())?;

    if let Some(name_ref) = function_call.ident_expression()
        && let Some(NodeOrToken::Token(token)) = name_ref.syntax().next_sibling_or_token()
    {
        remove_if_whitespace(&token);
    }

    let param_list = function_call.parameters()?;
    super::format_parameters(&param_list, indentation, options)?;
    Some(())
}

/// Formats infix expressions: ensures single space around operators.
fn format_infix_expression(syntax: &SyntaxNode) -> Option<()> {
    let expression = ast::InfixExpression::cast(syntax.clone())?;
    whitespace_to_single_around(&expression.operator()?);
    Some(())
}

/// Formats index expressions: `arr [ 0 ]` → `arr[0]`.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "only bracket tokens need formatting"
)]
fn format_index_expression(syntax: &SyntaxNode) -> Option<()> {
    let index_expr = ast::IndexExpression::cast(syntax.clone())?;
    for child in index_expr.syntax().children_with_tokens() {
        if let Some(tok) = child.as_token() {
            match tok.kind() {
                SyntaxKind::BracketLeft => {
                    remove_if_whitespace(&tok.prev_token()?); // spellchecker:disable-line
                    remove_if_whitespace(&tok.next_token()?);
                },
                SyntaxKind::BracketRight => {
                    remove_if_whitespace(&tok.prev_token()?); // spellchecker:disable-line
                },
                _ => {},
            }
        }
    }
    Some(())
}

/// Formats field expressions: `v . x` → `v.x`.
fn format_field_expression(syntax: &SyntaxNode) -> Option<()> {
    for child in syntax.children_with_tokens() {
        if let Some(tok) = child.as_token()
            && tok.kind() == SyntaxKind::Period
        {
            remove_if_whitespace(&tok.prev_token()?); // spellchecker:disable-line
            remove_if_whitespace(&tok.next_token()?);
        }
    }
    Some(())
}

/// Formats prefix expressions: `- y` → `-y`, `! cond` → `!cond`, etc.
fn format_prefix_expression(syntax: &SyntaxNode) -> Option<()> {
    let prefix = ast::PrefixExpression::cast(syntax.clone())?;
    let first = prefix.syntax().first_token()?;
    remove_if_whitespace(&first.next_token()?);
    Some(())
}

/// Formats parenthesized expressions and removes redundant parentheses
/// around conditions in `if`, `while`, `switch`, and `break if` statements.
fn format_parenthesis_expression(syntax: &SyntaxNode) -> Option<()> {
    let parenthesis_expression = ast::ParenthesisExpression::cast(syntax.clone())?;
    remove_if_whitespace(
        &parenthesis_expression
            .left_parenthesis_token()?
            .next_token()?,
    );
    remove_if_whitespace(
        &parenthesis_expression
            .right_parenthesis_token()?
            .prev_token()?, // spellchecker:disable-line
    );

    if parenthesis_expression
        .syntax()
        .parent()
        .is_some_and(|parent| {
            matches!(
                parent.kind(),
                SyntaxKind::WhileStatement
                    | SyntaxKind::IfClause
                    | SyntaxKind::ElseIfClause
                    | SyntaxKind::BreakIfStatement
                    | SyntaxKind::SwitchStatement
            )
        })
    {
        remove_token(&parenthesis_expression.right_parenthesis_token()?);
        remove_token(&parenthesis_expression.left_parenthesis_token()?);
    }
    Some(())
}

/// Formats type specifiers by normalizing template angles.
fn format_type_specifier(syntax: &SyntaxNode) -> Option<()> {
    let r#type = ast::TypeSpecifier::cast(syntax.clone())?;
    super::format_template_angles(&r#type.template_parameters()?);
    Some(())
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::format::tests::{check, check_tabs, check_wesl};

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
\tlet x = mat3x3(
\t\tcosR,  0.0, sinR,
\t\t0.0, 1.0, 0.0,
\t\t-sinR, 0.0, cosR,
\t);
}",
            expect![["
\t\t\tfn main() {
\t\t\t\tlet x = mat3x3(
\t\t\t\t\tcosR, 0.0, sinR,
\t\t\t\t\t0.0, 1.0, 0.0,
\t\t\t\t\t-sinR, 0.0, cosR,
\t\t\t\t);
\t\t\t}"]],
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
    fn format_index_expression_no_spaces() {
        check(
            "fn a() { let x = arr[0]; }",
            expect!["fn a() { let x = arr[0]; }"],
        );
    }

    #[test]
    fn format_index_expression_extra_spaces() {
        check(
            "fn a() { let x = arr  [  0  ]; }",
            expect!["fn a() { let x = arr[0]; }"],
        );
    }

    #[test]
    fn format_index_expression_nested() {
        check(
            "fn a() { let x = arr [ 0 ] [ 1 ]; }",
            expect!["fn a() { let x = arr[0][1]; }"],
        );
    }

    #[test]
    fn format_field_expression_no_spaces() {
        check(
            "fn a() { let x = v.x; }",
            expect!["fn a() { let x = v.x; }"],
        );
    }

    #[test]
    fn format_field_expression_extra_spaces() {
        check(
            "fn a() { let x = v . x; }",
            expect!["fn a() { let x = v.x; }"],
        );
    }

    #[test]
    fn format_field_expression_chained() {
        check(
            "fn a() { let x = obj . field . nested; }",
            expect!["fn a() { let x = obj.field.nested; }"],
        );
    }

    #[test]
    fn format_field_and_index_chained() {
        check(
            "fn a() { let x = obj . field [ 0 ] . nested; }",
            expect!["fn a() { let x = obj.field[0].nested; }"],
        );
    }

    #[test]
    fn format_prefix_negate() {
        check("fn a() { let x = - y; }", expect!["fn a() { let x = -y; }"]);
    }

    #[test]
    fn format_prefix_not() {
        check(
            "fn a() { let x = ! condition; }",
            expect!["fn a() { let x = !condition; }"],
        );
    }

    #[test]
    fn format_prefix_deref() {
        check(
            "fn a() { let x = * ptr; }",
            expect!["fn a() { let x = *ptr; }"],
        );
    }

    #[test]
    fn format_prefix_address_of() {
        check(
            "fn a() { let x = & var_name; }",
            expect!["fn a() { let x = &var_name; }"],
        );
    }

    #[test]
    fn format_wesl_path_double_colon_spacing() {
        check_wesl(
            "fn a() { let x = foo :: bar :: baz; }",
            expect!["fn a() { let x = foo::bar::baz; }"],
        );
    }
}
