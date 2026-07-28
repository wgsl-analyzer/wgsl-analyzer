#![expect(clippy::non_ascii_literal, reason = "looks better in snapshot")]

use expect_test::expect;
use toolchain::{Tool, require_tool};
use vfs::{AbsPath, AbsPathBuf};

use crate::DiagnosticsConfig;

use super::check_diagnostics_with_config;

#[test]
fn store_type_must_be_storable() {
    require_tool!(Tool::Tint);
    check_diagnostics_with_config(
        &DiagnosticsConfig {
            tint_enabled: true,
            ..DiagnosticsConfig::none()
        },
        "fn foo() { let ambiguous_clamp = clamp(1u, 0, 1i); }",
        expect![[r#"
            33..49 tint Error 15: no matching call to 'clamp(u32, abstract-int, i32)'

            2 candidate functions:
             • 'clamp(T  ✓ , T  ✓ , T  ✗ ) -> T' where:
                  ✓  'T' is 'abstract-float', 'abstract-int', 'f32', 'i32', 'u32' or 'f16'
             • 'clamp(vecN<T>  ✗ , vecN<T>  ✗ , vecN<T>  ✗ ) -> vecN<T>' where:
                  ✗  'T' is 'abstract-float', 'abstract-int', 'f32', 'i32', 'u32' or 'f16'

        "#]],
    );
}
