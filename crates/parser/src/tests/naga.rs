use expect_test::expect;

use crate::tests::check;

#[test]
fn extension_not_enabled() {
    check(
        "
        @fragment
        @early_depth_test(force)
        fn fragment(in: FragmentInput) -> @location(0) vec4<f32> { }
        ",
        expect![[r#"
            SourceFile@0..129
              Blankspace@0..9 "\n        "
              FunctionDeclaration@9..120
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
    check(
        "
        enable EARLY_DEPTH_TEST;
        @fragment
        @early_depth_test(force)
        fn fragment(in: FragmentInput) -> @location(0) vec4<f32> { }
        ",
        expect![[r#"
            SourceFile@0..162
              Blankspace@0..9 "\n        "
              EnableDirective@9..33
                Enable@9..15 "enable"
                Blankspace@15..16 " "
                EnableExtensionName@16..32
                  Identifier@16..32 "EARLY_DEPTH_TEST"
                Semicolon@32..33 ";"
              Blankspace@33..42 "\n        "
              FunctionDeclaration@42..153
                FragmentAttribute@42..51
                  AttributeOperator@42..43 "@"
                  Fragment@43..51 "fragment"
                Blankspace@51..60 "\n        "
                EarlyDepthTestAttribute@60..84
                  AttributeOperator@60..61 "@"
                  EarlyDepthTest@61..77 "early_depth_test"
                  ParenthesisLeft@77..78 "("
                  EarlyDepthTestMode@78..83
                    Force@78..83 "force"
                  ParenthesisRight@83..84 ")"
                Blankspace@84..93 "\n        "
                Fn@93..95 "fn"
                Blankspace@95..96 " "
                Name@96..104
                  Identifier@96..104 "fragment"
                FunctionParameters@104..123
                  ParenthesisLeft@104..105 "("
                  Parameter@105..122
                    Name@105..107
                      Identifier@105..107 "in"
                    Colon@107..108 ":"
                    Blankspace@108..109 " "
                    TypeSpecifier@109..122
                      Path@109..122
                        Identifier@109..122 "FragmentInput"
                  ParenthesisRight@122..123 ")"
                Blankspace@123..124 " "
                ReturnType@124..149
                  Arrow@124..126 "->"
                  Blankspace@126..127 " "
                  LocationAttribute@127..139
                    AttributeOperator@127..128 "@"
                    Location@128..136 "location"
                    ParenthesisLeft@136..137 "("
                    Literal@137..138
                      IntLiteral@137..138 "0"
                    ParenthesisRight@138..139 ")"
                  Blankspace@139..140 " "
                  TypeSpecifier@140..149
                    Path@140..144
                      Identifier@140..144 "vec4"
                    TemplateList@144..149
                      TemplateStart@144..145 "<"
                      IdentExpression@145..148
                        Path@145..148
                          Identifier@145..148 "f32"
                      TemplateEnd@148..149 ">"
                Blankspace@149..150 " "
                CompoundStatement@150..153
                  BraceLeft@150..151 "{"
                  Blankspace@151..152 " "
                  BraceRight@152..153 "}"
              Blankspace@153..162 "\n        ""#]],
    );
}

#[test]
fn parse_early_depth_test_greater_equal() {
    check(
        "
        enable EARLY_DEPTH_TEST;
        @fragment
        @early_depth_test(greater_equal)
        fn fragment(in: FragmentInput) -> @location(0) vec4<f32> { }
        ",
        expect![[r#"
            SourceFile@0..170
              Blankspace@0..9 "\n        "
              EnableDirective@9..33
                Enable@9..15 "enable"
                Blankspace@15..16 " "
                EnableExtensionName@16..32
                  Identifier@16..32 "EARLY_DEPTH_TEST"
                Semicolon@32..33 ";"
              Blankspace@33..42 "\n        "
              FunctionDeclaration@42..161
                FragmentAttribute@42..51
                  AttributeOperator@42..43 "@"
                  Fragment@43..51 "fragment"
                Blankspace@51..60 "\n        "
                EarlyDepthTestAttribute@60..92
                  AttributeOperator@60..61 "@"
                  EarlyDepthTest@61..77 "early_depth_test"
                  ParenthesisLeft@77..78 "("
                  EarlyDepthTestMode@78..91
                    GreaterEqual@78..91 "greater_equal"
                  ParenthesisRight@91..92 ")"
                Blankspace@92..101 "\n        "
                Fn@101..103 "fn"
                Blankspace@103..104 " "
                Name@104..112
                  Identifier@104..112 "fragment"
                FunctionParameters@112..131
                  ParenthesisLeft@112..113 "("
                  Parameter@113..130
                    Name@113..115
                      Identifier@113..115 "in"
                    Colon@115..116 ":"
                    Blankspace@116..117 " "
                    TypeSpecifier@117..130
                      Path@117..130
                        Identifier@117..130 "FragmentInput"
                  ParenthesisRight@130..131 ")"
                Blankspace@131..132 " "
                ReturnType@132..157
                  Arrow@132..134 "->"
                  Blankspace@134..135 " "
                  LocationAttribute@135..147
                    AttributeOperator@135..136 "@"
                    Location@136..144 "location"
                    ParenthesisLeft@144..145 "("
                    Literal@145..146
                      IntLiteral@145..146 "0"
                    ParenthesisRight@146..147 ")"
                  Blankspace@147..148 " "
                  TypeSpecifier@148..157
                    Path@148..152
                      Identifier@148..152 "vec4"
                    TemplateList@152..157
                      TemplateStart@152..153 "<"
                      IdentExpression@153..156
                        Path@153..156
                          Identifier@153..156 "f32"
                      TemplateEnd@156..157 ">"
                Blankspace@157..158 " "
                CompoundStatement@158..161
                  BraceLeft@158..159 "{"
                  Blankspace@159..160 " "
                  BraceRight@160..161 "}"
              Blankspace@161..170 "\n        ""#]],
    );
}

#[test]
fn parse_early_depth_test_less_equal() {
    check(
        "
        enable EARLY_DEPTH_TEST;
        @fragment
        @early_depth_test(less_equal)
        fn fragment(in: FragmentInput) -> @location(0) vec4<f32> { }
        ",
        expect![[r#"
            SourceFile@0..167
              Blankspace@0..9 "\n        "
              EnableDirective@9..33
                Enable@9..15 "enable"
                Blankspace@15..16 " "
                EnableExtensionName@16..32
                  Identifier@16..32 "EARLY_DEPTH_TEST"
                Semicolon@32..33 ";"
              Blankspace@33..42 "\n        "
              FunctionDeclaration@42..158
                FragmentAttribute@42..51
                  AttributeOperator@42..43 "@"
                  Fragment@43..51 "fragment"
                Blankspace@51..60 "\n        "
                EarlyDepthTestAttribute@60..89
                  AttributeOperator@60..61 "@"
                  EarlyDepthTest@61..77 "early_depth_test"
                  ParenthesisLeft@77..78 "("
                  EarlyDepthTestMode@78..88
                    LessEqual@78..88 "less_equal"
                  ParenthesisRight@88..89 ")"
                Blankspace@89..98 "\n        "
                Fn@98..100 "fn"
                Blankspace@100..101 " "
                Name@101..109
                  Identifier@101..109 "fragment"
                FunctionParameters@109..128
                  ParenthesisLeft@109..110 "("
                  Parameter@110..127
                    Name@110..112
                      Identifier@110..112 "in"
                    Colon@112..113 ":"
                    Blankspace@113..114 " "
                    TypeSpecifier@114..127
                      Path@114..127
                        Identifier@114..127 "FragmentInput"
                  ParenthesisRight@127..128 ")"
                Blankspace@128..129 " "
                ReturnType@129..154
                  Arrow@129..131 "->"
                  Blankspace@131..132 " "
                  LocationAttribute@132..144
                    AttributeOperator@132..133 "@"
                    Location@133..141 "location"
                    ParenthesisLeft@141..142 "("
                    Literal@142..143
                      IntLiteral@142..143 "0"
                    ParenthesisRight@143..144 ")"
                  Blankspace@144..145 " "
                  TypeSpecifier@145..154
                    Path@145..149
                      Identifier@145..149 "vec4"
                    TemplateList@149..154
                      TemplateStart@149..150 "<"
                      IdentExpression@150..153
                        Path@150..153
                          Identifier@150..153 "f32"
                      TemplateEnd@153..154 ">"
                Blankspace@154..155 " "
                CompoundStatement@155..158
                  BraceLeft@155..156 "{"
                  Blankspace@156..157 " "
                  BraceRight@157..158 "}"
              Blankspace@158..167 "\n        ""#]],
    );
}

#[test]
fn parse_early_depth_test_unchanged() {
    check(
        "
        enable EARLY_DEPTH_TEST;
        @fragment
        @early_depth_test(unchanged)
        fn fragment(in: FragmentInput) -> @location(0) vec4<f32> { }
        ",
        expect![[r#"
            SourceFile@0..166
              Blankspace@0..9 "\n        "
              EnableDirective@9..33
                Enable@9..15 "enable"
                Blankspace@15..16 " "
                EnableExtensionName@16..32
                  Identifier@16..32 "EARLY_DEPTH_TEST"
                Semicolon@32..33 ";"
              Blankspace@33..42 "\n        "
              FunctionDeclaration@42..157
                FragmentAttribute@42..51
                  AttributeOperator@42..43 "@"
                  Fragment@43..51 "fragment"
                Blankspace@51..60 "\n        "
                EarlyDepthTestAttribute@60..88
                  AttributeOperator@60..61 "@"
                  EarlyDepthTest@61..77 "early_depth_test"
                  ParenthesisLeft@77..78 "("
                  EarlyDepthTestMode@78..87
                    Unchanged@78..87 "unchanged"
                  ParenthesisRight@87..88 ")"
                Blankspace@88..97 "\n        "
                Fn@97..99 "fn"
                Blankspace@99..100 " "
                Name@100..108
                  Identifier@100..108 "fragment"
                FunctionParameters@108..127
                  ParenthesisLeft@108..109 "("
                  Parameter@109..126
                    Name@109..111
                      Identifier@109..111 "in"
                    Colon@111..112 ":"
                    Blankspace@112..113 " "
                    TypeSpecifier@113..126
                      Path@113..126
                        Identifier@113..126 "FragmentInput"
                  ParenthesisRight@126..127 ")"
                Blankspace@127..128 " "
                ReturnType@128..153
                  Arrow@128..130 "->"
                  Blankspace@130..131 " "
                  LocationAttribute@131..143
                    AttributeOperator@131..132 "@"
                    Location@132..140 "location"
                    ParenthesisLeft@140..141 "("
                    Literal@141..142
                      IntLiteral@141..142 "0"
                    ParenthesisRight@142..143 ")"
                  Blankspace@143..144 " "
                  TypeSpecifier@144..153
                    Path@144..148
                      Identifier@144..148 "vec4"
                    TemplateList@148..153
                      TemplateStart@148..149 "<"
                      IdentExpression@149..152
                        Path@149..152
                          Identifier@149..152 "f32"
                      TemplateEnd@152..153 ">"
                Blankspace@153..154 " "
                CompoundStatement@154..157
                  BraceLeft@154..155 "{"
                  Blankspace@155..156 " "
                  BraceRight@156..157 "}"
              Blankspace@157..166 "\n        ""#]],
    );
}

#[test]
fn parse_words() {
    check(
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
