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
            error at 0..6: invalid syntax, expected one of: 'alias', '@', '{', 'const', 'const_assert', 'diagnostic', <end of file>, 'enable', 'fn', <identifier>, 'import', 'let', 'override', 'private', 'public', 'requires', ';', 'struct', 'var'"#]],
    );
}

#[test]
fn reject_private_visibility_in_wgsl() {
    check_with_edition(
        edition::Edition::Wgsl,
        "private const x:u32=0;",
        expect![[r#"
            SourceFile@0..22
              ConstantDeclaration@0..22
                Visibility@0..7
                  Identifier@0..7 "private"
                Blankspace@7..8 " "
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

            error at 8..13: visibility `private` is not allowed in WGSL"#]],
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
        private struct Foo{a:u32}
        private var<private> x:u32;
        ",
        expect![[r#"
            SourceFile@0..103
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
              StructDeclaration@33..58
                Visibility@33..40
                  Private@33..40 "private"
                Blankspace@40..41 " "
                Struct@41..47 "struct"
                Blankspace@47..48 " "
                Name@48..51
                  Identifier@48..51 "Foo"
                StructBody@51..58
                  BraceLeft@51..52 "{"
                  StructMember@52..57
                    Name@52..53
                      Identifier@52..53 "a"
                    Colon@53..54 ":"
                    TypeSpecifier@54..57
                      Path@54..57
                        Identifier@54..57 "u32"
                  BraceRight@57..58 "}"
              Blankspace@58..67 "\n        "
              VariableDeclaration@67..94
                Visibility@67..74
                  Private@67..74 "private"
                Blankspace@74..75 " "
                Var@75..78 "var"
                TemplateList@78..87
                  TemplateStart@78..79 "<"
                  IdentExpression@79..86
                    Path@79..86
                      Identifier@79..86 "private"
                  TemplateEnd@86..87 ">"
                Blankspace@87..88 " "
                Name@88..89
                  Identifier@88..89 "x"
                Colon@89..90 ":"
                TypeSpecifier@90..93
                  Path@90..93
                    Identifier@90..93 "u32"
                Semicolon@93..94 ";"
              Blankspace@94..103 "\n        ""#]],
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
