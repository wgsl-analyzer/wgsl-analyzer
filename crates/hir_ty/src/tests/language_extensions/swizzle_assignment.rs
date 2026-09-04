use expect_test::expect;

use crate::{
    tests::{check_infer, check_infer_with_verbosity},
    ty::pretty::TypeVerbosity,
};

#[test]
fn reading_and_writing_via_swizzle_views_on_references() {
    check_infer(
        "
// requires swizzle_assignment;

fn swizzle_read_and_write() {
    var v: vec4u;

    // Same as:  v.y = 1; v.z = 2;
    // However, the vector is read from memory and written to memory exactly once.
    v.yz = vec2u(1,2);

    // Same as: let u = vec3(v.a, v.g, v.b);
    // Expression v.agb is a swizzle view of type swizzle<function,u32,4,3>.
    // The effective-value-type of a let declaration must be concrete constructible
    // or a pointer type.  So the swizzle view is automatically converted to vec3u
    // by invoking the Swizzle View Load Rule.
    let u = v.agb;

    // Swizzles can be chained.
    // Same as: v.zy = vec2(99,100);
    v.yz.yx = vec2(99,100);

    // Applying a single letter on a swizzle view yields a reference
    // to a scalar component of the underlying vector.
    // Same as: v.z = 99;
    v.yz.y = 99;

    // Indexing a swizzle view yields a reference to one of the vector's
    // components. It acts as if forming a single-letter vector access,
    // but the index chooses which letter to use from the swizzle name.
    v.zy[1] = 50; // Same as v.y = 50;
}
        ",
        expect![[r#"
            71..72 'v': ref<function, vec4<u32>, read_write>
            204..205 'v': ref<function, vec4<u32>, read_write>
            204..208 'v.yz': swizzle<function, u32, 4, 2>
            211..221 'vec2u(1,2)': vec2<u32>
            217..218 '1': integer
            219..220 '2': integer
            568..569 'u': vec3<u32>
            572..573 'v': ref<function, vec4<u32>, read_write>
            572..577 'v.agb': swizzle<function, u32, 4, 3>
            653..654 'v': ref<function, vec4<u32>, read_write>
            653..657 'v.yz': swizzle<function, u32, 4, 2>
            653..660 'v.yz.yx': swizzle<function, u32, 2, 2>
            663..675 'vec2(99,100)': vec2<integer>
            668..670 '99': integer
            671..674 '100': integer
            832..833 'v': ref<function, vec4<u32>, read_write>
            832..836 'v.yz': swizzle<function, u32, 4, 2>
            832..838 'v.yz.y': ref<function, u32, read_write>
            841..843 '99': integer
            1067..1068 'v': ref<function, vec4<u32>, read_write>
            1067..1071 'v.zy': swizzle<function, u32, 4, 2>
            1067..1074 'v.zy[1]': ref<function, u32, read_write>
            1072..1073 '1': integer
            1077..1079 '50': integer
        "#]],
    );
}

#[test]
fn reading_and_writing_via_swizzle_views_on_pointers() {
    check_infer(
        "
// requires pointer_composite_access, swizzle_assignment;

fn swizzle_read_and_write_via_pointer(p: ptr<function,vec4u>) {
    // Same as:  (*p).y = 1; (*p).z = 2;
    p.yz = vec2u(1,2);

    // Same as: let u = vec3((*p).a, (*p).g, (*p).b);
    let u = p.agb;

    // Swizzles can be chained.
    // Same as: p.zy = vec2(99,100);
    p.yz.yx = vec2(99,100);

    // A swizzle view can be constructed from a pointer.
    // Applying a single letter to a swizzle view yields a reference
    // to a scalar component of the underlying vector.
    // Same as: (*p).z = 99;
    p.yz.y = 99;

    // Indexing a swizzle view yields a reference to one of the vector's
    // components. It acts as if forming a single-letter vector access,
    // but the index chooses which letter to use from the swizzle name.
    p.zy[1] = 50; // Same as (*p).y = 50;
}
        ",
        expect![[r#"
            97..98 'p': ptr<function, vec4<u32>, read_write>
            168..169 'p': ptr<function, vec4<u32>, read_write>
            168..172 'p.yz': swizzle<function, u32, 4, 2>
            175..185 'vec2u(1,2)': vec2<u32>
            181..182 '1': integer
            183..184 '2': integer
            250..251 'u': vec3<u32>
            254..255 'p': ptr<function, vec4<u32>, read_write>
            254..259 'p.agb': swizzle<function, u32, 4, 3>
            335..336 'p': ptr<function, vec4<u32>, read_write>
            335..339 'p.yz': swizzle<function, u32, 4, 2>
            335..342 'p.yz.yx': swizzle<function, u32, 2, 2>
            345..357 'vec2(99,100)': vec2<integer>
            350..352 '99': integer
            353..356 '100': integer
            574..575 'p': ptr<function, vec4<u32>, read_write>
            574..578 'p.yz': swizzle<function, u32, 4, 2>
            574..580 'p.yz.y': ref<function, u32, read_write>
            583..585 '99': integer
            809..810 'p': ptr<function, vec4<u32>, read_write>
            809..813 'p.zy': swizzle<function, u32, 4, 2>
            809..816 'p.zy[1]': ref<function, u32, read_write>
            814..815 '1': integer
            819..821 '50': integer
        "#]],
    );
}

#[test]
fn invalid_swizzle_views() {
    check_infer(
        "
// requires pointer_composite_access, swizzle_assignment;

fn swizzle_read_and_write_via_pointer(p: ptr<function,vec4u>) {
    // Same as:  (*p).y = 1; (*p).z = 2;
    p.yz = vec2u(1,2);

    // Same as: let u = vec3((*p).a, (*p).g, (*p).b);
    let u = p.agb;

    // Swizzles can be chained.
    // Same as: p.zy = vec2(99,100);
    p.yz.yx = vec2(99,100);

    // A swizzle view can be constructed from a pointer.
    // Applying a single letter to a swizzle view yields a reference
    // to a scalar component of the underlying vector.
    // Same as: (*p).z = 99;
    p.yz.y = 99;

    // Indexing a swizzle view yields a reference to one of the vector's
    // components. It acts as if forming a single-letter vector access,
    // but the index chooses which letter to use from the swizzle name.
    p.zy[1] = 50; // Same as (*p).y = 50;
}
        ",
        expect![[r#"
            97..98 'p': ptr<function, vec4<u32>, read_write>
            168..169 'p': ptr<function, vec4<u32>, read_write>
            168..172 'p.yz': swizzle<function, u32, 4, 2>
            175..185 'vec2u(1,2)': vec2<u32>
            181..182 '1': integer
            183..184 '2': integer
            250..251 'u': vec3<u32>
            254..255 'p': ptr<function, vec4<u32>, read_write>
            254..259 'p.agb': swizzle<function, u32, 4, 3>
            335..336 'p': ptr<function, vec4<u32>, read_write>
            335..339 'p.yz': swizzle<function, u32, 4, 2>
            335..342 'p.yz.yx': swizzle<function, u32, 2, 2>
            345..357 'vec2(99,100)': vec2<integer>
            350..352 '99': integer
            353..356 '100': integer
            574..575 'p': ptr<function, vec4<u32>, read_write>
            574..578 'p.yz': swizzle<function, u32, 4, 2>
            574..580 'p.yz.y': ref<function, u32, read_write>
            583..585 '99': integer
            809..810 'p': ptr<function, vec4<u32>, read_write>
            809..813 'p.zy': swizzle<function, u32, 4, 2>
            809..816 'p.zy[1]': ref<function, u32, read_write>
            814..815 '1': integer
            819..821 '50': integer
        "#]],
    );
}

#[test]
fn swizzle_writes_to_different_vector_components_may_race() {
    check_infer(
        "
// requires swizzle_assignment;

var<workgroup> w: vec4u;

@compute @workgroup_size(2)
fn this_races(@builtin(local_invocation_index) gid: u32) {
    if (gid == 0) {
    w.xy = vec2u(0,1);  // Writes the whole vector, races with other invocation.
    } else {
    w.zw = vec2u(2,3);  // Writes the whole vector, races with other invocation.
    }
}
        ",
        expect![[r#"
            48..49 'w': ref<workgroup, vec4<u32>, read_write>
            134..137 'gid': u32
            154..157 'gid': u32
            154..162 'gid == 0': bool
            161..162 '0': integer
            170..171 'w': ref<workgroup, vec4<u32>, read_write>
            170..174 'w.xy': swizzle<workgroup, u32, 4, 2>
            177..187 'vec2u(0,1)': vec2<u32>
            183..184 '0': integer
            185..186 '1': integer
            264..265 'w': ref<workgroup, vec4<u32>, read_write>
            264..268 'w.zw': swizzle<workgroup, u32, 4, 2>
            271..281 'vec2u(2,3)': vec2<u32>
            277..278 '2': integer
            279..280 '3': integer
        "#]],
    );
}

#[test]
fn swizzle_swizzle() {
    check_infer(
        "
// requires swizzle_assignment;
fn foo() {
    var v = vec2();
    let s = v.yx.xy;
}
        ",
        expect![[r#"
            51..52 'v': ref<function, vec2<i32>, read_write>
            55..61 'vec2()': vec2<integer>
            71..72 's': vec2<i32>
            75..76 'v': ref<function, vec2<i32>, read_write>
            75..79 'v.yx': swizzle<function, i32, 2, 2>
            75..82 'v.yx.xy': swizzle<function, i32, 2, 2>
        "#]],
    );
}

#[test]
fn swizzle_swizzle_too_high_one() {
    check_infer(
        "
// requires swizzle_assignment;
fn foo() {
    var v = vec2();
    let s = v.yx.z;
}
        ",
        expect![[r#"
            51..52 'v': ref<function, vec2<i32>, read_write>
            55..61 'vec2()': vec2<integer>
            71..72 's': [error]
            75..76 'v': ref<function, vec2<i32>, read_write>
            75..79 'v.yx': swizzle<function, i32, 2, 2>
            75..81 'v.yx.z': [error]
            75..81 'v.yx.z': no such field `z` on type `swizzle<function, i32, 2, 2>`
        "#]],
    );
}

#[test]
fn swizzle_swizzle_too_high_multiple() {
    check_infer(
        "
// requires swizzle_assignment;
fn foo() {
    var v = vec2();
    let s = v.yx.zz;
}
        ",
        expect![[r#"
            51..52 'v': ref<function, vec2<i32>, read_write>
            55..61 'vec2()': vec2<integer>
            71..72 's': [error]
            75..76 'v': ref<function, vec2<i32>, read_write>
            75..79 'v.yx': swizzle<function, i32, 2, 2>
            75..82 'v.yx.zz': [error]
            75..82 'v.yx.zz': no such field `zz` on type `swizzle<function, i32, 2, 2>`
        "#]],
    );
}

#[test]
fn swizzle_swizzle_bad() {
    check_infer(
        "
// requires swizzle_assignment;
fn foo() {
    var v = vec2();
    let s = v.yx.c;
}
        ",
        expect![[r#"
            51..52 'v': ref<function, vec2<i32>, read_write>
            55..61 'vec2()': vec2<integer>
            71..72 's': [error]
            75..76 'v': ref<function, vec2<i32>, read_write>
            75..79 'v.yx': swizzle<function, i32, 2, 2>
            75..81 'v.yx.c': [error]
            75..81 'v.yx.c': no such field `c` on type `swizzle<function, i32, 2, 2>`
        "#]],
    );
}

// TODO: once support for the enable extension is actually added, do this test but with read_write
#[test]
fn readonly_ref_vec_is_not_swizzle() {
    check_infer(
        "
// Has access mode 'read'
@group(0) @binding(0) var<storage> robuf: vec4u;

fn foo() {
    robuf.xz = vec2u();
}
        ",
        expect![[r#"
            61..66 'robuf': ref<storage, vec4<u32>, read>
            91..96 'robuf': ref<storage, vec4<u32>, read>
            91..99 'robuf.xz': vec2<u32>
            102..109 'vec2u()': vec2<u32>
            91..99 'robuf.xz': cannot assign to non-reference `vec2<u32>`
        "#]],
    );
}

#[test]
fn swizzle_compact() {
    check_infer_with_verbosity(
        TypeVerbosity::Compact,
        "
fn foo() {
    var v = vec2();
    let s = v.yx;
}
        ",
        expect![[r#"
            19..20 'v': ref<vec2<i32>>
            23..29 'vec2()': vec2<integer>
            39..40 's': vec2<i32>
            43..44 'v': ref<vec2<i32>>
            43..47 'v.yx': swizzle<vec2<i32>, 2>
        "#]],
    );
}

#[test]
fn swizzle_inner() {
    check_infer_with_verbosity(
        TypeVerbosity::Inner,
        "
fn foo() {
    var v = vec2();
    let s = v.yx;
}
        ",
        expect![[r#"
            19..20 'v': vec2<i32>
            23..29 'vec2()': vec2<integer>
            39..40 's': vec2<i32>
            43..44 'v': vec2<i32>
            43..47 'v.yx': vec2<i32>
        "#]],
    );
}

#[test]
fn swizzle_conversion_rank_nonzero() {
    check_infer(
        "
fn foo() {
    var v = vec2(1);
    bar(v.yx);
}

fn bar(v: vec2<u32>) { }
        ",
        expect![[r#"
            19..20 'v': ref<function, vec2<i32>, read_write>
            23..30 'vec2(1)': vec2<integer>
            28..29 '1': integer
            36..45 'bar(v.yx)': [error]
            40..41 'v': ref<function, vec2<i32>, read_write>
            40..44 'v.yx': swizzle<function, i32, 2, 2>
            40..44 'v.yx': expected vec2<u32> but got swizzle<function, i32, 2, 2>
            57..58 'v': vec2<u32>
        "#]],
    );
}

#[test]
fn missing_swizzle_assignment() {
    check_infer(
        "
// explicitly not testing swizzle_assignment
// currently have to make the access mode 'read' to get this behavior
@group(0) @binding(0) var<storage> robuf: vec4u;

fn foo() {
    robuf.xz = vec2u();
}
        ",
        expect![[r#"
            150..155 'robuf': ref<storage, vec4<u32>, read>
            180..185 'robuf': ref<storage, vec4<u32>, read>
            180..188 'robuf.xz': vec2<u32>
            191..198 'vec2u()': vec2<u32>
            180..188 'robuf.xz': cannot assign to non-reference `vec2<u32>`
        "#]],
    );
}

#[test]
fn concrete_swizzle() {
    check_infer(
        "
// enable swizzle_assignment;
fn foo() {
    let v = vec2().xy;
}
        ",
        expect![[r#"
            49..50 'v': vec2<i32>
            53..59 'vec2()': vec2<integer>
            53..62 'vec2().xy': vec2<integer>
        "#]],
    );
}
