#![cfg_attr(not(test), allow(unused))]
#![expect(clippy::too_many_lines, reason = "snapshot test data")]

mod diagnostic;
mod expression;
mod imports;

use expect_test::{Expect, expect};

use crate::ParseEntryPoint;

#[expect(clippy::needless_pass_by_value, reason = "intended API")]
fn check(
    input: &str,
    expected_tree: Expect,
) {
    crate::check_entrypoint(input, ParseEntryPoint::File, &expected_tree);
}

#[expect(clippy::needless_pass_by_value, reason = "intended API")]
fn check_type(
    input: &str,
    expected_tree: Expect,
) {
    crate::check_entrypoint(input, ParseEntryPoint::Type, &expected_tree);
}

#[expect(clippy::needless_pass_by_value, reason = "intended API")]
fn check_statement(
    statement: &str,
    expected_tree: Expect,
) {
    crate::check_entrypoint(statement, ParseEntryPoint::Statement, &expected_tree);
}

#[expect(clippy::needless_pass_by_value, reason = "intended API")]
fn check_attribute(
    statement: &str,
    expected_tree: Expect,
) {
    crate::check_entrypoint(statement, ParseEntryPoint::Attribute, &expected_tree);
}

#[test]
fn can_parse_array_declaration() {
    check(
        "
        const dim: vec3u = vec3u();
        fn test(a: array<f32, dim.x>) { }
        ",
        expect![[r#"
            SourceFile@0..87
              Blankspace@0..9 "\n        "
              ConstantDeclaration@9..36
                Const@9..14 "const"
                Blankspace@14..15 " "
                Name@15..18
                  Identifier@15..18 "dim"
                Colon@18..19 ":"
                Blankspace@19..20 " "
                TypeSpecifier@20..25
                  Path@20..25
                    Identifier@20..25 "vec3u"
                Blankspace@25..26 " "
                Equal@26..27 "="
                Blankspace@27..28 " "
                FunctionCall@28..35
                  IdentExpression@28..33
                    Path@28..33
                      Identifier@28..33 "vec3u"
                  Arguments@33..35
                    ParenthesisLeft@33..34 "("
                    ParenthesisRight@34..35 ")"
                Semicolon@35..36 ";"
              Blankspace@36..45 "\n        "
              FunctionDeclaration@45..78
                Fn@45..47 "fn"
                Blankspace@47..48 " "
                Name@48..52
                  Identifier@48..52 "test"
                FunctionParameters@52..74
                  ParenthesisLeft@52..53 "("
                  Parameter@53..73
                    Name@53..54
                      Identifier@53..54 "a"
                    Colon@54..55 ":"
                    Blankspace@55..56 " "
                    TypeSpecifier@56..73
                      Path@56..61
                        Identifier@56..61 "array"
                      TemplateList@61..73
                        TemplateStart@61..62 "<"
                        IdentExpression@62..65
                          Path@62..65
                            Identifier@62..65 "f32"
                        Comma@65..66 ","
                        Blankspace@66..67 " "
                        FieldExpression@67..72
                          IdentExpression@67..70
                            Path@67..70
                              Identifier@67..70 "dim"
                          Period@70..71 "."
                          Identifier@71..72 "x"
                        TemplateEnd@72..73 ">"
                  ParenthesisRight@73..74 ")"
                Blankspace@74..75 " "
                CompoundStatement@75..78
                  BraceLeft@75..76 "{"
                  Blankspace@76..77 " "
                  BraceRight@77..78 "}"
              Blankspace@78..87 "\n        ""#]],
    );
}

#[test]
fn cannot_parse_bad_array_declaration() {
    check(
        "
        const dim: vec3u = vec3u();
        fn test(a: array<f32, dim.>) { }
        ",
        expect![[r#"
            SourceFile@0..86
              Blankspace@0..9 "\n        "
              ConstantDeclaration@9..36
                Const@9..14 "const"
                Blankspace@14..15 " "
                Name@15..18
                  Identifier@15..18 "dim"
                Colon@18..19 ":"
                Blankspace@19..20 " "
                TypeSpecifier@20..25
                  Path@20..25
                    Identifier@20..25 "vec3u"
                Blankspace@25..26 " "
                Equal@26..27 "="
                Blankspace@27..28 " "
                FunctionCall@28..35
                  IdentExpression@28..33
                    Path@28..33
                      Identifier@28..33 "vec3u"
                  Arguments@33..35
                    ParenthesisLeft@33..34 "("
                    ParenthesisRight@34..35 ")"
                Semicolon@35..36 ";"
              Blankspace@36..45 "\n        "
              FunctionDeclaration@45..77
                Fn@45..47 "fn"
                Blankspace@47..48 " "
                Name@48..52
                  Identifier@48..52 "test"
                FunctionParameters@52..73
                  ParenthesisLeft@52..53 "("
                  Parameter@53..72
                    Name@53..54
                      Identifier@53..54 "a"
                    Colon@54..55 ":"
                    Blankspace@55..56 " "
                    TypeSpecifier@56..72
                      Path@56..61
                        Identifier@56..61 "array"
                      TemplateList@61..72
                        TemplateStart@61..62 "<"
                        IdentExpression@62..65
                          Path@62..65
                            Identifier@62..65 "f32"
                        Comma@65..66 ","
                        Blankspace@66..67 " "
                        FieldExpression@67..71
                          IdentExpression@67..70
                            Path@67..70
                              Identifier@67..70 "dim"
                          Period@70..71 "."
                        TemplateEnd@71..72 ">"
                  ParenthesisRight@72..73 ")"
                Blankspace@73..74 " "
                CompoundStatement@74..77
                  BraceLeft@74..75 "{"
                  Blankspace@75..76 " "
                  BraceRight@76..77 "}"
              Blankspace@77..86 "\n        "

            error at 71..72: invalid syntax, expected: <identifier>"#]],
    );
}

#[test]
fn parse_empty() {
    check("", expect![["SourceFile@0..0"]]);
}

#[test]
fn fn_incomplete() {
    check(
        "fn name",
        expect![[r#"
            SourceFile@0..7
              FunctionDeclaration@0..7
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "name"

            error at 7..7: invalid syntax, expected: '('"#]],
    );
}

#[test]
fn parse_comments() {
    check(
        "
        const foo = 1.5; // This is line-ending comment.
        const bar = 2.5; /* This is a block comment
                that spans lines.
                /* Block comments can nest.
                 */
                But all block comments must terminate.
               */
        ",
        expect![[r#"
            SourceFile@0..289
              Blankspace@0..9 "\n        "
              ConstantDeclaration@9..25
                Const@9..14 "const"
                Blankspace@14..15 " "
                Name@15..18
                  Identifier@15..18 "foo"
                Blankspace@18..19 " "
                Equal@19..20 "="
                Blankspace@20..21 " "
                Literal@21..24
                  FloatLiteral@21..24 "1.5"
                Semicolon@24..25 ";"
              Blankspace@25..26 " "
              LineEndingComment@26..57 "// This is line-endin ..."
              Blankspace@57..66 "\n        "
              ConstantDeclaration@66..82
                Const@66..71 "const"
                Blankspace@71..72 " "
                Name@72..75
                  Identifier@72..75 "bar"
                Blankspace@75..76 " "
                Equal@76..77 "="
                Blankspace@77..78 " "
                Literal@78..81
                  FloatLiteral@78..81 "2.5"
                Semicolon@81..82 ";"
              Blankspace@82..83 " "
              BlockComment@83..280 "/* This is a block co ..."
              Blankspace@280..289 "\n        ""#]],
    );
}

#[test]
fn cannot_parse_unmatched_block_comment() {
    check(
        "
        /* This is a block comment that spans lines.
            /* Block comments can nest.
            But all block comments must terminate.
            */
        ",
        expect![[r#"
            SourceFile@0..168
              Blankspace@0..9 "\n        "
              Error@9..167 "/* This is a block co ..."
              Blankspace@167..168 " "

            error at 9..167: unexpected tokens"#]],
    );
}

#[test]
fn parse_unfinished_block_comment() {
    check(
        "/*",
        expect![[r#"
            SourceFile@0..2
              Error@0..2 "/*"

            error at 0..2: unexpected tokens"#]],
    );
}

#[test]
fn function() {
    check(
        "fn name(a: f32, b: i32) -> f32 {}",
        expect![[r#"
            SourceFile@0..33
              FunctionDeclaration@0..33
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "name"
                FunctionParameters@7..23
                  ParenthesisLeft@7..8 "("
                  Parameter@8..14
                    Name@8..9
                      Identifier@8..9 "a"
                    Colon@9..10 ":"
                    Blankspace@10..11 " "
                    TypeSpecifier@11..14
                      Path@11..14
                        Identifier@11..14 "f32"
                  Comma@14..15 ","
                  Blankspace@15..16 " "
                  Parameter@16..22
                    Name@16..17
                      Identifier@16..17 "b"
                    Colon@17..18 ":"
                    Blankspace@18..19 " "
                    TypeSpecifier@19..22
                      Path@19..22
                        Identifier@19..22 "i32"
                  ParenthesisRight@22..23 ")"
                Blankspace@23..24 " "
                ReturnType@24..30
                  Arrow@24..26 "->"
                  Blankspace@26..27 " "
                  TypeSpecifier@27..30
                    Path@27..30
                      Identifier@27..30 "f32"
                Blankspace@30..31 " "
                CompoundStatement@31..33
                  BraceLeft@31..32 "{"
                  BraceRight@32..33 "}""#]],
    );
}

#[test]
fn variable_declarations() {
    check(
        "fn name() {
let x: f32 = 1.0;
let y: f32 = 2.0;
        }",
        expect![[r#"
            SourceFile@0..57
              FunctionDeclaration@0..57
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "name"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..57
                  BraceLeft@10..11 "{"
                  Blankspace@11..12 "\n"
                  LetDeclaration@12..29
                    Let@12..15 "let"
                    Blankspace@15..16 " "
                    Name@16..17
                      Identifier@16..17 "x"
                    Colon@17..18 ":"
                    Blankspace@18..19 " "
                    TypeSpecifier@19..22
                      Path@19..22
                        Identifier@19..22 "f32"
                    Blankspace@22..23 " "
                    Equal@23..24 "="
                    Blankspace@24..25 " "
                    Literal@25..28
                      FloatLiteral@25..28 "1.0"
                    Semicolon@28..29 ";"
                  Blankspace@29..30 "\n"
                  LetDeclaration@30..47
                    Let@30..33 "let"
                    Blankspace@33..34 " "
                    Name@34..35
                      Identifier@34..35 "y"
                    Colon@35..36 ":"
                    Blankspace@36..37 " "
                    TypeSpecifier@37..40
                      Path@37..40
                        Identifier@37..40 "f32"
                    Blankspace@40..41 " "
                    Equal@41..42 "="
                    Blankspace@42..43 " "
                    Literal@43..46
                      FloatLiteral@43..46 "2.0"
                    Semicolon@46..47 ";"
                  Blankspace@47..56 "\n        "
                  BraceRight@56..57 "}""#]],
    );
}

#[test]
fn trivial_function() {
    check(
        "fn test() {}",
        expect![[r#"
            SourceFile@0..12
              FunctionDeclaration@0..12
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "test"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..12
                  BraceLeft@10..11 "{"
                  BraceRight@11..12 "}""#]],
    );
}

#[test]
fn nontrivial_function() {
    check(
        "fn foo() -> i32 { return 90 + 2; }",
        expect![[r#"
            SourceFile@0..34
              FunctionDeclaration@0..34
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..6
                  Identifier@3..6 "foo"
                FunctionParameters@6..8
                  ParenthesisLeft@6..7 "("
                  ParenthesisRight@7..8 ")"
                Blankspace@8..9 " "
                ReturnType@9..15
                  Arrow@9..11 "->"
                  Blankspace@11..12 " "
                  TypeSpecifier@12..15
                    Path@12..15
                      Identifier@12..15 "i32"
                Blankspace@15..16 " "
                CompoundStatement@16..34
                  BraceLeft@16..17 "{"
                  Blankspace@17..18 " "
                  ReturnStatement@18..32
                    Return@18..24 "return"
                    Blankspace@24..25 " "
                    InfixExpression@25..31
                      Literal@25..27
                        IntLiteral@25..27 "90"
                      Blankspace@27..28 " "
                      Plus@28..29 "+"
                      Blankspace@29..30 " "
                      Literal@30..31
                        IntLiteral@30..31 "2"
                    Semicolon@31..32 ";"
                  Blankspace@32..33 " "
                  BraceRight@33..34 "}""#]],
    );
}

#[test]
fn fn_recover() {
    check(
        "fn\nfn name",
        expect![[r#"
            SourceFile@0..10
              FunctionDeclaration@0..3
                Fn@0..2 "fn"
                Blankspace@2..3 "\n"
              FunctionDeclaration@3..10
                Fn@3..5 "fn"
                Blankspace@5..6 " "
                Name@6..10
                  Identifier@6..10 "name"

            error at 3..5: invalid syntax, expected: <identifier>
            error at 10..10: invalid syntax, expected: '('"#]],
    );
}

#[test]
fn fn_recover_2() {
    check(
        "fn name()
        fn test() {}",
        expect![[r#"
            SourceFile@0..30
              FunctionDeclaration@0..18
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "name"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..18 "\n        "
              FunctionDeclaration@18..30
                Fn@18..20 "fn"
                Blankspace@20..21 " "
                Name@21..25
                  Identifier@21..25 "test"
                FunctionParameters@25..27
                  ParenthesisLeft@25..26 "("
                  ParenthesisRight@26..27 ")"
                Blankspace@27..28 " "
                CompoundStatement@28..30
                  BraceLeft@28..29 "{"
                  BraceRight@29..30 "}"

            error at 18..20: invalid syntax, expected one of: '->', '@', '{'"#]],
    );
}

#[test]
fn parse_type_primitive() {
    check_type(
        "f32",
        expect![[r#"
            SourceFile@0..3
              TypeSpecifier@0..3
                Path@0..3
                  Identifier@0..3 "f32""#]],
    );
}

#[test]
fn parse_type_with_template() {
    check_type(
        "vec3<f32>",
        expect![[r#"
            SourceFile@0..9
              TypeSpecifier@0..9
                Path@0..4
                  Identifier@0..4 "vec3"
                TemplateList@4..9
                  TemplateStart@4..5 "<"
                  IdentExpression@5..8
                    Path@5..8
                      Identifier@5..8 "f32"
                  TemplateEnd@8..9 ">""#]],
    );
}

#[test]
fn parse_type_template_shift_ambiguity() {
    check_type(
        "array<vec3<f32, 2>>",
        expect![[r#"
            SourceFile@0..19
              TypeSpecifier@0..19
                Path@0..5
                  Identifier@0..5 "array"
                TemplateList@5..19
                  TemplateStart@5..6 "<"
                  IdentExpression@6..18
                    Path@6..10
                      Identifier@6..10 "vec3"
                    TemplateList@10..18
                      TemplateStart@10..11 "<"
                      IdentExpression@11..14
                        Path@11..14
                          Identifier@11..14 "f32"
                      Comma@14..15 ","
                      Blankspace@15..16 " "
                      Literal@16..17
                        IntLiteral@16..17 "2"
                      TemplateEnd@17..18 ">"
                  TemplateEnd@18..19 ">""#]],
    );
}

#[test]
fn parse_type_template_with_int() {
    check_type(
        "array<f32, 100>",
        expect![[r#"
            SourceFile@0..15
              TypeSpecifier@0..15
                Path@0..5
                  Identifier@0..5 "array"
                TemplateList@5..15
                  TemplateStart@5..6 "<"
                  IdentExpression@6..9
                    Path@6..9
                      Identifier@6..9 "f32"
                  Comma@9..10 ","
                  Blankspace@10..11 " "
                  Literal@11..14
                    IntLiteral@11..14 "100"
                  TemplateEnd@14..15 ">""#]],
    );
}

#[test]
fn parse_type_template_trailing_comma() {
    check_type(
        "array<
        f32,
        100,
        >",
        expect![[r#"
            SourceFile@0..42
              TypeSpecifier@0..42
                Path@0..5
                  Identifier@0..5 "array"
                TemplateList@5..42
                  TemplateStart@5..6 "<"
                  Blankspace@6..15 "\n        "
                  IdentExpression@15..18
                    Path@15..18
                      Identifier@15..18 "f32"
                  Comma@18..19 ","
                  Blankspace@19..28 "\n        "
                  Literal@28..31
                    IntLiteral@28..31 "100"
                  Comma@31..32 ","
                  Blankspace@32..41 "\n        "
                  TemplateEnd@41..42 ">""#]],
    );
}

#[test]
fn parse_type_empty_template() {
    check_type(
        "vec3<>",
        expect![[r#"
            SourceFile@0..6
              TypeSpecifier@0..6
                Path@0..4
                  Identifier@0..4 "vec3"
                TemplateList@4..6
                  TemplateStart@4..5 "<"
                  TemplateEnd@5..6 ">"

            error at 5..6: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'"#]],
    );
}

#[test]
fn parse_type_template_comma_recover() {
    check_type(
        "vec3<,>",
        expect![[r#"
            SourceFile@0..7
              TypeSpecifier@0..7
                Path@0..4
                  Identifier@0..4 "vec3"
                TemplateList@4..7
                  TemplateStart@4..5 "<"
                  Comma@5..6 ","
                  TemplateEnd@6..7 ">"

            error at 5..6: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'"#]],
    );
}

#[test]
fn parse_ptr_template() {
    check_type(
        "ptr<uniform, f32, read_write>",
        expect![[r#"
            SourceFile@0..29
              TypeSpecifier@0..29
                Path@0..3
                  Identifier@0..3 "ptr"
                TemplateList@3..29
                  TemplateStart@3..4 "<"
                  IdentExpression@4..11
                    Path@4..11
                      Identifier@4..11 "uniform"
                  Comma@11..12 ","
                  Blankspace@12..13 " "
                  IdentExpression@13..16
                    Path@13..16
                      Identifier@13..16 "f32"
                  Comma@16..17 ","
                  Blankspace@17..18 " "
                  IdentExpression@18..28
                    Path@18..28
                      Identifier@18..28 "read_write"
                  TemplateEnd@28..29 ">""#]],
    );
}

#[test]
fn parse_return_statement() {
    check(
        "fn foo() -> u32 {
            return 0;
        }",
        expect![[r#"
            SourceFile@0..49
              FunctionDeclaration@0..49
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..6
                  Identifier@3..6 "foo"
                FunctionParameters@6..8
                  ParenthesisLeft@6..7 "("
                  ParenthesisRight@7..8 ")"
                Blankspace@8..9 " "
                ReturnType@9..15
                  Arrow@9..11 "->"
                  Blankspace@11..12 " "
                  TypeSpecifier@12..15
                    Path@12..15
                      Identifier@12..15 "u32"
                Blankspace@15..16 " "
                CompoundStatement@16..49
                  BraceLeft@16..17 "{"
                  Blankspace@17..30 "\n            "
                  ReturnStatement@30..39
                    Return@30..36 "return"
                    Blankspace@36..37 " "
                    Literal@37..38
                      IntLiteral@37..38 "0"
                    Semicolon@38..39 ";"
                  Blankspace@39..48 "\n        "
                  BraceRight@48..49 "}""#]],
    );
}

#[test]
fn parse_invalid_global_let_statement() {
    check(
        "let foo = 3;",
        expect![[r#"
            SourceFile@0..12
              Error@0..12
                Let@0..3 "let"
                Blankspace@3..4 " "
                Name@4..7
                  Identifier@4..7 "foo"
                Blankspace@7..8 " "
                Equal@8..9 "="
                Blankspace@9..10 " "
                Literal@10..11
                  IntLiteral@10..11 "3"
                Semicolon@11..12 ";"

            error at 0..12: global let declarations are not allowed"#]],
    );
}

#[test]
fn parse_let_statement_recover() {
    check(
        "fn foo() -> u32 {
            let x =
            let y =
            return 0
        }",
        expect![[r#"
            SourceFile@0..88
              FunctionDeclaration@0..88
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..6
                  Identifier@3..6 "foo"
                FunctionParameters@6..8
                  ParenthesisLeft@6..7 "("
                  ParenthesisRight@7..8 ")"
                Blankspace@8..9 " "
                ReturnType@9..15
                  Arrow@9..11 "->"
                  Blankspace@11..12 " "
                  TypeSpecifier@12..15
                    Path@12..15
                      Identifier@12..15 "u32"
                Blankspace@15..16 " "
                CompoundStatement@16..88
                  BraceLeft@16..17 "{"
                  Blankspace@17..30 "\n            "
                  LetDeclaration@30..37
                    Let@30..33 "let"
                    Blankspace@33..34 " "
                    Name@34..35
                      Identifier@34..35 "x"
                    Blankspace@35..36 " "
                    Equal@36..37 "="
                  Blankspace@37..50 "\n            "
                  LetDeclaration@50..57
                    Let@50..53 "let"
                    Blankspace@53..54 " "
                    Name@54..55
                      Identifier@54..55 "y"
                    Blankspace@55..56 " "
                    Equal@56..57 "="
                  Blankspace@57..70 "\n            "
                  ReturnStatement@70..78
                    Return@70..76 "return"
                    Blankspace@76..77 " "
                    Literal@77..78
                      IntLiteral@77..78 "0"
                  Blankspace@78..87 "\n        "
                  BraceRight@87..88 "}"

            error at 50..53: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'
            error at 70..76: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'
            error at 87..88: invalid syntax, expected: ';'"#]],
    );
}

#[test]
fn parse_recover_covers_whole_file() {
    check(
        "fn count() {
    let b = a
    let c = b.x;
}",
        expect![[r#"
            SourceFile@0..45
              FunctionDeclaration@0..45
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..8
                  Identifier@3..8 "count"
                FunctionParameters@8..10
                  ParenthesisLeft@8..9 "("
                  ParenthesisRight@9..10 ")"
                Blankspace@10..11 " "
                CompoundStatement@11..45
                  BraceLeft@11..12 "{"
                  Blankspace@12..17 "\n    "
                  LetDeclaration@17..26
                    Let@17..20 "let"
                    Blankspace@20..21 " "
                    Name@21..22
                      Identifier@21..22 "b"
                    Blankspace@22..23 " "
                    Equal@23..24 "="
                    Blankspace@24..25 " "
                    IdentExpression@25..26
                      Path@25..26
                        Identifier@25..26 "a"
                  Blankspace@26..31 "\n    "
                  LetDeclaration@31..43
                    Let@31..34 "let"
                    Blankspace@34..35 " "
                    Name@35..36
                      Identifier@35..36 "c"
                    Blankspace@36..37 " "
                    Equal@37..38 "="
                    Blankspace@38..39 " "
                    FieldExpression@39..42
                      IdentExpression@39..40
                        Path@39..40
                          Identifier@39..40 "b"
                      Period@40..41 "."
                      Identifier@41..42 "x"
                    Semicolon@42..43 ";"
                  Blankspace@43..44 "\n"
                  BraceRight@44..45 "}"

            error at 31..34: invalid syntax, expected one of: '&', '&&', '&=', '@', '{', '}', '[', ']', ':', '::', ',', '/=', '=', '==', '/', '>', '>=', <identifier>, '<', '<=', '-', '-=', '--', '%', '%=', '!=', '|', '|=', '||', '(', ')', '.', '+', '+=', '++', ';', '<<', '<<=', '>>', '>>=', '*', <template end>, <template start>, '*=', '^', '^='"#]],
    );
}

#[test]
fn parse_statement_variable_declaration() {
    check_statement(
        "let x = 3;",
        expect![[r#"
            SourceFile@0..10
              LetDeclaration@0..10
                Let@0..3 "let"
                Blankspace@3..4 " "
                Name@4..5
                  Identifier@4..5 "x"
                Blankspace@5..6 " "
                Equal@6..7 "="
                Blankspace@7..8 " "
                Literal@8..9
                  IntLiteral@8..9 "3"
                Semicolon@9..10 ";""#]],
    );
}

#[test]
fn parse_statement_variable_declaration_shader_in64() {
    check(
        "fn foo() {
            let x: u64 = 3lu;
            let x: i64 = 3li;
        }
        ",
        expect![[r#"
            SourceFile@0..89
              FunctionDeclaration@0..80
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..6
                  Identifier@3..6 "foo"
                FunctionParameters@6..8
                  ParenthesisLeft@6..7 "("
                  ParenthesisRight@7..8 ")"
                Blankspace@8..9 " "
                CompoundStatement@9..80
                  BraceLeft@9..10 "{"
                  Blankspace@10..23 "\n            "
                  LetDeclaration@23..40
                    Let@23..26 "let"
                    Blankspace@26..27 " "
                    Name@27..28
                      Identifier@27..28 "x"
                    Colon@28..29 ":"
                    Blankspace@29..30 " "
                    TypeSpecifier@30..33
                      Path@30..33
                        Identifier@30..33 "u64"
                    Blankspace@33..34 " "
                    Equal@34..35 "="
                    Blankspace@35..36 " "
                    Literal@36..39
                      IntLiteral@36..39 "3lu"
                    Semicolon@39..40 ";"
                  Blankspace@40..53 "\n            "
                  LetDeclaration@53..70
                    Let@53..56 "let"
                    Blankspace@56..57 " "
                    Name@57..58
                      Identifier@57..58 "x"
                    Colon@58..59 ":"
                    Blankspace@59..60 " "
                    TypeSpecifier@60..63
                      Path@60..63
                        Identifier@60..63 "i64"
                    Blankspace@63..64 " "
                    Equal@64..65 "="
                    Blankspace@65..66 " "
                    Literal@66..69
                      IntLiteral@66..69 "3li"
                    Semicolon@69..70 ";"
                  Blankspace@70..79 "\n        "
                  BraceRight@79..80 "}"
              Blankspace@80..89 "\n        ""#]],
    );
}

#[test]
fn parse_not_statement() {
    check_statement(
        "   let a = 3;",
        expect![[r#"
            SourceFile@0..13
              Blankspace@0..3 "   "
              LetDeclaration@3..13
                Let@3..6 "let"
                Blankspace@6..7 " "
                Name@7..8
                  Identifier@7..8 "a"
                Blankspace@8..9 " "
                Equal@9..10 "="
                Blankspace@10..11 " "
                Literal@11..12
                  IntLiteral@11..12 "3"
                Semicolon@12..13 ";""#]],
    );
}

#[test]
fn parse_statement_return() {
    check_statement(
        "return 0;",
        expect![[r#"
            SourceFile@0..9
              ReturnStatement@0..9
                Return@0..6 "return"
                Blankspace@6..7 " "
                Literal@7..8
                  IntLiteral@7..8 "0"
                Semicolon@8..9 ";""#]],
    );
}

#[test]
fn parse_while_statement() {
    check_statement(
        "while 0 > 3 { let x = 3; }",
        expect![[r#"
            SourceFile@0..26
              WhileStatement@0..26
                While@0..5 "while"
                Blankspace@5..6 " "
                InfixExpression@6..11
                  Literal@6..7
                    IntLiteral@6..7 "0"
                  Blankspace@7..8 " "
                  GreaterThan@8..9 ">"
                  Blankspace@9..10 " "
                  Literal@10..11
                    IntLiteral@10..11 "3"
                Blankspace@11..12 " "
                CompoundStatement@12..26
                  BraceLeft@12..13 "{"
                  Blankspace@13..14 " "
                  LetDeclaration@14..24
                    Let@14..17 "let"
                    Blankspace@17..18 " "
                    Name@18..19
                      Identifier@18..19 "x"
                    Blankspace@19..20 " "
                    Equal@20..21 "="
                    Blankspace@21..22 " "
                    Literal@22..23
                      IntLiteral@22..23 "3"
                    Semicolon@23..24 ";"
                  Blankspace@24..25 " "
                  BraceRight@25..26 "}""#]],
    );
}

#[test]
fn parse_if_statement() {
    check_statement(
        "if (0 > 3) { let x = 3; return x; }",
        expect![[r#"
            SourceFile@0..35
              IfStatement@0..35
                IfClause@0..35
                  If@0..2 "if"
                  Blankspace@2..3 " "
                  ParenthesisExpression@3..10
                    ParenthesisLeft@3..4 "("
                    InfixExpression@4..9
                      Literal@4..5
                        IntLiteral@4..5 "0"
                      Blankspace@5..6 " "
                      GreaterThan@6..7 ">"
                      Blankspace@7..8 " "
                      Literal@8..9
                        IntLiteral@8..9 "3"
                    ParenthesisRight@9..10 ")"
                  Blankspace@10..11 " "
                  CompoundStatement@11..35
                    BraceLeft@11..12 "{"
                    Blankspace@12..13 " "
                    LetDeclaration@13..23
                      Let@13..16 "let"
                      Blankspace@16..17 " "
                      Name@17..18
                        Identifier@17..18 "x"
                      Blankspace@18..19 " "
                      Equal@19..20 "="
                      Blankspace@20..21 " "
                      Literal@21..22
                        IntLiteral@21..22 "3"
                      Semicolon@22..23 ";"
                    Blankspace@23..24 " "
                    ReturnStatement@24..33
                      Return@24..30 "return"
                      Blankspace@30..31 " "
                      IdentExpression@31..32
                        Path@31..32
                          Identifier@31..32 "x"
                      Semicolon@32..33 ";"
                    Blankspace@33..34 " "
                    BraceRight@34..35 "}""#]],
    );
}

#[test]
fn parse_if_recover_paren() {
    check_statement(
        "if () {
          let x = 3;
        }",
        expect![[r#"
            SourceFile@0..38
              IfStatement@0..38
                IfClause@0..38
                  If@0..2 "if"
                  Blankspace@2..3 " "
                  ParenthesisExpression@3..5
                    ParenthesisLeft@3..4 "("
                    ParenthesisRight@4..5 ")"
                  Blankspace@5..6 " "
                  CompoundStatement@6..38
                    BraceLeft@6..7 "{"
                    Blankspace@7..18 "\n          "
                    LetDeclaration@18..28
                      Let@18..21 "let"
                      Blankspace@21..22 " "
                      Name@22..23
                        Identifier@22..23 "x"
                      Blankspace@23..24 " "
                      Equal@24..25 "="
                      Blankspace@25..26 " "
                      Literal@26..27
                        IntLiteral@26..27 "3"
                      Semicolon@27..28 ";"
                    Blankspace@28..37 "\n        "
                    BraceRight@37..38 "}"

            error at 4..5: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'"#]],
    );
}

#[test]
fn parse_if_without_paren() {
    check_statement(
        "if true {
          let x = 3;
        }",
        expect![[r#"
            SourceFile@0..40
              IfStatement@0..40
                IfClause@0..40
                  If@0..2 "if"
                  Blankspace@2..3 " "
                  Literal@3..7
                    True@3..7 "true"
                  Blankspace@7..8 " "
                  CompoundStatement@8..40
                    BraceLeft@8..9 "{"
                    Blankspace@9..20 "\n          "
                    LetDeclaration@20..30
                      Let@20..23 "let"
                      Blankspace@23..24 " "
                      Name@24..25
                        Identifier@24..25 "x"
                      Blankspace@25..26 " "
                      Equal@26..27 "="
                      Blankspace@27..28 " "
                      Literal@28..29
                        IntLiteral@28..29 "3"
                      Semicolon@29..30 ";"
                    Blankspace@30..39 "\n        "
                    BraceRight@39..40 "}""#]],
    );
}

#[test]
fn parse_if_recover_empty() {
    check_statement(
        "if {
          let x = 3;
        }",
        expect![[r#"
            SourceFile@0..35
              IfStatement@0..35
                IfClause@0..35
                  If@0..2 "if"
                  Blankspace@2..3 " "
                  CompoundStatement@3..35
                    BraceLeft@3..4 "{"
                    Blankspace@4..15 "\n          "
                    LetDeclaration@15..25
                      Let@15..18 "let"
                      Blankspace@18..19 " "
                      Name@19..20
                        Identifier@19..20 "x"
                      Blankspace@20..21 " "
                      Equal@21..22 "="
                      Blankspace@22..23 " "
                      Literal@23..24
                        IntLiteral@23..24 "3"
                      Semicolon@24..25 ";"
                    Blankspace@25..34 "\n        "
                    BraceRight@34..35 "}"

            error at 3..4: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'"#]],
    );
}

#[test]
fn parse_if_else() {
    check_statement(
        "if (0) {} else if (1) {} else if (2) {} else {}",
        expect![[r#"
            SourceFile@0..47
              IfStatement@0..47
                IfClause@0..9
                  If@0..2 "if"
                  Blankspace@2..3 " "
                  ParenthesisExpression@3..6
                    ParenthesisLeft@3..4 "("
                    Literal@4..5
                      IntLiteral@4..5 "0"
                    ParenthesisRight@5..6 ")"
                  Blankspace@6..7 " "
                  CompoundStatement@7..9
                    BraceLeft@7..8 "{"
                    BraceRight@8..9 "}"
                Blankspace@9..10 " "
                ElseIfClause@10..24
                  Else@10..14 "else"
                  Blankspace@14..15 " "
                  If@15..17 "if"
                  Blankspace@17..18 " "
                  ParenthesisExpression@18..21
                    ParenthesisLeft@18..19 "("
                    Literal@19..20
                      IntLiteral@19..20 "1"
                    ParenthesisRight@20..21 ")"
                  Blankspace@21..22 " "
                  CompoundStatement@22..24
                    BraceLeft@22..23 "{"
                    BraceRight@23..24 "}"
                Blankspace@24..25 " "
                ElseIfClause@25..39
                  Else@25..29 "else"
                  Blankspace@29..30 " "
                  If@30..32 "if"
                  Blankspace@32..33 " "
                  ParenthesisExpression@33..36
                    ParenthesisLeft@33..34 "("
                    Literal@34..35
                      IntLiteral@34..35 "2"
                    ParenthesisRight@35..36 ")"
                  Blankspace@36..37 " "
                  CompoundStatement@37..39
                    BraceLeft@37..38 "{"
                    BraceRight@38..39 "}"
                Blankspace@39..40 " "
                ElseClause@40..47
                  Else@40..44 "else"
                  Blankspace@44..45 " "
                  CompoundStatement@45..47
                    BraceLeft@45..46 "{"
                    BraceRight@46..47 "}""#]],
    );
}

#[test]
fn parse_if_recovery_1() {
    check_statement(
        "if (false) {} else if {}",
        expect![[r#"
            SourceFile@0..24
              IfStatement@0..24
                IfClause@0..13
                  If@0..2 "if"
                  Blankspace@2..3 " "
                  ParenthesisExpression@3..10
                    ParenthesisLeft@3..4 "("
                    Literal@4..9
                      False@4..9 "false"
                    ParenthesisRight@9..10 ")"
                  Blankspace@10..11 " "
                  CompoundStatement@11..13
                    BraceLeft@11..12 "{"
                    BraceRight@12..13 "}"
                Blankspace@13..14 " "
                ElseIfClause@14..24
                  Else@14..18 "else"
                  Blankspace@18..19 " "
                  If@19..21 "if"
                  Blankspace@21..22 " "
                  CompoundStatement@22..24
                    BraceLeft@22..23 "{"
                    BraceRight@23..24 "}"

            error at 22..23: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'"#]],
    );
}

#[test]
fn parse_if_multiple_else_clauses() {
    check_statement(
        "if (0) {} else {} else {}",
        expect![[r#"
            SourceFile@0..25
              IfStatement@0..25
                IfClause@0..9
                  If@0..2 "if"
                  Blankspace@2..3 " "
                  ParenthesisExpression@3..6
                    ParenthesisLeft@3..4 "("
                    Literal@4..5
                      IntLiteral@4..5 "0"
                    ParenthesisRight@5..6 ")"
                  Blankspace@6..7 " "
                  CompoundStatement@7..9
                    BraceLeft@7..8 "{"
                    BraceRight@8..9 "}"
                Blankspace@9..10 " "
                ElseClause@10..17
                  Else@10..14 "else"
                  Blankspace@14..15 " "
                  CompoundStatement@15..17
                    BraceLeft@15..16 "{"
                    BraceRight@16..17 "}"
                Blankspace@17..18 " "
                ElseClause@18..25
                  Else@18..22 "else"
                  Blankspace@22..23 " "
                  CompoundStatement@23..25
                    BraceLeft@23..24 "{"
                    BraceRight@24..25 "}"

            error at 18..25: multiple 'else' clauses are not allowed"#]],
    );
}

#[test]
fn parse_if_else_if_after_else() {
    check_statement(
        "if (0) {} else {} else if (1) {}",
        expect![[r#"
            SourceFile@0..32
              IfStatement@0..32
                IfClause@0..9
                  If@0..2 "if"
                  Blankspace@2..3 " "
                  ParenthesisExpression@3..6
                    ParenthesisLeft@3..4 "("
                    Literal@4..5
                      IntLiteral@4..5 "0"
                    ParenthesisRight@5..6 ")"
                  Blankspace@6..7 " "
                  CompoundStatement@7..9
                    BraceLeft@7..8 "{"
                    BraceRight@8..9 "}"
                Blankspace@9..10 " "
                ElseClause@10..17
                  Else@10..14 "else"
                  Blankspace@14..15 " "
                  CompoundStatement@15..17
                    BraceLeft@15..16 "{"
                    BraceRight@16..17 "}"
                Blankspace@17..18 " "
                ElseIfClause@18..32
                  Else@18..22 "else"
                  Blankspace@22..23 " "
                  If@23..25 "if"
                  Blankspace@25..26 " "
                  ParenthesisExpression@26..29
                    ParenthesisLeft@26..27 "("
                    Literal@27..28
                      IntLiteral@27..28 "1"
                    ParenthesisRight@28..29 ")"
                  Blankspace@29..30 " "
                  CompoundStatement@30..32
                    BraceLeft@30..31 "{"
                    BraceRight@31..32 "}"

            error at 18..32: 'else if' after 'else' is not allowed"#]],
    );
}

#[test]
fn parse_for_statement() {
    check_statement(
        "for(let i = 0; i < 3; i = i + 1) {}",
        expect![[r#"
            SourceFile@0..35
              ForStatement@0..35
                For@0..3 "for"
                ParenthesisLeft@3..4 "("
                ForInitializer@4..13
                  LetDeclaration@4..13
                    Let@4..7 "let"
                    Blankspace@7..8 " "
                    Name@8..9
                      Identifier@8..9 "i"
                    Blankspace@9..10 " "
                    Equal@10..11 "="
                    Blankspace@11..12 " "
                    Literal@12..13
                      IntLiteral@12..13 "0"
                Semicolon@13..14 ";"
                Blankspace@14..15 " "
                ForCondition@15..20
                  InfixExpression@15..20
                    IdentExpression@15..16
                      Path@15..16
                        Identifier@15..16 "i"
                    Blankspace@16..17 " "
                    LessThan@17..18 "<"
                    Blankspace@18..19 " "
                    Literal@19..20
                      IntLiteral@19..20 "3"
                Semicolon@20..21 ";"
                Blankspace@21..22 " "
                ForContinuingPart@22..31
                  AssignmentStatement@22..31
                    IdentExpression@22..23
                      Path@22..23
                        Identifier@22..23 "i"
                    Blankspace@23..24 " "
                    Equal@24..25 "="
                    Blankspace@25..26 " "
                    InfixExpression@26..31
                      IdentExpression@26..27
                        Path@26..27
                          Identifier@26..27 "i"
                      Blankspace@27..28 " "
                      Plus@28..29 "+"
                      Blankspace@29..30 " "
                      Literal@30..31
                        IntLiteral@30..31 "1"
                ParenthesisRight@31..32 ")"
                Blankspace@32..33 " "
                CompoundStatement@33..35
                  BraceLeft@33..34 "{"
                  BraceRight@34..35 "}""#]],
    );
}

#[test]
fn parse_for_statement_comma() {
    check_statement(
        "for(let i = 0, i < 3, i = i + 1) {}",
        expect![[r#"
            SourceFile@0..35
              ForStatement@0..35
                For@0..3 "for"
                ParenthesisLeft@3..4 "("
                ForInitializer@4..13
                  LetDeclaration@4..13
                    Let@4..7 "let"
                    Blankspace@7..8 " "
                    Name@8..9
                      Identifier@8..9 "i"
                    Blankspace@9..10 " "
                    Equal@10..11 "="
                    Blankspace@11..12 " "
                    Literal@12..13
                      IntLiteral@12..13 "0"
                Error@13..14
                  Comma@13..14 ","
                Blankspace@14..15 " "
                ForCondition@15..20
                  InfixExpression@15..20
                    IdentExpression@15..16
                      Path@15..16
                        Identifier@15..16 "i"
                    Blankspace@16..17 " "
                    LessThan@17..18 "<"
                    Blankspace@18..19 " "
                    Literal@19..20
                      IntLiteral@19..20 "3"
                Error@20..21
                  Comma@20..21 ","
                Blankspace@21..22 " "
                ForContinuingPart@22..31
                  AssignmentStatement@22..31
                    IdentExpression@22..23
                      Path@22..23
                        Identifier@22..23 "i"
                    Blankspace@23..24 " "
                    Equal@24..25 "="
                    Blankspace@25..26 " "
                    InfixExpression@26..31
                      IdentExpression@26..27
                        Path@26..27
                          Identifier@26..27 "i"
                      Blankspace@27..28 " "
                      Plus@28..29 "+"
                      Blankspace@29..30 " "
                      Literal@30..31
                        IntLiteral@30..31 "1"
                ParenthesisRight@31..32 ")"
                Blankspace@32..33 " "
                CompoundStatement@33..35
                  BraceLeft@33..34 "{"
                  BraceRight@34..35 "}"

            error at 13..14: invalid syntax, expected: ';'
            error at 20..21: invalid syntax, expected: ';'"#]],
    );
}

#[test]
fn for_statement_incomplete_1() {
    check_statement(
        "for(;;)",
        expect![[r#"
            SourceFile@0..7
              ForStatement@0..7
                For@0..3 "for"
                ParenthesisLeft@3..4 "("
                Semicolon@4..5 ";"
                Semicolon@5..6 ";"
                ParenthesisRight@6..7 ")"

            error at 7..7: invalid syntax, expected one of: '@', '{'"#]],
    );
}

#[test]
fn for_statement_incomplete_2() {
    check_statement(
        "for(i=0;;)",
        expect![[r#"
            SourceFile@0..10
              ForStatement@0..10
                For@0..3 "for"
                ParenthesisLeft@3..4 "("
                ForInitializer@4..7
                  AssignmentStatement@4..7
                    IdentExpression@4..5
                      Path@4..5
                        Identifier@4..5 "i"
                    Equal@5..6 "="
                    Literal@6..7
                      IntLiteral@6..7 "0"
                Semicolon@7..8 ";"
                Semicolon@8..9 ";"
                ParenthesisRight@9..10 ")"

            error at 10..10: invalid syntax, expected one of: '@', '{'"#]],
    );
}

#[test]
fn for_statement_incomplete_3() {
    check_statement(
        "for(;false;)",
        expect![[r#"
            SourceFile@0..12
              ForStatement@0..12
                For@0..3 "for"
                ParenthesisLeft@3..4 "("
                Semicolon@4..5 ";"
                ForCondition@5..10
                  Literal@5..10
                    False@5..10 "false"
                Semicolon@10..11 ";"
                ParenthesisRight@11..12 ")"

            error at 12..12: invalid syntax, expected one of: '@', '{'"#]],
    );
}

#[test]
fn for_statement_incomplete_4() {
    check_statement(
        "for(;;a = 1)",
        expect![[r#"
            SourceFile@0..12
              ForStatement@0..12
                For@0..3 "for"
                ParenthesisLeft@3..4 "("
                Semicolon@4..5 ";"
                Semicolon@5..6 ";"
                ForContinuingPart@6..11
                  AssignmentStatement@6..11
                    IdentExpression@6..7
                      Path@6..7
                        Identifier@6..7 "a"
                    Blankspace@7..8 " "
                    Equal@8..9 "="
                    Blankspace@9..10 " "
                    Literal@10..11
                      IntLiteral@10..11 "1"
                ParenthesisRight@11..12 ")"

            error at 12..12: invalid syntax, expected one of: '@', '{'"#]],
    );
}

#[test]
fn for_statement_continue_break() {
    check_statement(
        "for(;;) { continue; break; }",
        expect![[r#"
            SourceFile@0..28
              ForStatement@0..28
                For@0..3 "for"
                ParenthesisLeft@3..4 "("
                Semicolon@4..5 ";"
                Semicolon@5..6 ";"
                ParenthesisRight@6..7 ")"
                Blankspace@7..8 " "
                CompoundStatement@8..28
                  BraceLeft@8..9 "{"
                  Blankspace@9..10 " "
                  ContinueStatement@10..19
                    Continue@10..18 "continue"
                    Semicolon@18..19 ";"
                  Blankspace@19..20 " "
                  BreakStatement@20..26
                    Break@20..25 "break"
                    Semicolon@25..26 ";"
                  Blankspace@26..27 " "
                  BraceRight@27..28 "}""#]],
    );
}
#[test]
fn loop_statement_continuing() {
    check_statement(
        "loop { continuing {} }",
        expect![[r#"
            SourceFile@0..22
              LoopStatement@0..22
                Loop@0..4 "loop"
                Blankspace@4..5 " "
                CompoundStatement@5..22
                  BraceLeft@5..6 "{"
                  Blankspace@6..7 " "
                  ContinuingStatement@7..20
                    Continuing@7..17 "continuing"
                    Blankspace@17..18 " "
                    CompoundStatement@18..20
                      BraceLeft@18..19 "{"
                      BraceRight@19..20 "}"
                  Blankspace@20..21 " "
                  BraceRight@21..22 "}""#]],
    );
}
#[test]
fn loop_statement_break_if() {
    check_statement(
        "loop { continuing { break if 5 >= 4; } }",
        expect![[r#"
            SourceFile@0..40
              LoopStatement@0..40
                Loop@0..4 "loop"
                Blankspace@4..5 " "
                CompoundStatement@5..40
                  BraceLeft@5..6 "{"
                  Blankspace@6..7 " "
                  ContinuingStatement@7..38
                    Continuing@7..17 "continuing"
                    Blankspace@17..18 " "
                    CompoundStatement@18..38
                      BraceLeft@18..19 "{"
                      Blankspace@19..20 " "
                      BreakIfStatement@20..36
                        Break@20..25 "break"
                        Blankspace@25..26 " "
                        If@26..28 "if"
                        Blankspace@28..29 " "
                        InfixExpression@29..35
                          Literal@29..30
                            IntLiteral@29..30 "5"
                          Blankspace@30..31 " "
                          GreaterThanEqual@31..33 ">="
                          Blankspace@33..34 " "
                          Literal@34..35
                            IntLiteral@34..35 "4"
                        Semicolon@35..36 ";"
                      Blankspace@36..37 " "
                      BraceRight@37..38 "}"
                  Blankspace@38..39 " "
                  BraceRight@39..40 "}""#]],
    );
}

#[test]
fn parse_statement_compound_empty() {
    check_statement(
        "{}",
        expect![[r#"
            SourceFile@0..2
              CompoundStatement@0..2
                BraceLeft@0..1 "{"
                BraceRight@1..2 "}""#]],
    );
}

#[test]
fn parse_statement_compound() {
    check_statement(
        "{ let x = 3; return x; }",
        expect![[r#"
            SourceFile@0..24
              CompoundStatement@0..24
                BraceLeft@0..1 "{"
                Blankspace@1..2 " "
                LetDeclaration@2..12
                  Let@2..5 "let"
                  Blankspace@5..6 " "
                  Name@6..7
                    Identifier@6..7 "x"
                  Blankspace@7..8 " "
                  Equal@8..9 "="
                  Blankspace@9..10 " "
                  Literal@10..11
                    IntLiteral@10..11 "3"
                  Semicolon@11..12 ";"
                Blankspace@12..13 " "
                ReturnStatement@13..22
                  Return@13..19 "return"
                  Blankspace@19..20 " "
                  IdentExpression@20..21
                    Path@20..21
                      Identifier@20..21 "x"
                  Semicolon@21..22 ";"
                Blankspace@22..23 " "
                BraceRight@23..24 "}""#]],
    );
}

#[test]
fn parse_statement_assignment() {
    check_statement(
        "a = 3;",
        expect![[r#"
            SourceFile@0..6
              AssignmentStatement@0..6
                IdentExpression@0..1
                  Path@0..1
                    Identifier@0..1 "a"
                Blankspace@1..2 " "
                Equal@2..3 "="
                Blankspace@3..4 " "
                Literal@4..5
                  IntLiteral@4..5 "3"
                Semicolon@5..6 ";""#]],
    );
}

#[test]
fn parse_statement_assignment_field() {
    check_statement(
        "a.b = a.c * 3;",
        expect![[r#"
            SourceFile@0..14
              AssignmentStatement@0..14
                FieldExpression@0..3
                  IdentExpression@0..1
                    Path@0..1
                      Identifier@0..1 "a"
                  Period@1..2 "."
                  Identifier@2..3 "b"
                Blankspace@3..4 " "
                Equal@4..5 "="
                Blankspace@5..6 " "
                InfixExpression@6..13
                  FieldExpression@6..9
                    IdentExpression@6..7
                      Path@6..7
                        Identifier@6..7 "a"
                    Period@7..8 "."
                    Identifier@8..9 "c"
                  Blankspace@9..10 " "
                  Star@10..11 "*"
                  Blankspace@11..12 " "
                  Literal@12..13
                    IntLiteral@12..13 "3"
                Semicolon@13..14 ";""#]],
    );
}

#[test]
fn parse_statement_assignment_invalid() {
    check(
        "fn a(){1+2=3;}",
        expect![[r#"
            SourceFile@0..14
              FunctionDeclaration@0..14
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "a"
                FunctionParameters@4..6
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                CompoundStatement@6..14
                  BraceLeft@6..7 "{"
                  Error@7..12
                    IntLiteral@7..8 "1"
                    Plus@8..9 "+"
                    IntLiteral@9..10 "2"
                    Equal@10..11 "="
                    IntLiteral@11..12 "3"
                  EmptyStatement@12..13
                    Semicolon@12..13 ";"
                  BraceRight@13..14 "}"

            error at 7..8: invalid syntax, expected one of: '&', '@', '{', '}', 'break', 'const', 'const_assert', 'continue', 'discard', 'for', <identifier>, 'if', 'let', 'loop', 'package', '(', 'return', ';', '*', 'super', 'switch', '_', 'var', 'while'"#]],
    );
}

#[test]
fn parse_statement_recover() {
    check_statement(
        "{ { let x = } { return 0 } }",
        expect![[r#"
            SourceFile@0..28
              CompoundStatement@0..28
                BraceLeft@0..1 "{"
                Blankspace@1..2 " "
                CompoundStatement@2..13
                  BraceLeft@2..3 "{"
                  Blankspace@3..4 " "
                  LetDeclaration@4..11
                    Let@4..7 "let"
                    Blankspace@7..8 " "
                    Name@8..9
                      Identifier@8..9 "x"
                    Blankspace@9..10 " "
                    Equal@10..11 "="
                  Blankspace@11..12 " "
                  BraceRight@12..13 "}"
                Blankspace@13..14 " "
                CompoundStatement@14..26
                  BraceLeft@14..15 "{"
                  Blankspace@15..16 " "
                  ReturnStatement@16..24
                    Return@16..22 "return"
                    Blankspace@22..23 " "
                    Literal@23..24
                      IntLiteral@23..24 "0"
                  Blankspace@24..25 " "
                  BraceRight@25..26 "}"
                Blankspace@26..27 " "
                BraceRight@27..28 "}"

            error at 12..13: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'
            error at 25..26: invalid syntax, expected: ';'"#]],
    );
}

#[test]
fn parse_missing_lhs_recover() {
    // Amusingly a unary plus is invalid in WGSL
    check_statement(
        "let a = +1;",
        expect![[r#"
            SourceFile@0..11
              LetDeclaration@0..11
                Let@0..3 "let"
                Blankspace@3..4 " "
                Name@4..5
                  Identifier@4..5 "a"
                Blankspace@5..6 " "
                Equal@6..7 "="
                Blankspace@7..8 " "
                InfixExpression@8..10
                  Plus@8..9 "+"
                  Literal@9..10
                    IntLiteral@9..10 "1"
                Semicolon@10..11 ";"

            error at 8..9: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'"#]],
    );
}

#[test]
fn var_recover_elided_name() {
    check(
        "var",
        expect![[r#"
            SourceFile@0..3
              VariableDeclaration@0..3
                Var@0..3 "var"

            error at 3..3: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>"#]],
    );
}

#[test]
fn parse_unary_minus() {
    check_statement(
        "let a = -1;",
        expect![[r#"
            SourceFile@0..11
              LetDeclaration@0..11
                Let@0..3 "let"
                Blankspace@3..4 " "
                Name@4..5
                  Identifier@4..5 "a"
                Blankspace@5..6 " "
                Equal@6..7 "="
                Blankspace@7..8 " "
                PrefixExpression@8..10
                  Minus@8..9 "-"
                  Literal@9..10
                    IntLiteral@9..10 "1"
                Semicolon@10..11 ";""#]],
    );
}

#[test]
fn parse_compound_assignment_statement() {
    check_statement(
        "a += 3;",
        expect![[r#"
            SourceFile@0..7
              CompoundAssignmentStatement@0..7
                IdentExpression@0..1
                  Path@0..1
                    Identifier@0..1 "a"
                Blankspace@1..2 " "
                PlusEqual@2..4 "+="
                Blankspace@4..5 " "
                Literal@5..6
                  IntLiteral@5..6 "3"
                Semicolon@6..7 ";""#]],
    );
}

#[test]
fn parse_compound_assignment_statement_expression() {
    check_statement(
        "*a += foo();",
        expect![[r#"
            SourceFile@0..12
              CompoundAssignmentStatement@0..12
                PrefixExpression@0..2
                  Star@0..1 "*"
                  IdentExpression@1..2
                    Path@1..2
                      Identifier@1..2 "a"
                Blankspace@2..3 " "
                PlusEqual@3..5 "+="
                Blankspace@5..6 " "
                FunctionCall@6..11
                  IdentExpression@6..9
                    Path@6..9
                      Identifier@6..9 "foo"
                  Arguments@9..11
                    ParenthesisLeft@9..10 "("
                    ParenthesisRight@10..11 ")"
                Semicolon@11..12 ";""#]],
    );
}

#[test]
fn parse_phony_assignment_statement() {
    check_statement(
        "_ = *foo;",
        expect![[r#"
            SourceFile@0..9
              PhonyAssignmentStatement@0..9
                Underscore@0..1 "_"
                Blankspace@1..2 " "
                Equal@2..3 "="
                Blankspace@3..4 " "
                PrefixExpression@4..8
                  Star@4..5 "*"
                  IdentExpression@5..8
                    Path@5..8
                      Identifier@5..8 "foo"
                Semicolon@8..9 ";""#]],
    );
}

#[test]
fn parse_indexed_statement() {
    check_statement(
        "a[0] += a[2];",
        expect![[r#"
            SourceFile@0..13
              CompoundAssignmentStatement@0..13
                IndexExpression@0..4
                  IdentExpression@0..1
                    Path@0..1
                      Identifier@0..1 "a"
                  BracketLeft@1..2 "["
                  Literal@2..3
                    IntLiteral@2..3 "0"
                  BracketRight@3..4 "]"
                Blankspace@4..5 " "
                PlusEqual@5..7 "+="
                Blankspace@7..8 " "
                IndexExpression@8..12
                  IdentExpression@8..9
                    Path@8..9
                      Identifier@8..9 "a"
                  BracketLeft@9..10 "["
                  Literal@10..11
                    IntLiteral@10..11 "2"
                  BracketRight@11..12 "]"
                Semicolon@12..13 ";""#]],
    );
}

#[test]
fn parse_var_without_initializer() {
    check_statement(
        "var x: u32;",
        expect![[r#"
            SourceFile@0..11
              VariableDeclaration@0..11
                Var@0..3 "var"
                Blankspace@3..4 " "
                Name@4..5
                  Identifier@4..5 "x"
                Colon@5..6 ":"
                Blankspace@6..7 " "
                TypeSpecifier@7..10
                  Path@7..10
                    Identifier@7..10 "u32"
                Semicolon@10..11 ";""#]],
    );
}

#[test]
fn parse_var_with_initializer() {
    check_statement(
        "var<function> x: u32;",
        expect![[r#"
            SourceFile@0..21
              VariableDeclaration@0..21
                Var@0..3 "var"
                TemplateList@3..13
                  TemplateStart@3..4 "<"
                  IdentExpression@4..12
                    Path@4..12
                      Identifier@4..12 "function"
                  TemplateEnd@12..13 ">"
                Blankspace@13..14 " "
                Name@14..15
                  Identifier@14..15 "x"
                Colon@15..16 ":"
                Blankspace@16..17 " "
                TypeSpecifier@17..20
                  Path@17..20
                    Identifier@17..20 "u32"
                Semicolon@20..21 ";""#]],
    );
}

#[test]
fn attribute_list_modern() {
    check_attribute(
        "@location(0)",
        expect![[r#"
            SourceFile@0..12
              LocationAttribute@0..12
                AttributeOperator@0..1 "@"
                Location@1..9 "location"
                ParenthesisLeft@9..10 "("
                Literal@10..11
                  IntLiteral@10..11 "0"
                ParenthesisRight@11..12 ")""#]],
    );
    check_attribute(
        "@interpolate(flat)",
        expect![[r#"
            SourceFile@0..18
              InterpolateAttribute@0..18
                AttributeOperator@0..1 "@"
                Interpolate@1..12 "interpolate"
                ParenthesisLeft@12..13 "("
                InterpolateTypeName@13..17
                  Flat@13..17 "flat"
                ParenthesisRight@17..18 ")""#]],
    );
    check_attribute(
        "@attr(1, 2, 0.0, ident)",
        expect![[r#"
            SourceFile@0..23
              OtherAttribute@0..23
                AttributeOperator@0..1 "@"
                Identifier@1..5 "attr"
                Arguments@5..23
                  ParenthesisLeft@5..6 "("
                  Literal@6..7
                    IntLiteral@6..7 "1"
                  Comma@7..8 ","
                  Blankspace@8..9 " "
                  Literal@9..10
                    IntLiteral@9..10 "2"
                  Comma@10..11 ","
                  Blankspace@11..12 " "
                  Literal@12..15
                    FloatLiteral@12..15 "0.0"
                  Comma@15..16 ","
                  Blankspace@16..17 " "
                  IdentExpression@17..22
                    Path@17..22
                      Identifier@17..22 "ident"
                  ParenthesisRight@22..23 ")""#]],
    );
}

#[test]
fn fn_recover_incomplete_param() {
    check(
        "fn main(p) {}",
        expect![[r#"
            SourceFile@0..13
              FunctionDeclaration@0..13
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                FunctionParameters@7..10
                  ParenthesisLeft@7..8 "("
                  Parameter@8..9
                    Name@8..9
                      Identifier@8..9 "p"
                    TypeSpecifier@9..9
                  ParenthesisRight@9..10 ")"
                Blankspace@10..11 " "
                CompoundStatement@11..13
                  BraceLeft@11..12 "{"
                  BraceRight@12..13 "}"

            error at 9..10: invalid syntax, expected: ':'"#]],
    );
}

#[test]
fn let_statement_recover_return_no_eq() {
    check(
        "fn main() {
            let x be
        }",
        expect![[r#"
            SourceFile@0..42
              FunctionDeclaration@0..42
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..42
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  LetDeclaration@24..42
                    Let@24..27 "let"
                    Blankspace@27..28 " "
                    Name@28..29
                      Identifier@28..29 "x"
                    Blankspace@29..30 " "
                    IdentExpression@30..42
                      Path@30..32
                        Identifier@30..32 "be"
                      Blankspace@32..41 "\n        "
                      Error@41..42
                        BraceRight@41..42 "}"

            error at 30..32: invalid syntax, expected one of: ':', '=', ';'
            error at 41..42: invalid syntax, expected one of: '&', '&&', '@', '{', '[', ']', ':', ',', '==', '/', '>', '>=', '<', '<=', '-', '%', '!=', '|', '||', '(', ')', '.', '+', ';', '<<', '>>', '*', <template end>, <template start>, '^'"#]],
    );
}

#[test]
fn let_statement_recover_return() {
    check(
        "fn main() {
            let
            return 0;
        }",
        expect![[r#"
            SourceFile@0..59
              FunctionDeclaration@0..59
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..59
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  LetDeclaration@24..40
                    Let@24..27 "let"
                    Blankspace@27..40 "\n            "
                  ReturnStatement@40..49
                    Return@40..46 "return"
                    Blankspace@46..47 " "
                    Literal@47..48
                      IntLiteral@47..48 "0"
                    Semicolon@48..49 ";"
                  Blankspace@49..58 "\n        "
                  BraceRight@58..59 "}"

            error at 40..46: invalid syntax, expected: <identifier>"#]],
    );
}

#[test]
fn let_statement_recover_return_2() {
    check(
        "fn main() {
            let x
            return 0;
        }",
        expect![[r#"
            SourceFile@0..61
              FunctionDeclaration@0..61
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..61
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  LetDeclaration@24..29
                    Let@24..27 "let"
                    Blankspace@27..28 " "
                    Name@28..29
                      Identifier@28..29 "x"
                  Blankspace@29..42 "\n            "
                  ReturnStatement@42..51
                    Return@42..48 "return"
                    Blankspace@48..49 " "
                    Literal@49..50
                      IntLiteral@49..50 "0"
                    Semicolon@50..51 ";"
                  Blankspace@51..60 "\n        "
                  BraceRight@60..61 "}"

            error at 42..48: invalid syntax, expected one of: ':', '=', ';'"#]],
    );
}

#[test]
fn let_statement_recover_return_3() {
    check(
        "fn main() {
            let x =
            return 0;
        }",
        expect![[r#"
            SourceFile@0..63
              FunctionDeclaration@0..63
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..63
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  LetDeclaration@24..31
                    Let@24..27 "let"
                    Blankspace@27..28 " "
                    Name@28..29
                      Identifier@28..29 "x"
                    Blankspace@29..30 " "
                    Equal@30..31 "="
                  Blankspace@31..44 "\n            "
                  ReturnStatement@44..53
                    Return@44..50 "return"
                    Blankspace@50..51 " "
                    Literal@51..52
                      IntLiteral@51..52 "0"
                    Semicolon@52..53 ";"
                  Blankspace@53..62 "\n        "
                  BraceRight@62..63 "}"

            error at 44..50: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'"#]],
    );
}

#[test]
fn let_statement_recover_1() {
    check(
        "fn main() {
            let x
        }",
        expect![[r#"
            SourceFile@0..39
              FunctionDeclaration@0..39
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..39
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  LetDeclaration@24..29
                    Let@24..27 "let"
                    Blankspace@27..28 " "
                    Name@28..29
                      Identifier@28..29 "x"
                  Blankspace@29..38 "\n        "
                  BraceRight@38..39 "}"

            error at 38..39: invalid syntax, expected one of: ':', '=', ';'"#]],
    );
}

#[test]
fn let_statement_recover_2() {
    check(
        "fn main() {
            let x =
        }",
        expect![[r#"
            SourceFile@0..41
              FunctionDeclaration@0..41
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..41
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  LetDeclaration@24..31
                    Let@24..27 "let"
                    Blankspace@27..28 " "
                    Name@28..29
                      Identifier@28..29 "x"
                    Blankspace@29..30 " "
                    Equal@30..31 "="
                  Blankspace@31..40 "\n        "
                  BraceRight@40..41 "}"

            error at 40..41: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'"#]],
    );
}

#[test]
fn let_statement_recover_3() {
    check(
        "fn main() {
            let
        }",
        expect![[r#"
            SourceFile@0..37
              FunctionDeclaration@0..37
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                FunctionParameters@7..9
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..37
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  LetDeclaration@24..36
                    Let@24..27 "let"
                    Blankspace@27..36 "\n        "
                  BraceRight@36..37 "}"

            error at 36..37: invalid syntax, expected: <identifier>"#]],
    );
}

#[test]
fn annotation_with_invalid_statement_recover() {
    check(
        "fn foo() {
    @if(MIXOKLAB_SRGB)
    let colorA = srgb2rgb(colA);
    @else
    let colorA = colA;
}
}",
        expect![[r#"
            SourceFile@0..103
              FunctionDeclaration@0..72
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..6
                  Identifier@3..6 "foo"
                FunctionParameters@6..8
                  ParenthesisLeft@6..7 "("
                  ParenthesisRight@7..8 ")"
                Blankspace@8..9 " "
                CompoundStatement@9..72
                  BraceLeft@9..10 "{"
                  Blankspace@10..15 "\n    "
                  IfStatement@15..38
                    Attribute@15..16
                      AttributeOperator@15..16 "@"
                    IfClause@16..38
                      If@16..18 "if"
                      ParenthesisExpression@18..33
                        ParenthesisLeft@18..19 "("
                        IdentExpression@19..32
                          Path@19..32
                            Identifier@19..32 "MIXOKLAB_SRGB"
                        ParenthesisRight@32..33 ")"
                      Blankspace@33..38 "\n    "
                  LetDeclaration@38..66
                    Let@38..41 "let"
                    Blankspace@41..42 " "
                    Name@42..48
                      Identifier@42..48 "colorA"
                    Blankspace@48..49 " "
                    Equal@49..50 "="
                    Blankspace@50..51 " "
                    FunctionCall@51..65
                      IdentExpression@51..59
                        Path@51..59
                          Identifier@51..59 "srgb2rgb"
                      Arguments@59..65
                        ParenthesisLeft@59..60 "("
                        IdentExpression@60..64
                          Path@60..64
                            Identifier@60..64 "colA"
                        ParenthesisRight@64..65 ")"
                    Semicolon@65..66 ";"
                  Blankspace@66..71 "\n    "
                  Error@71..72
                    Attribute@71..72
                      AttributeOperator@71..72 "@"
              Error@72..76
                Else@72..76 "else"
              Blankspace@76..81 "\n    "
              Error@81..99
                Let@81..84 "let"
                Blankspace@84..85 " "
                Name@85..91
                  Identifier@85..91 "colorA"
                Blankspace@91..92 " "
                Equal@92..93 "="
                Blankspace@93..94 " "
                IdentExpression@94..98
                  Path@94..98
                    Identifier@94..98 "colA"
                Semicolon@98..99 ";"
              Blankspace@99..100 "\n"
              Error@100..103
                BraceRight@100..101 "}"
                Blankspace@101..102 "\n"
                BraceRight@102..103 "}"

            error at 16..18: invalid syntax, expected one of: 'align', 'binding', 'blend_src', 'builtin', 'compute', 'const', 'diagnostic', 'fragment', 'group', 'id', <identifier>, 'interpolate', 'invariant', 'location', 'must_use', 'size', 'vertex', 'workgroup_size'
            error at 38..41: invalid syntax, expected one of: '@', '{'
            error at 72..76: invalid syntax, expected one of: 'align', 'binding', 'blend_src', 'builtin', 'compute', 'const', 'diagnostic', 'fragment', 'group', 'id', <identifier>, 'interpolate', 'invariant', 'location', 'must_use', 'size', 'vertex', 'workgroup_size'
            error at 81..99: global let declarations are not allowed
            error at 100..101: invalid syntax, expected one of: 'alias', '@', 'const', 'const_assert', 'diagnostic', <end of file>, 'enable', 'fn', 'import', 'let', 'override', 'requires', ';', 'struct', 'var'"#]],
    );
}

#[test]
fn weird_blankspace() {
    check(
        "\u{0020}\u{0009}\u{000A}\u{000B}\u{000C}\u{000D}\u{0085}\u{200E}\u{200F}\u{2028}\u{2029}",
        expect![[r#"
            SourceFile@0..20
              Blankspace@0..20 " \t\n\u{b}\u{c}\r\u{85}\u{200e}\u{200f}\u{2028}\u{2029}""#]],
    );
}

#[test]
fn tabs() {
    check(
        "
			fn foo() {}
            fn bar() {}
			fn baz() {}
        ",
        expect![[r#"
            SourceFile@0..63
              Blankspace@0..4 "\n\t\t\t"
              FunctionDeclaration@4..15
                Fn@4..6 "fn"
                Blankspace@6..7 " "
                Name@7..10
                  Identifier@7..10 "foo"
                FunctionParameters@10..12
                  ParenthesisLeft@10..11 "("
                  ParenthesisRight@11..12 ")"
                Blankspace@12..13 " "
                CompoundStatement@13..15
                  BraceLeft@13..14 "{"
                  BraceRight@14..15 "}"
              Blankspace@15..28 "\n            "
              FunctionDeclaration@28..39
                Fn@28..30 "fn"
                Blankspace@30..31 " "
                Name@31..34
                  Identifier@31..34 "bar"
                FunctionParameters@34..36
                  ParenthesisLeft@34..35 "("
                  ParenthesisRight@35..36 ")"
                Blankspace@36..37 " "
                CompoundStatement@37..39
                  BraceLeft@37..38 "{"
                  BraceRight@38..39 "}"
              Blankspace@39..43 "\n\t\t\t"
              FunctionDeclaration@43..54
                Fn@43..45 "fn"
                Blankspace@45..46 " "
                Name@46..49
                  Identifier@46..49 "baz"
                FunctionParameters@49..51
                  ParenthesisLeft@49..50 "("
                  ParenthesisRight@50..51 ")"
                Blankspace@51..52 " "
                CompoundStatement@52..54
                  BraceLeft@52..53 "{"
                  BraceRight@53..54 "}"
              Blankspace@54..63 "\n        ""#]],
    );
}

#[test]
fn weird_line_ending_comments() {
    check(
        "// line feed: \u{000A}// vertical tab: \u{000B}// form feed: \u{000C}// carriage return when not also followed by line feed: \u{000D}// carriage return followed by line feed: \u{000D}\u{000A}// next line: \u{0085}// line separator: \u{2028}// paragraph separator: \u{2029}",
        expect![[r#"
            SourceFile@0..214
              LineEndingComment@0..14 "// line feed: "
              Blankspace@14..15 "\n"
              LineEndingComment@15..32 "// vertical tab: "
              Blankspace@32..33 "\u{b}"
              LineEndingComment@33..47 "// form feed: "
              Blankspace@47..48 "\u{c}"
              LineEndingComment@48..104 "// carriage return wh ..."
              Blankspace@104..105 "\r"
              LineEndingComment@105..147 "// carriage return fo ..."
              Blankspace@147..149 "\r\n"
              LineEndingComment@149..163 "// next line: "
              Blankspace@163..165 "\u{85}"
              LineEndingComment@165..184 "// line separator: "
              Blankspace@184..187 "\u{2028}"
              LineEndingComment@187..211 "// paragraph separator: "
              Blankspace@211..214 "\u{2029}""#]],
    );
}

#[test]
fn enable_directive() {
    check(
        "enable f16,clip_distances,  dual_source_blending;",
        expect![[r#"
            SourceFile@0..49
              EnableDirective@0..49
                Enable@0..6 "enable"
                Blankspace@6..7 " "
                EnableExtensionName@7..10
                  Identifier@7..10 "f16"
                Comma@10..11 ","
                EnableExtensionName@11..25
                  Identifier@11..25 "clip_distances"
                Comma@25..26 ","
                Blankspace@26..28 "  "
                EnableExtensionName@28..48
                  Identifier@28..48 "dual_source_blending"
                Semicolon@48..49 ";""#]],
    );
}

#[test]
fn requires_directive() {
    check(
        "requires packed_4x8_integer_dot_product;",
        expect![[r#"
            SourceFile@0..40
              RequiresDirective@0..40
                Requires@0..8 "requires"
                Blankspace@8..9 " "
                LanguageExtensionName@9..39
                  Identifier@9..39 "packed_4x8_integer_do ..."
                Semicolon@39..40 ";""#]],
    );
}

#[test]
fn struct_underscore_field_name() {
    check(
        "
struct UBO {
  camera_position: vec3f,
  _pad: u32
  time: f32,
};
",
        expect![[r#"
            SourceFile@0..68
              Blankspace@0..1 "\n"
              StructDeclaration@1..66
                Struct@1..7 "struct"
                Blankspace@7..8 " "
                Name@8..11
                  Identifier@8..11 "UBO"
                Blankspace@11..12 " "
                StructBody@12..66
                  BraceLeft@12..13 "{"
                  Blankspace@13..16 "\n  "
                  StructMember@16..38
                    Name@16..31
                      Identifier@16..31 "camera_position"
                    Colon@31..32 ":"
                    Blankspace@32..33 " "
                    TypeSpecifier@33..38
                      Path@33..38
                        Identifier@33..38 "vec3f"
                  Comma@38..39 ","
                  Blankspace@39..42 "\n  "
                  StructMember@42..51
                    Name@42..46
                      Identifier@42..46 "_pad"
                    Colon@46..47 ":"
                    Blankspace@47..48 " "
                    TypeSpecifier@48..51
                      Path@48..51
                        Identifier@48..51 "u32"
                  Blankspace@51..54 "\n  "
                  StructMember@54..63
                    Name@54..58
                      Identifier@54..58 "time"
                    Colon@58..59 ":"
                    Blankspace@59..60 " "
                    TypeSpecifier@60..63
                      Path@60..63
                        Identifier@60..63 "f32"
                  Comma@63..64 ","
                  Blankspace@64..65 "\n"
                  BraceRight@65..66 "}"
              Semicolon@66..67 ";"
              Blankspace@67..68 "\n"

            error at 54..58: invalid syntax, expected ','"#]],
    );
}

#[test]
fn struct_declaration_semicolon() {
    check(
        "
struct Test {
    a: f32;
    b: vec3<f32>;
}
",
        expect![[r#"
            SourceFile@0..47
              Blankspace@0..1 "\n"
              StructDeclaration@1..46
                Struct@1..7 "struct"
                Blankspace@7..8 " "
                Name@8..12
                  Identifier@8..12 "Test"
                Blankspace@12..13 " "
                StructBody@13..46
                  BraceLeft@13..14 "{"
                  Blankspace@14..19 "\n    "
                  StructMember@19..25
                    Name@19..20
                      Identifier@19..20 "a"
                    Colon@20..21 ":"
                    Blankspace@21..22 " "
                    TypeSpecifier@22..25
                      Path@22..25
                        Identifier@22..25 "f32"
                  Semicolon@25..26 ";"
                  Blankspace@26..31 "\n    "
                  StructMember@31..43
                    Name@31..32
                      Identifier@31..32 "b"
                    Colon@32..33 ":"
                    Blankspace@33..34 " "
                    TypeSpecifier@34..43
                      Path@34..38
                        Identifier@34..38 "vec3"
                      TemplateList@38..43
                        TemplateStart@38..39 "<"
                        IdentExpression@39..42
                          Path@39..42
                            Identifier@39..42 "f32"
                        TemplateEnd@42..43 ">"
                  Error@43..44
                    Semicolon@43..44 ";"
                  Blankspace@44..45 "\n"
                  BraceRight@45..46 "}"
              Blankspace@46..47 "\n"

            error at 31..32: invalid syntax, expected ','
            error at 43..44: invalid syntax, expected one of: '@', '}', ',', <identifier>, ';'"#]],
    );
}

#[test]
fn struct_declaration() {
    check(
        "
struct Test {
    a: f32,
    b: vec3<f32>,
}
",
        expect![[r#"
            SourceFile@0..47
              Blankspace@0..1 "\n"
              StructDeclaration@1..46
                Struct@1..7 "struct"
                Blankspace@7..8 " "
                Name@8..12
                  Identifier@8..12 "Test"
                Blankspace@12..13 " "
                StructBody@13..46
                  BraceLeft@13..14 "{"
                  Blankspace@14..19 "\n    "
                  StructMember@19..25
                    Name@19..20
                      Identifier@19..20 "a"
                    Colon@20..21 ":"
                    Blankspace@21..22 " "
                    TypeSpecifier@22..25
                      Path@22..25
                        Identifier@22..25 "f32"
                  Comma@25..26 ","
                  Blankspace@26..31 "\n    "
                  StructMember@31..43
                    Name@31..32
                      Identifier@31..32 "b"
                    Colon@32..33 ":"
                    Blankspace@33..34 " "
                    TypeSpecifier@34..43
                      Path@34..38
                        Identifier@34..38 "vec3"
                      TemplateList@38..43
                        TemplateStart@38..39 "<"
                        IdentExpression@39..42
                          Path@39..42
                            Identifier@39..42 "f32"
                        TemplateEnd@42..43 ">"
                  Comma@43..44 ","
                  Blankspace@44..45 "\n"
                  BraceRight@45..46 "}"
              Blankspace@46..47 "\n""#]],
    );
}

#[test]
fn struct_recover() {
    check(
        "
struct
fn test()
",
        expect![[r#"
            SourceFile@0..18
              Blankspace@0..1 "\n"
              StructDeclaration@1..8
                Struct@1..7 "struct"
                Blankspace@7..8 "\n"
              FunctionDeclaration@8..18
                Fn@8..10 "fn"
                Blankspace@10..11 " "
                Name@11..15
                  Identifier@11..15 "test"
                FunctionParameters@15..17
                  ParenthesisLeft@15..16 "("
                  ParenthesisRight@16..17 ")"
                Blankspace@17..18 "\n"

            error at 8..10: invalid syntax, expected: <identifier>
            error at 18..18: invalid syntax, expected one of: '->', '@', '{'"#]],
    );
}

#[test]
fn struct_recover_2() {
    check(
        "
struct test
fn test()
",
        expect![[r#"
            SourceFile@0..23
              Blankspace@0..1 "\n"
              StructDeclaration@1..13
                Struct@1..7 "struct"
                Blankspace@7..8 " "
                Name@8..12
                  Identifier@8..12 "test"
                Blankspace@12..13 "\n"
              FunctionDeclaration@13..23
                Fn@13..15 "fn"
                Blankspace@15..16 " "
                Name@16..20
                  Identifier@16..20 "test"
                FunctionParameters@20..22
                  ParenthesisLeft@20..21 "("
                  ParenthesisRight@21..22 ")"
                Blankspace@22..23 "\n"

            error at 13..15: invalid syntax, expected: '{'
            error at 23..23: invalid syntax, expected one of: '->', '@', '{'"#]],
    );
}

#[test]
fn struct_recover_3() {
    check(
        "
struct test {}

fn test()
};
",
        expect![[r#"
            SourceFile@0..30
              Blankspace@0..1 "\n"
              StructDeclaration@1..15
                Struct@1..7 "struct"
                Blankspace@7..8 " "
                Name@8..12
                  Identifier@8..12 "test"
                Blankspace@12..13 " "
                StructBody@13..15
                  BraceLeft@13..14 "{"
                  StructMember@14..14
                    TypeSpecifier@14..14
                  BraceRight@14..15 "}"
              Blankspace@15..17 "\n\n"
              FunctionDeclaration@17..28
                Fn@17..19 "fn"
                Blankspace@19..20 " "
                Name@20..24
                  Identifier@20..24 "test"
                FunctionParameters@24..26
                  ParenthesisLeft@24..25 "("
                  ParenthesisRight@25..26 ")"
                Blankspace@26..27 "\n"
                Error@27..28
                  BraceRight@27..28 "}"
              Semicolon@28..29 ";"
              Blankspace@29..30 "\n"

            error at 14..15: invalid syntax, expected one of: '@', <identifier>
            error at 27..28: invalid syntax, expected one of: '->', '@', '{'
            error at 28..29: invalid syntax, expected one of: '@', '{'"#]],
    );
}

#[test]
fn global_variable_declaration_init() {
    check(
        "var flags = 0;",
        expect![[r#"
            SourceFile@0..14
              VariableDeclaration@0..14
                Var@0..3 "var"
                Blankspace@3..4 " "
                Name@4..9
                  Identifier@4..9 "flags"
                Blankspace@9..10 " "
                Equal@10..11 "="
                Blankspace@11..12 " "
                Literal@12..13
                  IntLiteral@12..13 "0"
                Semicolon@13..14 ";""#]],
    );
}

#[test]
fn global_constant_declaration() {
    check(
        "const constant = 0;",
        expect![[r#"
            SourceFile@0..19
              ConstantDeclaration@0..19
                Const@0..5 "const"
                Blankspace@5..6 " "
                Name@6..14
                  Identifier@6..14 "constant"
                Blankspace@14..15 " "
                Equal@15..16 "="
                Blankspace@16..17 " "
                Literal@17..18
                  IntLiteral@17..18 "0"
                Semicolon@18..19 ";""#]],
    );
}

#[test]
fn type_alias_declaration() {
    check(
        "alias float = f32;",
        expect![[r#"
            SourceFile@0..18
              TypeAliasDeclaration@0..18
                Alias@0..5 "alias"
                Blankspace@5..6 " "
                Name@6..11
                  Identifier@6..11 "float"
                Blankspace@11..12 " "
                Equal@12..13 "="
                Blankspace@13..14 " "
                TypeSpecifier@14..17
                  Path@14..17
                    Identifier@14..17 "f32"
                Semicolon@17..18 ";""#]],
    );
}

#[test]
fn type_alias_declaration_recover() {
    check(
        "alias float = f32\nalias other = u32;",
        expect![[r#"
            SourceFile@0..36
              TypeAliasDeclaration@0..17
                Alias@0..5 "alias"
                Blankspace@5..6 " "
                Name@6..11
                  Identifier@6..11 "float"
                Blankspace@11..12 " "
                Equal@12..13 "="
                Blankspace@13..14 " "
                TypeSpecifier@14..17
                  Path@14..17
                    Identifier@14..17 "f32"
              Blankspace@17..18 "\n"
              TypeAliasDeclaration@18..36
                Alias@18..23 "alias"
                Blankspace@23..24 " "
                Name@24..29
                  Identifier@24..29 "other"
                Blankspace@29..30 " "
                Equal@30..31 "="
                Blankspace@31..32 " "
                TypeSpecifier@32..35
                  Path@32..35
                    Identifier@32..35 "u32"
                Semicolon@35..36 ";"

            error at 18..23: invalid syntax, expected one of: '&', '&&', '&=', '@', '{', '}', '[', ']', ':', '::', ',', '/=', '=', '==', '/', '>', '>=', <identifier>, '<', '<=', '-', '-=', '--', '%', '%=', '!=', '|', '|=', '||', '(', ')', '.', '+', '+=', '++', ';', '<<', '<<=', '>>', '>>=', '*', <template end>, <template start>, '*=', '^', '^='"#]],
    );
}

#[test]
fn parse_statement_expression() {
    check_statement(
        "test(arguments);",
        expect![[r#"
            SourceFile@0..16
              FunctionCallStatement@0..16
                FunctionCall@0..15
                  IdentExpression@0..4
                    Path@0..4
                      Identifier@0..4 "test"
                  Arguments@4..15
                    ParenthesisLeft@4..5 "("
                    IdentExpression@5..14
                      Path@5..14
                        Identifier@5..14 "arguments"
                    ParenthesisRight@14..15 ")"
                Semicolon@15..16 ";""#]],
    );
}

#[test]
fn parse_statement_nested_functions() {
    check_statement(
        "test(arguments<a>());",
        expect![[r#"
            SourceFile@0..21
              FunctionCallStatement@0..21
                FunctionCall@0..20
                  IdentExpression@0..4
                    Path@0..4
                      Identifier@0..4 "test"
                  Arguments@4..20
                    ParenthesisLeft@4..5 "("
                    FunctionCall@5..19
                      IdentExpression@5..17
                        Path@5..14
                          Identifier@5..14 "arguments"
                        TemplateList@14..17
                          TemplateStart@14..15 "<"
                          IdentExpression@15..16
                            Path@15..16
                              Identifier@15..16 "a"
                          TemplateEnd@16..17 ">"
                      Arguments@17..19
                        ParenthesisLeft@17..18 "("
                        ParenthesisRight@18..19 ")"
                    ParenthesisRight@19..20 ")"
                Semicolon@20..21 ";""#]],
    );
}

#[test]
fn loop_statement() {
    check_statement(
        "loop {}",
        expect![[r#"
            SourceFile@0..7
              LoopStatement@0..7
                Loop@0..4 "loop"
                Blankspace@4..5 " "
                CompoundStatement@5..7
                  BraceLeft@5..6 "{"
                  BraceRight@6..7 "}""#]],
    );
}

#[test]
fn empty_return_statement() {
    check_statement(
        "return;",
        expect![[r#"
            SourceFile@0..7
              ReturnStatement@0..7
                Return@0..6 "return"
                Semicolon@6..7 ";""#]],
    );
}

#[test]
fn empty_return_statement_no_semi() {
    check_statement(
        "{ let x = 3; return x } ",
        expect![[r#"
            SourceFile@0..24
              CompoundStatement@0..23
                BraceLeft@0..1 "{"
                Blankspace@1..2 " "
                LetDeclaration@2..12
                  Let@2..5 "let"
                  Blankspace@5..6 " "
                  Name@6..7
                    Identifier@6..7 "x"
                  Blankspace@7..8 " "
                  Equal@8..9 "="
                  Blankspace@9..10 " "
                  Literal@10..11
                    IntLiteral@10..11 "3"
                  Semicolon@11..12 ";"
                Blankspace@12..13 " "
                ReturnStatement@13..23
                  Return@13..19 "return"
                  Blankspace@19..20 " "
                  IdentExpression@20..23
                    Path@20..21
                      Identifier@20..21 "x"
                    Blankspace@21..22 " "
                    Error@22..23
                      BraceRight@22..23 "}"
              Blankspace@23..24 " "

            error at 22..23: invalid syntax, expected one of: '&', '&&', '@', '{', '[', ']', ':', ',', '==', '/', '>', '>=', '<', '<=', '-', '%', '!=', '|', '||', '(', ')', '.', '+', ';', '<<', '>>', '*', <template end>, <template start>, '^'"#]],
    );
}

#[test]
fn parse_switch_statement() {
    check_statement(
        "
switch i {
  case 0: { }
  case 1, 2: { return 42; }
  default: { }
}
        ",
        expect![[r#"
            SourceFile@0..79
              Blankspace@0..1 "\n"
              SwitchStatement@1..70
                Switch@1..7 "switch"
                Blankspace@7..8 " "
                IdentExpression@8..9
                  Path@8..9
                    Identifier@8..9 "i"
                Blankspace@9..10 " "
                SwitchBody@10..70
                  BraceLeft@10..11 "{"
                  Blankspace@11..14 "\n  "
                  SwitchBodyCase@14..25
                    Case@14..18 "case"
                    Blankspace@18..19 " "
                    SwitchCaseSelectors@19..20
                      Literal@19..20
                        IntLiteral@19..20 "0"
                    Colon@20..21 ":"
                    Blankspace@21..22 " "
                    CompoundStatement@22..25
                      BraceLeft@22..23 "{"
                      Blankspace@23..24 " "
                      BraceRight@24..25 "}"
                  Blankspace@25..28 "\n  "
                  SwitchBodyCase@28..53
                    Case@28..32 "case"
                    Blankspace@32..33 " "
                    SwitchCaseSelectors@33..37
                      Literal@33..34
                        IntLiteral@33..34 "1"
                      Comma@34..35 ","
                      Blankspace@35..36 " "
                      Literal@36..37
                        IntLiteral@36..37 "2"
                    Colon@37..38 ":"
                    Blankspace@38..39 " "
                    CompoundStatement@39..53
                      BraceLeft@39..40 "{"
                      Blankspace@40..41 " "
                      ReturnStatement@41..51
                        Return@41..47 "return"
                        Blankspace@47..48 " "
                        Literal@48..50
                          IntLiteral@48..50 "42"
                        Semicolon@50..51 ";"
                      Blankspace@51..52 " "
                      BraceRight@52..53 "}"
                  Blankspace@53..56 "\n  "
                  SwitchBodyCase@56..68
                    Default@56..63 "default"
                    Colon@63..64 ":"
                    Blankspace@64..65 " "
                    CompoundStatement@65..68
                      BraceLeft@65..66 "{"
                      Blankspace@66..67 " "
                      BraceRight@67..68 "}"
                  Blankspace@68..69 "\n"
                  BraceRight@69..70 "}"
              Blankspace@70..79 "\n        ""#]],
    );
}

#[test]
fn parse_switch_statement_with_default_pattern() {
    check_statement(
        "
switch i {
  case 0, default, 1,: { }
}
        ",
        expect![[r#"
            SourceFile@0..49
              Blankspace@0..1 "\n"
              SwitchStatement@1..40
                Switch@1..7 "switch"
                Blankspace@7..8 " "
                IdentExpression@8..9
                  Path@8..9
                    Identifier@8..9 "i"
                Blankspace@9..10 " "
                SwitchBody@10..40
                  BraceLeft@10..11 "{"
                  Blankspace@11..14 "\n  "
                  SwitchBodyCase@14..38
                    Case@14..18 "case"
                    Blankspace@18..19 " "
                    SwitchCaseSelectors@19..33
                      Literal@19..20
                        IntLiteral@19..20 "0"
                      Comma@20..21 ","
                      Blankspace@21..22 " "
                      SwitchDefaultSelector@22..29
                        Default@22..29 "default"
                      Comma@29..30 ","
                      Blankspace@30..31 " "
                      Literal@31..32
                        IntLiteral@31..32 "1"
                      Comma@32..33 ","
                    Colon@33..34 ":"
                    Blankspace@34..35 " "
                    CompoundStatement@35..38
                      BraceLeft@35..36 "{"
                      Blankspace@36..37 " "
                      BraceRight@37..38 "}"
                  Blankspace@38..39 "\n"
                  BraceRight@39..40 "}"
              Blankspace@40..49 "\n        ""#]],
    );
}

#[test]
fn parse_switch_statement_recover_1() {
    check_statement(
        "
switch i {
  case
}
        ",
        expect![[r#"
            SourceFile@0..29
              Blankspace@0..1 "\n"
              SwitchStatement@1..20
                Switch@1..7 "switch"
                Blankspace@7..8 " "
                IdentExpression@8..9
                  Path@8..9
                    Identifier@8..9 "i"
                Blankspace@9..10 " "
                SwitchBody@10..20
                  BraceLeft@10..11 "{"
                  Blankspace@11..14 "\n  "
                  SwitchBodyCase@14..19
                    Case@14..18 "case"
                    Blankspace@18..19 "\n"
                  BraceRight@19..20 "}"
              Blankspace@20..29 "\n        "

            error at 19..20: invalid syntax, expected one of: '&', '!', 'default', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'"#]],
    );
}

#[test]
fn parse_switch_statement_recover_2() {
    check_statement(
        "
switch i {
  case 1
}
        ",
        expect![[r#"
            SourceFile@0..31
              Blankspace@0..1 "\n"
              SwitchStatement@1..22
                Switch@1..7 "switch"
                Blankspace@7..8 " "
                IdentExpression@8..9
                  Path@8..9
                    Identifier@8..9 "i"
                Blankspace@9..10 " "
                SwitchBody@10..22
                  BraceLeft@10..11 "{"
                  Blankspace@11..14 "\n  "
                  SwitchBodyCase@14..21
                    Case@14..18 "case"
                    Blankspace@18..19 " "
                    SwitchCaseSelectors@19..20
                      Literal@19..20
                        IntLiteral@19..20 "1"
                    Blankspace@20..21 "\n"
                  BraceRight@21..22 "}"
              Blankspace@22..31 "\n        "

            error at 21..22: invalid syntax, expected one of: '@', '{', ':', ','"#]],
    );
}

#[test]
fn parse_switch_statement_recover_3() {
    check_statement(
        "
{
switch i {
  case 1:
}

let x = 3;
}
        ",
        expect![[r#"
            SourceFile@0..48
              Blankspace@0..1 "\n"
              CompoundStatement@1..39
                BraceLeft@1..2 "{"
                Blankspace@2..3 "\n"
                SwitchStatement@3..25
                  Switch@3..9 "switch"
                  Blankspace@9..10 " "
                  IdentExpression@10..11
                    Path@10..11
                      Identifier@10..11 "i"
                  Blankspace@11..12 " "
                  SwitchBody@12..25
                    BraceLeft@12..13 "{"
                    Blankspace@13..16 "\n  "
                    SwitchBodyCase@16..24
                      Case@16..20 "case"
                      Blankspace@20..21 " "
                      SwitchCaseSelectors@21..22
                        Literal@21..22
                          IntLiteral@21..22 "1"
                      Colon@22..23 ":"
                      Blankspace@23..24 "\n"
                    BraceRight@24..25 "}"
                Blankspace@25..27 "\n\n"
                LetDeclaration@27..37
                  Let@27..30 "let"
                  Blankspace@30..31 " "
                  Name@31..32
                    Identifier@31..32 "x"
                  Blankspace@32..33 " "
                  Equal@33..34 "="
                  Blankspace@34..35 " "
                  Literal@35..36
                    IntLiteral@35..36 "3"
                  Semicolon@36..37 ";"
                Blankspace@37..38 "\n"
                BraceRight@38..39 "}"
              Blankspace@39..48 "\n        "

            error at 24..25: invalid syntax, expected one of: '@', '{'"#]],
    );
}

#[test]
fn parse_switch_statement_recover_4() {
    check_statement(
        "
{
switch i {
  case 1, 2,
}
let x = 3;
}
        ",
        expect![[r#"
            SourceFile@0..50
              Blankspace@0..1 "\n"
              CompoundStatement@1..41
                BraceLeft@1..2 "{"
                Blankspace@2..3 "\n"
                SwitchStatement@3..28
                  Switch@3..9 "switch"
                  Blankspace@9..10 " "
                  IdentExpression@10..11
                    Path@10..11
                      Identifier@10..11 "i"
                  Blankspace@11..12 " "
                  SwitchBody@12..28
                    BraceLeft@12..13 "{"
                    Blankspace@13..16 "\n  "
                    SwitchBodyCase@16..27
                      Case@16..20 "case"
                      Blankspace@20..21 " "
                      SwitchCaseSelectors@21..26
                        Literal@21..22
                          IntLiteral@21..22 "1"
                        Comma@22..23 ","
                        Blankspace@23..24 " "
                        Literal@24..25
                          IntLiteral@24..25 "2"
                        Comma@25..26 ","
                      Blankspace@26..27 "\n"
                    BraceRight@27..28 "}"
                Blankspace@28..29 "\n"
                LetDeclaration@29..39
                  Let@29..32 "let"
                  Blankspace@32..33 " "
                  Name@33..34
                    Identifier@33..34 "x"
                  Blankspace@34..35 " "
                  Equal@35..36 "="
                  Blankspace@36..37 " "
                  Literal@37..38
                    IntLiteral@37..38 "3"
                  Semicolon@38..39 ";"
                Blankspace@39..40 "\n"
                BraceRight@40..41 "}"
              Blankspace@41..50 "\n        "

            error at 27..28: invalid syntax, expected one of: '&', '!', 'default', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'"#]],
    );
}

#[test]
fn assert_statement() {
    check_statement(
        "const_assert 2 > 1;",
        expect![[r#"
            SourceFile@0..19
              AssertStatement@0..19
                ConstantAssert@0..12 "const_assert"
                Blankspace@12..13 " "
                InfixExpression@13..18
                  Literal@13..14
                    IntLiteral@13..14 "2"
                  Blankspace@14..15 " "
                  GreaterThan@15..16 ">"
                  Blankspace@16..17 " "
                  Literal@17..18
                    IntLiteral@17..18 "1"
                Semicolon@18..19 ";""#]],
    );
}

#[test]
fn global_assert_statement() {
    check(
        "const_assert 2 > 1;",
        expect![[r#"
            SourceFile@0..19
              AssertStatement@0..19
                ConstantAssert@0..12 "const_assert"
                Blankspace@12..13 " "
                InfixExpression@13..18
                  Literal@13..14
                    IntLiteral@13..14 "2"
                  Blankspace@14..15 " "
                  GreaterThan@15..16 ">"
                  Blankspace@16..17 " "
                  Literal@17..18
                    IntLiteral@17..18 "1"
                Semicolon@18..19 ";""#]],
    );
}

#[test]
fn global_override_statement() {
    check(
        "override foo: u32 = 3;",
        expect![[r#"
            SourceFile@0..22
              OverrideDeclaration@0..22
                Override@0..8 "override"
                Blankspace@8..9 " "
                Name@9..12
                  Identifier@9..12 "foo"
                Colon@12..13 ":"
                Blankspace@13..14 " "
                TypeSpecifier@14..17
                  Path@14..17
                    Identifier@14..17 "u32"
                Blankspace@17..18 " "
                Equal@18..19 "="
                Blankspace@19..20 " "
                Literal@20..21
                  IntLiteral@20..21 "3"
                Semicolon@21..22 ";""#]],
    );
}

#[test]
fn discard_statement() {
    check_statement(
        "discard;",
        expect![[r#"
            SourceFile@0..8
              DiscardStatement@0..8
                Discard@0..7 "discard"
                Semicolon@7..8 ";""#]],
    );
}

#[test]
fn attribute_only_recover() {
    check(
        "@fragment",
        expect![[r#"
            SourceFile@0..9
              FunctionDeclaration@0..9
                FragmentAttribute@0..9
                  AttributeOperator@0..1 "@"
                  Fragment@1..9 "fragment"

            error at 9..9: invalid syntax, expected one of: '@', 'fn', 'let', 'override', 'var'"#]],
    );
}

#[test]
fn expression_in_template() {
    check(
        "const data = array<u32, vec.x>();",
        expect![[r#"
            SourceFile@0..33
              ConstantDeclaration@0..33
                Const@0..5 "const"
                Blankspace@5..6 " "
                Name@6..10
                  Identifier@6..10 "data"
                Blankspace@10..11 " "
                Equal@11..12 "="
                Blankspace@12..13 " "
                FunctionCall@13..32
                  IdentExpression@13..30
                    Path@13..18
                      Identifier@13..18 "array"
                    TemplateList@18..30
                      TemplateStart@18..19 "<"
                      IdentExpression@19..22
                        Path@19..22
                          Identifier@19..22 "u32"
                      Comma@22..23 ","
                      Blankspace@23..24 " "
                      FieldExpression@24..29
                        IdentExpression@24..27
                          Path@24..27
                            Identifier@24..27 "vec"
                        Period@27..28 "."
                        Identifier@28..29 "x"
                      TemplateEnd@29..30 ">"
                  Arguments@30..32
                    ParenthesisLeft@30..31 "("
                    ParenthesisRight@31..32 ")"
                Semicolon@32..33 ";""#]],
    );
}

#[test]
fn fn_recover_missing_comma_between_params() {
    check(
        "fn foo(a: f32 b: f32) {}",
        expect![[r#"
            SourceFile@0..24
              FunctionDeclaration@0..24
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..6
                  Identifier@3..6 "foo"
                FunctionParameters@6..21
                  ParenthesisLeft@6..7 "("
                  Parameter@7..13
                    Name@7..8
                      Identifier@7..8 "a"
                    Colon@8..9 ":"
                    Blankspace@9..10 " "
                    TypeSpecifier@10..13
                      Path@10..13
                        Identifier@10..13 "f32"
                  Blankspace@13..14 " "
                  Parameter@14..20
                    Name@14..15
                      Identifier@14..15 "b"
                    Colon@15..16 ":"
                    Blankspace@16..17 " "
                    TypeSpecifier@17..20
                      Path@17..20
                        Identifier@17..20 "f32"
                  ParenthesisRight@20..21 ")"
                Blankspace@21..22 " "
                CompoundStatement@22..24
                  BraceLeft@22..23 "{"
                  BraceRight@23..24 "}"

            error at 14..15: expected ',' between parameters"#]],
    );
}

#[test]
fn struct_recover_missing_comma_between_members() {
    check(
        "struct Foo { x: f32 y: f32 }",
        expect![[r#"
            SourceFile@0..28
              StructDeclaration@0..28
                Struct@0..6 "struct"
                Blankspace@6..7 " "
                Name@7..10
                  Identifier@7..10 "Foo"
                Blankspace@10..11 " "
                StructBody@11..28
                  BraceLeft@11..12 "{"
                  Blankspace@12..13 " "
                  StructMember@13..19
                    Name@13..14
                      Identifier@13..14 "x"
                    Colon@14..15 ":"
                    Blankspace@15..16 " "
                    TypeSpecifier@16..19
                      Path@16..19
                        Identifier@16..19 "f32"
                  Blankspace@19..20 " "
                  StructMember@20..26
                    Name@20..21
                      Identifier@20..21 "y"
                    Colon@21..22 ":"
                    Blankspace@22..23 " "
                    TypeSpecifier@23..26
                      Path@23..26
                        Identifier@23..26 "f32"
                  Blankspace@26..27 " "
                  BraceRight@27..28 "}"

            error at 20..21: invalid syntax, expected ','"#]],
    );
}

#[test]
fn struct_recover_semicolon_instead_of_comma() {
    check(
        "struct Foo { x: f32; y: f32 }",
        expect![[r#"
            SourceFile@0..29
              StructDeclaration@0..29
                Struct@0..6 "struct"
                Blankspace@6..7 " "
                Name@7..10
                  Identifier@7..10 "Foo"
                Blankspace@10..11 " "
                StructBody@11..29
                  BraceLeft@11..12 "{"
                  Blankspace@12..13 " "
                  StructMember@13..19
                    Name@13..14
                      Identifier@13..14 "x"
                    Colon@14..15 ":"
                    Blankspace@15..16 " "
                    TypeSpecifier@16..19
                      Path@16..19
                        Identifier@16..19 "f32"
                  Semicolon@19..20 ";"
                  Blankspace@20..21 " "
                  StructMember@21..27
                    Name@21..22
                      Identifier@21..22 "y"
                    Colon@22..23 ":"
                    Blankspace@23..24 " "
                    TypeSpecifier@24..27
                      Path@24..27
                        Identifier@24..27 "f32"
                  Blankspace@27..28 " "
                  BraceRight@28..29 "}"

            error at 21..22: invalid syntax, expected ','"#]],
    );
}

#[test]
fn diagnostic_attribute() {
    check(
        "
        @diagnostic(off, bla)
        fn main() {}
        ",
        expect![[r#"
            SourceFile@0..60
              Blankspace@0..9 "\n        "
              FunctionDeclaration@9..51
                DiagnosticAttribute@9..30
                  AttributeOperator@9..10 "@"
                  Diagnostic@10..20 "diagnostic"
                  DiagnosticControl@20..30
                    ParenthesisLeft@20..21 "("
                    SeverityControlName@21..24
                      Identifier@21..24 "off"
                    Comma@24..25 ","
                    Blankspace@25..26 " "
                    DiagnosticRuleName@26..29
                      Identifier@26..29 "bla"
                    ParenthesisRight@29..30 ")"
                Blankspace@30..39 "\n        "
                Fn@39..41 "fn"
                Blankspace@41..42 " "
                Name@42..46
                  Identifier@42..46 "main"
                FunctionParameters@46..48
                  ParenthesisLeft@46..47 "("
                  ParenthesisRight@47..48 ")"
                Blankspace@48..49 " "
                CompoundStatement@49..51
                  BraceLeft@49..50 "{"
                  BraceRight@50..51 "}"
              Blankspace@51..60 "\n        ""#]],
    );
}

#[test]
fn parse_const_attribute() {
    check(
        "
        @const
        fn a() {}
        ",
        expect![[r#"
            SourceFile@0..42
              Blankspace@0..9 "\n        "
              FunctionDeclaration@9..33
                ConstantAttribute@9..15
                  AttributeOperator@9..10 "@"
                  Const@10..15 "const"
                Blankspace@15..24 "\n        "
                Fn@24..26 "fn"
                Blankspace@26..27 " "
                Name@27..28
                  Identifier@27..28 "a"
                FunctionParameters@28..30
                  ParenthesisLeft@28..29 "("
                  ParenthesisRight@29..30 ")"
                Blankspace@30..31 " "
                CompoundStatement@31..33
                  BraceLeft@31..32 "{"
                  BraceRight@32..33 "}"
              Blankspace@33..42 "\n        ""#]],
    );
}

// tests builtin in both positions plus whitespace handling and context handling
#[test]
fn parse_builtin_attribute() {
    check(
        "
        var builtin: i32 = 0;
        fn foo() -> @ builtin(position) vec4<f32> { let builtin = 0; return vec4(0.0, 0.0, 0.0, 0.0); }
        fn bar(@builtin( position ) coord_in: vec4<f32>) { }
        ",
        expect![[r#"
            SourceFile@0..204
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..30
                Var@9..12 "var"
                Blankspace@12..13 " "
                Name@13..20
                  Identifier@13..20 "builtin"
                Colon@20..21 ":"
                Blankspace@21..22 " "
                TypeSpecifier@22..25
                  Path@22..25
                    Identifier@22..25 "i32"
                Blankspace@25..26 " "
                Equal@26..27 "="
                Blankspace@27..28 " "
                Literal@28..29
                  IntLiteral@28..29 "0"
                Semicolon@29..30 ";"
              Blankspace@30..39 "\n        "
              FunctionDeclaration@39..134
                Fn@39..41 "fn"
                Blankspace@41..42 " "
                Name@42..45
                  Identifier@42..45 "foo"
                FunctionParameters@45..47
                  ParenthesisLeft@45..46 "("
                  ParenthesisRight@46..47 ")"
                Blankspace@47..48 " "
                ReturnType@48..80
                  Arrow@48..50 "->"
                  Blankspace@50..51 " "
                  BuiltinAttribute@51..70
                    AttributeOperator@51..52 "@"
                    Blankspace@52..53 " "
                    Builtin@53..60 "builtin"
                    ParenthesisLeft@60..61 "("
                    BuiltinValueName@61..69
                      Identifier@61..69 "position"
                    ParenthesisRight@69..70 ")"
                  Blankspace@70..71 " "
                  TypeSpecifier@71..80
                    Path@71..75
                      Identifier@71..75 "vec4"
                    TemplateList@75..80
                      TemplateStart@75..76 "<"
                      IdentExpression@76..79
                        Path@76..79
                          Identifier@76..79 "f32"
                      TemplateEnd@79..80 ">"
                Blankspace@80..81 " "
                CompoundStatement@81..134
                  BraceLeft@81..82 "{"
                  Blankspace@82..83 " "
                  LetDeclaration@83..99
                    Let@83..86 "let"
                    Blankspace@86..87 " "
                    Name@87..94
                      Identifier@87..94 "builtin"
                    Blankspace@94..95 " "
                    Equal@95..96 "="
                    Blankspace@96..97 " "
                    Literal@97..98
                      IntLiteral@97..98 "0"
                    Semicolon@98..99 ";"
                  Blankspace@99..100 " "
                  ReturnStatement@100..132
                    Return@100..106 "return"
                    Blankspace@106..107 " "
                    FunctionCall@107..131
                      IdentExpression@107..111
                        Path@107..111
                          Identifier@107..111 "vec4"
                      Arguments@111..131
                        ParenthesisLeft@111..112 "("
                        Literal@112..115
                          FloatLiteral@112..115 "0.0"
                        Comma@115..116 ","
                        Blankspace@116..117 " "
                        Literal@117..120
                          FloatLiteral@117..120 "0.0"
                        Comma@120..121 ","
                        Blankspace@121..122 " "
                        Literal@122..125
                          FloatLiteral@122..125 "0.0"
                        Comma@125..126 ","
                        Blankspace@126..127 " "
                        Literal@127..130
                          FloatLiteral@127..130 "0.0"
                        ParenthesisRight@130..131 ")"
                    Semicolon@131..132 ";"
                  Blankspace@132..133 " "
                  BraceRight@133..134 "}"
              Blankspace@134..143 "\n        "
              FunctionDeclaration@143..195
                Fn@143..145 "fn"
                Blankspace@145..146 " "
                Name@146..149
                  Identifier@146..149 "bar"
                FunctionParameters@149..191
                  ParenthesisLeft@149..150 "("
                  Parameter@150..190
                    BuiltinAttribute@150..170
                      AttributeOperator@150..151 "@"
                      Builtin@151..158 "builtin"
                      ParenthesisLeft@158..159 "("
                      Blankspace@159..160 " "
                      BuiltinValueName@160..168
                        Identifier@160..168 "position"
                      Blankspace@168..169 " "
                      ParenthesisRight@169..170 ")"
                    Blankspace@170..171 " "
                    Name@171..179
                      Identifier@171..179 "coord_in"
                    Colon@179..180 ":"
                    Blankspace@180..181 " "
                    TypeSpecifier@181..190
                      Path@181..185
                        Identifier@181..185 "vec4"
                      TemplateList@185..190
                        TemplateStart@185..186 "<"
                        IdentExpression@186..189
                          Path@186..189
                            Identifier@186..189 "f32"
                        TemplateEnd@189..190 ">"
                  ParenthesisRight@190..191 ")"
                Blankspace@191..192 " "
                CompoundStatement@192..195
                  BraceLeft@192..193 "{"
                  Blankspace@193..194 " "
                  BraceRight@194..195 "}"
              Blankspace@195..204 "\n        ""#]],
    );
}

// @interpolate — type only

#[test]
fn parse_interpolate_perspective() {
    check(
        "
        @interpolate(perspective)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..72
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..63
                InterpolateAttribute@9..34
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..33
                    Perspective@22..33 "perspective"
                  ParenthesisRight@33..34 ")"
                Blankspace@34..43 "\n        "
                Var@43..46 "var"
                TemplateList@46..55
                  TemplateStart@46..47 "<"
                  IdentExpression@47..54
                    Path@47..54
                      Identifier@47..54 "private"
                  TemplateEnd@54..55 ">"
                Blankspace@55..56 " "
                Name@56..57
                  Identifier@56..57 "x"
                Colon@57..58 ":"
                Blankspace@58..59 " "
                TypeSpecifier@59..62
                  Path@59..62
                    Identifier@59..62 "f32"
                Semicolon@62..63 ";"
              Blankspace@63..72 "\n        ""#]],
    );
}

#[test]
fn parse_interpolate_linear() {
    check(
        "
        @interpolate(linear)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..67
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..58
                InterpolateAttribute@9..29
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..28
                    Linear@22..28 "linear"
                  ParenthesisRight@28..29 ")"
                Blankspace@29..38 "\n        "
                Var@38..41 "var"
                TemplateList@41..50
                  TemplateStart@41..42 "<"
                  IdentExpression@42..49
                    Path@42..49
                      Identifier@42..49 "private"
                  TemplateEnd@49..50 ">"
                Blankspace@50..51 " "
                Name@51..52
                  Identifier@51..52 "x"
                Colon@52..53 ":"
                Blankspace@53..54 " "
                TypeSpecifier@54..57
                  Path@54..57
                    Identifier@54..57 "f32"
                Semicolon@57..58 ";"
              Blankspace@58..67 "\n        ""#]],
    );
}

#[test]
fn parse_interpolate_flat() {
    check(
        "
        @interpolate(flat)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..65
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..56
                InterpolateAttribute@9..27
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..26
                    Flat@22..26 "flat"
                  ParenthesisRight@26..27 ")"
                Blankspace@27..36 "\n        "
                Var@36..39 "var"
                TemplateList@39..48
                  TemplateStart@39..40 "<"
                  IdentExpression@40..47
                    Path@40..47
                      Identifier@40..47 "private"
                  TemplateEnd@47..48 ">"
                Blankspace@48..49 " "
                Name@49..50
                  Identifier@49..50 "x"
                Colon@50..51 ":"
                Blankspace@51..52 " "
                TypeSpecifier@52..55
                  Path@52..55
                    Identifier@52..55 "f32"
                Semicolon@55..56 ";"
              Blankspace@56..65 "\n        ""#]],
    );
}

// @interpolate(perspective, <sampling>)

#[test]
fn parse_interpolate_perspective_center() {
    check(
        "
        @interpolate(perspective, center)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..80
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..71
                InterpolateAttribute@9..42
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..33
                    Perspective@22..33 "perspective"
                  Comma@33..34 ","
                  Blankspace@34..35 " "
                  InterpolateSamplingName@35..41
                    Center@35..41 "center"
                  ParenthesisRight@41..42 ")"
                Blankspace@42..51 "\n        "
                Var@51..54 "var"
                TemplateList@54..63
                  TemplateStart@54..55 "<"
                  IdentExpression@55..62
                    Path@55..62
                      Identifier@55..62 "private"
                  TemplateEnd@62..63 ">"
                Blankspace@63..64 " "
                Name@64..65
                  Identifier@64..65 "x"
                Colon@65..66 ":"
                Blankspace@66..67 " "
                TypeSpecifier@67..70
                  Path@67..70
                    Identifier@67..70 "f32"
                Semicolon@70..71 ";"
              Blankspace@71..80 "\n        ""#]],
    );
}

#[test]
fn parse_interpolate_perspective_centroid() {
    check(
        "
        @interpolate(perspective, centroid)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..82
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..73
                InterpolateAttribute@9..44
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..33
                    Perspective@22..33 "perspective"
                  Comma@33..34 ","
                  Blankspace@34..35 " "
                  InterpolateSamplingName@35..43
                    Centroid@35..43 "centroid"
                  ParenthesisRight@43..44 ")"
                Blankspace@44..53 "\n        "
                Var@53..56 "var"
                TemplateList@56..65
                  TemplateStart@56..57 "<"
                  IdentExpression@57..64
                    Path@57..64
                      Identifier@57..64 "private"
                  TemplateEnd@64..65 ">"
                Blankspace@65..66 " "
                Name@66..67
                  Identifier@66..67 "x"
                Colon@67..68 ":"
                Blankspace@68..69 " "
                TypeSpecifier@69..72
                  Path@69..72
                    Identifier@69..72 "f32"
                Semicolon@72..73 ";"
              Blankspace@73..82 "\n        ""#]],
    );
}

#[test]
fn parse_interpolate_perspective_sample() {
    check(
        "
        @interpolate(perspective, sample)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..80
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..71
                InterpolateAttribute@9..42
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..33
                    Perspective@22..33 "perspective"
                  Comma@33..34 ","
                  Blankspace@34..35 " "
                  InterpolateSamplingName@35..41
                    Sample@35..41 "sample"
                  ParenthesisRight@41..42 ")"
                Blankspace@42..51 "\n        "
                Var@51..54 "var"
                TemplateList@54..63
                  TemplateStart@54..55 "<"
                  IdentExpression@55..62
                    Path@55..62
                      Identifier@55..62 "private"
                  TemplateEnd@62..63 ">"
                Blankspace@63..64 " "
                Name@64..65
                  Identifier@64..65 "x"
                Colon@65..66 ":"
                Blankspace@66..67 " "
                TypeSpecifier@67..70
                  Path@67..70
                    Identifier@67..70 "f32"
                Semicolon@70..71 ";"
              Blankspace@71..80 "\n        ""#]],
    );
}

// @interpolate(linear, <sampling>)

#[test]
fn parse_interpolate_linear_center() {
    check(
        "
        @interpolate(linear, center)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..75
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..66
                InterpolateAttribute@9..37
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..28
                    Linear@22..28 "linear"
                  Comma@28..29 ","
                  Blankspace@29..30 " "
                  InterpolateSamplingName@30..36
                    Center@30..36 "center"
                  ParenthesisRight@36..37 ")"
                Blankspace@37..46 "\n        "
                Var@46..49 "var"
                TemplateList@49..58
                  TemplateStart@49..50 "<"
                  IdentExpression@50..57
                    Path@50..57
                      Identifier@50..57 "private"
                  TemplateEnd@57..58 ">"
                Blankspace@58..59 " "
                Name@59..60
                  Identifier@59..60 "x"
                Colon@60..61 ":"
                Blankspace@61..62 " "
                TypeSpecifier@62..65
                  Path@62..65
                    Identifier@62..65 "f32"
                Semicolon@65..66 ";"
              Blankspace@66..75 "\n        ""#]],
    );
}

#[test]
fn parse_interpolate_linear_centroid() {
    check(
        "
        @interpolate(linear, centroid)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..77
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..68
                InterpolateAttribute@9..39
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..28
                    Linear@22..28 "linear"
                  Comma@28..29 ","
                  Blankspace@29..30 " "
                  InterpolateSamplingName@30..38
                    Centroid@30..38 "centroid"
                  ParenthesisRight@38..39 ")"
                Blankspace@39..48 "\n        "
                Var@48..51 "var"
                TemplateList@51..60
                  TemplateStart@51..52 "<"
                  IdentExpression@52..59
                    Path@52..59
                      Identifier@52..59 "private"
                  TemplateEnd@59..60 ">"
                Blankspace@60..61 " "
                Name@61..62
                  Identifier@61..62 "x"
                Colon@62..63 ":"
                Blankspace@63..64 " "
                TypeSpecifier@64..67
                  Path@64..67
                    Identifier@64..67 "f32"
                Semicolon@67..68 ";"
              Blankspace@68..77 "\n        ""#]],
    );
}

#[test]
fn parse_interpolate_linear_sample() {
    check(
        "
        @interpolate(linear, sample)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..75
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..66
                InterpolateAttribute@9..37
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..28
                    Linear@22..28 "linear"
                  Comma@28..29 ","
                  Blankspace@29..30 " "
                  InterpolateSamplingName@30..36
                    Sample@30..36 "sample"
                  ParenthesisRight@36..37 ")"
                Blankspace@37..46 "\n        "
                Var@46..49 "var"
                TemplateList@49..58
                  TemplateStart@49..50 "<"
                  IdentExpression@50..57
                    Path@50..57
                      Identifier@50..57 "private"
                  TemplateEnd@57..58 ">"
                Blankspace@58..59 " "
                Name@59..60
                  Identifier@59..60 "x"
                Colon@60..61 ":"
                Blankspace@61..62 " "
                TypeSpecifier@62..65
                  Path@62..65
                    Identifier@62..65 "f32"
                Semicolon@65..66 ";"
              Blankspace@66..75 "\n        ""#]],
    );
}

// @interpolate(flat, <sampling>)

#[test]
fn parse_interpolate_flat_first() {
    check(
        "
        @interpolate(flat, first)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..72
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..63
                InterpolateAttribute@9..34
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..26
                    Flat@22..26 "flat"
                  Comma@26..27 ","
                  Blankspace@27..28 " "
                  InterpolateSamplingName@28..33
                    First@28..33 "first"
                  ParenthesisRight@33..34 ")"
                Blankspace@34..43 "\n        "
                Var@43..46 "var"
                TemplateList@46..55
                  TemplateStart@46..47 "<"
                  IdentExpression@47..54
                    Path@47..54
                      Identifier@47..54 "private"
                  TemplateEnd@54..55 ">"
                Blankspace@55..56 " "
                Name@56..57
                  Identifier@56..57 "x"
                Colon@57..58 ":"
                Blankspace@58..59 " "
                TypeSpecifier@59..62
                  Path@59..62
                    Identifier@59..62 "f32"
                Semicolon@62..63 ";"
              Blankspace@63..72 "\n        ""#]],
    );
}

#[test]
fn parse_interpolate_flat_either() {
    check(
        "
        @interpolate(flat, either)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..73
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..64
                InterpolateAttribute@9..35
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..26
                    Flat@22..26 "flat"
                  Comma@26..27 ","
                  Blankspace@27..28 " "
                  InterpolateSamplingName@28..34
                    Either@28..34 "either"
                  ParenthesisRight@34..35 ")"
                Blankspace@35..44 "\n        "
                Var@44..47 "var"
                TemplateList@47..56
                  TemplateStart@47..48 "<"
                  IdentExpression@48..55
                    Path@48..55
                      Identifier@48..55 "private"
                  TemplateEnd@55..56 ">"
                Blankspace@56..57 " "
                Name@57..58
                  Identifier@57..58 "x"
                Colon@58..59 ":"
                Blankspace@59..60 " "
                TypeSpecifier@60..63
                  Path@60..63
                    Identifier@60..63 "f32"
                Semicolon@63..64 ";"
              Blankspace@64..73 "\n        ""#]],
    );
}

// TODO: should these be parser errors?

// flat only accepts first/either, not center/centroid/sample

#[test]
fn parse_interpolate_flat_center_error() {
    check(
        "
        @interpolate(flat, center)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..73
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..64
                InterpolateAttribute@9..35
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..26
                    Flat@22..26 "flat"
                  Comma@26..27 ","
                  Blankspace@27..28 " "
                  InterpolateSamplingName@28..34
                    Center@28..34 "center"
                  ParenthesisRight@34..35 ")"
                Blankspace@35..44 "\n        "
                Var@44..47 "var"
                TemplateList@47..56
                  TemplateStart@47..48 "<"
                  IdentExpression@48..55
                    Path@48..55
                      Identifier@48..55 "private"
                  TemplateEnd@55..56 ">"
                Blankspace@56..57 " "
                Name@57..58
                  Identifier@57..58 "x"
                Colon@58..59 ":"
                Blankspace@59..60 " "
                TypeSpecifier@60..63
                  Path@60..63
                    Identifier@60..63 "f32"
                Semicolon@63..64 ";"
              Blankspace@64..73 "\n        ""#]],
    );
}

#[test]
fn parse_interpolate_flat_centroid_error() {
    check(
        "
        @interpolate(flat, centroid)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..75
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..66
                InterpolateAttribute@9..37
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..26
                    Flat@22..26 "flat"
                  Comma@26..27 ","
                  Blankspace@27..28 " "
                  InterpolateSamplingName@28..36
                    Centroid@28..36 "centroid"
                  ParenthesisRight@36..37 ")"
                Blankspace@37..46 "\n        "
                Var@46..49 "var"
                TemplateList@49..58
                  TemplateStart@49..50 "<"
                  IdentExpression@50..57
                    Path@50..57
                      Identifier@50..57 "private"
                  TemplateEnd@57..58 ">"
                Blankspace@58..59 " "
                Name@59..60
                  Identifier@59..60 "x"
                Colon@60..61 ":"
                Blankspace@61..62 " "
                TypeSpecifier@62..65
                  Path@62..65
                    Identifier@62..65 "f32"
                Semicolon@65..66 ";"
              Blankspace@66..75 "\n        ""#]],
    );
}

#[test]
fn parse_interpolate_flat_sample_error() {
    check(
        "
        @interpolate(flat, sample)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..73
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..64
                InterpolateAttribute@9..35
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..26
                    Flat@22..26 "flat"
                  Comma@26..27 ","
                  Blankspace@27..28 " "
                  InterpolateSamplingName@28..34
                    Sample@28..34 "sample"
                  ParenthesisRight@34..35 ")"
                Blankspace@35..44 "\n        "
                Var@44..47 "var"
                TemplateList@47..56
                  TemplateStart@47..48 "<"
                  IdentExpression@48..55
                    Path@48..55
                      Identifier@48..55 "private"
                  TemplateEnd@55..56 ">"
                Blankspace@56..57 " "
                Name@57..58
                  Identifier@57..58 "x"
                Colon@58..59 ":"
                Blankspace@59..60 " "
                TypeSpecifier@60..63
                  Path@60..63
                    Identifier@60..63 "f32"
                Semicolon@63..64 ";"
              Blankspace@64..73 "\n        ""#]],
    );
}

// perspective/linear only accept center/centroid/sample, not first/either

#[test]
fn parse_interpolate_perspective_first_error() {
    check(
        "
        @interpolate(perspective, first)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..79
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..70
                InterpolateAttribute@9..41
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..33
                    Perspective@22..33 "perspective"
                  Comma@33..34 ","
                  Blankspace@34..35 " "
                  InterpolateSamplingName@35..40
                    First@35..40 "first"
                  ParenthesisRight@40..41 ")"
                Blankspace@41..50 "\n        "
                Var@50..53 "var"
                TemplateList@53..62
                  TemplateStart@53..54 "<"
                  IdentExpression@54..61
                    Path@54..61
                      Identifier@54..61 "private"
                  TemplateEnd@61..62 ">"
                Blankspace@62..63 " "
                Name@63..64
                  Identifier@63..64 "x"
                Colon@64..65 ":"
                Blankspace@65..66 " "
                TypeSpecifier@66..69
                  Path@66..69
                    Identifier@66..69 "f32"
                Semicolon@69..70 ";"
              Blankspace@70..79 "\n        ""#]],
    );
}

#[test]
fn parse_interpolate_perspective_either_error() {
    check(
        "
        @interpolate(perspective, either)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..80
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..71
                InterpolateAttribute@9..42
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..33
                    Perspective@22..33 "perspective"
                  Comma@33..34 ","
                  Blankspace@34..35 " "
                  InterpolateSamplingName@35..41
                    Either@35..41 "either"
                  ParenthesisRight@41..42 ")"
                Blankspace@42..51 "\n        "
                Var@51..54 "var"
                TemplateList@54..63
                  TemplateStart@54..55 "<"
                  IdentExpression@55..62
                    Path@55..62
                      Identifier@55..62 "private"
                  TemplateEnd@62..63 ">"
                Blankspace@63..64 " "
                Name@64..65
                  Identifier@64..65 "x"
                Colon@65..66 ":"
                Blankspace@66..67 " "
                TypeSpecifier@67..70
                  Path@67..70
                    Identifier@67..70 "f32"
                Semicolon@70..71 ";"
              Blankspace@71..80 "\n        ""#]],
    );
}

#[test]
fn parse_interpolate_linear_first_error() {
    check(
        "
        @interpolate(linear, first)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..74
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..65
                InterpolateAttribute@9..36
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..28
                    Linear@22..28 "linear"
                  Comma@28..29 ","
                  Blankspace@29..30 " "
                  InterpolateSamplingName@30..35
                    First@30..35 "first"
                  ParenthesisRight@35..36 ")"
                Blankspace@36..45 "\n        "
                Var@45..48 "var"
                TemplateList@48..57
                  TemplateStart@48..49 "<"
                  IdentExpression@49..56
                    Path@49..56
                      Identifier@49..56 "private"
                  TemplateEnd@56..57 ">"
                Blankspace@57..58 " "
                Name@58..59
                  Identifier@58..59 "x"
                Colon@59..60 ":"
                Blankspace@60..61 " "
                TypeSpecifier@61..64
                  Path@61..64
                    Identifier@61..64 "f32"
                Semicolon@64..65 ";"
              Blankspace@65..74 "\n        ""#]],
    );
}

#[test]
fn parse_interpolate_linear_either_error() {
    check(
        "
        @interpolate(linear, either)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..75
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..66
                InterpolateAttribute@9..37
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..28
                    Linear@22..28 "linear"
                  Comma@28..29 ","
                  Blankspace@29..30 " "
                  InterpolateSamplingName@30..36
                    Either@30..36 "either"
                  ParenthesisRight@36..37 ")"
                Blankspace@37..46 "\n        "
                Var@46..49 "var"
                TemplateList@49..58
                  TemplateStart@49..50 "<"
                  IdentExpression@50..57
                    Path@50..57
                      Identifier@50..57 "private"
                  TemplateEnd@57..58 ">"
                Blankspace@58..59 " "
                Name@59..60
                  Identifier@59..60 "x"
                Colon@60..61 ":"
                Blankspace@61..62 " "
                TypeSpecifier@62..65
                  Path@62..65
                    Identifier@62..65 "f32"
                Semicolon@65..66 ";"
              Blankspace@66..75 "\n        ""#]],
    );
}

/// Unknown interpolation type.
#[test]
fn parse_interpolate_unknown_type_error() {
    check(
        "
        @interpolate(smooth)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..67
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..58
                InterpolateAttribute@9..22
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                Error@22..29
                  Identifier@22..28 "smooth"
                  ParenthesisRight@28..29 ")"
                Blankspace@29..38 "\n        "
                Var@38..41 "var"
                TemplateList@41..50
                  TemplateStart@41..42 "<"
                  IdentExpression@42..49
                    Path@42..49
                      Identifier@42..49 "private"
                  TemplateEnd@49..50 ">"
                Blankspace@50..51 " "
                Name@51..52
                  Identifier@51..52 "x"
                Colon@52..53 ":"
                Blankspace@53..54 " "
                TypeSpecifier@54..57
                  Path@54..57
                    Identifier@54..57 "f32"
                Semicolon@57..58 ";"
              Blankspace@58..67 "\n        "

            error at 22..28: invalid syntax, expected one of: 'flat', 'linear', 'perspective'"#]],
    );
}

/// Unknown sampling with unknown type.
#[test]
fn parse_interpolate_unknown_type_and_sampling_error() {
    check(
        "
        @interpolate(smooth, fast)
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..73
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..64
                InterpolateAttribute@9..22
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                Error@22..35
                  Identifier@22..28 "smooth"
                  Comma@28..29 ","
                  Blankspace@29..30 " "
                  Identifier@30..34 "fast"
                  ParenthesisRight@34..35 ")"
                Blankspace@35..44 "\n        "
                Var@44..47 "var"
                TemplateList@47..56
                  TemplateStart@47..48 "<"
                  IdentExpression@48..55
                    Path@48..55
                      Identifier@48..55 "private"
                  TemplateEnd@55..56 ">"
                Blankspace@56..57 " "
                Name@57..58
                  Identifier@57..58 "x"
                Colon@58..59 ":"
                Blankspace@59..60 " "
                TypeSpecifier@60..63
                  Path@60..63
                    Identifier@60..63 "f32"
                Semicolon@63..64 ";"
              Blankspace@64..73 "\n        "

            error at 22..28: invalid syntax, expected one of: 'flat', 'linear', 'perspective'"#]],
    );
}

/// Missing type argument entirely.
#[test]
fn parse_interpolate_empty_error() {
    check(
        "
        @interpolate()
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..61
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..52
                InterpolateAttribute@9..23
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  ParenthesisRight@22..23 ")"
                Blankspace@23..32 "\n        "
                Var@32..35 "var"
                TemplateList@35..44
                  TemplateStart@35..36 "<"
                  IdentExpression@36..43
                    Path@36..43
                      Identifier@36..43 "private"
                  TemplateEnd@43..44 ">"
                Blankspace@44..45 " "
                Name@45..46
                  Identifier@45..46 "x"
                Colon@46..47 ":"
                Blankspace@47..48 " "
                TypeSpecifier@48..51
                  Path@48..51
                    Identifier@48..51 "f32"
                Semicolon@51..52 ";"
              Blankspace@52..61 "\n        "

            error at 22..23: invalid syntax, expected one of: 'flat', 'linear', 'perspective'"#]],
    );
}

/// Missing closing parenthesis.
#[test]
fn parse_interpolate_unclosed_error() {
    check(
        "
        @interpolate(perspective
        var<private> x: f32;
        ",
        expect![[r#"
            SourceFile@0..71
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..62
                InterpolateAttribute@9..33
                  AttributeOperator@9..10 "@"
                  Interpolate@10..21 "interpolate"
                  ParenthesisLeft@21..22 "("
                  InterpolateTypeName@22..33
                    Perspective@22..33 "perspective"
                Blankspace@33..42 "\n        "
                Var@42..45 "var"
                TemplateList@45..54
                  TemplateStart@45..46 "<"
                  IdentExpression@46..53
                    Path@46..53
                      Identifier@46..53 "private"
                  TemplateEnd@53..54 ">"
                Blankspace@54..55 " "
                Name@55..56
                  Identifier@55..56 "x"
                Colon@56..57 ":"
                Blankspace@57..58 " "
                TypeSpecifier@58..61
                  Path@58..61
                    Identifier@58..61 "f32"
                Semicolon@61..62 ";"
              Blankspace@62..71 "\n        "

            error at 42..45: invalid syntax, expected one of: 'center', 'centroid', ',', 'either', 'first', ')', 'sample'"#]],
    );
}

/// Tests context handling for no parentheses attribute.
#[test]
fn parse_fragment_shader() {
    check(
        "
        @fragment
        fn fragment() -> vec4<f32> {
            return vec4f(0.0, 0.0, 0.0, 0.0);
        }
        ",
        expect![[r#"
            SourceFile@0..120
              Blankspace@0..9 "\n        "
              FunctionDeclaration@9..111
                FragmentAttribute@9..18
                  AttributeOperator@9..10 "@"
                  Fragment@10..18 "fragment"
                Blankspace@18..27 "\n        "
                Fn@27..29 "fn"
                Blankspace@29..30 " "
                Name@30..38
                  Identifier@30..38 "fragment"
                FunctionParameters@38..40
                  ParenthesisLeft@38..39 "("
                  ParenthesisRight@39..40 ")"
                Blankspace@40..41 " "
                ReturnType@41..53
                  Arrow@41..43 "->"
                  Blankspace@43..44 " "
                  TypeSpecifier@44..53
                    Path@44..48
                      Identifier@44..48 "vec4"
                    TemplateList@48..53
                      TemplateStart@48..49 "<"
                      IdentExpression@49..52
                        Path@49..52
                          Identifier@49..52 "f32"
                      TemplateEnd@52..53 ">"
                Blankspace@53..54 " "
                CompoundStatement@54..111
                  BraceLeft@54..55 "{"
                  Blankspace@55..68 "\n            "
                  ReturnStatement@68..101
                    Return@68..74 "return"
                    Blankspace@74..75 " "
                    FunctionCall@75..100
                      IdentExpression@75..80
                        Path@75..80
                          Identifier@75..80 "vec4f"
                      Arguments@80..100
                        ParenthesisLeft@80..81 "("
                        Literal@81..84
                          FloatLiteral@81..84 "0.0"
                        Comma@84..85 ","
                        Blankspace@85..86 " "
                        Literal@86..89
                          FloatLiteral@86..89 "0.0"
                        Comma@89..90 ","
                        Blankspace@90..91 " "
                        Literal@91..94
                          FloatLiteral@91..94 "0.0"
                        Comma@94..95 ","
                        Blankspace@95..96 " "
                        Literal@96..99
                          FloatLiteral@96..99 "0.0"
                        ParenthesisRight@99..100 ")"
                    Semicolon@100..101 ";"
                  Blankspace@101..110 "\n        "
                  BraceRight@110..111 "}"
              Blankspace@111..120 "\n        ""#]],
    );
}

#[test]
fn parse_all_attributes() {
    check(
        "
        struct S {
            @align(16) @size(16)
            a: vec4<f32>,
        };

        @group(0) @binding(0)
        var<uniform> u: S;

        @id(0)
        override C: i32 = 1;

        @must_use
        fn f(@location(0) @interpolate(linear) x: f32) -> @location(0) f32 {
            return x;
        }

        @vertex
        fn vs(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
            return vec4<f32>(f(f32(i)), 0.0, 0.0, 1.0);
        }

        struct FSOut {
        @location(0) @blend_src(0)
            color: vec4<f32>,
        };

        @fragment
        @invariant
        fn fs() -> FSOut {
            return FSOut(vec4<f32>(1.0));
        }

        @compute
        @workgroup_size(1)
        fn cs() {}
        ",
        expect![[r#"
            SourceFile@0..772
              Blankspace@0..9 "\n        "
              StructDeclaration@9..88
                Struct@9..15 "struct"
                Blankspace@15..16 " "
                Name@16..17
                  Identifier@16..17 "S"
                Blankspace@17..18 " "
                StructBody@18..88
                  BraceLeft@18..19 "{"
                  Blankspace@19..32 "\n            "
                  StructMember@32..77
                    AlignAttribute@32..42
                      AttributeOperator@32..33 "@"
                      Align@33..38 "align"
                      ParenthesisLeft@38..39 "("
                      Literal@39..41
                        IntLiteral@39..41 "16"
                      ParenthesisRight@41..42 ")"
                    Blankspace@42..43 " "
                    SizeAttribute@43..52
                      AttributeOperator@43..44 "@"
                      Size@44..48 "size"
                      ParenthesisLeft@48..49 "("
                      Literal@49..51
                        IntLiteral@49..51 "16"
                      ParenthesisRight@51..52 ")"
                    Blankspace@52..65 "\n            "
                    Name@65..66
                      Identifier@65..66 "a"
                    Colon@66..67 ":"
                    Blankspace@67..68 " "
                    TypeSpecifier@68..77
                      Path@68..72
                        Identifier@68..72 "vec4"
                      TemplateList@72..77
                        TemplateStart@72..73 "<"
                        IdentExpression@73..76
                          Path@73..76
                            Identifier@73..76 "f32"
                        TemplateEnd@76..77 ">"
                  Comma@77..78 ","
                  Blankspace@78..87 "\n        "
                  BraceRight@87..88 "}"
              Semicolon@88..89 ";"
              Blankspace@89..99 "\n\n        "
              VariableDeclaration@99..147
                GroupAttribute@99..108
                  AttributeOperator@99..100 "@"
                  Group@100..105 "group"
                  ParenthesisLeft@105..106 "("
                  Literal@106..107
                    IntLiteral@106..107 "0"
                  ParenthesisRight@107..108 ")"
                Blankspace@108..109 " "
                BindingAttribute@109..120
                  AttributeOperator@109..110 "@"
                  Binding@110..117 "binding"
                  ParenthesisLeft@117..118 "("
                  Literal@118..119
                    IntLiteral@118..119 "0"
                  ParenthesisRight@119..120 ")"
                Blankspace@120..129 "\n        "
                Var@129..132 "var"
                TemplateList@132..141
                  TemplateStart@132..133 "<"
                  IdentExpression@133..140
                    Path@133..140
                      Identifier@133..140 "uniform"
                  TemplateEnd@140..141 ">"
                Blankspace@141..142 " "
                Name@142..143
                  Identifier@142..143 "u"
                Colon@143..144 ":"
                Blankspace@144..145 " "
                TypeSpecifier@145..146
                  Path@145..146
                    Identifier@145..146 "S"
                Semicolon@146..147 ";"
              Blankspace@147..157 "\n\n        "
              OverrideDeclaration@157..192
                IdAttribute@157..163
                  AttributeOperator@157..158 "@"
                  Id@158..160 "id"
                  ParenthesisLeft@160..161 "("
                  Literal@161..162
                    IntLiteral@161..162 "0"
                  ParenthesisRight@162..163 ")"
                Blankspace@163..172 "\n        "
                Override@172..180 "override"
                Blankspace@180..181 " "
                Name@181..182
                  Identifier@181..182 "C"
                Colon@182..183 ":"
                Blankspace@183..184 " "
                TypeSpecifier@184..187
                  Path@184..187
                    Identifier@184..187 "i32"
                Blankspace@187..188 " "
                Equal@188..189 "="
                Blankspace@189..190 " "
                Literal@190..191
                  IntLiteral@190..191 "1"
                Semicolon@191..192 ";"
              Blankspace@192..202 "\n\n        "
              FunctionDeclaration@202..320
                MustUseAttribute@202..211
                  AttributeOperator@202..203 "@"
                  MustUse@203..211 "must_use"
                Blankspace@211..220 "\n        "
                Fn@220..222 "fn"
                Blankspace@222..223 " "
                Name@223..224
                  Identifier@223..224 "f"
                FunctionParameters@224..266
                  ParenthesisLeft@224..225 "("
                  Parameter@225..265
                    LocationAttribute@225..237
                      AttributeOperator@225..226 "@"
                      Location@226..234 "location"
                      ParenthesisLeft@234..235 "("
                      Literal@235..236
                        IntLiteral@235..236 "0"
                      ParenthesisRight@236..237 ")"
                    Blankspace@237..238 " "
                    InterpolateAttribute@238..258
                      AttributeOperator@238..239 "@"
                      Interpolate@239..250 "interpolate"
                      ParenthesisLeft@250..251 "("
                      InterpolateTypeName@251..257
                        Linear@251..257 "linear"
                      ParenthesisRight@257..258 ")"
                    Blankspace@258..259 " "
                    Name@259..260
                      Identifier@259..260 "x"
                    Colon@260..261 ":"
                    Blankspace@261..262 " "
                    TypeSpecifier@262..265
                      Path@262..265
                        Identifier@262..265 "f32"
                  ParenthesisRight@265..266 ")"
                Blankspace@266..267 " "
                ReturnType@267..286
                  Arrow@267..269 "->"
                  Blankspace@269..270 " "
                  LocationAttribute@270..282
                    AttributeOperator@270..271 "@"
                    Location@271..279 "location"
                    ParenthesisLeft@279..280 "("
                    Literal@280..281
                      IntLiteral@280..281 "0"
                    ParenthesisRight@281..282 ")"
                  Blankspace@282..283 " "
                  TypeSpecifier@283..286
                    Path@283..286
                      Identifier@283..286 "f32"
                Blankspace@286..287 " "
                CompoundStatement@287..320
                  BraceLeft@287..288 "{"
                  Blankspace@288..301 "\n            "
                  ReturnStatement@301..310
                    Return@301..307 "return"
                    Blankspace@307..308 " "
                    IdentExpression@308..309
                      Path@308..309
                        Identifier@308..309 "x"
                    Semicolon@309..310 ";"
                  Blankspace@310..319 "\n        "
                  BraceRight@319..320 "}"
              Blankspace@320..330 "\n\n        "
              FunctionDeclaration@330..482
                VertexAttribute@330..337
                  AttributeOperator@330..331 "@"
                  Vertex@331..337 "vertex"
                Blankspace@337..346 "\n        "
                Fn@346..348 "fn"
                Blankspace@348..349 " "
                Name@349..351
                  Identifier@349..351 "vs"
                FunctionParameters@351..382
                  ParenthesisLeft@351..352 "("
                  Parameter@352..381
                    BuiltinAttribute@352..374
                      AttributeOperator@352..353 "@"
                      Builtin@353..360 "builtin"
                      ParenthesisLeft@360..361 "("
                      BuiltinValueName@361..373
                        Identifier@361..373 "vertex_index"
                      ParenthesisRight@373..374 ")"
                    Blankspace@374..375 " "
                    Name@375..376
                      Identifier@375..376 "i"
                    Colon@376..377 ":"
                    Blankspace@377..378 " "
                    TypeSpecifier@378..381
                      Path@378..381
                        Identifier@378..381 "u32"
                  ParenthesisRight@381..382 ")"
                Blankspace@382..383 " "
                ReturnType@383..414
                  Arrow@383..385 "->"
                  Blankspace@385..386 " "
                  BuiltinAttribute@386..404
                    AttributeOperator@386..387 "@"
                    Builtin@387..394 "builtin"
                    ParenthesisLeft@394..395 "("
                    BuiltinValueName@395..403
                      Identifier@395..403 "position"
                    ParenthesisRight@403..404 ")"
                  Blankspace@404..405 " "
                  TypeSpecifier@405..414
                    Path@405..409
                      Identifier@405..409 "vec4"
                    TemplateList@409..414
                      TemplateStart@409..410 "<"
                      IdentExpression@410..413
                        Path@410..413
                          Identifier@410..413 "f32"
                      TemplateEnd@413..414 ">"
                Blankspace@414..415 " "
                CompoundStatement@415..482
                  BraceLeft@415..416 "{"
                  Blankspace@416..429 "\n            "
                  ReturnStatement@429..472
                    Return@429..435 "return"
                    Blankspace@435..436 " "
                    FunctionCall@436..471
                      IdentExpression@436..445
                        Path@436..440
                          Identifier@436..440 "vec4"
                        TemplateList@440..445
                          TemplateStart@440..441 "<"
                          IdentExpression@441..444
                            Path@441..444
                              Identifier@441..444 "f32"
                          TemplateEnd@444..445 ">"
                      Arguments@445..471
                        ParenthesisLeft@445..446 "("
                        FunctionCall@446..455
                          IdentExpression@446..447
                            Path@446..447
                              Identifier@446..447 "f"
                          Arguments@447..455
                            ParenthesisLeft@447..448 "("
                            FunctionCall@448..454
                              IdentExpression@448..451
                                Path@448..451
                                  Identifier@448..451 "f32"
                              Arguments@451..454
                                ParenthesisLeft@451..452 "("
                                IdentExpression@452..453
                                  Path@452..453
                                    Identifier@452..453 "i"
                                ParenthesisRight@453..454 ")"
                            ParenthesisRight@454..455 ")"
                        Comma@455..456 ","
                        Blankspace@456..457 " "
                        Literal@457..460
                          FloatLiteral@457..460 "0.0"
                        Comma@460..461 ","
                        Blankspace@461..462 " "
                        Literal@462..465
                          FloatLiteral@462..465 "0.0"
                        Comma@465..466 ","
                        Blankspace@466..467 " "
                        Literal@467..470
                          FloatLiteral@467..470 "1.0"
                        ParenthesisRight@470..471 ")"
                    Semicolon@471..472 ";"
                  Blankspace@472..481 "\n        "
                  BraceRight@481..482 "}"
              Blankspace@482..492 "\n\n        "
              StructDeclaration@492..581
                Struct@492..498 "struct"
                Blankspace@498..499 " "
                Name@499..504
                  Identifier@499..504 "FSOut"
                Blankspace@504..505 " "
                StructBody@505..581
                  BraceLeft@505..506 "{"
                  Blankspace@506..515 "\n        "
                  StructMember@515..570
                    LocationAttribute@515..527
                      AttributeOperator@515..516 "@"
                      Location@516..524 "location"
                      ParenthesisLeft@524..525 "("
                      Literal@525..526
                        IntLiteral@525..526 "0"
                      ParenthesisRight@526..527 ")"
                    Blankspace@527..528 " "
                    BlendSrcAttribute@528..541
                      AttributeOperator@528..529 "@"
                      BlendSrc@529..538 "blend_src"
                      ParenthesisLeft@538..539 "("
                      Literal@539..540
                        IntLiteral@539..540 "0"
                      ParenthesisRight@540..541 ")"
                    Blankspace@541..554 "\n            "
                    Name@554..559
                      Identifier@554..559 "color"
                    Colon@559..560 ":"
                    Blankspace@560..561 " "
                    TypeSpecifier@561..570
                      Path@561..565
                        Identifier@561..565 "vec4"
                      TemplateList@565..570
                        TemplateStart@565..566 "<"
                        IdentExpression@566..569
                          Path@566..569
                            Identifier@566..569 "f32"
                        TemplateEnd@569..570 ">"
                  Comma@570..571 ","
                  Blankspace@571..580 "\n        "
                  BraceRight@580..581 "}"
              Semicolon@581..582 ";"
              Blankspace@582..592 "\n\n        "
              FunctionDeclaration@592..699
                FragmentAttribute@592..601
                  AttributeOperator@592..593 "@"
                  Fragment@593..601 "fragment"
                Blankspace@601..610 "\n        "
                InvariantAttribute@610..620
                  AttributeOperator@610..611 "@"
                  Invariant@611..620 "invariant"
                Blankspace@620..629 "\n        "
                Fn@629..631 "fn"
                Blankspace@631..632 " "
                Name@632..634
                  Identifier@632..634 "fs"
                FunctionParameters@634..636
                  ParenthesisLeft@634..635 "("
                  ParenthesisRight@635..636 ")"
                Blankspace@636..637 " "
                ReturnType@637..645
                  Arrow@637..639 "->"
                  Blankspace@639..640 " "
                  TypeSpecifier@640..645
                    Path@640..645
                      Identifier@640..645 "FSOut"
                Blankspace@645..646 " "
                CompoundStatement@646..699
                  BraceLeft@646..647 "{"
                  Blankspace@647..660 "\n            "
                  ReturnStatement@660..689
                    Return@660..666 "return"
                    Blankspace@666..667 " "
                    FunctionCall@667..688
                      IdentExpression@667..672
                        Path@667..672
                          Identifier@667..672 "FSOut"
                      Arguments@672..688
                        ParenthesisLeft@672..673 "("
                        FunctionCall@673..687
                          IdentExpression@673..682
                            Path@673..677
                              Identifier@673..677 "vec4"
                            TemplateList@677..682
                              TemplateStart@677..678 "<"
                              IdentExpression@678..681
                                Path@678..681
                                  Identifier@678..681 "f32"
                              TemplateEnd@681..682 ">"
                          Arguments@682..687
                            ParenthesisLeft@682..683 "("
                            Literal@683..686
                              FloatLiteral@683..686 "1.0"
                            ParenthesisRight@686..687 ")"
                        ParenthesisRight@687..688 ")"
                    Semicolon@688..689 ";"
                  Blankspace@689..698 "\n        "
                  BraceRight@698..699 "}"
              Blankspace@699..709 "\n\n        "
              FunctionDeclaration@709..763
                ComputeAttribute@709..717
                  AttributeOperator@709..710 "@"
                  Compute@710..717 "compute"
                Blankspace@717..726 "\n        "
                Attribute@726..744
                  AttributeOperator@726..727 "@"
                  WorkgroupSizeAttribute@727..744
                    WorkgroupSize@727..741 "workgroup_size"
                    ParenthesisLeft@741..742 "("
                    Literal@742..743
                      IntLiteral@742..743 "1"
                    ParenthesisRight@743..744 ")"
                Blankspace@744..753 "\n        "
                Fn@753..755 "fn"
                Blankspace@755..756 " "
                Name@756..758
                  Identifier@756..758 "cs"
                FunctionParameters@758..760
                  ParenthesisLeft@758..759 "("
                  ParenthesisRight@759..760 ")"
                Blankspace@760..761 " "
                CompoundStatement@761..763
                  BraceLeft@761..762 "{"
                  BraceRight@762..763 "}"
              Blankspace@763..772 "\n        ""#]],
    );
}

#[test]
fn reserved_words_do_parse() {
    check(
        "
        var NULL = 0;
        var Self = 0;
        var abstract = 0;
        var active = 0;
        var alignas = 0;
        var alignof = 0;
        // WESL keyword
        // var as = 0;
        var asm = 0;
        var asm_fragment = 0;
        var async = 0;
        var attribute = 0;
        var auto = 0;
        var await = 0;
        var become = 0;
        var cast = 0;
        var catch = 0;
        var class = 0;
        var co_await = 0;
        var co_return = 0;
        var co_yield = 0;
        var coherent = 0;
        var column_major = 0;
        var common = 0;
        var compile = 0;
        var compile_fragment = 0;
        var concept = 0;
        var const_cast = 0;
        var consteval = 0;
        var constexpr = 0;
        var constinit = 0;
        var crate = 0;
        var debugger = 0;
        var decltype = 0;
        var delete = 0;
        var demote = 0;
        var demote_to_helper = 0;
        var do = 0;
        var dynamic_cast = 0;
        var enum = 0;
        var explicit = 0;
        var export = 0;
        var extends = 0;
        var extern = 0;
        var external = 0;
        var fallthrough = 0;
        var filter = 0;
        var final = 0;
        var finally = 0;
        var friend = 0;
        var from = 0;
        var fxgroup = 0;
        var get = 0;
        var goto = 0;
        var groupshared = 0;
        var highp = 0;
        var impl = 0;
        var implements = 0;
        // WESL keyword
        // var import = 0;
        var inline = 0;
        var instanceof = 0;
        var interface = 0;
        var layout = 0;
        var lowp = 0;
        var macro = 0;
        var macro_rules = 0;
        var match = 0;
        var mediump = 0;
        var meta = 0;
        var mod = 0;
        var module = 0;
        var move = 0;
        var mut = 0;
        var mutable = 0;
        var namespace = 0;
        var new = 0;
        var nil = 0;
        var noexcept = 0;
        var noinline = 0;
        var nointerpolation = 0;
        var non_coherent = 0;
        var noncoherent = 0;
        var noperspective = 0;
        var null = 0;
        var nullptr = 0;
        var of = 0;
        var operator = 0;
        // WESL keyword
        // var package = 0;
        var packoffset = 0;
        var partition = 0;
        var pass = 0;
        var patch = 0;
        var pixelfragment = 0;
        var precise = 0;
        var precision = 0;
        var premerge = 0;
        var priv = 0;
        var protected = 0;
        var pub = 0;
        var public = 0;
        var readonly = 0;
        var ref = 0;
        var regardless = 0;
        var register = 0;
        var reinterpret_cast = 0;
        var require = 0;
        var resource = 0;
        var restrict = 0;
        var self = 0;
        var set = 0;
        var shared = 0;
        var sizeof = 0;
        var smooth = 0;
        var snorm = 0;
        var static = 0;
        var static_assert = 0;
        var static_cast = 0;
        var std = 0;
        var subroutine = 0;
        // WESL keyword
        // var super = 0;
        var target = 0;
        var template = 0;
        var this = 0;
        var thread_local = 0;
        var throw = 0;
        var trait = 0;
        var try = 0;
        var type = 0;
        var typedef = 0;
        var typeid = 0;
        var typename = 0;
        var typeof = 0;
        var union = 0;
        var unless = 0;
        var unorm = 0;
        var unsafe = 0;
        var unsized = 0;
        var use = 0;
        var using = 0;
        var varying = 0;
        var virtual = 0;
        var volatile = 0;
        var wgsl = 0;
        var where = 0;
        var with = 0;
        var writeonly = 0;
        var yield = 0;
        ",
        expect![[r#"
            SourceFile@0..3743
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..22
                Var@9..12 "var"
                Blankspace@12..13 " "
                Name@13..17
                  Identifier@13..17 "NULL"
                Blankspace@17..18 " "
                Equal@18..19 "="
                Blankspace@19..20 " "
                Literal@20..21
                  IntLiteral@20..21 "0"
                Semicolon@21..22 ";"
              Blankspace@22..31 "\n        "
              VariableDeclaration@31..44
                Var@31..34 "var"
                Blankspace@34..35 " "
                Name@35..39
                  Identifier@35..39 "Self"
                Blankspace@39..40 " "
                Equal@40..41 "="
                Blankspace@41..42 " "
                Literal@42..43
                  IntLiteral@42..43 "0"
                Semicolon@43..44 ";"
              Blankspace@44..53 "\n        "
              VariableDeclaration@53..70
                Var@53..56 "var"
                Blankspace@56..57 " "
                Name@57..65
                  Identifier@57..65 "abstract"
                Blankspace@65..66 " "
                Equal@66..67 "="
                Blankspace@67..68 " "
                Literal@68..69
                  IntLiteral@68..69 "0"
                Semicolon@69..70 ";"
              Blankspace@70..79 "\n        "
              VariableDeclaration@79..94
                Var@79..82 "var"
                Blankspace@82..83 " "
                Name@83..89
                  Identifier@83..89 "active"
                Blankspace@89..90 " "
                Equal@90..91 "="
                Blankspace@91..92 " "
                Literal@92..93
                  IntLiteral@92..93 "0"
                Semicolon@93..94 ";"
              Blankspace@94..103 "\n        "
              VariableDeclaration@103..119
                Var@103..106 "var"
                Blankspace@106..107 " "
                Name@107..114
                  Identifier@107..114 "alignas"
                Blankspace@114..115 " "
                Equal@115..116 "="
                Blankspace@116..117 " "
                Literal@117..118
                  IntLiteral@117..118 "0"
                Semicolon@118..119 ";"
              Blankspace@119..128 "\n        "
              VariableDeclaration@128..144
                Var@128..131 "var"
                Blankspace@131..132 " "
                Name@132..139
                  Identifier@132..139 "alignof"
                Blankspace@139..140 " "
                Equal@140..141 "="
                Blankspace@141..142 " "
                Literal@142..143
                  IntLiteral@142..143 "0"
                Semicolon@143..144 ";"
              Blankspace@144..153 "\n        "
              LineEndingComment@153..168 "// WESL keyword"
              Blankspace@168..177 "\n        "
              LineEndingComment@177..191 "// var as = 0;"
              Blankspace@191..200 "\n        "
              VariableDeclaration@200..212
                Var@200..203 "var"
                Blankspace@203..204 " "
                Name@204..207
                  Identifier@204..207 "asm"
                Blankspace@207..208 " "
                Equal@208..209 "="
                Blankspace@209..210 " "
                Literal@210..211
                  IntLiteral@210..211 "0"
                Semicolon@211..212 ";"
              Blankspace@212..221 "\n        "
              VariableDeclaration@221..242
                Var@221..224 "var"
                Blankspace@224..225 " "
                Name@225..237
                  Identifier@225..237 "asm_fragment"
                Blankspace@237..238 " "
                Equal@238..239 "="
                Blankspace@239..240 " "
                Literal@240..241
                  IntLiteral@240..241 "0"
                Semicolon@241..242 ";"
              Blankspace@242..251 "\n        "
              VariableDeclaration@251..265
                Var@251..254 "var"
                Blankspace@254..255 " "
                Name@255..260
                  Identifier@255..260 "async"
                Blankspace@260..261 " "
                Equal@261..262 "="
                Blankspace@262..263 " "
                Literal@263..264
                  IntLiteral@263..264 "0"
                Semicolon@264..265 ";"
              Blankspace@265..274 "\n        "
              VariableDeclaration@274..292
                Var@274..277 "var"
                Blankspace@277..278 " "
                Name@278..287
                  Identifier@278..287 "attribute"
                Blankspace@287..288 " "
                Equal@288..289 "="
                Blankspace@289..290 " "
                Literal@290..291
                  IntLiteral@290..291 "0"
                Semicolon@291..292 ";"
              Blankspace@292..301 "\n        "
              VariableDeclaration@301..314
                Var@301..304 "var"
                Blankspace@304..305 " "
                Name@305..309
                  Identifier@305..309 "auto"
                Blankspace@309..310 " "
                Equal@310..311 "="
                Blankspace@311..312 " "
                Literal@312..313
                  IntLiteral@312..313 "0"
                Semicolon@313..314 ";"
              Blankspace@314..323 "\n        "
              VariableDeclaration@323..337
                Var@323..326 "var"
                Blankspace@326..327 " "
                Name@327..332
                  Identifier@327..332 "await"
                Blankspace@332..333 " "
                Equal@333..334 "="
                Blankspace@334..335 " "
                Literal@335..336
                  IntLiteral@335..336 "0"
                Semicolon@336..337 ";"
              Blankspace@337..346 "\n        "
              VariableDeclaration@346..361
                Var@346..349 "var"
                Blankspace@349..350 " "
                Name@350..356
                  Identifier@350..356 "become"
                Blankspace@356..357 " "
                Equal@357..358 "="
                Blankspace@358..359 " "
                Literal@359..360
                  IntLiteral@359..360 "0"
                Semicolon@360..361 ";"
              Blankspace@361..370 "\n        "
              VariableDeclaration@370..383
                Var@370..373 "var"
                Blankspace@373..374 " "
                Name@374..378
                  Identifier@374..378 "cast"
                Blankspace@378..379 " "
                Equal@379..380 "="
                Blankspace@380..381 " "
                Literal@381..382
                  IntLiteral@381..382 "0"
                Semicolon@382..383 ";"
              Blankspace@383..392 "\n        "
              VariableDeclaration@392..406
                Var@392..395 "var"
                Blankspace@395..396 " "
                Name@396..401
                  Identifier@396..401 "catch"
                Blankspace@401..402 " "
                Equal@402..403 "="
                Blankspace@403..404 " "
                Literal@404..405
                  IntLiteral@404..405 "0"
                Semicolon@405..406 ";"
              Blankspace@406..415 "\n        "
              VariableDeclaration@415..429
                Var@415..418 "var"
                Blankspace@418..419 " "
                Name@419..424
                  Identifier@419..424 "class"
                Blankspace@424..425 " "
                Equal@425..426 "="
                Blankspace@426..427 " "
                Literal@427..428
                  IntLiteral@427..428 "0"
                Semicolon@428..429 ";"
              Blankspace@429..438 "\n        "
              VariableDeclaration@438..455
                Var@438..441 "var"
                Blankspace@441..442 " "
                Name@442..450
                  Identifier@442..450 "co_await"
                Blankspace@450..451 " "
                Equal@451..452 "="
                Blankspace@452..453 " "
                Literal@453..454
                  IntLiteral@453..454 "0"
                Semicolon@454..455 ";"
              Blankspace@455..464 "\n        "
              VariableDeclaration@464..482
                Var@464..467 "var"
                Blankspace@467..468 " "
                Name@468..477
                  Identifier@468..477 "co_return"
                Blankspace@477..478 " "
                Equal@478..479 "="
                Blankspace@479..480 " "
                Literal@480..481
                  IntLiteral@480..481 "0"
                Semicolon@481..482 ";"
              Blankspace@482..491 "\n        "
              VariableDeclaration@491..508
                Var@491..494 "var"
                Blankspace@494..495 " "
                Name@495..503
                  Identifier@495..503 "co_yield"
                Blankspace@503..504 " "
                Equal@504..505 "="
                Blankspace@505..506 " "
                Literal@506..507
                  IntLiteral@506..507 "0"
                Semicolon@507..508 ";"
              Blankspace@508..517 "\n        "
              VariableDeclaration@517..534
                Var@517..520 "var"
                Blankspace@520..521 " "
                Name@521..529
                  Identifier@521..529 "coherent"
                Blankspace@529..530 " "
                Equal@530..531 "="
                Blankspace@531..532 " "
                Literal@532..533
                  IntLiteral@532..533 "0"
                Semicolon@533..534 ";"
              Blankspace@534..543 "\n        "
              VariableDeclaration@543..564
                Var@543..546 "var"
                Blankspace@546..547 " "
                Name@547..559
                  Identifier@547..559 "column_major"
                Blankspace@559..560 " "
                Equal@560..561 "="
                Blankspace@561..562 " "
                Literal@562..563
                  IntLiteral@562..563 "0"
                Semicolon@563..564 ";"
              Blankspace@564..573 "\n        "
              VariableDeclaration@573..588
                Var@573..576 "var"
                Blankspace@576..577 " "
                Name@577..583
                  Identifier@577..583 "common"
                Blankspace@583..584 " "
                Equal@584..585 "="
                Blankspace@585..586 " "
                Literal@586..587
                  IntLiteral@586..587 "0"
                Semicolon@587..588 ";"
              Blankspace@588..597 "\n        "
              VariableDeclaration@597..613
                Var@597..600 "var"
                Blankspace@600..601 " "
                Name@601..608
                  Identifier@601..608 "compile"
                Blankspace@608..609 " "
                Equal@609..610 "="
                Blankspace@610..611 " "
                Literal@611..612
                  IntLiteral@611..612 "0"
                Semicolon@612..613 ";"
              Blankspace@613..622 "\n        "
              VariableDeclaration@622..647
                Var@622..625 "var"
                Blankspace@625..626 " "
                Name@626..642
                  Identifier@626..642 "compile_fragment"
                Blankspace@642..643 " "
                Equal@643..644 "="
                Blankspace@644..645 " "
                Literal@645..646
                  IntLiteral@645..646 "0"
                Semicolon@646..647 ";"
              Blankspace@647..656 "\n        "
              VariableDeclaration@656..672
                Var@656..659 "var"
                Blankspace@659..660 " "
                Name@660..667
                  Identifier@660..667 "concept"
                Blankspace@667..668 " "
                Equal@668..669 "="
                Blankspace@669..670 " "
                Literal@670..671
                  IntLiteral@670..671 "0"
                Semicolon@671..672 ";"
              Blankspace@672..681 "\n        "
              VariableDeclaration@681..700
                Var@681..684 "var"
                Blankspace@684..685 " "
                Name@685..695
                  Identifier@685..695 "const_cast"
                Blankspace@695..696 " "
                Equal@696..697 "="
                Blankspace@697..698 " "
                Literal@698..699
                  IntLiteral@698..699 "0"
                Semicolon@699..700 ";"
              Blankspace@700..709 "\n        "
              VariableDeclaration@709..727
                Var@709..712 "var"
                Blankspace@712..713 " "
                Name@713..722
                  Identifier@713..722 "consteval"
                Blankspace@722..723 " "
                Equal@723..724 "="
                Blankspace@724..725 " "
                Literal@725..726
                  IntLiteral@725..726 "0"
                Semicolon@726..727 ";"
              Blankspace@727..736 "\n        "
              VariableDeclaration@736..754
                Var@736..739 "var"
                Blankspace@739..740 " "
                Name@740..749
                  Identifier@740..749 "constexpr"
                Blankspace@749..750 " "
                Equal@750..751 "="
                Blankspace@751..752 " "
                Literal@752..753
                  IntLiteral@752..753 "0"
                Semicolon@753..754 ";"
              Blankspace@754..763 "\n        "
              VariableDeclaration@763..781
                Var@763..766 "var"
                Blankspace@766..767 " "
                Name@767..776
                  Identifier@767..776 "constinit"
                Blankspace@776..777 " "
                Equal@777..778 "="
                Blankspace@778..779 " "
                Literal@779..780
                  IntLiteral@779..780 "0"
                Semicolon@780..781 ";"
              Blankspace@781..790 "\n        "
              VariableDeclaration@790..804
                Var@790..793 "var"
                Blankspace@793..794 " "
                Name@794..799
                  Identifier@794..799 "crate"
                Blankspace@799..800 " "
                Equal@800..801 "="
                Blankspace@801..802 " "
                Literal@802..803
                  IntLiteral@802..803 "0"
                Semicolon@803..804 ";"
              Blankspace@804..813 "\n        "
              VariableDeclaration@813..830
                Var@813..816 "var"
                Blankspace@816..817 " "
                Name@817..825
                  Identifier@817..825 "debugger"
                Blankspace@825..826 " "
                Equal@826..827 "="
                Blankspace@827..828 " "
                Literal@828..829
                  IntLiteral@828..829 "0"
                Semicolon@829..830 ";"
              Blankspace@830..839 "\n        "
              VariableDeclaration@839..856
                Var@839..842 "var"
                Blankspace@842..843 " "
                Name@843..851
                  Identifier@843..851 "decltype"
                Blankspace@851..852 " "
                Equal@852..853 "="
                Blankspace@853..854 " "
                Literal@854..855
                  IntLiteral@854..855 "0"
                Semicolon@855..856 ";"
              Blankspace@856..865 "\n        "
              VariableDeclaration@865..880
                Var@865..868 "var"
                Blankspace@868..869 " "
                Name@869..875
                  Identifier@869..875 "delete"
                Blankspace@875..876 " "
                Equal@876..877 "="
                Blankspace@877..878 " "
                Literal@878..879
                  IntLiteral@878..879 "0"
                Semicolon@879..880 ";"
              Blankspace@880..889 "\n        "
              VariableDeclaration@889..904
                Var@889..892 "var"
                Blankspace@892..893 " "
                Name@893..899
                  Identifier@893..899 "demote"
                Blankspace@899..900 " "
                Equal@900..901 "="
                Blankspace@901..902 " "
                Literal@902..903
                  IntLiteral@902..903 "0"
                Semicolon@903..904 ";"
              Blankspace@904..913 "\n        "
              VariableDeclaration@913..938
                Var@913..916 "var"
                Blankspace@916..917 " "
                Name@917..933
                  Identifier@917..933 "demote_to_helper"
                Blankspace@933..934 " "
                Equal@934..935 "="
                Blankspace@935..936 " "
                Literal@936..937
                  IntLiteral@936..937 "0"
                Semicolon@937..938 ";"
              Blankspace@938..947 "\n        "
              VariableDeclaration@947..958
                Var@947..950 "var"
                Blankspace@950..951 " "
                Name@951..953
                  Identifier@951..953 "do"
                Blankspace@953..954 " "
                Equal@954..955 "="
                Blankspace@955..956 " "
                Literal@956..957
                  IntLiteral@956..957 "0"
                Semicolon@957..958 ";"
              Blankspace@958..967 "\n        "
              VariableDeclaration@967..988
                Var@967..970 "var"
                Blankspace@970..971 " "
                Name@971..983
                  Identifier@971..983 "dynamic_cast"
                Blankspace@983..984 " "
                Equal@984..985 "="
                Blankspace@985..986 " "
                Literal@986..987
                  IntLiteral@986..987 "0"
                Semicolon@987..988 ";"
              Blankspace@988..997 "\n        "
              VariableDeclaration@997..1010
                Var@997..1000 "var"
                Blankspace@1000..1001 " "
                Name@1001..1005
                  Identifier@1001..1005 "enum"
                Blankspace@1005..1006 " "
                Equal@1006..1007 "="
                Blankspace@1007..1008 " "
                Literal@1008..1009
                  IntLiteral@1008..1009 "0"
                Semicolon@1009..1010 ";"
              Blankspace@1010..1019 "\n        "
              VariableDeclaration@1019..1036
                Var@1019..1022 "var"
                Blankspace@1022..1023 " "
                Name@1023..1031
                  Identifier@1023..1031 "explicit"
                Blankspace@1031..1032 " "
                Equal@1032..1033 "="
                Blankspace@1033..1034 " "
                Literal@1034..1035
                  IntLiteral@1034..1035 "0"
                Semicolon@1035..1036 ";"
              Blankspace@1036..1045 "\n        "
              VariableDeclaration@1045..1060
                Var@1045..1048 "var"
                Blankspace@1048..1049 " "
                Name@1049..1055
                  Identifier@1049..1055 "export"
                Blankspace@1055..1056 " "
                Equal@1056..1057 "="
                Blankspace@1057..1058 " "
                Literal@1058..1059
                  IntLiteral@1058..1059 "0"
                Semicolon@1059..1060 ";"
              Blankspace@1060..1069 "\n        "
              VariableDeclaration@1069..1085
                Var@1069..1072 "var"
                Blankspace@1072..1073 " "
                Name@1073..1080
                  Identifier@1073..1080 "extends"
                Blankspace@1080..1081 " "
                Equal@1081..1082 "="
                Blankspace@1082..1083 " "
                Literal@1083..1084
                  IntLiteral@1083..1084 "0"
                Semicolon@1084..1085 ";"
              Blankspace@1085..1094 "\n        "
              VariableDeclaration@1094..1109
                Var@1094..1097 "var"
                Blankspace@1097..1098 " "
                Name@1098..1104
                  Identifier@1098..1104 "extern"
                Blankspace@1104..1105 " "
                Equal@1105..1106 "="
                Blankspace@1106..1107 " "
                Literal@1107..1108
                  IntLiteral@1107..1108 "0"
                Semicolon@1108..1109 ";"
              Blankspace@1109..1118 "\n        "
              VariableDeclaration@1118..1135
                Var@1118..1121 "var"
                Blankspace@1121..1122 " "
                Name@1122..1130
                  Identifier@1122..1130 "external"
                Blankspace@1130..1131 " "
                Equal@1131..1132 "="
                Blankspace@1132..1133 " "
                Literal@1133..1134
                  IntLiteral@1133..1134 "0"
                Semicolon@1134..1135 ";"
              Blankspace@1135..1144 "\n        "
              VariableDeclaration@1144..1164
                Var@1144..1147 "var"
                Blankspace@1147..1148 " "
                Name@1148..1159
                  Identifier@1148..1159 "fallthrough"
                Blankspace@1159..1160 " "
                Equal@1160..1161 "="
                Blankspace@1161..1162 " "
                Literal@1162..1163
                  IntLiteral@1162..1163 "0"
                Semicolon@1163..1164 ";"
              Blankspace@1164..1173 "\n        "
              VariableDeclaration@1173..1188
                Var@1173..1176 "var"
                Blankspace@1176..1177 " "
                Name@1177..1183
                  Identifier@1177..1183 "filter"
                Blankspace@1183..1184 " "
                Equal@1184..1185 "="
                Blankspace@1185..1186 " "
                Literal@1186..1187
                  IntLiteral@1186..1187 "0"
                Semicolon@1187..1188 ";"
              Blankspace@1188..1197 "\n        "
              VariableDeclaration@1197..1211
                Var@1197..1200 "var"
                Blankspace@1200..1201 " "
                Name@1201..1206
                  Identifier@1201..1206 "final"
                Blankspace@1206..1207 " "
                Equal@1207..1208 "="
                Blankspace@1208..1209 " "
                Literal@1209..1210
                  IntLiteral@1209..1210 "0"
                Semicolon@1210..1211 ";"
              Blankspace@1211..1220 "\n        "
              VariableDeclaration@1220..1236
                Var@1220..1223 "var"
                Blankspace@1223..1224 " "
                Name@1224..1231
                  Identifier@1224..1231 "finally"
                Blankspace@1231..1232 " "
                Equal@1232..1233 "="
                Blankspace@1233..1234 " "
                Literal@1234..1235
                  IntLiteral@1234..1235 "0"
                Semicolon@1235..1236 ";"
              Blankspace@1236..1245 "\n        "
              VariableDeclaration@1245..1260
                Var@1245..1248 "var"
                Blankspace@1248..1249 " "
                Name@1249..1255
                  Identifier@1249..1255 "friend"
                Blankspace@1255..1256 " "
                Equal@1256..1257 "="
                Blankspace@1257..1258 " "
                Literal@1258..1259
                  IntLiteral@1258..1259 "0"
                Semicolon@1259..1260 ";"
              Blankspace@1260..1269 "\n        "
              VariableDeclaration@1269..1282
                Var@1269..1272 "var"
                Blankspace@1272..1273 " "
                Name@1273..1277
                  Identifier@1273..1277 "from"
                Blankspace@1277..1278 " "
                Equal@1278..1279 "="
                Blankspace@1279..1280 " "
                Literal@1280..1281
                  IntLiteral@1280..1281 "0"
                Semicolon@1281..1282 ";"
              Blankspace@1282..1291 "\n        "
              VariableDeclaration@1291..1307
                Var@1291..1294 "var"
                Blankspace@1294..1295 " "
                Name@1295..1302
                  Identifier@1295..1302 "fxgroup"
                Blankspace@1302..1303 " "
                Equal@1303..1304 "="
                Blankspace@1304..1305 " "
                Literal@1305..1306
                  IntLiteral@1305..1306 "0"
                Semicolon@1306..1307 ";"
              Blankspace@1307..1316 "\n        "
              VariableDeclaration@1316..1328
                Var@1316..1319 "var"
                Blankspace@1319..1320 " "
                Name@1320..1323
                  Identifier@1320..1323 "get"
                Blankspace@1323..1324 " "
                Equal@1324..1325 "="
                Blankspace@1325..1326 " "
                Literal@1326..1327
                  IntLiteral@1326..1327 "0"
                Semicolon@1327..1328 ";"
              Blankspace@1328..1337 "\n        "
              VariableDeclaration@1337..1350
                Var@1337..1340 "var"
                Blankspace@1340..1341 " "
                Name@1341..1345
                  Identifier@1341..1345 "goto"
                Blankspace@1345..1346 " "
                Equal@1346..1347 "="
                Blankspace@1347..1348 " "
                Literal@1348..1349
                  IntLiteral@1348..1349 "0"
                Semicolon@1349..1350 ";"
              Blankspace@1350..1359 "\n        "
              VariableDeclaration@1359..1379
                Var@1359..1362 "var"
                Blankspace@1362..1363 " "
                Name@1363..1374
                  Identifier@1363..1374 "groupshared"
                Blankspace@1374..1375 " "
                Equal@1375..1376 "="
                Blankspace@1376..1377 " "
                Literal@1377..1378
                  IntLiteral@1377..1378 "0"
                Semicolon@1378..1379 ";"
              Blankspace@1379..1388 "\n        "
              VariableDeclaration@1388..1402
                Var@1388..1391 "var"
                Blankspace@1391..1392 " "
                Name@1392..1397
                  Identifier@1392..1397 "highp"
                Blankspace@1397..1398 " "
                Equal@1398..1399 "="
                Blankspace@1399..1400 " "
                Literal@1400..1401
                  IntLiteral@1400..1401 "0"
                Semicolon@1401..1402 ";"
              Blankspace@1402..1411 "\n        "
              VariableDeclaration@1411..1424
                Var@1411..1414 "var"
                Blankspace@1414..1415 " "
                Name@1415..1419
                  Identifier@1415..1419 "impl"
                Blankspace@1419..1420 " "
                Equal@1420..1421 "="
                Blankspace@1421..1422 " "
                Literal@1422..1423
                  IntLiteral@1422..1423 "0"
                Semicolon@1423..1424 ";"
              Blankspace@1424..1433 "\n        "
              VariableDeclaration@1433..1452
                Var@1433..1436 "var"
                Blankspace@1436..1437 " "
                Name@1437..1447
                  Identifier@1437..1447 "implements"
                Blankspace@1447..1448 " "
                Equal@1448..1449 "="
                Blankspace@1449..1450 " "
                Literal@1450..1451
                  IntLiteral@1450..1451 "0"
                Semicolon@1451..1452 ";"
              Blankspace@1452..1461 "\n        "
              LineEndingComment@1461..1476 "// WESL keyword"
              Blankspace@1476..1485 "\n        "
              LineEndingComment@1485..1503 "// var import = 0;"
              Blankspace@1503..1512 "\n        "
              VariableDeclaration@1512..1527
                Var@1512..1515 "var"
                Blankspace@1515..1516 " "
                Name@1516..1522
                  Identifier@1516..1522 "inline"
                Blankspace@1522..1523 " "
                Equal@1523..1524 "="
                Blankspace@1524..1525 " "
                Literal@1525..1526
                  IntLiteral@1525..1526 "0"
                Semicolon@1526..1527 ";"
              Blankspace@1527..1536 "\n        "
              VariableDeclaration@1536..1555
                Var@1536..1539 "var"
                Blankspace@1539..1540 " "
                Name@1540..1550
                  Identifier@1540..1550 "instanceof"
                Blankspace@1550..1551 " "
                Equal@1551..1552 "="
                Blankspace@1552..1553 " "
                Literal@1553..1554
                  IntLiteral@1553..1554 "0"
                Semicolon@1554..1555 ";"
              Blankspace@1555..1564 "\n        "
              VariableDeclaration@1564..1582
                Var@1564..1567 "var"
                Blankspace@1567..1568 " "
                Name@1568..1577
                  Identifier@1568..1577 "interface"
                Blankspace@1577..1578 " "
                Equal@1578..1579 "="
                Blankspace@1579..1580 " "
                Literal@1580..1581
                  IntLiteral@1580..1581 "0"
                Semicolon@1581..1582 ";"
              Blankspace@1582..1591 "\n        "
              VariableDeclaration@1591..1606
                Var@1591..1594 "var"
                Blankspace@1594..1595 " "
                Name@1595..1601
                  Identifier@1595..1601 "layout"
                Blankspace@1601..1602 " "
                Equal@1602..1603 "="
                Blankspace@1603..1604 " "
                Literal@1604..1605
                  IntLiteral@1604..1605 "0"
                Semicolon@1605..1606 ";"
              Blankspace@1606..1615 "\n        "
              VariableDeclaration@1615..1628
                Var@1615..1618 "var"
                Blankspace@1618..1619 " "
                Name@1619..1623
                  Identifier@1619..1623 "lowp"
                Blankspace@1623..1624 " "
                Equal@1624..1625 "="
                Blankspace@1625..1626 " "
                Literal@1626..1627
                  IntLiteral@1626..1627 "0"
                Semicolon@1627..1628 ";"
              Blankspace@1628..1637 "\n        "
              VariableDeclaration@1637..1651
                Var@1637..1640 "var"
                Blankspace@1640..1641 " "
                Name@1641..1646
                  Identifier@1641..1646 "macro"
                Blankspace@1646..1647 " "
                Equal@1647..1648 "="
                Blankspace@1648..1649 " "
                Literal@1649..1650
                  IntLiteral@1649..1650 "0"
                Semicolon@1650..1651 ";"
              Blankspace@1651..1660 "\n        "
              VariableDeclaration@1660..1680
                Var@1660..1663 "var"
                Blankspace@1663..1664 " "
                Name@1664..1675
                  Identifier@1664..1675 "macro_rules"
                Blankspace@1675..1676 " "
                Equal@1676..1677 "="
                Blankspace@1677..1678 " "
                Literal@1678..1679
                  IntLiteral@1678..1679 "0"
                Semicolon@1679..1680 ";"
              Blankspace@1680..1689 "\n        "
              VariableDeclaration@1689..1703
                Var@1689..1692 "var"
                Blankspace@1692..1693 " "
                Name@1693..1698
                  Identifier@1693..1698 "match"
                Blankspace@1698..1699 " "
                Equal@1699..1700 "="
                Blankspace@1700..1701 " "
                Literal@1701..1702
                  IntLiteral@1701..1702 "0"
                Semicolon@1702..1703 ";"
              Blankspace@1703..1712 "\n        "
              VariableDeclaration@1712..1728
                Var@1712..1715 "var"
                Blankspace@1715..1716 " "
                Name@1716..1723
                  Identifier@1716..1723 "mediump"
                Blankspace@1723..1724 " "
                Equal@1724..1725 "="
                Blankspace@1725..1726 " "
                Literal@1726..1727
                  IntLiteral@1726..1727 "0"
                Semicolon@1727..1728 ";"
              Blankspace@1728..1737 "\n        "
              VariableDeclaration@1737..1750
                Var@1737..1740 "var"
                Blankspace@1740..1741 " "
                Name@1741..1745
                  Identifier@1741..1745 "meta"
                Blankspace@1745..1746 " "
                Equal@1746..1747 "="
                Blankspace@1747..1748 " "
                Literal@1748..1749
                  IntLiteral@1748..1749 "0"
                Semicolon@1749..1750 ";"
              Blankspace@1750..1759 "\n        "
              VariableDeclaration@1759..1771
                Var@1759..1762 "var"
                Blankspace@1762..1763 " "
                Name@1763..1766
                  Identifier@1763..1766 "mod"
                Blankspace@1766..1767 " "
                Equal@1767..1768 "="
                Blankspace@1768..1769 " "
                Literal@1769..1770
                  IntLiteral@1769..1770 "0"
                Semicolon@1770..1771 ";"
              Blankspace@1771..1780 "\n        "
              VariableDeclaration@1780..1795
                Var@1780..1783 "var"
                Blankspace@1783..1784 " "
                Name@1784..1790
                  Identifier@1784..1790 "module"
                Blankspace@1790..1791 " "
                Equal@1791..1792 "="
                Blankspace@1792..1793 " "
                Literal@1793..1794
                  IntLiteral@1793..1794 "0"
                Semicolon@1794..1795 ";"
              Blankspace@1795..1804 "\n        "
              VariableDeclaration@1804..1817
                Var@1804..1807 "var"
                Blankspace@1807..1808 " "
                Name@1808..1812
                  Identifier@1808..1812 "move"
                Blankspace@1812..1813 " "
                Equal@1813..1814 "="
                Blankspace@1814..1815 " "
                Literal@1815..1816
                  IntLiteral@1815..1816 "0"
                Semicolon@1816..1817 ";"
              Blankspace@1817..1826 "\n        "
              VariableDeclaration@1826..1838
                Var@1826..1829 "var"
                Blankspace@1829..1830 " "
                Name@1830..1833
                  Identifier@1830..1833 "mut"
                Blankspace@1833..1834 " "
                Equal@1834..1835 "="
                Blankspace@1835..1836 " "
                Literal@1836..1837
                  IntLiteral@1836..1837 "0"
                Semicolon@1837..1838 ";"
              Blankspace@1838..1847 "\n        "
              VariableDeclaration@1847..1863
                Var@1847..1850 "var"
                Blankspace@1850..1851 " "
                Name@1851..1858
                  Identifier@1851..1858 "mutable"
                Blankspace@1858..1859 " "
                Equal@1859..1860 "="
                Blankspace@1860..1861 " "
                Literal@1861..1862
                  IntLiteral@1861..1862 "0"
                Semicolon@1862..1863 ";"
              Blankspace@1863..1872 "\n        "
              VariableDeclaration@1872..1890
                Var@1872..1875 "var"
                Blankspace@1875..1876 " "
                Name@1876..1885
                  Identifier@1876..1885 "namespace"
                Blankspace@1885..1886 " "
                Equal@1886..1887 "="
                Blankspace@1887..1888 " "
                Literal@1888..1889
                  IntLiteral@1888..1889 "0"
                Semicolon@1889..1890 ";"
              Blankspace@1890..1899 "\n        "
              VariableDeclaration@1899..1911
                Var@1899..1902 "var"
                Blankspace@1902..1903 " "
                Name@1903..1906
                  Identifier@1903..1906 "new"
                Blankspace@1906..1907 " "
                Equal@1907..1908 "="
                Blankspace@1908..1909 " "
                Literal@1909..1910
                  IntLiteral@1909..1910 "0"
                Semicolon@1910..1911 ";"
              Blankspace@1911..1920 "\n        "
              VariableDeclaration@1920..1932
                Var@1920..1923 "var"
                Blankspace@1923..1924 " "
                Name@1924..1927
                  Identifier@1924..1927 "nil"
                Blankspace@1927..1928 " "
                Equal@1928..1929 "="
                Blankspace@1929..1930 " "
                Literal@1930..1931
                  IntLiteral@1930..1931 "0"
                Semicolon@1931..1932 ";"
              Blankspace@1932..1941 "\n        "
              VariableDeclaration@1941..1958
                Var@1941..1944 "var"
                Blankspace@1944..1945 " "
                Name@1945..1953
                  Identifier@1945..1953 "noexcept"
                Blankspace@1953..1954 " "
                Equal@1954..1955 "="
                Blankspace@1955..1956 " "
                Literal@1956..1957
                  IntLiteral@1956..1957 "0"
                Semicolon@1957..1958 ";"
              Blankspace@1958..1967 "\n        "
              VariableDeclaration@1967..1984
                Var@1967..1970 "var"
                Blankspace@1970..1971 " "
                Name@1971..1979
                  Identifier@1971..1979 "noinline"
                Blankspace@1979..1980 " "
                Equal@1980..1981 "="
                Blankspace@1981..1982 " "
                Literal@1982..1983
                  IntLiteral@1982..1983 "0"
                Semicolon@1983..1984 ";"
              Blankspace@1984..1993 "\n        "
              VariableDeclaration@1993..2017
                Var@1993..1996 "var"
                Blankspace@1996..1997 " "
                Name@1997..2012
                  Identifier@1997..2012 "nointerpolation"
                Blankspace@2012..2013 " "
                Equal@2013..2014 "="
                Blankspace@2014..2015 " "
                Literal@2015..2016
                  IntLiteral@2015..2016 "0"
                Semicolon@2016..2017 ";"
              Blankspace@2017..2026 "\n        "
              VariableDeclaration@2026..2047
                Var@2026..2029 "var"
                Blankspace@2029..2030 " "
                Name@2030..2042
                  Identifier@2030..2042 "non_coherent"
                Blankspace@2042..2043 " "
                Equal@2043..2044 "="
                Blankspace@2044..2045 " "
                Literal@2045..2046
                  IntLiteral@2045..2046 "0"
                Semicolon@2046..2047 ";"
              Blankspace@2047..2056 "\n        "
              VariableDeclaration@2056..2076
                Var@2056..2059 "var"
                Blankspace@2059..2060 " "
                Name@2060..2071
                  Identifier@2060..2071 "noncoherent"
                Blankspace@2071..2072 " "
                Equal@2072..2073 "="
                Blankspace@2073..2074 " "
                Literal@2074..2075
                  IntLiteral@2074..2075 "0"
                Semicolon@2075..2076 ";"
              Blankspace@2076..2085 "\n        "
              VariableDeclaration@2085..2107
                Var@2085..2088 "var"
                Blankspace@2088..2089 " "
                Name@2089..2102
                  Identifier@2089..2102 "noperspective"
                Blankspace@2102..2103 " "
                Equal@2103..2104 "="
                Blankspace@2104..2105 " "
                Literal@2105..2106
                  IntLiteral@2105..2106 "0"
                Semicolon@2106..2107 ";"
              Blankspace@2107..2116 "\n        "
              VariableDeclaration@2116..2129
                Var@2116..2119 "var"
                Blankspace@2119..2120 " "
                Name@2120..2124
                  Identifier@2120..2124 "null"
                Blankspace@2124..2125 " "
                Equal@2125..2126 "="
                Blankspace@2126..2127 " "
                Literal@2127..2128
                  IntLiteral@2127..2128 "0"
                Semicolon@2128..2129 ";"
              Blankspace@2129..2138 "\n        "
              VariableDeclaration@2138..2154
                Var@2138..2141 "var"
                Blankspace@2141..2142 " "
                Name@2142..2149
                  Identifier@2142..2149 "nullptr"
                Blankspace@2149..2150 " "
                Equal@2150..2151 "="
                Blankspace@2151..2152 " "
                Literal@2152..2153
                  IntLiteral@2152..2153 "0"
                Semicolon@2153..2154 ";"
              Blankspace@2154..2163 "\n        "
              VariableDeclaration@2163..2174
                Var@2163..2166 "var"
                Blankspace@2166..2167 " "
                Name@2167..2169
                  Identifier@2167..2169 "of"
                Blankspace@2169..2170 " "
                Equal@2170..2171 "="
                Blankspace@2171..2172 " "
                Literal@2172..2173
                  IntLiteral@2172..2173 "0"
                Semicolon@2173..2174 ";"
              Blankspace@2174..2183 "\n        "
              VariableDeclaration@2183..2200
                Var@2183..2186 "var"
                Blankspace@2186..2187 " "
                Name@2187..2195
                  Identifier@2187..2195 "operator"
                Blankspace@2195..2196 " "
                Equal@2196..2197 "="
                Blankspace@2197..2198 " "
                Literal@2198..2199
                  IntLiteral@2198..2199 "0"
                Semicolon@2199..2200 ";"
              Blankspace@2200..2209 "\n        "
              LineEndingComment@2209..2224 "// WESL keyword"
              Blankspace@2224..2233 "\n        "
              LineEndingComment@2233..2252 "// var package = 0;"
              Blankspace@2252..2261 "\n        "
              VariableDeclaration@2261..2280
                Var@2261..2264 "var"
                Blankspace@2264..2265 " "
                Name@2265..2275
                  Identifier@2265..2275 "packoffset"
                Blankspace@2275..2276 " "
                Equal@2276..2277 "="
                Blankspace@2277..2278 " "
                Literal@2278..2279
                  IntLiteral@2278..2279 "0"
                Semicolon@2279..2280 ";"
              Blankspace@2280..2289 "\n        "
              VariableDeclaration@2289..2307
                Var@2289..2292 "var"
                Blankspace@2292..2293 " "
                Name@2293..2302
                  Identifier@2293..2302 "partition"
                Blankspace@2302..2303 " "
                Equal@2303..2304 "="
                Blankspace@2304..2305 " "
                Literal@2305..2306
                  IntLiteral@2305..2306 "0"
                Semicolon@2306..2307 ";"
              Blankspace@2307..2316 "\n        "
              VariableDeclaration@2316..2329
                Var@2316..2319 "var"
                Blankspace@2319..2320 " "
                Name@2320..2324
                  Identifier@2320..2324 "pass"
                Blankspace@2324..2325 " "
                Equal@2325..2326 "="
                Blankspace@2326..2327 " "
                Literal@2327..2328
                  IntLiteral@2327..2328 "0"
                Semicolon@2328..2329 ";"
              Blankspace@2329..2338 "\n        "
              VariableDeclaration@2338..2352
                Var@2338..2341 "var"
                Blankspace@2341..2342 " "
                Name@2342..2347
                  Identifier@2342..2347 "patch"
                Blankspace@2347..2348 " "
                Equal@2348..2349 "="
                Blankspace@2349..2350 " "
                Literal@2350..2351
                  IntLiteral@2350..2351 "0"
                Semicolon@2351..2352 ";"
              Blankspace@2352..2361 "\n        "
              VariableDeclaration@2361..2383
                Var@2361..2364 "var"
                Blankspace@2364..2365 " "
                Name@2365..2378
                  Identifier@2365..2378 "pixelfragment"
                Blankspace@2378..2379 " "
                Equal@2379..2380 "="
                Blankspace@2380..2381 " "
                Literal@2381..2382
                  IntLiteral@2381..2382 "0"
                Semicolon@2382..2383 ";"
              Blankspace@2383..2392 "\n        "
              VariableDeclaration@2392..2408
                Var@2392..2395 "var"
                Blankspace@2395..2396 " "
                Name@2396..2403
                  Identifier@2396..2403 "precise"
                Blankspace@2403..2404 " "
                Equal@2404..2405 "="
                Blankspace@2405..2406 " "
                Literal@2406..2407
                  IntLiteral@2406..2407 "0"
                Semicolon@2407..2408 ";"
              Blankspace@2408..2417 "\n        "
              VariableDeclaration@2417..2435
                Var@2417..2420 "var"
                Blankspace@2420..2421 " "
                Name@2421..2430
                  Identifier@2421..2430 "precision"
                Blankspace@2430..2431 " "
                Equal@2431..2432 "="
                Blankspace@2432..2433 " "
                Literal@2433..2434
                  IntLiteral@2433..2434 "0"
                Semicolon@2434..2435 ";"
              Blankspace@2435..2444 "\n        "
              VariableDeclaration@2444..2461
                Var@2444..2447 "var"
                Blankspace@2447..2448 " "
                Name@2448..2456
                  Identifier@2448..2456 "premerge"
                Blankspace@2456..2457 " "
                Equal@2457..2458 "="
                Blankspace@2458..2459 " "
                Literal@2459..2460
                  IntLiteral@2459..2460 "0"
                Semicolon@2460..2461 ";"
              Blankspace@2461..2470 "\n        "
              VariableDeclaration@2470..2483
                Var@2470..2473 "var"
                Blankspace@2473..2474 " "
                Name@2474..2478
                  Identifier@2474..2478 "priv"
                Blankspace@2478..2479 " "
                Equal@2479..2480 "="
                Blankspace@2480..2481 " "
                Literal@2481..2482
                  IntLiteral@2481..2482 "0"
                Semicolon@2482..2483 ";"
              Blankspace@2483..2492 "\n        "
              VariableDeclaration@2492..2510
                Var@2492..2495 "var"
                Blankspace@2495..2496 " "
                Name@2496..2505
                  Identifier@2496..2505 "protected"
                Blankspace@2505..2506 " "
                Equal@2506..2507 "="
                Blankspace@2507..2508 " "
                Literal@2508..2509
                  IntLiteral@2508..2509 "0"
                Semicolon@2509..2510 ";"
              Blankspace@2510..2519 "\n        "
              VariableDeclaration@2519..2531
                Var@2519..2522 "var"
                Blankspace@2522..2523 " "
                Name@2523..2526
                  Identifier@2523..2526 "pub"
                Blankspace@2526..2527 " "
                Equal@2527..2528 "="
                Blankspace@2528..2529 " "
                Literal@2529..2530
                  IntLiteral@2529..2530 "0"
                Semicolon@2530..2531 ";"
              Blankspace@2531..2540 "\n        "
              VariableDeclaration@2540..2555
                Var@2540..2543 "var"
                Blankspace@2543..2544 " "
                Name@2544..2550
                  Identifier@2544..2550 "public"
                Blankspace@2550..2551 " "
                Equal@2551..2552 "="
                Blankspace@2552..2553 " "
                Literal@2553..2554
                  IntLiteral@2553..2554 "0"
                Semicolon@2554..2555 ";"
              Blankspace@2555..2564 "\n        "
              VariableDeclaration@2564..2581
                Var@2564..2567 "var"
                Blankspace@2567..2568 " "
                Name@2568..2576
                  Identifier@2568..2576 "readonly"
                Blankspace@2576..2577 " "
                Equal@2577..2578 "="
                Blankspace@2578..2579 " "
                Literal@2579..2580
                  IntLiteral@2579..2580 "0"
                Semicolon@2580..2581 ";"
              Blankspace@2581..2590 "\n        "
              VariableDeclaration@2590..2602
                Var@2590..2593 "var"
                Blankspace@2593..2594 " "
                Name@2594..2597
                  Identifier@2594..2597 "ref"
                Blankspace@2597..2598 " "
                Equal@2598..2599 "="
                Blankspace@2599..2600 " "
                Literal@2600..2601
                  IntLiteral@2600..2601 "0"
                Semicolon@2601..2602 ";"
              Blankspace@2602..2611 "\n        "
              VariableDeclaration@2611..2630
                Var@2611..2614 "var"
                Blankspace@2614..2615 " "
                Name@2615..2625
                  Identifier@2615..2625 "regardless"
                Blankspace@2625..2626 " "
                Equal@2626..2627 "="
                Blankspace@2627..2628 " "
                Literal@2628..2629
                  IntLiteral@2628..2629 "0"
                Semicolon@2629..2630 ";"
              Blankspace@2630..2639 "\n        "
              VariableDeclaration@2639..2656
                Var@2639..2642 "var"
                Blankspace@2642..2643 " "
                Name@2643..2651
                  Identifier@2643..2651 "register"
                Blankspace@2651..2652 " "
                Equal@2652..2653 "="
                Blankspace@2653..2654 " "
                Literal@2654..2655
                  IntLiteral@2654..2655 "0"
                Semicolon@2655..2656 ";"
              Blankspace@2656..2665 "\n        "
              VariableDeclaration@2665..2690
                Var@2665..2668 "var"
                Blankspace@2668..2669 " "
                Name@2669..2685
                  Identifier@2669..2685 "reinterpret_cast"
                Blankspace@2685..2686 " "
                Equal@2686..2687 "="
                Blankspace@2687..2688 " "
                Literal@2688..2689
                  IntLiteral@2688..2689 "0"
                Semicolon@2689..2690 ";"
              Blankspace@2690..2699 "\n        "
              VariableDeclaration@2699..2715
                Var@2699..2702 "var"
                Blankspace@2702..2703 " "
                Name@2703..2710
                  Identifier@2703..2710 "require"
                Blankspace@2710..2711 " "
                Equal@2711..2712 "="
                Blankspace@2712..2713 " "
                Literal@2713..2714
                  IntLiteral@2713..2714 "0"
                Semicolon@2714..2715 ";"
              Blankspace@2715..2724 "\n        "
              VariableDeclaration@2724..2741
                Var@2724..2727 "var"
                Blankspace@2727..2728 " "
                Name@2728..2736
                  Identifier@2728..2736 "resource"
                Blankspace@2736..2737 " "
                Equal@2737..2738 "="
                Blankspace@2738..2739 " "
                Literal@2739..2740
                  IntLiteral@2739..2740 "0"
                Semicolon@2740..2741 ";"
              Blankspace@2741..2750 "\n        "
              VariableDeclaration@2750..2767
                Var@2750..2753 "var"
                Blankspace@2753..2754 " "
                Name@2754..2762
                  Identifier@2754..2762 "restrict"
                Blankspace@2762..2763 " "
                Equal@2763..2764 "="
                Blankspace@2764..2765 " "
                Literal@2765..2766
                  IntLiteral@2765..2766 "0"
                Semicolon@2766..2767 ";"
              Blankspace@2767..2776 "\n        "
              VariableDeclaration@2776..2789
                Var@2776..2779 "var"
                Blankspace@2779..2780 " "
                Name@2780..2784
                  Identifier@2780..2784 "self"
                Blankspace@2784..2785 " "
                Equal@2785..2786 "="
                Blankspace@2786..2787 " "
                Literal@2787..2788
                  IntLiteral@2787..2788 "0"
                Semicolon@2788..2789 ";"
              Blankspace@2789..2798 "\n        "
              VariableDeclaration@2798..2810
                Var@2798..2801 "var"
                Blankspace@2801..2802 " "
                Name@2802..2805
                  Identifier@2802..2805 "set"
                Blankspace@2805..2806 " "
                Equal@2806..2807 "="
                Blankspace@2807..2808 " "
                Literal@2808..2809
                  IntLiteral@2808..2809 "0"
                Semicolon@2809..2810 ";"
              Blankspace@2810..2819 "\n        "
              VariableDeclaration@2819..2834
                Var@2819..2822 "var"
                Blankspace@2822..2823 " "
                Name@2823..2829
                  Identifier@2823..2829 "shared"
                Blankspace@2829..2830 " "
                Equal@2830..2831 "="
                Blankspace@2831..2832 " "
                Literal@2832..2833
                  IntLiteral@2832..2833 "0"
                Semicolon@2833..2834 ";"
              Blankspace@2834..2843 "\n        "
              VariableDeclaration@2843..2858
                Var@2843..2846 "var"
                Blankspace@2846..2847 " "
                Name@2847..2853
                  Identifier@2847..2853 "sizeof"
                Blankspace@2853..2854 " "
                Equal@2854..2855 "="
                Blankspace@2855..2856 " "
                Literal@2856..2857
                  IntLiteral@2856..2857 "0"
                Semicolon@2857..2858 ";"
              Blankspace@2858..2867 "\n        "
              VariableDeclaration@2867..2882
                Var@2867..2870 "var"
                Blankspace@2870..2871 " "
                Name@2871..2877
                  Identifier@2871..2877 "smooth"
                Blankspace@2877..2878 " "
                Equal@2878..2879 "="
                Blankspace@2879..2880 " "
                Literal@2880..2881
                  IntLiteral@2880..2881 "0"
                Semicolon@2881..2882 ";"
              Blankspace@2882..2891 "\n        "
              VariableDeclaration@2891..2905
                Var@2891..2894 "var"
                Blankspace@2894..2895 " "
                Name@2895..2900
                  Identifier@2895..2900 "snorm"
                Blankspace@2900..2901 " "
                Equal@2901..2902 "="
                Blankspace@2902..2903 " "
                Literal@2903..2904
                  IntLiteral@2903..2904 "0"
                Semicolon@2904..2905 ";"
              Blankspace@2905..2914 "\n        "
              VariableDeclaration@2914..2929
                Var@2914..2917 "var"
                Blankspace@2917..2918 " "
                Name@2918..2924
                  Identifier@2918..2924 "static"
                Blankspace@2924..2925 " "
                Equal@2925..2926 "="
                Blankspace@2926..2927 " "
                Literal@2927..2928
                  IntLiteral@2927..2928 "0"
                Semicolon@2928..2929 ";"
              Blankspace@2929..2938 "\n        "
              VariableDeclaration@2938..2960
                Var@2938..2941 "var"
                Blankspace@2941..2942 " "
                Name@2942..2955
                  Identifier@2942..2955 "static_assert"
                Blankspace@2955..2956 " "
                Equal@2956..2957 "="
                Blankspace@2957..2958 " "
                Literal@2958..2959
                  IntLiteral@2958..2959 "0"
                Semicolon@2959..2960 ";"
              Blankspace@2960..2969 "\n        "
              VariableDeclaration@2969..2989
                Var@2969..2972 "var"
                Blankspace@2972..2973 " "
                Name@2973..2984
                  Identifier@2973..2984 "static_cast"
                Blankspace@2984..2985 " "
                Equal@2985..2986 "="
                Blankspace@2986..2987 " "
                Literal@2987..2988
                  IntLiteral@2987..2988 "0"
                Semicolon@2988..2989 ";"
              Blankspace@2989..2998 "\n        "
              VariableDeclaration@2998..3010
                Var@2998..3001 "var"
                Blankspace@3001..3002 " "
                Name@3002..3005
                  Identifier@3002..3005 "std"
                Blankspace@3005..3006 " "
                Equal@3006..3007 "="
                Blankspace@3007..3008 " "
                Literal@3008..3009
                  IntLiteral@3008..3009 "0"
                Semicolon@3009..3010 ";"
              Blankspace@3010..3019 "\n        "
              VariableDeclaration@3019..3038
                Var@3019..3022 "var"
                Blankspace@3022..3023 " "
                Name@3023..3033
                  Identifier@3023..3033 "subroutine"
                Blankspace@3033..3034 " "
                Equal@3034..3035 "="
                Blankspace@3035..3036 " "
                Literal@3036..3037
                  IntLiteral@3036..3037 "0"
                Semicolon@3037..3038 ";"
              Blankspace@3038..3047 "\n        "
              LineEndingComment@3047..3062 "// WESL keyword"
              Blankspace@3062..3071 "\n        "
              LineEndingComment@3071..3088 "// var super = 0;"
              Blankspace@3088..3097 "\n        "
              VariableDeclaration@3097..3112
                Var@3097..3100 "var"
                Blankspace@3100..3101 " "
                Name@3101..3107
                  Identifier@3101..3107 "target"
                Blankspace@3107..3108 " "
                Equal@3108..3109 "="
                Blankspace@3109..3110 " "
                Literal@3110..3111
                  IntLiteral@3110..3111 "0"
                Semicolon@3111..3112 ";"
              Blankspace@3112..3121 "\n        "
              VariableDeclaration@3121..3138
                Var@3121..3124 "var"
                Blankspace@3124..3125 " "
                Name@3125..3133
                  Identifier@3125..3133 "template"
                Blankspace@3133..3134 " "
                Equal@3134..3135 "="
                Blankspace@3135..3136 " "
                Literal@3136..3137
                  IntLiteral@3136..3137 "0"
                Semicolon@3137..3138 ";"
              Blankspace@3138..3147 "\n        "
              VariableDeclaration@3147..3160
                Var@3147..3150 "var"
                Blankspace@3150..3151 " "
                Name@3151..3155
                  Identifier@3151..3155 "this"
                Blankspace@3155..3156 " "
                Equal@3156..3157 "="
                Blankspace@3157..3158 " "
                Literal@3158..3159
                  IntLiteral@3158..3159 "0"
                Semicolon@3159..3160 ";"
              Blankspace@3160..3169 "\n        "
              VariableDeclaration@3169..3190
                Var@3169..3172 "var"
                Blankspace@3172..3173 " "
                Name@3173..3185
                  Identifier@3173..3185 "thread_local"
                Blankspace@3185..3186 " "
                Equal@3186..3187 "="
                Blankspace@3187..3188 " "
                Literal@3188..3189
                  IntLiteral@3188..3189 "0"
                Semicolon@3189..3190 ";"
              Blankspace@3190..3199 "\n        "
              VariableDeclaration@3199..3213
                Var@3199..3202 "var"
                Blankspace@3202..3203 " "
                Name@3203..3208
                  Identifier@3203..3208 "throw"
                Blankspace@3208..3209 " "
                Equal@3209..3210 "="
                Blankspace@3210..3211 " "
                Literal@3211..3212
                  IntLiteral@3211..3212 "0"
                Semicolon@3212..3213 ";"
              Blankspace@3213..3222 "\n        "
              VariableDeclaration@3222..3236
                Var@3222..3225 "var"
                Blankspace@3225..3226 " "
                Name@3226..3231
                  Identifier@3226..3231 "trait"
                Blankspace@3231..3232 " "
                Equal@3232..3233 "="
                Blankspace@3233..3234 " "
                Literal@3234..3235
                  IntLiteral@3234..3235 "0"
                Semicolon@3235..3236 ";"
              Blankspace@3236..3245 "\n        "
              VariableDeclaration@3245..3257
                Var@3245..3248 "var"
                Blankspace@3248..3249 " "
                Name@3249..3252
                  Identifier@3249..3252 "try"
                Blankspace@3252..3253 " "
                Equal@3253..3254 "="
                Blankspace@3254..3255 " "
                Literal@3255..3256
                  IntLiteral@3255..3256 "0"
                Semicolon@3256..3257 ";"
              Blankspace@3257..3266 "\n        "
              VariableDeclaration@3266..3279
                Var@3266..3269 "var"
                Blankspace@3269..3270 " "
                Name@3270..3274
                  Identifier@3270..3274 "type"
                Blankspace@3274..3275 " "
                Equal@3275..3276 "="
                Blankspace@3276..3277 " "
                Literal@3277..3278
                  IntLiteral@3277..3278 "0"
                Semicolon@3278..3279 ";"
              Blankspace@3279..3288 "\n        "
              VariableDeclaration@3288..3304
                Var@3288..3291 "var"
                Blankspace@3291..3292 " "
                Name@3292..3299
                  Identifier@3292..3299 "typedef"
                Blankspace@3299..3300 " "
                Equal@3300..3301 "="
                Blankspace@3301..3302 " "
                Literal@3302..3303
                  IntLiteral@3302..3303 "0"
                Semicolon@3303..3304 ";"
              Blankspace@3304..3313 "\n        "
              VariableDeclaration@3313..3328
                Var@3313..3316 "var"
                Blankspace@3316..3317 " "
                Name@3317..3323
                  Identifier@3317..3323 "typeid"
                Blankspace@3323..3324 " "
                Equal@3324..3325 "="
                Blankspace@3325..3326 " "
                Literal@3326..3327
                  IntLiteral@3326..3327 "0"
                Semicolon@3327..3328 ";"
              Blankspace@3328..3337 "\n        "
              VariableDeclaration@3337..3354
                Var@3337..3340 "var"
                Blankspace@3340..3341 " "
                Name@3341..3349
                  Identifier@3341..3349 "typename"
                Blankspace@3349..3350 " "
                Equal@3350..3351 "="
                Blankspace@3351..3352 " "
                Literal@3352..3353
                  IntLiteral@3352..3353 "0"
                Semicolon@3353..3354 ";"
              Blankspace@3354..3363 "\n        "
              VariableDeclaration@3363..3378
                Var@3363..3366 "var"
                Blankspace@3366..3367 " "
                Name@3367..3373
                  Identifier@3367..3373 "typeof"
                Blankspace@3373..3374 " "
                Equal@3374..3375 "="
                Blankspace@3375..3376 " "
                Literal@3376..3377
                  IntLiteral@3376..3377 "0"
                Semicolon@3377..3378 ";"
              Blankspace@3378..3387 "\n        "
              VariableDeclaration@3387..3401
                Var@3387..3390 "var"
                Blankspace@3390..3391 " "
                Name@3391..3396
                  Identifier@3391..3396 "union"
                Blankspace@3396..3397 " "
                Equal@3397..3398 "="
                Blankspace@3398..3399 " "
                Literal@3399..3400
                  IntLiteral@3399..3400 "0"
                Semicolon@3400..3401 ";"
              Blankspace@3401..3410 "\n        "
              VariableDeclaration@3410..3425
                Var@3410..3413 "var"
                Blankspace@3413..3414 " "
                Name@3414..3420
                  Identifier@3414..3420 "unless"
                Blankspace@3420..3421 " "
                Equal@3421..3422 "="
                Blankspace@3422..3423 " "
                Literal@3423..3424
                  IntLiteral@3423..3424 "0"
                Semicolon@3424..3425 ";"
              Blankspace@3425..3434 "\n        "
              VariableDeclaration@3434..3448
                Var@3434..3437 "var"
                Blankspace@3437..3438 " "
                Name@3438..3443
                  Identifier@3438..3443 "unorm"
                Blankspace@3443..3444 " "
                Equal@3444..3445 "="
                Blankspace@3445..3446 " "
                Literal@3446..3447
                  IntLiteral@3446..3447 "0"
                Semicolon@3447..3448 ";"
              Blankspace@3448..3457 "\n        "
              VariableDeclaration@3457..3472
                Var@3457..3460 "var"
                Blankspace@3460..3461 " "
                Name@3461..3467
                  Identifier@3461..3467 "unsafe"
                Blankspace@3467..3468 " "
                Equal@3468..3469 "="
                Blankspace@3469..3470 " "
                Literal@3470..3471
                  IntLiteral@3470..3471 "0"
                Semicolon@3471..3472 ";"
              Blankspace@3472..3481 "\n        "
              VariableDeclaration@3481..3497
                Var@3481..3484 "var"
                Blankspace@3484..3485 " "
                Name@3485..3492
                  Identifier@3485..3492 "unsized"
                Blankspace@3492..3493 " "
                Equal@3493..3494 "="
                Blankspace@3494..3495 " "
                Literal@3495..3496
                  IntLiteral@3495..3496 "0"
                Semicolon@3496..3497 ";"
              Blankspace@3497..3506 "\n        "
              VariableDeclaration@3506..3518
                Var@3506..3509 "var"
                Blankspace@3509..3510 " "
                Name@3510..3513
                  Identifier@3510..3513 "use"
                Blankspace@3513..3514 " "
                Equal@3514..3515 "="
                Blankspace@3515..3516 " "
                Literal@3516..3517
                  IntLiteral@3516..3517 "0"
                Semicolon@3517..3518 ";"
              Blankspace@3518..3527 "\n        "
              VariableDeclaration@3527..3541
                Var@3527..3530 "var"
                Blankspace@3530..3531 " "
                Name@3531..3536
                  Identifier@3531..3536 "using"
                Blankspace@3536..3537 " "
                Equal@3537..3538 "="
                Blankspace@3538..3539 " "
                Literal@3539..3540
                  IntLiteral@3539..3540 "0"
                Semicolon@3540..3541 ";"
              Blankspace@3541..3550 "\n        "
              VariableDeclaration@3550..3566
                Var@3550..3553 "var"
                Blankspace@3553..3554 " "
                Name@3554..3561
                  Identifier@3554..3561 "varying"
                Blankspace@3561..3562 " "
                Equal@3562..3563 "="
                Blankspace@3563..3564 " "
                Literal@3564..3565
                  IntLiteral@3564..3565 "0"
                Semicolon@3565..3566 ";"
              Blankspace@3566..3575 "\n        "
              VariableDeclaration@3575..3591
                Var@3575..3578 "var"
                Blankspace@3578..3579 " "
                Name@3579..3586
                  Identifier@3579..3586 "virtual"
                Blankspace@3586..3587 " "
                Equal@3587..3588 "="
                Blankspace@3588..3589 " "
                Literal@3589..3590
                  IntLiteral@3589..3590 "0"
                Semicolon@3590..3591 ";"
              Blankspace@3591..3600 "\n        "
              VariableDeclaration@3600..3617
                Var@3600..3603 "var"
                Blankspace@3603..3604 " "
                Name@3604..3612
                  Identifier@3604..3612 "volatile"
                Blankspace@3612..3613 " "
                Equal@3613..3614 "="
                Blankspace@3614..3615 " "
                Literal@3615..3616
                  IntLiteral@3615..3616 "0"
                Semicolon@3616..3617 ";"
              Blankspace@3617..3626 "\n        "
              VariableDeclaration@3626..3639
                Var@3626..3629 "var"
                Blankspace@3629..3630 " "
                Name@3630..3634
                  Identifier@3630..3634 "wgsl"
                Blankspace@3634..3635 " "
                Equal@3635..3636 "="
                Blankspace@3636..3637 " "
                Literal@3637..3638
                  IntLiteral@3637..3638 "0"
                Semicolon@3638..3639 ";"
              Blankspace@3639..3648 "\n        "
              VariableDeclaration@3648..3662
                Var@3648..3651 "var"
                Blankspace@3651..3652 " "
                Name@3652..3657
                  Identifier@3652..3657 "where"
                Blankspace@3657..3658 " "
                Equal@3658..3659 "="
                Blankspace@3659..3660 " "
                Literal@3660..3661
                  IntLiteral@3660..3661 "0"
                Semicolon@3661..3662 ";"
              Blankspace@3662..3671 "\n        "
              VariableDeclaration@3671..3684
                Var@3671..3674 "var"
                Blankspace@3674..3675 " "
                Name@3675..3679
                  Identifier@3675..3679 "with"
                Blankspace@3679..3680 " "
                Equal@3680..3681 "="
                Blankspace@3681..3682 " "
                Literal@3682..3683
                  IntLiteral@3682..3683 "0"
                Semicolon@3683..3684 ";"
              Blankspace@3684..3693 "\n        "
              VariableDeclaration@3693..3711
                Var@3693..3696 "var"
                Blankspace@3696..3697 " "
                Name@3697..3706
                  Identifier@3697..3706 "writeonly"
                Blankspace@3706..3707 " "
                Equal@3707..3708 "="
                Blankspace@3708..3709 " "
                Literal@3709..3710
                  IntLiteral@3709..3710 "0"
                Semicolon@3710..3711 ";"
              Blankspace@3711..3720 "\n        "
              VariableDeclaration@3720..3734
                Var@3720..3723 "var"
                Blankspace@3723..3724 " "
                Name@3724..3729
                  Identifier@3724..3729 "yield"
                Blankspace@3729..3730 " "
                Equal@3730..3731 "="
                Blankspace@3731..3732 " "
                Literal@3732..3733
                  IntLiteral@3732..3733 "0"
                Semicolon@3733..3734 ";"
              Blankspace@3734..3743 "\n        ""#]],
    );
}

#[test]
fn keywords_do_not_parse() {
    check(
        "
        var alias = 0;
        var break = 0;
        var case = 0;
        var const = 0;
        var const_assert = 0;
        var continue = 0;
        var continuing = 0;
        var default = 0;
        var diagnostic = 0;
        var discard = 0;
        var else = 0;
        var enable = 0;
        var false = 0;
        var fn = 0;
        var for = 0;
        var if = 0;
        var let = 0;
        var loop = 0;
        var override = 0;
        var requires = 0;
        var return = 0;
        var struct = 0;
        var switch = 0;
        var true = 0;
        var var = 0;
        var while = 0;
        ",
        expect![[r#"
            SourceFile@0..625
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..13
                Var@9..12 "var"
                Blankspace@12..13 " "
              TypeAliasDeclaration@13..23
                Alias@13..18 "alias"
                Blankspace@18..19 " "
                Equal@19..20 "="
                Blankspace@20..21 " "
                TypeSpecifier@21..22
                  Path@21..22
                    Error@21..22
                      IntLiteral@21..22 "0"
                Semicolon@22..23 ";"
              Blankspace@23..32 "\n        "
              VariableDeclaration@32..46
                Var@32..35 "var"
                Blankspace@35..36 " "
                Error@36..41
                  Break@36..41 "break"
                Blankspace@41..42 " "
                Equal@42..43 "="
                Blankspace@43..44 " "
                Literal@44..45
                  IntLiteral@44..45 "0"
                Semicolon@45..46 ";"
              Blankspace@46..55 "\n        "
              VariableDeclaration@55..68
                Var@55..58 "var"
                Blankspace@58..59 " "
                Error@59..63
                  Case@59..63 "case"
                Blankspace@63..64 " "
                Equal@64..65 "="
                Blankspace@65..66 " "
                Literal@66..67
                  IntLiteral@66..67 "0"
                Semicolon@67..68 ";"
              Blankspace@68..77 "\n        "
              VariableDeclaration@77..81
                Var@77..80 "var"
                Blankspace@80..81 " "
              ConstantDeclaration@81..91
                Const@81..86 "const"
                Blankspace@86..87 " "
                Equal@87..88 "="
                Blankspace@88..89 " "
                Literal@89..90
                  IntLiteral@89..90 "0"
                Semicolon@90..91 ";"
              Blankspace@91..100 "\n        "
              VariableDeclaration@100..104
                Var@100..103 "var"
                Blankspace@103..104 " "
              AssertStatement@104..116
                ConstantAssert@104..116 "const_assert"
              Blankspace@116..117 " "
              Error@117..120
                Equal@117..118 "="
                Blankspace@118..119 " "
                IntLiteral@119..120 "0"
              Semicolon@120..121 ";"
              Blankspace@121..130 "\n        "
              VariableDeclaration@130..147
                Var@130..133 "var"
                Blankspace@133..134 " "
                Error@134..142
                  Continue@134..142 "continue"
                Blankspace@142..143 " "
                Equal@143..144 "="
                Blankspace@144..145 " "
                Literal@145..146
                  IntLiteral@145..146 "0"
                Semicolon@146..147 ";"
              Blankspace@147..156 "\n        "
              VariableDeclaration@156..175
                Var@156..159 "var"
                Blankspace@159..160 " "
                Error@160..170
                  Continuing@160..170 "continuing"
                Blankspace@170..171 " "
                Equal@171..172 "="
                Blankspace@172..173 " "
                Literal@173..174
                  IntLiteral@173..174 "0"
                Semicolon@174..175 ";"
              Blankspace@175..184 "\n        "
              VariableDeclaration@184..200
                Var@184..187 "var"
                Blankspace@187..188 " "
                Error@188..195
                  Default@188..195 "default"
                Blankspace@195..196 " "
                Equal@196..197 "="
                Blankspace@197..198 " "
                Literal@198..199
                  IntLiteral@198..199 "0"
                Semicolon@199..200 ";"
              Blankspace@200..209 "\n        "
              VariableDeclaration@209..213
                Var@209..212 "var"
                Blankspace@212..213 " "
              DiagnosticDirective@213..228
                Diagnostic@213..223 "diagnostic"
                Blankspace@223..224 " "
                DiagnosticControl@224..227
                  DiagnosticRuleName@224..227
                    Error@224..227
                      Equal@224..225 "="
                      Blankspace@225..226 " "
                      IntLiteral@226..227 "0"
                Semicolon@227..228 ";"
              Blankspace@228..237 "\n        "
              VariableDeclaration@237..253
                Var@237..240 "var"
                Blankspace@240..241 " "
                Error@241..248
                  Discard@241..248 "discard"
                Blankspace@248..249 " "
                Equal@249..250 "="
                Blankspace@250..251 " "
                Literal@251..252
                  IntLiteral@251..252 "0"
                Semicolon@252..253 ";"
              Blankspace@253..262 "\n        "
              VariableDeclaration@262..275
                Var@262..265 "var"
                Blankspace@265..266 " "
                Error@266..270
                  Else@266..270 "else"
                Blankspace@270..271 " "
                Equal@271..272 "="
                Blankspace@272..273 " "
                Literal@273..274
                  IntLiteral@273..274 "0"
                Semicolon@274..275 ";"
              Blankspace@275..284 "\n        "
              VariableDeclaration@284..288
                Var@284..287 "var"
                Blankspace@287..288 " "
              EnableDirective@288..299
                Enable@288..294 "enable"
                Blankspace@294..295 " "
                Error@295..298
                  Equal@295..296 "="
                  Blankspace@296..297 " "
                  IntLiteral@297..298 "0"
                Semicolon@298..299 ";"
              Blankspace@299..308 "\n        "
              VariableDeclaration@308..322
                Var@308..311 "var"
                Blankspace@311..312 " "
                Error@312..317
                  False@312..317 "false"
                Blankspace@317..318 " "
                Equal@318..319 "="
                Blankspace@319..320 " "
                Literal@320..321
                  IntLiteral@320..321 "0"
                Semicolon@321..322 ";"
              Blankspace@322..331 "\n        "
              VariableDeclaration@331..335
                Var@331..334 "var"
                Blankspace@334..335 " "
              FunctionDeclaration@335..341
                Fn@335..337 "fn"
                Blankspace@337..338 " "
                FunctionParameters@338..341
                  Error@338..341
                    Equal@338..339 "="
                    Blankspace@339..340 " "
                    IntLiteral@340..341 "0"
              Semicolon@341..342 ";"
              Blankspace@342..351 "\n        "
              VariableDeclaration@351..363
                Var@351..354 "var"
                Blankspace@354..355 " "
                Error@355..358
                  For@355..358 "for"
                Blankspace@358..359 " "
                Equal@359..360 "="
                Blankspace@360..361 " "
                Literal@361..362
                  IntLiteral@361..362 "0"
                Semicolon@362..363 ";"
              Blankspace@363..372 "\n        "
              VariableDeclaration@372..383
                Var@372..375 "var"
                Blankspace@375..376 " "
                Error@376..378
                  If@376..378 "if"
                Blankspace@378..379 " "
                Equal@379..380 "="
                Blankspace@380..381 " "
                Literal@381..382
                  IntLiteral@381..382 "0"
                Semicolon@382..383 ";"
              Blankspace@383..392 "\n        "
              VariableDeclaration@392..396
                Var@392..395 "var"
                Blankspace@395..396 " "
              Error@396..404
                Let@396..399 "let"
                Blankspace@399..400 " "
                Equal@400..401 "="
                Blankspace@401..402 " "
                Literal@402..403
                  IntLiteral@402..403 "0"
                Semicolon@403..404 ";"
              Blankspace@404..413 "\n        "
              VariableDeclaration@413..426
                Var@413..416 "var"
                Blankspace@416..417 " "
                Error@417..421
                  Loop@417..421 "loop"
                Blankspace@421..422 " "
                Equal@422..423 "="
                Blankspace@423..424 " "
                Literal@424..425
                  IntLiteral@424..425 "0"
                Semicolon@425..426 ";"
              Blankspace@426..435 "\n        "
              VariableDeclaration@435..439
                Var@435..438 "var"
                Blankspace@438..439 " "
              OverrideDeclaration@439..452
                Override@439..447 "override"
                Blankspace@447..448 " "
                Equal@448..449 "="
                Blankspace@449..450 " "
                Literal@450..451
                  IntLiteral@450..451 "0"
                Semicolon@451..452 ";"
              Blankspace@452..461 "\n        "
              VariableDeclaration@461..465
                Var@461..464 "var"
                Blankspace@464..465 " "
              RequiresDirective@465..478
                Requires@465..473 "requires"
                Blankspace@473..474 " "
                Error@474..477
                  Equal@474..475 "="
                  Blankspace@475..476 " "
                  IntLiteral@476..477 "0"
                Semicolon@477..478 ";"
              Blankspace@478..487 "\n        "
              VariableDeclaration@487..502
                Var@487..490 "var"
                Blankspace@490..491 " "
                Error@491..497
                  Return@491..497 "return"
                Blankspace@497..498 " "
                Equal@498..499 "="
                Blankspace@499..500 " "
                Literal@500..501
                  IntLiteral@500..501 "0"
                Semicolon@501..502 ";"
              Blankspace@502..511 "\n        "
              VariableDeclaration@511..515
                Var@511..514 "var"
                Blankspace@514..515 " "
              StructDeclaration@515..522
                Struct@515..521 "struct"
                Blankspace@521..522 " "
              Error@522..525
                Equal@522..523 "="
                Blankspace@523..524 " "
                IntLiteral@524..525 "0"
              Semicolon@525..526 ";"
              Blankspace@526..535 "\n        "
              VariableDeclaration@535..550
                Var@535..538 "var"
                Blankspace@538..539 " "
                Error@539..545
                  Switch@539..545 "switch"
                Blankspace@545..546 " "
                Equal@546..547 "="
                Blankspace@547..548 " "
                Literal@548..549
                  IntLiteral@548..549 "0"
                Semicolon@549..550 ";"
              Blankspace@550..559 "\n        "
              VariableDeclaration@559..572
                Var@559..562 "var"
                Blankspace@562..563 " "
                Error@563..567
                  True@563..567 "true"
                Blankspace@567..568 " "
                Equal@568..569 "="
                Blankspace@569..570 " "
                Literal@570..571
                  IntLiteral@570..571 "0"
                Semicolon@571..572 ";"
              Blankspace@572..581 "\n        "
              VariableDeclaration@581..585
                Var@581..584 "var"
                Blankspace@584..585 " "
              VariableDeclaration@585..593
                Var@585..588 "var"
                Blankspace@588..589 " "
                Equal@589..590 "="
                Blankspace@590..591 " "
                Literal@591..592
                  IntLiteral@591..592 "0"
                Semicolon@592..593 ";"
              Blankspace@593..602 "\n        "
              VariableDeclaration@602..616
                Var@602..605 "var"
                Blankspace@605..606 " "
                Error@606..611
                  While@606..611 "while"
                Blankspace@611..612 " "
                Equal@612..613 "="
                Blankspace@613..614 " "
                Literal@614..615
                  IntLiteral@614..615 "0"
                Semicolon@615..616 ";"
              Blankspace@616..625 "\n        "

            error at 13..18: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 19..20: invalid syntax, expected: <identifier>
            error at 21..22: invalid syntax, expected one of: <identifier>, 'package', 'super'
            error at 36..41: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 42..43: invalid syntax, expected: <identifier>
            error at 59..63: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 64..65: invalid syntax, expected: <identifier>
            error at 81..86: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 87..88: invalid syntax, expected: <identifier>
            error at 104..116: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 117..118: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '-', 'package', '(', '*', 'super', '~', 'true'
            error at 134..142: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 143..144: invalid syntax, expected: <identifier>
            error at 160..170: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 171..172: invalid syntax, expected: <identifier>
            error at 188..195: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 196..197: invalid syntax, expected: <identifier>
            error at 213..223: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 224..225: invalid syntax, expected: '('
            error at 241..248: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 249..250: invalid syntax, expected: <identifier>
            error at 266..270: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 271..272: invalid syntax, expected: <identifier>
            error at 288..294: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 295..296: invalid syntax, expected: <identifier>
            error at 312..317: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 318..319: invalid syntax, expected: <identifier>
            error at 335..337: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 338..339: invalid syntax, expected: <identifier>
            error at 341..342: invalid syntax, expected one of: '@', '{'
            error at 355..358: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 359..360: invalid syntax, expected: <identifier>
            error at 376..378: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 379..380: invalid syntax, expected: <identifier>
            error at 396..399: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 400..401: invalid syntax, expected: <identifier>
            error at 396..404: global let declarations are not allowed
            error at 417..421: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 422..423: invalid syntax, expected: <identifier>
            error at 439..447: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 448..449: invalid syntax, expected: <identifier>
            error at 465..473: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 474..475: invalid syntax, expected: <identifier>
            error at 491..497: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 498..499: invalid syntax, expected: <identifier>
            error at 515..521: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 522..523: invalid syntax, expected: <identifier>
            error at 539..545: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 546..547: invalid syntax, expected: <identifier>
            error at 563..567: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 568..569: invalid syntax, expected: <identifier>
            error at 585..588: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 589..590: invalid syntax, expected: <identifier>
            error at 606..611: invalid syntax, expected one of: '@', '{', '}', ',', '=', <identifier>, ')', ';', <template start>
            error at 612..613: invalid syntax, expected: <identifier>"#]],
    );
}
