use expect_test::expect;

use crate::tests::{check, check_with_edition};

#[test]
fn public_visibility() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "public const x:u32=0;",
        expect![[r#"
            SourceFile@0..21
              ConstantDeclaration@0..21
                Visibility@0..6
                  Public@0..6 "public"
                Blankspace@6..7 " "
                Const@7..12 "const"
                Blankspace@12..13 " "
                Name@13..14
                  Identifier@13..14 "x"
                Colon@14..15 ":"
                TypeSpecifier@15..18
                  Path@15..18
                    Identifier@15..18 "u32"
                Equal@18..19 "="
                Literal@19..20
                  IntLiteral@19..20 "0"
                Semicolon@20..21 ";""#]],
    );
}

#[test]
fn reject_public_visibility_in_wgsl() {
    check_with_edition(
        edition::Edition::Wgsl,
        "public const x:u32=0;",
        expect![[r#"
            SourceFile@0..21
              Error@0..6
                Reserved@0..6 "public"
              Blankspace@6..7 " "
              ConstantDeclaration@7..21
                Const@7..12 "const"
                Blankspace@12..13 " "
                Name@13..14
                  Identifier@13..14 "x"
                Colon@14..15 ":"
                TypeSpecifier@15..18
                  Path@15..18
                    Identifier@15..18 "u32"
                Equal@18..19 "="
                Literal@19..20
                  IntLiteral@19..20 "0"
                Semicolon@20..21 ";"

            error at 0..6: 'public' is a reserved word in WGSL
            error at 0..6: switch to WESL to use `public`
            error at 0..6: invalid syntax, expected one of: 'alias', '@', '{', 'const', 'const_assert', 'diagnostic', <end of file>, 'enable', 'fn', 'import', 'let', 'override', 'private', 'public', 'requires', ';', 'struct', 'var'"#]],
    );
}

#[test]
fn reject_private_visibility_in_wgsl() {
    // in WGSL mode, the lexer treats 'private' as a normal identifier
    // so we won't throw the helpful switch to WESL to use `private` message
    check_with_edition(
        edition::Edition::Wgsl,
        "private const x:u32=0;",
        expect![[r#"
            SourceFile@0..22
              Error@0..7
                Identifier@0..7 "private"
              Blankspace@7..8 " "
              ConstantDeclaration@8..22
                Const@8..13 "const"
                Blankspace@13..14 " "
                Name@14..15
                  Identifier@14..15 "x"
                Colon@15..16 ":"
                TypeSpecifier@16..19
                  Path@16..19
                    Identifier@16..19 "u32"
                Equal@19..20 "="
                Literal@20..21
                  IntLiteral@20..21 "0"
                Semicolon@21..22 ";"

            error at 0..7: invalid syntax, expected one of: 'alias', '@', '{', 'const', 'const_assert', 'diagnostic', <end of file>, 'enable', 'fn', 'import', 'let', 'override', 'private', 'public', 'requires', ';', 'struct', 'var'"#]],
    );
}

#[test]
fn reject_public_visibility_before_const_assert() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "public const_assert true;",
        expect![[r#"
            SourceFile@0..25
              ConstantDeclaration@0..6
                Visibility@0..6
                  Public@0..6 "public"
              Blankspace@6..7 " "
              AssertStatement@7..25
                ConstantAssert@7..19 "const_assert"
                Blankspace@19..20 " "
                Literal@20..24
                  True@20..24 "true"
                Semicolon@24..25 ";"

            error at 7..19: invalid syntax, expected one of: 'alias', 'const', 'fn', 'let', 'override', 'struct', 'var'"#]],
    );
}

#[test]
fn private_visibility() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "private fn x(){}",
        expect![[r#"
            SourceFile@0..16
              FunctionDeclaration@0..16
                Visibility@0..7
                  Private@0..7 "private"
                Blankspace@7..8 " "
                Fn@8..10 "fn"
                Blankspace@10..11 " "
                Name@11..12
                  Identifier@11..12 "x"
                FunctionParameters@12..14
                  ParenthesisLeft@12..13 "("
                  ParenthesisRight@13..14 ")"
                CompoundStatement@14..16
                  BraceLeft@14..15 "{"
                  BraceRight@15..16 "}""#]],
    );
}

#[test]
fn multiple_items_with_visibility() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "
        public fn x(){}
        private struct Foo{ a: u32 }
        private var<private> x:u32;
        ",
        expect![[r#"
            SourceFile@0..106
              Blankspace@0..9 "\n        "
              FunctionDeclaration@9..24
                Visibility@9..15
                  Public@9..15 "public"
                Blankspace@15..16 " "
                Fn@16..18 "fn"
                Blankspace@18..19 " "
                Name@19..20
                  Identifier@19..20 "x"
                FunctionParameters@20..22
                  ParenthesisLeft@20..21 "("
                  ParenthesisRight@21..22 ")"
                CompoundStatement@22..24
                  BraceLeft@22..23 "{"
                  BraceRight@23..24 "}"
              Blankspace@24..33 "\n        "
              StructDeclaration@33..61
                Visibility@33..40
                  Private@33..40 "private"
                Blankspace@40..41 " "
                Struct@41..47 "struct"
                Blankspace@47..48 " "
                Name@48..51
                  Identifier@48..51 "Foo"
                StructBody@51..61
                  BraceLeft@51..52 "{"
                  Blankspace@52..53 " "
                  StructMember@53..59
                    Name@53..54
                      Identifier@53..54 "a"
                    Colon@54..55 ":"
                    Blankspace@55..56 " "
                    TypeSpecifier@56..59
                      Path@56..59
                        Identifier@56..59 "u32"
                  Blankspace@59..60 " "
                  BraceRight@60..61 "}"
              Blankspace@61..70 "\n        "
              VariableDeclaration@70..97
                Visibility@70..77
                  Private@70..77 "private"
                Blankspace@77..78 " "
                Var@78..81 "var"
                TemplateList@81..90
                  TemplateStart@81..82 "<"
                  IdentExpression@82..89
                    Path@82..89
                      Identifier@82..89 "private"
                  TemplateEnd@89..90 ">"
                Blankspace@90..91 " "
                Name@91..92
                  Identifier@91..92 "x"
                Colon@92..93 ":"
                TypeSpecifier@93..96
                  Path@93..96
                    Identifier@93..96 "u32"
                Semicolon@96..97 ";"
              Blankspace@97..106 "\n        ""#]],
    );
}

#[test]
fn reject_public_and_private_visibility() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "public private fn x(){}",
        expect![[r#"
            SourceFile@0..23
              ConstantDeclaration@0..6
                Visibility@0..6
                  Public@0..6 "public"
              Blankspace@6..7 " "
              FunctionDeclaration@7..23
                Visibility@7..14
                  Private@7..14 "private"
                Blankspace@14..15 " "
                Fn@15..17 "fn"
                Blankspace@17..18 " "
                Name@18..19
                  Identifier@18..19 "x"
                FunctionParameters@19..21
                  ParenthesisLeft@19..20 "("
                  ParenthesisRight@20..21 ")"
                CompoundStatement@21..23
                  BraceLeft@21..22 "{"
                  BraceRight@22..23 "}"

            error at 7..14: invalid syntax, expected one of: 'alias', 'const', 'fn', 'let', 'override', 'struct', 'var'"#]],
    );
}

#[test]
fn private_is_valid_identifier() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "public const private = private() + mylib::private();",
        expect![[r#"
            SourceFile@0..52
              ConstantDeclaration@0..52
                Visibility@0..6
                  Public@0..6 "public"
                Blankspace@6..7 " "
                Const@7..12 "const"
                Blankspace@12..13 " "
                Name@13..20
                  Identifier@13..20 "private"
                Blankspace@20..21 " "
                Equal@21..22 "="
                Blankspace@22..23 " "
                InfixExpression@23..51
                  FunctionCall@23..32
                    IdentExpression@23..30
                      Path@23..30
                        Identifier@23..30 "private"
                    Arguments@30..32
                      ParenthesisLeft@30..31 "("
                      ParenthesisRight@31..32 ")"
                  Blankspace@32..33 " "
                  Plus@33..34 "+"
                  Blankspace@34..35 " "
                  FunctionCall@35..51
                    IdentExpression@35..49
                      Path@35..49
                        Identifier@35..40 "mylib"
                        ColonColon@40..42 "::"
                        Identifier@42..49 "private"
                    Arguments@49..51
                      ParenthesisLeft@49..50 "("
                      ParenthesisRight@50..51 ")"
                Semicolon@51..52 ";""#]],
    );
}

#[test]
fn private_is_usable_in_template() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "var<private> a: foo<private> = bar<private, private>();",
        expect![[r#"
            SourceFile@0..55
              VariableDeclaration@0..55
                Var@0..3 "var"
                TemplateList@3..12
                  TemplateStart@3..4 "<"
                  IdentExpression@4..11
                    Path@4..11
                      Identifier@4..11 "private"
                  TemplateEnd@11..12 ">"
                Blankspace@12..13 " "
                Name@13..14
                  Identifier@13..14 "a"
                Colon@14..15 ":"
                Blankspace@15..16 " "
                TypeSpecifier@16..28
                  Path@16..19
                    Identifier@16..19 "foo"
                  TemplateList@19..28
                    TemplateStart@19..20 "<"
                    IdentExpression@20..27
                      Path@20..27
                        Identifier@20..27 "private"
                    TemplateEnd@27..28 ">"
                Blankspace@28..29 " "
                Equal@29..30 "="
                Blankspace@30..31 " "
                FunctionCall@31..54
                  IdentExpression@31..52
                    Path@31..34
                      Identifier@31..34 "bar"
                    TemplateList@34..52
                      TemplateStart@34..35 "<"
                      IdentExpression@35..42
                        Path@35..42
                          Identifier@35..42 "private"
                      Comma@42..43 ","
                      Blankspace@43..44 " "
                      IdentExpression@44..51
                        Path@44..51
                          Identifier@44..51 "private"
                      TemplateEnd@51..52 ">"
                  Arguments@52..54
                    ParenthesisLeft@52..53 "("
                    ParenthesisRight@53..54 ")"
                Semicolon@54..55 ";""#]],
    );
}

#[test]
fn fn_named_private_can_have_template() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "const a = private<private>(private);",
        expect![[r#"
            SourceFile@0..36
              ConstantDeclaration@0..36
                Const@0..5 "const"
                Blankspace@5..6 " "
                Name@6..7
                  Identifier@6..7 "a"
                Blankspace@7..8 " "
                Equal@8..9 "="
                Blankspace@9..10 " "
                FunctionCall@10..35
                  IdentExpression@10..26
                    Path@10..17
                      Identifier@10..17 "private"
                    TemplateList@17..26
                      TemplateStart@17..18 "<"
                      IdentExpression@18..25
                        Path@18..25
                          Identifier@18..25 "private"
                      TemplateEnd@25..26 ">"
                  Arguments@26..35
                    ParenthesisLeft@26..27 "("
                    IdentExpression@27..34
                      Path@27..34
                        Identifier@27..34 "private"
                    ParenthesisRight@34..35 ")"
                Semicolon@35..36 ";""#]],
    );
}

#[test]
fn private_is_valid_in_function_statement_module_path() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "fn bar() {
          private::private::private();
        }",
        expect![[r#"
            SourceFile@0..59
              FunctionDeclaration@0..59
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..6
                  Identifier@3..6 "bar"
                FunctionParameters@6..8
                  ParenthesisLeft@6..7 "("
                  ParenthesisRight@7..8 ")"
                Blankspace@8..9 " "
                CompoundStatement@9..59
                  BraceLeft@9..10 "{"
                  Blankspace@10..21 "\n          "
                  FunctionCallStatement@21..49
                    FunctionCall@21..48
                      IdentExpression@21..46
                        Path@21..46
                          Identifier@21..28 "private"
                          ColonColon@28..30 "::"
                          Identifier@30..37 "private"
                          ColonColon@37..39 "::"
                          Identifier@39..46 "private"
                      Arguments@46..48
                        ParenthesisLeft@46..47 "("
                        ParenthesisRight@47..48 ")"
                    Semicolon@48..49 ";"
                  Blankspace@49..58 "\n        "
                  BraceRight@58..59 "}""#]],
    );
}
