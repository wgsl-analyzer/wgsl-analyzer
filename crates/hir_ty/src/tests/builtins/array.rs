use expect_test::expect;

use crate::tests::check_infer;

#[test]
fn arrayLength() {
    check_infer(
        "
enable f16;
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
            96..108 'storage_read': ref<storage, S, read>
            161..171 'storage_rw': ref<storage, S, read_write>
            194..210 'runtim...adonly': u32
            213..244 'arrayL....data)': u32
            225..243 '&stora...d.data': ptr<storage, array<vec4<f32>>, read>
            226..238 'storage_read': ref<storage, S, read>
            226..243 'storag...d.data': ref<storage, array<vec4<f32>>, read>
            254..271 'runtim...dwrite': u32
            274..303 'arrayL....data)': u32
            286..302 '&stora...w.data': ptr<storage, array<vec4<f32>>, read_write>
            287..297 'storage_rw': ref<storage, S, read_write>
            287..302 'storage_rw.data': ref<storage, array<vec4<f32>>, read_write>
        "#]],
    );
}
