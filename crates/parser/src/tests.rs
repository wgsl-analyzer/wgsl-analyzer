#![cfg_attr(not(test), allow(unused))]

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
              GlobalConstantDeclaration@9..36
                ConstDeclaration@9..35
                  Constant@9..14 "const"
                  Blankspace@14..15 " "
                  Identifier@15..18 "dim"
                  Colon@18..19 ":"
                  Blankspace@19..20 " "
                  TypeSpecifier@20..25
                    Identifier@20..25 "vec3u"
                  Blankspace@25..26 " "
                  Equal@26..27 "="
                  Blankspace@27..28 " "
                  FunctionCall@28..35
                    IdentExpression@28..33
                      Identifier@28..33 "vec3u"
                    Arguments@33..35
                      ParenthesisLeft@33..34 "("
                      ParenthesisRight@34..35 ")"
                Semicolon@35..36 ";"
              Blankspace@36..45 "\n        "
              FunctionDeclaration@45..78
                FunctionHeader@45..74
                  Fn@45..47 "fn"
                  Blankspace@47..48 " "
                  Identifier@48..52 "test"
                  FunctionParameters@52..74
                    ParenthesisLeft@52..53 "("
                    Parameter@53..73
                      Identifier@53..54 "a"
                      Colon@54..55 ":"
                      Blankspace@55..56 " "
                      TypeSpecifier@56..73
                        Identifier@56..61 "array"
                        GenericArgumentList@61..73
                          LessThan@61..62 "<"
                          IdentExpression@62..65
                            Identifier@62..65 "f32"
                          Comma@65..66 ","
                          Blankspace@66..67 " "
                          FieldExpression@67..72
                            IdentExpression@67..70
                              Identifier@67..70 "dim"
                            Period@70..71 "."
                            Identifier@71..72 "x"
                          GreaterThan@72..73 ">"
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
              GlobalConstantDeclaration@9..36
                ConstDeclaration@9..35
                  Constant@9..14 "const"
                  Blankspace@14..15 " "
                  Identifier@15..18 "dim"
                  Colon@18..19 ":"
                  Blankspace@19..20 " "
                  TypeSpecifier@20..25
                    Identifier@20..25 "vec3u"
                  Blankspace@25..26 " "
                  Equal@26..27 "="
                  Blankspace@27..28 " "
                  FunctionCall@28..35
                    IdentExpression@28..33
                      Identifier@28..33 "vec3u"
                    Arguments@33..35
                      ParenthesisLeft@33..34 "("
                      ParenthesisRight@34..35 ")"
                Semicolon@35..36 ";"
              Blankspace@36..45 "\n        "
              FunctionDeclaration@45..77
                FunctionHeader@45..73
                  Fn@45..47 "fn"
                  Blankspace@47..48 " "
                  Identifier@48..52 "test"
                  FunctionParameters@52..73
                    ParenthesisLeft@52..53 "("
                    Parameter@53..72
                      Identifier@53..54 "a"
                      Colon@54..55 ":"
                      Blankspace@55..56 " "
                      TypeSpecifier@56..72
                        Identifier@56..61 "array"
                        GenericArgumentList@61..72
                          LessThan@61..62 "<"
                          IdentExpression@62..65
                            Identifier@62..65 "f32"
                          Comma@65..66 ","
                          Blankspace@66..67 " "
                          FieldExpression@67..71
                            IdentExpression@67..70
                              Identifier@67..70 "dim"
                            Period@70..71 "."
                          GreaterThan@71..72 ">"
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
                FunctionHeader@0..7
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..7 "name"
                  FunctionParameters@7..7
                Error@7..7

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
              GlobalConstantDeclaration@9..25
                ConstDeclaration@9..24
                  Constant@9..14 "const"
                  Blankspace@14..15 " "
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
              GlobalConstantDeclaration@66..82
                ConstDeclaration@66..81
                  Constant@66..71 "const"
                  Blankspace@71..72 " "
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
              Blankspace@167..168 " ""#]],
    );
}

