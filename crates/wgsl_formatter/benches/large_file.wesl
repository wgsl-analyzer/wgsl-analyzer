    import aaaaaaaaa::{bbbbbbbb::{cdddddd::{dddddd::{eeeeeee::{ffffffff::{gggggg::{hhhhhh::{iiiiii::jjjjjjjjjj, kkkkkkkk}}}}}}}};
    import aaaaaaaaa::bbbbbbbb::{cdddddd::dddddd::{eeeeeee::ffffffff::{gggggg::hhhhhh::{iiiiii::jjjjjjjjjj, kkkkkkkk}}}};
    @if(THING)
    import the::thing;
    import a;


    import a::b::c::d::e::f;


    import package::b::c;


    import super::b::c;


    import super::super::super::super::c;


    import a::b::c;
    import a::b::d;


    import a::b::c;
    import d::e::f;


    import a::b::c as d;


    import a::{ZZZ,
YYY,
    XXX};


    import a::{a::x, a::m, a::a};


    import a::{a::x::a, a::m::a, a::a::a};


    import a::{a::x, a::a, a::m::a};


    import a::{a::x::a, a::a::a, a::{b,c}};


    import a::{b,
    c,
    d,};


import package::tracer_plugin::renderer::buffers::{
a as b, c as d, e as f, ffffffffffffffffff, aaaaaaaaaaaaa,
aaaaaaaaaaaaaaa::bbbbbbbbbbbbbbbbbbbbbb as cccccca, gggggggggggg
};



   diagnostic
   (off, something);


   diagnostic
   (off, something.something);


   diagnostic
   (off, something
   .
   something);


   enable f16,
   clip_distances
   ,dual_source_blending;


    //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
    enable f16, clip_distances, dual_source_blending, subgroups, primitive_index, subgroup_size_control, wgpu_mesh_shader, wgpu_ray_query, wgpu_ray_query_vertex_return, wgpu_ray_tracing_pipeline, wgpu_int16, wgpu_cooperative_matrix, per_vertex, draw_index, wgpu_binding_array;


    requires
    uniform_buffer_standard_layout
    ,
    subgroup_id,;


    //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
   requires
   uniform_buffer_standard_layout,subgroup_id,subgroup_uniformity,texture_and_sampler_let,texture_formats_tier1,linear_indexing,immediate_address_space,buffer_view;




        alias Test =
        array<i32, // Length
        17>
        ;


        alias Test =
        array<
        array<i32, // Length
        17>,18>
        ;


        alias Test =
        array<
11 /* after 11 */,
12, /* after 12 */
/*before 13*/ 13,
/*line before 14*/
14,
// line before 15
15,

16, // after 16

17
// line after 17
>
        ;


        fn main() {
            let a = 1 + (((((((((1+1)))))))));
        }


        fn main() {
            let a = 1 + (2);
        }


        @if(true)
        {
        fn a() {}
        }
        @if(false)
        {
        fn a() {}
        }
        @else
        {
        fn a() {}
        }


        @if(true)
        {
        fn a() {}
        }
        @elif(false)
        {
        fn a() {}
        }
        @else
        {
        fn a() {}
        }


        fn main() {
        @if(true)
        {
            var x: u32;
        }
        @if(false)
        {
            var x: u32;
        }
        @else
        {
            var x: u32;
        }
            return x;
        }


        fn main() {
        @if(true)
        {
            var x: u32;
        }
        @elif(false)
        {
            var x: u32;
        }
        @else
        {
            var x: u32;
        }
            return x;
        }


        fn main() {
            @if(true) {
            var x: u32;
            } @elif(false) {
            var x: u32;
            } @else {
            var x: u32;
            }
            return x;
        }


        fn main() {
        @if(true)
        var x: u32;
        @elif(false)
        var x: u32;
        @else
        var x: u32;
            return x;
        }


        fn a() {}
        fn b() {}
        {
        fn c() {}fn d() {}
        }
        fn e() {}


        fn a() {}
        fn b() {}
        { fn c() {}fn d() {} }
        fn e() {}


        fn a() {}
        fn b() {}
        {
            fn c() {}fn d() {}
        }
        fn e() {}


        fn a() {}
        { fn b() {} }
        fn c() {}

        { fn d() {} }
        fn e() {}

        { fn f() {} }

        fn g() {}


        fn a() {}
        @if(true)
        { fn b() {} }
        fn c() {}

        @else
        { fn b() {} }


        {




        fn a() {}
        fn b() {}

        fn c() {}


        }


        { // Same Line
        fn a () {}
        }

        {
        // Next
        fn a () {}
        }

        {
        } // Same Line

        {
        }
        // Next Line


        { /* Same Line */
        fn a () {}
        }

        {
        /* Next */
        fn a () {}
        }

        {
        } /* Same Line */

        {
        }
        /* Next Line */


        {{{
        fn a() {}
        }}}


