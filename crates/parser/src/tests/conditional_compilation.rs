use expect_test::expect;

use crate::tests::check;

#[test]
fn module_compound_if_true() {
    check(
        "
        fn f() {} @if(true) { const_assert true; fn foo() {} struct bar { x: u32 } }
        ",
        expect![[r#"
            SourceFile@0..94
              Blankspace@0..9 "\n        "
              FunctionDeclaration@9..18
                Fn@9..11 "fn"
                Blankspace@11..12 " "
                Name@12..13
                  Identifier@12..13 "f"
                FunctionParameters@13..15
                  ParenthesisLeft@13..14 "("
                  ParenthesisRight@14..15 ")"
                Blankspace@15..16 " "
                CompoundStatement@16..18
                  BraceLeft@16..17 "{"
                  BraceRight@17..18 "}"
              Blankspace@18..19 " "
              AttributeList@19..28
                IfAttribute@19..28
                  AttributeOperator@19..20 "@"
                  If@20..22 "if"
                  ParenthesisLeft@22..23 "("
                  Literal@23..27
                    True@23..27 "true"
                  ParenthesisRight@27..28 ")"
              Blankspace@28..29 " "
              GlobalCompoundDeclaration@29..85
                BraceLeft@29..30 "{"
                Blankspace@30..31 " "
                AssertStatement@31..49
                  ConstantAssert@31..43 "const_assert"
                  Blankspace@43..44 " "
                  Literal@44..48
                    True@44..48 "true"
                  Semicolon@48..49 ";"
                Blankspace@49..50 " "
                FunctionDeclaration@50..61
                  Fn@50..52 "fn"
                  Blankspace@52..53 " "
                  Name@53..56
                    Identifier@53..56 "foo"
                  FunctionParameters@56..58
                    ParenthesisLeft@56..57 "("
                    ParenthesisRight@57..58 ")"
                  Blankspace@58..59 " "
                  CompoundStatement@59..61
                    BraceLeft@59..60 "{"
                    BraceRight@60..61 "}"
                Blankspace@61..62 " "
                StructDeclaration@62..83
                  Struct@62..68 "struct"
                  Blankspace@68..69 " "
                  Name@69..72
                    Identifier@69..72 "bar"
                  Blankspace@72..73 " "
                  StructBody@73..83
                    BraceLeft@73..74 "{"
                    Blankspace@74..75 " "
                    StructMember@75..81
                      Name@75..76
                        Identifier@75..76 "x"
                      Colon@76..77 ":"
                      Blankspace@77..78 " "
                      TypeSpecifier@78..81
                        Path@78..81
                          Identifier@78..81 "u32"
                    Blankspace@81..82 " "
                    BraceRight@82..83 "}"
                Blankspace@83..84 " "
                BraceRight@84..85 "}"
              Blankspace@85..94 "\n        ""#]],
    );
}

#[test]
fn module_compound_if_false() {
    check(
        "
        fn f() {} @if(false) { const_assert true; fn foo() {} struct bar { x: u32 } }
        ",
        expect![[r#"
            SourceFile@0..95
              Blankspace@0..9 "\n        "
              FunctionDeclaration@9..18
                Fn@9..11 "fn"
                Blankspace@11..12 " "
                Name@12..13
                  Identifier@12..13 "f"
                FunctionParameters@13..15
                  ParenthesisLeft@13..14 "("
                  ParenthesisRight@14..15 ")"
                Blankspace@15..16 " "
                CompoundStatement@16..18
                  BraceLeft@16..17 "{"
                  BraceRight@17..18 "}"
              Blankspace@18..19 " "
              AttributeList@19..29
                IfAttribute@19..29
                  AttributeOperator@19..20 "@"
                  If@20..22 "if"
                  ParenthesisLeft@22..23 "("
                  Literal@23..28
                    False@23..28 "false"
                  ParenthesisRight@28..29 ")"
              Blankspace@29..30 " "
              GlobalCompoundDeclaration@30..86
                BraceLeft@30..31 "{"
                Blankspace@31..32 " "
                AssertStatement@32..50
                  ConstantAssert@32..44 "const_assert"
                  Blankspace@44..45 " "
                  Literal@45..49
                    True@45..49 "true"
                  Semicolon@49..50 ";"
                Blankspace@50..51 " "
                FunctionDeclaration@51..62
                  Fn@51..53 "fn"
                  Blankspace@53..54 " "
                  Name@54..57
                    Identifier@54..57 "foo"
                  FunctionParameters@57..59
                    ParenthesisLeft@57..58 "("
                    ParenthesisRight@58..59 ")"
                  Blankspace@59..60 " "
                  CompoundStatement@60..62
                    BraceLeft@60..61 "{"
                    BraceRight@61..62 "}"
                Blankspace@62..63 " "
                StructDeclaration@63..84
                  Struct@63..69 "struct"
                  Blankspace@69..70 " "
                  Name@70..73
                    Identifier@70..73 "bar"
                  Blankspace@73..74 " "
                  StructBody@74..84
                    BraceLeft@74..75 "{"
                    Blankspace@75..76 " "
                    StructMember@76..82
                      Name@76..77
                        Identifier@76..77 "x"
                      Colon@77..78 ":"
                      Blankspace@78..79 " "
                      TypeSpecifier@79..82
                        Path@79..82
                          Identifier@79..82 "u32"
                    Blankspace@82..83 " "
                    BraceRight@83..84 "}"
                Blankspace@84..85 " "
                BraceRight@85..86 "}"
              Blankspace@86..95 "\n        ""#]],
    );
}

#[test]
fn module_compound_if_false_elif_true() {
    check(
        "
        @if(false) { const foo: u32 = 0; } @elif(true) { const bar: u32 = 0; }
        ",
        expect![[r#"
            SourceFile@0..88
              Blankspace@0..9 "\n        "
              AttributeList@9..19
                IfAttribute@9..19
                  AttributeOperator@9..10 "@"
                  If@10..12 "if"
                  ParenthesisLeft@12..13 "("
                  Literal@13..18
                    False@13..18 "false"
                  ParenthesisRight@18..19 ")"
              Blankspace@19..20 " "
              GlobalCompoundDeclaration@20..43
                BraceLeft@20..21 "{"
                Blankspace@21..22 " "
                ConstantDeclaration@22..41
                  Const@22..27 "const"
                  Blankspace@27..28 " "
                  Name@28..31
                    Identifier@28..31 "foo"
                  Colon@31..32 ":"
                  Blankspace@32..33 " "
                  TypeSpecifier@33..36
                    Path@33..36
                      Identifier@33..36 "u32"
                  Blankspace@36..37 " "
                  Equal@37..38 "="
                  Blankspace@38..39 " "
                  Literal@39..40
                    IntLiteral@39..40 "0"
                  Semicolon@40..41 ";"
                Blankspace@41..42 " "
                BraceRight@42..43 "}"
              Blankspace@43..44 " "
              AttributeList@44..55
                OtherAttribute@44..55
                  AttributeOperator@44..45 "@"
                  Identifier@45..49 "elif"
                  Arguments@49..55
                    ParenthesisLeft@49..50 "("
                    Literal@50..54
                      True@50..54 "true"
                    ParenthesisRight@54..55 ")"
              Blankspace@55..56 " "
              GlobalCompoundDeclaration@56..79
                BraceLeft@56..57 "{"
                Blankspace@57..58 " "
                ConstantDeclaration@58..77
                  Const@58..63 "const"
                  Blankspace@63..64 " "
                  Name@64..67
                    Identifier@64..67 "bar"
                  Colon@67..68 ":"
                  Blankspace@68..69 " "
                  TypeSpecifier@69..72
                    Path@69..72
                      Identifier@69..72 "u32"
                  Blankspace@72..73 " "
                  Equal@73..74 "="
                  Blankspace@74..75 " "
                  Literal@75..76
                    IntLiteral@75..76 "0"
                  Semicolon@76..77 ";"
                Blankspace@77..78 " "
                BraceRight@78..79 "}"
              Blankspace@79..88 "\n        ""#]],
    );
}

#[test]
fn module_compound_if_true_compound_elif_true() {
    check(
        "
        @if(true) { const foo: u32 = 0; } @elif(true) { const bar: u32 = 0; }
        ",
        expect![[r#"
            SourceFile@0..87
              Blankspace@0..9 "\n        "
              AttributeList@9..18
                IfAttribute@9..18
                  AttributeOperator@9..10 "@"
                  If@10..12 "if"
                  ParenthesisLeft@12..13 "("
                  Literal@13..17
                    True@13..17 "true"
                  ParenthesisRight@17..18 ")"
              Blankspace@18..19 " "
              GlobalCompoundDeclaration@19..42
                BraceLeft@19..20 "{"
                Blankspace@20..21 " "
                ConstantDeclaration@21..40
                  Const@21..26 "const"
                  Blankspace@26..27 " "
                  Name@27..30
                    Identifier@27..30 "foo"
                  Colon@30..31 ":"
                  Blankspace@31..32 " "
                  TypeSpecifier@32..35
                    Path@32..35
                      Identifier@32..35 "u32"
                  Blankspace@35..36 " "
                  Equal@36..37 "="
                  Blankspace@37..38 " "
                  Literal@38..39
                    IntLiteral@38..39 "0"
                  Semicolon@39..40 ";"
                Blankspace@40..41 " "
                BraceRight@41..42 "}"
              Blankspace@42..43 " "
              AttributeList@43..54
                OtherAttribute@43..54
                  AttributeOperator@43..44 "@"
                  Identifier@44..48 "elif"
                  Arguments@48..54
                    ParenthesisLeft@48..49 "("
                    Literal@49..53
                      True@49..53 "true"
                    ParenthesisRight@53..54 ")"
              Blankspace@54..55 " "
              GlobalCompoundDeclaration@55..78
                BraceLeft@55..56 "{"
                Blankspace@56..57 " "
                ConstantDeclaration@57..76
                  Const@57..62 "const"
                  Blankspace@62..63 " "
                  Name@63..66
                    Identifier@63..66 "bar"
                  Colon@66..67 ":"
                  Blankspace@67..68 " "
                  TypeSpecifier@68..71
                    Path@68..71
                      Identifier@68..71 "u32"
                  Blankspace@71..72 " "
                  Equal@72..73 "="
                  Blankspace@73..74 " "
                  Literal@74..75
                    IntLiteral@74..75 "0"
                  Semicolon@75..76 ";"
                Blankspace@76..77 " "
                BraceRight@77..78 "}"
              Blankspace@78..87 "\n        ""#]],
    );
}

#[test]
fn function_compound_nested() {
    check(
        "
        fn foo() { @if(true) { var x = 0; { @if(true) x++; } @if(true) { x--; } } }
        ",
        expect![[r#"
            SourceFile@0..93
              Blankspace@0..9 "\n        "
              FunctionDeclaration@9..84
                Fn@9..11 "fn"
                Blankspace@11..12 " "
                Name@12..15
                  Identifier@12..15 "foo"
                FunctionParameters@15..17
                  ParenthesisLeft@15..16 "("
                  ParenthesisRight@16..17 ")"
                Blankspace@17..18 " "
                CompoundStatement@18..84
                  BraceLeft@18..19 "{"
                  Blankspace@19..20 " "
                  AttributeList@20..29
                    IfAttribute@20..29
                      AttributeOperator@20..21 "@"
                      If@21..23 "if"
                      ParenthesisLeft@23..24 "("
                      Literal@24..28
                        True@24..28 "true"
                      ParenthesisRight@28..29 ")"
                  Blankspace@29..30 " "
                  CompoundStatement@30..82
                    BraceLeft@30..31 "{"
                    Blankspace@31..32 " "
                    VariableDeclaration@32..42
                      Var@32..35 "var"
                      Blankspace@35..36 " "
                      Name@36..37
                        Identifier@36..37 "x"
                      Blankspace@37..38 " "
                      Equal@38..39 "="
                      Blankspace@39..40 " "
                      Literal@40..41
                        IntLiteral@40..41 "0"
                      Semicolon@41..42 ";"
                    Blankspace@42..43 " "
                    CompoundStatement@43..61
                      BraceLeft@43..44 "{"
                      Blankspace@44..45 " "
                      AttributeList@45..54
                        IfAttribute@45..54
                          AttributeOperator@45..46 "@"
                          If@46..48 "if"
                          ParenthesisLeft@48..49 "("
                          Literal@49..53
                            True@49..53 "true"
                          ParenthesisRight@53..54 ")"
                      Blankspace@54..55 " "
                      IncrementDecrementStatement@55..59
                        IdentExpression@55..56
                          Path@55..56
                            Identifier@55..56 "x"
                        PlusPlus@56..58 "++"
                        Semicolon@58..59 ";"
                      Blankspace@59..60 " "
                      BraceRight@60..61 "}"
                    Blankspace@61..62 " "
                    AttributeList@62..71
                      IfAttribute@62..71
                        AttributeOperator@62..63 "@"
                        If@63..65 "if"
                        ParenthesisLeft@65..66 "("
                        Literal@66..70
                          True@66..70 "true"
                        ParenthesisRight@70..71 ")"
                    Blankspace@71..72 " "
                    CompoundStatement@72..80
                      BraceLeft@72..73 "{"
                      Blankspace@73..74 " "
                      IncrementDecrementStatement@74..78
                        IdentExpression@74..75
                          Path@74..75
                            Identifier@74..75 "x"
                        MinusMinus@75..77 "--"
                        Semicolon@77..78 ";"
                      Blankspace@78..79 " "
                      BraceRight@79..80 "}"
                    Blankspace@80..81 " "
                    BraceRight@81..82 "}"
                  Blankspace@82..83 " "
                  BraceRight@83..84 "}"
              Blankspace@84..93 "\n        ""#]],
    );
}

#[test]
fn module_if_false_compound_elif_true() {
    check(
        "@if(false) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; }",
        expect![[r#"
            SourceFile@0..66
              AttributeList@0..10
                IfAttribute@0..10
                  AttributeOperator@0..1 "@"
                  If@1..3 "if"
                  ParenthesisLeft@3..4 "("
                  Literal@4..9
                    False@4..9 "false"
                  ParenthesisRight@9..10 ")"
              Blankspace@10..11 " "
              ConstantDeclaration@11..30
                Const@11..16 "const"
                Blankspace@16..17 " "
                Name@17..20
                  Identifier@17..20 "foo"
                Colon@20..21 ":"
                Blankspace@21..22 " "
                TypeSpecifier@22..25
                  Path@22..25
                    Identifier@22..25 "u32"
                Blankspace@25..26 " "
                Equal@26..27 "="
                Blankspace@27..28 " "
                Literal@28..29
                  IntLiteral@28..29 "0"
                Semicolon@29..30 ";"
              Blankspace@30..31 " "
              AttributeList@31..42
                OtherAttribute@31..42
                  AttributeOperator@31..32 "@"
                  Identifier@32..36 "elif"
                  Arguments@36..42
                    ParenthesisLeft@36..37 "("
                    Literal@37..41
                      True@37..41 "true"
                    ParenthesisRight@41..42 ")"
              Blankspace@42..43 " "
              GlobalCompoundDeclaration@43..66
                BraceLeft@43..44 "{"
                Blankspace@44..45 " "
                ConstantDeclaration@45..64
                  Const@45..50 "const"
                  Blankspace@50..51 " "
                  Name@51..54
                    Identifier@51..54 "bar"
                  Colon@54..55 ":"
                  Blankspace@55..56 " "
                  TypeSpecifier@56..59
                    Path@56..59
                      Identifier@56..59 "u32"
                  Blankspace@59..60 " "
                  Equal@60..61 "="
                  Blankspace@61..62 " "
                  Literal@62..63
                    IntLiteral@62..63 "0"
                  Semicolon@63..64 ";"
                Blankspace@64..65 " "
                BraceRight@65..66 "}""#]],
    );
}

#[test]
fn module_if_true_compound_elif_true() {
    check(
        "@if(true) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; }",
        expect![[r#"
            SourceFile@0..65
              AttributeList@0..9
                IfAttribute@0..9
                  AttributeOperator@0..1 "@"
                  If@1..3 "if"
                  ParenthesisLeft@3..4 "("
                  Literal@4..8
                    True@4..8 "true"
                  ParenthesisRight@8..9 ")"
              Blankspace@9..10 " "
              ConstantDeclaration@10..29
                Const@10..15 "const"
                Blankspace@15..16 " "
                Name@16..19
                  Identifier@16..19 "foo"
                Colon@19..20 ":"
                Blankspace@20..21 " "
                TypeSpecifier@21..24
                  Path@21..24
                    Identifier@21..24 "u32"
                Blankspace@24..25 " "
                Equal@25..26 "="
                Blankspace@26..27 " "
                Literal@27..28
                  IntLiteral@27..28 "0"
                Semicolon@28..29 ";"
              Blankspace@29..30 " "
              AttributeList@30..41
                OtherAttribute@30..41
                  AttributeOperator@30..31 "@"
                  Identifier@31..35 "elif"
                  Arguments@35..41
                    ParenthesisLeft@35..36 "("
                    Literal@36..40
                      True@36..40 "true"
                    ParenthesisRight@40..41 ")"
              Blankspace@41..42 " "
              GlobalCompoundDeclaration@42..65
                BraceLeft@42..43 "{"
                Blankspace@43..44 " "
                ConstantDeclaration@44..63
                  Const@44..49 "const"
                  Blankspace@49..50 " "
                  Name@50..53
                    Identifier@50..53 "bar"
                  Colon@53..54 ":"
                  Blankspace@54..55 " "
                  TypeSpecifier@55..58
                    Path@55..58
                      Identifier@55..58 "u32"
                  Blankspace@58..59 " "
                  Equal@59..60 "="
                  Blankspace@60..61 " "
                  Literal@61..62
                    IntLiteral@61..62 "0"
                  Semicolon@62..63 ";"
                Blankspace@63..64 " "
                BraceRight@64..65 "}""#]],
    );
}

#[test]
fn module_compound_else_hit() {
    check(
        "@if(false) { const foo: u32 = 0; } @elif(false) { const bar: u32 = 0; } @else { const baz: u32 = 0; }",
        expect![[r#"
            SourceFile@0..101
              AttributeList@0..10
                IfAttribute@0..10
                  AttributeOperator@0..1 "@"
                  If@1..3 "if"
                  ParenthesisLeft@3..4 "("
                  Literal@4..9
                    False@4..9 "false"
                  ParenthesisRight@9..10 ")"
              Blankspace@10..11 " "
              GlobalCompoundDeclaration@11..34
                BraceLeft@11..12 "{"
                Blankspace@12..13 " "
                ConstantDeclaration@13..32
                  Const@13..18 "const"
                  Blankspace@18..19 " "
                  Name@19..22
                    Identifier@19..22 "foo"
                  Colon@22..23 ":"
                  Blankspace@23..24 " "
                  TypeSpecifier@24..27
                    Path@24..27
                      Identifier@24..27 "u32"
                  Blankspace@27..28 " "
                  Equal@28..29 "="
                  Blankspace@29..30 " "
                  Literal@30..31
                    IntLiteral@30..31 "0"
                  Semicolon@31..32 ";"
                Blankspace@32..33 " "
                BraceRight@33..34 "}"
              Blankspace@34..35 " "
              AttributeList@35..47
                OtherAttribute@35..47
                  AttributeOperator@35..36 "@"
                  Identifier@36..40 "elif"
                  Arguments@40..47
                    ParenthesisLeft@40..41 "("
                    Literal@41..46
                      False@41..46 "false"
                    ParenthesisRight@46..47 ")"
              Blankspace@47..48 " "
              GlobalCompoundDeclaration@48..71
                BraceLeft@48..49 "{"
                Blankspace@49..50 " "
                ConstantDeclaration@50..69
                  Const@50..55 "const"
                  Blankspace@55..56 " "
                  Name@56..59
                    Identifier@56..59 "bar"
                  Colon@59..60 ":"
                  Blankspace@60..61 " "
                  TypeSpecifier@61..64
                    Path@61..64
                      Identifier@61..64 "u32"
                  Blankspace@64..65 " "
                  Equal@65..66 "="
                  Blankspace@66..67 " "
                  Literal@67..68
                    IntLiteral@67..68 "0"
                  Semicolon@68..69 ";"
                Blankspace@69..70 " "
                BraceRight@70..71 "}"
              Blankspace@71..72 " "
              AttributeList@72..77
                ElseAttribute@72..77
                  AttributeOperator@72..73 "@"
                  Else@73..77 "else"
              Blankspace@77..78 " "
              GlobalCompoundDeclaration@78..101
                BraceLeft@78..79 "{"
                Blankspace@79..80 " "
                ConstantDeclaration@80..99
                  Const@80..85 "const"
                  Blankspace@85..86 " "
                  Name@86..89
                    Identifier@86..89 "baz"
                  Colon@89..90 ":"
                  Blankspace@90..91 " "
                  TypeSpecifier@91..94
                    Path@91..94
                      Identifier@91..94 "u32"
                  Blankspace@94..95 " "
                  Equal@95..96 "="
                  Blankspace@96..97 " "
                  Literal@97..98
                    IntLiteral@97..98 "0"
                  Semicolon@98..99 ";"
                Blankspace@99..100 " "
                BraceRight@100..101 "}""#]],
    );
}

#[test]
fn module_compound_else_skipped() {
    check(
        "@if(false) { const foo: u32 = 0; } @elif(true) { const bar: u32 = 0; } @else { const baz: u32 = 0; }",
        expect![[r#"
            SourceFile@0..100
              AttributeList@0..10
                IfAttribute@0..10
                  AttributeOperator@0..1 "@"
                  If@1..3 "if"
                  ParenthesisLeft@3..4 "("
                  Literal@4..9
                    False@4..9 "false"
                  ParenthesisRight@9..10 ")"
              Blankspace@10..11 " "
              GlobalCompoundDeclaration@11..34
                BraceLeft@11..12 "{"
                Blankspace@12..13 " "
                ConstantDeclaration@13..32
                  Const@13..18 "const"
                  Blankspace@18..19 " "
                  Name@19..22
                    Identifier@19..22 "foo"
                  Colon@22..23 ":"
                  Blankspace@23..24 " "
                  TypeSpecifier@24..27
                    Path@24..27
                      Identifier@24..27 "u32"
                  Blankspace@27..28 " "
                  Equal@28..29 "="
                  Blankspace@29..30 " "
                  Literal@30..31
                    IntLiteral@30..31 "0"
                  Semicolon@31..32 ";"
                Blankspace@32..33 " "
                BraceRight@33..34 "}"
              Blankspace@34..35 " "
              AttributeList@35..46
                OtherAttribute@35..46
                  AttributeOperator@35..36 "@"
                  Identifier@36..40 "elif"
                  Arguments@40..46
                    ParenthesisLeft@40..41 "("
                    Literal@41..45
                      True@41..45 "true"
                    ParenthesisRight@45..46 ")"
              Blankspace@46..47 " "
              GlobalCompoundDeclaration@47..70
                BraceLeft@47..48 "{"
                Blankspace@48..49 " "
                ConstantDeclaration@49..68
                  Const@49..54 "const"
                  Blankspace@54..55 " "
                  Name@55..58
                    Identifier@55..58 "bar"
                  Colon@58..59 ":"
                  Blankspace@59..60 " "
                  TypeSpecifier@60..63
                    Path@60..63
                      Identifier@60..63 "u32"
                  Blankspace@63..64 " "
                  Equal@64..65 "="
                  Blankspace@65..66 " "
                  Literal@66..67
                    IntLiteral@66..67 "0"
                  Semicolon@67..68 ";"
                Blankspace@68..69 " "
                BraceRight@69..70 "}"
              Blankspace@70..71 " "
              AttributeList@71..76
                ElseAttribute@71..76
                  AttributeOperator@71..72 "@"
                  Else@72..76 "else"
              Blankspace@76..77 " "
              GlobalCompoundDeclaration@77..100
                BraceLeft@77..78 "{"
                Blankspace@78..79 " "
                ConstantDeclaration@79..98
                  Const@79..84 "const"
                  Blankspace@84..85 " "
                  Name@85..88
                    Identifier@85..88 "baz"
                  Colon@88..89 ":"
                  Blankspace@89..90 " "
                  TypeSpecifier@90..93
                    Path@90..93
                      Identifier@90..93 "u32"
                  Blankspace@93..94 " "
                  Equal@94..95 "="
                  Blankspace@95..96 " "
                  Literal@96..97
                    IntLiteral@96..97 "0"
                  Semicolon@97..98 ";"
                Blankspace@98..99 " "
                BraceRight@99..100 "}""#]],
    );
}

#[test]
fn module_compound_noop() {
    check("{ fn foo() {} }", expect![[r#"
        SourceFile@0..15
          GlobalCompoundDeclaration@0..15
            BraceLeft@0..1 "{"
            Blankspace@1..2 " "
            FunctionDeclaration@2..13
              Fn@2..4 "fn"
              Blankspace@4..5 " "
              Name@5..8
                Identifier@5..8 "foo"
              FunctionParameters@8..10
                ParenthesisLeft@8..9 "("
                ParenthesisRight@9..10 ")"
              Blankspace@10..11 " "
              CompoundStatement@11..13
                BraceLeft@11..12 "{"
                BraceRight@12..13 "}"
            Blankspace@13..14 " "
            BraceRight@14..15 "}""#]]);
}

#[test]
fn module_compound_nested_noop() {
    check(
        "@if (true) { fn foo() {} { @if(true) fn bar() {} } }",
        expect![[r#"
            SourceFile@0..52
              AttributeList@0..10
                IfAttribute@0..10
                  AttributeOperator@0..1 "@"
                  If@1..3 "if"
                  Blankspace@3..4 " "
                  ParenthesisLeft@4..5 "("
                  Literal@5..9
                    True@5..9 "true"
                  ParenthesisRight@9..10 ")"
              Blankspace@10..11 " "
              GlobalCompoundDeclaration@11..52
                BraceLeft@11..12 "{"
                Blankspace@12..13 " "
                FunctionDeclaration@13..24
                  Fn@13..15 "fn"
                  Blankspace@15..16 " "
                  Name@16..19
                    Identifier@16..19 "foo"
                  FunctionParameters@19..21
                    ParenthesisLeft@19..20 "("
                    ParenthesisRight@20..21 ")"
                  Blankspace@21..22 " "
                  CompoundStatement@22..24
                    BraceLeft@22..23 "{"
                    BraceRight@23..24 "}"
                Blankspace@24..25 " "
                GlobalCompoundDeclaration@25..50
                  BraceLeft@25..26 "{"
                  Blankspace@26..27 " "
                  AttributeList@27..36
                    IfAttribute@27..36
                      AttributeOperator@27..28 "@"
                      If@28..30 "if"
                      ParenthesisLeft@30..31 "("
                      Literal@31..35
                        True@31..35 "true"
                      ParenthesisRight@35..36 ")"
                  Blankspace@36..37 " "
                  FunctionDeclaration@37..48
                    Fn@37..39 "fn"
                    Blankspace@39..40 " "
                    Name@40..43
                      Identifier@40..43 "bar"
                    FunctionParameters@43..45
                      ParenthesisLeft@43..44 "("
                      ParenthesisRight@44..45 ")"
                    Blankspace@45..46 " "
                    CompoundStatement@46..48
                      BraceLeft@46..47 "{"
                      BraceRight@47..48 "}"
                  Blankspace@48..49 " "
                  BraceRight@49..50 "}"
                Blankspace@50..51 " "
                BraceRight@51..52 "}""#]],
    );
}

#[test]
fn module_compound_nested_if() {
    check(
        "@if(true) { fn foo() {} @if(true) { fn bar() {} } }",
        expect![[r#"
            SourceFile@0..51
              AttributeList@0..9
                IfAttribute@0..9
                  AttributeOperator@0..1 "@"
                  If@1..3 "if"
                  ParenthesisLeft@3..4 "("
                  Literal@4..8
                    True@4..8 "true"
                  ParenthesisRight@8..9 ")"
              Blankspace@9..10 " "
              GlobalCompoundDeclaration@10..51
                BraceLeft@10..11 "{"
                Blankspace@11..12 " "
                FunctionDeclaration@12..23
                  Fn@12..14 "fn"
                  Blankspace@14..15 " "
                  Name@15..18
                    Identifier@15..18 "foo"
                  FunctionParameters@18..20
                    ParenthesisLeft@18..19 "("
                    ParenthesisRight@19..20 ")"
                  Blankspace@20..21 " "
                  CompoundStatement@21..23
                    BraceLeft@21..22 "{"
                    BraceRight@22..23 "}"
                Blankspace@23..24 " "
                AttributeList@24..33
                  IfAttribute@24..33
                    AttributeOperator@24..25 "@"
                    If@25..27 "if"
                    ParenthesisLeft@27..28 "("
                    Literal@28..32
                      True@28..32 "true"
                    ParenthesisRight@32..33 ")"
                Blankspace@33..34 " "
                GlobalCompoundDeclaration@34..49
                  BraceLeft@34..35 "{"
                  Blankspace@35..36 " "
                  FunctionDeclaration@36..47
                    Fn@36..38 "fn"
                    Blankspace@38..39 " "
                    Name@39..42
                      Identifier@39..42 "bar"
                    FunctionParameters@42..44
                      ParenthesisLeft@42..43 "("
                      ParenthesisRight@43..44 ")"
                    Blankspace@44..45 " "
                    CompoundStatement@45..47
                      BraceLeft@45..46 "{"
                      BraceRight@46..47 "}"
                  Blankspace@47..48 " "
                  BraceRight@48..49 "}"
                Blankspace@49..50 " "
                BraceRight@50..51 "}""#]],
    );
}

#[test]
fn module_compound_nested_elif_hit() {
    check(
        "@if(true) { @if(false) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; } } @else { const baz: u32 = 0; }",
        expect![[r#"
            SourceFile@0..110
              AttributeList@0..9
                IfAttribute@0..9
                  AttributeOperator@0..1 "@"
                  If@1..3 "if"
                  ParenthesisLeft@3..4 "("
                  Literal@4..8
                    True@4..8 "true"
                  ParenthesisRight@8..9 ")"
              Blankspace@9..10 " "
              GlobalCompoundDeclaration@10..80
                BraceLeft@10..11 "{"
                Blankspace@11..12 " "
                AttributeList@12..22
                  IfAttribute@12..22
                    AttributeOperator@12..13 "@"
                    If@13..15 "if"
                    ParenthesisLeft@15..16 "("
                    Literal@16..21
                      False@16..21 "false"
                    ParenthesisRight@21..22 ")"
                Blankspace@22..23 " "
                ConstantDeclaration@23..42
                  Const@23..28 "const"
                  Blankspace@28..29 " "
                  Name@29..32
                    Identifier@29..32 "foo"
                  Colon@32..33 ":"
                  Blankspace@33..34 " "
                  TypeSpecifier@34..37
                    Path@34..37
                      Identifier@34..37 "u32"
                  Blankspace@37..38 " "
                  Equal@38..39 "="
                  Blankspace@39..40 " "
                  Literal@40..41
                    IntLiteral@40..41 "0"
                  Semicolon@41..42 ";"
                Blankspace@42..43 " "
                AttributeList@43..54
                  OtherAttribute@43..54
                    AttributeOperator@43..44 "@"
                    Identifier@44..48 "elif"
                    Arguments@48..54
                      ParenthesisLeft@48..49 "("
                      Literal@49..53
                        True@49..53 "true"
                      ParenthesisRight@53..54 ")"
                Blankspace@54..55 " "
                GlobalCompoundDeclaration@55..78
                  BraceLeft@55..56 "{"
                  Blankspace@56..57 " "
                  ConstantDeclaration@57..76
                    Const@57..62 "const"
                    Blankspace@62..63 " "
                    Name@63..66
                      Identifier@63..66 "bar"
                    Colon@66..67 ":"
                    Blankspace@67..68 " "
                    TypeSpecifier@68..71
                      Path@68..71
                        Identifier@68..71 "u32"
                    Blankspace@71..72 " "
                    Equal@72..73 "="
                    Blankspace@73..74 " "
                    Literal@74..75
                      IntLiteral@74..75 "0"
                    Semicolon@75..76 ";"
                  Blankspace@76..77 " "
                  BraceRight@77..78 "}"
                Blankspace@78..79 " "
                BraceRight@79..80 "}"
              Blankspace@80..81 " "
              AttributeList@81..86
                ElseAttribute@81..86
                  AttributeOperator@81..82 "@"
                  Else@82..86 "else"
              Blankspace@86..87 " "
              GlobalCompoundDeclaration@87..110
                BraceLeft@87..88 "{"
                Blankspace@88..89 " "
                ConstantDeclaration@89..108
                  Const@89..94 "const"
                  Blankspace@94..95 " "
                  Name@95..98
                    Identifier@95..98 "baz"
                  Colon@98..99 ":"
                  Blankspace@99..100 " "
                  TypeSpecifier@100..103
                    Path@100..103
                      Identifier@100..103 "u32"
                  Blankspace@103..104 " "
                  Equal@104..105 "="
                  Blankspace@105..106 " "
                  Literal@106..107
                    IntLiteral@106..107 "0"
                  Semicolon@107..108 ";"
                Blankspace@108..109 " "
                BraceRight@109..110 "}""#]],
    );
}

#[test]
fn module_compound_nested_elif_skipped() {
    check(
        "@if(true) { @if(true) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; } } @else { const baz: u32 = 0; }",
        expect![[r#"
            SourceFile@0..109
              AttributeList@0..9
                IfAttribute@0..9
                  AttributeOperator@0..1 "@"
                  If@1..3 "if"
                  ParenthesisLeft@3..4 "("
                  Literal@4..8
                    True@4..8 "true"
                  ParenthesisRight@8..9 ")"
              Blankspace@9..10 " "
              GlobalCompoundDeclaration@10..79
                BraceLeft@10..11 "{"
                Blankspace@11..12 " "
                AttributeList@12..21
                  IfAttribute@12..21
                    AttributeOperator@12..13 "@"
                    If@13..15 "if"
                    ParenthesisLeft@15..16 "("
                    Literal@16..20
                      True@16..20 "true"
                    ParenthesisRight@20..21 ")"
                Blankspace@21..22 " "
                ConstantDeclaration@22..41
                  Const@22..27 "const"
                  Blankspace@27..28 " "
                  Name@28..31
                    Identifier@28..31 "foo"
                  Colon@31..32 ":"
                  Blankspace@32..33 " "
                  TypeSpecifier@33..36
                    Path@33..36
                      Identifier@33..36 "u32"
                  Blankspace@36..37 " "
                  Equal@37..38 "="
                  Blankspace@38..39 " "
                  Literal@39..40
                    IntLiteral@39..40 "0"
                  Semicolon@40..41 ";"
                Blankspace@41..42 " "
                AttributeList@42..53
                  OtherAttribute@42..53
                    AttributeOperator@42..43 "@"
                    Identifier@43..47 "elif"
                    Arguments@47..53
                      ParenthesisLeft@47..48 "("
                      Literal@48..52
                        True@48..52 "true"
                      ParenthesisRight@52..53 ")"
                Blankspace@53..54 " "
                GlobalCompoundDeclaration@54..77
                  BraceLeft@54..55 "{"
                  Blankspace@55..56 " "
                  ConstantDeclaration@56..75
                    Const@56..61 "const"
                    Blankspace@61..62 " "
                    Name@62..65
                      Identifier@62..65 "bar"
                    Colon@65..66 ":"
                    Blankspace@66..67 " "
                    TypeSpecifier@67..70
                      Path@67..70
                        Identifier@67..70 "u32"
                    Blankspace@70..71 " "
                    Equal@71..72 "="
                    Blankspace@72..73 " "
                    Literal@73..74
                      IntLiteral@73..74 "0"
                    Semicolon@74..75 ";"
                  Blankspace@75..76 " "
                  BraceRight@76..77 "}"
                Blankspace@77..78 " "
                BraceRight@78..79 "}"
              Blankspace@79..80 " "
              AttributeList@80..85
                ElseAttribute@80..85
                  AttributeOperator@80..81 "@"
                  Else@81..85 "else"
              Blankspace@85..86 " "
              GlobalCompoundDeclaration@86..109
                BraceLeft@86..87 "{"
                Blankspace@87..88 " "
                ConstantDeclaration@88..107
                  Const@88..93 "const"
                  Blankspace@93..94 " "
                  Name@94..97
                    Identifier@94..97 "baz"
                  Colon@97..98 ":"
                  Blankspace@98..99 " "
                  TypeSpecifier@99..102
                    Path@99..102
                      Identifier@99..102 "u32"
                  Blankspace@102..103 " "
                  Equal@103..104 "="
                  Blankspace@104..105 " "
                  Literal@105..106
                    IntLiteral@105..106 "0"
                  Semicolon@106..107 ";"
                Blankspace@107..108 " "
                BraceRight@108..109 "}""#]],
    );
}

#[test]
fn module_compound_nested_else_hit() {
    check(
        "@if(true) { @if(false) const foo: u32 = 0; @else { const bar: u32 = 0; } } @else { const baz: u32 = 0; }",
        expect![[r#"
            SourceFile@0..104
              AttributeList@0..9
                IfAttribute@0..9
                  AttributeOperator@0..1 "@"
                  If@1..3 "if"
                  ParenthesisLeft@3..4 "("
                  Literal@4..8
                    True@4..8 "true"
                  ParenthesisRight@8..9 ")"
              Blankspace@9..10 " "
              GlobalCompoundDeclaration@10..74
                BraceLeft@10..11 "{"
                Blankspace@11..12 " "
                AttributeList@12..22
                  IfAttribute@12..22
                    AttributeOperator@12..13 "@"
                    If@13..15 "if"
                    ParenthesisLeft@15..16 "("
                    Literal@16..21
                      False@16..21 "false"
                    ParenthesisRight@21..22 ")"
                Blankspace@22..23 " "
                ConstantDeclaration@23..42
                  Const@23..28 "const"
                  Blankspace@28..29 " "
                  Name@29..32
                    Identifier@29..32 "foo"
                  Colon@32..33 ":"
                  Blankspace@33..34 " "
                  TypeSpecifier@34..37
                    Path@34..37
                      Identifier@34..37 "u32"
                  Blankspace@37..38 " "
                  Equal@38..39 "="
                  Blankspace@39..40 " "
                  Literal@40..41
                    IntLiteral@40..41 "0"
                  Semicolon@41..42 ";"
                Blankspace@42..43 " "
                AttributeList@43..48
                  ElseAttribute@43..48
                    AttributeOperator@43..44 "@"
                    Else@44..48 "else"
                Blankspace@48..49 " "
                GlobalCompoundDeclaration@49..72
                  BraceLeft@49..50 "{"
                  Blankspace@50..51 " "
                  ConstantDeclaration@51..70
                    Const@51..56 "const"
                    Blankspace@56..57 " "
                    Name@57..60
                      Identifier@57..60 "bar"
                    Colon@60..61 ":"
                    Blankspace@61..62 " "
                    TypeSpecifier@62..65
                      Path@62..65
                        Identifier@62..65 "u32"
                    Blankspace@65..66 " "
                    Equal@66..67 "="
                    Blankspace@67..68 " "
                    Literal@68..69
                      IntLiteral@68..69 "0"
                    Semicolon@69..70 ";"
                  Blankspace@70..71 " "
                  BraceRight@71..72 "}"
                Blankspace@72..73 " "
                BraceRight@73..74 "}"
              Blankspace@74..75 " "
              AttributeList@75..80
                ElseAttribute@75..80
                  AttributeOperator@75..76 "@"
                  Else@76..80 "else"
              Blankspace@80..81 " "
              GlobalCompoundDeclaration@81..104
                BraceLeft@81..82 "{"
                Blankspace@82..83 " "
                ConstantDeclaration@83..102
                  Const@83..88 "const"
                  Blankspace@88..89 " "
                  Name@89..92
                    Identifier@89..92 "baz"
                  Colon@92..93 ":"
                  Blankspace@93..94 " "
                  TypeSpecifier@94..97
                    Path@94..97
                      Identifier@94..97 "u32"
                  Blankspace@97..98 " "
                  Equal@98..99 "="
                  Blankspace@99..100 " "
                  Literal@100..101
                    IntLiteral@100..101 "0"
                  Semicolon@101..102 ";"
                Blankspace@102..103 " "
                BraceRight@103..104 "}""#]],
    );
}

#[test]
fn module_compound_nested_else_skipped() {
    check(
        "@if(true) { @if(true) const foo: u32 = 0; @else { const bar: u32 = 0; } } @else { const baz: u32 = 0; }",
        expect![[r#"
            SourceFile@0..103
              AttributeList@0..9
                IfAttribute@0..9
                  AttributeOperator@0..1 "@"
                  If@1..3 "if"
                  ParenthesisLeft@3..4 "("
                  Literal@4..8
                    True@4..8 "true"
                  ParenthesisRight@8..9 ")"
              Blankspace@9..10 " "
              GlobalCompoundDeclaration@10..73
                BraceLeft@10..11 "{"
                Blankspace@11..12 " "
                AttributeList@12..21
                  IfAttribute@12..21
                    AttributeOperator@12..13 "@"
                    If@13..15 "if"
                    ParenthesisLeft@15..16 "("
                    Literal@16..20
                      True@16..20 "true"
                    ParenthesisRight@20..21 ")"
                Blankspace@21..22 " "
                ConstantDeclaration@22..41
                  Const@22..27 "const"
                  Blankspace@27..28 " "
                  Name@28..31
                    Identifier@28..31 "foo"
                  Colon@31..32 ":"
                  Blankspace@32..33 " "
                  TypeSpecifier@33..36
                    Path@33..36
                      Identifier@33..36 "u32"
                  Blankspace@36..37 " "
                  Equal@37..38 "="
                  Blankspace@38..39 " "
                  Literal@39..40
                    IntLiteral@39..40 "0"
                  Semicolon@40..41 ";"
                Blankspace@41..42 " "
                AttributeList@42..47
                  ElseAttribute@42..47
                    AttributeOperator@42..43 "@"
                    Else@43..47 "else"
                Blankspace@47..48 " "
                GlobalCompoundDeclaration@48..71
                  BraceLeft@48..49 "{"
                  Blankspace@49..50 " "
                  ConstantDeclaration@50..69
                    Const@50..55 "const"
                    Blankspace@55..56 " "
                    Name@56..59
                      Identifier@56..59 "bar"
                    Colon@59..60 ":"
                    Blankspace@60..61 " "
                    TypeSpecifier@61..64
                      Path@61..64
                        Identifier@61..64 "u32"
                    Blankspace@64..65 " "
                    Equal@65..66 "="
                    Blankspace@66..67 " "
                    Literal@67..68
                      IntLiteral@67..68 "0"
                    Semicolon@68..69 ";"
                  Blankspace@69..70 " "
                  BraceRight@70..71 "}"
                Blankspace@71..72 " "
                BraceRight@72..73 "}"
              Blankspace@73..74 " "
              AttributeList@74..79
                ElseAttribute@74..79
                  AttributeOperator@74..75 "@"
                  Else@75..79 "else"
              Blankspace@79..80 " "
              GlobalCompoundDeclaration@80..103
                BraceLeft@80..81 "{"
                Blankspace@81..82 " "
                ConstantDeclaration@82..101
                  Const@82..87 "const"
                  Blankspace@87..88 " "
                  Name@88..91
                    Identifier@88..91 "baz"
                  Colon@91..92 ":"
                  Blankspace@92..93 " "
                  TypeSpecifier@93..96
                    Path@93..96
                      Identifier@93..96 "u32"
                  Blankspace@96..97 " "
                  Equal@97..98 "="
                  Blankspace@98..99 " "
                  Literal@99..100
                    IntLiteral@99..100 "0"
                  Semicolon@100..101 ";"
                Blankspace@101..102 " "
                BraceRight@102..103 "}""#]],
    );
}

#[test]
fn module_compound_shadow() {
    check(
        "{ const foo: u32 = 0; } const foo: u32 = 1;",
        expect![[r#"
            SourceFile@0..43
              GlobalCompoundDeclaration@0..23
                BraceLeft@0..1 "{"
                Blankspace@1..2 " "
                ConstantDeclaration@2..21
                  Const@2..7 "const"
                  Blankspace@7..8 " "
                  Name@8..11
                    Identifier@8..11 "foo"
                  Colon@11..12 ":"
                  Blankspace@12..13 " "
                  TypeSpecifier@13..16
                    Path@13..16
                      Identifier@13..16 "u32"
                  Blankspace@16..17 " "
                  Equal@17..18 "="
                  Blankspace@18..19 " "
                  Literal@19..20
                    IntLiteral@19..20 "0"
                  Semicolon@20..21 ";"
                Blankspace@21..22 " "
                BraceRight@22..23 "}"
              Blankspace@23..24 " "
              ConstantDeclaration@24..43
                Const@24..29 "const"
                Blankspace@29..30 " "
                Name@30..33
                  Identifier@30..33 "foo"
                Colon@33..34 ":"
                Blankspace@34..35 " "
                TypeSpecifier@35..38
                  Path@35..38
                    Identifier@35..38 "u32"
                Blankspace@38..39 " "
                Equal@39..40 "="
                Blankspace@40..41 " "
                Literal@41..42
                  IntLiteral@41..42 "1"
                Semicolon@42..43 ";""#]],
    );
}

#[test]
fn function_compound_if_true() {
    check(
        "fn f() { @if(true) { const_assert true; const x: u32 = 0; } }",
        expect![[r#"
            SourceFile@0..61
              FunctionDeclaration@0..61
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                FunctionParameters@4..6
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                Blankspace@6..7 " "
                CompoundStatement@7..61
                  BraceLeft@7..8 "{"
                  Blankspace@8..9 " "
                  AttributeList@9..18
                    IfAttribute@9..18
                      AttributeOperator@9..10 "@"
                      If@10..12 "if"
                      ParenthesisLeft@12..13 "("
                      Literal@13..17
                        True@13..17 "true"
                      ParenthesisRight@17..18 ")"
                  Blankspace@18..19 " "
                  CompoundStatement@19..59
                    BraceLeft@19..20 "{"
                    Blankspace@20..21 " "
                    AssertStatement@21..39
                      ConstantAssert@21..33 "const_assert"
                      Blankspace@33..34 " "
                      Literal@34..38
                        True@34..38 "true"
                      Semicolon@38..39 ";"
                    Blankspace@39..40 " "
                    ConstantDeclaration@40..57
                      Const@40..45 "const"
                      Blankspace@45..46 " "
                      Name@46..47
                        Identifier@46..47 "x"
                      Colon@47..48 ":"
                      Blankspace@48..49 " "
                      TypeSpecifier@49..52
                        Path@49..52
                          Identifier@49..52 "u32"
                      Blankspace@52..53 " "
                      Equal@53..54 "="
                      Blankspace@54..55 " "
                      Literal@55..56
                        IntLiteral@55..56 "0"
                      Semicolon@56..57 ";"
                    Blankspace@57..58 " "
                    BraceRight@58..59 "}"
                  Blankspace@59..60 " "
                  BraceRight@60..61 "}""#]],
    );
}

#[test]
fn function_compound_if_false() {
    check(
        "fn f() { @if(false) { const_assert true; const x: u32 = 0; } }",
        expect![[r#"
            SourceFile@0..62
              FunctionDeclaration@0..62
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                FunctionParameters@4..6
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                Blankspace@6..7 " "
                CompoundStatement@7..62
                  BraceLeft@7..8 "{"
                  Blankspace@8..9 " "
                  AttributeList@9..19
                    IfAttribute@9..19
                      AttributeOperator@9..10 "@"
                      If@10..12 "if"
                      ParenthesisLeft@12..13 "("
                      Literal@13..18
                        False@13..18 "false"
                      ParenthesisRight@18..19 ")"
                  Blankspace@19..20 " "
                  CompoundStatement@20..60
                    BraceLeft@20..21 "{"
                    Blankspace@21..22 " "
                    AssertStatement@22..40
                      ConstantAssert@22..34 "const_assert"
                      Blankspace@34..35 " "
                      Literal@35..39
                        True@35..39 "true"
                      Semicolon@39..40 ";"
                    Blankspace@40..41 " "
                    ConstantDeclaration@41..58
                      Const@41..46 "const"
                      Blankspace@46..47 " "
                      Name@47..48
                        Identifier@47..48 "x"
                      Colon@48..49 ":"
                      Blankspace@49..50 " "
                      TypeSpecifier@50..53
                        Path@50..53
                          Identifier@50..53 "u32"
                      Blankspace@53..54 " "
                      Equal@54..55 "="
                      Blankspace@55..56 " "
                      Literal@56..57
                        IntLiteral@56..57 "0"
                      Semicolon@57..58 ";"
                    Blankspace@58..59 " "
                    BraceRight@59..60 "}"
                  Blankspace@60..61 " "
                  BraceRight@61..62 "}""#]],
    );
}

#[test]
fn function_compound_if_false_compound_elif_true() {
    check(
        "fn f() { @if(false) { const foo: u32 = 0; } @elif(true) { const bar: u32 = 0; } }",
        expect![[r#"
            SourceFile@0..81
              FunctionDeclaration@0..81
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                FunctionParameters@4..6
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                Blankspace@6..7 " "
                CompoundStatement@7..81
                  BraceLeft@7..8 "{"
                  Blankspace@8..9 " "
                  AttributeList@9..19
                    IfAttribute@9..19
                      AttributeOperator@9..10 "@"
                      If@10..12 "if"
                      ParenthesisLeft@12..13 "("
                      Literal@13..18
                        False@13..18 "false"
                      ParenthesisRight@18..19 ")"
                  Blankspace@19..20 " "
                  CompoundStatement@20..43
                    BraceLeft@20..21 "{"
                    Blankspace@21..22 " "
                    ConstantDeclaration@22..41
                      Const@22..27 "const"
                      Blankspace@27..28 " "
                      Name@28..31
                        Identifier@28..31 "foo"
                      Colon@31..32 ":"
                      Blankspace@32..33 " "
                      TypeSpecifier@33..36
                        Path@33..36
                          Identifier@33..36 "u32"
                      Blankspace@36..37 " "
                      Equal@37..38 "="
                      Blankspace@38..39 " "
                      Literal@39..40
                        IntLiteral@39..40 "0"
                      Semicolon@40..41 ";"
                    Blankspace@41..42 " "
                    BraceRight@42..43 "}"
                  Blankspace@43..44 " "
                  AttributeList@44..55
                    OtherAttribute@44..55
                      AttributeOperator@44..45 "@"
                      Identifier@45..49 "elif"
                      Arguments@49..55
                        ParenthesisLeft@49..50 "("
                        Literal@50..54
                          True@50..54 "true"
                        ParenthesisRight@54..55 ")"
                  Blankspace@55..56 " "
                  CompoundStatement@56..79
                    BraceLeft@56..57 "{"
                    Blankspace@57..58 " "
                    ConstantDeclaration@58..77
                      Const@58..63 "const"
                      Blankspace@63..64 " "
                      Name@64..67
                        Identifier@64..67 "bar"
                      Colon@67..68 ":"
                      Blankspace@68..69 " "
                      TypeSpecifier@69..72
                        Path@69..72
                          Identifier@69..72 "u32"
                      Blankspace@72..73 " "
                      Equal@73..74 "="
                      Blankspace@74..75 " "
                      Literal@75..76
                        IntLiteral@75..76 "0"
                      Semicolon@76..77 ";"
                    Blankspace@77..78 " "
                    BraceRight@78..79 "}"
                  Blankspace@79..80 " "
                  BraceRight@80..81 "}""#]],
    );
}

#[test]
fn function_compound_if_true_compound_elif_true() {
    check(
        "fn f() { @if(true) { const foo: u32 = 0; } @elif(true) { const bar: u32 = 0; } }",
        expect![[r#"
            SourceFile@0..80
              FunctionDeclaration@0..80
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                FunctionParameters@4..6
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                Blankspace@6..7 " "
                CompoundStatement@7..80
                  BraceLeft@7..8 "{"
                  Blankspace@8..9 " "
                  AttributeList@9..18
                    IfAttribute@9..18
                      AttributeOperator@9..10 "@"
                      If@10..12 "if"
                      ParenthesisLeft@12..13 "("
                      Literal@13..17
                        True@13..17 "true"
                      ParenthesisRight@17..18 ")"
                  Blankspace@18..19 " "
                  CompoundStatement@19..42
                    BraceLeft@19..20 "{"
                    Blankspace@20..21 " "
                    ConstantDeclaration@21..40
                      Const@21..26 "const"
                      Blankspace@26..27 " "
                      Name@27..30
                        Identifier@27..30 "foo"
                      Colon@30..31 ":"
                      Blankspace@31..32 " "
                      TypeSpecifier@32..35
                        Path@32..35
                          Identifier@32..35 "u32"
                      Blankspace@35..36 " "
                      Equal@36..37 "="
                      Blankspace@37..38 " "
                      Literal@38..39
                        IntLiteral@38..39 "0"
                      Semicolon@39..40 ";"
                    Blankspace@40..41 " "
                    BraceRight@41..42 "}"
                  Blankspace@42..43 " "
                  AttributeList@43..54
                    OtherAttribute@43..54
                      AttributeOperator@43..44 "@"
                      Identifier@44..48 "elif"
                      Arguments@48..54
                        ParenthesisLeft@48..49 "("
                        Literal@49..53
                          True@49..53 "true"
                        ParenthesisRight@53..54 ")"
                  Blankspace@54..55 " "
                  CompoundStatement@55..78
                    BraceLeft@55..56 "{"
                    Blankspace@56..57 " "
                    ConstantDeclaration@57..76
                      Const@57..62 "const"
                      Blankspace@62..63 " "
                      Name@63..66
                        Identifier@63..66 "bar"
                      Colon@66..67 ":"
                      Blankspace@67..68 " "
                      TypeSpecifier@68..71
                        Path@68..71
                          Identifier@68..71 "u32"
                      Blankspace@71..72 " "
                      Equal@72..73 "="
                      Blankspace@73..74 " "
                      Literal@74..75
                        IntLiteral@74..75 "0"
                      Semicolon@75..76 ";"
                    Blankspace@76..77 " "
                    BraceRight@77..78 "}"
                  Blankspace@78..79 " "
                  BraceRight@79..80 "}""#]],
    );
}

#[test]
fn function_if_false_compound_elif_true() {
    check(
        "fn f() { @if(false) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; } }",
        expect![[r#"
            SourceFile@0..77
              FunctionDeclaration@0..77
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                FunctionParameters@4..6
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                Blankspace@6..7 " "
                CompoundStatement@7..77
                  BraceLeft@7..8 "{"
                  Blankspace@8..9 " "
                  AttributeList@9..19
                    IfAttribute@9..19
                      AttributeOperator@9..10 "@"
                      If@10..12 "if"
                      ParenthesisLeft@12..13 "("
                      Literal@13..18
                        False@13..18 "false"
                      ParenthesisRight@18..19 ")"
                  Blankspace@19..20 " "
                  ConstantDeclaration@20..39
                    Const@20..25 "const"
                    Blankspace@25..26 " "
                    Name@26..29
                      Identifier@26..29 "foo"
                    Colon@29..30 ":"
                    Blankspace@30..31 " "
                    TypeSpecifier@31..34
                      Path@31..34
                        Identifier@31..34 "u32"
                    Blankspace@34..35 " "
                    Equal@35..36 "="
                    Blankspace@36..37 " "
                    Literal@37..38
                      IntLiteral@37..38 "0"
                    Semicolon@38..39 ";"
                  Blankspace@39..40 " "
                  AttributeList@40..51
                    OtherAttribute@40..51
                      AttributeOperator@40..41 "@"
                      Identifier@41..45 "elif"
                      Arguments@45..51
                        ParenthesisLeft@45..46 "("
                        Literal@46..50
                          True@46..50 "true"
                        ParenthesisRight@50..51 ")"
                  Blankspace@51..52 " "
                  CompoundStatement@52..75
                    BraceLeft@52..53 "{"
                    Blankspace@53..54 " "
                    ConstantDeclaration@54..73
                      Const@54..59 "const"
                      Blankspace@59..60 " "
                      Name@60..63
                        Identifier@60..63 "bar"
                      Colon@63..64 ":"
                      Blankspace@64..65 " "
                      TypeSpecifier@65..68
                        Path@65..68
                          Identifier@65..68 "u32"
                      Blankspace@68..69 " "
                      Equal@69..70 "="
                      Blankspace@70..71 " "
                      Literal@71..72
                        IntLiteral@71..72 "0"
                      Semicolon@72..73 ";"
                    Blankspace@73..74 " "
                    BraceRight@74..75 "}"
                  Blankspace@75..76 " "
                  BraceRight@76..77 "}""#]],
    );
}

#[test]
fn function_if_true_compound_elif_true() {
    check(
        "fn f() { @if(true) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; } }",
        expect![[r#"
            SourceFile@0..76
              FunctionDeclaration@0..76
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                FunctionParameters@4..6
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                Blankspace@6..7 " "
                CompoundStatement@7..76
                  BraceLeft@7..8 "{"
                  Blankspace@8..9 " "
                  AttributeList@9..18
                    IfAttribute@9..18
                      AttributeOperator@9..10 "@"
                      If@10..12 "if"
                      ParenthesisLeft@12..13 "("
                      Literal@13..17
                        True@13..17 "true"
                      ParenthesisRight@17..18 ")"
                  Blankspace@18..19 " "
                  ConstantDeclaration@19..38
                    Const@19..24 "const"
                    Blankspace@24..25 " "
                    Name@25..28
                      Identifier@25..28 "foo"
                    Colon@28..29 ":"
                    Blankspace@29..30 " "
                    TypeSpecifier@30..33
                      Path@30..33
                        Identifier@30..33 "u32"
                    Blankspace@33..34 " "
                    Equal@34..35 "="
                    Blankspace@35..36 " "
                    Literal@36..37
                      IntLiteral@36..37 "0"
                    Semicolon@37..38 ";"
                  Blankspace@38..39 " "
                  AttributeList@39..50
                    OtherAttribute@39..50
                      AttributeOperator@39..40 "@"
                      Identifier@40..44 "elif"
                      Arguments@44..50
                        ParenthesisLeft@44..45 "("
                        Literal@45..49
                          True@45..49 "true"
                        ParenthesisRight@49..50 ")"
                  Blankspace@50..51 " "
                  CompoundStatement@51..74
                    BraceLeft@51..52 "{"
                    Blankspace@52..53 " "
                    ConstantDeclaration@53..72
                      Const@53..58 "const"
                      Blankspace@58..59 " "
                      Name@59..62
                        Identifier@59..62 "bar"
                      Colon@62..63 ":"
                      Blankspace@63..64 " "
                      TypeSpecifier@64..67
                        Path@64..67
                          Identifier@64..67 "u32"
                      Blankspace@67..68 " "
                      Equal@68..69 "="
                      Blankspace@69..70 " "
                      Literal@70..71
                        IntLiteral@70..71 "0"
                      Semicolon@71..72 ";"
                    Blankspace@72..73 " "
                    BraceRight@73..74 "}"
                  Blankspace@74..75 " "
                  BraceRight@75..76 "}""#]],
    );
}

#[test]
fn function_compound_else_hit() {
    check(
        "fn f() { @if(false) { const foo: u32 = 0; } @elif(false) { const bar: u32 = 0; } @else { const baz: u32 = 0; } }",
        expect![[r#"
            SourceFile@0..112
              FunctionDeclaration@0..112
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                FunctionParameters@4..6
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                Blankspace@6..7 " "
                CompoundStatement@7..112
                  BraceLeft@7..8 "{"
                  Blankspace@8..9 " "
                  AttributeList@9..19
                    IfAttribute@9..19
                      AttributeOperator@9..10 "@"
                      If@10..12 "if"
                      ParenthesisLeft@12..13 "("
                      Literal@13..18
                        False@13..18 "false"
                      ParenthesisRight@18..19 ")"
                  Blankspace@19..20 " "
                  CompoundStatement@20..43
                    BraceLeft@20..21 "{"
                    Blankspace@21..22 " "
                    ConstantDeclaration@22..41
                      Const@22..27 "const"
                      Blankspace@27..28 " "
                      Name@28..31
                        Identifier@28..31 "foo"
                      Colon@31..32 ":"
                      Blankspace@32..33 " "
                      TypeSpecifier@33..36
                        Path@33..36
                          Identifier@33..36 "u32"
                      Blankspace@36..37 " "
                      Equal@37..38 "="
                      Blankspace@38..39 " "
                      Literal@39..40
                        IntLiteral@39..40 "0"
                      Semicolon@40..41 ";"
                    Blankspace@41..42 " "
                    BraceRight@42..43 "}"
                  Blankspace@43..44 " "
                  AttributeList@44..56
                    OtherAttribute@44..56
                      AttributeOperator@44..45 "@"
                      Identifier@45..49 "elif"
                      Arguments@49..56
                        ParenthesisLeft@49..50 "("
                        Literal@50..55
                          False@50..55 "false"
                        ParenthesisRight@55..56 ")"
                  Blankspace@56..57 " "
                  CompoundStatement@57..80
                    BraceLeft@57..58 "{"
                    Blankspace@58..59 " "
                    ConstantDeclaration@59..78
                      Const@59..64 "const"
                      Blankspace@64..65 " "
                      Name@65..68
                        Identifier@65..68 "bar"
                      Colon@68..69 ":"
                      Blankspace@69..70 " "
                      TypeSpecifier@70..73
                        Path@70..73
                          Identifier@70..73 "u32"
                      Blankspace@73..74 " "
                      Equal@74..75 "="
                      Blankspace@75..76 " "
                      Literal@76..77
                        IntLiteral@76..77 "0"
                      Semicolon@77..78 ";"
                    Blankspace@78..79 " "
                    BraceRight@79..80 "}"
                  Blankspace@80..81 " "
                  AttributeList@81..86
                    ElseAttribute@81..86
                      AttributeOperator@81..82 "@"
                      Else@82..86 "else"
                  Blankspace@86..87 " "
                  CompoundStatement@87..110
                    BraceLeft@87..88 "{"
                    Blankspace@88..89 " "
                    ConstantDeclaration@89..108
                      Const@89..94 "const"
                      Blankspace@94..95 " "
                      Name@95..98
                        Identifier@95..98 "baz"
                      Colon@98..99 ":"
                      Blankspace@99..100 " "
                      TypeSpecifier@100..103
                        Path@100..103
                          Identifier@100..103 "u32"
                      Blankspace@103..104 " "
                      Equal@104..105 "="
                      Blankspace@105..106 " "
                      Literal@106..107
                        IntLiteral@106..107 "0"
                      Semicolon@107..108 ";"
                    Blankspace@108..109 " "
                    BraceRight@109..110 "}"
                  Blankspace@110..111 " "
                  BraceRight@111..112 "}""#]],
    );
}

#[test]
fn function_compound_else_skipped() {
    check(
        "fn f() { @if(false) { const foo: u32 = 0; } @elif(true) { const bar: u32 = 0; } @else { const baz: u32 = 0; } }",
        expect![[r#"
            SourceFile@0..111
              FunctionDeclaration@0..111
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                FunctionParameters@4..6
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                Blankspace@6..7 " "
                CompoundStatement@7..111
                  BraceLeft@7..8 "{"
                  Blankspace@8..9 " "
                  AttributeList@9..19
                    IfAttribute@9..19
                      AttributeOperator@9..10 "@"
                      If@10..12 "if"
                      ParenthesisLeft@12..13 "("
                      Literal@13..18
                        False@13..18 "false"
                      ParenthesisRight@18..19 ")"
                  Blankspace@19..20 " "
                  CompoundStatement@20..43
                    BraceLeft@20..21 "{"
                    Blankspace@21..22 " "
                    ConstantDeclaration@22..41
                      Const@22..27 "const"
                      Blankspace@27..28 " "
                      Name@28..31
                        Identifier@28..31 "foo"
                      Colon@31..32 ":"
                      Blankspace@32..33 " "
                      TypeSpecifier@33..36
                        Path@33..36
                          Identifier@33..36 "u32"
                      Blankspace@36..37 " "
                      Equal@37..38 "="
                      Blankspace@38..39 " "
                      Literal@39..40
                        IntLiteral@39..40 "0"
                      Semicolon@40..41 ";"
                    Blankspace@41..42 " "
                    BraceRight@42..43 "}"
                  Blankspace@43..44 " "
                  AttributeList@44..55
                    OtherAttribute@44..55
                      AttributeOperator@44..45 "@"
                      Identifier@45..49 "elif"
                      Arguments@49..55
                        ParenthesisLeft@49..50 "("
                        Literal@50..54
                          True@50..54 "true"
                        ParenthesisRight@54..55 ")"
                  Blankspace@55..56 " "
                  CompoundStatement@56..79
                    BraceLeft@56..57 "{"
                    Blankspace@57..58 " "
                    ConstantDeclaration@58..77
                      Const@58..63 "const"
                      Blankspace@63..64 " "
                      Name@64..67
                        Identifier@64..67 "bar"
                      Colon@67..68 ":"
                      Blankspace@68..69 " "
                      TypeSpecifier@69..72
                        Path@69..72
                          Identifier@69..72 "u32"
                      Blankspace@72..73 " "
                      Equal@73..74 "="
                      Blankspace@74..75 " "
                      Literal@75..76
                        IntLiteral@75..76 "0"
                      Semicolon@76..77 ";"
                    Blankspace@77..78 " "
                    BraceRight@78..79 "}"
                  Blankspace@79..80 " "
                  AttributeList@80..85
                    ElseAttribute@80..85
                      AttributeOperator@80..81 "@"
                      Else@81..85 "else"
                  Blankspace@85..86 " "
                  CompoundStatement@86..109
                    BraceLeft@86..87 "{"
                    Blankspace@87..88 " "
                    ConstantDeclaration@88..107
                      Const@88..93 "const"
                      Blankspace@93..94 " "
                      Name@94..97
                        Identifier@94..97 "baz"
                      Colon@97..98 ":"
                      Blankspace@98..99 " "
                      TypeSpecifier@99..102
                        Path@99..102
                          Identifier@99..102 "u32"
                      Blankspace@102..103 " "
                      Equal@103..104 "="
                      Blankspace@104..105 " "
                      Literal@105..106
                        IntLiteral@105..106 "0"
                      Semicolon@106..107 ";"
                    Blankspace@107..108 " "
                    BraceRight@108..109 "}"
                  Blankspace@109..110 " "
                  BraceRight@110..111 "}""#]],
    );
}

#[test]
fn function_compound_nested_if() {
    check(
        "fn f() { @if(true) { const foo: u32 = 0; @if(true) { const bar: u32 = 0; } } }",
        expect![[r#"
            SourceFile@0..78
              FunctionDeclaration@0..78
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                FunctionParameters@4..6
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                Blankspace@6..7 " "
                CompoundStatement@7..78
                  BraceLeft@7..8 "{"
                  Blankspace@8..9 " "
                  AttributeList@9..18
                    IfAttribute@9..18
                      AttributeOperator@9..10 "@"
                      If@10..12 "if"
                      ParenthesisLeft@12..13 "("
                      Literal@13..17
                        True@13..17 "true"
                      ParenthesisRight@17..18 ")"
                  Blankspace@18..19 " "
                  CompoundStatement@19..76
                    BraceLeft@19..20 "{"
                    Blankspace@20..21 " "
                    ConstantDeclaration@21..40
                      Const@21..26 "const"
                      Blankspace@26..27 " "
                      Name@27..30
                        Identifier@27..30 "foo"
                      Colon@30..31 ":"
                      Blankspace@31..32 " "
                      TypeSpecifier@32..35
                        Path@32..35
                          Identifier@32..35 "u32"
                      Blankspace@35..36 " "
                      Equal@36..37 "="
                      Blankspace@37..38 " "
                      Literal@38..39
                        IntLiteral@38..39 "0"
                      Semicolon@39..40 ";"
                    Blankspace@40..41 " "
                    AttributeList@41..50
                      IfAttribute@41..50
                        AttributeOperator@41..42 "@"
                        If@42..44 "if"
                        ParenthesisLeft@44..45 "("
                        Literal@45..49
                          True@45..49 "true"
                        ParenthesisRight@49..50 ")"
                    Blankspace@50..51 " "
                    CompoundStatement@51..74
                      BraceLeft@51..52 "{"
                      Blankspace@52..53 " "
                      ConstantDeclaration@53..72
                        Const@53..58 "const"
                        Blankspace@58..59 " "
                        Name@59..62
                          Identifier@59..62 "bar"
                        Colon@62..63 ":"
                        Blankspace@63..64 " "
                        TypeSpecifier@64..67
                          Path@64..67
                            Identifier@64..67 "u32"
                        Blankspace@67..68 " "
                        Equal@68..69 "="
                        Blankspace@69..70 " "
                        Literal@70..71
                          IntLiteral@70..71 "0"
                        Semicolon@71..72 ";"
                      Blankspace@72..73 " "
                      BraceRight@73..74 "}"
                    Blankspace@74..75 " "
                    BraceRight@75..76 "}"
                  Blankspace@76..77 " "
                  BraceRight@77..78 "}""#]],
    );
}

#[test]
fn function_compound_nested_elif_hit() {
    check(
        "fn f() { @if(true) { @if(false) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; } } @else { const baz: u32 = 0; } }",
        expect![[r#"
            SourceFile@0..121
              FunctionDeclaration@0..121
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                FunctionParameters@4..6
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                Blankspace@6..7 " "
                CompoundStatement@7..121
                  BraceLeft@7..8 "{"
                  Blankspace@8..9 " "
                  AttributeList@9..18
                    IfAttribute@9..18
                      AttributeOperator@9..10 "@"
                      If@10..12 "if"
                      ParenthesisLeft@12..13 "("
                      Literal@13..17
                        True@13..17 "true"
                      ParenthesisRight@17..18 ")"
                  Blankspace@18..19 " "
                  CompoundStatement@19..89
                    BraceLeft@19..20 "{"
                    Blankspace@20..21 " "
                    AttributeList@21..31
                      IfAttribute@21..31
                        AttributeOperator@21..22 "@"
                        If@22..24 "if"
                        ParenthesisLeft@24..25 "("
                        Literal@25..30
                          False@25..30 "false"
                        ParenthesisRight@30..31 ")"
                    Blankspace@31..32 " "
                    ConstantDeclaration@32..51
                      Const@32..37 "const"
                      Blankspace@37..38 " "
                      Name@38..41
                        Identifier@38..41 "foo"
                      Colon@41..42 ":"
                      Blankspace@42..43 " "
                      TypeSpecifier@43..46
                        Path@43..46
                          Identifier@43..46 "u32"
                      Blankspace@46..47 " "
                      Equal@47..48 "="
                      Blankspace@48..49 " "
                      Literal@49..50
                        IntLiteral@49..50 "0"
                      Semicolon@50..51 ";"
                    Blankspace@51..52 " "
                    AttributeList@52..63
                      OtherAttribute@52..63
                        AttributeOperator@52..53 "@"
                        Identifier@53..57 "elif"
                        Arguments@57..63
                          ParenthesisLeft@57..58 "("
                          Literal@58..62
                            True@58..62 "true"
                          ParenthesisRight@62..63 ")"
                    Blankspace@63..64 " "
                    CompoundStatement@64..87
                      BraceLeft@64..65 "{"
                      Blankspace@65..66 " "
                      ConstantDeclaration@66..85
                        Const@66..71 "const"
                        Blankspace@71..72 " "
                        Name@72..75
                          Identifier@72..75 "bar"
                        Colon@75..76 ":"
                        Blankspace@76..77 " "
                        TypeSpecifier@77..80
                          Path@77..80
                            Identifier@77..80 "u32"
                        Blankspace@80..81 " "
                        Equal@81..82 "="
                        Blankspace@82..83 " "
                        Literal@83..84
                          IntLiteral@83..84 "0"
                        Semicolon@84..85 ";"
                      Blankspace@85..86 " "
                      BraceRight@86..87 "}"
                    Blankspace@87..88 " "
                    BraceRight@88..89 "}"
                  Blankspace@89..90 " "
                  AttributeList@90..95
                    ElseAttribute@90..95
                      AttributeOperator@90..91 "@"
                      Else@91..95 "else"
                  Blankspace@95..96 " "
                  CompoundStatement@96..119
                    BraceLeft@96..97 "{"
                    Blankspace@97..98 " "
                    ConstantDeclaration@98..117
                      Const@98..103 "const"
                      Blankspace@103..104 " "
                      Name@104..107
                        Identifier@104..107 "baz"
                      Colon@107..108 ":"
                      Blankspace@108..109 " "
                      TypeSpecifier@109..112
                        Path@109..112
                          Identifier@109..112 "u32"
                      Blankspace@112..113 " "
                      Equal@113..114 "="
                      Blankspace@114..115 " "
                      Literal@115..116
                        IntLiteral@115..116 "0"
                      Semicolon@116..117 ";"
                    Blankspace@117..118 " "
                    BraceRight@118..119 "}"
                  Blankspace@119..120 " "
                  BraceRight@120..121 "}""#]],
    );
}

#[test]
fn function_compound_nested_elif_skipped() {
    check(
        "fn f() { @if(true) { @if(true) const foo: u32 = 0; @elif(true) { const bar: u32 = 0; } } @else { const baz: u32 = 0; } }",
        expect![[r#"
            SourceFile@0..120
              FunctionDeclaration@0..120
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                FunctionParameters@4..6
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                Blankspace@6..7 " "
                CompoundStatement@7..120
                  BraceLeft@7..8 "{"
                  Blankspace@8..9 " "
                  AttributeList@9..18
                    IfAttribute@9..18
                      AttributeOperator@9..10 "@"
                      If@10..12 "if"
                      ParenthesisLeft@12..13 "("
                      Literal@13..17
                        True@13..17 "true"
                      ParenthesisRight@17..18 ")"
                  Blankspace@18..19 " "
                  CompoundStatement@19..88
                    BraceLeft@19..20 "{"
                    Blankspace@20..21 " "
                    AttributeList@21..30
                      IfAttribute@21..30
                        AttributeOperator@21..22 "@"
                        If@22..24 "if"
                        ParenthesisLeft@24..25 "("
                        Literal@25..29
                          True@25..29 "true"
                        ParenthesisRight@29..30 ")"
                    Blankspace@30..31 " "
                    ConstantDeclaration@31..50
                      Const@31..36 "const"
                      Blankspace@36..37 " "
                      Name@37..40
                        Identifier@37..40 "foo"
                      Colon@40..41 ":"
                      Blankspace@41..42 " "
                      TypeSpecifier@42..45
                        Path@42..45
                          Identifier@42..45 "u32"
                      Blankspace@45..46 " "
                      Equal@46..47 "="
                      Blankspace@47..48 " "
                      Literal@48..49
                        IntLiteral@48..49 "0"
                      Semicolon@49..50 ";"
                    Blankspace@50..51 " "
                    AttributeList@51..62
                      OtherAttribute@51..62
                        AttributeOperator@51..52 "@"
                        Identifier@52..56 "elif"
                        Arguments@56..62
                          ParenthesisLeft@56..57 "("
                          Literal@57..61
                            True@57..61 "true"
                          ParenthesisRight@61..62 ")"
                    Blankspace@62..63 " "
                    CompoundStatement@63..86
                      BraceLeft@63..64 "{"
                      Blankspace@64..65 " "
                      ConstantDeclaration@65..84
                        Const@65..70 "const"
                        Blankspace@70..71 " "
                        Name@71..74
                          Identifier@71..74 "bar"
                        Colon@74..75 ":"
                        Blankspace@75..76 " "
                        TypeSpecifier@76..79
                          Path@76..79
                            Identifier@76..79 "u32"
                        Blankspace@79..80 " "
                        Equal@80..81 "="
                        Blankspace@81..82 " "
                        Literal@82..83
                          IntLiteral@82..83 "0"
                        Semicolon@83..84 ";"
                      Blankspace@84..85 " "
                      BraceRight@85..86 "}"
                    Blankspace@86..87 " "
                    BraceRight@87..88 "}"
                  Blankspace@88..89 " "
                  AttributeList@89..94
                    ElseAttribute@89..94
                      AttributeOperator@89..90 "@"
                      Else@90..94 "else"
                  Blankspace@94..95 " "
                  CompoundStatement@95..118
                    BraceLeft@95..96 "{"
                    Blankspace@96..97 " "
                    ConstantDeclaration@97..116
                      Const@97..102 "const"
                      Blankspace@102..103 " "
                      Name@103..106
                        Identifier@103..106 "baz"
                      Colon@106..107 ":"
                      Blankspace@107..108 " "
                      TypeSpecifier@108..111
                        Path@108..111
                          Identifier@108..111 "u32"
                      Blankspace@111..112 " "
                      Equal@112..113 "="
                      Blankspace@113..114 " "
                      Literal@114..115
                        IntLiteral@114..115 "0"
                      Semicolon@115..116 ";"
                    Blankspace@116..117 " "
                    BraceRight@117..118 "}"
                  Blankspace@118..119 " "
                  BraceRight@119..120 "}""#]],
    );
}

#[test]
fn function_compound_nested_else_hit() {
    check(
        "fn f() { @if(true) { @if(false) const foo: u32 = 0; @else { const bar: u32 = 0; } } @else { const baz: u32 = 0; } }",
        expect![[r#"
            SourceFile@0..115
              FunctionDeclaration@0..115
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                FunctionParameters@4..6
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                Blankspace@6..7 " "
                CompoundStatement@7..115
                  BraceLeft@7..8 "{"
                  Blankspace@8..9 " "
                  AttributeList@9..18
                    IfAttribute@9..18
                      AttributeOperator@9..10 "@"
                      If@10..12 "if"
                      ParenthesisLeft@12..13 "("
                      Literal@13..17
                        True@13..17 "true"
                      ParenthesisRight@17..18 ")"
                  Blankspace@18..19 " "
                  CompoundStatement@19..83
                    BraceLeft@19..20 "{"
                    Blankspace@20..21 " "
                    AttributeList@21..31
                      IfAttribute@21..31
                        AttributeOperator@21..22 "@"
                        If@22..24 "if"
                        ParenthesisLeft@24..25 "("
                        Literal@25..30
                          False@25..30 "false"
                        ParenthesisRight@30..31 ")"
                    Blankspace@31..32 " "
                    ConstantDeclaration@32..51
                      Const@32..37 "const"
                      Blankspace@37..38 " "
                      Name@38..41
                        Identifier@38..41 "foo"
                      Colon@41..42 ":"
                      Blankspace@42..43 " "
                      TypeSpecifier@43..46
                        Path@43..46
                          Identifier@43..46 "u32"
                      Blankspace@46..47 " "
                      Equal@47..48 "="
                      Blankspace@48..49 " "
                      Literal@49..50
                        IntLiteral@49..50 "0"
                      Semicolon@50..51 ";"
                    Blankspace@51..52 " "
                    AttributeList@52..57
                      ElseAttribute@52..57
                        AttributeOperator@52..53 "@"
                        Else@53..57 "else"
                    Blankspace@57..58 " "
                    CompoundStatement@58..81
                      BraceLeft@58..59 "{"
                      Blankspace@59..60 " "
                      ConstantDeclaration@60..79
                        Const@60..65 "const"
                        Blankspace@65..66 " "
                        Name@66..69
                          Identifier@66..69 "bar"
                        Colon@69..70 ":"
                        Blankspace@70..71 " "
                        TypeSpecifier@71..74
                          Path@71..74
                            Identifier@71..74 "u32"
                        Blankspace@74..75 " "
                        Equal@75..76 "="
                        Blankspace@76..77 " "
                        Literal@77..78
                          IntLiteral@77..78 "0"
                        Semicolon@78..79 ";"
                      Blankspace@79..80 " "
                      BraceRight@80..81 "}"
                    Blankspace@81..82 " "
                    BraceRight@82..83 "}"
                  Blankspace@83..84 " "
                  AttributeList@84..89
                    ElseAttribute@84..89
                      AttributeOperator@84..85 "@"
                      Else@85..89 "else"
                  Blankspace@89..90 " "
                  CompoundStatement@90..113
                    BraceLeft@90..91 "{"
                    Blankspace@91..92 " "
                    ConstantDeclaration@92..111
                      Const@92..97 "const"
                      Blankspace@97..98 " "
                      Name@98..101
                        Identifier@98..101 "baz"
                      Colon@101..102 ":"
                      Blankspace@102..103 " "
                      TypeSpecifier@103..106
                        Path@103..106
                          Identifier@103..106 "u32"
                      Blankspace@106..107 " "
                      Equal@107..108 "="
                      Blankspace@108..109 " "
                      Literal@109..110
                        IntLiteral@109..110 "0"
                      Semicolon@110..111 ";"
                    Blankspace@111..112 " "
                    BraceRight@112..113 "}"
                  Blankspace@113..114 " "
                  BraceRight@114..115 "}""#]],
    );
}

#[test]
fn function_compound_nested_else_skipped() {
    check(
        "fn f() { @if(true) { @if(true) const foo: u32 = 0; @else { const bar: u32 = 0; } } @else { const baz: u32 = 0; } }",
        expect![[r#"
            SourceFile@0..114
              FunctionDeclaration@0..114
                Fn@0..2 "fn"
                Blankspace@2..3 " "
                Name@3..4
                  Identifier@3..4 "f"
                FunctionParameters@4..6
                  ParenthesisLeft@4..5 "("
                  ParenthesisRight@5..6 ")"
                Blankspace@6..7 " "
                CompoundStatement@7..114
                  BraceLeft@7..8 "{"
                  Blankspace@8..9 " "
                  AttributeList@9..18
                    IfAttribute@9..18
                      AttributeOperator@9..10 "@"
                      If@10..12 "if"
                      ParenthesisLeft@12..13 "("
                      Literal@13..17
                        True@13..17 "true"
                      ParenthesisRight@17..18 ")"
                  Blankspace@18..19 " "
                  CompoundStatement@19..82
                    BraceLeft@19..20 "{"
                    Blankspace@20..21 " "
                    AttributeList@21..30
                      IfAttribute@21..30
                        AttributeOperator@21..22 "@"
                        If@22..24 "if"
                        ParenthesisLeft@24..25 "("
                        Literal@25..29
                          True@25..29 "true"
                        ParenthesisRight@29..30 ")"
                    Blankspace@30..31 " "
                    ConstantDeclaration@31..50
                      Const@31..36 "const"
                      Blankspace@36..37 " "
                      Name@37..40
                        Identifier@37..40 "foo"
                      Colon@40..41 ":"
                      Blankspace@41..42 " "
                      TypeSpecifier@42..45
                        Path@42..45
                          Identifier@42..45 "u32"
                      Blankspace@45..46 " "
                      Equal@46..47 "="
                      Blankspace@47..48 " "
                      Literal@48..49
                        IntLiteral@48..49 "0"
                      Semicolon@49..50 ";"
                    Blankspace@50..51 " "
                    AttributeList@51..56
                      ElseAttribute@51..56
                        AttributeOperator@51..52 "@"
                        Else@52..56 "else"
                    Blankspace@56..57 " "
                    CompoundStatement@57..80
                      BraceLeft@57..58 "{"
                      Blankspace@58..59 " "
                      ConstantDeclaration@59..78
                        Const@59..64 "const"
                        Blankspace@64..65 " "
                        Name@65..68
                          Identifier@65..68 "bar"
                        Colon@68..69 ":"
                        Blankspace@69..70 " "
                        TypeSpecifier@70..73
                          Path@70..73
                            Identifier@70..73 "u32"
                        Blankspace@73..74 " "
                        Equal@74..75 "="
                        Blankspace@75..76 " "
                        Literal@76..77
                          IntLiteral@76..77 "0"
                        Semicolon@77..78 ";"
                      Blankspace@78..79 " "
                      BraceRight@79..80 "}"
                    Blankspace@80..81 " "
                    BraceRight@81..82 "}"
                  Blankspace@82..83 " "
                  AttributeList@83..88
                    ElseAttribute@83..88
                      AttributeOperator@83..84 "@"
                      Else@84..88 "else"
                  Blankspace@88..89 " "
                  CompoundStatement@89..112
                    BraceLeft@89..90 "{"
                    Blankspace@90..91 " "
                    ConstantDeclaration@91..110
                      Const@91..96 "const"
                      Blankspace@96..97 " "
                      Name@97..100
                        Identifier@97..100 "baz"
                      Colon@100..101 ":"
                      Blankspace@101..102 " "
                      TypeSpecifier@102..105
                        Path@102..105
                          Identifier@102..105 "u32"
                      Blankspace@105..106 " "
                      Equal@106..107 "="
                      Blankspace@107..108 " "
                      Literal@108..109
                        IntLiteral@108..109 "0"
                      Semicolon@109..110 ";"
                    Blankspace@110..111 " "
                    BraceRight@111..112 "}"
                  Blankspace@112..113 " "
                  BraceRight@113..114 "}""#]],
    );
}
