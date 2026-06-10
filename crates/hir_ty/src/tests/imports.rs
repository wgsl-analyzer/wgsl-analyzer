use expect_test::expect;
use syntax::ExtensionsConfig;

use crate::tests::check_infer;

#[test]
fn import_statement_simple() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        import package::foo::bar;
        const output = bar;

        //- /foo.wesl
        import package::foo::utils::barValue;
        const bar = barValue;

        //- /foo/utils.wesl
        const barValue = 3;
        ",
        expect![[r#"
            ---
            32..38 'output': integer
            41..44 'bar': integer
            ---
            44..47 'bar': integer
            50..58 'barValue': integer
            ---
            6..14 'barValue': integer
            17..18 '3': integer
        "#]],
    );
}

#[test]
fn import_statement_inline() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        fn main() {
            package::foo::bar();
            package::foo::utils::barValue();
        }

        //- /foo.wesl
        import package::foo::utils::barValue;
        fn bar() -> f32 { return barValue(); }

        //- /foo/utils.wesl
        fn barValue() -> f32 { return 3; }
        ",
        expect![[r#"
            ---
            16..35 'packag...:bar()': f32
            41..72 'packag...alue()': f32
            ---
            63..73 'barValue()': f32
            ---
            30..31 '3': integer
        "#]],
    );
}

#[test]
fn cannot_import_imported_item() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        const b = package::foo::A; // this should fail, because A is not public

        //- /foo.wesl
        import package::foo::utils::A;

        //- /foo/utils.wesl
        const A = 3;
        ",
        expect![[r#"
            ---
            6..7 'b': integer
            10..25 'package::foo::A': integer
            ---
            ---
            6..7 'A': integer
            10..11 '3': integer
        "#]],
    );
}

#[test]
fn import_statement_multiple_items() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        import package::{foo, foo::bar, foo::utils::bar as boolBar};
        fn main() {
            if(boolBar) {
                let foo = foo::bar + bar;
            }
        }

        //- /foo.wesl
        const bar = 3;

        //- /foo/utils.wesl
        const bar = true;
        ",
        expect![[r#"
            ---
            80..87 'boolBar': bool
            103..106 'foo': i32
            109..117 'foo::bar': integer
            109..123 'foo::bar + bar': integer
            120..123 'bar': integer
            ---
            6..9 'bar': integer
            12..13 '3': integer
            ---
            6..9 'bar': bool
            12..16 'true': bool
        "#]],
    );
}

#[test]
fn import_statement_self_shadowing() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        import package::shadows;

        const shadows = 3;
        const foo = shadows;

        //- /shadows.wesl
        const bar = 3;
        ",
        expect![[r#"
            ---
            32..39 'shadows': integer
            42..43 '3': integer
            51..54 'foo': integer
            57..64 'shadows': integer
            ---
            6..9 'bar': integer
            12..13 '3': integer
        "#]],
    );
}

#[test]
fn import_statement_self_shadowing_error() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        import package::foo::bar; // this should fail, because
        const foo = bar;          // package::foo resolves to this constant

        //- /foo.wesl
        const bar = 3;
        ",
        expect![[r#"
            ---
            61..64 'foo': [error]
            67..70 'bar': [error]
            InferenceDiagnostic { source: Body, kind: InvalidType { error: TypeLoweringError { container: Expression(Idx::<Expression>(0)), kind: UnresolvedPath { path: Path(ModPath { kind: Plain, segments: [Name("bar")] }), failed_segment: 0 } } } }
            InferenceDiagnostic { source: Body, kind: ExpectedLoweredKind { expression: Idx::<Expression>(0), expected: Variable, actual: Type, path: Path(ModPath { kind: Plain, segments: [Name("bar")] }) } }
            ---
            6..9 'bar': integer
            12..13 '3': integer
        "#]],
    );
}

#[test]
fn import_statement_local_shadows() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        import package::foo::bar;
        fn main() {
            let bar = true;
            if(false) {
                let foo = bar;
            }
        }

        //- /foo.wesl
        const bar = 3;
        ",
        expect![[r#"
            ---
            46..49 'bar': bool
            52..56 'true': bool
            65..70 'false': bool
            86..89 'foo': bool
            92..95 'bar': bool
            ---
            6..9 'bar': integer
            12..13 '3': integer
        "#]],
    );
}

#[test]
fn import_statement_local_uses_and_shadows() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        import package::foo::bar;
        fn main() {
            let bar = bar;
            let foo = bar + 3; // bar here is a concrete i32
        }

        //- /foo.wesl
        const bar = 3; // abstract int
        ",
        expect![[r#"
            ---
            46..49 'bar': i32
            52..55 'bar': integer
            65..68 'foo': i32
            71..74 'bar': i32
            71..78 'bar + 3': i32
            77..78 '3': integer
            ---
            6..9 'bar': integer
            12..13 '3': integer
        "#]],
    );
}
