use expect_test::expect;

use crate::test_util::check_sweep;

#[expect(clippy::too_many_lines, reason = "It is a long string too.")]
#[test]
fn sweep_function_declaration_with_complex_types_and_annotations() {
    check_sweep(
        "
        fn do_the_thing(@location(1) a: u32, @builtin(subgroup_invocation_id) b: u32, c: ptr<workgroup, array<array<u32, 27>, 1>, read_write>) -> @location(12282) array<vec4<f32>, 17> {
        let a = 1;
        }
        ",
        expect![[r#"
            // max_length: 184----------------------------------------------------------------------------------------------------------------------------------------------------------------------
            fn do_the_thing(@location(1) a: u32, @builtin(subgroup_invocation_id) b: u32, c: ptr<workgroup, array<array<u32, 27>, 1>, read_write>) -> @location(12282) array<vec4<f32>, 17> {
            	let a = 1;
            }

            // max_length: 176--------------------------------------------------------------------------------------------------------------------------------------------------------------
            fn do_the_thing(
            	@location(1) a: u32,
            	@builtin(subgroup_invocation_id) b: u32,
            	c: ptr<workgroup, array<array<u32, 27>, 1>, read_write>,
            ) -> @location(12282) array<vec4<f32>, 17> {
            	let a = 1;
            }

            // max_length: 59------------------------------------------
            fn do_the_thing(
            	@location(1) a: u32,
            	@builtin(subgroup_invocation_id) b: u32,
            	c: ptr<
            		workgroup,
            		array<array<u32, 27>, 1>,
            		read_write,
            	>,
            ) -> @location(12282) array<vec4<f32>, 17> {
            	let a = 1;
            }

            // max_length: 43--------------------------
            fn do_the_thing(
            	@location(1) a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            	c: ptr<
            		workgroup,
            		array<array<u32, 27>, 1>,
            		read_write,
            	>,
            ) -> @location(12282)
            array<vec4<f32>, 17> {
            	let a = 1;
            }

            // max_length: 32---------------
            fn do_the_thing(
            	@location(1) a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            	c: ptr<
            		workgroup,
            		array<
            			array<u32, 27>,
            			1,
            		>,
            		read_write,
            	>,
            ) -> @location(12282)
            array<vec4<f32>, 17> {
            	let a = 1;
            }

            // max_length: 28-----------
            fn do_the_thing(
            	@location(1) a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            	c: ptr<
            		workgroup,
            		array<
            			array<
            				u32,
            				27,
            			>,
            			1,
            		>,
            		read_write,
            	>,
            ) -> @location(12282)
            array<vec4<f32>, 17> {
            	let a = 1;
            }

            // max_length: 23------
            fn do_the_thing(
            	@location(1)
            	a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            	c: ptr<
            		workgroup,
            		array<
            			array<
            				u32,
            				27,
            			>,
            			1,
            		>,
            		read_write,
            	>,
            ) -> @location(12282)
            array<vec4<f32>, 17> {
            	let a = 1;
            }

            // max_length: 21----
            fn do_the_thing(
            	@location(1)
            	a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            	c: ptr<
            		workgroup,
            		array<
            			array<
            				u32,
            				27,
            			>,
            			1,
            		>,
            		read_write,
            	>,
            ) -> @location(12282)
            array<
            	vec4<f32>,
            	17,
            > {
            	let a = 1;
            }

            // max_length: 20---
            fn do_the_thing(
            	@location(1)
            	a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            	c: ptr<
            		workgroup,
            		array<
            			array<
            				u32,
            				27,
            			>,
            			1,
            		>,
            		read_write,
            	>,
            ) -> @location(
            	12282,
            )
            array<
            	vec4<f32>,
            	17,
            > {
            	let a = 1;
            }

            // max_length: 15
            fn do_the_thing(
            	@location(
            		1,
            	) a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            	c: ptr<
            		workgroup,
            		array<
            			array<
            				u32,
            				27,
            			>,
            			1,
            		>,
            		read_write,
            	>,
            ) -> @location(
            	12282,
            )
            array<
            	vec4<
            		f32,
            	>,
            	17,
            > {
            	let a = 1;
            }

            // max_length: 13
            fn do_the_thing(
            	@location(
            		1,
            	) a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            	c: ptr<
            		workgroup,
            		array<
            			array<
            				u32,
            				27,
            			>,
            			1,
            		>,
            		read_write,
            	>,
            ) -> @location(
            	12282,
            )
            array<
            	vec4<
            		f32,
            	>,
            	17,
            > {
            	let a =
            		1;
            }

            // max_length: 12
            fn do_the_thing(
            	@location(
            		1,
            	)
            	a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            	c: ptr<
            		workgroup,
            		array<
            			array<
            				u32,
            				27,
            			>,
            			1,
            		>,
            		read_write,
            	>,
            ) -> @location(
            	12282,
            )
            array<
            	vec4<
            		f32,
            	>,
            	17,
            > {
            	let a =
            		1;
            }

        "#]],
    );
}

#[expect(clippy::too_many_lines, reason = "It is a long string too.")]
#[test]
fn sweep_function_declaration_with_simple_types_and_annotations() {
    check_sweep(
        "
        fn do_the_thing(@location(1) a: u32, @builtin(subgroup_invocation_id) b: u32) -> @location(12282) vec4<f32>{
        let a = 1;
        }
        ",
        expect![[r#"
            // max_length: 115-------------------------------------------------------------------------------------------------
            fn do_the_thing(@location(1) a: u32, @builtin(subgroup_invocation_id) b: u32) -> @location(12282) vec4<f32> {
            	let a = 1;
            }

            // max_length: 108------------------------------------------------------------------------------------------
            fn do_the_thing(
            	@location(1) a: u32,
            	@builtin(subgroup_invocation_id) b: u32,
            ) -> @location(12282) vec4<f32> {
            	let a = 1;
            }

            // max_length: 43--------------------------
            fn do_the_thing(
            	@location(1) a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            ) -> @location(12282) vec4<f32> {
            	let a = 1;
            }

            // max_length: 32---------------
            fn do_the_thing(
            	@location(1) a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            ) -> @location(12282)
            vec4<f32> {
            	let a = 1;
            }

            // max_length: 23------
            fn do_the_thing(
            	@location(1)
            	a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            ) -> @location(12282)
            vec4<f32> {
            	let a = 1;
            }

            // max_length: 20---
            fn do_the_thing(
            	@location(1)
            	a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            ) -> @location(
            	12282,
            ) vec4<f32> {
            	let a = 1;
            }

            // max_length: 15
            fn do_the_thing(
            	@location(
            		1,
            	) a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            ) -> @location(
            	12282,
            ) vec4<f32> {
            	let a = 1;
            }

            // max_length: 13
            fn do_the_thing(
            	@location(
            		1,
            	) a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            ) -> @location(
            	12282,
            ) vec4<f32> {
            	let a =
            		1;
            }

            // max_length: 12
            fn do_the_thing(
            	@location(
            		1,
            	)
            	a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            ) -> @location(
            	12282,
            )
            vec4<f32> {
            	let a =
            		1;
            }

            // max_length: 10
            fn do_the_thing(
            	@location(
            		1,
            	)
            	a: u32,
            	@builtin(subgroup_invocation_id)
            	b: u32,
            ) -> @location(
            	12282,
            )
            vec4<
            	f32,
            > {
            	let a =
            		1;
            }

        "#]],
    );
}

#[test]
fn sweep_super_simple_function_declaration_with_simple_types() {
    check_sweep(
        "
        fn do_the_thing() -> vec4<f32> {
        let a = 1;
        }
        ",
        expect![[r#"
            // max_length: 39----------------------
            fn do_the_thing() -> vec4<f32> {
            	let a = 1;
            }

            // max_length: 31--------------
            fn do_the_thing(
            ) -> vec4<f32> {
            	let a = 1;
            }

            // max_length: 15
            fn do_the_thing(
            ) -> vec4<
            	f32,
            > {
            	let a = 1;
            }

            // max_length: 13
            fn do_the_thing(
            ) -> vec4<
            	f32,
            > {
            	let a =
            		1;
            }

        "#]],
    );
}