// @wgslfmt(ignore)
fn
a
(
u: u32
) {
let
a=1
;
}

/* @wgslfmt(ignore) */
fn
a
(
u: u32
) {
let
a=1
;
}


fn main() {
    // @wgslfmt(ignore)
    let a = mat3x3(
        1.,           2.,       3.,
        4., long_fn_bla(2.),    3.,
        5.,           2.,       3.,

    );
}


fn main() {
    // @wgslfmt(ignore)
    let verts: array<u32, 16> = array(
        1., 0., 0., 1., // The thing
        4., 0., 0., 1., // The second vertex
        1., 7., 0., 1.,//Who added this comment?!
        1., 0., 0., 1., // The last vertex!
    );
}


fn main() {
    // @wgslfmt(ignore)
    if ( vec3<   f32>(0.0)  >1.0 ) {}

    /* @wgslfmt(ignore) */
    if ( vec3<   f32>(0.0)  >1.0 ) {}
}


fn main() {

    // @wgslfmt(ignore)
    if ( vec3<   f32>(0.0)  >1.0 ) {

    }

}


fn main() {


    // @wgslfmt(ignore)
    if ( vec3<   f32>(0.0)  >1.0 ) {


    }


}


fn main() {
    // @wgslfmt(ignore)
    let a = 1 + 2
        +   3 - 4;
}


fn main() {

let a = thing(
1,
2,
/* @wgslfmt(ignore) */ vec3<   f32>(1.0,
0.0,
1.0),
3
);

}


fn bla(/* @wgslfmt(ignore) */ a:             u32          ,          b: u32) {}


        // @!wgslfmt(ignore)
fn bla(a:             u32          ,          b: u32) {}

        // @!wgslfmt(ignore)
fn bla(a:             u32          ,          b: u32) {}





fn
a
(
u: u32
) {
// @!wgslfmt(ignore)
let
a=1
;
}

fn
a
(
u: u32
) {
/* @!wgslfmt(ignore) */
let
a=1
;
}


