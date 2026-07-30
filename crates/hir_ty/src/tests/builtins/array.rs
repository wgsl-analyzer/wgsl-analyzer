#![expect(clippy::too_many_lines, reason = "snapshot test data")]

use expect_test::expect;
use syntax::ExtensionsConfig;

use crate::tests::check_infer;

#[test]
fn arrayLength() {
    check_infer(
        ExtensionsConfig {
            f16: true,
            ..Default::default()
        },
        "
struct S {
    data: array<vec4<f32>>,
};

@group(0) @binding(0)
var<storage, read> storage_read: S;

@group(0) @binding(1)
var<storage, read_write> storage_rw: S;

fn f() {
    let runtime_readonly = arrayLength(&storage_read.data);
    let runtime_readwrite = arrayLength(&storage_rw.data);

    // let uniform_view = bufferView<S>(0);
    // let from_buffer_view = arrayLength(&uniform_view.data);

    // let workgroup_view = bufferArrayView<S>(0);
    // let from_buffer_array_view = arrayLength(&workgroup_view.data);
}
",
        expect![[r#"
            84..96 'storage_read': ref<storage, S, read>
            149..159 'storage_rw': ref<storage, S, read_write>
            182..198 'runtim...adonly': u32
            201..232 'arrayL....data)': u32
            213..231 '&stora...d.data': ptr<storage, array<vec4<f32>>, read>
            214..226 'storage_read': ref<storage, S, read>
            214..231 'storag...d.data': ref<storage, array<vec4<f32>>, read>
            242..259 'runtim...dwrite': u32
            262..291 'arrayL....data)': u32
            274..290 '&stora...w.data': ptr<storage, array<vec4<f32>>, read_write>
            275..285 'storage_rw': ref<storage, S, read_write>
            275..290 'storage_rw.data': ref<storage, array<vec4<f32>>, read_write>
        "#]],
    );
}
