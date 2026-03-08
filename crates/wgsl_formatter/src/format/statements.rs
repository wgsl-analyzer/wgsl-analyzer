use syntax::{AstNode, SyntaxKind, SyntaxNode, ast, ast::SyntaxToken};

use crate::FormattingOptions;
use crate::util::{
    create_whitespace, has_newline_between, indent_before, is_whitespace_with_newline,
    remove_if_whitespace, remove_token, replace_token_with, set_whitespace_single_after,
    set_whitespace_single_before, whitespace_to_single_around,
};

/// Formats statement nodes: control flow (`if`, `for`, `while`, `switch`,
/// `loop`, `continuing`, `break if`), compound statements, assignments,
/// return, phony assignment, and assert statements.
pub(crate) fn format_statement(
    syntax: &SyntaxNode,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
    match syntax.kind() {
        SyntaxKind::IfStatement => {
            let if_statement = ast::IfStatement::cast(syntax.clone())?;

            if let Some(if_block) = if_statement.if_block() {
                set_whitespace_single_after(&if_block.if_token()?);
                set_whitespace_single_before(&if_block.block()?.left_brace_token()?);
            }

            if let Some(else_block) = if_statement.else_block() {
                whitespace_to_single_around(&else_block.else_token()?);
            }

            for else_if_block in if_statement.else_if_blocks() {
                whitespace_to_single_around(&else_if_block.else_token()?);
                whitespace_to_single_around(&else_if_block.if_token()?);

                set_whitespace_single_before(&else_if_block.block()?.left_brace_token()?);
            }
        },
        SyntaxKind::WhileStatement => {
            let while_statement = ast::WhileStatement::cast(syntax.clone())?;

            set_whitespace_single_after(&while_statement.while_token()?);

            set_whitespace_single_before(&while_statement.block()?.left_brace_token()?);
        },
        SyntaxKind::SwitchStatement => {
            let switch_statement = ast::SwitchStatement::cast(syntax.clone())?;
            let first_token = switch_statement.syntax().first_token()?;
            if first_token.kind() == SyntaxKind::Switch {
                set_whitespace_single_after(&first_token);
            }
            if let Some(body) = switch_statement.block() {
                let l_brace = body.left_brace_token()?;
                let r_brace = body.right_brace_token()?;
                set_whitespace_single_before(&l_brace);

                if is_whitespace_with_newline(&l_brace.next_token()?) {
                    fix_comment_indentation(body.syntax(), indentation + 1, options);
                    indent_before(&r_brace, indentation, options);
                }
            }
        },
        SyntaxKind::LoopStatement => {
            let loop_statement = ast::LoopStatement::cast(syntax.clone())?;
            if let Some(block) = loop_statement.block() {
                set_whitespace_single_before(&block.left_brace_token()?);
            }
        },
        SyntaxKind::ContinuingStatement => {
            let continuing = ast::ContinuingStatement::cast(syntax.clone())?;
            if let Some(block) = continuing.block() {
                set_whitespace_single_before(&block.left_brace_token()?);
            }
        },
        SyntaxKind::BreakIfStatement => {
            let break_if = ast::BreakIfStatement::cast(syntax.clone())?;
            let first = break_if.syntax().first_token()?;
            if first.kind() == SyntaxKind::Break {
                set_whitespace_single_after(&first);
            }
            for child in break_if.syntax().children_with_tokens() {
                if let Some(tok) = child.as_token()
                    && tok.kind() == SyntaxKind::If
                {
                    set_whitespace_single_before(tok);
                    set_whitespace_single_after(tok);
                    break;
                }
            }
        },
        SyntaxKind::SwitchBodyCase => {
            let case = ast::SwitchBodyCase::cast(syntax.clone())?;
            let first = case.syntax().first_token()?;
            if matches!(first.kind(), SyntaxKind::Case | SyntaxKind::Default) {
                set_whitespace_single_after(&first);
            }
            for token in case.syntax().children_with_tokens() {
                if let Some(tok) = token.as_token()
                    && tok.kind() == SyntaxKind::Colon
                {
                    remove_if_whitespace(&tok.prev_token()?); // spellchecker:disable-line
                    break;
                }
            }
            if let Some(selectors) = case.selectors() {
                for token in selectors.syntax().children_with_tokens() {
                    if let Some(tok) = token.as_token()
                        && tok.kind() == SyntaxKind::Comma
                    {
                        remove_if_whitespace(&tok.prev_token()?); // spellchecker:disable-line
                        set_whitespace_single_after(tok);
                    }
                }
            }
            if let Some(block) = case.block() {
                set_whitespace_single_before(&block.left_brace_token()?);
            }
        },
        SyntaxKind::ForStatement => {
            let for_statement = ast::ForStatement::cast(syntax.clone())?;

            set_whitespace_single_after(&for_statement.for_token()?);

            set_whitespace_single_before(&for_statement.block()?.left_brace_token()?);

            remove_if_whitespace(
                &for_statement
                    .initializer()?
                    .syntax()
                    .first_token()?
                    .prev_token()?, // spellchecker:disable-line
            );
            for child in for_statement.syntax().children_with_tokens() {
                if let Some(tok) = child.as_token()
                    && tok.kind() == SyntaxKind::Semicolon
                {
                    remove_if_whitespace(&tok.prev_token()?); // spellchecker:disable-line
                }
            }
            set_whitespace_single_before(&for_statement.condition()?.syntax().first_token()?);
            set_whitespace_single_before(&for_statement.continuing_part()?.syntax().first_token()?);
            let cont_last = for_statement.continuing_part()?.syntax().last_token()?;
            if let Some(after) = cont_last.next_token() {
                remove_if_whitespace(&after);
            }
        },
        _ => return format_statement_rest(syntax, indentation, options),
    }
    Some(())
}