fn a() {
    loop{
        /* AAA
    AAA
        AAA
        */
    }
}


        alias Test = vec3f;

        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        alias Teeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeest = vec3f;
        ;


        fn main() {
            b[(1+1)]   = 0;
        }


        fn main() {
            b[   1+1    ]   = 0;
        }


        fn main() {
            b[   // A
            1+1    ]   = 0;
        }


        fn main() {
        aaaaaaaaaaa::bbbbbbbbbbbbbbbbb[
            curr_idx
        ] = ccccccc::dddddddddddddddd(
                trav_result.hit,
                trav_result.local_position,
            );
        }

    fn main() {



        }
    fn main() {/* Hello */}
    fn main(
            // This comment describes A
            a: u32,
            // This comment describes B
            b: u32,

            c: u32,



            d: u32
        ) {}

            fn main(
a: u32 /* after a */,
b: u32, /* after b */
/*before c*/ c: u32,
/*line before d*/
d: u32,

                // line before e
                e: u32,

                f: u32, // after f

                g: u32
                // line after g
) {}

    fn main(
            a: u32, // This comment describes B
            b: u32, // This comment describes C
            // This comment describes C
            c: u32,
            // This comment describes D
            d: u32
        ) {}
    fn main (a: b /*fff*/) -> f32 {}
    fn main (a: b // Hi
        ) -> f32 {}

        alias Test =
        i32
        ;


        alias Test =
        array<i32, 17>
        ;


        alias Test =
        array<
        array
        <i32,
        19>,
        17>
        ;


        alias Test =
        array<
        array

        <i32, // Miep
        19>,
        17>
        ;

    fn main() {
        loop {
        }


        }
    fn main() {
        continue;
        }

        fn main() {
            @if(true)
            {
                var x: u32;
            }
            @elif(false)
            {
                var x: u32;
            }
            @else
            {
                var x: u32;
            }
            return x;
        }


        fn main() {
            @if(true)
            {{
                var x: u32;
            }}
            @elif(false)
            {{
                var x: u32;
            }}
            @else
            {{
                var x: u32;
            }}
            return x;
        }

    fn main() {
        let a = 1 + (1+1);
        }
    fn main() {
        let a = 1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+1))))))))))))))))))))))))))))))))))))))))))))))))));
        }
    fn main() {
        loop {


        }
        }

        const_assert(x<y);
        const_assert
        (x
        < y
        );

    struct Foo {a: i32,b:i32}
    struct Foo {
            // This comment describes A
            a: i32,
            // This comment describes B
            b:i32
        }
    struct Foo {
            a: i32,
            b: i32,

            c:i32,


            d:i32,
        }
    struct Foo {


            a: i32,
        }
    struct Foo {
            a: i32,


        }

        struct A { // This comment should stick to the member
        a: i32
        }

    struct Foo {
            // This comment describes A
            a: i32,
            b: i32,

            // This comment describes C
            // This comment also describes C
            c:i32,

            d:i32, // This comment describes d
            e:i32, // This comment describes e

            // A lonesome comment?

            d: i32
        }
    struct Foo {
            /* This comment describes A */
            a: i32,
            b: i32,

            /* This comment describes C */
            /* This comment also describes C */
            c:i32,

            d:i32, /* This comment describes d */
            e:i32, /* This comment describes e */

            /* A lonesome comment? */

            d: i32
        }

        struct VertexInput {
            @builtin(vertex_index) vertexIndex: u32,
        };


        struct VertexInput {vertexIndex: u32};

    fn main() {
    if(x < 1){}
    if    x < 1     {}
}
    fn main() {
        if
        false
        {
        }
        else
        {
        }
        }
    fn main() {
        if
        false
        {
        }
        else
        {
        // HI
        }
        }

        fn main() {
        if blaaa < aaaaaaaaaaaaaaa + bbbbbbbbbbbbb // Force Break
        + ccccccccccccc + ddddddddddddddd + eeeeeeeeeeeeee
        {
            return d;
        }
        }


        fn main() {
        if a {
        return a;
        }
        else if blaaa < aaaaaaaaaaaaaaa + bbbbbbbbbbbbb // Force Break
        + ccccccccccccc + ddddddddddddddd + eeeeeeeeeeeeee
        {
            return d;
        }
        }

    fn main() {
        bla(12, // Force break
        bar(), 1 + vubble);
        }
    fn main() {
        bla(12, bar(), 1 + vubble, );
        }
    fn main() {
    min(
        min(
            1, // Force break
            2,
        ), min(1,2)
    );
}
    fn main() {
    my_function<f32>(x,y,z);
    my_function<array<f32, 28>>(x,y,z);
}
    fn main() {
        bla(12, bar() /* a */    );
        }
    fn main() {
            bla(
                11 /* after 11 */,
                12, /* after 12 */
                /*before 13*/ 13,
                /*line before 14*/
                14,

                // line before 15
                15,

                16, // after 16

                17
                // line after 17
            );
        }
    fn main() {
        while true {
        }


        }
    fn main() {
        while (1+2==3) {}


        }
    fn main() {
        while aaaaaaaaaaaaa + bbbbbbbbbbbbbbbbbb // Force break
        < ccccccccccccccccc + ddddddddddddddd {
        a += b;
        }


        }


        override a /* a */ ;



    fn main() {

        const a = 1 /* A */ ;


        }
    fn main() {

        let a = 1 /* A */ ;


        }


        override a = 1 /* A */ ;



    fn main() {

        var a = 1 /* A */ ;


        }
    struct VertexOutput {
            @align(7)
            @size(9)
            @offset(28)
            a: u32,
            @align(7)
            @location(1)
            @size(9)
            @offset(28)
            b: u32,
        }

        @blaa(off)
        @binding(1)
        @group(0)
        var<storage> a: b;


        @workgroup_size(1,1,1)
        @compute
        fn main() {
        }


        @bla()
        @const
        @blo()
        fn thing() {}


        @bla()
        @must_use
        @blo()
        fn thing() {}


        @must_use
        @const
        fn thing() {}

    struct VertexOutput {
            @attr(0) @attr(1) position: vec4<f32>,
            @attr(0) @attr(1) uv: vec2<f32>,
        }
    @attr(0)
        @attr(1)
        fn main(
        ) {
        }

        fn thing(
        ) -> @attr(0) @attr(1) vec4<f32> {
        }

        fn thing(
            @attr(0) @attr(1) position: vec4<f32>,
            @attr(0) @attr(1) uv: vec2<f32>,
        ) -> vec4<f32> {
        }

        @attr(0) @attr(1) var<uniform> material: CustomMaterial;


        @attr(0) @attr(1) override amount: u64 = 0;


        fn main() {
            @attr(0) @attr(1) {}
            if true @attr(0) @attr(1) {}
        }


        fn main() {
            @attr(0) @attr(1) {
                let a = 0;
            }
            if true @attr(0) @attr(1) {
                let a = 0;
            }
        }


        fn main() {
            @attr(0) @attr(1) if true {}
        }


        fn main() {
            @attr(0) @attr(1) switch a {}
        }


        fn main() {
            switch a @attr(0) @attr(1) {}
        }


        fn main() {
            @attr(0) @attr(1) loop {}
        }


        fn main() {
            loop @attr(0) @attr(1) {}
        }


        fn main() {
            @attr(0) @attr(1) for(var i = 0; i < 10; i++) {}
        }


        fn main() {
        loop{
        }

}

        fn main() {
        @attr(0) @attr(1)
        while true {}
        }




        @const
        @align(1)
        @binding(1)
        @blend_src(1)
        @builtin(position)
        @group(1)
        @id(1)
        @interpolate(flat)
        @invariant
        @location(0)
        @must_use
        @size(1)
        @workgroup_size(1,2,3)
        @vertex
        @fragment
        @compute
        @if(true)
        @elif(false)
        @else
        fn a() {}



            fn a() {}


        @must_use //Hello
        fn a() {}

        @fragment //Hello
        fn b() {}


        @must_use /* Hello */
        fn a() {}

        @fragment /* Hello */
        fn b() {}

        @must_use
        /* Hello */
        fn a() {}

        @fragment
        /* Hello */
        fn b() {}


