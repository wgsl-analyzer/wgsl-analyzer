    
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
        
    
        alias Test =
        array<i32, // Length
        17>
        ;
        
    
        alias Test =
        array<
        array<i32, // Length
        17>,18>
        ;
        
    import aaaaaaaaa::{bbbbbbbb::{cdddddd::{dddddd::{eeeeeee::{ffffffff::{gggggg::{hhhhhh::{iiiiii::jjjjjjjjjj, kkkkkkkk}}}}}}}};
    import aaaaaaaaa::bbbbbbbb::{cdddddd::dddddd::{eeeeeee::ffffffff::{gggggg::hhhhhh::{iiiiii::jjjjjjjjjj, kkkkkkkk}}}};
    
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

        <i32,
        // Miep
        19>,
        17>
        ;
        
    
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
        fn a() { let a = 1; } // Should be broken into new line
        // Line After

        // Alone
        
    
        /* Alone */

        /* Line Before */
        fn a() { let a = 1; } /* Should be broken into new line */
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
        {  @location(0) @attribute(1) x: i32,                    a: i32,
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
    
        fn a(
        a: texture_2d<f32>,
        bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: f32
        ) {}
        
    fn main() {
        continue;
        }
    
       diagnostic
       (off, something);
       
    
       diagnostic
       (off, something.something);
       
    
       diagnostic
       (off, something
       .
       something);
       
    
       enable
       thing, bla,
       thingy,;
       
    
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
       enable
       thing, blaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,thingyyyyyyyyyyyyyyyyyyyyyyyyy,;
       
    
       requires
       thing, bla,
       thingy,;
       
    
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
       requires
       thing, blaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,thingyyyyyyyyyyyyyyyyyyyyyyyyy,;
       
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

        const a = (1 + 1);


        }
    
        fn main() {

        let a = //a
        /* A */
        // b
        1;


        for(/* A */ var i = 0;
        // Force Multiline
        ; a++) {}
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
                // Seperate
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
            let a = 1 + (((((((((1+1)))))))));
        }
        
    
        fn main() {
            let a = 1 + (2);
        }
        
    fn directional_light(light: DirectionalLight, roughness: f32, NdotV: f32, normal: vec3<f32>, view: vec3<f32>, R: vec3<f32>, F0: vec3<f32>, diffuseColor: vec3<f32>) -> vec3<f32> {}
    fn specular(f0: vec3<f32>, roughness: f32, h: vec3<f32>, NoV: f32, NoL: f32,
              NoH: f32, LoH: f32, specularIntensity: f32) -> vec3<f32> {}
    fn main() {
        while true {
        }


        }
    fn main() {
        while (1+2==3) {}


        }
    fn main() {
        loop {
        continuing {

        }
        }


        }
    fn main() {
        loop {
        continuing{

        let a = 3;
        }
        }


        }
    fn main() {
        loop {
        let a = 3;
        continue;
        let b = 3;
        continuing {
        let c = 3;

        }
        }


        }
    fn main() {
        loop {
        continuing {
        break if (false);

        }
        }


        }
    fn main() {
        while(true) {

        continue;
        }


        }
    fn main() {
        (a) = (b + c)
        ;
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
       
    
        const_assert(x<y);
        const_assert
        (x
        < y
        );
        
    fn main() {

        var a = (1 + 1);


        }
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
        
    fn main() {
        let a = 111111111111111111111 + 2222222222222222222222222 + 3333333333333333333333333333 + 4444444444444444444444 + 555555555555555555555 + 666666666666666666666666 + 777777777777777777777 + 88888888888888888888 + 999999999999999999999;
        }
    fn main() {
        let a = 111111111111111111111 + 2222222222222222222222222 + 333333333333333333333333333333 +  44444444444444 +                    55555555555 +                       66666666666666  + 777777777777777777777 + 88888888888888888888 + 999999999999999999999;
        let b = 111111111111111111111 + 2222222222222222222222222 + 333333333333333333333333333333 + (44444444444444 +                    55555555555 +                       66666666666666) + 777777777777777777777 + 88888888888888888888 + 999999999999999999999;
        let c = 111111111111111111111 + 2222222222222222222222222 + 333333333333333333333333333333 + (444444444444444444444444444444444 + 555555555555555555555555555555555 + 666666666666666666666666666666666666) + 777777777777777777777 + 88888888888888888888 + 999999999999999999999;
        }
    fn main() {
        const a_multiline_binding = 1 // The thing
                + 1 // The other thing
                + 7 // The other thing
                // Separate
                ;

        }
    fn main() {
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        let aaaaaaaaaaaaa = 1 + 1 + long_function(aaaaaaaaaaa, bbbbbbbbbbb, ccccccccccccc, dddddddddddddd);
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
        for(;;) {
        }


        }
    fn main() {
        for(var i = 0; // A
        i < 4; i++) {
        }


        }
    fn        main         (           a    :    b )  -> f32   {}
    fn  main ( a :  b,  c : d )  -> f32   {}
    fn  main ( a :  b )    {}
    fn  this_is_a_very_long_name_who_knows_when_it_will_end_because_it_just_goes_on_and_on_and_on( a :  b,c:d )  -> f32   {}
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
        loop {
        }


        }
    