fn format_statement_rest(
    syntax: &SyntaxNode,
    indentation: usize,
    options: &FormattingOptions,
) -> Option<()> {
    match syntax.kind() {
        SyntaxKind::CompoundStatement => {
            let statement = ast::CompoundStatement::cast(syntax.clone())?;
            let l_brace = statement.left_brace_token()?;
            let r_brace = statement.right_brace_token()?;
            let has_newline = has_newline_between(&l_brace, &r_brace);

            if has_newline {
                // If `{` has content on the same line but `}` is on a new line,
                // push the content to a new indented line for consistency.
                if let Some(first_tok) = statement
                    .statements()
                    .next()
                    .and_then(|s| s.syntax().first_token())
                {
                    let on_same_line = first_tok
                        .prev_token() // spellchecker:disable-line
                        .is_none_or(|t| !t.text().contains('\n'));
                    if on_same_line {
                        indent_before(&first_tok, indentation, options);
                    }
                }

                // Fix indentation of standalone comment tokens inside the block.
                fix_comment_indentation(syntax, indentation, options);

                indent_before(&r_brace, indentation.saturating_sub(1), options);
            } else if statement.statements().next().is_none() {
                remove_if_whitespace(&l_brace.next_token()?);
            } else {
                set_whitespace_single_after(&l_brace);
                set_whitespace_single_before(&r_brace);
            }
        },
        SyntaxKind::AssignmentStatement => {
            let statement = ast::AssignmentStatement::cast(syntax.clone())?;
            whitespace_to_single_around(&statement.equal_token()?);
        },
        SyntaxKind::CompoundAssignmentStatement => {
            let statement = ast::CompoundAssignmentStatement::cast(syntax.clone())?;
            let left_last = statement.left_side()?.syntax().last_token()?;
            let mut tok = left_last.next_token()?;
            loop {
                let kind = tok.kind();
                let is_comment = matches!(
                    kind,
                    SyntaxKind::BlockComment | SyntaxKind::LineEndingComment
                );
                if !kind.is_whitespace() && !is_comment {
                    break;
                }
                let next = tok.next_token()?;
                if kind.is_whitespace() {
                    remove_token(&tok);
                }
                if is_comment {
                    whitespace_to_single_around(&tok);
                }
                tok = next;
            }
            whitespace_to_single_around(&tok);
        },
        SyntaxKind::IncrementDecrementStatement => {
            for token in syntax.children_with_tokens() {
                if let Some(tok) = token.as_token()
                    && matches!(tok.kind(), SyntaxKind::PlusPlus | SyntaxKind::MinusMinus)
                {
                    remove_if_whitespace(&tok.prev_token()?); // spellchecker:disable-line
                    break;
                }
            }
        },
        SyntaxKind::ReturnStatement => {
            let first_token = syntax.first_token()?;
            if first_token.kind() == SyntaxKind::Return {
                let next = first_token.next_token()?;
                if next.kind() != SyntaxKind::Semicolon {
                    set_whitespace_single_after(&first_token);
                }
            }
        },
        SyntaxKind::PhonyAssignmentStatement => {
            let statement = ast::PhonyAssignmentStatement::cast(syntax.clone())?;
            whitespace_to_single_around(&statement.equal_token()?);
        },
        SyntaxKind::AssertStatement => {
            let first_token = syntax.first_token()?;
            if first_token.kind() == SyntaxKind::ConstantAssert {
                set_whitespace_single_after(&first_token);
            }
        },
        _ => return None,
    }
    Some(())
}