@if(true)
/**
 * Doc comment that should probably go between @if and @fragment
 */
@fragment
fn a() {}


        @must_use /* @some_inline_attr */ fn a() {}

        @fragment
        /* @some_inline_attr */ fn b() {}

        @fragment
        /* @some_noninline_attr */
        fn c() {}

        @fragment
        // @some_noninline_attr
        fn d() {}


        fn main() {
        let a = 1; // This is one
        }

    fn main() {
            let a = 1;let b = 2;

            let c = 2;let d = 2;
        }
    fn main() {
            let a = 1;




            let b = 2;
        }
    fn main() {
            let a = 1;

            let b = 2;
        }
    fn main() {

        let a = 1;let b = 2;
        let c = 1;let d = 2;


        }
    fn main() {

            let a = 1;
            let b = 2;
        }
    fn main() {
            let a = 1;
            let b = 2;

        }
    fn main() {

        // Assign 1
        let a = 1;


        }
    fn main() {

        let a = 1;
        // Assign 1


        }
    fn main() {

        // Assign 1

        let a = 1;


        }
    fn main() {


        let a = 1;

        // Assign 1

        }
    fn main() {

        let a = 1; /* Assign 1 */


        }
    fn main() {

        /* Assign 1 */
        let a = 1;


        }
    fn main() {

        let a = 1;
        /* Assign 1 */


        }
    fn main() {

        /* Assign 1 */

        let a = 1;


        }
    fn main() {


        let a = 1;

        /* Assign 1 */

        }
    fn main() {
            loop {
            loop {loop {
            a += 1;b += 1;

            }}}
        }

        fn main() {
        let a = 1; let b = 2;
        }


        fn main() {
        let a = 1; let b = 2; // Comment
        }

    fn main() {
        (a) = (b + c)
        ;
        }
    fn directional_light(light: DirectionalLight, roughness: f32, NdotV: f32, normal: vec3<f32>, view: vec3<f32>, R: vec3<f32>, F0: vec3<f32>, diffuseColor: vec3<f32>) -> vec3<f32> {}
    fn specular(f0: vec3<f32>, roughness: f32, h: vec3<f32>, NoV: f32, NoL: f32,
              NoH: f32, LoH: f32, specularIntensity: f32) -> vec3<f32> {}
    fn main() {
        ;
        }
    fn main() {
        let a =
        foo
        (

        )
        ;
        }
    fn main() {
        let a =
        foo
        (
        3

        )
        ;
        }
    fn main() {
        let a =
        foo
        (
        3
        ,
        2
        )
        ;
        }
    fn main() {
        let a = foo(1,2,3,4,5,6,7,8,9,10,1,2,3,4,5,6,7,8,9,10,1,2,3,4,5,6,7,8,9)
        ;
        }
    fn main() { let a = bitcast   <  vec4<u32>  >  ( x+5 ); }
    fn main() {
        for(;;) {
        }


        }
    fn main() {
        for(var i = 0; // A
        i < 4; i++) {
        }


        }

        fn a() {}
        fn b() {}

        fn c() {}fn d() {}


        fn e() {}


        struct A {
          a: i32
        }
        struct B {
          b: i32
        }

        struct C {
          c: i32
        }




        struct D {
          d: i32
        }


        var<workgroup> a: u32;
        var<workgroup> b: u32;

        var<workgroup> c: u32;


        var<workgroup> d: u32;


        const a: u32 = 0u;
        const b: u32 = 0u;

        const c: u32 = 0u;


        const d: u32 = 0u;


        override a: u32 = 0u;
        override b: u32 = 0u;

        override c: u32 = 0u;


        override d: u32 = 0u;


        alias A = array<i32, 5>;
        alias B = array<i32, 5>;

        alias C = array<i32, 5>;


        alias D = array<i32, 5>;


        alias A = array<i32, 5>;
        alias B = array<i32, 5>;
        fn c() {}
        fn d() {}
        struct E {
            f: u32,
        }
        fn f() {}


        fn a() {
            callOtherFn();
        }
        fn b() {
            callOtherFn();
        }

        fn c() {
            callOtherFn();
        }


        fn d() {
            callOtherFn();
        }


        struct A {
            a: i32,
        }
        struct B {
            b: i32,
        }

        struct C {
            c: i32,
        }


        struct D {
            d: i32,
        }


        // Alone

        // Line Before
        fn a() { let a = 1; } // Should Stay on the Same line
        // Line After

        // Alone


        /* Alone */

        /* Line Before */
        fn a() { let a = 1; } /* Should stay on the same line */
        /* Line After */

        /* Alone */


        /* Alone */

        /* Line Before */
        struct A { item: u32 } /* Should be broken into new line */
        /* Line After */

        /* Alone */


        // Alone

        // Line Before
        fn a() {} // Can be kept on the same line
        // Line After

        // Alone


        // Alone

        // Line Before
        const a: i32 = 1; // Should be kept on the same line
        // Line After

        // Alone


        override a = 1; // This is one

    fn main() {
var x=0;
}
    fn main() {
for(let i = 0; i < 10; i++) {
if i % 2 == 0 {
var x = 0;
}
}
}
    fn main() {
let x = 3;

let y = 4;
}

        @must_use /* bla */ @fragment fn a() {}

        @must_use /* bla */
        @fragment fn a() {}

        @must_use
        /* bla */
        @fragment fn a() {}

        @must_use
        /* bla */ @fragment fn a() {}


        @must_use /* bla */ @must_use fn a() {}

        @must_use /* bla */
        @must_use fn a() {}

        @must_use
        /* bla */
        @must_use fn a() {}

        @must_use
        /* bla */ @must_use fn a() {}


        @fragment /* bla */ @must_use fn a() {}

        @fragment /* bla */
        @must_use fn a() {}

        @fragment
        /* bla */
        @must_use fn a() {}

        @fragment
        /* bla */ @must_use fn a() {}


        @compute /* bla */ @if(true) fn a() {}

        @compute /* bla */
        @if(true) fn a() {}

        @compute
        /* bla */
        @if(true) fn a() {}

        @compute
        /* bla */ @if(true) fn a() {}


        @if(true) /* bla */ @compute fn a() {}

        @if(true) /* bla */
        @compute fn a() {}

        @if(true)
        /* bla */
        @compute fn a() {}

        @if(true)
        /* bla */ @compute fn a() {}

    fn main() {
        let a = other_thing;
        }
    fn main() {
        let a = ΔέλταréflexionКызыл𐰓𐰏𐰇朝焼けسلام검정שָׁלוֹםगुलाփիրուզ𓃞𓃢𓆣;
        }
    fn main() {
        let a = 0;
        let a = 123;
        let a = 123u;
        let a = 123i;
        let a = 0x1234;
        let a = 0xcaFE;
        let a = OXcaFE;
        }
    fn main() {
        let a = my_module::MY_CONSTANT;
        }

    fn main() {
        let a =
        foo
        .
        bar
        ;
        }
    fn main() {
        let a =
        foo
        .
        bar
        .
        baz
        .
        blub
        .
        blob
        ;
        }
    fn main() {
        let a =
        foo[17]
        .
        bar[28]
        .
        baz[39]
        ;
        }
    fn main() {
        let a =
        get_my_foo()
        .
        bar
        ;
        }
    fn main() {
        let a = -

        1;
        }
    fn main() {
        let a = &

        1;
        }
    fn main() {
        let a = *

        1;
        }
    fn main() {
        let a = ~

        1;
        }
    fn main() {
        let a = ~

        /* A */

        1;
        let b = ~1;
        }
    fn main() {
        let a = 111111111111111111111 + 2222222222222222222222222 + 3333333333333333333333333333 + 4444444444444444444444 + 555555555555555555555 + 666666666666666666666666 + 777777777777777777777 + 88888888888888888888 + 999999999999999999999;
        }
    fn main() {
        const a_multiline_binding = 1 // The thing
                + 1 // The other thing
                + 7 // The other thing
                // Separate
                ;

        }
    fn main() {
        let a = - aaaa;
        }
    fn main() {
        let a = 1+(1 + // Hi
        2);
        }
    fn main() {
        let a = a[1 + // Hi
        2];
        }

        @
        fragment
        fn main() {}


        @
        attr
        (
        0
        )
        fn main() {}


        @
        attr
        (
        0,
        0,
        0
        )
        fn main() {}


        @
        thingy
        (
        magic,
        thing
        )
        fn main() {}


       @bla
       @
       diagnostic
       (off, something)
       @blu
       fn main() {}


        @foo(12, // Force break
        foo, 1 + vubble)
        override a: usize = 0;

    fn main() {
        let a = 1+1;
        }
    fn main() {
        let a = 1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1;
        }
    fn main() {
    let a=x+y*z;
}
    fn main() { let x = 1u >> 3u; }
    fn main() { let x = 1u << 3u; }

        fn main() {

        let a = //a
        /* A */
        // b
        1;


        for(/* A */ var i = 0; // Force Multiline
        ; a++) {}
        }


