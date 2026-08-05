use expect_test::expect;

use crate::name_resolution::tests::check;

#[test]
fn name_resolution_smoke_test() {
    check(
        r#"
//- /shaders.wesl edition:2026_pre
use package::foo::bar::g;

//- /shaders/foo.wesl
fn f() {}

//- /shaders/foo/bar.wesl
fn g() {}
"#,
        expect![[r#"
            package::shaders
            package::shaders::foo
            - fn f
            package::shaders::foo::bar
            - fn g
        "#]],
    );
}

#[test]
fn ignore_outside_of_root() {
    check(
        r#"
//- /shaders/package.wesl package:my_package root:/shaders

//- /shaders/foo.wesl

//- /unreachable.wesl

"#,
        expect![[r#"
            package
            package::foo
        "#]],
    );
}

#[test]
fn wesl_files() {
    check(
        r#"
//- /shaders/package.wesl package:my_package root:/shaders
const a = 3;
//- /shaders.wesl
const hidden = 9;
//- /unrelated.wesl

//- /shaders/foo.wesl

//- /shaders/bar.wesl

//- /shaders/foo/baz.wesl
"#,
        expect![[r#"
            package
            - const a
            package::bar
            package::foo
            package::foo::baz
        "#]],
    );
}

#[test]
fn wesl_shadows_wgsl() {
    check(
        r#"
//- /shaders.wesl

//- /shaders/foo.wesl
const A = 3;

//- /shaders/foo.wgsl
const WGSL = 5;


//- /shaders/bar.wgsl
const WGSL = 3;

//- /shaders/bar.wesl
const A = 5;
"#,
        expect![[r#"
            package::shaders
            package::shaders::bar
            - const A
            package::shaders::foo
            - const A
        "#]],
    );
}

#[test]
fn import_as_test() {
    check(
        r#"
//- /shaders.wesl edition:2026_pre
const Foo = 32;

//- /shaders/bar.wesl
import package::shaders::Foo as MyFoo;
const Bar = package::shaders::Foo + MyFoo;

//- /shaders/foo.wesl
fn Foo() {}
"#,
        expect![[r#"
            package::shaders
            - const Foo
            package::shaders::bar
            - path MyFoo (import)
            - const Bar
            - const MyFoo (import)
            package::shaders::foo
            - fn Foo
        "#]],
    );
}

#[test]
fn import_super() {
    check(
        r#"
//- /package.wesl edition:2026_pre
const foo = 4;

//- /bar.wesl
import package::foo as SimpleFoo;
import super::foo;
"#,
        expect![[r#"
            package
            - const foo
            package::bar
            - path SimpleFoo (import)
            - path foo (import)
            - const SimpleFoo (import)
            - const foo (import)
        "#]],
    );
}

#[test]
fn import_escapes_root() {
    check(
        r#"
//- /package.wesl edition:2026_pre
import super::foo;

//- /bar.wesl
import super::super::bar;
"#,
        expect![[r#"
            package
            error: too many supers
            package::bar
            error: too many supers
        "#]],
    );
}

#[test]
fn name_conflict_with_const() {
    check(
        r#"
//- /package.wesl edition:2026_pre
import package::bar::foo;
const foo = 3;

//- /bar.wesl
const foo = 5;
"#,
        expect![[r#"
            package
            - path foo (import)
            - const foo
            error: name conflict for foo
            package::bar
            - const foo
        "#]],
    );
}

#[test]
fn name_conflict_with_import() {
    check(
        r#"
//- /package.wesl edition:2026_pre
import package::bar::{foo, foo};

//- /bar.wesl
const foo = 5;
"#,
        expect![[r#"
            package
            - path foo (import)
            - const foo (import)
            error: name conflict for foo
            error: name conflict for foo
            package::bar
            - const foo
        "#]],
    );
}

#[test]
fn unresolved_import() {
    check(
        r#"
//- /package.wesl edition:2026_pre
import package::bar::foo;

//- /bar.wesl
"#,
        expect![[r#"
            package
            - path foo (import)
            error: import resolved to neither a module nor an item
            package::bar
        "#]],
    );
}

#[test]
fn import_resolves_to_folder() {
    check(
        r#"
//- /package.wesl edition:2026_pre
import package::bar::foo;

//- /bar/foo/deep.wesl
"#,
        expect![[r#"
            package
            - path foo (import)
            package::bar::foo::deep
        "#]],
    );
}
