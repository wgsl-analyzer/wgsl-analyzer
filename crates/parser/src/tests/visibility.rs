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
