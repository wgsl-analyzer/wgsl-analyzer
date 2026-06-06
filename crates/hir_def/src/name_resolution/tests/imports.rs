use expect_test::expect;

use crate::name_resolution::tests::{check, check_modules};

#[test]
fn crate_modules_map_smoke_test() {
    check_modules(
        r#"
//- /shaders.wesl edition:2026_pre
use package::foo::bar::g;

//- /shaders/foo.wesl
fn f() {}

//- /shaders/foo/bar.wesl
fn g() {}
"#,
        expect![[r#"
            package
            package::foo
            package::foo::bar
        "#]],
    );
}

#[test]
fn module_map_ignores_unreachable() {
    // The current implementation ignores files that do not have a corresponding parent
    // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1182

    check_modules(
        r#"
//- /shaders.wesl

//- /shaders/foo.wesl

//- /shaders/bar/unreachable.wesl

//- /shaders/foo/bar.wesl
"#,
        expect![[r#"
            package
            package::foo
            package::foo::bar
        "#]],
    );
}

#[test]
fn module_map_wesl_shadows_wgsl() {
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
            package
            package::bar
            - const A
            package::foo
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
import package::Foo as MyFoo;
const Bar = package::Foo + MyFoo;

//- /shaders/foo.wesl
fn Foo() {}
"#,
        expect![[r#"
            package
            - const Foo
            package::bar
            - const Bar
            - const MyFoo (import)
            package::foo
            - fn Foo
        "#]],
    );
}