fn main() {
    let x = mat3x3(
        cosR,  0.0, sinR,
        0.0, 1.0, 0.0,
        -sinR, 0.0, cosR,
    );
}
    

        override a = (1 + 1);


        
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
//Hello


        }
    fn main() {
/* Hello */


        }
    fn main() {let a = 1;let b = 2;}
    
        fn thing() {}
        
    fn main() {
return (1);


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
    
fn a() {
    loop{
        /* AAA
    AAA
        AAA
        */
    }
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
                // Seperate
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
                // Seperate
                ;

        }
    fn main() {

        let a = 1; // Assign 1


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
        loop {
        continuing {
        break if false;

        }
        }


        }
    fn main() {
        loop {
        continuing {
        break if (1 + (1 + 1));

        }
        }


        }
    fn main() {
        loop {
        continuing{

        break if false;
        }
        }


        }
    fn main() {
        loop {
        continuing{

        break if a == 3 && b < 4 && ((a < 4) == (b > 9)) && c < 5 && do_the_thing(d);
        }
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
        foo_efek_felkj_soiu_flejk_lkjef_aoieu_flkejfalk_lkjeifou_flj_lkjsieuf_flkj_Ljklllefjief();
        }
    fn main() {
        bla(12, bar(), 1 + vubble);
        }
    fn main() {
    min  (  x,y );
}
    fn main() {
        let a = 1 + (1+1);
        }
    fn main() {
        let a = 1+(1 + // Hi
        2);
        }
    fn main() {
        let a = 1+ (1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+1)))))))))))))))))))));
        }
    fn main() {
        let a = 1+ ((((((((((((((((((((((((((((1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1)+1);
        }
    fn main() {
        let a = 1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+(1+1))))))))))))))))))))))))))))))))))))))))))))))))));
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
        let a = 1+1;
        }
    fn main() {
        let a = 1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1;
        }
    fn main() {
    let a=x+y*z;
}
    fn main() { let x = 1u >> 3u; }
    fn main() { let x = 1u << 3u; }
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
        for(let a = 1+1+1+1+alculate_something_really_long(172832782);a<3;a+=1) {
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
    
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn thing() -> vec4<f32> @attr(0) @attr(1) @diagnostic(bla, off) {
        }
    
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        fn thing() -> vec4<f32> @aaaaaaaa(3) @bbbbbbb(1) @ccccccccccccc(4,3) @dddddddddd(28) @eeeeeeeeeeee(11,11,11) {
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
        continuing
        @attr(0) @attr(1) {}
        }
        }
        
    
        fn main() {
        @attr(0) @attr(1) while true {}
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
        fn a() {}
        
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
discard;


        }
