use expect_test::expect;

use crate::tests::{check, check_with_edition};

#[test]
fn evil_attribute() {
    check(
        "@9",
        expect![[r#"
            SourceFile@0..2
              AttributeList@0..2
                Attribute@0..2
                  AttributeOperator@0..1 "@"
                  Error@1..2
                    IntLiteral@1..2 "9"

            error at 1..2: invalid syntax, expected: <identifier>"#]],
    );
}

#[test]
fn attribute_name_after_comment() {
    check(
        "@ /* foo */ bar
        const a = 1;",
        expect![[r#"
            SourceFile@0..36
              AttributeList@0..15
                Attribute@0..15
                  AttributeOperator@0..1 "@"
                  Blankspace@1..2 " "
                  BlockComment@2..11 "/* foo */"
                  Blankspace@11..12 " "
                  Identifier@12..15 "bar"
              Blankspace@15..24 "\n        "
              ConstantDeclaration@24..36
                Const@24..29 "const"
                Blankspace@29..30 " "
                Name@30..31
                  Identifier@30..31 "a"
                Blankspace@31..32 " "
                Equal@32..33 "="
                Blankspace@33..34 " "
                Literal@34..35
                  IntLiteral@34..35 "1"
                Semicolon@35..36 ";""#]],
    );
}

#[test]
fn attribute_recovery_after_symbol() {
    check(
        "@ || bar
        const a = 1;",
        expect![[r#"
            SourceFile@0..29
              AttributeList@0..4
                Attribute@0..4
                  AttributeOperator@0..1 "@"
                  Blankspace@1..2 " "
                  Error@2..4
                    OrOr@2..4 "||"
              Blankspace@4..5 " "
              Error@5..8
                Identifier@5..8 "bar"
              Blankspace@8..17 "\n        "
              ConstantDeclaration@17..29
                Const@17..22 "const"
                Blankspace@22..23 " "
                Name@23..24
                  Identifier@23..24 "a"
                Blankspace@24..25 " "
                Equal@25..26 "="
                Blankspace@26..27 " "
                Literal@27..28
                  IntLiteral@27..28 "1"
                Semicolon@28..29 ";"

            error at 2..4: invalid syntax, expected: <identifier>"#]],
    );
}

#[test]
fn attribute_recovery_keyword_after_symbol() {
    // Suboptimal recovery, see: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1578
    check(
        "@ || const a = 1;",
        expect![[r#"
            SourceFile@0..17
              AttributeList@0..4
                Attribute@0..4
                  AttributeOperator@0..1 "@"
                  Blankspace@1..2 " "
                  Error@2..4
                    OrOr@2..4 "||"
              Blankspace@4..5 " "
              Error@5..16
                Identifier@5..10 "const"
                Blankspace@10..11 " "
                Identifier@11..12 "a"
                Blankspace@12..13 " "
                Equal@13..14 "="
                Blankspace@14..15 " "
                IntLiteral@15..16 "1"
              Semicolon@16..17 ";"

            error at 2..4: invalid syntax, expected: <identifier>"#]],
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
              AttributeList@9..30
                Attribute@9..30
                  AttributeOperator@9..10 "@"
                  Identifier@10..20 "diagnostic"
                  Arguments@20..30
                    ParenthesisLeft@20..21 "("
                    IdentExpression@21..24
                      Path@21..24
                        Identifier@21..24 "off"
                    Comma@24..25 ","
                    Blankspace@25..26 " "
                    IdentExpression@26..29
                      Path@26..29
                        Identifier@26..29 "bla"
                    ParenthesisRight@29..30 ")"
              Blankspace@30..39 "\n        "
              FunctionDeclaration@39..51
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
              AttributeList@9..15
                Attribute@9..15
                  AttributeOperator@9..10 "@"
                  Identifier@10..15 "const"
              Blankspace@15..24 "\n        "
              FunctionDeclaration@24..33
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
                  AttributeList@51..70
                    Attribute@51..70
                      AttributeOperator@51..52 "@"
                      Blankspace@52..53 " "
                      Identifier@53..60 "builtin"
                      Arguments@60..70
                        ParenthesisLeft@60..61 "("
                        IdentExpression@61..69
                          Path@61..69
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
                  AttributeList@150..170
                    Attribute@150..170
                      AttributeOperator@150..151 "@"
                      Identifier@151..158 "builtin"
                      Arguments@158..170
                        ParenthesisLeft@158..159 "("
                        Blankspace@159..160 " "
                        IdentExpression@160..168
                          Path@160..168
                            Identifier@160..168 "position"
                        Blankspace@168..169 " "
                        ParenthesisRight@169..170 ")"
                  Blankspace@170..171 " "
                  Parameter@171..190
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
              AttributeList@9..34
                Attribute@9..34
                  AttributeOperator@9..10 "@"
                  Identifier@10..21 "interpolate"
                  Arguments@21..34
                    ParenthesisLeft@21..22 "("
                    IdentExpression@22..33
                      Path@22..33
                        Identifier@22..33 "perspective"
                    ParenthesisRight@33..34 ")"
              Blankspace@34..43 "\n        "
              VariableDeclaration@43..63
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
              AttributeList@9..29
                Attribute@9..29
                  AttributeOperator@9..10 "@"
                  Identifier@10..21 "interpolate"
                  Arguments@21..29
                    ParenthesisLeft@21..22 "("
                    IdentExpression@22..28
                      Path@22..28
                        Identifier@22..28 "linear"
                    ParenthesisRight@28..29 ")"
              Blankspace@29..38 "\n        "
              VariableDeclaration@38..58
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
              AttributeList@9..27
                Attribute@9..27
                  AttributeOperator@9..10 "@"
                  Identifier@10..21 "interpolate"
                  Arguments@21..27
                    ParenthesisLeft@21..22 "("
                    IdentExpression@22..26
                      Path@22..26
                        Identifier@22..26 "flat"
                    ParenthesisRight@26..27 ")"
              Blankspace@27..36 "\n        "
              VariableDeclaration@36..56
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
              AttributeList@9..42
                Attribute@9..42
                  AttributeOperator@9..10 "@"
                  Identifier@10..21 "interpolate"
                  Arguments@21..42
                    ParenthesisLeft@21..22 "("
                    IdentExpression@22..33
                      Path@22..33
                        Identifier@22..33 "perspective"
                    Comma@33..34 ","
                    Blankspace@34..35 " "
                    IdentExpression@35..41
                      Path@35..41
                        Identifier@35..41 "center"
                    ParenthesisRight@41..42 ")"
              Blankspace@42..51 "\n        "
              VariableDeclaration@51..71
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
              AttributeList@9..44
                Attribute@9..44
                  AttributeOperator@9..10 "@"
                  Identifier@10..21 "interpolate"
                  Arguments@21..44
                    ParenthesisLeft@21..22 "("
                    IdentExpression@22..33
                      Path@22..33
                        Identifier@22..33 "perspective"
                    Comma@33..34 ","
                    Blankspace@34..35 " "
                    IdentExpression@35..43
                      Path@35..43
                        Identifier@35..43 "centroid"
                    ParenthesisRight@43..44 ")"
              Blankspace@44..53 "\n        "
              VariableDeclaration@53..73
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
              AttributeList@9..42
                Attribute@9..42
                  AttributeOperator@9..10 "@"
                  Identifier@10..21 "interpolate"
                  Arguments@21..42
                    ParenthesisLeft@21..22 "("
                    IdentExpression@22..33
                      Path@22..33
                        Identifier@22..33 "perspective"
                    Comma@33..34 ","
                    Blankspace@34..35 " "
                    IdentExpression@35..41
                      Path@35..41
                        Identifier@35..41 "sample"
                    ParenthesisRight@41..42 ")"
              Blankspace@42..51 "\n        "
              VariableDeclaration@51..71
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
              AttributeList@9..37
                Attribute@9..37
                  AttributeOperator@9..10 "@"
                  Identifier@10..21 "interpolate"
                  Arguments@21..37
                    ParenthesisLeft@21..22 "("
                    IdentExpression@22..28
                      Path@22..28
                        Identifier@22..28 "linear"
                    Comma@28..29 ","
                    Blankspace@29..30 " "
                    IdentExpression@30..36
                      Path@30..36
                        Identifier@30..36 "center"
                    ParenthesisRight@36..37 ")"
              Blankspace@37..46 "\n        "
              VariableDeclaration@46..66
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
              AttributeList@9..39
                Attribute@9..39
                  AttributeOperator@9..10 "@"
                  Identifier@10..21 "interpolate"
                  Arguments@21..39
                    ParenthesisLeft@21..22 "("
                    IdentExpression@22..28
                      Path@22..28
                        Identifier@22..28 "linear"
                    Comma@28..29 ","
                    Blankspace@29..30 " "
                    IdentExpression@30..38
                      Path@30..38
                        Identifier@30..38 "centroid"
                    ParenthesisRight@38..39 ")"
              Blankspace@39..48 "\n        "
              VariableDeclaration@48..68
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
              AttributeList@9..37
                Attribute@9..37
                  AttributeOperator@9..10 "@"
                  Identifier@10..21 "interpolate"
                  Arguments@21..37
                    ParenthesisLeft@21..22 "("
                    IdentExpression@22..28
                      Path@22..28
                        Identifier@22..28 "linear"
                    Comma@28..29 ","
                    Blankspace@29..30 " "
                    IdentExpression@30..36
                      Path@30..36
                        Identifier@30..36 "sample"
                    ParenthesisRight@36..37 ")"
              Blankspace@37..46 "\n        "
              VariableDeclaration@46..66
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
              AttributeList@9..34
                Attribute@9..34
                  AttributeOperator@9..10 "@"
                  Identifier@10..21 "interpolate"
                  Arguments@21..34
                    ParenthesisLeft@21..22 "("
                    IdentExpression@22..26
                      Path@22..26
                        Identifier@22..26 "flat"
                    Comma@26..27 ","
                    Blankspace@27..28 " "
                    IdentExpression@28..33
                      Path@28..33
                        Identifier@28..33 "first"
                    ParenthesisRight@33..34 ")"
              Blankspace@34..43 "\n        "
              VariableDeclaration@43..63
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
              AttributeList@9..35
                Attribute@9..35
                  AttributeOperator@9..10 "@"
                  Identifier@10..21 "interpolate"
                  Arguments@21..35
                    ParenthesisLeft@21..22 "("
                    IdentExpression@22..26
                      Path@22..26
                        Identifier@22..26 "flat"
                    Comma@26..27 ","
                    Blankspace@27..28 " "
                    IdentExpression@28..34
                      Path@28..34
                        Identifier@28..34 "either"
                    ParenthesisRight@34..35 ")"
              Blankspace@35..44 "\n        "
              VariableDeclaration@44..64
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
              AttributeList@9..33
                Attribute@9..33
                  AttributeOperator@9..10 "@"
                  Identifier@10..21 "interpolate"
                  Arguments@21..33
                    ParenthesisLeft@21..22 "("
                    IdentExpression@22..33
                      Path@22..33
                        Identifier@22..33 "perspective"
              Blankspace@33..42 "\n        "
              VariableDeclaration@42..62
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

            error at 42..45: invalid syntax, expected one of: '&', '&&', '&=', '@', '{', '}', '[', ']', ':', '::', ',', '/=', '=', '==', '/', '>', '>=', <identifier>, '<', '<=', '-', '-=', '--', '%', '%=', '!=', '|', '|=', '||', '(', ')', '.', '+', '+=', '++', ';', '<<', '<<=', '>>', '>>=', '*', <template end>, <template start>, '*=', '^', '^='"#]],
    );
}

#[test]
fn parse_conditional_transpilation() {
    check_with_edition(
        edition::Edition::Wesl2025Unstable,
        "
        @if(true)
        fn fragment() {}
        @elif(false)
        fn fragment() {}
        @else
        fn fragment() {}
        ",
        expect![[r#"
            SourceFile@0..137
              Blankspace@0..9 "\n        "
              AttributeList@9..18
                Attribute@9..18
                  AttributeOperator@9..10 "@"
                  Identifier@10..12 "if"
                  Arguments@12..18
                    ParenthesisLeft@12..13 "("
                    Literal@13..17
                      True@13..17 "true"
                    ParenthesisRight@17..18 ")"
              Blankspace@18..27 "\n        "
              FunctionDeclaration@27..43
                Fn@27..29 "fn"
                Blankspace@29..30 " "
                Name@30..38
                  Identifier@30..38 "fragment"
                FunctionParameters@38..40
                  ParenthesisLeft@38..39 "("
                  ParenthesisRight@39..40 ")"
                Blankspace@40..41 " "
                CompoundStatement@41..43
                  BraceLeft@41..42 "{"
                  BraceRight@42..43 "}"
              Blankspace@43..52 "\n        "
              AttributeList@52..64
                Attribute@52..64
                  AttributeOperator@52..53 "@"
                  Identifier@53..57 "elif"
                  Arguments@57..64
                    ParenthesisLeft@57..58 "("
                    Literal@58..63
                      False@58..63 "false"
                    ParenthesisRight@63..64 ")"
              Blankspace@64..73 "\n        "
              FunctionDeclaration@73..89
                Fn@73..75 "fn"
                Blankspace@75..76 " "
                Name@76..84
                  Identifier@76..84 "fragment"
                FunctionParameters@84..86
                  ParenthesisLeft@84..85 "("
                  ParenthesisRight@85..86 ")"
                Blankspace@86..87 " "
                CompoundStatement@87..89
                  BraceLeft@87..88 "{"
                  BraceRight@88..89 "}"
              Blankspace@89..98 "\n        "
              AttributeList@98..103
                Attribute@98..103
                  AttributeOperator@98..99 "@"
                  Identifier@99..103 "else"
              Blankspace@103..112 "\n        "
              FunctionDeclaration@112..128
                Fn@112..114 "fn"
                Blankspace@114..115 " "
                Name@115..123
                  Identifier@115..123 "fragment"
                FunctionParameters@123..125
                  ParenthesisLeft@123..124 "("
                  ParenthesisRight@124..125 ")"
                Blankspace@125..126 " "
                CompoundStatement@126..128
                  BraceLeft@126..127 "{"
                  BraceRight@127..128 "}"
              Blankspace@128..137 "\n        ""#]],
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
              AttributeList@9..18
                Attribute@9..18
                  AttributeOperator@9..10 "@"
                  Identifier@10..18 "fragment"
              Blankspace@18..27 "\n        "
              FunctionDeclaration@27..111
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
                  AttributeList@32..52
                    Attribute@32..42
                      AttributeOperator@32..33 "@"
                      Identifier@33..38 "align"
                      Arguments@38..42
                        ParenthesisLeft@38..39 "("
                        Literal@39..41
                          IntLiteral@39..41 "16"
                        ParenthesisRight@41..42 ")"
                    Blankspace@42..43 " "
                    Attribute@43..52
                      AttributeOperator@43..44 "@"
                      Identifier@44..48 "size"
                      Arguments@48..52
                        ParenthesisLeft@48..49 "("
                        Literal@49..51
                          IntLiteral@49..51 "16"
                        ParenthesisRight@51..52 ")"
                  Blankspace@52..65 "\n            "
                  StructMember@65..77
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
              AttributeList@99..120
                Attribute@99..108
                  AttributeOperator@99..100 "@"
                  Identifier@100..105 "group"
                  Arguments@105..108
                    ParenthesisLeft@105..106 "("
                    Literal@106..107
                      IntLiteral@106..107 "0"
                    ParenthesisRight@107..108 ")"
                Blankspace@108..109 " "
                Attribute@109..120
                  AttributeOperator@109..110 "@"
                  Identifier@110..117 "binding"
                  Arguments@117..120
                    ParenthesisLeft@117..118 "("
                    Literal@118..119
                      IntLiteral@118..119 "0"
                    ParenthesisRight@119..120 ")"
              Blankspace@120..129 "\n        "
              VariableDeclaration@129..147
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
              AttributeList@157..163
                Attribute@157..163
                  AttributeOperator@157..158 "@"
                  Identifier@158..160 "id"
                  Arguments@160..163
                    ParenthesisLeft@160..161 "("
                    Literal@161..162
                      IntLiteral@161..162 "0"
                    ParenthesisRight@162..163 ")"
              Blankspace@163..172 "\n        "
              OverrideDeclaration@172..192
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
              AttributeList@202..211
                Attribute@202..211
                  AttributeOperator@202..203 "@"
                  Identifier@203..211 "must_use"
              Blankspace@211..220 "\n        "
              FunctionDeclaration@220..320
                Fn@220..222 "fn"
                Blankspace@222..223 " "
                Name@223..224
                  Identifier@223..224 "f"
                FunctionParameters@224..266
                  ParenthesisLeft@224..225 "("
                  AttributeList@225..258
                    Attribute@225..237
                      AttributeOperator@225..226 "@"
                      Identifier@226..234 "location"
                      Arguments@234..237
                        ParenthesisLeft@234..235 "("
                        Literal@235..236
                          IntLiteral@235..236 "0"
                        ParenthesisRight@236..237 ")"
                    Blankspace@237..238 " "
                    Attribute@238..258
                      AttributeOperator@238..239 "@"
                      Identifier@239..250 "interpolate"
                      Arguments@250..258
                        ParenthesisLeft@250..251 "("
                        IdentExpression@251..257
                          Path@251..257
                            Identifier@251..257 "linear"
                        ParenthesisRight@257..258 ")"
                  Blankspace@258..259 " "
                  Parameter@259..265
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
                  AttributeList@270..282
                    Attribute@270..282
                      AttributeOperator@270..271 "@"
                      Identifier@271..279 "location"
                      Arguments@279..282
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
              AttributeList@330..337
                Attribute@330..337
                  AttributeOperator@330..331 "@"
                  Identifier@331..337 "vertex"
              Blankspace@337..346 "\n        "
              FunctionDeclaration@346..482
                Fn@346..348 "fn"
                Blankspace@348..349 " "
                Name@349..351
                  Identifier@349..351 "vs"
                FunctionParameters@351..382
                  ParenthesisLeft@351..352 "("
                  AttributeList@352..374
                    Attribute@352..374
                      AttributeOperator@352..353 "@"
                      Identifier@353..360 "builtin"
                      Arguments@360..374
                        ParenthesisLeft@360..361 "("
                        IdentExpression@361..373
                          Path@361..373
                            Identifier@361..373 "vertex_index"
                        ParenthesisRight@373..374 ")"
                  Blankspace@374..375 " "
                  Parameter@375..381
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
                  AttributeList@386..404
                    Attribute@386..404
                      AttributeOperator@386..387 "@"
                      Identifier@387..394 "builtin"
                      Arguments@394..404
                        ParenthesisLeft@394..395 "("
                        IdentExpression@395..403
                          Path@395..403
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
                  AttributeList@515..541
                    Attribute@515..527
                      AttributeOperator@515..516 "@"
                      Identifier@516..524 "location"
                      Arguments@524..527
                        ParenthesisLeft@524..525 "("
                        Literal@525..526
                          IntLiteral@525..526 "0"
                        ParenthesisRight@526..527 ")"
                    Blankspace@527..528 " "
                    Attribute@528..541
                      AttributeOperator@528..529 "@"
                      Identifier@529..538 "blend_src"
                      Arguments@538..541
                        ParenthesisLeft@538..539 "("
                        Literal@539..540
                          IntLiteral@539..540 "0"
                        ParenthesisRight@540..541 ")"
                  Blankspace@541..554 "\n            "
                  StructMember@554..570
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
              AttributeList@592..620
                Attribute@592..601
                  AttributeOperator@592..593 "@"
                  Identifier@593..601 "fragment"
                Blankspace@601..610 "\n        "
                Attribute@610..620
                  AttributeOperator@610..611 "@"
                  Identifier@611..620 "invariant"
              Blankspace@620..629 "\n        "
              FunctionDeclaration@629..699
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
              AttributeList@709..744
                Attribute@709..717
                  AttributeOperator@709..710 "@"
                  Identifier@710..717 "compute"
                Blankspace@717..726 "\n        "
                Attribute@726..744
                  AttributeOperator@726..727 "@"
                  Identifier@727..741 "workgroup_size"
                  Arguments@741..744
                    ParenthesisLeft@741..742 "("
                    Literal@742..743
                      IntLiteral@742..743 "1"
                    ParenthesisRight@743..744 ")"
              Blankspace@744..753 "\n        "
              FunctionDeclaration@753..763
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
fn parse_compute_workgroup_size_attrs() {
    check(
        "
        @compute
        @workgroup_size(1)
        fn cs() {}
        ",
        expect![[r#"
            SourceFile@0..72
              Blankspace@0..9 "\n        "
              AttributeList@9..44
                Attribute@9..17
                  AttributeOperator@9..10 "@"
                  Identifier@10..17 "compute"
                Blankspace@17..26 "\n        "
                Attribute@26..44
                  AttributeOperator@26..27 "@"
                  Identifier@27..41 "workgroup_size"
                  Arguments@41..44
                    ParenthesisLeft@41..42 "("
                    Literal@42..43
                      IntLiteral@42..43 "1"
                    ParenthesisRight@43..44 ")"
              Blankspace@44..53 "\n        "
              FunctionDeclaration@53..63
                Fn@53..55 "fn"
                Blankspace@55..56 " "
                Name@56..58
                  Identifier@56..58 "cs"
                FunctionParameters@58..60
                  ParenthesisLeft@58..59 "("
                  ParenthesisRight@59..60 ")"
                Blankspace@60..61 " "
                CompoundStatement@61..63
                  BraceLeft@61..62 "{"
                  BraceRight@62..63 "}"
              Blankspace@63..72 "\n        ""#]],
    );
}
