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
fn inline_import_simple() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        const bar = true;
        fn main() {
            let a = package::foo::bar;
        }

        //- /foo.wesl
        const bar = 4;
        ",
        expect![[r#"
            6..9 'bar': bool
            12..16 'true': bool
            38..39 'a': i32
            42..59 'packag...o::bar': integer
            ---
            6..9 'bar': integer
            12..13 '4': integer
        "#]],
    );
}

#[test]
fn import_super() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        const foo = 4;

        //- /module_a.wesl
        const bar = 4;

        //- /module_a/utils.wesl
        import super::bar as superBar;
        fn utility() {
            let a = super::bar;
            let b = super::super::foo;
            let c = superBar;
        }
        ",
        expect![[r#"
            6..9 'foo': integer
            12..13 '4': integer
            ---
            6..9 'bar': integer
            12..13 '4': integer
            ---
            54..55 'a': i32
            58..68 'super::bar': integer
            78..79 'b': i32
            82..99 'super:...r::foo': integer
            109..110 'c': i32
            113..121 'superBar': integer
        "#]],
    );
}

#[test]
fn import_statement_cycle_allowed() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        import package::foo::bar;
        const output = bar;

        //- /foo.wesl
        import package::output;
        const bar = 3;
        ",
        expect![[r#"
            32..38 'output': integer
            41..44 'bar': integer
            ---
            30..33 'bar': integer
            36..37 '3': integer
        "#]],
    );
}

#[test]
fn import_statement_cycle_error() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        import package::foo::bar;
        const output = bar;

        //- /foo.wesl
        import package::output;
        const bar = output;
        ",
        expect![[r#"
            [EditionedFileId(Id(2000))] CyclicType { name: Name("output"), range: 26..45 } in Body
            ---
            [EditionedFileId(Id(2001))] CyclicType { name: Name("bar"), range: 24..43 } in Body
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
            6..7 'b': [error]
            10..25 'package::foo::A': [error]
            10..25 'package::foo::A': `A` is private
            10..25 'package::foo::A': expected variable, but got type `package::foo::A`
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
fn import_statement_package_and_local_same_name() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        import package::foo;
        const foo = 3;
        const bar = foo + foo::a;

        //- /foo.wesl
        const a: u32 = 6;
        ",
        expect![[r#"
            27..30 'foo': integer
            33..34 '3': integer
            42..45 'bar': u32
            48..51 'foo': integer
            48..60 'foo + foo::a': u32
            54..60 'foo::a': u32
            ---
            6..7 'a': u32
            15..16 '6': integer
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

#[test]
fn import_statement_shadows_submodule() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        import package::foo::bar;
        const output = bar;

        //- /foo.wesl
        const bar = 3;

        //- /foo.bar.wesl
        const shadowed = 3;
        ",
        expect![[r#"
            32..38 'output': integer
            41..44 'bar': integer
            ---
            6..9 'bar': integer
            12..13 '3': integer
            ---
            6..14 'shadowed': integer
            17..18 '3': integer
        "#]],
    );
}

#[test]
fn import_statement_shadows_predeclared() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        import package::foo::{bar as vec2f, vec3f};
        const output: vec3f = vec2f;

        //- /foo.wesl
        const bar = 3;
        alias vec3f = u32;
        ",
        expect![[r#"
            50..56 'output': u32
            66..71 'vec2f': integer
            ---
            6..9 'bar': integer
            12..13 '3': integer
        "#]],
    );
}

#[test]
fn import_escapes_root() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /foo.wesl edition:2026_pre
        const_assert(super::super::MyType(3) == true);
        ",
        expect![[r#"
            13..36 'super:...ype(3)': [error]
            13..44 'super:...= true': [error]
            34..35 '3': integer
            40..44 'true': bool
            13..36 'super:...ype(3)': too many `super::`s
        "#]],
    );
}

#[test]
fn import_nonexistent_module() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /foo.wesl edition:2026_pre
        struct Bar {
            a: not_a_module::foo,
        }
        const a = Bar(2);
        ",
        expect![[r#"
            20..37 'not_a_...e::foo': package `not_a_module` not found
            47..48 'a': [error]
            51..57 'Bar(2)': [error]
            55..56 '2': integer
            55..56 '2': expected [error] but got integer
        "#]],
    );
}

#[test]
fn invalid_import_starting_with_item() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /foo.wesl edition:2026_pre
        const bar = 5;

        // The error should point at `nya`. `bar` itself is valid.
        const fails = bar::nya;
        ",
        expect![[r#"
            6..9 'bar': integer
            12..13 '5': integer
            81..86 'fails': [error]
            89..97 'bar::nya': [error]
            89..97 'bar::nya': package `bar` not found
            89..97 'bar::nya': expected variable, but got type `bar::nya`
        "#]],
    );
}

#[test]
fn mod_path_super_display() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl edition:2026_pre
        const Foo = f32(1.0);
        //- /foo.wesl edition:2026_pre
        //- /foo/bar.wesl edition:2026_pre
        const fails = super::super::Foo::nya;
        ",
        expect![[r#"
            6..9 'Foo': f32
            12..20 'f32(1.0)': f32
            16..19 '1.0': float
            ---
            ---
            6..11 'fails': [error]
            14..36 'super:...o::nya': [error]
            14..36 'super:...o::nya': could not find file
            14..36 'super:...o::nya': expected variable, but got type `super::super::Foo::nya`
        "#]],
    );
}

#[test]
fn import_with_basic_dependency() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /bar/package.wesl package:bar edition:2026_pre
        const myValue = 3;

        //- /foo/package.wesl package:foo edition:2026_pre dependencies:bar
        const a = bar::myValue;
        ",
        expect![[r#"
            6..13 'myValue': integer
            16..17 '3': integer
            ---
            6..7 'a': integer
            10..22 'bar::myValue': integer
        "#]],
    );
}

#[test]
fn import_with_nested_dependency() {
    check_infer(
        ExtensionsConfig::default(),
        "
        //- /package.wesl package:my_shaders edition:2026_pre dependencies:nested
        import nested;
        const a = nested::myValue;

        //- /foo.wesl
        const a = nested::myValue;

        //- /nested_shaders/package.wesl package:nested edition:2026_pre
        const myValue = 3;
        ",
        expect![[r#"
            21..22 'a': integer
            25..40 'nested::myValue': integer
            ---
            6..7 'a': integer
            10..25 'nested::myValue': integer
            ---
            6..13 'myValue': integer
            16..17 '3': integer
        "#]],
    );
}
