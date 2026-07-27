//! Test for completions that are WESL-specific.

use crate::tests::completion_list;

#[test]
// TODO https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1323
fn complete_package() {
    assert_eq!(
        completion_list(
            "
            //- /shaders.wesl package:my_package dependencies:other_package edition:2026_pre
            import $0;
            //- /shaders.wesl package:other_package edition:2026_pre
            ",
        ),
        String::new(),
    );
}

#[test]
// TODO https://github.com/wgsl-analyzer/wgsl-analyzer/issues/1323
fn complete_module() {
    assert_eq!(
        completion_list(
            "
            //- /shaders.wesl edition:2026_pre
            import package::$0;
            //- /shaders/foo.wesl
            alias Foo = u32;
            ",
        ),
        String::new(),
    );
}
