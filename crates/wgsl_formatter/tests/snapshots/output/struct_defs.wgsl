struct StructOne {
    field: vec3f
}

struct StructTwo {
    field: vec3f,
    field2: vec3<f32>
}

struct StructThree {
    field: vec3f,
    field2: vec3<f32>,
    field3: array<i32, 10>
}

struct StructFour {
    field: vec3f,
    field2: vec3<f32>,
    field3: array<i32, 10>,
    field4: u32
}

struct StructFive {
    field: vec3f,
    field2: vec3<f32>,
    field3: array<i32, 10>,
    field4: u32,
    field5: u32,
}

struct StructSix {
    field: vec3f,
    field2: vec3<f32>,
    field3: array<i32, 10>,
    field4: u32,
    field5: u32,
    field6: u32,
}

struct StructTrailingComma {
    a: u32,
    b: u32,
}

struct StructOneLine {
    a: u32,
    b: u32
}
struct StructOneLineComma {
    a: u32,
    b: u32,
}

struct StructGarbledOne {
    a: u32,
    b: u32,
}

struct StructGarbledTwo {
    a /*Hey Look a Comment */: u32,
    b: u32,
}


struct ThisIsAStructWithAVeryLongNameThatYouCantEvenImagineHowLongItCanGetWhoaEvenLongerThanThisDoesThisEndOkNowItsGettingRidiculous {
    hey_dont_leave_me_out_i_also_have_such_a_long_name_that_you_cant_even_imagine_how_long_it_can_get_whoa_even_longer_than_this_does_this_end_ok_now_its_getting_ridiculous: u32,
    a: u32,
}

struct StructWithMistakenSemicolon {
    a: u32,
};

struct StructFieldWithManyTypeConstructors {
    this_is_a_long_name_already: array<array<array<array<array<array<array<u32, 10>, 1>, 5>, >, 2>, 3>, 4>,
    5>,
}
