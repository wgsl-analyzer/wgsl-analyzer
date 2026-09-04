use expect_test::expect;

use crate::test_util::{check, check_comments};

#[test]
pub fn format_diagnostic_simple_1() {
    check(
        "
       diagnostic
       (off, something);
       ",
        expect![[r#"
            diagnostic(off, something);
        "#]],
    );
}

#[test]
pub fn format_diagnostic_with_dot_simple_1() {
    check(
        "
       diagnostic
       (off, something.something);
       ",
        expect![[r#"
            diagnostic(off, something.something);
        "#]],
    );
}

#[test]
pub fn format_diagnostic_with_dot_and_newline() {
    check(
        "
       diagnostic
       (off, something
       .
       something);
       ",
        expect![[r#"
            diagnostic(off, something.something);
        "#]],
    );
}

//pub fn format_comments_in_diagnostic_1() { ... } already exists in attribute::comments::format_comments_in_diagnostic_attr_simple_1

#[test]
pub fn format_enable_simple_1() {
    check(
        "
       enable f16,
       clip_distances
       ,dual_source_blending,;
       ",
        expect![[r#"
            enable f16, clip_distances, dual_source_blending;
        "#]],
    );
}

#[test]
pub fn format_enable_long_1() {
    check(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
        enable f16, clip_distances, dual_source_blending, subgroups, primitive_index, subgroup_size_control, wgpu_mesh_shader, wgpu_ray_query, wgpu_ray_query_vertex_return, wgpu_ray_tracing_pipeline, wgpu_int16, wgpu_cooperative_matrix, per_vertex, draw_index, wgpu_binding_array;
       ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            enable
                f16,
                clip_distances,
                dual_source_blending,
                subgroups,
                primitive_index,
                subgroup_size_control,
                wgpu_mesh_shader,
                wgpu_ray_query,
                wgpu_ray_query_vertex_return,
                wgpu_ray_tracing_pipeline,
                wgpu_int16,
                wgpu_cooperative_matrix,
                per_vertex,
                draw_index,
                wgpu_binding_array;
        "#]],
    );
}

#[test]
pub fn format_comments_in_enable_1() {
    check_comments(
        "enable ## subgroups ## , ## primitive_index ## , ## ; ##",
        expect![[r#"
            enable /* 0 */ subgroups /* 1 */ /* 2 */ , primitive_index /* 3 */ /* 4 */; /* 5 */
        "#]],
        expect![[r#"
            enable
                // 0
                subgroups // 1
                // 2
                ,
                primitive_index // 3
                // 4
                ; // 5
        "#]],
    );
}

#[test]
pub fn format_requires_simple_1() {
    check(
        "
        requires
        uniform_buffer_standard_layout
        ,
        subgroup_id,;
       ",
        expect![[r#"
            requires uniform_buffer_standard_layout, subgroup_id;
        "#]],
    );
}

#[test]
pub fn format_requires_long_1() {
    check(
        "
        //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
       requires
       uniform_buffer_standard_layout,subgroup_id,subgroup_uniformity,texture_and_sampler_let,texture_formats_tier1,linear_indexing,immediate_address_space,buffer_view;

       ",
        expect![[r#"
            //Ruler:_|10_____20|_______30|_______40|_______50|_______60|_______70|_______80|
            requires
                uniform_buffer_standard_layout,
                subgroup_id,
                subgroup_uniformity,
                texture_and_sampler_let,
                texture_formats_tier1,
                linear_indexing,
                immediate_address_space,
                buffer_view;
        "#]],
    );
}

#[test]
pub fn format_comments_in_requires_1() {
    check_comments(
        "requires ## linear_indexing ## , ## buffer_view ## , ## ; ##",
        expect![[r#"
            requires /* 0 */ linear_indexing /* 1 */ /* 2 */ , buffer_view /* 3 */ /* 4 */; /* 5 */
        "#]],
        expect![[r#"
            requires
                // 0
                linear_indexing // 1
                // 2
                ,
                buffer_view // 3
                // 4
                ; // 5
        "#]],
    );
}
