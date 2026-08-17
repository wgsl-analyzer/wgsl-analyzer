use edition::Capabilities;
use expect_test::expect;

use crate::{check_entrypoint_with_capabilities, tests::check_with_capabilities};

#[test]
fn capability_not_present() {
    check_with_capabilities(
        Capabilities::default(),
        "
        @fragment
        @early_depth_test(force)
        fn fragment(in: FragmentInput) -> @location(0) vec4<f32> { }
        ",
        expect![[r#"
            SourceFile@0..129
              Blankspace@0..9 "\n        "
              AttributeList@9..51
                FragmentAttribute@9..18
                  AttributeOperator@9..10 "@"
                  Fragment@10..18 "fragment"
                Blankspace@18..27 "\n        "
                EarlyDepthTestAttribute@27..51
                  AttributeOperator@27..28 "@"
                  EarlyDepthTest@28..44 "early_depth_test"
                  ParenthesisLeft@44..45 "("
                  EarlyDepthTestMode@45..50
                    Force@45..50 "force"
                  ParenthesisRight@50..51 ")"
              Blankspace@51..60 "\n        "
              FunctionDeclaration@60..120
                Fn@60..62 "fn"
                Blankspace@62..63 " "
                Name@63..71
                  Identifier@63..71 "fragment"
                FunctionParameters@71..90
                  ParenthesisLeft@71..72 "("
                  Parameter@72..89
                    Name@72..74
                      Identifier@72..74 "in"
                    Colon@74..75 ":"
                    Blankspace@75..76 " "
                    TypeSpecifier@76..89
                      Path@76..89
                        Identifier@76..89 "FragmentInput"
                  ParenthesisRight@89..90 ")"
                Blankspace@90..91 " "
                ReturnType@91..116
                  Arrow@91..93 "->"
                  Blankspace@93..94 " "
                  AttributeList@94..106
                    LocationAttribute@94..106
                      AttributeOperator@94..95 "@"
                      Location@95..103 "location"
                      ParenthesisLeft@103..104 "("
                      Literal@104..105
                        IntLiteral@104..105 "0"
                      ParenthesisRight@105..106 ")"
                  Blankspace@106..107 " "
                  TypeSpecifier@107..116
                    Path@107..111
                      Identifier@107..111 "vec4"
                    TemplateList@111..116
                      TemplateStart@111..112 "<"
                      IdentExpression@112..115
                        Path@112..115
                          Identifier@112..115 "f32"
                      TemplateEnd@115..116 ">"
                Blankspace@116..117 " "
                CompoundStatement@117..120
                  BraceLeft@117..118 "{"
                  Blankspace@118..119 " "
                  BraceRight@119..120 "}"
              Blankspace@120..129 "\n        "

            error at 27..51: the extension EARLY_DEPTH_TEST is not enabled"#]],
    );
}

#[test]
fn parse_early_depth_test_force() {
    check_with_capabilities(
        Capabilities {
            early_depth_test: true,
            ..Default::default()
        },
        "
        @fragment
        @early_depth_test(force)
        fn fragment(in: FragmentInput) -> @location(0) vec4<f32> { }
        ",
        expect![[r#"
            SourceFile@0..129
              Blankspace@0..9 "\n        "
              AttributeList@9..51
                FragmentAttribute@9..18
                  AttributeOperator@9..10 "@"
                  Fragment@10..18 "fragment"
                Blankspace@18..27 "\n        "
                EarlyDepthTestAttribute@27..51
                  AttributeOperator@27..28 "@"
                  EarlyDepthTest@28..44 "early_depth_test"
                  ParenthesisLeft@44..45 "("
                  EarlyDepthTestMode@45..50
                    Force@45..50 "force"
                  ParenthesisRight@50..51 ")"
              Blankspace@51..60 "\n        "
              FunctionDeclaration@60..120
                Fn@60..62 "fn"
                Blankspace@62..63 " "
                Name@63..71
                  Identifier@63..71 "fragment"
                FunctionParameters@71..90
                  ParenthesisLeft@71..72 "("
                  Parameter@72..89
                    Name@72..74
                      Identifier@72..74 "in"
                    Colon@74..75 ":"
                    Blankspace@75..76 " "
                    TypeSpecifier@76..89
                      Path@76..89
                        Identifier@76..89 "FragmentInput"
                  ParenthesisRight@89..90 ")"
                Blankspace@90..91 " "
                ReturnType@91..116
                  Arrow@91..93 "->"
                  Blankspace@93..94 " "
                  AttributeList@94..106
                    LocationAttribute@94..106
                      AttributeOperator@94..95 "@"
                      Location@95..103 "location"
                      ParenthesisLeft@103..104 "("
                      Literal@104..105
                        IntLiteral@104..105 "0"
                      ParenthesisRight@105..106 ")"
                  Blankspace@106..107 " "
                  TypeSpecifier@107..116
                    Path@107..111
                      Identifier@107..111 "vec4"
                    TemplateList@111..116
                      TemplateStart@111..112 "<"
                      IdentExpression@112..115
                        Path@112..115
                          Identifier@112..115 "f32"
                      TemplateEnd@115..116 ">"
                Blankspace@116..117 " "
                CompoundStatement@117..120
                  BraceLeft@117..118 "{"
                  Blankspace@118..119 " "
                  BraceRight@119..120 "}"
              Blankspace@120..129 "\n        ""#]],
    );
}

#[test]
fn parse_early_depth_test_greater_equal() {
    check_with_capabilities(
        Capabilities {
            early_depth_test: true,
            ..Default::default()
        },
        "
        @fragment
        @early_depth_test(greater_equal)
        fn fragment(in: FragmentInput) -> @location(0) vec4<f32> { }
        ",
        expect![[r#"
            SourceFile@0..137
              Blankspace@0..9 "\n        "
              AttributeList@9..59
                FragmentAttribute@9..18
                  AttributeOperator@9..10 "@"
                  Fragment@10..18 "fragment"
                Blankspace@18..27 "\n        "
                EarlyDepthTestAttribute@27..59
                  AttributeOperator@27..28 "@"
                  EarlyDepthTest@28..44 "early_depth_test"
                  ParenthesisLeft@44..45 "("
                  EarlyDepthTestMode@45..58
                    GreaterEqual@45..58 "greater_equal"
                  ParenthesisRight@58..59 ")"
              Blankspace@59..68 "\n        "
              FunctionDeclaration@68..128
                Fn@68..70 "fn"
                Blankspace@70..71 " "
                Name@71..79
                  Identifier@71..79 "fragment"
                FunctionParameters@79..98
                  ParenthesisLeft@79..80 "("
                  Parameter@80..97
                    Name@80..82
                      Identifier@80..82 "in"
                    Colon@82..83 ":"
                    Blankspace@83..84 " "
                    TypeSpecifier@84..97
                      Path@84..97
                        Identifier@84..97 "FragmentInput"
                  ParenthesisRight@97..98 ")"
                Blankspace@98..99 " "
                ReturnType@99..124
                  Arrow@99..101 "->"
                  Blankspace@101..102 " "
                  AttributeList@102..114
                    LocationAttribute@102..114
                      AttributeOperator@102..103 "@"
                      Location@103..111 "location"
                      ParenthesisLeft@111..112 "("
                      Literal@112..113
                        IntLiteral@112..113 "0"
                      ParenthesisRight@113..114 ")"
                  Blankspace@114..115 " "
                  TypeSpecifier@115..124
                    Path@115..119
                      Identifier@115..119 "vec4"
                    TemplateList@119..124
                      TemplateStart@119..120 "<"
                      IdentExpression@120..123
                        Path@120..123
                          Identifier@120..123 "f32"
                      TemplateEnd@123..124 ">"
                Blankspace@124..125 " "
                CompoundStatement@125..128
                  BraceLeft@125..126 "{"
                  Blankspace@126..127 " "
                  BraceRight@127..128 "}"
              Blankspace@128..137 "\n        ""#]],
    );
}

#[test]
fn parse_early_depth_test_less_equal() {
    check_with_capabilities(
        Capabilities {
            early_depth_test: true,
            ..Default::default()
        },
        "
        @fragment
        @early_depth_test(less_equal)
        fn fragment(in: FragmentInput) -> @location(0) vec4<f32> { }
        ",
        expect![[r#"
            SourceFile@0..134
              Blankspace@0..9 "\n        "
              AttributeList@9..56
                FragmentAttribute@9..18
                  AttributeOperator@9..10 "@"
                  Fragment@10..18 "fragment"
                Blankspace@18..27 "\n        "
                EarlyDepthTestAttribute@27..56
                  AttributeOperator@27..28 "@"
                  EarlyDepthTest@28..44 "early_depth_test"
                  ParenthesisLeft@44..45 "("
                  EarlyDepthTestMode@45..55
                    LessEqual@45..55 "less_equal"
                  ParenthesisRight@55..56 ")"
              Blankspace@56..65 "\n        "
              FunctionDeclaration@65..125
                Fn@65..67 "fn"
                Blankspace@67..68 " "
                Name@68..76
                  Identifier@68..76 "fragment"
                FunctionParameters@76..95
                  ParenthesisLeft@76..77 "("
                  Parameter@77..94
                    Name@77..79
                      Identifier@77..79 "in"
                    Colon@79..80 ":"
                    Blankspace@80..81 " "
                    TypeSpecifier@81..94
                      Path@81..94
                        Identifier@81..94 "FragmentInput"
                  ParenthesisRight@94..95 ")"
                Blankspace@95..96 " "
                ReturnType@96..121
                  Arrow@96..98 "->"
                  Blankspace@98..99 " "
                  AttributeList@99..111
                    LocationAttribute@99..111
                      AttributeOperator@99..100 "@"
                      Location@100..108 "location"
                      ParenthesisLeft@108..109 "("
                      Literal@109..110
                        IntLiteral@109..110 "0"
                      ParenthesisRight@110..111 ")"
                  Blankspace@111..112 " "
                  TypeSpecifier@112..121
                    Path@112..116
                      Identifier@112..116 "vec4"
                    TemplateList@116..121
                      TemplateStart@116..117 "<"
                      IdentExpression@117..120
                        Path@117..120
                          Identifier@117..120 "f32"
                      TemplateEnd@120..121 ">"
                Blankspace@121..122 " "
                CompoundStatement@122..125
                  BraceLeft@122..123 "{"
                  Blankspace@123..124 " "
                  BraceRight@124..125 "}"
              Blankspace@125..134 "\n        ""#]],
    );
}

#[test]
fn parse_early_depth_test_unchanged() {
    check_with_capabilities(
        Capabilities {
            early_depth_test: true,
            ..Default::default()
        },
        "
        @fragment
        @early_depth_test(unchanged)
        fn fragment(in: FragmentInput) -> @location(0) vec4<f32> { }
        ",
        expect![[r#"
            SourceFile@0..133
              Blankspace@0..9 "\n        "
              AttributeList@9..55
                FragmentAttribute@9..18
                  AttributeOperator@9..10 "@"
                  Fragment@10..18 "fragment"
                Blankspace@18..27 "\n        "
                EarlyDepthTestAttribute@27..55
                  AttributeOperator@27..28 "@"
                  EarlyDepthTest@28..44 "early_depth_test"
                  ParenthesisLeft@44..45 "("
                  EarlyDepthTestMode@45..54
                    Unchanged@45..54 "unchanged"
                  ParenthesisRight@54..55 ")"
              Blankspace@55..64 "\n        "
              FunctionDeclaration@64..124
                Fn@64..66 "fn"
                Blankspace@66..67 " "
                Name@67..75
                  Identifier@67..75 "fragment"
                FunctionParameters@75..94
                  ParenthesisLeft@75..76 "("
                  Parameter@76..93
                    Name@76..78
                      Identifier@76..78 "in"
                    Colon@78..79 ":"
                    Blankspace@79..80 " "
                    TypeSpecifier@80..93
                      Path@80..93
                        Identifier@80..93 "FragmentInput"
                  ParenthesisRight@93..94 ")"
                Blankspace@94..95 " "
                ReturnType@95..120
                  Arrow@95..97 "->"
                  Blankspace@97..98 " "
                  AttributeList@98..110
                    LocationAttribute@98..110
                      AttributeOperator@98..99 "@"
                      Location@99..107 "location"
                      ParenthesisLeft@107..108 "("
                      Literal@108..109
                        IntLiteral@108..109 "0"
                      ParenthesisRight@109..110 ")"
                  Blankspace@110..111 " "
                  TypeSpecifier@111..120
                    Path@111..115
                      Identifier@111..115 "vec4"
                    TemplateList@115..120
                      TemplateStart@115..116 "<"
                      IdentExpression@116..119
                        Path@116..119
                          Identifier@116..119 "f32"
                      TemplateEnd@119..120 ">"
                Blankspace@120..121 " "
                CompoundStatement@121..124
                  BraceLeft@121..122 "{"
                  Blankspace@122..123 " "
                  BraceRight@123..124 "}"
              Blankspace@124..133 "\n        ""#]],
    );
}

#[test]
fn parse_words() {
    check_with_capabilities(
        Capabilities::default(),
        "
        var early_depth_test: i32 = 0;
        var less_equal: i32 = 0;
        var greater_equal: i32 = 0;
        var force: i32 = 0;
        var unchanged: i32 = 0;
        ",
        expect![[r#"
            SourceFile@0..177
              Blankspace@0..9 "\n        "
              VariableDeclaration@9..39
                Var@9..12 "var"
                Blankspace@12..13 " "
                Name@13..29
                  Identifier@13..29 "early_depth_test"
                Colon@29..30 ":"
                Blankspace@30..31 " "
                TypeSpecifier@31..34
                  Path@31..34
                    Identifier@31..34 "i32"
                Blankspace@34..35 " "
                Equal@35..36 "="
                Blankspace@36..37 " "
                Literal@37..38
                  IntLiteral@37..38 "0"
                Semicolon@38..39 ";"
              Blankspace@39..48 "\n        "
              VariableDeclaration@48..72
                Var@48..51 "var"
                Blankspace@51..52 " "
                Name@52..62
                  Identifier@52..62 "less_equal"
                Colon@62..63 ":"
                Blankspace@63..64 " "
                TypeSpecifier@64..67
                  Path@64..67
                    Identifier@64..67 "i32"
                Blankspace@67..68 " "
                Equal@68..69 "="
                Blankspace@69..70 " "
                Literal@70..71
                  IntLiteral@70..71 "0"
                Semicolon@71..72 ";"
              Blankspace@72..81 "\n        "
              VariableDeclaration@81..108
                Var@81..84 "var"
                Blankspace@84..85 " "
                Name@85..98
                  Identifier@85..98 "greater_equal"
                Colon@98..99 ":"
                Blankspace@99..100 " "
                TypeSpecifier@100..103
                  Path@100..103
                    Identifier@100..103 "i32"
                Blankspace@103..104 " "
                Equal@104..105 "="
                Blankspace@105..106 " "
                Literal@106..107
                  IntLiteral@106..107 "0"
                Semicolon@107..108 ";"
              Blankspace@108..117 "\n        "
              VariableDeclaration@117..136
                Var@117..120 "var"
                Blankspace@120..121 " "
                Name@121..126
                  Identifier@121..126 "force"
                Colon@126..127 ":"
                Blankspace@127..128 " "
                TypeSpecifier@128..131
                  Path@128..131
                    Identifier@128..131 "i32"
                Blankspace@131..132 " "
                Equal@132..133 "="
                Blankspace@133..134 " "
                Literal@134..135
                  IntLiteral@134..135 "0"
                Semicolon@135..136 ";"
              Blankspace@136..145 "\n        "
              VariableDeclaration@145..168
                Var@145..148 "var"
                Blankspace@148..149 " "
                Name@149..158
                  Identifier@149..158 "unchanged"
                Colon@158..159 ":"
                Blankspace@159..160 " "
                TypeSpecifier@160..163
                  Path@160..163
                    Identifier@160..163 "i32"
                Blankspace@163..164 " "
                Equal@164..165 "="
                Blankspace@165..166 " "
                Literal@166..167
                  IntLiteral@166..167 "0"
                Semicolon@167..168 ";"
              Blankspace@168..177 "\n        ""#]],
    );
}
