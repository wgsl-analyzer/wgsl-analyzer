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

struct StructAttributes {
    @location(0) field: vec3f,
    @location(1) field2: vec3<f32>,
    @builtin(thing) field3: array<i32, 10>,
}