fn main() {
    let z = mat2x4<f32>(a,b,c,d,g,h,i,j);
}

fn main() {
    let x = mat3x3(
        cosR,  0.0, sinR,
        0.0, 1.0, 0.0,
        -sinR, 0.0, cosR,
    );
}

fn main() {
    let x = mat3x3(
        cosR,  0.0, sinR,
        0.0, 1.0, 0.0,
        -sinR, 0.0,
    );
}

fn main() {
    let x = mat3x3(
        cosR,
        1.0, // Breaky
        1.0,
        sinR,
        0.0, 1.0, 0.0,
        -sinR, 0.0,
    );
}

fn main() {
    let x = mat3x3(
        cosR, do_thing(
        1.0 // Breaky
        ), 1.0,
        sinR,
        0.0, 1.0, 0.0,
        -sinR, 0.0,
    );
}

fn main() {
    let x = mat2x4<f32>(
    vec2(0.0, 0.0),
    vec2(0.0, 0.0),
    vec2(0.0, 0.0),
    vec2(0.0, 0.0),
    );
}

fn main() {
    let x = mat2x4<f32>(
    0.0, 0.0,
    1.0, 1.0,
    0.1
    );
}

        const RGB2YIQ: mat3x3<f32> = mat3x3<f32>(
                vec3f(0.300, 0.5900, 0.1100),
                vec3f(0.599,-0.2773,-0.3217,),
                vec3f(0.213, -0.5251, 0.3121),
            );

    fn main() {
        const
        a
        =
        1
        ;
        }
    fn main() {
        const
        a
        =
        1
        +
        1
        +
        1
        ;
        }
    fn main() {

        const a = 1; // Assign 1


        }
    fn main() {

        const a
        :
        u32
        =
        1;


        }
    fn main() {

        const a
        :
        array
        <
        u32
        ,
        28
        >
        = 1;


        }

        const
        a
        =
        1
        ;


        const
        a: array<u32, 28>
        =
        make_array()
        ;



    struct Foo {a: i32}
    struct

        Foo

        {

        a

        :

        i32

        ,

        b

        :

        i32

        }


        struct  Test
        {  @location(0) @attr(1) x: i32,                    a: i32,
        b: f32,

                }
    struct Foo {
            // This comment describes A
            a: i32,
            // This comment describes B
            b:i32
        }

        struct A {
        // This comment should stick to the member
        a: i32
        }


        struct A { // This comment should stick to the member
        a: i32
        }


        struct VertexInput {
            @builtin(vertex_index) vertexIndex: u32,
        };

    fn main() {
            switch(a) {
            }
        }
    fn main() {
            switch(a) {
                case 1: {
                    let a = 1;
                }
            }
        }
    fn main() {
            switch(a) {
                case 1 {
                    let a = 1;
                }
                default {
                    let a = 1;
                }
                case 2 {
                    let a = 1;
                }
            }
        }
    fn main() {
            switch(a) {
                case default {
                    let a = 1;
                }
            }
        }
    fn main() {
            switch(a) {
                case 1, 2, {
                    let a = 1;
                }
            }
        }
    fn main() {
            switch(a) {
                case 1 {
                    let a = 1;
                }
                case 2 {
                    let a = 1;
                    let a = 1;
                }
            }
        }
    fn        main         (           a    :    b )  -> f32   {}
    fn  main ( a :  b,  c : d )  -> f32   {}
    fn  main ( a :  b )    {}
    fn main(a: b , c: d ,)  -> f32   {}
    fn main(a:b)->f32{}
    fn main(
    a:b
)->f32{}
    fn main(
    a:b, c:d)->f32{}
    fn main(
    a:b,
    c:d
)->f32{}

 fn  main( a:  b )  -> f32   {}
  fn  main( a:  b )  -> f32   {}

    fn main() {
        let a =
        foo
        [
        0
        ]
        ;
        }
    fn main() {
        let a =
        foo[17]
        [28]
        [39]
        ;
        }
    fn main() {
        let a =
        foo
        [
        0 //Comment about the index
        ]
        ;
        }

        fn debug_draw_line(b: array<f32, 5>, index: u32) {
            b[index]   = 0;
        }


        fn debug_draw_line(b: array<f32, 5>, index: u32) {
            b[2+2]   = 0;
        }

    fn main() {
        a
        +=
        1
        ;
        }
    fn main() {
    y  +=  x + y;
}
    fn main() {
        a
        += 1;
        a
        -= 1;
        a
        *= 1;
        a
        /= 1;
        a
        %= 1;
        a
        &= 1;
        a
        |= 1;
        a
        ^= 1;
        a
        <<= 1;
        a
        >>= 1;
        }
    fn main() {
        a[1] <<= 728;
        a[1] += 728;
        a[1] /= 728;
        }
    fn main() {
        a.b <<= 728;
        a.b += 728;
        a.b /= 728;
        a.b /= a.b;
        }

        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn main() {
        aaaaaaaaaaaaaaaaaaaaa
        .bbbbbbbbbbbbbbbbbbb
        .cccccccccccccccccc
        .ddddddddddddddddddd <<= foooooooooooooooo(
        baaaaaaaaaaaaaaar(
        111111111111111111111
        +
        8228282828282882828282828828282
        )
        );
        }

        fn a() {let a = 1;} // A

        fn b() {
            let b = 1;
        } // B

        fn c() {let c = 2;}
        // C


    fn main() {
        while(x < 1){}
        while  (  x < 1   )  {}
    }
    fn main() {
        while x < (1 + 1) {}
    }
    fn main() {
        while(true) {
        let a = 3;
        }


        }
    fn main() {
        while(true) {
        let a = 3;
        continue;
        let b = 3;
        }


        }
    fn main() {
return (1);


        }
    fn main() {
        a
        ++
        ;
        b
        --
        ;
        }
    fn main() {
        a[0]
        ++
        ;
        b[247]
        --
        ;
        }
    fn main() {
        for(var a = 0; a < 17; a++) {}
        for(var a = 0; a < 17; a--) {}
        }
    fn main() {
        _
        =
        a
        ;
        }
    fn main() {
        _
        =
        a +
        foo(
        27
        )
        ;
        }
    fn main() {
        _ = (b + c)
        ;
        }
    fn main() {
//Hello


        }
    fn main() {
/* Hello */


        }
    fn main() {let a = 1;let b = 2;}

        fn thing() {}

    fn main() {
        a
        =
        b
        ;
        }
    fn main() {
        foo.a
        =
        b
        ;
        }
    fn main() {
        foo[0]
        =
        b
        ;
        }
    fn main() {
        for(a
        = 27;
        a < 100
        ; a
        = process(a)){}
        }
    fn main() {
        let
        a
        =
        1
        ;
        }
    fn main() {
        let
        a
        =
        1
        +
        1
        +
        1
        ;
        }
    fn main() {
        let a_multiline_binding = 1 // The thing
                + 1 // The other thing
                + 7 // The other thing
                // Separate
                ;

        }
    fn main() {

        let a = 1; // Assign 1


        }
    fn main() {
        loop {
        }


        }
    fn main() {
        loop {
        }


        }
    fn main() {
        loop {
        let a = 3;
        continue;
        let b = 3;
        }


        }
    fn main() {
discard;


        }

        const_assert
        x
        <
        y
        ;


        fn main() {
            const_assert
            x
            <
            y
            ;
        }

    fn main() {
        if
        false
        {
        }
        }
    fn main() {
    if(x < 1){}
    else if (x > 2){
        let a = 3;
    }else     if(  x > 2 ){}
}
    fn main() {
        if
        false
        {
        let a = 1;
        }
        }
    fn main() {
        if
        false
        {
        let a = 1;
        }
        else
        {
        let b = 1;
        }
        }
    fn main() {
        if
        false
        {
        }
        else
        if
        true
        {
        }
        else
        {
        }
        }
    fn main() {
        if
        false
        {
        let a = 1;
        }
        else
        if
        true
        {
        let b = 1;
        }
        else
        {
        let c = 1;
        }
        }
    fn main() {
        if
        a
        {
        }
        else
        if
        b
        {
        }
        else
        if
        c
        {
        }
        else
        {
        }
        }
    fn main() {
        if
        a
        {
        let a = 1;
        }
        else
        if
        b
        {
        let b = 1;
        }
        else
        if
        c
        {
        let c = 1;
        }
        else
        {
        let e = 1;
        }
        }
    fn main() {
        for(;i<4;i++) {
        }


        }
    fn main() {
        for(var a = 0;;i++) {
        }


        }
    fn main() {
        for(var a = 0;a<4;) {
        }


        }
    fn main() {
        for(let a = 1+1+1+1+alculate_something_really_long(172832782);compute_some_value(a % 12847248 * 1827348 + 182748) < A_LONG_CONSTANT;a = increment_but_fancy(a)) {
        }


        }
    fn main() {
        for(let a = 1+1+1+1+1+1+calculate_something_really_long(172832782, 1827387428, 3487348342);compute_some_value_that_has_a_long_name_from(a % 12847248 * 1827348 + 182748) < AN_INCONVENIENTLY_LONG_CONSTANT_DECLARED_SOMEWHERE_ELSE;a = increment_but_in_a_very_fancy_manner(a)) {
        }


        }
    fn main() {
        for(var i = 0; i < 4; i++) {
        }


        }
    fn main() {
    for( var i = 0;i < 100;   i = i + 1  ){}
}
    fn main() {
        var
        a
        =
        1
        ;
        }
    fn main() {
        var
        a
        =
        1
        +
        1
        +
        1
        ;
        }
    fn main() {var x   : u32=0;}
    fn main() {
        var a_multiline_binding = 1 // The thing
                + 1 // The other thing
                + 7 // The other thing
                // Separate
                ;

        }
    fn main() {

        var a = 1; // Assign 1


        }

        var
        a
        =
        1
        ;


        var
        a: array<u32, 28>
        =
        make_array()
        ;


        var<workgroup>
        a: array<u32, 28>
        ;


        var<storage,read_write>
        a: array<u32, 28>
        ;