/// Fixes indentation of standalone comment tokens that are direct children
/// of a block node (e.g. `CompoundStatement`, `SwitchBody`).
///
/// Comments are tokens, not nodes, so the normal node-walking indentation
/// logic in `format_syntax_node` doesn't reach them. This function iterates
/// over the token children and adjusts the preceding whitespace so comments
/// align to the expected indentation level.
fn fix_comment_indentation(
    block: &SyntaxNode,
    indentation: usize,
    options: &FormattingOptions,
) {
    for child in block.children_with_tokens() {
        if let Some(token) = child.as_token() {
            if !matches!(
                token.kind(),
                SyntaxKind::LineEndingComment | SyntaxKind::BlockComment
            ) {
                continue;
            }
            // Only fix comments that start on their own line (preceded by
            // whitespace containing a newline).
            if let Some(preceding) = token.prev_token() // spellchecker:disable-line
                && preceding.kind().is_whitespace()
                && preceding.text().contains('\n')
            {
                let expected =
                    format!("\n{}", options.indent_symbol.repeat(indentation));
                if preceding.text() != expected {
                    replace_token_with(&preceding, create_whitespace(&expected));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::format::tests::check;

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
    fn format_compound_assignment_with_comment() {
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
    fn format_if_else_chain_single_line() {
        check(
            "fn main() { if(x<0){return -1;}else if(x>0){return 1;}else{return 0;} }",
            expect![
                "fn main() { if x < 0 { return -1; } else if x > 0 { return 1; } else { return 0; } }"
            ],
        );
    }

    #[test]
    fn format_while_paren_removal() {
        check(
            "fn main() { while(i<10){i+=1;} }",
            expect!["fn main() { while i < 10 { i += 1; } }"],
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
    fn format_comment_indentation_in_block() {
        check(
            "
fn foo() {
    var x = 1;
  // misaligned comment
    var y = 2;
}",
            expect![["
            fn foo() {
                var x = 1;
                // misaligned comment
                var y = 2;
            }"]],
        );
    }

    #[test]
    fn format_comment_indentation_nested() {
        check(
            "
fn foo() {
    loop {
        continuing {
  // deeply misaligned
        }
    }
}",
            expect![["
            fn foo() {
                loop {
                    continuing {
                        // deeply misaligned
                    }
                }
            }"]],
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
            expect!["fn main() { switch x { case 0u: { return 0u; } default: { return 1u; } } }"],
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
                switch x {
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
            expect![[r#"
            fn main() {
                loop {
                    if true { break; }
                    continuing {
                        x += 1;
                    }
                }
            }"#]],
        );
    }

    #[test]
    fn format_switch_case_spacing() {
        check(
            "fn a() { switch z { case  0u  :  { break; } case  1u, 2u  :  { x = 1.0; } default  :  { break; } } }",
            expect!["fn a() { switch z { case 0u: { break; } case 1u, 2u: { x = 1.0; } default: { break; } } }"],
        );
    }

    #[test]
    fn format_increment_decrement() {
        check(
            "fn a() { var x = 0; x  ++; x  --; }",
            expect![["fn a() { var x = 0; x++; x--; }"]],
        );
    }

    #[test]
    fn format_for_semicolon_spacing() {
        check(
            "fn a() { for (var i: u32 = 0u   ; i < 10u   ; i += 1u) { x += 1.0; } }",
            expect![["fn a() { for (var i: u32 = 0u; i < 10u; i += 1u) { x += 1.0; } }"]],
        );
    }

    #[test]
    fn format_single_line_block_spacing() {
        check("fn a() {return 1;}", expect!["fn a() { return 1; }"]);
        check("fn b() {   break;   }", expect!["fn b() { break; }"]);
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
    fn format_break_if_basic() {
        check(
            "
        fn main() {
        loop {
        continuing {
        break if false;
        }
        }
        }",
            expect![[r#"
            fn main() {
                loop {
                    continuing {
                        break if false;
                    }
                }
            }"#]],
        );
    }

    #[test]
    fn format_break_if_paren_removal() {
        check(
            "
        fn main() {
        loop {
        continuing {
        break if (false);
        }
        }
        }",
            expect![[r#"
            fn main() {
                loop {
                    continuing {
                        break if false;
                    }
                }
            }"#]],
        );
    }

    #[test]
    fn format_break_if_important_parens_kept() {
        check(
            "
        fn main() {
        loop {
        continuing {
        break if (1 + (1 + 1));
        }
        }
        }",
            expect![[r#"
            fn main() {
                loop {
                    continuing {
                        break if 1 + (1 + 1);
                    }
                }
            }"#]],
        );
    }

    #[test]
    fn format_break_if_spacing() {
        check(
            "
        fn main() {
        loop {
        continuing {
        break   if   false ;
        }
        }
        }",
            expect![[r#"
            fn main() {
                loop {
                    continuing {
                        break if false;
                    }
                }
            }"#]],
        );
    }

    #[test]
    fn format_break_if_complex_expression() {
        check(
            "
        fn main() {
        loop {
        continuing {
        break if x > 10 && y < 20;
        }
        }
        }",
            expect![[r#"
            fn main() {
                loop {
                    continuing {
                        break if x > 10 && y < 20;
                    }
                }
            }"#]],
        );
    }

    #[test]
    fn format_if_inside_for() {
        check(
            "fn a() {
for(var i: u32 = 0u;i < 10u;i += 1u){
if(i > 5u){
break;
}
}
}",
            expect![[r#"
            fn a() {
                for (var i: u32 = 0u; i < 10u; i += 1u) {
                    if i > 5u {
                        break;
                    }
                }
            }"#]],
        );
    }

    #[test]
    fn format_while_inside_if() {
        check(
            "fn a() {
if(true){
while(x < 10){
x += 1;
}
}
}",
            expect![[r#"
            fn a() {
                if true {
                    while x < 10 {
                        x += 1;
                    }
                }
            }"#]],
        );
    }

    #[test]
    fn format_switch_multiple_selectors() {
        check(
            "fn a() {
switch(x){
case 1,2,3:{
y = 1;
}
default:{
y = 0;
}
}
}",
            expect![[r#"
            fn a() {
                switch x {
                    case 1, 2, 3: {
                        y = 1;
                    }
                    default: {
                        y = 0;
                    }
                }
            }"#]],
        );
    }

    #[test]
    fn format_empty_if_body() {
        check(
            "fn a() { if true {    } }",
            expect!["fn a() { if true {} }"],
        );
    }

    #[test]
    fn format_empty_else_body() {
        check(
            "fn a() { if true { x = 1; } else {    } }",
            expect!["fn a() { if true { x = 1; } else {} }"],
        );
    }

    #[test]
    fn format_empty_for_body() {
        check(
            "fn a() { for (var i: u32 = 0u; i < 10u; i += 1u) {    } }",
            expect!["fn a() { for (var i: u32 = 0u; i < 10u; i += 1u) {} }"],
        );
    }

    #[test]
    fn format_empty_while_body() {
        check(
            "fn a() { while true {    } }",
            expect!["fn a() { while true {} }"],
        );
    }

    #[test]
    fn format_discard_statement() {
        check(
            "fn a() {
discard;
}",
            expect![[r#"
            fn a() {
                discard;
            }"#]],
        );
    }

    #[test]
    fn format_continue_statement() {
        check(
            "fn a() {
loop {
continue;
}
}",
            expect![[r#"
            fn a() {
                loop {
                    continue;
                }
            }"#]],
        );
    }

    #[test]
    fn format_nested_function_call_in_assignment() {
        check(
            "fn a() { x  =  foo(  bar(  1  ,  2  )  ,  3  ); }",
            expect!["fn a() { x = foo(bar(1, 2), 3); }"],
        );
    }
}
