#![cfg_attr(not(test), allow(unused))]
use expect_test::{Expect, expect};

use crate::ParseEntryPoint;

fn check(
    input: &str,
    expected_tree: Expect,
) {
    crate::check_entrypoint(input, ParseEntryPoint::File, expected_tree);
}

fn check_type(
    input: &str,
    expected_tree: Expect,
) {
    crate::check_entrypoint(input, ParseEntryPoint::Type, expected_tree);
}

fn check_statement(
    statement: &str,
    expected_tree: Expect,
) {
    crate::check_entrypoint(statement, ParseEntryPoint::Statement, expected_tree);
}

fn check_attribute_list(
    statement: &str,
    expected_tree: Expect,
) {
    crate::check_entrypoint(statement, ParseEntryPoint::AttributeList, expected_tree);
}

#[test]
fn can_parse_array_declaration() {
    check(
        r#"
		const dim: vec3u = vec3u();
		fn test(a: array<f32, dim.x>) { }
		"#,
        expect![[r#"
            SourceFile@0..69
              Blankspace@0..3 "\n\t\t"
              GlobalConstantDeclaration@3..33
                Constant@3..8 "const"
                Blankspace@8..9 " "
                Binding@9..12
                  Name@9..12
                    Identifier@9..12 "dim"
                Colon@12..13 ":"
                Blankspace@13..14 " "
                PathType@14..20
                  NameReference@14..20
                    Identifier@14..19 "vec3u"
                    Blankspace@19..20 " "
                Equal@20..21 "="
                Blankspace@21..22 " "
                FunctionCall@22..29
                  NameReference@22..27
                    Identifier@22..27 "vec3u"
                  FunctionParameterList@27..29
                    ParenthesisLeft@27..28 "("
                    ParenthesisRight@28..29 ")"
                Semicolon@29..30 ";"
                Blankspace@30..33 "\n\t\t"
              Function@33..69
                Fn@33..35 "fn"
                Blankspace@35..36 " "
                Name@36..40
                  Identifier@36..40 "test"
                ParameterList@40..63
                  ParenthesisLeft@40..41 "("
                  Parameter@41..61
                    VariableIdentDeclaration@41..61
                      Binding@41..42
                        Name@41..42
                          Identifier@41..42 "a"
                      Colon@42..43 ":"
                      Blankspace@43..44 " "
                      Array@44..61
                        Array@44..49 "array"
                        GenericArgumentList@49..61
                          LessThan@49..50 "<"
                          Float32@50..53
                            Float32@50..53 "f32"
                          Comma@53..54 ","
                          Blankspace@54..55 " "
                          PathType@55..60
                            NameReference@55..58
                              Identifier@55..58 "dim"
                            FieldExpression@58..60
                              Period@58..59 "."
                              Identifier@59..60 "x"
                          GreaterThan@60..61 ">"
                  ParenthesisRight@61..62 ")"
                  Blankspace@62..63 " "
                CompoundStatement@63..69
                  BraceLeft@63..64 "{"
                  Blankspace@64..65 " "
                  BraceRight@65..66 "}"
                  Blankspace@66..69 "\n\t\t""#]],
    );
}

#[test]
fn cannot_parse_bad_array_declaration() {
    check(
        r#"
		const dim: vec3u = vec3u();
		fn test(a: array<f32, dim.>) { }
		"#,
        expect![[r#"
            SourceFile@0..68
              Blankspace@0..3 "\n\t\t"
              GlobalConstantDeclaration@3..33
                Constant@3..8 "const"
                Blankspace@8..9 " "
                Binding@9..12
                  Name@9..12
                    Identifier@9..12 "dim"
                Colon@12..13 ":"
                Blankspace@13..14 " "
                PathType@14..20
                  NameReference@14..20
                    Identifier@14..19 "vec3u"
                    Blankspace@19..20 " "
                Equal@20..21 "="
                Blankspace@21..22 " "
                FunctionCall@22..29
                  NameReference@22..27
                    Identifier@22..27 "vec3u"
                  FunctionParameterList@27..29
                    ParenthesisLeft@27..28 "("
                    ParenthesisRight@28..29 ")"
                Semicolon@29..30 ";"
                Blankspace@30..33 "\n\t\t"
              Function@33..68
                Fn@33..35 "fn"
                Blankspace@35..36 " "
                Name@36..40
                  Identifier@36..40 "test"
                ParameterList@40..68
                  ParenthesisLeft@40..41 "("
                  Parameter@41..68
                    VariableIdentDeclaration@41..68
                      Binding@41..42
                        Name@41..42
                          Identifier@41..42 "a"
                      Colon@42..43 ":"
                      Blankspace@43..44 " "
                      Array@44..68
                        Array@44..49 "array"
                        GenericArgumentList@49..68
                          LessThan@49..50 "<"
                          Float32@50..53
                            Float32@50..53 "f32"
                          Comma@53..54 ","
                          Blankspace@54..55 " "
                          PathType@55..60
                            NameReference@55..58
                              Identifier@55..58 "dim"
                            FieldExpression@58..60
                              Period@58..59 "."
                              Error@59..60
                                GreaterThan@59..60 ">"
                          Error@60..62
                            ParenthesisRight@60..61 ")"
                            Blankspace@61..62 " "
                          Error@62..64
                            BraceLeft@62..63 "{"
                            Blankspace@63..64 " "
                          Error@64..68
                            BraceRight@64..65 "}"
                            Blankspace@65..68 "\n\t\t"

            error at 59..60: expected Identifier, but found GreaterThan
            error at 60..61: expected Period, Comma, GreaterThan, or Identifier, but found ParenthesisRight
            error at 62..63: expected Comma, GreaterThan, or Identifier, but found BraceLeft
            error at 64..65: expected Comma, GreaterThan, or Identifier, but found BraceRight
            error at 65..68: expected Comma or GreaterThan
            error at 65..68: expected Comma or ParenthesisRight
            error at 65..68: expected Arrow or BraceLeft"#]],
    );
}

#[test]
fn parse_empty() {
    check("", expect![[r#"SourceFile@0..0"#]]);
}

#[test]
fn fn_incomplete() {
    check(
        "fn name",
        expect![[r#"
            SourceFile@0..7
              Function@0..7
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "name"

            error at 3..7: expected ParenthesisLeft
            error at 3..7: expected Arrow or BraceLeft"#]],
    );
}

#[test]
fn parse_comments() {
    check(
        r#"
		const f = 1.5; // This is line-ending comment.
		const g = 2.5; /* This is a block comment
                that spans lines.
                /* Block comments can nest.
                 */
                But all block comments must terminate.
               */
		"#,
        expect![[r#"
            SourceFile@0..267
              Blankspace@0..3 "\n\t\t"
              GlobalConstantDeclaration@3..52
                Constant@3..8 "const"
                Blankspace@8..9 " "
                Binding@9..11
                  Name@9..11
                    Identifier@9..10 "f"
                    Blankspace@10..11 " "
                Equal@11..12 "="
                Blankspace@12..13 " "
                Literal@13..16
                  DecimalFloatLiteral@13..16 "1.5"
                Semicolon@16..17 ";"
                Blankspace@17..18 " "
                LineEndingComment@18..49 "// This is line-endin ..."
                Blankspace@49..52 "\n\t\t"
              GlobalConstantDeclaration@52..267
                Constant@52..57 "const"
                Blankspace@57..58 " "
                Binding@58..60
                  Name@58..60
                    Identifier@58..59 "g"
                    Blankspace@59..60 " "
                Equal@60..61 "="
                Blankspace@61..62 " "
                Literal@62..65
                  DecimalFloatLiteral@62..65 "2.5"
                Semicolon@65..66 ";"
                Blankspace@66..67 " "
                BlockComment@67..264 "/* This is a block co ..."
                Blankspace@264..267 "\n\t\t""#]],
    );
}

#[test]
fn cannot_parse_unmatched_block_comment() {
    check(
        r#"
		/* This is a block comment that spans lines.
			/* Block comments can nest.
			But all block comments must terminate.
			*/
		"#,
        expect![[r#"
            SourceFile@0..129
              Blankspace@0..3 "\n\t\t"
              Error@3..129
                Error@3..129
                  Error@3..128 "/* This is a block co ..."
                  Blankspace@128..129 "\t"

            error at 3..128: expected Fn, Struct, Var, Let, Constant, Alias, or Override, but found Error"#]],
    );
}

#[test]
fn function() {
    check(
        "fn name(a: f32, b: i32) -> f32 {}",
        expect![[r#"
            SourceFile@0..33
              Function@0..33
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "name"
                ParameterList@7..24
                  ParenthesisLeft@7..8 "("
                  Parameter@8..14
                    VariableIdentDeclaration@8..14
                      Binding@8..9
                        Name@8..9
                          Identifier@8..9 "a"
                      Colon@9..10 ":"
                      Blankspace@10..11 " "
                      Float32@11..14
                        Float32@11..14 "f32"
                  Comma@14..15 ","
                  Blankspace@15..16 " "
                  Parameter@16..22
                    VariableIdentDeclaration@16..22
                      Binding@16..17
                        Name@16..17
                          Identifier@16..17 "b"
                      Colon@17..18 ":"
                      Blankspace@18..19 " "
                      Int32@19..22
                        Int32@19..22 "i32"
                  ParenthesisRight@22..23 ")"
                  Blankspace@23..24 " "
                ReturnType@24..31
                  Arrow@24..26 "->"
                  Blankspace@26..27 " "
                  Float32@27..31
                    Float32@27..30 "f32"
                    Blankspace@30..31 " "
                CompoundStatement@31..33
                  BraceLeft@31..32 "{"
                  BraceRight@32..33 "}""#]],
    );
}

#[test]
fn variable_declarations() {
    check(
        r#"fn name() {
let x: f32 = 1.0;
let y: f32 = 2.0;
        }"#,
        expect![[r#"
            SourceFile@0..57
              Function@0..57
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "name"
                ParameterList@7..10
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                  Blankspace@9..10 " "
                CompoundStatement@10..57
                  BraceLeft@10..11 "{"
                  Blankspace@11..12 "\n"
                  VariableStatement@12..28
                    Let@12..15 "let"
                    Blankspace@15..16 " "
                    Binding@16..17
                      Name@16..17
                        Identifier@16..17 "x"
                    Colon@17..18 ":"
                    Blankspace@18..19 " "
                    Float32@19..23
                      Float32@19..22 "f32"
                      Blankspace@22..23 " "
                    Equal@23..24 "="
                    Blankspace@24..25 " "
                    Literal@25..28
                      DecimalFloatLiteral@25..28 "1.0"
                  Semicolon@28..29 ";"
                  Blankspace@29..30 "\n"
                  VariableStatement@30..46
                    Let@30..33 "let"
                    Blankspace@33..34 " "
                    Binding@34..35
                      Name@34..35
                        Identifier@34..35 "y"
                    Colon@35..36 ":"
                    Blankspace@36..37 " "
                    Float32@37..41
                      Float32@37..40 "f32"
                      Blankspace@40..41 " "
                    Equal@41..42 "="
                    Blankspace@42..43 " "
                    Literal@43..46
                      DecimalFloatLiteral@43..46 "2.0"
                  Semicolon@46..47 ";"
                  Blankspace@47..56 "\n        "
                  BraceRight@56..57 "}""#]],
    );
}

#[test]
fn fn_recover() {
    check(
        "fn\nfn name",
        expect![[r#"
            SourceFile@0..10
              Function@0..3
                Fn@0..2 "fn"
                Blankspace@2..3 "\n"
              Function@3..10
                Fn@3..5 "fn"
                Blankspace@5..6 " "
                Name@6..10
                  Identifier@6..10 "name"

            error at 6..10: expected ParenthesisLeft
            error at 6..10: expected Arrow or BraceLeft"#]],
    );
}

#[test]
fn fn_recover_2() {
    check(
        r#"fn name()
        fn test() {}"#,
        expect![[r#"
            SourceFile@0..30
              Function@0..18
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "name"
                ParameterList@7..18
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                  Blankspace@9..18 "\n        "
              Function@18..30
                Fn@18..20 "fn"
                Blankspace@20..21 " "
                Name@21..25
                  Identifier@21..25 "test"
                ParameterList@25..28
                  ParenthesisLeft@25..26 "("
                  ParenthesisRight@26..27 ")"
                  Blankspace@27..28 " "
                CompoundStatement@28..30
                  BraceLeft@28..29 "{"
                  BraceRight@29..30 "}"

            error at 18..20: expected Arrow or BraceLeft, but found Fn"#]],
    );
}

#[test]
fn parse_type_primitive() {
    check_type(
        "f32",
        expect![[r#"
            Float32@0..3
              Float32@0..3 "f32""#]],
    );
}

#[test]
fn parse_type_generic() {
    check_type(
        "vec3<f32>",
        expect![[r#"
            Vec3@0..9
              Vec3@0..4 "vec3"
              GenericArgumentList@4..9
                LessThan@4..5 "<"
                Float32@5..8
                  Float32@5..8 "f32"
                GreaterThan@8..9 ">""#]],
    );
}

#[test]
fn parse_type_generic_shift_ambiguity() {
    check_type(
        "array<vec3<f32, 2>>",
        expect![[r#"
            Array@0..19
              Array@0..5 "array"
              GenericArgumentList@5..19
                LessThan@5..6 "<"
                Vec3@6..18
                  Vec3@6..10 "vec3"
                  GenericArgumentList@10..18
                    LessThan@10..11 "<"
                    Float32@11..14
                      Float32@11..14 "f32"
                    Comma@14..15 ","
                    Blankspace@15..16 " "
                    Literal@16..17
                      DecimalIntLiteral@16..17 "2"
                    GreaterThan@17..18 ">"
                GreaterThan@18..19 ">""#]],
    );
}

#[test]
fn parse_type_generic_int() {
    check_type(
        "array<f32, 100>",
        expect![[r#"
            Array@0..15
              Array@0..5 "array"
              GenericArgumentList@5..15
                LessThan@5..6 "<"
                Float32@6..9
                  Float32@6..9 "f32"
                Comma@9..10 ","
                Blankspace@10..11 " "
                Literal@11..14
                  DecimalIntLiteral@11..14 "100"
                GreaterThan@14..15 ">""#]],
    );
}

#[test]
fn parse_type_generic_empty() {
    check_type(
        "vec3<>",
        expect![[r#"
            Vec3@0..6
              Vec3@0..4 "vec3"
              GenericArgumentList@4..6
                LessThan@4..5 "<"
                GreaterThan@5..6 ">""#]],
    );
}

#[test]
fn parse_type_generic_comma_recover() {
    check_type(
        "vec3<,>",
        expect![[r#"
            Vec3@0..7
              Vec3@0..4 "vec3"
              GenericArgumentList@4..7
                LessThan@4..5 "<"
                Error@5..6
                  Comma@5..6 ","
                GreaterThan@6..7 ">"

            error at 5..6: expected GreaterThan or Identifier, but found Comma"#]],
    );
}

#[test]
fn parse_type_generic_ptr() {
    check_type(
        "ptr<uniform, f32, read_write>",
        expect![[r#"
            Pointer@0..29
              Pointer@0..3 "ptr"
              GenericArgumentList@3..29
                LessThan@3..4 "<"
                Uniform@4..11 "uniform"
                Comma@11..12 ","
                Blankspace@12..13 " "
                Float32@13..16
                  Float32@13..16 "f32"
                Comma@16..17 ","
                Blankspace@17..18 " "
                ReadWrite@18..28 "read_write"
                GreaterThan@28..29 ">""#]],
    );
}

#[test]
fn parse_return_statement() {
    check(
        r#"fn f() -> u32 {
            return 0;
        }"#,
        expect![[r#"
            SourceFile@0..47
              Function@0..47
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                ParameterList@4..7
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                  Blankspace@6..7 " "
                ReturnType@7..14
                  Arrow@7..9 "->"
                  Blankspace@9..10 " "
                  Uint32@10..14
                    Uint32@10..13 "u32"
                    Blankspace@13..14 " "
                CompoundStatement@14..47
                  BraceLeft@14..15 "{"
                  Blankspace@15..28 "\n            "
                  ReturnStatement@28..36
                    Return@28..34 "return"
                    Blankspace@34..35 " "
                    Literal@35..36
                      DecimalIntLiteral@35..36 "0"
                  Semicolon@36..37 ";"
                  Blankspace@37..46 "\n        "
                  BraceRight@46..47 "}""#]],
    );
}

#[test]
fn parse_let_statement_recover() {
    check(
        r#"fn f() -> u32 {
            let x =
            let y =
            return 0
        }"#,
        expect![[r#"
            SourceFile@0..86
              Function@0..86
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                ParameterList@4..7
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                  Blankspace@6..7 " "
                ReturnType@7..14
                  Arrow@7..9 "->"
                  Blankspace@9..10 " "
                  Uint32@10..14
                    Uint32@10..13 "u32"
                    Blankspace@13..14 " "
                CompoundStatement@14..86
                  BraceLeft@14..15 "{"
                  Blankspace@15..28 "\n            "
                  VariableStatement@28..48
                    Let@28..31 "let"
                    Blankspace@31..32 " "
                    Binding@32..34
                      Name@32..34
                        Identifier@32..33 "x"
                        Blankspace@33..34 " "
                    Equal@34..35 "="
                    Blankspace@35..48 "\n            "
                  VariableStatement@48..68
                    Let@48..51 "let"
                    Blankspace@51..52 " "
                    Binding@52..54
                      Name@52..54
                        Identifier@52..53 "y"
                        Blankspace@53..54 " "
                    Equal@54..55 "="
                    Blankspace@55..68 "\n            "
                  ReturnStatement@68..85
                    Return@68..74 "return"
                    Blankspace@74..75 " "
                    Literal@75..85
                      DecimalIntLiteral@75..76 "0"
                      Blankspace@76..85 "\n        "
                  BraceRight@85..86 "}""#]],
    );
}

#[test]
fn parse_statement_variable_decl() {
    check_statement(
        "let x = 3;",
        expect![[r#"
            VariableStatement@0..9
              Let@0..3 "let"
              Blankspace@3..4 " "
              Binding@4..6
                Name@4..6
                  Identifier@4..5 "x"
                  Blankspace@5..6 " "
              Equal@6..7 "="
              Blankspace@7..8 " "
              Literal@8..9
                DecimalIntLiteral@8..9 "3""#]],
    );
}

#[test]
fn parse_statement_return() {
    check_statement(
        "return 0;",
        expect![[r#"
            ReturnStatement@0..8
              Return@0..6 "return"
              Blankspace@6..7 " "
              Literal@7..8
                DecimalIntLiteral@7..8 "0""#]],
    );
}

#[test]
fn parse_while_statement() {
    check_statement(
        r#"while 0 > 3 { let x = 3; }"#,
        expect![[r#"
            WhileStatement@0..26
              While@0..5 "while"
              Blankspace@5..6 " "
              InfixExpression@6..12
                Literal@6..8
                  DecimalIntLiteral@6..7 "0"
                  Blankspace@7..8 " "
                GreaterThan@8..9 ">"
                Blankspace@9..10 " "
                Literal@10..12
                  DecimalIntLiteral@10..11 "3"
                  Blankspace@11..12 " "
              CompoundStatement@12..26
                BraceLeft@12..13 "{"
                Blankspace@13..14 " "
                VariableStatement@14..23
                  Let@14..17 "let"
                  Blankspace@17..18 " "
                  Binding@18..20
                    Name@18..20
                      Identifier@18..19 "x"
                      Blankspace@19..20 " "
                  Equal@20..21 "="
                  Blankspace@21..22 " "
                  Literal@22..23
                    DecimalIntLiteral@22..23 "3"
                Semicolon@23..24 ";"
                Blankspace@24..25 " "
                BraceRight@25..26 "}""#]],
    );
}

#[test]
fn parse_if_statement() {
    check_statement(
        r#"if (0 > 3) { let x = 3; return x; }"#,
        expect![[r#"
            IfStatement@0..35
              If@0..2 "if"
              Blankspace@2..3 " "
              ParenthesisExpression@3..11
                ParenthesisLeft@3..4 "("
                InfixExpression@4..9
                  Literal@4..6
                    DecimalIntLiteral@4..5 "0"
                    Blankspace@5..6 " "
                  GreaterThan@6..7 ">"
                  Blankspace@7..8 " "
                  Literal@8..9
                    DecimalIntLiteral@8..9 "3"
                ParenthesisRight@9..10 ")"
                Blankspace@10..11 " "
              CompoundStatement@11..35
                BraceLeft@11..12 "{"
                Blankspace@12..13 " "
                VariableStatement@13..22
                  Let@13..16 "let"
                  Blankspace@16..17 " "
                  Binding@17..19
                    Name@17..19
                      Identifier@17..18 "x"
                      Blankspace@18..19 " "
                  Equal@19..20 "="
                  Blankspace@20..21 " "
                  Literal@21..22
                    DecimalIntLiteral@21..22 "3"
                Semicolon@22..23 ";"
                Blankspace@23..24 " "
                ReturnStatement@24..32
                  Return@24..30 "return"
                  Blankspace@30..31 " "
                  PathExpression@31..32
                    NameReference@31..32
                      Identifier@31..32 "x"
                Semicolon@32..33 ";"
                Blankspace@33..34 " "
                BraceRight@34..35 "}""#]],
    );
}

#[test]
fn parse_if_recover_paren() {
    check_statement(
        r#"if () {
          let x = 3;
        }"#,
        expect![[r#"
            IfStatement@0..38
              If@0..2 "if"
              Blankspace@2..3 " "
              ParenthesisExpression@3..6
                ParenthesisLeft@3..4 "("
                Error@4..4
                ParenthesisRight@4..5 ")"
                Blankspace@5..6 " "
              CompoundStatement@6..38
                BraceLeft@6..7 "{"
                Blankspace@7..18 "\n          "
                VariableStatement@18..27
                  Let@18..21 "let"
                  Blankspace@21..22 " "
                  Binding@22..24
                    Name@22..24
                      Identifier@22..23 "x"
                      Blankspace@23..24 " "
                  Equal@24..25 "="
                  Blankspace@25..26 " "
                  Literal@26..27
                    DecimalIntLiteral@26..27 "3"
                Semicolon@27..28 ";"
                Blankspace@28..37 "\n        "
                BraceRight@37..38 "}"

            error at 4..5: expected ParenthesisExpression, but found ParenthesisRight"#]],
    );
}

#[test]
fn parse_if_without_paren() {
    check_statement(
        r#"if true {
          let x = 3;
        }"#,
        expect![[r#"
            IfStatement@0..40
              If@0..2 "if"
              Blankspace@2..3 " "
              Literal@3..8
                True@3..7 "true"
                Blankspace@7..8 " "
              CompoundStatement@8..40
                BraceLeft@8..9 "{"
                Blankspace@9..20 "\n          "
                VariableStatement@20..29
                  Let@20..23 "let"
                  Blankspace@23..24 " "
                  Binding@24..26
                    Name@24..26
                      Identifier@24..25 "x"
                      Blankspace@25..26 " "
                  Equal@26..27 "="
                  Blankspace@27..28 " "
                  Literal@28..29
                    DecimalIntLiteral@28..29 "3"
                Semicolon@29..30 ";"
                Blankspace@30..39 "\n        "
                BraceRight@39..40 "}""#]],
    );
}

#[test]
fn parse_if_recover_empty() {
    check_statement(
        r#"if {
          let x = 3;
        }"#,
        expect![[r#"
            IfStatement@0..35
              If@0..2 "if"
              Blankspace@2..3 " "
              Error@3..3
              CompoundStatement@3..35
                BraceLeft@3..4 "{"
                Blankspace@4..15 "\n          "
                VariableStatement@15..24
                  Let@15..18 "let"
                  Blankspace@18..19 " "
                  Binding@19..21
                    Name@19..21
                      Identifier@19..20 "x"
                      Blankspace@20..21 " "
                  Equal@21..22 "="
                  Blankspace@22..23 " "
                  Literal@23..24
                    DecimalIntLiteral@23..24 "3"
                Semicolon@24..25 ";"
                Blankspace@25..34 "\n        "
                BraceRight@34..35 "}"

            error at 3..4: expected Bool, but found BraceLeft"#]],
    );
}

#[test]
fn parse_if_else() {
    check_statement(
        "if (0) {} else if (1) {} else if (2) {} else {}",
        expect![[r#"
            IfStatement@0..47
              If@0..2 "if"
              Blankspace@2..3 " "
              ParenthesisExpression@3..7
                ParenthesisLeft@3..4 "("
                Literal@4..5
                  DecimalIntLiteral@4..5 "0"
                ParenthesisRight@5..6 ")"
                Blankspace@6..7 " "
              CompoundStatement@7..10
                BraceLeft@7..8 "{"
                BraceRight@8..9 "}"
                Blankspace@9..10 " "
              ElseIfBlock@10..25
                Else@10..14 "else"
                Blankspace@14..15 " "
                If@15..17 "if"
                Blankspace@17..18 " "
                ParenthesisExpression@18..22
                  ParenthesisLeft@18..19 "("
                  Literal@19..20
                    DecimalIntLiteral@19..20 "1"
                  ParenthesisRight@20..21 ")"
                  Blankspace@21..22 " "
                CompoundStatement@22..25
                  BraceLeft@22..23 "{"
                  BraceRight@23..24 "}"
                  Blankspace@24..25 " "
              ElseIfBlock@25..40
                Else@25..29 "else"
                Blankspace@29..30 " "
                If@30..32 "if"
                Blankspace@32..33 " "
                ParenthesisExpression@33..37
                  ParenthesisLeft@33..34 "("
                  Literal@34..35
                    DecimalIntLiteral@34..35 "2"
                  ParenthesisRight@35..36 ")"
                  Blankspace@36..37 " "
                CompoundStatement@37..40
                  BraceLeft@37..38 "{"
                  BraceRight@38..39 "}"
                  Blankspace@39..40 " "
              ElseBlock@40..47
                Else@40..44 "else"
                Blankspace@44..45 " "
                CompoundStatement@45..47
                  BraceLeft@45..46 "{"
                  BraceRight@46..47 "}""#]],
    )
}

#[test]
fn parse_if_recovery_1() {
    check_statement(
        "if (false) {} else if {}",
        expect![[r#"
            IfStatement@0..24
              If@0..2 "if"
              Blankspace@2..3 " "
              ParenthesisExpression@3..11
                ParenthesisLeft@3..4 "("
                Literal@4..9
                  False@4..9 "false"
                ParenthesisRight@9..10 ")"
                Blankspace@10..11 " "
              CompoundStatement@11..14
                BraceLeft@11..12 "{"
                BraceRight@12..13 "}"
                Blankspace@13..14 " "
              ElseIfBlock@14..24
                Else@14..18 "else"
                Blankspace@18..19 " "
                If@19..21 "if"
                Blankspace@21..22 " "
                Error@22..22
                CompoundStatement@22..24
                  BraceLeft@22..23 "{"
                  BraceRight@23..24 "}"

            error at 22..23: expected Bool, but found BraceLeft"#]],
    );
}

#[test]
fn parse_for_statement() {
    check_statement(
        "for(let i = 0; i < 3; i = i + 1) {}",
        expect![[r#"
            ForStatement@0..35
              For@0..3 "for"
              ParenthesisLeft@3..4 "("
              ForInitializer@4..13
                VariableStatement@4..13
                  Let@4..7 "let"
                  Blankspace@7..8 " "
                  Binding@8..10
                    Name@8..10
                      Identifier@8..9 "i"
                      Blankspace@9..10 " "
                  Equal@10..11 "="
                  Blankspace@11..12 " "
                  Literal@12..13
                    DecimalIntLiteral@12..13 "0"
              Semicolon@13..14 ";"
              Blankspace@14..15 " "
              ForCondition@15..20
                InfixExpression@15..20
                  PathExpression@15..17
                    NameReference@15..17
                      Identifier@15..16 "i"
                      Blankspace@16..17 " "
                  LessThan@17..18 "<"
                  Blankspace@18..19 " "
                  Literal@19..20
                    DecimalIntLiteral@19..20 "3"
              Semicolon@20..21 ";"
              Blankspace@21..22 " "
              ForContinuingPart@22..31
                AssignmentStatement@22..31
                  PathExpression@22..24
                    NameReference@22..24
                      Identifier@22..23 "i"
                      Blankspace@23..24 " "
                  Equal@24..25 "="
                  Blankspace@25..26 " "
                  InfixExpression@26..31
                    PathExpression@26..28
                      NameReference@26..28
                        Identifier@26..27 "i"
                        Blankspace@27..28 " "
                    Plus@28..29 "+"
                    Blankspace@29..30 " "
                    Literal@30..31
                      DecimalIntLiteral@30..31 "1"
              ParenthesisRight@31..32 ")"
              Blankspace@32..33 " "
              CompoundStatement@33..35
                BraceLeft@33..34 "{"
                BraceRight@34..35 "}""#]],
    )
}

#[test]
fn parse_for_statement_comma() {
    check_statement(
        "for(let i = 0, i < 3, i = i + 1) {}",
        expect![[r#"
            ForStatement@0..35
              For@0..3 "for"
              ParenthesisLeft@3..4 "("
              ForInitializer@4..13
                VariableStatement@4..13
                  Let@4..7 "let"
                  Blankspace@7..8 " "
                  Binding@8..10
                    Name@8..10
                      Identifier@8..9 "i"
                      Blankspace@9..10 " "
                  Equal@10..11 "="
                  Blankspace@11..12 " "
                  Literal@12..13
                    DecimalIntLiteral@12..13 "0"
              Comma@13..14 ","
              Blankspace@14..15 " "
              ForCondition@15..20
                InfixExpression@15..20
                  PathExpression@15..17
                    NameReference@15..17
                      Identifier@15..16 "i"
                      Blankspace@16..17 " "
                  LessThan@17..18 "<"
                  Blankspace@18..19 " "
                  Literal@19..20
                    DecimalIntLiteral@19..20 "3"
              Comma@20..21 ","
              Blankspace@21..22 " "
              ForContinuingPart@22..31
                AssignmentStatement@22..31
                  PathExpression@22..24
                    NameReference@22..24
                      Identifier@22..23 "i"
                      Blankspace@23..24 " "
                  Equal@24..25 "="
                  Blankspace@25..26 " "
                  InfixExpression@26..31
                    PathExpression@26..28
                      NameReference@26..28
                        Identifier@26..27 "i"
                        Blankspace@27..28 " "
                    Plus@28..29 "+"
                    Blankspace@29..30 " "
                    Literal@30..31
                      DecimalIntLiteral@30..31 "1"
              ParenthesisRight@31..32 ")"
              Blankspace@32..33 " "
              CompoundStatement@33..35
                BraceLeft@33..34 "{"
                BraceRight@34..35 "}""#]],
    )
}

#[test]
fn for_statement_incomplete_1() {
    check_statement(
        "for(;;)",
        expect![[r#"
            ForStatement@0..7
              For@0..3 "for"
              ParenthesisLeft@3..4 "("
              Semicolon@4..5 ";"
              Semicolon@5..6 ";"
              ParenthesisRight@6..7 ")"
              CompoundStatement@7..7

            error at 6..7: expected BraceLeft
            error at 6..7: expected BraceRight"#]],
    );
}

#[test]
fn for_statement_incomplete_2() {
    check_statement(
        "for(i=0;;)",
        expect![[r#"
            ForStatement@0..10
              For@0..3 "for"
              ParenthesisLeft@3..4 "("
              ForInitializer@4..7
                AssignmentStatement@4..7
                  PathExpression@4..5
                    NameReference@4..5
                      Identifier@4..5 "i"
                  Equal@5..6 "="
                  Literal@6..7
                    DecimalIntLiteral@6..7 "0"
              Semicolon@7..8 ";"
              Semicolon@8..9 ";"
              ParenthesisRight@9..10 ")"
              CompoundStatement@10..10

            error at 9..10: expected BraceLeft
            error at 9..10: expected BraceRight"#]],
    );
}

#[test]
fn for_statement_incomplete_3() {
    check_statement(
        "for(;false;)",
        expect![[r#"
            ForStatement@0..12
              For@0..3 "for"
              ParenthesisLeft@3..4 "("
              Semicolon@4..5 ";"
              ForCondition@5..10
                Literal@5..10
                  False@5..10 "false"
              Semicolon@10..11 ";"
              ParenthesisRight@11..12 ")"
              CompoundStatement@12..12

            error at 11..12: expected BraceLeft
            error at 11..12: expected BraceRight"#]],
    );
}

#[test]
fn for_statement_incomplete_4() {
    check_statement(
        "for(;;a = 1)",
        expect![[r#"
            ForStatement@0..12
              For@0..3 "for"
              ParenthesisLeft@3..4 "("
              Semicolon@4..5 ";"
              Semicolon@5..6 ";"
              ForContinuingPart@6..11
                AssignmentStatement@6..11
                  PathExpression@6..8
                    NameReference@6..8
                      Identifier@6..7 "a"
                      Blankspace@7..8 " "
                  Equal@8..9 "="
                  Blankspace@9..10 " "
                  Literal@10..11
                    DecimalIntLiteral@10..11 "1"
              ParenthesisRight@11..12 ")"
              CompoundStatement@12..12

            error at 11..12: expected BraceLeft
            error at 11..12: expected BraceRight"#]],
    );
}

#[test]
fn for_statement_continue_break() {
    check_statement(
        "for(;;) { continue; break; continuing {}; }",
        expect![[r#"
            ForStatement@0..43
              For@0..3 "for"
              ParenthesisLeft@3..4 "("
              Semicolon@4..5 ";"
              Semicolon@5..6 ";"
              ParenthesisRight@6..7 ")"
              Blankspace@7..8 " "
              CompoundStatement@8..43
                BraceLeft@8..9 "{"
                Blankspace@9..10 " "
                Continue@10..18 "continue"
                Semicolon@18..19 ";"
                Blankspace@19..20 " "
                Break@20..25 "break"
                Semicolon@25..26 ";"
                Blankspace@26..27 " "
                ContinuingStatement@27..40
                  Continuing@27..37 "continuing"
                  Blankspace@37..38 " "
                  CompoundStatement@38..40
                    BraceLeft@38..39 "{"
                    BraceRight@39..40 "}"
                Semicolon@40..41 ";"
                Blankspace@41..42 " "
                BraceRight@42..43 "}""#]],
    );
}

#[test]
fn parse_statement_compound_empty() {
    check_statement(
        "{}",
        expect![[r#"
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
            CompoundStatement@0..24
              BraceLeft@0..1 "{"
              Blankspace@1..2 " "
              VariableStatement@2..11
                Let@2..5 "let"
                Blankspace@5..6 " "
                Binding@6..8
                  Name@6..8
                    Identifier@6..7 "x"
                    Blankspace@7..8 " "
                Equal@8..9 "="
                Blankspace@9..10 " "
                Literal@10..11
                  DecimalIntLiteral@10..11 "3"
              Semicolon@11..12 ";"
              Blankspace@12..13 " "
              ReturnStatement@13..21
                Return@13..19 "return"
                Blankspace@19..20 " "
                PathExpression@20..21
                  NameReference@20..21
                    Identifier@20..21 "x"
              Semicolon@21..22 ";"
              Blankspace@22..23 " "
              BraceRight@23..24 "}""#]],
    );
}

#[test]
fn parse_statement_assignment() {
    check_statement(
        "a = 3",
        expect![[r#"
            AssignmentStatement@0..5
              PathExpression@0..2
                NameReference@0..2
                  Identifier@0..1 "a"
                  Blankspace@1..2 " "
              Equal@2..3 "="
              Blankspace@3..4 " "
              Literal@4..5
                DecimalIntLiteral@4..5 "3""#]],
    );
}

#[test]
fn parse_statement_assignment_field() {
    check_statement(
        "a.b = a.c * 3",
        expect![[r#"
            AssignmentStatement@0..13
              FieldExpression@0..4
                PathExpression@0..1
                  NameReference@0..1
                    Identifier@0..1 "a"
                Period@1..2 "."
                NameReference@2..4
                  Identifier@2..3 "b"
                  Blankspace@3..4 " "
              Equal@4..5 "="
              Blankspace@5..6 " "
              InfixExpression@6..13
                FieldExpression@6..10
                  PathExpression@6..7
                    NameReference@6..7
                      Identifier@6..7 "a"
                  Period@7..8 "."
                  NameReference@8..10
                    Identifier@8..9 "c"
                    Blankspace@9..10 " "
                Star@10..11 "*"
                Blankspace@11..12 " "
                Literal@12..13
                  DecimalIntLiteral@12..13 "3""#]],
    );
}

#[test]
fn parse_statement_assignment_invalid() {
    check_statement(
        "1+2=3",
        expect![[r#"
        AssignmentStatement@0..5
          InfixExpression@0..3
            Literal@0..1
              DecimalIntLiteral@0..1 "1"
            Plus@1..2 "+"
            Literal@2..3
              DecimalIntLiteral@2..3 "2"
          Equal@3..4 "="
          Literal@4..5
            DecimalIntLiteral@4..5 "3""#]],
    );
}

#[test]
fn parse_statement_recover() {
    check_statement(
        "{ { let x = } { return 0 } }",
        expect![[r#"
            CompoundStatement@0..28
              BraceLeft@0..1 "{"
              Blankspace@1..2 " "
              CompoundStatement@2..14
                BraceLeft@2..3 "{"
                Blankspace@3..4 " "
                VariableStatement@4..12
                  Let@4..7 "let"
                  Blankspace@7..8 " "
                  Binding@8..10
                    Name@8..10
                      Identifier@8..9 "x"
                      Blankspace@9..10 " "
                  Equal@10..11 "="
                  Blankspace@11..12 " "
                BraceRight@12..13 "}"
                Blankspace@13..14 " "
              CompoundStatement@14..27
                BraceLeft@14..15 "{"
                Blankspace@15..16 " "
                ReturnStatement@16..25
                  Return@16..22 "return"
                  Blankspace@22..23 " "
                  Literal@23..25
                    DecimalIntLiteral@23..24 "0"
                    Blankspace@24..25 " "
                BraceRight@25..26 "}"
                Blankspace@26..27 " "
              BraceRight@27..28 "}""#]],
    );
}

#[test]
fn parse_compound_assignment_statement() {
    check_statement(
        "a += 3",
        expect![[r#"
            CompoundAssignmentStatement@0..6
              PathExpression@0..2
                NameReference@0..2
                  Identifier@0..1 "a"
                  Blankspace@1..2 " "
              PlusEqual@2..4 "+="
              Blankspace@4..5 " "
              Literal@5..6
                DecimalIntLiteral@5..6 "3""#]],
    );
}

#[test]
fn parse_compound_assignment_statement_expression() {
    check_statement(
        "*func() += foo()",
        expect![[r#"
            CompoundAssignmentStatement@0..16
              PrefixExpression@0..8
                Star@0..1 "*"
                FunctionCall@1..8
                  NameReference@1..5
                    Identifier@1..5 "func"
                  FunctionParameterList@5..8
                    ParenthesisLeft@5..6 "("
                    ParenthesisRight@6..7 ")"
                    Blankspace@7..8 " "
              PlusEqual@8..10 "+="
              Blankspace@10..11 " "
              FunctionCall@11..16
                NameReference@11..14
                  Identifier@11..14 "foo"
                FunctionParameterList@14..16
                  ParenthesisLeft@14..15 "("
                  ParenthesisRight@15..16 ")""#]],
    );
}

#[test]
fn parse_var_without_initializer() {
    check_statement(
        "var x: u32;",
        expect![[r#"
            VariableStatement@0..10
              Var@0..3 "var"
              Blankspace@3..4 " "
              Binding@4..5
                Name@4..5
                  Identifier@4..5 "x"
              Colon@5..6 ":"
              Blankspace@6..7 " "
              Uint32@7..10
                Uint32@7..10 "u32""#]],
    )
}

#[test]
fn parse_var_with_initializer() {
    check_statement(
        "var<function> x: u32;",
        expect![[r#"
            VariableStatement@0..20
              Var@0..3 "var"
              VariableQualifier@3..14
                LessThan@3..4 "<"
                FunctionClass@4..12 "function"
                GreaterThan@12..13 ">"
                Blankspace@13..14 " "
              Binding@14..15
                Name@14..15
                  Identifier@14..15 "x"
              Colon@15..16 ":"
              Blankspace@16..17 " "
              Uint32@17..20
                Uint32@17..20 "u32""#]],
    )
}

#[test]
fn attribute_list_modern() {
    check_attribute_list(
        "@location(0) @interpolate(flat) @attr(1, 2, 0.0, ident)",
        expect![[r#"
            AttributeList@0..55
              AttributeOperator@0..1 "@"
              Attribute@1..13
                Identifier@1..9 "location"
                AttributeParameters@9..13
                  ParenthesisLeft@9..10 "("
                  Literal@10..11
                    DecimalIntLiteral@10..11 "0"
                  ParenthesisRight@11..12 ")"
                  Blankspace@12..13 " "
              AttributeOperator@13..14 "@"
              Attribute@14..32
                Identifier@14..25 "interpolate"
                AttributeParameters@25..32
                  ParenthesisLeft@25..26 "("
                  Identifier@26..30 "flat"
                  ParenthesisRight@30..31 ")"
                  Blankspace@31..32 " "
              AttributeOperator@32..33 "@"
              Attribute@33..55
                Identifier@33..37 "attr"
                AttributeParameters@37..55
                  ParenthesisLeft@37..38 "("
                  Literal@38..39
                    DecimalIntLiteral@38..39 "1"
                  Comma@39..40 ","
                  Blankspace@40..41 " "
                  Literal@41..42
                    DecimalIntLiteral@41..42 "2"
                  Comma@42..43 ","
                  Blankspace@43..44 " "
                  Literal@44..47
                    DecimalFloatLiteral@44..47 "0.0"
                  Comma@47..48 ","
                  Blankspace@48..49 " "
                  Identifier@49..54 "ident"
                  ParenthesisRight@54..55 ")""#]],
    );
}

#[test]
fn fn_recover_incomplete_param() {
    check(
        "fn main(p) {}",
        expect![[r#"
            SourceFile@0..13
              Function@0..13
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                ParameterList@7..11
                  ParenthesisLeft@7..8 "("
                  Parameter@8..9
                    VariableIdentDeclaration@8..9
                      Binding@8..9
                        Name@8..9
                          Identifier@8..9 "p"
                      Error@9..9
                  ParenthesisRight@9..10 ")"
                  Blankspace@10..11 " "
                CompoundStatement@11..13
                  BraceLeft@11..12 "{"
                  BraceRight@12..13 "}"

            error at 9..10: expected Colon, but found ParenthesisRight"#]],
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
              Function@0..42
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                ParameterList@7..10
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                  Blankspace@9..10 " "
                CompoundStatement@10..42
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  VariableStatement@24..41
                    Let@24..27 "let"
                    Blankspace@27..28 " "
                    Binding@28..30
                      Name@28..30
                        Identifier@28..29 "x"
                        Blankspace@29..30 " "
                    Error@30..41
                      Identifier@30..32 "be"
                      Blankspace@32..41 "\n        "
                  BraceRight@41..42 "}"

            error at 30..32: expected Colon, but found Identifier"#]],
    )
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
              Function@0..59
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                ParameterList@7..10
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                  Blankspace@9..10 " "
                CompoundStatement@10..59
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  VariableStatement@24..40
                    Let@24..27 "let"
                    Blankspace@27..40 "\n            "
                    Error@40..40
                  ReturnStatement@40..48
                    Return@40..46 "return"
                    Blankspace@46..47 " "
                    Literal@47..48
                      DecimalIntLiteral@47..48 "0"
                  Semicolon@48..49 ";"
                  Blankspace@49..58 "\n        "
                  BraceRight@58..59 "}"

            error at 40..46: expected Binding, but found Return"#]],
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
              Function@0..61
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                ParameterList@7..10
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                  Blankspace@9..10 " "
                CompoundStatement@10..61
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  VariableStatement@24..42
                    Let@24..27 "let"
                    Blankspace@27..28 " "
                    Binding@28..42
                      Name@28..42
                        Identifier@28..29 "x"
                        Blankspace@29..42 "\n            "
                    Error@42..42
                  ReturnStatement@42..50
                    Return@42..48 "return"
                    Blankspace@48..49 " "
                    Literal@49..50
                      DecimalIntLiteral@49..50 "0"
                  Semicolon@50..51 ";"
                  Blankspace@51..60 "\n        "
                  BraceRight@60..61 "}"

            error at 42..48: expected Binding, but found Return"#]],
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
              Function@0..63
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                ParameterList@7..10
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                  Blankspace@9..10 " "
                CompoundStatement@10..63
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  VariableStatement@24..44
                    Let@24..27 "let"
                    Blankspace@27..28 " "
                    Binding@28..30
                      Name@28..30
                        Identifier@28..29 "x"
                        Blankspace@29..30 " "
                    Equal@30..31 "="
                    Blankspace@31..44 "\n            "
                  ReturnStatement@44..52
                    Return@44..50 "return"
                    Blankspace@50..51 " "
                    Literal@51..52
                      DecimalIntLiteral@51..52 "0"
                  Semicolon@52..53 ";"
                  Blankspace@53..62 "\n        "
                  BraceRight@62..63 "}""#]],
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
              Function@0..39
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                ParameterList@7..10
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                  Blankspace@9..10 " "
                CompoundStatement@10..39
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  VariableStatement@24..38
                    Let@24..27 "let"
                    Blankspace@27..28 " "
                    Binding@28..38
                      Name@28..38
                        Identifier@28..29 "x"
                        Blankspace@29..38 "\n        "
                    Error@38..38
                  BraceRight@38..39 "}"

            error at 38..39: expected Binding, but found BraceRight"#]],
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
              Function@0..41
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                ParameterList@7..10
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                  Blankspace@9..10 " "
                CompoundStatement@10..41
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  VariableStatement@24..40
                    Let@24..27 "let"
                    Blankspace@27..28 " "
                    Binding@28..30
                      Name@28..30
                        Identifier@28..29 "x"
                        Blankspace@29..30 " "
                    Equal@30..31 "="
                    Blankspace@31..40 "\n        "
                  BraceRight@40..41 "}""#]],
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
              Function@0..37
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..7
                  Identifier@3..7 "main"
                ParameterList@7..10
                  ParenthesisLeft@7..8 "("
                  ParenthesisRight@8..9 ")"
                  Blankspace@9..10 " "
                CompoundStatement@10..37
                  BraceLeft@10..11 "{"
                  Blankspace@11..24 "\n            "
                  VariableStatement@24..36
                    Let@24..27 "let"
                    Blankspace@27..36 "\n        "
                    Error@36..36
                  BraceRight@36..37 "}"

            error at 36..37: expected Binding, but found BraceRight"#]],
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
        r#"
struct UBO {
  camera_position: vec3f,
  _pad: u32
  time: f32,
};
"#,
        expect![[r#"
            SourceFile@0..68
              Blankspace@0..1 "\n"
              StructDeclaration@1..68
                Struct@1..7 "struct"
                Blankspace@7..8 " "
                Name@8..12
                  Identifier@8..11 "UBO"
                  Blankspace@11..12 " "
                StructDeclBody@12..66
                  BraceLeft@12..13 "{"
                  Blankspace@13..16 "\n  "
                  StructDeclarationField@16..42
                    VariableIdentDeclaration@16..38
                      Binding@16..31
                        Name@16..31
                          Identifier@16..31 "camera_position"
                      Colon@31..32 ":"
                      Blankspace@32..33 " "
                      PathType@33..38
                        NameReference@33..38
                          Identifier@33..38 "vec3f"
                    Comma@38..39 ","
                    Blankspace@39..42 "\n  "
                  StructDeclarationField@42..54
                    VariableIdentDeclaration@42..54
                      Binding@42..46
                        Name@42..46
                          Identifier@42..46 "_pad"
                      Colon@46..47 ":"
                      Blankspace@47..48 " "
                      Uint32@48..54
                        Uint32@48..51 "u32"
                        Blankspace@51..54 "\n  "
                  StructDeclarationField@54..65
                    VariableIdentDeclaration@54..63
                      Binding@54..58
                        Name@54..58
                          Identifier@54..58 "time"
                      Colon@58..59 ":"
                      Blankspace@59..60 " "
                      Float32@60..63
                        Float32@60..63 "f32"
                    Comma@63..64 ","
                    Blankspace@64..65 "\n"
                  BraceRight@65..66 "}"
                Semicolon@66..67 ";"
                Blankspace@67..68 "\n""#]],
    );
}

#[test]
fn struct_decl_semi() {
    check(
        r#"
struct Test {
    a: f32;
    b: vec3<f32>;
}
"#,
        expect![[r#"
            SourceFile@0..47
              Blankspace@0..1 "\n"
              StructDeclaration@1..47
                Struct@1..7 "struct"
                Blankspace@7..8 " "
                Name@8..13
                  Identifier@8..12 "Test"
                  Blankspace@12..13 " "
                StructDeclBody@13..47
                  BraceLeft@13..14 "{"
                  Blankspace@14..19 "\n    "
                  StructDeclarationField@19..31
                    VariableIdentDeclaration@19..25
                      Binding@19..20
                        Name@19..20
                          Identifier@19..20 "a"
                      Colon@20..21 ":"
                      Blankspace@21..22 " "
                      Float32@22..25
                        Float32@22..25 "f32"
                    Semicolon@25..26 ";"
                    Blankspace@26..31 "\n    "
                  StructDeclarationField@31..45
                    VariableIdentDeclaration@31..43
                      Binding@31..32
                        Name@31..32
                          Identifier@31..32 "b"
                      Colon@32..33 ":"
                      Blankspace@33..34 " "
                      Vec3@34..43
                        Vec3@34..38 "vec3"
                        GenericArgumentList@38..43
                          LessThan@38..39 "<"
                          Float32@39..42
                            Float32@39..42 "f32"
                          GreaterThan@42..43 ">"
                    Semicolon@43..44 ";"
                    Blankspace@44..45 "\n"
                  BraceRight@45..46 "}"
                  Blankspace@46..47 "\n""#]],
    );
}

#[test]
fn struct_decl() {
    check(
        r#"
struct Test {
    a: f32,
    b: vec3<f32>,
}
"#,
        expect![[r#"
            SourceFile@0..47
              Blankspace@0..1 "\n"
              StructDeclaration@1..47
                Struct@1..7 "struct"
                Blankspace@7..8 " "
                Name@8..13
                  Identifier@8..12 "Test"
                  Blankspace@12..13 " "
                StructDeclBody@13..47
                  BraceLeft@13..14 "{"
                  Blankspace@14..19 "\n    "
                  StructDeclarationField@19..31
                    VariableIdentDeclaration@19..25
                      Binding@19..20
                        Name@19..20
                          Identifier@19..20 "a"
                      Colon@20..21 ":"
                      Blankspace@21..22 " "
                      Float32@22..25
                        Float32@22..25 "f32"
                    Comma@25..26 ","
                    Blankspace@26..31 "\n    "
                  StructDeclarationField@31..45
                    VariableIdentDeclaration@31..43
                      Binding@31..32
                        Name@31..32
                          Identifier@31..32 "b"
                      Colon@32..33 ":"
                      Blankspace@33..34 " "
                      Vec3@34..43
                        Vec3@34..38 "vec3"
                        GenericArgumentList@38..43
                          LessThan@38..39 "<"
                          Float32@39..42
                            Float32@39..42 "f32"
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
        r#"
struct
fn test()
"#,
        expect![[r#"
            SourceFile@0..18
              Blankspace@0..1 "\n"
              Struct@1..8
                Struct@1..7 "struct"
                Blankspace@7..8 "\n"
                Error@8..8
              Function@8..18
                Fn@8..10 "fn"
                Blankspace@10..11 " "
                Name@11..15
                  Identifier@11..15 "test"
                ParameterList@15..18
                  ParenthesisLeft@15..16 "("
                  ParenthesisRight@16..17 ")"
                  Blankspace@17..18 "\n"

            error at 8..10: expected BraceLeft, but found Fn
            error at 17..18: expected Arrow or BraceLeft"#]],
    );
}

#[test]
fn struct_recover_2() {
    check(
        r#"
struct test
fn test()
"#,
        expect![[r#"
            SourceFile@0..23
              Blankspace@0..1 "\n"
              Struct@1..13
                Struct@1..7 "struct"
                Blankspace@7..8 " "
                Name@8..13
                  Identifier@8..12 "test"
                  Blankspace@12..13 "\n"
                Error@13..13
              Function@13..23
                Fn@13..15 "fn"
                Blankspace@15..16 " "
                Name@16..20
                  Identifier@16..20 "test"
                ParameterList@20..23
                  ParenthesisLeft@20..21 "("
                  ParenthesisRight@21..22 ")"
                  Blankspace@22..23 "\n"

            error at 13..15: expected BraceLeft, but found Fn
            error at 22..23: expected Arrow or BraceLeft"#]],
    );
}

#[test]
fn struct_recover_3() {
    check(
        r#"
struct test {}

fn test()
};
"#,
        expect![[r#"
            SourceFile@0..30
              Blankspace@0..1 "\n"
              StructDeclaration@1..17
                Struct@1..7 "struct"
                Blankspace@7..8 " "
                Name@8..13
                  Identifier@8..12 "test"
                  Blankspace@12..13 " "
                StructDeclBody@13..17
                  BraceLeft@13..14 "{"
                  BraceRight@14..15 "}"
                  Blankspace@15..17 "\n\n"
              Function@17..28
                Fn@17..19 "fn"
                Blankspace@19..20 " "
                Name@20..24
                  Identifier@20..24 "test"
                ParameterList@24..27
                  ParenthesisLeft@24..25 "("
                  ParenthesisRight@25..26 ")"
                  Blankspace@26..27 "\n"
                Error@27..28
                  BraceRight@27..28 "}"
              Error@28..30
                Error@28..30
                  Semicolon@28..29 ";"
                  Blankspace@29..30 "\n"

            error at 27..28: expected Arrow or BraceLeft, but found BraceRight
            error at 28..29: expected Fn, Struct, Var, Let, Constant, Alias, or Override, but found Semicolon"#]],
    );
}

#[test]
fn global_variable_decl_init() {
    check(
        "var flags = 0;",
        expect![[r#"
            SourceFile@0..14
              GlobalVariableDeclaration@0..14
                Var@0..3 "var"
                Blankspace@3..4 " "
                Binding@4..10
                  Name@4..10
                    Identifier@4..9 "flags"
                    Blankspace@9..10 " "
                Equal@10..11 "="
                Blankspace@11..12 " "
                Literal@12..13
                  DecimalIntLiteral@12..13 "0"
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
                Constant@0..5 "const"
                Blankspace@5..6 " "
                Binding@6..15
                  Name@6..15
                    Identifier@6..14 "constant"
                    Blankspace@14..15 " "
                Equal@15..16 "="
                Blankspace@16..17 " "
                Literal@17..18
                  DecimalIntLiteral@17..18 "0"
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
                Name@6..12
                  Identifier@6..11 "float"
                  Blankspace@11..12 " "
                Equal@12..13 "="
                Blankspace@13..14 " "
                Float32@14..17
                  Float32@14..17 "f32"
                Semicolon@17..18 ";""#]],
    );
}

#[test]
fn type_alias_decl_old() {
    check(
        "type float = f32;",
        expect![[r#"
            SourceFile@0..17
              TypeAliasDeclaration@0..17
                Type@0..4 "type"
                Blankspace@4..5 " "
                Name@5..11
                  Identifier@5..10 "float"
                  Blankspace@10..11 " "
                Equal@11..12 "="
                Blankspace@12..13 " "
                Float32@13..16
                  Float32@13..16 "f32"
                Semicolon@16..17 ";""#]],
    );
}

#[test]
fn type_alias_decl_recover() {
    check(
        "type float = f32\ntype other = u32;",
        expect![[r#"
            SourceFile@0..34
              TypeAliasDeclaration@0..17
                Type@0..4 "type"
                Blankspace@4..5 " "
                Name@5..11
                  Identifier@5..10 "float"
                  Blankspace@10..11 " "
                Equal@11..12 "="
                Blankspace@12..13 " "
                Float32@13..17
                  Float32@13..16 "f32"
                  Blankspace@16..17 "\n"
                Error@17..17
              TypeAliasDeclaration@17..34
                Type@17..21 "type"
                Blankspace@21..22 " "
                Name@22..28
                  Identifier@22..27 "other"
                  Blankspace@27..28 " "
                Equal@28..29 "="
                Blankspace@29..30 " "
                Uint32@30..33
                  Uint32@30..33 "u32"
                Semicolon@33..34 ";"

            error at 17..21: expected LessThan or Semicolon, but found Type"#]],
    );
}

#[test]
fn parse_statement_expression() {
    check_statement(
        "test(args);",
        expect![[r#"
            ExpressionStatement@0..10
              FunctionCall@0..10
                NameReference@0..4
                  Identifier@0..4 "test"
                FunctionParameterList@4..10
                  ParenthesisLeft@4..5 "("
                  PathExpression@5..9
                    NameReference@5..9
                      Identifier@5..9 "args"
                  ParenthesisRight@9..10 ")""#]],
    );
}

#[test]
fn loop_statement() {
    check_statement(
        "loop {}",
        expect![[r#"
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
        ReturnStatement@0..6
          Return@0..6 "return""#]],
    );
}

#[test]
fn empty_return_statement_no_semi() {
    check_statement(
        "{ let x = 3; return x } ",
        expect![[r#"
            CompoundStatement@0..24
              BraceLeft@0..1 "{"
              Blankspace@1..2 " "
              VariableStatement@2..11
                Let@2..5 "let"
                Blankspace@5..6 " "
                Binding@6..8
                  Name@6..8
                    Identifier@6..7 "x"
                    Blankspace@7..8 " "
                Equal@8..9 "="
                Blankspace@9..10 " "
                Literal@10..11
                  DecimalIntLiteral@10..11 "3"
              Semicolon@11..12 ";"
              Blankspace@12..13 " "
              ReturnStatement@13..22
                Return@13..19 "return"
                Blankspace@19..20 " "
                PathExpression@20..22
                  NameReference@20..22
                    Identifier@20..21 "x"
                    Blankspace@21..22 " "
              BraceRight@22..23 "}"
              Blankspace@23..24 " ""#]],
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
        r#"
switch i {
  case 0: { fallthrough; }
  case 1, 2: { return 42; }
  default: { }
}
        "#,
        expect![[r#"
            SwitchStatement@0..92
              Blankspace@0..1 "\n"
              Switch@1..7 "switch"
              Blankspace@7..8 " "
              PathExpression@8..10
                NameReference@8..10
                  Identifier@8..9 "i"
                  Blankspace@9..10 " "
              SwitchBlock@10..92
                BraceLeft@10..11 "{"
                Blankspace@11..14 "\n  "
                SwitchBodyCase@14..41
                  Case@14..18 "case"
                  Blankspace@18..19 " "
                  SwitchCaseSelectors@19..20
                    Literal@19..20
                      DecimalIntLiteral@19..20 "0"
                  Colon@20..21 ":"
                  Blankspace@21..22 " "
                  CompoundStatement@22..41
                    BraceLeft@22..23 "{"
                    Blankspace@23..24 " "
                    Fallthrough@24..35 "fallthrough"
                    Semicolon@35..36 ";"
                    Blankspace@36..37 " "
                    BraceRight@37..38 "}"
                    Blankspace@38..41 "\n  "
                SwitchBodyCase@41..69
                  Case@41..45 "case"
                  Blankspace@45..46 " "
                  SwitchCaseSelectors@46..50
                    Literal@46..47
                      DecimalIntLiteral@46..47 "1"
                    Comma@47..48 ","
                    Blankspace@48..49 " "
                    Literal@49..50
                      DecimalIntLiteral@49..50 "2"
                  Colon@50..51 ":"
                  Blankspace@51..52 " "
                  CompoundStatement@52..69
                    BraceLeft@52..53 "{"
                    Blankspace@53..54 " "
                    ReturnStatement@54..63
                      Return@54..60 "return"
                      Blankspace@60..61 " "
                      Literal@61..63
                        DecimalIntLiteral@61..63 "42"
                    Semicolon@63..64 ";"
                    Blankspace@64..65 " "
                    BraceRight@65..66 "}"
                    Blankspace@66..69 "\n  "
                SwitchBodyDefault@69..82
                  Default@69..76 "default"
                  Colon@76..77 ":"
                  Blankspace@77..78 " "
                  CompoundStatement@78..82
                    BraceLeft@78..79 "{"
                    Blankspace@79..80 " "
                    BraceRight@80..81 "}"
                    Blankspace@81..82 "\n"
                BraceRight@82..83 "}"
                Blankspace@83..92 "\n        ""#]],
    );
}

#[test]
fn parse_switch_statement_recover_1() {
    check_statement(
        r#"
switch i {
  case
}
        "#,
        expect![[r#"
            SwitchStatement@0..29
              Blankspace@0..1 "\n"
              Switch@1..7 "switch"
              Blankspace@7..8 " "
              PathExpression@8..10
                NameReference@8..10
                  Identifier@8..9 "i"
                  Blankspace@9..10 " "
              SwitchBlock@10..29
                BraceLeft@10..11 "{"
                Blankspace@11..14 "\n  "
                SwitchBodyCase@14..19
                  Case@14..18 "case"
                  Blankspace@18..19 "\n"
                  SwitchCaseSelectors@19..19
                BraceRight@19..20 "}"
                Blankspace@20..29 "\n        ""#]],
    );
}

#[test]
fn parse_switch_statement_recover_2() {
    check_statement(
        r#"
switch i {
  case 1
}
        "#,
        expect![[r#"
            SwitchStatement@0..31
              Blankspace@0..1 "\n"
              Switch@1..7 "switch"
              Blankspace@7..8 " "
              PathExpression@8..10
                NameReference@8..10
                  Identifier@8..9 "i"
                  Blankspace@9..10 " "
              SwitchBlock@10..31
                BraceLeft@10..11 "{"
                Blankspace@11..14 "\n  "
                SwitchBodyCase@14..21
                  Case@14..18 "case"
                  Blankspace@18..19 " "
                  SwitchCaseSelectors@19..21
                    Literal@19..21
                      DecimalIntLiteral@19..20 "1"
                      Blankspace@20..21 "\n"
                BraceRight@21..22 "}"
                Blankspace@22..31 "\n        ""#]],
    );
}

#[test]
fn parse_switch_statement_recover_3() {
    check_statement(
        r#"
{
switch i {
  case 1:
}

let x = 3;
}
        "#,
        expect![[r#"
            CompoundStatement@0..48
              Blankspace@0..1 "\n"
              BraceLeft@1..2 "{"
              Blankspace@2..3 "\n"
              SwitchStatement@3..27
                Switch@3..9 "switch"
                Blankspace@9..10 " "
                PathExpression@10..12
                  NameReference@10..12
                    Identifier@10..11 "i"
                    Blankspace@11..12 " "
                SwitchBlock@12..27
                  BraceLeft@12..13 "{"
                  Blankspace@13..16 "\n  "
                  SwitchBodyCase@16..24
                    Case@16..20 "case"
                    Blankspace@20..21 " "
                    SwitchCaseSelectors@21..22
                      Literal@21..22
                        DecimalIntLiteral@21..22 "1"
                    Colon@22..23 ":"
                    Blankspace@23..24 "\n"
                  BraceRight@24..25 "}"
                  Blankspace@25..27 "\n\n"
              VariableStatement@27..36
                Let@27..30 "let"
                Blankspace@30..31 " "
                Binding@31..33
                  Name@31..33
                    Identifier@31..32 "x"
                    Blankspace@32..33 " "
                Equal@33..34 "="
                Blankspace@34..35 " "
                Literal@35..36
                  DecimalIntLiteral@35..36 "3"
              Semicolon@36..37 ";"
              Blankspace@37..38 "\n"
              BraceRight@38..39 "}"
              Blankspace@39..48 "\n        ""#]],
    );
}

#[test]
fn parse_switch_statement_recover_4() {
    check_statement(
        r#"
{
switch i {
  case 1, 2,
}
let x = 3;
}
        "#,
        expect![[r#"
            CompoundStatement@0..50
              Blankspace@0..1 "\n"
              BraceLeft@1..2 "{"
              Blankspace@2..3 "\n"
              SwitchStatement@3..29
                Switch@3..9 "switch"
                Blankspace@9..10 " "
                PathExpression@10..12
                  NameReference@10..12
                    Identifier@10..11 "i"
                    Blankspace@11..12 " "
                SwitchBlock@12..29
                  BraceLeft@12..13 "{"
                  Blankspace@13..16 "\n  "
                  SwitchBodyCase@16..27
                    Case@16..20 "case"
                    Blankspace@20..21 " "
                    SwitchCaseSelectors@21..27
                      Literal@21..22
                        DecimalIntLiteral@21..22 "1"
                      Comma@22..23 ","
                      Blankspace@23..24 " "
                      Literal@24..25
                        DecimalIntLiteral@24..25 "2"
                      Comma@25..26 ","
                      Blankspace@26..27 "\n"
                  BraceRight@27..28 "}"
                  Blankspace@28..29 "\n"
              VariableStatement@29..38
                Let@29..32 "let"
                Blankspace@32..33 " "
                Binding@33..35
                  Name@33..35
                    Identifier@33..34 "x"
                    Blankspace@34..35 " "
                Equal@35..36 "="
                Blankspace@36..37 " "
                Literal@37..38
                  DecimalIntLiteral@37..38 "3"
              Semicolon@38..39 ";"
              Blankspace@39..40 "\n"
              BraceRight@40..41 "}"
              Blankspace@41..50 "\n        ""#]],
    );
}