#[test]
fn function() {
    check(
        "fn name(a: f32, b: i32) -> f32 {}",
        expect![[r#"
            SourceFile@0..33
              FunctionDeclaration@0..33
                FunctionHeader@0..30
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..7 "name"
                  FunctionParameters@7..23
                    ParenthesisLeft@7..8 "("
                    Parameter@8..14
                      Identifier@8..9 "a"
                      Colon@9..10 ":"
                      Blankspace@10..11 " "
                      TypeSpecifier@11..14
                        Identifier@11..14 "f32"
                    Comma@14..15 ","
                    Blankspace@15..16 " "
                    Parameter@16..22
                      Identifier@16..17 "b"
                      Colon@17..18 ":"
                      Blankspace@18..19 " "
                      TypeSpecifier@19..22
                        Identifier@19..22 "i32"
                    ParenthesisRight@22..23 ")"
                  Blankspace@23..24 " "
                  ReturnType@24..30
                    Arrow@24..26 "->"
                    Blankspace@26..27 " "
                    TypeSpecifier@27..30
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
                FunctionHeader@0..9
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..7 "name"
                  FunctionParameters@7..9
                    ParenthesisLeft@7..8 "("
                    ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..57
                  BraceLeft@10..11 "{"
                  Blankspace@11..12 "\n"
                  VariableStatement@12..29
                    LetDeclaration@12..28
                      Let@12..15 "let"
                      Blankspace@15..16 " "
                      Identifier@16..17 "x"
                      Colon@17..18 ":"
                      Blankspace@18..19 " "
                      TypeSpecifier@19..22
                        Identifier@19..22 "f32"
                      Blankspace@22..23 " "
                      Equal@23..24 "="
                      Blankspace@24..25 " "
                      Literal@25..28
                        FloatLiteral@25..28 "1.0"
                    Semicolon@28..29 ";"
                  Blankspace@29..30 "\n"
                  VariableStatement@30..47
                    LetDeclaration@30..46
                      Let@30..33 "let"
                      Blankspace@33..34 " "
                      Identifier@34..35 "y"
                      Colon@35..36 ":"
                      Blankspace@36..37 " "
                      TypeSpecifier@37..40
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
                FunctionHeader@0..9
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
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
                FunctionHeader@0..15
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..6 "foo"
                  FunctionParameters@6..8
                    ParenthesisLeft@6..7 "("
                    ParenthesisRight@7..8 ")"
                  Blankspace@8..9 " "
                  ReturnType@9..15
                    Arrow@9..11 "->"
                    Blankspace@11..12 " "
                    TypeSpecifier@12..15
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
                FunctionHeader@0..3
                  Fn@0..2 "fn"
                  Blankspace@2..3 "\n"
                  FunctionParameters@3..3
                Error@3..3
              FunctionDeclaration@3..10
                FunctionHeader@3..10
                  Fn@3..5 "fn"
                  Blankspace@5..6 " "
                  Identifier@6..10 "name"
                  FunctionParameters@10..10
                Error@10..10

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
                FunctionHeader@0..9
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..7 "name"
                  FunctionParameters@7..9
                    ParenthesisLeft@7..8 "("
                    ParenthesisRight@8..9 ")"
                Blankspace@9..18 "\n        "
                Error@18..18
              FunctionDeclaration@18..30
                FunctionHeader@18..27
                  Fn@18..20 "fn"
                  Blankspace@20..21 " "
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
                Identifier@0..3 "f32""#]],
    );
}

#[test]
fn parse_type_generic() {
    check_type(
        "vec3<f32>",
        expect![[r#"
            SourceFile@0..9
              TypeSpecifier@0..9
                Identifier@0..4 "vec3"
                GenericArgumentList@4..9
                  LessThan@4..5 "<"
                  IdentExpression@5..8
                    Identifier@5..8 "f32"
                  GreaterThan@8..9 ">""#]],
    );
}

#[test]
fn parse_type_generic_shift_ambiguity() {
    check_type(
        "array<vec3<f32, 2>>",
        expect![[r#"
            SourceFile@0..19
              TypeSpecifier@0..19
                Identifier@0..5 "array"
                GenericArgumentList@5..19
                  LessThan@5..6 "<"
                  IdentExpression@6..18
                    Identifier@6..10 "vec3"
                    GenericArgumentList@10..18
                      LessThan@10..11 "<"
                      IdentExpression@11..14
                        Identifier@11..14 "f32"
                      Comma@14..15 ","
                      Blankspace@15..16 " "
                      Literal@16..17
                        IntLiteral@16..17 "2"
                      GreaterThan@17..18 ">"
                  GreaterThan@18..19 ">""#]],
    );
}

#[test]
fn parse_type_generic_int() {
    check_type(
        "array<f32, 100>",
        expect![[r#"
            SourceFile@0..15
              TypeSpecifier@0..15
                Identifier@0..5 "array"
                GenericArgumentList@5..15
                  LessThan@5..6 "<"
                  IdentExpression@6..9
                    Identifier@6..9 "f32"
                  Comma@9..10 ","
                  Blankspace@10..11 " "
                  Literal@11..14
                    IntLiteral@11..14 "100"
                  GreaterThan@14..15 ">""#]],
    );
}

#[test]
fn parse_type_generic_empty() {
    check_type(
        "vec3<>",
        expect![[r#"
            SourceFile@0..6
              TypeSpecifier@0..6
                Identifier@0..4 "vec3"
                GenericArgumentList@4..6
                  LessThan@4..5 "<"
                  GreaterThan@5..6 ">"

            error at 5..6: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '(', '-', '*', '~', 'true'"#]],
    );
}

#[test]
fn parse_type_generic_comma_recover() {
    check_type(
        "vec3<,>",
        expect![[r#"
            SourceFile@0..7
              TypeSpecifier@0..7
                Identifier@0..4 "vec3"
                GenericArgumentList@4..7
                  LessThan@4..5 "<"
                  Comma@5..6 ","
                  GreaterThan@6..7 ">"

            error at 5..6: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '(', '-', '*', '~', 'true'"#]],
    );
}

#[test]
fn parse_type_generic_ptr() {
    check_type(
        "ptr<uniform, f32, read_write>",
        expect![[r#"
            SourceFile@0..29
              TypeSpecifier@0..29
                Identifier@0..3 "ptr"
                GenericArgumentList@3..29
                  LessThan@3..4 "<"
                  IdentExpression@4..11
                    Identifier@4..11 "uniform"
                  Comma@11..12 ","
                  Blankspace@12..13 " "
                  IdentExpression@13..16
                    Identifier@13..16 "f32"
                  Comma@16..17 ","
                  Blankspace@17..18 " "
                  IdentExpression@18..28
                    Identifier@18..28 "read_write"
                  GreaterThan@28..29 ">""#]],
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
                FunctionHeader@0..15
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..6 "foo"
                  FunctionParameters@6..8
                    ParenthesisLeft@6..7 "("
                    ParenthesisRight@7..8 ")"
                  Blankspace@8..9 " "
                  ReturnType@9..15
                    Arrow@9..11 "->"
                    Blankspace@11..12 " "
                    TypeSpecifier@12..15
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
                FunctionHeader@0..15
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..6 "foo"
                  FunctionParameters@6..8
                    ParenthesisLeft@6..7 "("
                    ParenthesisRight@7..8 ")"
                  Blankspace@8..9 " "
                  ReturnType@9..15
                    Arrow@9..11 "->"
                    Blankspace@11..12 " "
                    TypeSpecifier@12..15
                      Identifier@12..15 "u32"
                Blankspace@15..16 " "
                CompoundStatement@16..88
                  BraceLeft@16..17 "{"
                  Blankspace@17..30 "\n            "
                  VariableStatement@30..37
                    LetDeclaration@30..37
                      Let@30..33 "let"
                      Blankspace@33..34 " "
                      Identifier@34..35 "x"
                      Blankspace@35..36 " "
                      Equal@36..37 "="
                  Blankspace@37..50 "\n            "
                  VariableStatement@50..57
                    LetDeclaration@50..57
                      Let@50..53 "let"
                      Blankspace@53..54 " "
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

            error at 50..53: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '(', '-', '*', '~', 'true'
            error at 70..76: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '(', '-', '*', '~', 'true'
            error at 87..88: invalid syntax, expected: ';'"#]],
    );
}

#[test]
fn parse_statement_variable_decl() {
    check_statement(
        "let x = 3;",
        expect![[r#"
            SourceFile@0..10
              VariableStatement@0..10
                LetDeclaration@0..9
                  Let@0..3 "let"
                  Blankspace@3..4 " "
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
                  VariableStatement@14..24
                    LetDeclaration@14..23
                      Let@14..17 "let"
                      Blankspace@17..18 " "
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
                    VariableStatement@13..23
                      LetDeclaration@13..22
                        Let@13..16 "let"
                        Blankspace@16..17 " "
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
                    VariableStatement@18..28
                      LetDeclaration@18..27
                        Let@18..21 "let"
                        Blankspace@21..22 " "
                        Identifier@22..23 "x"
                        Blankspace@23..24 " "
                        Equal@24..25 "="
                        Blankspace@25..26 " "
                        Literal@26..27
                          IntLiteral@26..27 "3"
                      Semicolon@27..28 ";"
                    Blankspace@28..37 "\n        "
                    BraceRight@37..38 "}"

            error at 4..5: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '(', '-', '*', '~', 'true'"#]],
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
                    VariableStatement@20..30
                      LetDeclaration@20..29
                        Let@20..23 "let"
                        Blankspace@23..24 " "
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
                    VariableStatement@15..25
                      LetDeclaration@15..24
                        Let@15..18 "let"
                        Blankspace@18..19 " "
                        Identifier@19..20 "x"
                        Blankspace@20..21 " "
                        Equal@21..22 "="
                        Blankspace@22..23 " "
                        Literal@23..24
                          IntLiteral@23..24 "3"
                      Semicolon@24..25 ";"
                    Blankspace@25..34 "\n        "
                    BraceRight@34..35 "}"

            error at 3..4: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '(', '-', '*', '~', 'true'"#]],
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

            error at 22..23: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '(', '-', '*', '~', 'true'"#]],
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
                      Identifier@22..23 "i"
                    Blankspace@23..24 " "
                    Equal@24..25 "="
                    Blankspace@25..26 " "
                    InfixExpression@26..31
                      IdentExpression@26..27
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
    // TODO: I think this test is no longer useful
    check_statement(
        "for(let i = 0, i < 3, i = i + 1) {}",
        expect![[r#"
            SourceFile@0..35
              ForStatement@0..14
                For@0..3 "for"
                ParenthesisLeft@3..4 "("
                ForInitializer@4..13
                  LetDeclaration@4..13
                    Let@4..7 "let"
                    Blankspace@7..8 " "
                    Identifier@8..9 "i"
                    Blankspace@9..10 " "
                    Equal@10..11 "="
                    Blankspace@11..12 " "
                    Literal@12..13
                      IntLiteral@12..13 "0"
                Error@13..14
                  Error@13..14
                    Comma@13..14 ","
              Blankspace@14..15 " "
              Error@15..35
                Identifier@15..16 "i"
                Blankspace@16..17 " "
                LessThan@17..18 "<"
                Blankspace@18..19 " "
                IntLiteral@19..20 "3"
                Comma@20..21 ","
                Blankspace@21..22 " "
                Identifier@22..23 "i"
                Blankspace@23..24 " "
                Equal@24..25 "="
                Blankspace@25..26 " "
                Identifier@26..27 "i"
                Blankspace@27..28 " "
                Plus@28..29 "+"
                Blankspace@29..30 " "
                IntLiteral@30..31 "1"
                ParenthesisRight@31..32 ")"
                Blankspace@32..33 " "
                BraceLeft@33..34 "{"
                BraceRight@34..35 "}"

            error at 13..14: invalid syntax, expected: ';'"#]],
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
                Error@7..7

            error at 7..7: invalid syntax, expected: '{'"#]],
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
                      Identifier@4..5 "i"
                    Equal@5..6 "="
                    Literal@6..7
                      IntLiteral@6..7 "0"
                Semicolon@7..8 ";"
                Semicolon@8..9 ";"
                ParenthesisRight@9..10 ")"
                Error@10..10

            error at 10..10: invalid syntax, expected: '{'"#]],
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
                Error@12..12

            error at 12..12: invalid syntax, expected: '{'"#]],
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
                      Identifier@6..7 "a"
                    Blankspace@7..8 " "
                    Equal@8..9 "="
                    Blankspace@9..10 " "
                    Literal@10..11
                      IntLiteral@10..11 "1"
                ParenthesisRight@11..12 ")"
                Error@12..12

            error at 12..12: invalid syntax, expected: '{'"#]],
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
                BraceLeft@5..6 "{"
                Blankspace@6..7 " "
                ContinuingStatement@7..20
                  Continuing@7..17 "continuing"
                  Blankspace@17..18 " "
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
                BraceLeft@5..6 "{"
                Blankspace@6..7 " "
                ContinuingStatement@7..38
                  Continuing@7..17 "continuing"
                  Blankspace@17..18 " "
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
                      GreaterThan@31..32 ">"
                      Equal@32..33 "="
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
                VariableStatement@2..12
                  LetDeclaration@2..11
                    Let@2..5 "let"
                    Blankspace@5..6 " "
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
                    Identifier@0..1 "a"
                  Period@1..2 "."
                  Identifier@2..3 "b"
                Blankspace@3..4 " "
                Equal@4..5 "="
                Blankspace@5..6 " "
                InfixExpression@6..13
                  FieldExpression@6..9
                    IdentExpression@6..7
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
                FunctionHeader@0..6
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..4 "a"
                  FunctionParameters@4..6
                    ParenthesisLeft@4..5 "("
                    ParenthesisRight@5..6 ")"
                CompoundStatement@6..14
                  BraceLeft@6..7 "{"
                  Error@7..8
                    IntLiteral@7..8 "1"
                  Error@8..9
                    Plus@8..9 "+"
                  Error@9..10
                    IntLiteral@9..10 "2"
                  Error@10..11
                    Equal@10..11 "="
                  Error@11..12
                    IntLiteral@11..12 "3"
                  EmptyStatement@12..13
                    Semicolon@12..13 ";"
                  BraceRight@13..14 "}"

            error at 7..8: invalid syntax, expected one of: '&', '@', 'break', 'const', 'const_assert', 'continue', 'discard', 'for', <identifier>, 'if', '{', '(', 'let', 'loop', '}', 'return', ';', '*', 'switch', '_', 'var', 'while'"#]],
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
                  VariableStatement@4..11
                    LetDeclaration@4..11
                      Let@4..7 "let"
                      Blankspace@7..8 " "
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

            error at 12..13: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '(', '-', '*', '~', 'true'
            error at 25..26: invalid syntax, expected: ';'"#]],
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
                    Identifier@1..2 "a"
                Blankspace@2..3 " "
                PlusEqual@3..5 "+="
                Blankspace@5..6 " "
                FunctionCall@6..11
                  IdentExpression@6..9
                    Identifier@6..9 "foo"
                  Arguments@9..11
                    ParenthesisLeft@9..10 "("
                    ParenthesisRight@10..11 ")"
                Semicolon@11..12 ";""#]],
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
              VariableStatement@0..11
                VariableDeclaration@0..10
                  Var@0..3 "var"
                  Blankspace@3..4 " "
                  Identifier@4..5 "x"
                  Colon@5..6 ":"
                  Blankspace@6..7 " "
                  TypeSpecifier@7..10
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
              VariableStatement@0..21
                VariableDeclaration@0..20
                  Var@0..3 "var"
                  GenericArgumentList@3..13
                    LessThan@3..4 "<"
                    IdentExpression@4..12
                      Identifier@4..12 "function"
                    GreaterThan@12..13 ">"
                  Blankspace@13..14 " "
                  Identifier@14..15 "x"
                  Colon@15..16 ":"
                  Blankspace@16..17 " "
                  TypeSpecifier@17..20
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
              Attribute@0..12
                AttributeOperator@0..1 "@"
                Identifier@1..9 "location"
                Arguments@9..12
                  ParenthesisLeft@9..10 "("
                  Literal@10..11
                    IntLiteral@10..11 "0"
                  ParenthesisRight@11..12 ")""#]],
    );
    check_attribute(
        "@interpolate(flat)",
        expect![[r#"
            SourceFile@0..18
              Attribute@0..18
                AttributeOperator@0..1 "@"
                Identifier@1..12 "interpolate"
                Arguments@12..18
                  ParenthesisLeft@12..13 "("
                  IdentExpression@13..17
                    Identifier@13..17 "flat"
                  ParenthesisRight@17..18 ")""#]],
    );
    check_attribute(
        "@attr(1, 2, 0.0, ident)",
        expect![[r#"
            SourceFile@0..23
              Attribute@0..23
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
                FunctionHeader@0..10
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..7 "main"
                  FunctionParameters@7..10
                    ParenthesisLeft@7..8 "("
                    Parameter@8..9
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
                FunctionHeader@0..9
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..7 "main"
                  FunctionParameters@7..9
                    ParenthesisLeft@7..8 "("
                    ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..42
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  VariableStatement@24..32
                    LetDeclaration@24..32
                      Let@24..27 "let"
                      Blankspace@27..28 " "
                      Identifier@28..29 "x"
                      Blankspace@29..30 " "
                      IdentExpression@30..32
                        Identifier@30..32 "be"
                  Blankspace@32..41 "\n        "
                  BraceRight@41..42 "}"

            error at 30..32: invalid syntax, expected one of: ':', '=', ';'
            error at 41..42: invalid syntax, expected one of: '&', '&&', '@', '^', ':', ',', '.', <end of file>, '==', '!=', '>', '{', '[', '(', '<', '-', '%', '|', '||', '+', ']', ')', ';', '/', '*'"#]],
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
                FunctionHeader@0..9
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..7 "main"
                  FunctionParameters@7..9
                    ParenthesisLeft@7..8 "("
                    ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..59
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  VariableStatement@24..27
                    LetDeclaration@24..27
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
                FunctionHeader@0..9
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..7 "main"
                  FunctionParameters@7..9
                    ParenthesisLeft@7..8 "("
                    ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..61
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  VariableStatement@24..29
                    LetDeclaration@24..29
                      Let@24..27 "let"
                      Blankspace@27..28 " "
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
                FunctionHeader@0..9
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..7 "main"
                  FunctionParameters@7..9
                    ParenthesisLeft@7..8 "("
                    ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..63
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  VariableStatement@24..31
                    LetDeclaration@24..31
                      Let@24..27 "let"
                      Blankspace@27..28 " "
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

            error at 44..50: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '(', '-', '*', '~', 'true'"#]],
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
                FunctionHeader@0..9
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..7 "main"
                  FunctionParameters@7..9
                    ParenthesisLeft@7..8 "("
                    ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..39
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  VariableStatement@24..29
                    LetDeclaration@24..29
                      Let@24..27 "let"
                      Blankspace@27..28 " "
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
                FunctionHeader@0..9
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..7 "main"
                  FunctionParameters@7..9
                    ParenthesisLeft@7..8 "("
                    ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..41
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  VariableStatement@24..31
                    LetDeclaration@24..31
                      Let@24..27 "let"
                      Blankspace@27..28 " "
                      Identifier@28..29 "x"
                      Blankspace@29..30 " "
                      Equal@30..31 "="
                  Blankspace@31..40 "\n        "
                  BraceRight@40..41 "}"

            error at 40..41: invalid syntax, expected one of: '&', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '(', '-', '*', '~', 'true'"#]],
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
                FunctionHeader@0..9
                  Fn@0..2 "fn"
                  Blankspace@2..3 " "
                  Identifier@3..7 "main"
                  FunctionParameters@7..9
                    ParenthesisLeft@7..8 "("
                    ParenthesisRight@8..9 ")"
                Blankspace@9..10 " "
                CompoundStatement@10..37
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  VariableStatement@24..27
                    LetDeclaration@24..27
                      Let@24..27 "let"
                  Blankspace@27..36 "\n        "
                  BraceRight@36..37 "}"

            error at 36..37: invalid syntax, expected: <identifier>"#]],
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
                FunctionHeader@4..12
                  Fn@4..6 "fn"
                  Blankspace@6..7 " "
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
                FunctionHeader@28..36
                  Fn@28..30 "fn"
                  Blankspace@30..31 " "
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
                FunctionHeader@43..51
                  Fn@43..45 "fn"
                  Blankspace@45..46 " "
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
                Identifier@8..11 "UBO"
                Blankspace@11..12 " "
                StructBody@12..66
                  BraceLeft@12..13 "{"
                  Blankspace@13..16 "\n  "
                  StructMember@16..38
                    Identifier@16..31 "camera_position"
                    Colon@31..32 ":"
                    Blankspace@32..33 " "
                    TypeSpecifier@33..38
                      Identifier@33..38 "vec3f"
                  Comma@38..39 ","
                  Blankspace@39..42 "\n  "
                  StructMember@42..51
                    Identifier@42..46 "_pad"
                    Colon@46..47 ":"
                    Blankspace@47..48 " "
                    TypeSpecifier@48..51
                      Identifier@48..51 "u32"
                  Blankspace@51..54 "\n  "
                  StructMember@54..63
                    Identifier@54..58 "time"
                    Colon@58..59 ":"
                    Blankspace@59..60 " "
                    TypeSpecifier@60..63
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
fn struct_decl_semi() {
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
                Identifier@8..12 "Test"
                Blankspace@12..13 " "
                StructBody@13..46
                  BraceLeft@13..14 "{"
                  Blankspace@14..19 "\n    "
                  StructMember@19..25
                    Identifier@19..20 "a"
                    Colon@20..21 ":"
                    Blankspace@21..22 " "
                    TypeSpecifier@22..25
                      Identifier@22..25 "f32"
                  Semicolon@25..26 ";"
                  Blankspace@26..31 "\n    "
                  StructMember@31..43
                    Identifier@31..32 "b"
                    Colon@32..33 ":"
                    Blankspace@33..34 " "
                    TypeSpecifier@34..43
                      Identifier@34..38 "vec3"
                      GenericArgumentList@38..43
                        LessThan@38..39 "<"
                        IdentExpression@39..42
                          Identifier@39..42 "f32"
                        GreaterThan@42..43 ">"
                  Error@43..44
                    Semicolon@43..44 ";"
                  Blankspace@44..45 "\n"
                  BraceRight@45..46 "}"
              Blankspace@46..47 "\n"

            error at 31..32: invalid syntax, expected ','
            error at 43..44: invalid syntax, expected one of: '@', ',', <identifier>, '}', ';'"#]],
    );
}

#[test]
fn struct_decl() {
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
                Identifier@8..12 "Test"
                Blankspace@12..13 " "
                StructBody@13..46
                  BraceLeft@13..14 "{"
                  Blankspace@14..19 "\n    "
                  StructMember@19..25
                    Identifier@19..20 "a"
                    Colon@20..21 ":"
                    Blankspace@21..22 " "
                    TypeSpecifier@22..25
                      Identifier@22..25 "f32"
                  Comma@25..26 ","
                  Blankspace@26..31 "\n    "
                  StructMember@31..43
                    Identifier@31..32 "b"
                    Colon@32..33 ":"
                    Blankspace@33..34 " "
                    TypeSpecifier@34..43
                      Identifier@34..38 "vec3"
                      GenericArgumentList@38..43
                        LessThan@38..39 "<"
                        IdentExpression@39..42
                          Identifier@39..42 "f32"
                        GreaterThan@42..43 ">"
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
                Error@8..8
              FunctionDeclaration@8..18
                FunctionHeader@8..17
                  Fn@8..10 "fn"
                  Blankspace@10..11 " "
                  Identifier@11..15 "test"
                  FunctionParameters@15..17
                    ParenthesisLeft@15..16 "("
                    ParenthesisRight@16..17 ")"
                Blankspace@17..18 "\n"
                Error@18..18

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
                Identifier@8..12 "test"
                Blankspace@12..13 "\n"
                Error@13..13
              FunctionDeclaration@13..23
                FunctionHeader@13..22
                  Fn@13..15 "fn"
                  Blankspace@15..16 " "
                  Identifier@16..20 "test"
                  FunctionParameters@20..22
                    ParenthesisLeft@20..21 "("
                    ParenthesisRight@21..22 ")"
                Blankspace@22..23 "\n"
                Error@23..23

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
                Identifier@8..12 "test"
                Blankspace@12..13 " "
                StructBody@13..15
                  BraceLeft@13..14 "{"
                  StructMember@14..14
                    TypeSpecifier@14..14
                  BraceRight@14..15 "}"
              Blankspace@15..17 "\n\n"
              FunctionDeclaration@17..27
                FunctionHeader@17..26
                  Fn@17..19 "fn"
                  Blankspace@19..20 " "
                  Identifier@20..24 "test"
                  FunctionParameters@24..26
                    ParenthesisLeft@24..25 "("
                    ParenthesisRight@25..26 ")"
                Blankspace@26..27 "\n"
                Error@27..27
              Error@27..28
                BraceRight@27..28 "}"
              Semicolon@28..29 ";"
              Blankspace@29..30 "\n"

            error at 14..15: invalid syntax, expected: <identifier>
            error at 27..28: invalid syntax, expected one of: '->', '@', '{'"#]],
    );
}

#[test]
fn global_variable_decl_init() {
    check(
        "var flags = 0;",
        expect![[r#"
            SourceFile@0..14
              GlobalVariableDeclaration@0..14
                VariableDeclaration@0..13
                  Var@0..3 "var"
                  Blankspace@3..4 " "
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
fn global_const_decl() {
    check(
        "const constant = 0;",
        expect![[r#"
            SourceFile@0..19
              GlobalConstantDeclaration@0..19
                ConstDeclaration@0..18
                  Constant@0..5 "const"
                  Blankspace@5..6 " "
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
fn type_alias_decl() {
    check(
        "alias float = f32;",
        expect![[r#"
            SourceFile@0..18
              TypeAliasDeclaration@0..18
                Alias@0..5 "alias"
                Blankspace@5..6 " "
                Identifier@6..11 "float"
                Blankspace@11..12 " "
                Equal@12..13 "="
                Blankspace@13..14 " "
                TypeSpecifier@14..17
                  Identifier@14..17 "f32"
                Semicolon@17..18 ";""#]],
    );
}

#[test]
fn type_alias_decl_recover() {
    check(
        "alias float = f32\nalias other = u32;",
        expect![[r#"
            SourceFile@0..36
              TypeAliasDeclaration@0..17
                Alias@0..5 "alias"
                Blankspace@5..6 " "
                Identifier@6..11 "float"
                Blankspace@11..12 " "
                Equal@12..13 "="
                Blankspace@13..14 " "
                TypeSpecifier@14..17
                  Identifier@14..17 "f32"
              Blankspace@17..18 "\n"
              TypeAliasDeclaration@18..36
                Alias@18..23 "alias"
                Blankspace@23..24 " "
                Identifier@24..29 "other"
                Blankspace@29..30 " "
                Equal@30..31 "="
                Blankspace@31..32 " "
                TypeSpecifier@32..35
                  Identifier@32..35 "u32"
                Semicolon@35..36 ";"

            error at 18..23: invalid syntax, expected one of: '@', ',', <end of file>, '=', <identifier>, '{', '<', '}', ')', ';'"#]],
    );
}

#[test]
fn parse_statement_expression() {
    check_statement(
        "test(args);",
        expect![[r#"
            SourceFile@0..11
              FunctionCallStatement@0..11
                FunctionCall@0..10
                  IdentExpression@0..4
                    Identifier@0..4 "test"
                  Arguments@4..10
                    ParenthesisLeft@4..5 "("
                    IdentExpression@5..9
                      Identifier@5..9 "args"
                    ParenthesisRight@9..10 ")"
                Semicolon@10..11 ";""#]],
    );
}

#[test]
fn parse_statement_nested_functions() {
    check_statement(
        "test(args<a>());",
        expect![[r#"
            SourceFile@0..16
              FunctionCallStatement@0..16
                FunctionCall@0..15
                  IdentExpression@0..4
                    Identifier@0..4 "test"
                  Arguments@4..15
                    ParenthesisLeft@4..5 "("
                    FunctionCall@5..14
                      IdentExpression@5..12
                        Identifier@5..9 "args"
                        GenericArgumentList@9..12
                          LessThan@9..10 "<"
                          IdentExpression@10..11
                            Identifier@10..11 "a"
                          GreaterThan@11..12 ">"
                      Arguments@12..14
                        ParenthesisLeft@12..13 "("
                        ParenthesisRight@13..14 ")"
                    ParenthesisRight@14..15 ")"
                Semicolon@15..16 ";""#]],
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
                VariableStatement@2..12
                  LetDeclaration@2..11
                    Let@2..5 "let"
                    Blankspace@5..6 " "
                    Identifier@6..7 "x"
                    Blankspace@7..8 " "
                    Equal@8..9 "="
                    Blankspace@9..10 " "
                    Literal@10..11
                      IntLiteral@10..11 "3"
                  Semicolon@11..12 ";"
                Blankspace@12..13 " "
                ReturnStatement@13..21
                  Return@13..19 "return"
                  Blankspace@19..20 " "
                  IdentExpression@20..21
                    Identifier@20..21 "x"
                Blankspace@21..22 " "
                BraceRight@22..23 "}"
              Blankspace@23..24 " "

            error at 22..23: invalid syntax, expected one of: '&', '&&', '@', '^', ':', ',', '.', <end of file>, '==', '!=', '>', '{', '[', '(', '<', '-', '%', '|', '||', '+', ']', ')', ';', '/', '*'"#]],
    );
}

#[test]
fn parse_import() {
    check(
        "#import test",
        expect![[r##"
            SourceFile@0..12
              Import@0..12
                UnofficialPreprocessorImport@0..7 "#import"
                Blankspace@7..8 " "
                ImportCustom@8..12
                  Identifier@8..12 "test""##]],
    );
}

#[test]
fn parse_import_colon() {
    check(
        "#import bevy_pbr::mesh_struct",
        expect![[r##"
            SourceFile@0..29
              Import@0..29
                UnofficialPreprocessorImport@0..7 "#import"
                Blankspace@7..8 " "
                ImportCustom@8..29
                  Identifier@8..16 "bevy_pbr"
                  ColonColon@16..18 "::"
                  Identifier@18..29 "mesh_struct""##]],
    );
}

#[test]
fn parse_string_import() {
    check(
        r#"#import "file.wgsl""#,
        expect![[r##"
            SourceFile@0..19
              Import@0..19
                UnofficialPreprocessorImport@0..7 "#import"
                Blankspace@7..8 " "
                ImportPath@8..19
                  StringLiteral@8..19 "\"file.wgsl\"""##]],
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
                  Identifier@8..9 "i"
                Blankspace@9..10 " "
                SwitchBody@10..70
                  BraceLeft@10..11 "{"
                  Blankspace@11..14 "\n  "
                  SwitchBodyCase@14..25
                    Case@14..18 "case"
                    Blankspace@18..19 " "
                    SwitchCaseSelectors@19..20
                      SwitchCaseSelector@19..20
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
                      SwitchCaseSelector@33..34
                        Literal@33..34
                          IntLiteral@33..34 "1"
                      Comma@34..35 ","
                      Blankspace@35..36 " "
                      SwitchCaseSelector@36..37
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
                  SwitchBodyDefault@56..68
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
                  Identifier@8..9 "i"
                Blankspace@9..10 " "
                SwitchBody@10..20
                  BraceLeft@10..11 "{"
                  Blankspace@11..14 "\n  "
                  SwitchBodyCase@14..19
                    Case@14..18 "case"
                    Blankspace@18..19 "\n"
                    SwitchCaseSelectors@19..19
                      SwitchCaseSelector@19..19
                    Error@19..19
                  BraceRight@19..20 "}"
              Blankspace@20..29 "\n        "

            error at 19..20: invalid syntax, expected one of: '&', 'default', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '(', '-', '*', '~', 'true'"#]],
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
                  Identifier@8..9 "i"
                Blankspace@9..10 " "
                SwitchBody@10..22
                  BraceLeft@10..11 "{"
                  Blankspace@11..14 "\n  "
                  SwitchBodyCase@14..21
                    Case@14..18 "case"
                    Blankspace@18..19 " "
                    SwitchCaseSelectors@19..20
                      SwitchCaseSelector@19..20
                        Literal@19..20
                          IntLiteral@19..20 "1"
                    Blankspace@20..21 "\n"
                    Error@21..21
                  BraceRight@21..22 "}"
              Blankspace@22..31 "\n        "

            error at 21..22: invalid syntax, expected one of: '@', ':', ',', '{'"#]],
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
                    Identifier@10..11 "i"
                  Blankspace@11..12 " "
                  SwitchBody@12..25
                    BraceLeft@12..13 "{"
                    Blankspace@13..16 "\n  "
                    SwitchBodyCase@16..24
                      Case@16..20 "case"
                      Blankspace@20..21 " "
                      SwitchCaseSelectors@21..22
                        SwitchCaseSelector@21..22
                          Literal@21..22
                            IntLiteral@21..22 "1"
                      Colon@22..23 ":"
                      Blankspace@23..24 "\n"
                      Error@24..24
                    BraceRight@24..25 "}"
                Blankspace@25..27 "\n\n"
                VariableStatement@27..37
                  LetDeclaration@27..36
                    Let@27..30 "let"
                    Blankspace@30..31 " "
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

            error at 24..25: invalid syntax, expected: '{'"#]],
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
                    Identifier@10..11 "i"
                  Blankspace@11..12 " "
                  SwitchBody@12..28
                    BraceLeft@12..13 "{"
                    Blankspace@13..16 "\n  "
                    SwitchBodyCase@16..27
                      Case@16..20 "case"
                      Blankspace@20..21 " "
                      SwitchCaseSelectors@21..27
                        SwitchCaseSelector@21..22
                          Literal@21..22
                            IntLiteral@21..22 "1"
                        Comma@22..23 ","
                        Blankspace@23..24 " "
                        SwitchCaseSelector@24..25
                          Literal@24..25
                            IntLiteral@24..25 "2"
                        Comma@25..26 ","
                        Blankspace@26..27 "\n"
                        SwitchCaseSelector@27..27
                      Error@27..27
                    BraceRight@27..28 "}"
                Blankspace@28..29 "\n"
                VariableStatement@29..39
                  LetDeclaration@29..38
                    Let@29..32 "let"
                    Blankspace@32..33 " "
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

            error at 27..28: invalid syntax, expected one of: '&', 'default', '!', 'false', <floating point literal>, <identifier>, <integer literal>, '(', '-', '*', '~', 'true'"#]],
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
              GlobalAssert@0..19
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
              GlobalOverrideDeclaration@0..22
                OverrideDeclaration@0..21
                  Override@0..8 "override"
                  Blankspace@8..9 " "
                  Identifier@9..12 "foo"
                  Colon@12..13 ":"
                  Blankspace@13..14 " "
                  TypeSpecifier@14..17
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
                Attribute@0..9
                  AttributeOperator@0..1 "@"
                  Identifier@1..9 "fragment"

            error at 9..9: invalid syntax, expected one of: 'fn', 'override', 'var'"#]],
    );
}
