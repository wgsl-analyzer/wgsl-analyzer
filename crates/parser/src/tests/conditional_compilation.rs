use expect_test::expect;

use crate::tests::check;

#[test]
fn module_compound() {
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
fn module_compound_nested() {
    check(
        "
        @if(true) { fn foo() {} { @if(true) fn bar() {} } @if(true) { fn baz() {} } }
        ",
        expect![[r#"
            SourceFile@0..95
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
              GlobalCompoundDeclaration@19..86
                BraceLeft@19..20 "{"
                Blankspace@20..21 " "
                FunctionDeclaration@21..32
                  Fn@21..23 "fn"
                  Blankspace@23..24 " "
                  Name@24..27
                    Identifier@24..27 "foo"
                  FunctionParameters@27..29
                    ParenthesisLeft@27..28 "("
                    ParenthesisRight@28..29 ")"
                  Blankspace@29..30 " "
                  CompoundStatement@30..32
                    BraceLeft@30..31 "{"
                    BraceRight@31..32 "}"
                Blankspace@32..33 " "
                GlobalCompoundDeclaration@33..58
                  BraceLeft@33..34 "{"
                  Blankspace@34..35 " "
                  AttributeList@35..44
                    IfAttribute@35..44
                      AttributeOperator@35..36 "@"
                      If@36..38 "if"
                      ParenthesisLeft@38..39 "("
                      Literal@39..43
                        True@39..43 "true"
                      ParenthesisRight@43..44 ")"
                  Blankspace@44..45 " "
                  FunctionDeclaration@45..56
                    Fn@45..47 "fn"
                    Blankspace@47..48 " "
                    Name@48..51
                      Identifier@48..51 "bar"
                    FunctionParameters@51..53
                      ParenthesisLeft@51..52 "("
                      ParenthesisRight@52..53 ")"
                    Blankspace@53..54 " "
                    CompoundStatement@54..56
                      BraceLeft@54..55 "{"
                      BraceRight@55..56 "}"
                  Blankspace@56..57 " "
                  BraceRight@57..58 "}"
                Blankspace@58..59 " "
                AttributeList@59..68
                  IfAttribute@59..68
                    AttributeOperator@59..60 "@"
                    If@60..62 "if"
                    ParenthesisLeft@62..63 "("
                    Literal@63..67
                      True@63..67 "true"
                    ParenthesisRight@67..68 ")"
                Blankspace@68..69 " "
                GlobalCompoundDeclaration@69..84
                  BraceLeft@69..70 "{"
                  Blankspace@70..71 " "
                  FunctionDeclaration@71..82
                    Fn@71..73 "fn"
                    Blankspace@73..74 " "
                    Name@74..77
                      Identifier@74..77 "baz"
                    FunctionParameters@77..79
                      ParenthesisLeft@77..78 "("
                      ParenthesisRight@78..79 ")"
                    Blankspace@79..80 " "
                    CompoundStatement@80..82
                      BraceLeft@80..81 "{"
                      BraceRight@81..82 "}"
                  Blankspace@82..83 " "
                  BraceRight@83..84 "}"
                Blankspace@84..85 " "
                BraceRight@85..86 "}"
              Blankspace@86..95 "\n        ""#]],
    );
}

#[test]
fn module_compound_shadow() {
    check(
        "
        { const foo: u32 = 0; } const foo: u32 = 1;
        ",
        expect![[r#"
            SourceFile@0..61
              Blankspace@0..9 "\n        "
              GlobalCompoundDeclaration@9..32
                BraceLeft@9..10 "{"
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
                BraceRight@31..32 "}"
              Blankspace@32..33 " "
              ConstantDeclaration@33..52
                Const@33..38 "const"
                Blankspace@38..39 " "
                Name@39..42
                  Identifier@39..42 "foo"
                Colon@42..43 ":"
                Blankspace@43..44 " "
                TypeSpecifier@44..47
                  Path@44..47
                    Identifier@44..47 "u32"
                Blankspace@47..48 " "
                Equal@48..49 "="
                Blankspace@49..50 " "
                Literal@50..51
                  IntLiteral@50..51 "1"
                Semicolon@51..52 ";"
              Blankspace@52..61 "\n        ""#]],
    );
}

#[test]
fn function_compound() {
    check(
        "
        fn foo() { @if(true) { var x = 0; } x++; }
        ",
        expect![[r#"
            SourceFile@0..60
              Blankspace@0..9 "\n        "
              FunctionDeclaration@9..51
                Fn@9..11 "fn"
                Blankspace@11..12 " "
                Name@12..15
                  Identifier@12..15 "foo"
                FunctionParameters@15..17
                  ParenthesisLeft@15..16 "("
                  ParenthesisRight@16..17 ")"
                Blankspace@17..18 " "
                CompoundStatement@18..51
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
                  CompoundStatement@30..44
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
                    BraceRight@43..44 "}"
                  Blankspace@44..45 " "
                  IncrementDecrementStatement@45..49
                    IdentExpression@45..46
                      Path@45..46
                        Identifier@45..46 "x"
                    PlusPlus@46..48 "++"
                    Semicolon@48..49 ";"
                  Blankspace@49..50 " "
                  BraceRight@50..51 "}"
              Blankspace@51..60 "\n        ""#]],
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
