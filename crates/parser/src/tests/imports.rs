use expect_test::expect;

use crate::tests::{check, check_with_edition};

#[test]
fn simplest_import_fail() {
    check(
        "import foo;",
        expect![[r#"
            SourceFile@0..11
              Error@0..10
                Reserved@0..6 "import"
                Blankspace@6..7 " "
                Identifier@7..10 "foo"
              Semicolon@10..11 ";"

            error at 0..6: 'import' is a reserved word in WGSL
            error at 0..6: switch to WESL to use `import`
            error at 0..6: invalid syntax, expected one of: 'alias', '@', 'const', 'const_assert', 'diagnostic', <end of file>, 'enable', 'fn', 'import', 'let', 'override', 'requires', ';', 'struct', 'var'"#]],
    );
}

#[test]
fn qualified_path_fail() {
    check(
        "const x: some_package::Type = 0;",
        expect![[r#"
            SourceFile@0..32
              ConstantDeclaration@0..32
                Const@0..5 "const"
                Blankspace@5..6 " "
                Name@6..7
                  Identifier@6..7 "x"
                Colon@7..8 ":"
                Blankspace@8..9 " "
                TypeSpecifier@9..27
                  Path@9..27
                    Identifier@9..21 "some_package"
                    ColonColon@21..23 "::"
                    Identifier@23..27 "Type"
                Blankspace@27..28 " "
                Equal@28..29 "="
                Blankspace@29..30 " "
                Literal@30..31
                  IntLiteral@30..31 "0"
                Semicolon@31..32 ";"

            error at 23..27: switch to WESL to use qualified paths"#]],
    );
}

#[test]
fn simplest_import() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "import foo;",
        expect![[r#"
            SourceFile@0..11
              ImportStatement@0..11
                Import@0..6 "import"
                Blankspace@6..7 " "
                ImportItem@7..10
                  Name@7..10
                    Identifier@7..10 "foo"
                Semicolon@10..11 ";""#]],
    );
}

#[test]
fn super_import() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "import super::super::bar;",
        expect![[r#"
            SourceFile@0..25
              ImportStatement@0..25
                Import@0..6 "import"
                Blankspace@6..7 " "
                ImportSuperRelative@7..21
                  Super@7..12 "super"
                  ColonColon@12..14 "::"
                  Super@14..19 "super"
                  ColonColon@19..21 "::"
                ImportItem@21..24
                  Name@21..24
                    Identifier@21..24 "bar"
                Semicolon@24..25 ";""#]],
    );
}

#[test]
fn package_import() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "import package::{bar};",
        expect![[r#"
            SourceFile@0..22
              ImportStatement@0..22
                Import@0..6 "import"
                Blankspace@6..7 " "
                ImportPackageRelative@7..16
                  Package@7..14 "package"
                  ColonColon@14..16 "::"
                ImportCollection@16..21
                  BraceLeft@16..17 "{"
                  ImportItem@17..20
                    Name@17..20
                      Identifier@17..20 "bar"
                  BraceRight@20..21 "}"
                Semicolon@21..22 ";""#]],
    );
}

#[test]
fn import_alias() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "import foo::bar as bar;",
        expect![[r#"
            SourceFile@0..23
              ImportStatement@0..23
                Import@0..6 "import"
                Blankspace@6..7 " "
                ImportPath@7..22
                  Name@7..10
                    Identifier@7..10 "foo"
                  ColonColon@10..12 "::"
                  ImportItem@12..22
                    Name@12..15
                      Identifier@12..15 "bar"
                    Blankspace@15..16 " "
                    As@16..18 "as"
                    Blankspace@18..19 " "
                    Name@19..22
                      Identifier@19..22 "bar"
                Semicolon@22..23 ";""#]],
    );
}

#[test]
fn illegal_import_aliasing_super() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "import super as bar;",
        expect![[r#"
            SourceFile@0..20
              ImportStatement@0..20
                Import@0..6 "import"
                Blankspace@6..7 " "
                ImportSuperRelative@7..15
                  Super@7..12 "super"
                  Blankspace@12..13 " "
                  Error@13..15
                    As@13..15 "as"
                Blankspace@15..16 " "
                ImportItem@16..19
                  Name@16..19
                    Identifier@16..19 "bar"
                Semicolon@19..20 ";"

            error at 13..15: invalid syntax, expected: '::'"#]],
    );
}

#[test]
fn import_nested_collections() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "import bevy_pbr::{
  forward_io::VertexOutput,
  pbr_types::{PbrInput as PbrOutput, pbr_input_new},
  pbr_bindings,
};",
        expect![[r#"
            SourceFile@0..118
              ImportStatement@0..118
                Import@0..6 "import"
                Blankspace@6..7 " "
                ImportPath@7..117
                  Name@7..15
                    Identifier@7..15 "bevy_pbr"
                  ColonColon@15..17 "::"
                  ImportCollection@17..117
                    BraceLeft@17..18 "{"
                    Blankspace@18..21 "\n  "
                    ImportPath@21..45
                      Name@21..31
                        Identifier@21..31 "forward_io"
                      ColonColon@31..33 "::"
                      ImportItem@33..45
                        Name@33..45
                          Identifier@33..45 "VertexOutput"
                    Comma@45..46 ","
                    Blankspace@46..49 "\n  "
                    ImportPath@49..98
                      Name@49..58
                        Identifier@49..58 "pbr_types"
                      ColonColon@58..60 "::"
                      ImportCollection@60..98
                        BraceLeft@60..61 "{"
                        ImportItem@61..82
                          Name@61..69
                            Identifier@61..69 "PbrInput"
                          Blankspace@69..70 " "
                          As@70..72 "as"
                          Blankspace@72..73 " "
                          Name@73..82
                            Identifier@73..82 "PbrOutput"
                        Comma@82..83 ","
                        Blankspace@83..84 " "
                        ImportItem@84..97
                          Name@84..97
                            Identifier@84..97 "pbr_input_new"
                        BraceRight@97..98 "}"
                    Comma@98..99 ","
                    Blankspace@99..102 "\n  "
                    ImportItem@102..114
                      Name@102..114
                        Identifier@102..114 "pbr_bindings"
                    Comma@114..115 ","
                    Blankspace@115..116 "\n"
                    BraceRight@116..117 "}"
                Semicolon@117..118 ";""#]],
    );
}
