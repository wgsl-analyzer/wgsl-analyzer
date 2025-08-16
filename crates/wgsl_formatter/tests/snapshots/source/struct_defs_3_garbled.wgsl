struct StructGarbledOne         { a
:
u32, b: u32, }

struct


StructGarbledTwo         {
a /*Hey Look a Comment */
:
u32, b: u32, }

struct

StructAttributes {
    @location(

    0)
    field: vec3f,
    @location(1)


    field2: vec3<f32>,@builtin(thing) field3: array<i32, 10>,
}