fn main() {
    let x = mat2x2<f32>(a,b,c,d);
    let y = mat2x3<f32>(a,b,c,d,e,f);
    let z = mat2x4<f32>(a,b,c,d,g,h,i,j);

    let h = mat2x2f(a,b,c,d);
    let f = mat2x2h(a,b,c,d);
}

fn main() {
    let x = mat3x2<f32>(a,b,c,d,e,f);
    let y = mat3x3<f32>(a,b,c,d,e,f,g,h,i);
    let z = mat3x4<f32>(a,b,c,d,g,h,i,j,i,j,k,l);

    let h = mat3x2h(a,b,c,d,e,f);
    let f = mat3x2f(a,b,c,d,e,f);
}

fn main() {
    let x = mat4x2<f32>(a,b,c,d,e,f,g,h);
    let y = mat4x3<f32>(a,b,c,d,e,f,g,h,i,j,k,l);
    let z = mat4x4<f32>(a,b,c,d,g,h,i,j,i,j,k,l,m,n,o,p);

    let h = mat4x2h(a,b,c,d,e,f,g,h);
    let f = mat4x2f(a,b,c,d,e,f,g,h);
}

        override
        a
        =
        1
        ;


        override
        a
        =
        1
        +
        1
        +
        1
        ;


        override a_multiline_binding = 1 // The thing
                + 1 // The other thing
                + 7 // The other thing
                // Separate
                ;




        override a = 1; // Assign 1





        override a
        :
        u32
        =
        1;





        override a
        :
        array
        <
        u32
        ,
        28
        >
        = 1;



    fn main() {
        while(true) {

        continue;
        }


        }
    fn main() {
        foo();
        }
    fn main() {
        foo();
        bar();
        }
    fn main() {
        bla(12, bar(), 1 + vubble);
        }
    fn main() {
    min  (  x,y );
}
    fn main() {
return;


        }
    fn main() {
return 1;


        }
    fn main() {
return 1 + 2 + (47 * get_the_number() - a ) >> 18 - 2;


        }
    fn main() {
            switch(a) {
                case 1 {
                    let a = 1;
                }
            }
        }
    fn main() {
            switch(a) {
                case 1, default, 2 {
                    let a = 1;
                }
            }
        }
    fn main() {
            switch(a) {
                case default {
                    let a = 1;
                }
            }
        }
    fn main() {
            switch(a) {
                /* ABC */ case /* DEF */ default {}
            }
            switch(b) {
                /* ABC */
                case /* DEF */ default {}
            }
            switch(c) {
                /* ABC */ case
                /* DEF */ default {}
            }
            switch(d) {
                /* ABC */ case /* DEF */
                default {}
            }
        }
    fn main() {
            switch(a) {
                case 1, 2, {
                    let a = 1;
                }
            }
        }
    fn main() {
            switch(a) {
                case 1,: {
                    let a = 1;
                }
            }
        }
    fn main() {
            switch(a) {
                case 1, c: {
                    let a = 1;
                }
            }
        }

        fn main() {
            /* Before */
            switch a {} /* Can stay on the same line */
            /* After */
        }


        fn main() {
            /* Before */
            switch a {default {}} /* Should be broken onto separate the same line */
            /* After */
        }

