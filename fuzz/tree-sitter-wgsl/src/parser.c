#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 493
#define LARGE_STATE_COUNT 80
#define SYMBOL_COUNT 456
#define ALIAS_COUNT 0
#define TOKEN_COUNT 343
#define EXTERNAL_TOKEN_COUNT 1
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 9
#define PRODUCTION_ID_COUNT 1

enum {
  sym_ident_pattern_token = 1,
  anon_sym_SEMI = 2,
  anon_sym_true = 3,
  anon_sym_false = 4,
  aux_sym_decimal_int_literal_token1 = 5,
  aux_sym_decimal_int_literal_token2 = 6,
  sym_hex_int_literal = 7,
  aux_sym_decimal_float_literal_token1 = 8,
  aux_sym_decimal_float_literal_token2 = 9,
  aux_sym_decimal_float_literal_token3 = 10,
  aux_sym_decimal_float_literal_token4 = 11,
  aux_sym_decimal_float_literal_token5 = 12,
  aux_sym_hex_float_literal_token1 = 13,
  aux_sym_hex_float_literal_token2 = 14,
  aux_sym_hex_float_literal_token3 = 15,
  anon_sym_AT = 16,
  anon_sym_align = 17,
  anon_sym_LPAREN = 18,
  anon_sym_binding = 19,
  anon_sym_builtin = 20,
  anon_sym_const = 21,
  anon_sym_group = 22,
  anon_sym_id = 23,
  anon_sym_interpolate = 24,
  anon_sym_COMMA = 25,
  anon_sym_invariant = 26,
  anon_sym_location = 27,
  anon_sym_size = 28,
  anon_sym_workgroup_size = 29,
  anon_sym_vertex = 30,
  anon_sym_fragment = 31,
  anon_sym_compute = 32,
  anon_sym_RPAREN = 33,
  anon_sym_array = 34,
  anon_sym_LT = 35,
  anon_sym_GT = 36,
  anon_sym_struct = 37,
  anon_sym_LBRACE = 38,
  anon_sym_RBRACE = 39,
  anon_sym_COLON = 40,
  anon_sym_sampler = 41,
  anon_sym_sampler_comparison = 42,
  anon_sym_texture_1d = 43,
  anon_sym_texture_2d = 44,
  anon_sym_texture_2d_array = 45,
  anon_sym_texture_3d = 46,
  anon_sym_texture_cube = 47,
  anon_sym_texture_cube_array = 48,
  sym_multisampled_texture_type = 49,
  anon_sym_texture_storage_1d = 50,
  anon_sym_texture_storage_2d = 51,
  anon_sym_texture_storage_2d_array = 52,
  anon_sym_texture_storage_3d = 53,
  anon_sym_texture_depth_2d = 54,
  anon_sym_texture_depth_2d_array = 55,
  anon_sym_texture_depth_cube = 56,
  anon_sym_texture_depth_cube_array = 57,
  anon_sym_texture_depth_multisampled_2d = 58,
  anon_sym_alias = 59,
  anon_sym_EQ = 60,
  anon_sym_bool = 61,
  anon_sym_f32 = 62,
  anon_sym_f16 = 63,
  anon_sym_i32 = 64,
  anon_sym_u32 = 65,
  anon_sym_ptr = 66,
  anon_sym_atomic = 67,
  anon_sym_vec2 = 68,
  anon_sym_vec3 = 69,
  anon_sym_vec4 = 70,
  anon_sym_mat2x2 = 71,
  anon_sym_mat2x3 = 72,
  anon_sym_mat2x4 = 73,
  anon_sym_mat3x2 = 74,
  anon_sym_mat3x3 = 75,
  anon_sym_mat3x4 = 76,
  anon_sym_mat4x2 = 77,
  anon_sym_mat4x3 = 78,
  anon_sym_mat4x4 = 79,
  anon_sym_let = 80,
  anon_sym_var = 81,
  anon_sym_override = 82,
  anon_sym_bitcast = 83,
  anon_sym_LBRACK = 84,
  anon_sym_RBRACK = 85,
  anon_sym_DOT = 86,
  anon_sym_DASH = 87,
  anon_sym_BANG = 88,
  anon_sym_TILDE = 89,
  anon_sym_STAR = 90,
  anon_sym_AMP = 91,
  anon_sym_SLASH = 92,
  anon_sym_PERCENT = 93,
  anon_sym_PLUS = 94,
  anon_sym_LT_LT = 95,
  anon_sym_GT_GT = 96,
  anon_sym_LT_EQ = 97,
  anon_sym_GT_EQ = 98,
  anon_sym_EQ_EQ = 99,
  anon_sym_BANG_EQ = 100,
  anon_sym_AMP_AMP = 101,
  anon_sym_PIPE_PIPE = 102,
  anon_sym_PIPE = 103,
  anon_sym_CARET = 104,
  anon_sym__ = 105,
  anon_sym_PLUS_EQ = 106,
  anon_sym_DASH_EQ = 107,
  anon_sym_STAR_EQ = 108,
  anon_sym_SLASH_EQ = 109,
  anon_sym_PERCENT_EQ = 110,
  anon_sym_AMP_EQ = 111,
  anon_sym_PIPE_EQ = 112,
  anon_sym_CARET_EQ = 113,
  anon_sym_GT_GT_EQ = 114,
  anon_sym_LT_LT_EQ = 115,
  anon_sym_PLUS_PLUS = 116,
  anon_sym_DASH_DASH = 117,
  anon_sym_if = 118,
  anon_sym_else = 119,
  anon_sym_switch = 120,
  anon_sym_case = 121,
  anon_sym_default = 122,
  anon_sym_loop = 123,
  anon_sym_for = 124,
  anon_sym_while = 125,
  anon_sym_break = 126,
  sym_continue_statement = 127,
  anon_sym_continuing = 128,
  anon_sym_return = 129,
  anon_sym_const_assert = 130,
  anon_sym_discard = 131,
  anon_sym_fn = 132,
  anon_sym_DASH_GT = 133,
  anon_sym_enable = 134,
  anon_sym_perspective = 135,
  anon_sym_linear = 136,
  anon_sym_flat = 137,
  anon_sym_center = 138,
  anon_sym_centroid = 139,
  anon_sym_sample = 140,
  anon_sym_vertex_index = 141,
  anon_sym_instance_index = 142,
  anon_sym_position = 143,
  anon_sym_front_facing = 144,
  anon_sym_frag_depth = 145,
  anon_sym_local_invocation_id = 146,
  anon_sym_local_invocation_index = 147,
  anon_sym_global_invocation_id = 148,
  anon_sym_workgroup_id = 149,
  anon_sym_num_workgroups = 150,
  anon_sym_sample_index = 151,
  anon_sym_sample_mask = 152,
  anon_sym_read = 153,
  anon_sym_write = 154,
  anon_sym_read_write = 155,
  anon_sym_function = 156,
  anon_sym_private = 157,
  anon_sym_workgroup = 158,
  anon_sym_uniform = 159,
  anon_sym_storage = 160,
  anon_sym_rgba8unorm = 161,
  anon_sym_rgba8snorm = 162,
  anon_sym_rgba8uint = 163,
  anon_sym_rgba8sint = 164,
  anon_sym_rgba16uint = 165,
  anon_sym_rgba16sint = 166,
  anon_sym_rgba16float = 167,
  anon_sym_r32uint = 168,
  anon_sym_r32sint = 169,
  anon_sym_r32float = 170,
  anon_sym_rg32uint = 171,
  anon_sym_rg32sint = 172,
  anon_sym_rg32float = 173,
  anon_sym_rgba32uint = 174,
  anon_sym_rgba32sint = 175,
  anon_sym_rgba32float = 176,
  anon_sym_bgra8unorm = 177,
  anon_sym_SLASH_LBRACKrgba_RBRACK_SLASH = 178,
  anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH = 179,
  anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH = 180,
  anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH = 181,
  anon_sym_SLASH_LBRACKxyzw_RBRACK_SLASH = 182,
  anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH = 183,
  anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH = 184,
  anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH = 185,
  anon_sym_CompileShader = 186,
  anon_sym_ComputeShader = 187,
  anon_sym_DomainShader = 188,
  anon_sym_GeometryShader = 189,
  anon_sym_Hullshader = 190,
  anon_sym_NULL = 191,
  anon_sym_Self = 192,
  anon_sym_abstract = 193,
  anon_sym_active = 194,
  anon_sym_alignas = 195,
  anon_sym_alignof = 196,
  anon_sym_as = 197,
  anon_sym_asm = 198,
  anon_sym_asm_fragment = 199,
  anon_sym_async = 200,
  anon_sym_attribute = 201,
  anon_sym_auto = 202,
  anon_sym_await = 203,
  anon_sym_become = 204,
  anon_sym_binding_array = 205,
  anon_sym_cast = 206,
  anon_sym_catch = 207,
  anon_sym_class = 208,
  anon_sym_co_await = 209,
  anon_sym_co_return = 210,
  anon_sym_co_yield = 211,
  anon_sym_coherent = 212,
  anon_sym_column_major = 213,
  anon_sym_common = 214,
  anon_sym_compile = 215,
  anon_sym_compile_fragment = 216,
  anon_sym_concept = 217,
  anon_sym_const_cast = 218,
  anon_sym_consteval = 219,
  anon_sym_constexpr = 220,
  anon_sym_constinit = 221,
  anon_sym_crate = 222,
  anon_sym_debugger = 223,
  anon_sym_decltype = 224,
  anon_sym_delete = 225,
  anon_sym_demote = 226,
  anon_sym_demote_to_helper = 227,
  anon_sym_do = 228,
  anon_sym_dynamic_cast = 229,
  anon_sym_enum = 230,
  anon_sym_explicit = 231,
  anon_sym_export = 232,
  anon_sym_extends = 233,
  anon_sym_extern = 234,
  anon_sym_external = 235,
  anon_sym_fallthrough = 236,
  anon_sym_filter = 237,
  anon_sym_final = 238,
  anon_sym_finally = 239,
  anon_sym_friend = 240,
  anon_sym_from = 241,
  anon_sym_fxgroup = 242,
  anon_sym_get = 243,
  anon_sym_goto = 244,
  anon_sym_groupshared = 245,
  anon_sym_handle = 246,
  anon_sym_highp = 247,
  anon_sym_impl = 248,
  anon_sym_implements = 249,
  anon_sym_import = 250,
  anon_sym_inline = 251,
  anon_sym_inout = 252,
  anon_sym_instanceof = 253,
  anon_sym_interface = 254,
  anon_sym_layout = 255,
  anon_sym_lowp = 256,
  anon_sym_macro = 257,
  anon_sym_macro_rules = 258,
  anon_sym_match = 259,
  anon_sym_mediump = 260,
  anon_sym_meta = 261,
  anon_sym_mod = 262,
  anon_sym_module = 263,
  anon_sym_move = 264,
  anon_sym_mut = 265,
  anon_sym_mutable = 266,
  anon_sym_namespace = 267,
  anon_sym_new = 268,
  anon_sym_nil = 269,
  anon_sym_noexcept = 270,
  anon_sym_noinline = 271,
  anon_sym_nointerpolation = 272,
  anon_sym_noperspective = 273,
  anon_sym_null = 274,
  anon_sym_nullptr = 275,
  anon_sym_of = 276,
  anon_sym_operator = 277,
  anon_sym_package = 278,
  anon_sym_packoffset = 279,
  anon_sym_partition = 280,
  anon_sym_pass = 281,
  anon_sym_patch = 282,
  anon_sym_pixelfragment = 283,
  anon_sym_precise = 284,
  anon_sym_precision = 285,
  anon_sym_premerge = 286,
  anon_sym_priv = 287,
  anon_sym_protected = 288,
  anon_sym_pub = 289,
  anon_sym_public = 290,
  anon_sym_readonly = 291,
  anon_sym_ref = 292,
  anon_sym_regardless = 293,
  anon_sym_register = 294,
  anon_sym_reinterpret_cast = 295,
  anon_sym_requires = 296,
  anon_sym_resource = 297,
  anon_sym_restrict = 298,
  anon_sym_self = 299,
  anon_sym_set = 300,
  anon_sym_shared = 301,
  anon_sym_signed = 302,
  anon_sym_sizeof = 303,
  anon_sym_smooth = 304,
  anon_sym_snorm = 305,
  anon_sym_static = 306,
  anon_sym_static_assert = 307,
  anon_sym_static_cast = 308,
  anon_sym_std = 309,
  anon_sym_subroutine = 310,
  anon_sym_super = 311,
  anon_sym_target = 312,
  anon_sym_template = 313,
  anon_sym_this = 314,
  anon_sym_thread_local = 315,
  anon_sym_throw = 316,
  anon_sym_trait = 317,
  anon_sym_try = 318,
  anon_sym_type = 319,
  anon_sym_typedef = 320,
  anon_sym_typeid = 321,
  anon_sym_typename = 322,
  anon_sym_typeof = 323,
  anon_sym_union = 324,
  anon_sym_unless = 325,
  anon_sym_unorm = 326,
  anon_sym_unsafe = 327,
  anon_sym_unsized = 328,
  anon_sym_use = 329,
  anon_sym_using = 330,
  anon_sym_varying = 331,
  anon_sym_virtual = 332,
  anon_sym_volatile = 333,
  anon_sym_wgsl = 334,
  anon_sym_where = 335,
  anon_sym_with = 336,
  anon_sym_writeonly = 337,
  anon_sym_yield = 338,
  anon_sym_SLASH_SLASH = 339,
  aux_sym__comment_token1 = 340,
  sym__blankspace = 341,
  sym__block_comment = 342,
  sym_translation_unit = 343,
  sym_global_directive = 344,
  sym_bool_literal = 345,
  sym_int_literal = 346,
  sym_decimal_int_literal = 347,
  sym_float_literal = 348,
  sym_decimal_float_literal = 349,
  sym_hex_float_literal = 350,
  sym_literal = 351,
  sym_member_ident = 352,
  sym_attribute = 353,
  sym_attrib_end = 354,
  sym_array_type_specifier = 355,
  sym_element_count_expression = 356,
  sym_struct_decl = 357,
  sym_struct_body_decl = 358,
  sym_struct_member = 359,
  sym_texture_and_sampler_types = 360,
  sym_sampler_type = 361,
  sym_sampled_texture_type = 362,
  sym_storage_texture_type = 363,
  sym_depth_texture_type = 364,
  sym_type_alias_decl = 365,
  sym_type_specifier = 366,
  sym_type_specifier_without_ident = 367,
  sym_vec_prefix = 368,
  sym_mat_prefix = 369,
  sym_variable_statement = 370,
  sym_variable_decl = 371,
  sym_optionally_typed_ident = 372,
  sym_variable_qualifier = 373,
  sym_global_variable_decl = 374,
  sym_global_constant_decl = 375,
  sym_primary_expression = 376,
  sym_call_expression = 377,
  sym_call_phrase = 378,
  sym_callable = 379,
  sym_paren_expression = 380,
  sym_argument_expression_list = 381,
  sym_expression_comma_list = 382,
  sym_component_or_swizzle_specifier = 383,
  sym_unary_expression = 384,
  sym_singular_expression = 385,
  sym_lhs_expression = 386,
  sym_core_lhs_expression = 387,
  sym_multiplicative_expression = 388,
  sym_multiplicative_operator = 389,
  sym_additive_expression = 390,
  sym_additive_operator = 391,
  sym_shift_expression = 392,
  sym_relational_expression = 393,
  sym_short_circuit_and_expression = 394,
  sym_short_circuit_or_expression = 395,
  sym_binary_or_expression = 396,
  sym_binary_and_expression = 397,
  sym_binary_xor_expression = 398,
  sym_bitwise_expression = 399,
  sym_expression = 400,
  sym_compound_statement = 401,
  sym_assignment_statement = 402,
  sym_compound_assignment_operator = 403,
  sym_increment_statement = 404,
  sym_decrement_statement = 405,
  sym_if_statement = 406,
  sym_if_clause = 407,
  sym_else_if_clause = 408,
  sym_else_clause = 409,
  sym_switch_statement = 410,
  sym_switch_body = 411,
  sym_case_clause = 412,
  sym_default_alone_clause = 413,
  sym_case_selectors = 414,
  sym_case_selector = 415,
  sym_loop_statement = 416,
  sym_for_statement = 417,
  sym_for_header = 418,
  sym_for_init = 419,
  sym_for_update = 420,
  sym_while_statement = 421,
  sym_break_statement = 422,
  sym_break_if_statement = 423,
  sym_continuing_statement = 424,
  sym_continuing_compound_statement = 425,
  sym_return_statement = 426,
  sym_func_call_statement = 427,
  sym_const_assert_statement = 428,
  sym_statement = 429,
  sym_variable_updating_statement = 430,
  sym_function_decl = 431,
  sym_function_header = 432,
  sym_param_list = 433,
  sym_param = 434,
  sym_enable_directive = 435,
  sym_interpolation_type_name = 436,
  sym_interpolation_sample_name = 437,
  sym_builtin_value_name = 438,
  sym_access_mode = 439,
  sym_address_space = 440,
  sym_texel_format = 441,
  sym_extension_name = 442,
  sym_swizzle_name = 443,
  sym_ident = 444,
  sym__comment = 445,
  aux_sym_translation_unit_repeat1 = 446,
  aux_sym_translation_unit_repeat2 = 447,
  aux_sym_struct_body_decl_repeat1 = 448,
  aux_sym_struct_member_repeat1 = 449,
  aux_sym_expression_comma_list_repeat1 = 450,
  aux_sym_compound_statement_repeat1 = 451,
  aux_sym_if_statement_repeat1 = 452,
  aux_sym_switch_statement_repeat1 = 453,
  aux_sym_case_selectors_repeat1 = 454,
  aux_sym_param_list_repeat1 = 455,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_ident_pattern_token] = "ident_pattern_token",
  [anon_sym_SEMI] = ";",
  [anon_sym_true] = "true",
  [anon_sym_false] = "false",
  [aux_sym_decimal_int_literal_token1] = "decimal_int_literal_token1",
  [aux_sym_decimal_int_literal_token2] = "decimal_int_literal_token2",
  [sym_hex_int_literal] = "hex_int_literal",
  [aux_sym_decimal_float_literal_token1] = "decimal_float_literal_token1",
  [aux_sym_decimal_float_literal_token2] = "decimal_float_literal_token2",
  [aux_sym_decimal_float_literal_token3] = "decimal_float_literal_token3",
  [aux_sym_decimal_float_literal_token4] = "decimal_float_literal_token4",
  [aux_sym_decimal_float_literal_token5] = "decimal_float_literal_token5",
  [aux_sym_hex_float_literal_token1] = "hex_float_literal_token1",
  [aux_sym_hex_float_literal_token2] = "hex_float_literal_token2",
  [aux_sym_hex_float_literal_token3] = "hex_float_literal_token3",
  [anon_sym_AT] = "@",
  [anon_sym_align] = "align",
  [anon_sym_LPAREN] = "(",
  [anon_sym_binding] = "binding",
  [anon_sym_builtin] = "builtin",
  [anon_sym_const] = "const",
  [anon_sym_group] = "group",
  [anon_sym_id] = "id",
  [anon_sym_interpolate] = "interpolate",
  [anon_sym_COMMA] = ",",
  [anon_sym_invariant] = "invariant",
  [anon_sym_location] = "location",
  [anon_sym_size] = "size",
  [anon_sym_workgroup_size] = "workgroup_size",
  [anon_sym_vertex] = "vertex",
  [anon_sym_fragment] = "fragment",
  [anon_sym_compute] = "compute",
  [anon_sym_RPAREN] = ")",
  [anon_sym_array] = "array",
  [anon_sym_LT] = "<",
  [anon_sym_GT] = ">",
  [anon_sym_struct] = "struct",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_COLON] = ":",
  [anon_sym_sampler] = "sampler",
  [anon_sym_sampler_comparison] = "sampler_comparison",
  [anon_sym_texture_1d] = "texture_1d",
  [anon_sym_texture_2d] = "texture_2d",
  [anon_sym_texture_2d_array] = "texture_2d_array",
  [anon_sym_texture_3d] = "texture_3d",
  [anon_sym_texture_cube] = "texture_cube",
  [anon_sym_texture_cube_array] = "texture_cube_array",
  [sym_multisampled_texture_type] = "multisampled_texture_type",
  [anon_sym_texture_storage_1d] = "texture_storage_1d",
  [anon_sym_texture_storage_2d] = "texture_storage_2d",
  [anon_sym_texture_storage_2d_array] = "texture_storage_2d_array",
  [anon_sym_texture_storage_3d] = "texture_storage_3d",
  [anon_sym_texture_depth_2d] = "texture_depth_2d",
  [anon_sym_texture_depth_2d_array] = "texture_depth_2d_array",
  [anon_sym_texture_depth_cube] = "texture_depth_cube",
  [anon_sym_texture_depth_cube_array] = "texture_depth_cube_array",
  [anon_sym_texture_depth_multisampled_2d] = "texture_depth_multisampled_2d",
  [anon_sym_alias] = "alias",
  [anon_sym_EQ] = "=",
  [anon_sym_bool] = "bool",
  [anon_sym_f32] = "f32",
  [anon_sym_f16] = "f16",
  [anon_sym_i32] = "i32",
  [anon_sym_u32] = "u32",
  [anon_sym_ptr] = "ptr",
  [anon_sym_atomic] = "atomic",
  [anon_sym_vec2] = "vec2",
  [anon_sym_vec3] = "vec3",
  [anon_sym_vec4] = "vec4",
  [anon_sym_mat2x2] = "mat2x2",
  [anon_sym_mat2x3] = "mat2x3",
  [anon_sym_mat2x4] = "mat2x4",
  [anon_sym_mat3x2] = "mat3x2",
  [anon_sym_mat3x3] = "mat3x3",
  [anon_sym_mat3x4] = "mat3x4",
  [anon_sym_mat4x2] = "mat4x2",
  [anon_sym_mat4x3] = "mat4x3",
  [anon_sym_mat4x4] = "mat4x4",
  [anon_sym_let] = "let",
  [anon_sym_var] = "var",
  [anon_sym_override] = "override",
  [anon_sym_bitcast] = "bitcast",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [anon_sym_DOT] = ".",
  [anon_sym_DASH] = "-",
  [anon_sym_BANG] = "!",
  [anon_sym_TILDE] = "~",
  [anon_sym_STAR] = "*",
  [anon_sym_AMP] = "&",
  [anon_sym_SLASH] = "/",
  [anon_sym_PERCENT] = "%",
  [anon_sym_PLUS] = "+",
  [anon_sym_LT_LT] = "<<",
  [anon_sym_GT_GT] = ">>",
  [anon_sym_LT_EQ] = "<=",
  [anon_sym_GT_EQ] = ">=",
  [anon_sym_EQ_EQ] = "==",
  [anon_sym_BANG_EQ] = "!=",
  [anon_sym_AMP_AMP] = "&&",
  [anon_sym_PIPE_PIPE] = "||",
  [anon_sym_PIPE] = "|",
  [anon_sym_CARET] = "^",
  [anon_sym__] = "_",
  [anon_sym_PLUS_EQ] = "+=",
  [anon_sym_DASH_EQ] = "-=",
  [anon_sym_STAR_EQ] = "*=",
  [anon_sym_SLASH_EQ] = "/=",
  [anon_sym_PERCENT_EQ] = "%=",
  [anon_sym_AMP_EQ] = "&=",
  [anon_sym_PIPE_EQ] = "|=",
  [anon_sym_CARET_EQ] = "^=",
  [anon_sym_GT_GT_EQ] = ">>=",
  [anon_sym_LT_LT_EQ] = "<<=",
  [anon_sym_PLUS_PLUS] = "++",
  [anon_sym_DASH_DASH] = "--",
  [anon_sym_if] = "if",
  [anon_sym_else] = "else",
  [anon_sym_switch] = "switch",
  [anon_sym_case] = "case",
  [anon_sym_default] = "default",
  [anon_sym_loop] = "loop",
  [anon_sym_for] = "for",
  [anon_sym_while] = "while",
  [anon_sym_break] = "break",
  [sym_continue_statement] = "continue_statement",
  [anon_sym_continuing] = "continuing",
  [anon_sym_return] = "return",
  [anon_sym_const_assert] = "const_assert",
  [anon_sym_discard] = "discard",
  [anon_sym_fn] = "fn",
  [anon_sym_DASH_GT] = "->",
  [anon_sym_enable] = "enable",
  [anon_sym_perspective] = "perspective",
  [anon_sym_linear] = "linear",
  [anon_sym_flat] = "flat",
  [anon_sym_center] = "center",
  [anon_sym_centroid] = "centroid",
  [anon_sym_sample] = "sample",
  [anon_sym_vertex_index] = "vertex_index",
  [anon_sym_instance_index] = "instance_index",
  [anon_sym_position] = "position",
  [anon_sym_front_facing] = "front_facing",
  [anon_sym_frag_depth] = "frag_depth",
  [anon_sym_local_invocation_id] = "local_invocation_id",
  [anon_sym_local_invocation_index] = "local_invocation_index",
  [anon_sym_global_invocation_id] = "global_invocation_id",
  [anon_sym_workgroup_id] = "workgroup_id",
  [anon_sym_num_workgroups] = "num_workgroups",
  [anon_sym_sample_index] = "sample_index",
  [anon_sym_sample_mask] = "sample_mask",
  [anon_sym_read] = "read",
  [anon_sym_write] = "write",
  [anon_sym_read_write] = "read_write",
  [anon_sym_function] = "function",
  [anon_sym_private] = "private",
  [anon_sym_workgroup] = "workgroup",
  [anon_sym_uniform] = "uniform",
  [anon_sym_storage] = "storage",
  [anon_sym_rgba8unorm] = "rgba8unorm",
  [anon_sym_rgba8snorm] = "rgba8snorm",
  [anon_sym_rgba8uint] = "rgba8uint",
  [anon_sym_rgba8sint] = "rgba8sint",
  [anon_sym_rgba16uint] = "rgba16uint",
  [anon_sym_rgba16sint] = "rgba16sint",
  [anon_sym_rgba16float] = "rgba16float",
  [anon_sym_r32uint] = "r32uint",
  [anon_sym_r32sint] = "r32sint",
  [anon_sym_r32float] = "r32float",
  [anon_sym_rg32uint] = "rg32uint",
  [anon_sym_rg32sint] = "rg32sint",
  [anon_sym_rg32float] = "rg32float",
  [anon_sym_rgba32uint] = "rgba32uint",
  [anon_sym_rgba32sint] = "rgba32sint",
  [anon_sym_rgba32float] = "rgba32float",
  [anon_sym_bgra8unorm] = "bgra8unorm",
  [anon_sym_SLASH_LBRACKrgba_RBRACK_SLASH] = "/[rgba]/",
  [anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH] = "/[rgba][rgba]/",
  [anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH] = "/[rgba][rgba][rgba]/",
  [anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH] = "/[rgba][rgba][rgba][rgba]/",
  [anon_sym_SLASH_LBRACKxyzw_RBRACK_SLASH] = "/[xyzw]/",
  [anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH] = "/[xyzw][xyzw]/",
  [anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH] = "/[xyzw][xyzw][xyzw]/",
  [anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH] = "/[xyzw][xyzw][xyzw][xyzw]/",
  [anon_sym_CompileShader] = "CompileShader",
  [anon_sym_ComputeShader] = "ComputeShader",
  [anon_sym_DomainShader] = "DomainShader",
  [anon_sym_GeometryShader] = "GeometryShader",
  [anon_sym_Hullshader] = "Hullshader",
  [anon_sym_NULL] = "NULL",
  [anon_sym_Self] = "Self",
  [anon_sym_abstract] = "abstract",
  [anon_sym_active] = "active",
  [anon_sym_alignas] = "alignas",
  [anon_sym_alignof] = "alignof",
  [anon_sym_as] = "as",
  [anon_sym_asm] = "asm",
  [anon_sym_asm_fragment] = "asm_fragment",
  [anon_sym_async] = "async",
  [anon_sym_attribute] = "attribute",
  [anon_sym_auto] = "auto",
  [anon_sym_await] = "await",
  [anon_sym_become] = "become",
  [anon_sym_binding_array] = "binding_array",
  [anon_sym_cast] = "cast",
  [anon_sym_catch] = "catch",
  [anon_sym_class] = "class",
  [anon_sym_co_await] = "co_await",
  [anon_sym_co_return] = "co_return",
  [anon_sym_co_yield] = "co_yield",
  [anon_sym_coherent] = "coherent",
  [anon_sym_column_major] = "column_major",
  [anon_sym_common] = "common",
  [anon_sym_compile] = "compile",
  [anon_sym_compile_fragment] = "compile_fragment",
  [anon_sym_concept] = "concept",
  [anon_sym_const_cast] = "const_cast",
  [anon_sym_consteval] = "consteval",
  [anon_sym_constexpr] = "constexpr",
  [anon_sym_constinit] = "constinit",
  [anon_sym_crate] = "crate",
  [anon_sym_debugger] = "debugger",
  [anon_sym_decltype] = "decltype",
  [anon_sym_delete] = "delete",
  [anon_sym_demote] = "demote",
  [anon_sym_demote_to_helper] = "demote_to_helper",
  [anon_sym_do] = "do",
  [anon_sym_dynamic_cast] = "dynamic_cast",
  [anon_sym_enum] = "enum",
  [anon_sym_explicit] = "explicit",
  [anon_sym_export] = "export",
  [anon_sym_extends] = "extends",
  [anon_sym_extern] = "extern",
  [anon_sym_external] = "external",
  [anon_sym_fallthrough] = "fallthrough",
  [anon_sym_filter] = "filter",
  [anon_sym_final] = "final",
  [anon_sym_finally] = "finally",
  [anon_sym_friend] = "friend",
  [anon_sym_from] = "from",
  [anon_sym_fxgroup] = "fxgroup",
  [anon_sym_get] = "get",
  [anon_sym_goto] = "goto",
  [anon_sym_groupshared] = "groupshared",
  [anon_sym_handle] = "handle",
  [anon_sym_highp] = "highp",
  [anon_sym_impl] = "impl",
  [anon_sym_implements] = "implements",
  [anon_sym_import] = "import",
  [anon_sym_inline] = "inline",
  [anon_sym_inout] = "inout",
  [anon_sym_instanceof] = "instanceof",
  [anon_sym_interface] = "interface",
  [anon_sym_layout] = "layout",
  [anon_sym_lowp] = "lowp",
  [anon_sym_macro] = "macro",
  [anon_sym_macro_rules] = "macro_rules",
  [anon_sym_match] = "match",
  [anon_sym_mediump] = "mediump",
  [anon_sym_meta] = "meta",
  [anon_sym_mod] = "mod",
  [anon_sym_module] = "module",
  [anon_sym_move] = "move",
  [anon_sym_mut] = "mut",
  [anon_sym_mutable] = "mutable",
  [anon_sym_namespace] = "namespace",
  [anon_sym_new] = "new",
  [anon_sym_nil] = "nil",
  [anon_sym_noexcept] = "noexcept",
  [anon_sym_noinline] = "noinline",
  [anon_sym_nointerpolation] = "nointerpolation",
  [anon_sym_noperspective] = "noperspective",
  [anon_sym_null] = "null",
  [anon_sym_nullptr] = "nullptr",
  [anon_sym_of] = "of",
  [anon_sym_operator] = "operator",
  [anon_sym_package] = "package",
  [anon_sym_packoffset] = "packoffset",
  [anon_sym_partition] = "partition",
  [anon_sym_pass] = "pass",
  [anon_sym_patch] = "patch",
  [anon_sym_pixelfragment] = "pixelfragment",
  [anon_sym_precise] = "precise",
  [anon_sym_precision] = "precision",
  [anon_sym_premerge] = "premerge",
  [anon_sym_priv] = "priv",
  [anon_sym_protected] = "protected",
  [anon_sym_pub] = "pub",
  [anon_sym_public] = "public",
  [anon_sym_readonly] = "readonly",
  [anon_sym_ref] = "ref",
  [anon_sym_regardless] = "regardless",
  [anon_sym_register] = "register",
  [anon_sym_reinterpret_cast] = "reinterpret_cast",
  [anon_sym_requires] = "requires",
  [anon_sym_resource] = "resource",
  [anon_sym_restrict] = "restrict",
  [anon_sym_self] = "self",
  [anon_sym_set] = "set",
  [anon_sym_shared] = "shared",
  [anon_sym_signed] = "signed",
  [anon_sym_sizeof] = "sizeof",
  [anon_sym_smooth] = "smooth",
  [anon_sym_snorm] = "snorm",
  [anon_sym_static] = "static",
  [anon_sym_static_assert] = "static_assert",
  [anon_sym_static_cast] = "static_cast",
  [anon_sym_std] = "std",
  [anon_sym_subroutine] = "subroutine",
  [anon_sym_super] = "super",
  [anon_sym_target] = "target",
  [anon_sym_template] = "template",
  [anon_sym_this] = "this",
  [anon_sym_thread_local] = "thread_local",
  [anon_sym_throw] = "throw",
  [anon_sym_trait] = "trait",
  [anon_sym_try] = "try",
  [anon_sym_type] = "type",
  [anon_sym_typedef] = "typedef",
  [anon_sym_typeid] = "typeid",
  [anon_sym_typename] = "typename",
  [anon_sym_typeof] = "typeof",
  [anon_sym_union] = "union",
  [anon_sym_unless] = "unless",
  [anon_sym_unorm] = "unorm",
  [anon_sym_unsafe] = "unsafe",
  [anon_sym_unsized] = "unsized",
  [anon_sym_use] = "use",
  [anon_sym_using] = "using",
  [anon_sym_varying] = "varying",
  [anon_sym_virtual] = "virtual",
  [anon_sym_volatile] = "volatile",
  [anon_sym_wgsl] = "wgsl",
  [anon_sym_where] = "where",
  [anon_sym_with] = "with",
  [anon_sym_writeonly] = "writeonly",
  [anon_sym_yield] = "yield",
  [anon_sym_SLASH_SLASH] = "//",
  [aux_sym__comment_token1] = "_comment_token1",
  [sym__blankspace] = "_blankspace",
  [sym__block_comment] = "_block_comment",
  [sym_translation_unit] = "translation_unit",
  [sym_global_directive] = "global_directive",
  [sym_bool_literal] = "bool_literal",
  [sym_int_literal] = "int_literal",
  [sym_decimal_int_literal] = "decimal_int_literal",
  [sym_float_literal] = "float_literal",
  [sym_decimal_float_literal] = "decimal_float_literal",
  [sym_hex_float_literal] = "hex_float_literal",
  [sym_literal] = "literal",
  [sym_member_ident] = "member_ident",
  [sym_attribute] = "attribute",
  [sym_attrib_end] = "attrib_end",
  [sym_array_type_specifier] = "array_type_specifier",
  [sym_element_count_expression] = "element_count_expression",
  [sym_struct_decl] = "struct_decl",
  [sym_struct_body_decl] = "struct_body_decl",
  [sym_struct_member] = "struct_member",
  [sym_texture_and_sampler_types] = "texture_and_sampler_types",
  [sym_sampler_type] = "sampler_type",
  [sym_sampled_texture_type] = "sampled_texture_type",
  [sym_storage_texture_type] = "storage_texture_type",
  [sym_depth_texture_type] = "depth_texture_type",
  [sym_type_alias_decl] = "type_alias_decl",
  [sym_type_specifier] = "type_specifier",
  [sym_type_specifier_without_ident] = "type_specifier_without_ident",
  [sym_vec_prefix] = "vec_prefix",
  [sym_mat_prefix] = "mat_prefix",
  [sym_variable_statement] = "variable_statement",
  [sym_variable_decl] = "variable_decl",
  [sym_optionally_typed_ident] = "optionally_typed_ident",
  [sym_variable_qualifier] = "variable_qualifier",
  [sym_global_variable_decl] = "global_variable_decl",
  [sym_global_constant_decl] = "global_constant_decl",
  [sym_primary_expression] = "primary_expression",
  [sym_call_expression] = "call_expression",
  [sym_call_phrase] = "call_phrase",
  [sym_callable] = "callable",
  [sym_paren_expression] = "paren_expression",
  [sym_argument_expression_list] = "argument_expression_list",
  [sym_expression_comma_list] = "expression_comma_list",
  [sym_component_or_swizzle_specifier] = "component_or_swizzle_specifier",
  [sym_unary_expression] = "unary_expression",
  [sym_singular_expression] = "singular_expression",
  [sym_lhs_expression] = "lhs_expression",
  [sym_core_lhs_expression] = "core_lhs_expression",
  [sym_multiplicative_expression] = "multiplicative_expression",
  [sym_multiplicative_operator] = "multiplicative_operator",
  [sym_additive_expression] = "additive_expression",
  [sym_additive_operator] = "additive_operator",
  [sym_shift_expression] = "shift_expression",
  [sym_relational_expression] = "relational_expression",
  [sym_short_circuit_and_expression] = "short_circuit_and_expression",
  [sym_short_circuit_or_expression] = "short_circuit_or_expression",
  [sym_binary_or_expression] = "binary_or_expression",
  [sym_binary_and_expression] = "binary_and_expression",
  [sym_binary_xor_expression] = "binary_xor_expression",
  [sym_bitwise_expression] = "bitwise_expression",
  [sym_expression] = "expression",
  [sym_compound_statement] = "compound_statement",
  [sym_assignment_statement] = "assignment_statement",
  [sym_compound_assignment_operator] = "compound_assignment_operator",
  [sym_increment_statement] = "increment_statement",
  [sym_decrement_statement] = "decrement_statement",
  [sym_if_statement] = "if_statement",
  [sym_if_clause] = "if_clause",
  [sym_else_if_clause] = "else_if_clause",
  [sym_else_clause] = "else_clause",
  [sym_switch_statement] = "switch_statement",
  [sym_switch_body] = "switch_body",
  [sym_case_clause] = "case_clause",
  [sym_default_alone_clause] = "default_alone_clause",
  [sym_case_selectors] = "case_selectors",
  [sym_case_selector] = "case_selector",
  [sym_loop_statement] = "loop_statement",
  [sym_for_statement] = "for_statement",
  [sym_for_header] = "for_header",
  [sym_for_init] = "for_init",
  [sym_for_update] = "for_update",
  [sym_while_statement] = "while_statement",
  [sym_break_statement] = "break_statement",
  [sym_break_if_statement] = "break_if_statement",
  [sym_continuing_statement] = "continuing_statement",
  [sym_continuing_compound_statement] = "continuing_compound_statement",
  [sym_return_statement] = "return_statement",
  [sym_func_call_statement] = "func_call_statement",
  [sym_const_assert_statement] = "const_assert_statement",
  [sym_statement] = "statement",
  [sym_variable_updating_statement] = "variable_updating_statement",
  [sym_function_decl] = "function_decl",
  [sym_function_header] = "function_header",
  [sym_param_list] = "param_list",
  [sym_param] = "param",
  [sym_enable_directive] = "enable_directive",
  [sym_interpolation_type_name] = "interpolation_type_name",
  [sym_interpolation_sample_name] = "interpolation_sample_name",
  [sym_builtin_value_name] = "builtin_value_name",
  [sym_access_mode] = "access_mode",
  [sym_address_space] = "address_space",
  [sym_texel_format] = "texel_format",
  [sym_extension_name] = "extension_name",
  [sym_swizzle_name] = "swizzle_name",
  [sym_ident] = "ident",
  [sym__comment] = "_comment",
  [aux_sym_translation_unit_repeat1] = "translation_unit_repeat1",
  [aux_sym_translation_unit_repeat2] = "translation_unit_repeat2",
  [aux_sym_struct_body_decl_repeat1] = "struct_body_decl_repeat1",
  [aux_sym_struct_member_repeat1] = "struct_member_repeat1",
  [aux_sym_expression_comma_list_repeat1] = "expression_comma_list_repeat1",
  [aux_sym_compound_statement_repeat1] = "compound_statement_repeat1",
  [aux_sym_if_statement_repeat1] = "if_statement_repeat1",
  [aux_sym_switch_statement_repeat1] = "switch_statement_repeat1",
  [aux_sym_case_selectors_repeat1] = "case_selectors_repeat1",
  [aux_sym_param_list_repeat1] = "param_list_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_ident_pattern_token] = sym_ident_pattern_token,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_true] = anon_sym_true,
  [anon_sym_false] = anon_sym_false,
  [aux_sym_decimal_int_literal_token1] = aux_sym_decimal_int_literal_token1,
  [aux_sym_decimal_int_literal_token2] = aux_sym_decimal_int_literal_token2,
  [sym_hex_int_literal] = sym_hex_int_literal,
  [aux_sym_decimal_float_literal_token1] = aux_sym_decimal_float_literal_token1,
  [aux_sym_decimal_float_literal_token2] = aux_sym_decimal_float_literal_token2,
  [aux_sym_decimal_float_literal_token3] = aux_sym_decimal_float_literal_token3,
  [aux_sym_decimal_float_literal_token4] = aux_sym_decimal_float_literal_token4,
  [aux_sym_decimal_float_literal_token5] = aux_sym_decimal_float_literal_token5,
  [aux_sym_hex_float_literal_token1] = aux_sym_hex_float_literal_token1,
  [aux_sym_hex_float_literal_token2] = aux_sym_hex_float_literal_token2,
  [aux_sym_hex_float_literal_token3] = aux_sym_hex_float_literal_token3,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_align] = anon_sym_align,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_binding] = anon_sym_binding,
  [anon_sym_builtin] = anon_sym_builtin,
  [anon_sym_const] = anon_sym_const,
  [anon_sym_group] = anon_sym_group,
  [anon_sym_id] = anon_sym_id,
  [anon_sym_interpolate] = anon_sym_interpolate,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_invariant] = anon_sym_invariant,
  [anon_sym_location] = anon_sym_location,
  [anon_sym_size] = anon_sym_size,
  [anon_sym_workgroup_size] = anon_sym_workgroup_size,
  [anon_sym_vertex] = anon_sym_vertex,
  [anon_sym_fragment] = anon_sym_fragment,
  [anon_sym_compute] = anon_sym_compute,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_array] = anon_sym_array,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_struct] = anon_sym_struct,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_sampler] = anon_sym_sampler,
  [anon_sym_sampler_comparison] = anon_sym_sampler_comparison,
  [anon_sym_texture_1d] = anon_sym_texture_1d,
  [anon_sym_texture_2d] = anon_sym_texture_2d,
  [anon_sym_texture_2d_array] = anon_sym_texture_2d_array,
  [anon_sym_texture_3d] = anon_sym_texture_3d,
  [anon_sym_texture_cube] = anon_sym_texture_cube,
  [anon_sym_texture_cube_array] = anon_sym_texture_cube_array,
  [sym_multisampled_texture_type] = sym_multisampled_texture_type,
  [anon_sym_texture_storage_1d] = anon_sym_texture_storage_1d,
  [anon_sym_texture_storage_2d] = anon_sym_texture_storage_2d,
  [anon_sym_texture_storage_2d_array] = anon_sym_texture_storage_2d_array,
  [anon_sym_texture_storage_3d] = anon_sym_texture_storage_3d,
  [anon_sym_texture_depth_2d] = anon_sym_texture_depth_2d,
  [anon_sym_texture_depth_2d_array] = anon_sym_texture_depth_2d_array,
  [anon_sym_texture_depth_cube] = anon_sym_texture_depth_cube,
  [anon_sym_texture_depth_cube_array] = anon_sym_texture_depth_cube_array,
  [anon_sym_texture_depth_multisampled_2d] = anon_sym_texture_depth_multisampled_2d,
  [anon_sym_alias] = anon_sym_alias,
  [anon_sym_EQ] = anon_sym_EQ,
  [anon_sym_bool] = anon_sym_bool,
  [anon_sym_f32] = anon_sym_f32,
  [anon_sym_f16] = anon_sym_f16,
  [anon_sym_i32] = anon_sym_i32,
  [anon_sym_u32] = anon_sym_u32,
  [anon_sym_ptr] = anon_sym_ptr,
  [anon_sym_atomic] = anon_sym_atomic,
  [anon_sym_vec2] = anon_sym_vec2,
  [anon_sym_vec3] = anon_sym_vec3,
  [anon_sym_vec4] = anon_sym_vec4,
  [anon_sym_mat2x2] = anon_sym_mat2x2,
  [anon_sym_mat2x3] = anon_sym_mat2x3,
  [anon_sym_mat2x4] = anon_sym_mat2x4,
  [anon_sym_mat3x2] = anon_sym_mat3x2,
  [anon_sym_mat3x3] = anon_sym_mat3x3,
  [anon_sym_mat3x4] = anon_sym_mat3x4,
  [anon_sym_mat4x2] = anon_sym_mat4x2,
  [anon_sym_mat4x3] = anon_sym_mat4x3,
  [anon_sym_mat4x4] = anon_sym_mat4x4,
  [anon_sym_let] = anon_sym_let,
  [anon_sym_var] = anon_sym_var,
  [anon_sym_override] = anon_sym_override,
  [anon_sym_bitcast] = anon_sym_bitcast,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_BANG] = anon_sym_BANG,
  [anon_sym_TILDE] = anon_sym_TILDE,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_AMP] = anon_sym_AMP,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_PERCENT] = anon_sym_PERCENT,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_LT_LT] = anon_sym_LT_LT,
  [anon_sym_GT_GT] = anon_sym_GT_GT,
  [anon_sym_LT_EQ] = anon_sym_LT_EQ,
  [anon_sym_GT_EQ] = anon_sym_GT_EQ,
  [anon_sym_EQ_EQ] = anon_sym_EQ_EQ,
  [anon_sym_BANG_EQ] = anon_sym_BANG_EQ,
  [anon_sym_AMP_AMP] = anon_sym_AMP_AMP,
  [anon_sym_PIPE_PIPE] = anon_sym_PIPE_PIPE,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_CARET] = anon_sym_CARET,
  [anon_sym__] = anon_sym__,
  [anon_sym_PLUS_EQ] = anon_sym_PLUS_EQ,
  [anon_sym_DASH_EQ] = anon_sym_DASH_EQ,
  [anon_sym_STAR_EQ] = anon_sym_STAR_EQ,
  [anon_sym_SLASH_EQ] = anon_sym_SLASH_EQ,
  [anon_sym_PERCENT_EQ] = anon_sym_PERCENT_EQ,
  [anon_sym_AMP_EQ] = anon_sym_AMP_EQ,
  [anon_sym_PIPE_EQ] = anon_sym_PIPE_EQ,
  [anon_sym_CARET_EQ] = anon_sym_CARET_EQ,
  [anon_sym_GT_GT_EQ] = anon_sym_GT_GT_EQ,
  [anon_sym_LT_LT_EQ] = anon_sym_LT_LT_EQ,
  [anon_sym_PLUS_PLUS] = anon_sym_PLUS_PLUS,
  [anon_sym_DASH_DASH] = anon_sym_DASH_DASH,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_else] = anon_sym_else,
  [anon_sym_switch] = anon_sym_switch,
  [anon_sym_case] = anon_sym_case,
  [anon_sym_default] = anon_sym_default,
  [anon_sym_loop] = anon_sym_loop,
  [anon_sym_for] = anon_sym_for,
  [anon_sym_while] = anon_sym_while,
  [anon_sym_break] = anon_sym_break,
  [sym_continue_statement] = sym_continue_statement,
  [anon_sym_continuing] = anon_sym_continuing,
  [anon_sym_return] = anon_sym_return,
  [anon_sym_const_assert] = anon_sym_const_assert,
  [anon_sym_discard] = anon_sym_discard,
  [anon_sym_fn] = anon_sym_fn,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
  [anon_sym_enable] = anon_sym_enable,
  [anon_sym_perspective] = anon_sym_perspective,
  [anon_sym_linear] = anon_sym_linear,
  [anon_sym_flat] = anon_sym_flat,
  [anon_sym_center] = anon_sym_center,
  [anon_sym_centroid] = anon_sym_centroid,
  [anon_sym_sample] = anon_sym_sample,
  [anon_sym_vertex_index] = anon_sym_vertex_index,
  [anon_sym_instance_index] = anon_sym_instance_index,
  [anon_sym_position] = anon_sym_position,
  [anon_sym_front_facing] = anon_sym_front_facing,
  [anon_sym_frag_depth] = anon_sym_frag_depth,
  [anon_sym_local_invocation_id] = anon_sym_local_invocation_id,
  [anon_sym_local_invocation_index] = anon_sym_local_invocation_index,
  [anon_sym_global_invocation_id] = anon_sym_global_invocation_id,
  [anon_sym_workgroup_id] = anon_sym_workgroup_id,
  [anon_sym_num_workgroups] = anon_sym_num_workgroups,
  [anon_sym_sample_index] = anon_sym_sample_index,
  [anon_sym_sample_mask] = anon_sym_sample_mask,
  [anon_sym_read] = anon_sym_read,
  [anon_sym_write] = anon_sym_write,
  [anon_sym_read_write] = anon_sym_read_write,
  [anon_sym_function] = anon_sym_function,
  [anon_sym_private] = anon_sym_private,
  [anon_sym_workgroup] = anon_sym_workgroup,
  [anon_sym_uniform] = anon_sym_uniform,
  [anon_sym_storage] = anon_sym_storage,
  [anon_sym_rgba8unorm] = anon_sym_rgba8unorm,
  [anon_sym_rgba8snorm] = anon_sym_rgba8snorm,
  [anon_sym_rgba8uint] = anon_sym_rgba8uint,
  [anon_sym_rgba8sint] = anon_sym_rgba8sint,
  [anon_sym_rgba16uint] = anon_sym_rgba16uint,
  [anon_sym_rgba16sint] = anon_sym_rgba16sint,
  [anon_sym_rgba16float] = anon_sym_rgba16float,
  [anon_sym_r32uint] = anon_sym_r32uint,
  [anon_sym_r32sint] = anon_sym_r32sint,
  [anon_sym_r32float] = anon_sym_r32float,
  [anon_sym_rg32uint] = anon_sym_rg32uint,
  [anon_sym_rg32sint] = anon_sym_rg32sint,
  [anon_sym_rg32float] = anon_sym_rg32float,
  [anon_sym_rgba32uint] = anon_sym_rgba32uint,
  [anon_sym_rgba32sint] = anon_sym_rgba32sint,
  [anon_sym_rgba32float] = anon_sym_rgba32float,
  [anon_sym_bgra8unorm] = anon_sym_bgra8unorm,
  [anon_sym_SLASH_LBRACKrgba_RBRACK_SLASH] = anon_sym_SLASH_LBRACKrgba_RBRACK_SLASH,
  [anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH] = anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
  [anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH] = anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
  [anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH] = anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
  [anon_sym_SLASH_LBRACKxyzw_RBRACK_SLASH] = anon_sym_SLASH_LBRACKxyzw_RBRACK_SLASH,
  [anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH] = anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
  [anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH] = anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
  [anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH] = anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
  [anon_sym_CompileShader] = anon_sym_CompileShader,
  [anon_sym_ComputeShader] = anon_sym_ComputeShader,
  [anon_sym_DomainShader] = anon_sym_DomainShader,
  [anon_sym_GeometryShader] = anon_sym_GeometryShader,
  [anon_sym_Hullshader] = anon_sym_Hullshader,
  [anon_sym_NULL] = anon_sym_NULL,
  [anon_sym_Self] = anon_sym_Self,
  [anon_sym_abstract] = anon_sym_abstract,
  [anon_sym_active] = anon_sym_active,
  [anon_sym_alignas] = anon_sym_alignas,
  [anon_sym_alignof] = anon_sym_alignof,
  [anon_sym_as] = anon_sym_as,
  [anon_sym_asm] = anon_sym_asm,
  [anon_sym_asm_fragment] = anon_sym_asm_fragment,
  [anon_sym_async] = anon_sym_async,
  [anon_sym_attribute] = anon_sym_attribute,
  [anon_sym_auto] = anon_sym_auto,
  [anon_sym_await] = anon_sym_await,
  [anon_sym_become] = anon_sym_become,
  [anon_sym_binding_array] = anon_sym_binding_array,
  [anon_sym_cast] = anon_sym_cast,
  [anon_sym_catch] = anon_sym_catch,
  [anon_sym_class] = anon_sym_class,
  [anon_sym_co_await] = anon_sym_co_await,
  [anon_sym_co_return] = anon_sym_co_return,
  [anon_sym_co_yield] = anon_sym_co_yield,
  [anon_sym_coherent] = anon_sym_coherent,
  [anon_sym_column_major] = anon_sym_column_major,
  [anon_sym_common] = anon_sym_common,
  [anon_sym_compile] = anon_sym_compile,
  [anon_sym_compile_fragment] = anon_sym_compile_fragment,
  [anon_sym_concept] = anon_sym_concept,
  [anon_sym_const_cast] = anon_sym_const_cast,
  [anon_sym_consteval] = anon_sym_consteval,
  [anon_sym_constexpr] = anon_sym_constexpr,
  [anon_sym_constinit] = anon_sym_constinit,
  [anon_sym_crate] = anon_sym_crate,
  [anon_sym_debugger] = anon_sym_debugger,
  [anon_sym_decltype] = anon_sym_decltype,
  [anon_sym_delete] = anon_sym_delete,
  [anon_sym_demote] = anon_sym_demote,
  [anon_sym_demote_to_helper] = anon_sym_demote_to_helper,
  [anon_sym_do] = anon_sym_do,
  [anon_sym_dynamic_cast] = anon_sym_dynamic_cast,
  [anon_sym_enum] = anon_sym_enum,
  [anon_sym_explicit] = anon_sym_explicit,
  [anon_sym_export] = anon_sym_export,
  [anon_sym_extends] = anon_sym_extends,
  [anon_sym_extern] = anon_sym_extern,
  [anon_sym_external] = anon_sym_external,
  [anon_sym_fallthrough] = anon_sym_fallthrough,
  [anon_sym_filter] = anon_sym_filter,
  [anon_sym_final] = anon_sym_final,
  [anon_sym_finally] = anon_sym_finally,
  [anon_sym_friend] = anon_sym_friend,
  [anon_sym_from] = anon_sym_from,
  [anon_sym_fxgroup] = anon_sym_fxgroup,
  [anon_sym_get] = anon_sym_get,
  [anon_sym_goto] = anon_sym_goto,
  [anon_sym_groupshared] = anon_sym_groupshared,
  [anon_sym_handle] = anon_sym_handle,
  [anon_sym_highp] = anon_sym_highp,
  [anon_sym_impl] = anon_sym_impl,
  [anon_sym_implements] = anon_sym_implements,
  [anon_sym_import] = anon_sym_import,
  [anon_sym_inline] = anon_sym_inline,
  [anon_sym_inout] = anon_sym_inout,
  [anon_sym_instanceof] = anon_sym_instanceof,
  [anon_sym_interface] = anon_sym_interface,
  [anon_sym_layout] = anon_sym_layout,
  [anon_sym_lowp] = anon_sym_lowp,
  [anon_sym_macro] = anon_sym_macro,
  [anon_sym_macro_rules] = anon_sym_macro_rules,
  [anon_sym_match] = anon_sym_match,
  [anon_sym_mediump] = anon_sym_mediump,
  [anon_sym_meta] = anon_sym_meta,
  [anon_sym_mod] = anon_sym_mod,
  [anon_sym_module] = anon_sym_module,
  [anon_sym_move] = anon_sym_move,
  [anon_sym_mut] = anon_sym_mut,
  [anon_sym_mutable] = anon_sym_mutable,
  [anon_sym_namespace] = anon_sym_namespace,
  [anon_sym_new] = anon_sym_new,
  [anon_sym_nil] = anon_sym_nil,
  [anon_sym_noexcept] = anon_sym_noexcept,
  [anon_sym_noinline] = anon_sym_noinline,
  [anon_sym_nointerpolation] = anon_sym_nointerpolation,
  [anon_sym_noperspective] = anon_sym_noperspective,
  [anon_sym_null] = anon_sym_null,
  [anon_sym_nullptr] = anon_sym_nullptr,
  [anon_sym_of] = anon_sym_of,
  [anon_sym_operator] = anon_sym_operator,
  [anon_sym_package] = anon_sym_package,
  [anon_sym_packoffset] = anon_sym_packoffset,
  [anon_sym_partition] = anon_sym_partition,
  [anon_sym_pass] = anon_sym_pass,
  [anon_sym_patch] = anon_sym_patch,
  [anon_sym_pixelfragment] = anon_sym_pixelfragment,
  [anon_sym_precise] = anon_sym_precise,
  [anon_sym_precision] = anon_sym_precision,
  [anon_sym_premerge] = anon_sym_premerge,
  [anon_sym_priv] = anon_sym_priv,
  [anon_sym_protected] = anon_sym_protected,
  [anon_sym_pub] = anon_sym_pub,
  [anon_sym_public] = anon_sym_public,
  [anon_sym_readonly] = anon_sym_readonly,
  [anon_sym_ref] = anon_sym_ref,
  [anon_sym_regardless] = anon_sym_regardless,
  [anon_sym_register] = anon_sym_register,
  [anon_sym_reinterpret_cast] = anon_sym_reinterpret_cast,
  [anon_sym_requires] = anon_sym_requires,
  [anon_sym_resource] = anon_sym_resource,
  [anon_sym_restrict] = anon_sym_restrict,
  [anon_sym_self] = anon_sym_self,
  [anon_sym_set] = anon_sym_set,
  [anon_sym_shared] = anon_sym_shared,
  [anon_sym_signed] = anon_sym_signed,
  [anon_sym_sizeof] = anon_sym_sizeof,
  [anon_sym_smooth] = anon_sym_smooth,
  [anon_sym_snorm] = anon_sym_snorm,
  [anon_sym_static] = anon_sym_static,
  [anon_sym_static_assert] = anon_sym_static_assert,
  [anon_sym_static_cast] = anon_sym_static_cast,
  [anon_sym_std] = anon_sym_std,
  [anon_sym_subroutine] = anon_sym_subroutine,
  [anon_sym_super] = anon_sym_super,
  [anon_sym_target] = anon_sym_target,
  [anon_sym_template] = anon_sym_template,
  [anon_sym_this] = anon_sym_this,
  [anon_sym_thread_local] = anon_sym_thread_local,
  [anon_sym_throw] = anon_sym_throw,
  [anon_sym_trait] = anon_sym_trait,
  [anon_sym_try] = anon_sym_try,
  [anon_sym_type] = anon_sym_type,
  [anon_sym_typedef] = anon_sym_typedef,
  [anon_sym_typeid] = anon_sym_typeid,
  [anon_sym_typename] = anon_sym_typename,
  [anon_sym_typeof] = anon_sym_typeof,
  [anon_sym_union] = anon_sym_union,
  [anon_sym_unless] = anon_sym_unless,
  [anon_sym_unorm] = anon_sym_unorm,
  [anon_sym_unsafe] = anon_sym_unsafe,
  [anon_sym_unsized] = anon_sym_unsized,
  [anon_sym_use] = anon_sym_use,
  [anon_sym_using] = anon_sym_using,
  [anon_sym_varying] = anon_sym_varying,
  [anon_sym_virtual] = anon_sym_virtual,
  [anon_sym_volatile] = anon_sym_volatile,
  [anon_sym_wgsl] = anon_sym_wgsl,
  [anon_sym_where] = anon_sym_where,
  [anon_sym_with] = anon_sym_with,
  [anon_sym_writeonly] = anon_sym_writeonly,
  [anon_sym_yield] = anon_sym_yield,
  [anon_sym_SLASH_SLASH] = anon_sym_SLASH_SLASH,
  [aux_sym__comment_token1] = aux_sym__comment_token1,
  [sym__blankspace] = sym__blankspace,
  [sym__block_comment] = sym__block_comment,
  [sym_translation_unit] = sym_translation_unit,
  [sym_global_directive] = sym_global_directive,
  [sym_bool_literal] = sym_bool_literal,
  [sym_int_literal] = sym_int_literal,
  [sym_decimal_int_literal] = sym_decimal_int_literal,
  [sym_float_literal] = sym_float_literal,
  [sym_decimal_float_literal] = sym_decimal_float_literal,
  [sym_hex_float_literal] = sym_hex_float_literal,
  [sym_literal] = sym_literal,
  [sym_member_ident] = sym_member_ident,
  [sym_attribute] = sym_attribute,
  [sym_attrib_end] = sym_attrib_end,
  [sym_array_type_specifier] = sym_array_type_specifier,
  [sym_element_count_expression] = sym_element_count_expression,
  [sym_struct_decl] = sym_struct_decl,
  [sym_struct_body_decl] = sym_struct_body_decl,
  [sym_struct_member] = sym_struct_member,
  [sym_texture_and_sampler_types] = sym_texture_and_sampler_types,
  [sym_sampler_type] = sym_sampler_type,
  [sym_sampled_texture_type] = sym_sampled_texture_type,
  [sym_storage_texture_type] = sym_storage_texture_type,
  [sym_depth_texture_type] = sym_depth_texture_type,
  [sym_type_alias_decl] = sym_type_alias_decl,
  [sym_type_specifier] = sym_type_specifier,
  [sym_type_specifier_without_ident] = sym_type_specifier_without_ident,
  [sym_vec_prefix] = sym_vec_prefix,
  [sym_mat_prefix] = sym_mat_prefix,
  [sym_variable_statement] = sym_variable_statement,
  [sym_variable_decl] = sym_variable_decl,
  [sym_optionally_typed_ident] = sym_optionally_typed_ident,
  [sym_variable_qualifier] = sym_variable_qualifier,
  [sym_global_variable_decl] = sym_global_variable_decl,
  [sym_global_constant_decl] = sym_global_constant_decl,
  [sym_primary_expression] = sym_primary_expression,
  [sym_call_expression] = sym_call_expression,
  [sym_call_phrase] = sym_call_phrase,
  [sym_callable] = sym_callable,
  [sym_paren_expression] = sym_paren_expression,
  [sym_argument_expression_list] = sym_argument_expression_list,
  [sym_expression_comma_list] = sym_expression_comma_list,
  [sym_component_or_swizzle_specifier] = sym_component_or_swizzle_specifier,
  [sym_unary_expression] = sym_unary_expression,
  [sym_singular_expression] = sym_singular_expression,
  [sym_lhs_expression] = sym_lhs_expression,
  [sym_core_lhs_expression] = sym_core_lhs_expression,
  [sym_multiplicative_expression] = sym_multiplicative_expression,
  [sym_multiplicative_operator] = sym_multiplicative_operator,
  [sym_additive_expression] = sym_additive_expression,
  [sym_additive_operator] = sym_additive_operator,
  [sym_shift_expression] = sym_shift_expression,
  [sym_relational_expression] = sym_relational_expression,
  [sym_short_circuit_and_expression] = sym_short_circuit_and_expression,
  [sym_short_circuit_or_expression] = sym_short_circuit_or_expression,
  [sym_binary_or_expression] = sym_binary_or_expression,
  [sym_binary_and_expression] = sym_binary_and_expression,
  [sym_binary_xor_expression] = sym_binary_xor_expression,
  [sym_bitwise_expression] = sym_bitwise_expression,
  [sym_expression] = sym_expression,
  [sym_compound_statement] = sym_compound_statement,
  [sym_assignment_statement] = sym_assignment_statement,
  [sym_compound_assignment_operator] = sym_compound_assignment_operator,
  [sym_increment_statement] = sym_increment_statement,
  [sym_decrement_statement] = sym_decrement_statement,
  [sym_if_statement] = sym_if_statement,
  [sym_if_clause] = sym_if_clause,
  [sym_else_if_clause] = sym_else_if_clause,
  [sym_else_clause] = sym_else_clause,
  [sym_switch_statement] = sym_switch_statement,
  [sym_switch_body] = sym_switch_body,
  [sym_case_clause] = sym_case_clause,
  [sym_default_alone_clause] = sym_default_alone_clause,
  [sym_case_selectors] = sym_case_selectors,
  [sym_case_selector] = sym_case_selector,
  [sym_loop_statement] = sym_loop_statement,
  [sym_for_statement] = sym_for_statement,
  [sym_for_header] = sym_for_header,
  [sym_for_init] = sym_for_init,
  [sym_for_update] = sym_for_update,
  [sym_while_statement] = sym_while_statement,
  [sym_break_statement] = sym_break_statement,
  [sym_break_if_statement] = sym_break_if_statement,
  [sym_continuing_statement] = sym_continuing_statement,
  [sym_continuing_compound_statement] = sym_continuing_compound_statement,
  [sym_return_statement] = sym_return_statement,
  [sym_func_call_statement] = sym_func_call_statement,
  [sym_const_assert_statement] = sym_const_assert_statement,
  [sym_statement] = sym_statement,
  [sym_variable_updating_statement] = sym_variable_updating_statement,
  [sym_function_decl] = sym_function_decl,
  [sym_function_header] = sym_function_header,
  [sym_param_list] = sym_param_list,
  [sym_param] = sym_param,
  [sym_enable_directive] = sym_enable_directive,
  [sym_interpolation_type_name] = sym_interpolation_type_name,
  [sym_interpolation_sample_name] = sym_interpolation_sample_name,
  [sym_builtin_value_name] = sym_builtin_value_name,
  [sym_access_mode] = sym_access_mode,
  [sym_address_space] = sym_address_space,
  [sym_texel_format] = sym_texel_format,
  [sym_extension_name] = sym_extension_name,
  [sym_swizzle_name] = sym_swizzle_name,
  [sym_ident] = sym_ident,
  [sym__comment] = sym__comment,
  [aux_sym_translation_unit_repeat1] = aux_sym_translation_unit_repeat1,
  [aux_sym_translation_unit_repeat2] = aux_sym_translation_unit_repeat2,
  [aux_sym_struct_body_decl_repeat1] = aux_sym_struct_body_decl_repeat1,
  [aux_sym_struct_member_repeat1] = aux_sym_struct_member_repeat1,
  [aux_sym_expression_comma_list_repeat1] = aux_sym_expression_comma_list_repeat1,
  [aux_sym_compound_statement_repeat1] = aux_sym_compound_statement_repeat1,
  [aux_sym_if_statement_repeat1] = aux_sym_if_statement_repeat1,
  [aux_sym_switch_statement_repeat1] = aux_sym_switch_statement_repeat1,
  [aux_sym_case_selectors_repeat1] = aux_sym_case_selectors_repeat1,
  [aux_sym_param_list_repeat1] = aux_sym_param_list_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_ident_pattern_token] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_true] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_false] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_decimal_int_literal_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_decimal_int_literal_token2] = {
    .visible = false,
    .named = false,
  },
  [sym_hex_int_literal] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_decimal_float_literal_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_decimal_float_literal_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_decimal_float_literal_token3] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_decimal_float_literal_token4] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_decimal_float_literal_token5] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_hex_float_literal_token1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_hex_float_literal_token2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_hex_float_literal_token3] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_align] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_binding] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_builtin] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_const] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_group] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_id] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_interpolate] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_invariant] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_location] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_size] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_workgroup_size] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_vertex] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_fragment] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_compute] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_array] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_struct] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_sampler] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_sampler_comparison] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_texture_1d] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_texture_2d] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_texture_2d_array] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_texture_3d] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_texture_cube] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_texture_cube_array] = {
    .visible = true,
    .named = false,
  },
  [sym_multisampled_texture_type] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_texture_storage_1d] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_texture_storage_2d] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_texture_storage_2d_array] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_texture_storage_3d] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_texture_depth_2d] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_texture_depth_2d_array] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_texture_depth_cube] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_texture_depth_cube_array] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_texture_depth_multisampled_2d] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_alias] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_bool] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_f32] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_f16] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_i32] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_u32] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ptr] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_atomic] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_vec2] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_vec3] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_vec4] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mat2x2] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mat2x3] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mat2x4] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mat3x2] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mat3x3] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mat3x4] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mat4x2] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mat4x3] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mat4x4] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_let] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_var] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_override] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_bitcast] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_TILDE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP_AMP] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CARET] = {
    .visible = true,
    .named = false,
  },
  [anon_sym__] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PERCENT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CARET_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT_GT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_LT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_if] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_else] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_switch] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_case] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_default] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_loop] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_for] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_while] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_break] = {
    .visible = true,
    .named = false,
  },
  [sym_continue_statement] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_continuing] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_return] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_const_assert] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_discard] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_fn] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_enable] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_perspective] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_linear] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_flat] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_center] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_centroid] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_sample] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_vertex_index] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_instance_index] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_position] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_front_facing] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_frag_depth] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_local_invocation_id] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_local_invocation_index] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_global_invocation_id] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_workgroup_id] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_num_workgroups] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_sample_index] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_sample_mask] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_read] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_write] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_read_write] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_function] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_private] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_workgroup] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_uniform] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_storage] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rgba8unorm] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rgba8snorm] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rgba8uint] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rgba8sint] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rgba16uint] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rgba16sint] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rgba16float] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_r32uint] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_r32sint] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_r32float] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rg32uint] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rg32sint] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rg32float] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rgba32uint] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rgba32sint] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_rgba32float] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_bgra8unorm] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_LBRACKrgba_RBRACK_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_LBRACKxyzw_RBRACK_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CompileShader] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ComputeShader] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DomainShader] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GeometryShader] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Hullshader] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_NULL] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Self] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_abstract] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_active] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_alignas] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_alignof] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_as] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_asm] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_asm_fragment] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_async] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_attribute] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_auto] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_await] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_become] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_binding_array] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_cast] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_catch] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_class] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_co_await] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_co_return] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_co_yield] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_coherent] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_column_major] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_common] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_compile] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_compile_fragment] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_concept] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_const_cast] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_consteval] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_constexpr] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_constinit] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_crate] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_debugger] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_decltype] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_delete] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_demote] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_demote_to_helper] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_do] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_dynamic_cast] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_enum] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_explicit] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_export] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_extends] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_extern] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_external] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_fallthrough] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_filter] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_final] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_finally] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_friend] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_from] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_fxgroup] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_get] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_goto] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_groupshared] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_handle] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_highp] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_impl] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_implements] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_import] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_inline] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_inout] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_instanceof] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_interface] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_layout] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_lowp] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_macro] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_macro_rules] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_match] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mediump] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_meta] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mod] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_module] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_move] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mut] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mutable] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_namespace] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_new] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_nil] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_noexcept] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_noinline] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_nointerpolation] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_noperspective] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_null] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_nullptr] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_of] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_operator] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_package] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_packoffset] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_partition] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_pass] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_patch] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_pixelfragment] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_precise] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_precision] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_premerge] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_priv] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_protected] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_pub] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_public] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_readonly] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ref] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_regardless] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_register] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_reinterpret_cast] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_requires] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_resource] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_restrict] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_self] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_set] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_shared] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_signed] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_sizeof] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_smooth] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_snorm] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_static] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_static_assert] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_static_cast] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_std] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_subroutine] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_super] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_target] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_template] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_this] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_thread_local] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_throw] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_trait] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_try] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_type] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_typedef] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_typeid] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_typename] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_typeof] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_union] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_unless] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_unorm] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_unsafe] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_unsized] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_use] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_using] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_varying] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_virtual] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_volatile] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_wgsl] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_where] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_with] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_writeonly] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_yield] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH_SLASH] = {
    .visible = true,
    .named = false,
  },
  [aux_sym__comment_token1] = {
    .visible = false,
    .named = false,
  },
  [sym__blankspace] = {
    .visible = false,
    .named = true,
  },
  [sym__block_comment] = {
    .visible = false,
    .named = true,
  },
  [sym_translation_unit] = {
    .visible = true,
    .named = true,
  },
  [sym_global_directive] = {
    .visible = true,
    .named = true,
  },
  [sym_bool_literal] = {
    .visible = true,
    .named = true,
  },
  [sym_int_literal] = {
    .visible = true,
    .named = true,
  },
  [sym_decimal_int_literal] = {
    .visible = true,
    .named = true,
  },
  [sym_float_literal] = {
    .visible = true,
    .named = true,
  },
  [sym_decimal_float_literal] = {
    .visible = true,
    .named = true,
  },
  [sym_hex_float_literal] = {
    .visible = true,
    .named = true,
  },
  [sym_literal] = {
    .visible = true,
    .named = true,
  },
  [sym_member_ident] = {
    .visible = true,
    .named = true,
  },
  [sym_attribute] = {
    .visible = true,
    .named = true,
  },
  [sym_attrib_end] = {
    .visible = true,
    .named = true,
  },
  [sym_array_type_specifier] = {
    .visible = true,
    .named = true,
  },
  [sym_element_count_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_struct_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_struct_body_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_struct_member] = {
    .visible = true,
    .named = true,
  },
  [sym_texture_and_sampler_types] = {
    .visible = true,
    .named = true,
  },
  [sym_sampler_type] = {
    .visible = true,
    .named = true,
  },
  [sym_sampled_texture_type] = {
    .visible = true,
    .named = true,
  },
  [sym_storage_texture_type] = {
    .visible = true,
    .named = true,
  },
  [sym_depth_texture_type] = {
    .visible = true,
    .named = true,
  },
  [sym_type_alias_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_type_specifier] = {
    .visible = true,
    .named = true,
  },
  [sym_type_specifier_without_ident] = {
    .visible = true,
    .named = true,
  },
  [sym_vec_prefix] = {
    .visible = true,
    .named = true,
  },
  [sym_mat_prefix] = {
    .visible = true,
    .named = true,
  },
  [sym_variable_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_variable_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_optionally_typed_ident] = {
    .visible = true,
    .named = true,
  },
  [sym_variable_qualifier] = {
    .visible = true,
    .named = true,
  },
  [sym_global_variable_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_global_constant_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_primary_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_call_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_call_phrase] = {
    .visible = true,
    .named = true,
  },
  [sym_callable] = {
    .visible = true,
    .named = true,
  },
  [sym_paren_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_argument_expression_list] = {
    .visible = true,
    .named = true,
  },
  [sym_expression_comma_list] = {
    .visible = true,
    .named = true,
  },
  [sym_component_or_swizzle_specifier] = {
    .visible = true,
    .named = true,
  },
  [sym_unary_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_singular_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_lhs_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_core_lhs_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_multiplicative_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_multiplicative_operator] = {
    .visible = true,
    .named = true,
  },
  [sym_additive_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_additive_operator] = {
    .visible = true,
    .named = true,
  },
  [sym_shift_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_relational_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_short_circuit_and_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_short_circuit_or_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_binary_or_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_binary_and_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_binary_xor_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_bitwise_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_expression] = {
    .visible = true,
    .named = true,
  },
  [sym_compound_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_assignment_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_compound_assignment_operator] = {
    .visible = true,
    .named = true,
  },
  [sym_increment_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_decrement_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_if_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_if_clause] = {
    .visible = true,
    .named = true,
  },
  [sym_else_if_clause] = {
    .visible = true,
    .named = true,
  },
  [sym_else_clause] = {
    .visible = true,
    .named = true,
  },
  [sym_switch_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_switch_body] = {
    .visible = true,
    .named = true,
  },
  [sym_case_clause] = {
    .visible = true,
    .named = true,
  },
  [sym_default_alone_clause] = {
    .visible = true,
    .named = true,
  },
  [sym_case_selectors] = {
    .visible = true,
    .named = true,
  },
  [sym_case_selector] = {
    .visible = true,
    .named = true,
  },
  [sym_loop_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_for_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_for_header] = {
    .visible = true,
    .named = true,
  },
  [sym_for_init] = {
    .visible = true,
    .named = true,
  },
  [sym_for_update] = {
    .visible = true,
    .named = true,
  },
  [sym_while_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_break_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_break_if_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_continuing_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_continuing_compound_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_return_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_func_call_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_const_assert_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_variable_updating_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_function_decl] = {
    .visible = true,
    .named = true,
  },
  [sym_function_header] = {
    .visible = true,
    .named = true,
  },
  [sym_param_list] = {
    .visible = true,
    .named = true,
  },
  [sym_param] = {
    .visible = true,
    .named = true,
  },
  [sym_enable_directive] = {
    .visible = true,
    .named = true,
  },
  [sym_interpolation_type_name] = {
    .visible = true,
    .named = true,
  },
  [sym_interpolation_sample_name] = {
    .visible = true,
    .named = true,
  },
  [sym_builtin_value_name] = {
    .visible = true,
    .named = true,
  },
  [sym_access_mode] = {
    .visible = true,
    .named = true,
  },
  [sym_address_space] = {
    .visible = true,
    .named = true,
  },
  [sym_texel_format] = {
    .visible = true,
    .named = true,
  },
  [sym_extension_name] = {
    .visible = true,
    .named = true,
  },
  [sym_swizzle_name] = {
    .visible = true,
    .named = true,
  },
  [sym_ident] = {
    .visible = true,
    .named = true,
  },
  [sym__comment] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_translation_unit_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_translation_unit_repeat2] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_struct_body_decl_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_struct_member_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_expression_comma_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_compound_statement_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_if_statement_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_switch_statement_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_case_selectors_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_param_list_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 5,
  [8] = 5,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 17,
  [24] = 24,
  [25] = 16,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 16,
  [33] = 33,
  [34] = 34,
  [35] = 16,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 17,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 47,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 50,
  [52] = 50,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 55,
  [57] = 53,
  [58] = 58,
  [59] = 59,
  [60] = 54,
  [61] = 59,
  [62] = 62,
  [63] = 62,
  [64] = 53,
  [65] = 53,
  [66] = 58,
  [67] = 55,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 77,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 83,
  [84] = 84,
  [85] = 85,
  [86] = 86,
  [87] = 87,
  [88] = 88,
  [89] = 89,
  [90] = 90,
  [91] = 91,
  [92] = 92,
  [93] = 93,
  [94] = 94,
  [95] = 95,
  [96] = 96,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 102,
  [104] = 104,
  [105] = 105,
  [106] = 106,
  [107] = 107,
  [108] = 108,
  [109] = 109,
  [110] = 102,
  [111] = 111,
  [112] = 112,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 116,
  [117] = 117,
  [118] = 118,
  [119] = 119,
  [120] = 120,
  [121] = 121,
  [122] = 122,
  [123] = 123,
  [124] = 124,
  [125] = 125,
  [126] = 126,
  [127] = 127,
  [128] = 128,
  [129] = 129,
  [130] = 130,
  [131] = 131,
  [132] = 132,
  [133] = 133,
  [134] = 134,
  [135] = 135,
  [136] = 136,
  [137] = 137,
  [138] = 138,
  [139] = 139,
  [140] = 140,
  [141] = 141,
  [142] = 142,
  [143] = 143,
  [144] = 144,
  [145] = 145,
  [146] = 146,
  [147] = 147,
  [148] = 124,
  [149] = 123,
  [150] = 120,
  [151] = 120,
  [152] = 124,
  [153] = 123,
  [154] = 122,
  [155] = 125,
  [156] = 128,
  [157] = 121,
  [158] = 126,
  [159] = 127,
  [160] = 160,
  [161] = 136,
  [162] = 162,
  [163] = 132,
  [164] = 133,
  [165] = 135,
  [166] = 166,
  [167] = 140,
  [168] = 139,
  [169] = 138,
  [170] = 137,
  [171] = 134,
  [172] = 130,
  [173] = 173,
  [174] = 131,
  [175] = 129,
  [176] = 128,
  [177] = 125,
  [178] = 145,
  [179] = 179,
  [180] = 180,
  [181] = 142,
  [182] = 145,
  [183] = 143,
  [184] = 142,
  [185] = 147,
  [186] = 186,
  [187] = 146,
  [188] = 188,
  [189] = 189,
  [190] = 122,
  [191] = 133,
  [192] = 138,
  [193] = 130,
  [194] = 134,
  [195] = 137,
  [196] = 132,
  [197] = 131,
  [198] = 126,
  [199] = 140,
  [200] = 129,
  [201] = 136,
  [202] = 135,
  [203] = 139,
  [204] = 127,
  [205] = 205,
  [206] = 206,
  [207] = 207,
  [208] = 208,
  [209] = 209,
  [210] = 147,
  [211] = 143,
  [212] = 146,
  [213] = 173,
  [214] = 214,
  [215] = 215,
  [216] = 216,
  [217] = 180,
  [218] = 218,
  [219] = 219,
  [220] = 220,
  [221] = 179,
  [222] = 222,
  [223] = 120,
  [224] = 123,
  [225] = 121,
  [226] = 124,
  [227] = 227,
  [228] = 228,
  [229] = 128,
  [230] = 125,
  [231] = 231,
  [232] = 232,
  [233] = 231,
  [234] = 205,
  [235] = 231,
  [236] = 231,
  [237] = 237,
  [238] = 238,
  [239] = 239,
  [240] = 240,
  [241] = 241,
  [242] = 242,
  [243] = 243,
  [244] = 244,
  [245] = 121,
  [246] = 142,
  [247] = 145,
  [248] = 248,
  [249] = 249,
  [250] = 250,
  [251] = 251,
  [252] = 252,
  [253] = 253,
  [254] = 254,
  [255] = 255,
  [256] = 256,
  [257] = 257,
  [258] = 258,
  [259] = 259,
  [260] = 260,
  [261] = 261,
  [262] = 262,
  [263] = 263,
  [264] = 264,
  [265] = 265,
  [266] = 266,
  [267] = 267,
  [268] = 268,
  [269] = 269,
  [270] = 270,
  [271] = 271,
  [272] = 272,
  [273] = 273,
  [274] = 179,
  [275] = 180,
  [276] = 276,
  [277] = 277,
  [278] = 278,
  [279] = 279,
  [280] = 273,
  [281] = 281,
  [282] = 282,
  [283] = 283,
  [284] = 284,
  [285] = 186,
  [286] = 188,
  [287] = 287,
  [288] = 288,
  [289] = 289,
  [290] = 290,
  [291] = 291,
  [292] = 292,
  [293] = 293,
  [294] = 294,
  [295] = 295,
  [296] = 296,
  [297] = 297,
  [298] = 206,
  [299] = 299,
  [300] = 300,
  [301] = 301,
  [302] = 302,
  [303] = 303,
  [304] = 304,
  [305] = 305,
  [306] = 306,
  [307] = 307,
  [308] = 308,
  [309] = 309,
  [310] = 310,
  [311] = 311,
  [312] = 312,
  [313] = 313,
  [314] = 314,
  [315] = 315,
  [316] = 316,
  [317] = 317,
  [318] = 318,
  [319] = 319,
  [320] = 320,
  [321] = 321,
  [322] = 322,
  [323] = 323,
  [324] = 324,
  [325] = 325,
  [326] = 326,
  [327] = 327,
  [328] = 328,
  [329] = 329,
  [330] = 330,
  [331] = 331,
  [332] = 332,
  [333] = 333,
  [334] = 334,
  [335] = 335,
  [336] = 336,
  [337] = 337,
  [338] = 338,
  [339] = 339,
  [340] = 340,
  [341] = 340,
  [342] = 342,
  [343] = 343,
  [344] = 344,
  [345] = 345,
  [346] = 346,
  [347] = 347,
  [348] = 348,
  [349] = 349,
  [350] = 350,
  [351] = 351,
  [352] = 352,
  [353] = 353,
  [354] = 354,
  [355] = 355,
  [356] = 356,
  [357] = 357,
  [358] = 358,
  [359] = 359,
  [360] = 360,
  [361] = 361,
  [362] = 362,
  [363] = 363,
  [364] = 364,
  [365] = 365,
  [366] = 366,
  [367] = 367,
  [368] = 368,
  [369] = 369,
  [370] = 340,
  [371] = 371,
  [372] = 335,
  [373] = 373,
  [374] = 374,
  [375] = 375,
  [376] = 376,
  [377] = 377,
  [378] = 378,
  [379] = 379,
  [380] = 380,
  [381] = 381,
  [382] = 382,
  [383] = 383,
  [384] = 335,
  [385] = 385,
  [386] = 386,
  [387] = 387,
  [388] = 388,
  [389] = 389,
  [390] = 390,
  [391] = 391,
  [392] = 392,
  [393] = 393,
  [394] = 394,
  [395] = 395,
  [396] = 396,
  [397] = 397,
  [398] = 398,
  [399] = 399,
  [400] = 400,
  [401] = 401,
  [402] = 402,
  [403] = 403,
  [404] = 404,
  [405] = 405,
  [406] = 406,
  [407] = 407,
  [408] = 408,
  [409] = 409,
  [410] = 410,
  [411] = 411,
  [412] = 412,
  [413] = 413,
  [414] = 414,
  [415] = 415,
  [416] = 416,
  [417] = 417,
  [418] = 418,
  [419] = 419,
  [420] = 420,
  [421] = 421,
  [422] = 422,
  [423] = 423,
  [424] = 424,
  [425] = 425,
  [426] = 426,
  [427] = 427,
  [428] = 428,
  [429] = 429,
  [430] = 430,
  [431] = 431,
  [432] = 432,
  [433] = 433,
  [434] = 434,
  [435] = 435,
  [436] = 436,
  [437] = 437,
  [438] = 438,
  [439] = 439,
  [440] = 440,
  [441] = 441,
  [442] = 442,
  [443] = 443,
  [444] = 444,
  [445] = 445,
  [446] = 446,
  [447] = 447,
  [448] = 448,
  [449] = 449,
  [450] = 450,
  [451] = 451,
  [452] = 452,
  [453] = 453,
  [454] = 454,
  [455] = 455,
  [456] = 456,
  [457] = 457,
  [458] = 428,
  [459] = 426,
  [460] = 409,
  [461] = 461,
  [462] = 462,
  [463] = 463,
  [464] = 464,
  [465] = 437,
  [466] = 466,
  [467] = 432,
  [468] = 436,
  [469] = 469,
  [470] = 470,
  [471] = 471,
  [472] = 409,
  [473] = 473,
  [474] = 474,
  [475] = 432,
  [476] = 436,
  [477] = 477,
  [478] = 478,
  [479] = 432,
  [480] = 480,
  [481] = 427,
  [482] = 482,
  [483] = 418,
  [484] = 484,
  [485] = 485,
  [486] = 486,
  [487] = 418,
  [488] = 488,
  [489] = 489,
  [490] = 490,
  [491] = 437,
  [492] = 492,
};

static inline bool sym_ident_pattern_token_character_set_1(int32_t c) {
  return (c < 43514
    ? (c < 4193
      ? (c < 2707
        ? (c < 1994
          ? (c < 931
            ? (c < 748
              ? (c < 192
                ? (c < 170
                  ? (c < 'a'
                    ? (c >= 'A' && c <= 'Z')
                    : c <= 'z')
                  : (c <= 170 || (c < 186
                    ? c == 181
                    : c <= 186)))
                : (c <= 214 || (c < 710
                  ? (c < 248
                    ? (c >= 216 && c <= 246)
                    : c <= 705)
                  : (c <= 721 || (c >= 736 && c <= 740)))))
              : (c <= 748 || (c < 895
                ? (c < 886
                  ? (c < 880
                    ? c == 750
                    : c <= 884)
                  : (c <= 887 || (c >= 891 && c <= 893)))
                : (c <= 895 || (c < 908
                  ? (c < 904
                    ? c == 902
                    : c <= 906)
                  : (c <= 908 || (c >= 910 && c <= 929)))))))
            : (c <= 1013 || (c < 1649
              ? (c < 1376
                ? (c < 1329
                  ? (c < 1162
                    ? (c >= 1015 && c <= 1153)
                    : c <= 1327)
                  : (c <= 1366 || c == 1369))
                : (c <= 1416 || (c < 1568
                  ? (c < 1519
                    ? (c >= 1488 && c <= 1514)
                    : c <= 1522)
                  : (c <= 1610 || (c >= 1646 && c <= 1647)))))
              : (c <= 1747 || (c < 1791
                ? (c < 1774
                  ? (c < 1765
                    ? c == 1749
                    : c <= 1766)
                  : (c <= 1775 || (c >= 1786 && c <= 1788)))
                : (c <= 1791 || (c < 1869
                  ? (c < 1810
                    ? c == 1808
                    : c <= 1839)
                  : (c <= 1957 || c == 1969))))))))
          : (c <= 2026 || (c < 2482
            ? (c < 2208
              ? (c < 2088
                ? (c < 2048
                  ? (c < 2042
                    ? (c >= 2036 && c <= 2037)
                    : c <= 2042)
                  : (c <= 2069 || (c < 2084
                    ? c == 2074
                    : c <= 2084)))
                : (c <= 2088 || (c < 2160
                  ? (c < 2144
                    ? (c >= 2112 && c <= 2136)
                    : c <= 2154)
                  : (c <= 2183 || (c >= 2185 && c <= 2190)))))
              : (c <= 2249 || (c < 2417
                ? (c < 2384
                  ? (c < 2365
                    ? (c >= 2308 && c <= 2361)
                    : c <= 2365)
                  : (c <= 2384 || (c >= 2392 && c <= 2401)))
                : (c <= 2432 || (c < 2451
                  ? (c < 2447
                    ? (c >= 2437 && c <= 2444)
                    : c <= 2448)
                  : (c <= 2472 || (c >= 2474 && c <= 2480)))))))
            : (c <= 2482 || (c < 2579
              ? (c < 2527
                ? (c < 2510
                  ? (c < 2493
                    ? (c >= 2486 && c <= 2489)
                    : c <= 2493)
                  : (c <= 2510 || (c >= 2524 && c <= 2525)))
                : (c <= 2529 || (c < 2565
                  ? (c < 2556
                    ? (c >= 2544 && c <= 2545)
                    : c <= 2556)
                  : (c <= 2570 || (c >= 2575 && c <= 2576)))))
              : (c <= 2600 || (c < 2649
                ? (c < 2613
                  ? (c < 2610
                    ? (c >= 2602 && c <= 2608)
                    : c <= 2611)
                  : (c <= 2614 || (c >= 2616 && c <= 2617)))
                : (c <= 2652 || (c < 2693
                  ? (c < 2674
                    ? c == 2654
                    : c <= 2676)
                  : (c <= 2701 || (c >= 2703 && c <= 2705)))))))))))
        : (c <= 2728 || (c < 3242
          ? (c < 2962
            ? (c < 2858
              ? (c < 2784
                ? (c < 2741
                  ? (c < 2738
                    ? (c >= 2730 && c <= 2736)
                    : c <= 2739)
                  : (c <= 2745 || (c < 2768
                    ? c == 2749
                    : c <= 2768)))
                : (c <= 2785 || (c < 2831
                  ? (c < 2821
                    ? c == 2809
                    : c <= 2828)
                  : (c <= 2832 || (c >= 2835 && c <= 2856)))))
              : (c <= 2864 || (c < 2911
                ? (c < 2877
                  ? (c < 2869
                    ? (c >= 2866 && c <= 2867)
                    : c <= 2873)
                  : (c <= 2877 || (c >= 2908 && c <= 2909)))
                : (c <= 2913 || (c < 2949
                  ? (c < 2947
                    ? c == 2929
                    : c <= 2947)
                  : (c <= 2954 || (c >= 2958 && c <= 2960)))))))
            : (c <= 2965 || (c < 3090
              ? (c < 2984
                ? (c < 2974
                  ? (c < 2972
                    ? (c >= 2969 && c <= 2970)
                    : c <= 2972)
                  : (c <= 2975 || (c >= 2979 && c <= 2980)))
                : (c <= 2986 || (c < 3077
                  ? (c < 3024
                    ? (c >= 2990 && c <= 3001)
                    : c <= 3024)
                  : (c <= 3084 || (c >= 3086 && c <= 3088)))))
              : (c <= 3112 || (c < 3168
                ? (c < 3160
                  ? (c < 3133
                    ? (c >= 3114 && c <= 3129)
                    : c <= 3133)
                  : (c <= 3162 || c == 3165))
                : (c <= 3169 || (c < 3214
                  ? (c < 3205
                    ? c == 3200
                    : c <= 3212)
                  : (c <= 3216 || (c >= 3218 && c <= 3240)))))))))
          : (c <= 3251 || (c < 3648
            ? (c < 3412
              ? (c < 3332
                ? (c < 3293
                  ? (c < 3261
                    ? (c >= 3253 && c <= 3257)
                    : c <= 3261)
                  : (c <= 3294 || (c < 3313
                    ? (c >= 3296 && c <= 3297)
                    : c <= 3314)))
                : (c <= 3340 || (c < 3389
                  ? (c < 3346
                    ? (c >= 3342 && c <= 3344)
                    : c <= 3386)
                  : (c <= 3389 || c == 3406))))
              : (c <= 3414 || (c < 3507
                ? (c < 3461
                  ? (c < 3450
                    ? (c >= 3423 && c <= 3425)
                    : c <= 3455)
                  : (c <= 3478 || (c >= 3482 && c <= 3505)))
                : (c <= 3515 || (c < 3585
                  ? (c < 3520
                    ? c == 3517
                    : c <= 3526)
                  : (c <= 3632 || c == 3634))))))
            : (c <= 3654 || (c < 3782
              ? (c < 3749
                ? (c < 3718
                  ? (c < 3716
                    ? (c >= 3713 && c <= 3714)
                    : c <= 3716)
                  : (c <= 3722 || (c >= 3724 && c <= 3747)))
                : (c <= 3749 || (c < 3773
                  ? (c < 3762
                    ? (c >= 3751 && c <= 3760)
                    : c <= 3762)
                  : (c <= 3773 || (c >= 3776 && c <= 3780)))))
              : (c <= 3782 || (c < 3976
                ? (c < 3904
                  ? (c < 3840
                    ? (c >= 3804 && c <= 3807)
                    : c <= 3840)
                  : (c <= 3911 || (c >= 3913 && c <= 3948)))
                : (c <= 3980 || (c < 4176
                  ? (c < 4159
                    ? (c >= 4096 && c <= 4138)
                    : c <= 4159)
                  : (c <= 4181 || (c >= 4186 && c <= 4189)))))))))))))
      : (c <= 4193 || (c < 8134
        ? (c < 6176
          ? (c < 4808
            ? (c < 4688
              ? (c < 4295
                ? (c < 4213
                  ? (c < 4206
                    ? (c >= 4197 && c <= 4198)
                    : c <= 4208)
                  : (c <= 4225 || (c < 4256
                    ? c == 4238
                    : c <= 4293)))
                : (c <= 4295 || (c < 4348
                  ? (c < 4304
                    ? c == 4301
                    : c <= 4346)
                  : (c <= 4680 || (c >= 4682 && c <= 4685)))))
              : (c <= 4694 || (c < 4752
                ? (c < 4704
                  ? (c < 4698
                    ? c == 4696
                    : c <= 4701)
                  : (c <= 4744 || (c >= 4746 && c <= 4749)))
                : (c <= 4784 || (c < 4800
                  ? (c < 4792
                    ? (c >= 4786 && c <= 4789)
                    : c <= 4798)
                  : (c <= 4800 || (c >= 4802 && c <= 4805)))))))
            : (c <= 4822 || (c < 5792
              ? (c < 5024
                ? (c < 4888
                  ? (c < 4882
                    ? (c >= 4824 && c <= 4880)
                    : c <= 4885)
                  : (c <= 4954 || (c >= 4992 && c <= 5007)))
                : (c <= 5109 || (c < 5743
                  ? (c < 5121
                    ? (c >= 5112 && c <= 5117)
                    : c <= 5740)
                  : (c <= 5759 || (c >= 5761 && c <= 5786)))))
              : (c <= 5866 || (c < 5984
                ? (c < 5919
                  ? (c < 5888
                    ? (c >= 5870 && c <= 5880)
                    : c <= 5905)
                  : (c <= 5937 || (c >= 5952 && c <= 5969)))
                : (c <= 5996 || (c < 6103
                  ? (c < 6016
                    ? (c >= 5998 && c <= 6000)
                    : c <= 6067)
                  : (c <= 6103 || c == 6108))))))))
          : (c <= 6264 || (c < 7312
            ? (c < 6823
              ? (c < 6512
                ? (c < 6320
                  ? (c < 6314
                    ? (c >= 6272 && c <= 6312)
                    : c <= 6314)
                  : (c <= 6389 || (c < 6480
                    ? (c >= 6400 && c <= 6430)
                    : c <= 6509)))
                : (c <= 6516 || (c < 6656
                  ? (c < 6576
                    ? (c >= 6528 && c <= 6571)
                    : c <= 6601)
                  : (c <= 6678 || (c >= 6688 && c <= 6740)))))
              : (c <= 6823 || (c < 7098
                ? (c < 7043
                  ? (c < 6981
                    ? (c >= 6917 && c <= 6963)
                    : c <= 6988)
                  : (c <= 7072 || (c >= 7086 && c <= 7087)))
                : (c <= 7141 || (c < 7258
                  ? (c < 7245
                    ? (c >= 7168 && c <= 7203)
                    : c <= 7247)
                  : (c <= 7293 || (c >= 7296 && c <= 7304)))))))
            : (c <= 7354 || (c < 8008
              ? (c < 7418
                ? (c < 7406
                  ? (c < 7401
                    ? (c >= 7357 && c <= 7359)
                    : c <= 7404)
                  : (c <= 7411 || (c >= 7413 && c <= 7414)))
                : (c <= 7418 || (c < 7960
                  ? (c < 7680
                    ? (c >= 7424 && c <= 7615)
                    : c <= 7957)
                  : (c <= 7965 || (c >= 7968 && c <= 8005)))))
              : (c <= 8013 || (c < 8031
                ? (c < 8027
                  ? (c < 8025
                    ? (c >= 8016 && c <= 8023)
                    : c <= 8025)
                  : (c <= 8027 || c == 8029))
                : (c <= 8061 || (c < 8126
                  ? (c < 8118
                    ? (c >= 8064 && c <= 8116)
                    : c <= 8124)
                  : (c <= 8126 || (c >= 8130 && c <= 8132)))))))))))
        : (c <= 8140 || (c < 12337
          ? (c < 8544
            ? (c < 8458
              ? (c < 8305
                ? (c < 8160
                  ? (c < 8150
                    ? (c >= 8144 && c <= 8147)
                    : c <= 8155)
                  : (c <= 8172 || (c < 8182
                    ? (c >= 8178 && c <= 8180)
                    : c <= 8188)))
                : (c <= 8305 || (c < 8450
                  ? (c < 8336
                    ? c == 8319
                    : c <= 8348)
                  : (c <= 8450 || c == 8455))))
              : (c <= 8467 || (c < 8488
                ? (c < 8484
                  ? (c < 8472
                    ? c == 8469
                    : c <= 8477)
                  : (c <= 8484 || c == 8486))
                : (c <= 8488 || (c < 8517
                  ? (c < 8508
                    ? (c >= 8490 && c <= 8505)
                    : c <= 8511)
                  : (c <= 8521 || c == 8526))))))
            : (c <= 8584 || (c < 11680
              ? (c < 11559
                ? (c < 11506
                  ? (c < 11499
                    ? (c >= 11264 && c <= 11492)
                    : c <= 11502)
                  : (c <= 11507 || (c >= 11520 && c <= 11557)))
                : (c <= 11559 || (c < 11631
                  ? (c < 11568
                    ? c == 11565
                    : c <= 11623)
                  : (c <= 11631 || (c >= 11648 && c <= 11670)))))
              : (c <= 11686 || (c < 11720
                ? (c < 11704
                  ? (c < 11696
                    ? (c >= 11688 && c <= 11694)
                    : c <= 11702)
                  : (c <= 11710 || (c >= 11712 && c <= 11718)))
                : (c <= 11726 || (c < 12293
                  ? (c < 11736
                    ? (c >= 11728 && c <= 11734)
                    : c <= 11742)
                  : (c <= 12295 || (c >= 12321 && c <= 12329)))))))))
          : (c <= 12341 || (c < 42891
            ? (c < 19968
              ? (c < 12549
                ? (c < 12445
                  ? (c < 12353
                    ? (c >= 12344 && c <= 12348)
                    : c <= 12438)
                  : (c <= 12447 || (c < 12540
                    ? (c >= 12449 && c <= 12538)
                    : c <= 12543)))
                : (c <= 12591 || (c < 12784
                  ? (c < 12704
                    ? (c >= 12593 && c <= 12686)
                    : c <= 12735)
                  : (c <= 12799 || (c >= 13312 && c <= 19903)))))
              : (c <= 42124 || (c < 42560
                ? (c < 42512
                  ? (c < 42240
                    ? (c >= 42192 && c <= 42237)
                    : c <= 42508)
                  : (c <= 42527 || (c >= 42538 && c <= 42539)))
                : (c <= 42606 || (c < 42775
                  ? (c < 42656
                    ? (c >= 42623 && c <= 42653)
                    : c <= 42735)
                  : (c <= 42783 || (c >= 42786 && c <= 42888)))))))
            : (c <= 42954 || (c < 43250
              ? (c < 43011
                ? (c < 42965
                  ? (c < 42963
                    ? (c >= 42960 && c <= 42961)
                    : c <= 42963)
                  : (c <= 42969 || (c >= 42994 && c <= 43009)))
                : (c <= 43013 || (c < 43072
                  ? (c < 43020
                    ? (c >= 43015 && c <= 43018)
                    : c <= 43042)
                  : (c <= 43123 || (c >= 43138 && c <= 43187)))))
              : (c <= 43255 || (c < 43360
                ? (c < 43274
                  ? (c < 43261
                    ? c == 43259
                    : c <= 43262)
                  : (c <= 43301 || (c >= 43312 && c <= 43334)))
                : (c <= 43388 || (c < 43488
                  ? (c < 43471
                    ? (c >= 43396 && c <= 43442)
                    : c <= 43471)
                  : (c <= 43492 || (c >= 43494 && c <= 43503)))))))))))))))
    : (c <= 43518 || (c < 70727
      ? (c < 66956
        ? (c < 64914
          ? (c < 43868
            ? (c < 43714
              ? (c < 43646
                ? (c < 43588
                  ? (c < 43584
                    ? (c >= 43520 && c <= 43560)
                    : c <= 43586)
                  : (c <= 43595 || (c < 43642
                    ? (c >= 43616 && c <= 43638)
                    : c <= 43642)))
                : (c <= 43695 || (c < 43705
                  ? (c < 43701
                    ? c == 43697
                    : c <= 43702)
                  : (c <= 43709 || c == 43712))))
              : (c <= 43714 || (c < 43785
                ? (c < 43762
                  ? (c < 43744
                    ? (c >= 43739 && c <= 43741)
                    : c <= 43754)
                  : (c <= 43764 || (c >= 43777 && c <= 43782)))
                : (c <= 43790 || (c < 43816
                  ? (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)
                  : (c <= 43822 || (c >= 43824 && c <= 43866)))))))
            : (c <= 43881 || (c < 64287
              ? (c < 63744
                ? (c < 55216
                  ? (c < 44032
                    ? (c >= 43888 && c <= 44002)
                    : c <= 55203)
                  : (c <= 55238 || (c >= 55243 && c <= 55291)))
                : (c <= 64109 || (c < 64275
                  ? (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)
                  : (c <= 64279 || c == 64285))))
              : (c <= 64296 || (c < 64323
                ? (c < 64318
                  ? (c < 64312
                    ? (c >= 64298 && c <= 64310)
                    : c <= 64316)
                  : (c <= 64318 || (c >= 64320 && c <= 64321)))
                : (c <= 64324 || (c < 64612
                  ? (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64605)
                  : (c <= 64829 || (c >= 64848 && c <= 64911)))))))))
          : (c <= 64967 || (c < 65599
            ? (c < 65382
              ? (c < 65147
                ? (c < 65139
                  ? (c < 65137
                    ? (c >= 65008 && c <= 65017)
                    : c <= 65137)
                  : (c <= 65139 || (c < 65145
                    ? c == 65143
                    : c <= 65145)))
                : (c <= 65147 || (c < 65313
                  ? (c < 65151
                    ? c == 65149
                    : c <= 65276)
                  : (c <= 65338 || (c >= 65345 && c <= 65370)))))
              : (c <= 65437 || (c < 65498
                ? (c < 65482
                  ? (c < 65474
                    ? (c >= 65440 && c <= 65470)
                    : c <= 65479)
                  : (c <= 65487 || (c >= 65490 && c <= 65495)))
                : (c <= 65500 || (c < 65576
                  ? (c < 65549
                    ? (c >= 65536 && c <= 65547)
                    : c <= 65574)
                  : (c <= 65594 || (c >= 65596 && c <= 65597)))))))
            : (c <= 65613 || (c < 66464
              ? (c < 66208
                ? (c < 65856
                  ? (c < 65664
                    ? (c >= 65616 && c <= 65629)
                    : c <= 65786)
                  : (c <= 65908 || (c >= 66176 && c <= 66204)))
                : (c <= 66256 || (c < 66384
                  ? (c < 66349
                    ? (c >= 66304 && c <= 66335)
                    : c <= 66378)
                  : (c <= 66421 || (c >= 66432 && c <= 66461)))))
              : (c <= 66499 || (c < 66776
                ? (c < 66560
                  ? (c < 66513
                    ? (c >= 66504 && c <= 66511)
                    : c <= 66517)
                  : (c <= 66717 || (c >= 66736 && c <= 66771)))
                : (c <= 66811 || (c < 66928
                  ? (c < 66864
                    ? (c >= 66816 && c <= 66855)
                    : c <= 66915)
                  : (c <= 66938 || (c >= 66940 && c <= 66954)))))))))))
        : (c <= 66962 || (c < 68864
          ? (c < 67828
            ? (c < 67506
              ? (c < 67072
                ? (c < 66979
                  ? (c < 66967
                    ? (c >= 66964 && c <= 66965)
                    : c <= 66977)
                  : (c <= 66993 || (c < 67003
                    ? (c >= 66995 && c <= 67001)
                    : c <= 67004)))
                : (c <= 67382 || (c < 67456
                  ? (c < 67424
                    ? (c >= 67392 && c <= 67413)
                    : c <= 67431)
                  : (c <= 67461 || (c >= 67463 && c <= 67504)))))
              : (c <= 67514 || (c < 67644
                ? (c < 67594
                  ? (c < 67592
                    ? (c >= 67584 && c <= 67589)
                    : c <= 67592)
                  : (c <= 67637 || (c >= 67639 && c <= 67640)))
                : (c <= 67644 || (c < 67712
                  ? (c < 67680
                    ? (c >= 67647 && c <= 67669)
                    : c <= 67702)
                  : (c <= 67742 || (c >= 67808 && c <= 67826)))))))
            : (c <= 67829 || (c < 68224
              ? (c < 68096
                ? (c < 67968
                  ? (c < 67872
                    ? (c >= 67840 && c <= 67861)
                    : c <= 67897)
                  : (c <= 68023 || (c >= 68030 && c <= 68031)))
                : (c <= 68096 || (c < 68121
                  ? (c < 68117
                    ? (c >= 68112 && c <= 68115)
                    : c <= 68119)
                  : (c <= 68149 || (c >= 68192 && c <= 68220)))))
              : (c <= 68252 || (c < 68448
                ? (c < 68352
                  ? (c < 68297
                    ? (c >= 68288 && c <= 68295)
                    : c <= 68324)
                  : (c <= 68405 || (c >= 68416 && c <= 68437)))
                : (c <= 68466 || (c < 68736
                  ? (c < 68608
                    ? (c >= 68480 && c <= 68497)
                    : c <= 68680)
                  : (c <= 68786 || (c >= 68800 && c <= 68850)))))))))
          : (c <= 68899 || (c < 70106
            ? (c < 69749
              ? (c < 69488
                ? (c < 69376
                  ? (c < 69296
                    ? (c >= 69248 && c <= 69289)
                    : c <= 69297)
                  : (c <= 69404 || (c < 69424
                    ? c == 69415
                    : c <= 69445)))
                : (c <= 69505 || (c < 69635
                  ? (c < 69600
                    ? (c >= 69552 && c <= 69572)
                    : c <= 69622)
                  : (c <= 69687 || (c >= 69745 && c <= 69746)))))
              : (c <= 69749 || (c < 69959
                ? (c < 69891
                  ? (c < 69840
                    ? (c >= 69763 && c <= 69807)
                    : c <= 69864)
                  : (c <= 69926 || c == 69956))
                : (c <= 69959 || (c < 70019
                  ? (c < 70006
                    ? (c >= 69968 && c <= 70002)
                    : c <= 70006)
                  : (c <= 70066 || (c >= 70081 && c <= 70084)))))))
            : (c <= 70106 || (c < 70405
              ? (c < 70280
                ? (c < 70163
                  ? (c < 70144
                    ? c == 70108
                    : c <= 70161)
                  : (c <= 70187 || (c >= 70272 && c <= 70278)))
                : (c <= 70280 || (c < 70303
                  ? (c < 70287
                    ? (c >= 70282 && c <= 70285)
                    : c <= 70301)
                  : (c <= 70312 || (c >= 70320 && c <= 70366)))))
              : (c <= 70412 || (c < 70453
                ? (c < 70442
                  ? (c < 70419
                    ? (c >= 70415 && c <= 70416)
                    : c <= 70440)
                  : (c <= 70448 || (c >= 70450 && c <= 70451)))
                : (c <= 70457 || (c < 70493
                  ? (c < 70480
                    ? c == 70461
                    : c <= 70480)
                  : (c <= 70497 || (c >= 70656 && c <= 70708)))))))))))))
      : (c <= 70730 || (c < 119894
        ? (c < 73056
          ? (c < 72001
            ? (c < 71424
              ? (c < 71128
                ? (c < 70852
                  ? (c < 70784
                    ? (c >= 70751 && c <= 70753)
                    : c <= 70831)
                  : (c <= 70853 || (c < 71040
                    ? c == 70855
                    : c <= 71086)))
                : (c <= 71131 || (c < 71296
                  ? (c < 71236
                    ? (c >= 71168 && c <= 71215)
                    : c <= 71236)
                  : (c <= 71338 || c == 71352))))
              : (c <= 71450 || (c < 71945
                ? (c < 71840
                  ? (c < 71680
                    ? (c >= 71488 && c <= 71494)
                    : c <= 71723)
                  : (c <= 71903 || (c >= 71935 && c <= 71942)))
                : (c <= 71945 || (c < 71960
                  ? (c < 71957
                    ? (c >= 71948 && c <= 71955)
                    : c <= 71958)
                  : (c <= 71983 || c == 71999))))))
            : (c <= 72001 || (c < 72349
              ? (c < 72192
                ? (c < 72161
                  ? (c < 72106
                    ? (c >= 72096 && c <= 72103)
                    : c <= 72144)
                  : (c <= 72161 || c == 72163))
                : (c <= 72192 || (c < 72272
                  ? (c < 72250
                    ? (c >= 72203 && c <= 72242)
                    : c <= 72250)
                  : (c <= 72272 || (c >= 72284 && c <= 72329)))))
              : (c <= 72349 || (c < 72818
                ? (c < 72714
                  ? (c < 72704
                    ? (c >= 72368 && c <= 72440)
                    : c <= 72712)
                  : (c <= 72750 || c == 72768))
                : (c <= 72847 || (c < 72971
                  ? (c < 72968
                    ? (c >= 72960 && c <= 72966)
                    : c <= 72969)
                  : (c <= 73008 || c == 73030))))))))
          : (c <= 73061 || (c < 93952
            ? (c < 82944
              ? (c < 73728
                ? (c < 73112
                  ? (c < 73066
                    ? (c >= 73063 && c <= 73064)
                    : c <= 73097)
                  : (c <= 73112 || (c < 73648
                    ? (c >= 73440 && c <= 73458)
                    : c <= 73648)))
                : (c <= 74649 || (c < 77712
                  ? (c < 74880
                    ? (c >= 74752 && c <= 74862)
                    : c <= 75075)
                  : (c <= 77808 || (c >= 77824 && c <= 78894)))))
              : (c <= 83526 || (c < 92928
                ? (c < 92784
                  ? (c < 92736
                    ? (c >= 92160 && c <= 92728)
                    : c <= 92766)
                  : (c <= 92862 || (c >= 92880 && c <= 92909)))
                : (c <= 92975 || (c < 93053
                  ? (c < 93027
                    ? (c >= 92992 && c <= 92995)
                    : c <= 93047)
                  : (c <= 93071 || (c >= 93760 && c <= 93823)))))))
            : (c <= 94026 || (c < 110589
              ? (c < 94208
                ? (c < 94176
                  ? (c < 94099
                    ? c == 94032
                    : c <= 94111)
                  : (c <= 94177 || c == 94179))
                : (c <= 100343 || (c < 110576
                  ? (c < 101632
                    ? (c >= 100352 && c <= 101589)
                    : c <= 101640)
                  : (c <= 110579 || (c >= 110581 && c <= 110587)))))
              : (c <= 110590 || (c < 113664
                ? (c < 110948
                  ? (c < 110928
                    ? (c >= 110592 && c <= 110882)
                    : c <= 110930)
                  : (c <= 110951 || (c >= 110960 && c <= 111355)))
                : (c <= 113770 || (c < 113808
                  ? (c < 113792
                    ? (c >= 113776 && c <= 113788)
                    : c <= 113800)
                  : (c <= 113817 || (c >= 119808 && c <= 119892)))))))))))
        : (c <= 119964 || (c < 125259
          ? (c < 120572
            ? (c < 120086
              ? (c < 119995
                ? (c < 119973
                  ? (c < 119970
                    ? (c >= 119966 && c <= 119967)
                    : c <= 119970)
                  : (c <= 119974 || (c < 119982
                    ? (c >= 119977 && c <= 119980)
                    : c <= 119993)))
                : (c <= 119995 || (c < 120071
                  ? (c < 120005
                    ? (c >= 119997 && c <= 120003)
                    : c <= 120069)
                  : (c <= 120074 || (c >= 120077 && c <= 120084)))))
              : (c <= 120092 || (c < 120138
                ? (c < 120128
                  ? (c < 120123
                    ? (c >= 120094 && c <= 120121)
                    : c <= 120126)
                  : (c <= 120132 || c == 120134))
                : (c <= 120144 || (c < 120514
                  ? (c < 120488
                    ? (c >= 120146 && c <= 120485)
                    : c <= 120512)
                  : (c <= 120538 || (c >= 120540 && c <= 120570)))))))
            : (c <= 120596 || (c < 123191
              ? (c < 120714
                ? (c < 120656
                  ? (c < 120630
                    ? (c >= 120598 && c <= 120628)
                    : c <= 120654)
                  : (c <= 120686 || (c >= 120688 && c <= 120712)))
                : (c <= 120744 || (c < 122624
                  ? (c < 120772
                    ? (c >= 120746 && c <= 120770)
                    : c <= 120779)
                  : (c <= 122654 || (c >= 123136 && c <= 123180)))))
              : (c <= 123197 || (c < 124904
                ? (c < 123584
                  ? (c < 123536
                    ? c == 123214
                    : c <= 123565)
                  : (c <= 123627 || (c >= 124896 && c <= 124902)))
                : (c <= 124907 || (c < 124928
                  ? (c < 124912
                    ? (c >= 124909 && c <= 124910)
                    : c <= 124926)
                  : (c <= 125124 || (c >= 125184 && c <= 125251)))))))))
          : (c <= 125259 || (c < 126559
            ? (c < 126535
              ? (c < 126505
                ? (c < 126497
                  ? (c < 126469
                    ? (c >= 126464 && c <= 126467)
                    : c <= 126495)
                  : (c <= 126498 || (c < 126503
                    ? c == 126500
                    : c <= 126503)))
                : (c <= 126514 || (c < 126523
                  ? (c < 126521
                    ? (c >= 126516 && c <= 126519)
                    : c <= 126521)
                  : (c <= 126523 || c == 126530))))
              : (c <= 126535 || (c < 126548
                ? (c < 126541
                  ? (c < 126539
                    ? c == 126537
                    : c <= 126539)
                  : (c <= 126543 || (c >= 126545 && c <= 126546)))
                : (c <= 126548 || (c < 126555
                  ? (c < 126553
                    ? c == 126551
                    : c <= 126553)
                  : (c <= 126555 || c == 126557))))))
            : (c <= 126559 || (c < 126625
              ? (c < 126580
                ? (c < 126567
                  ? (c < 126564
                    ? (c >= 126561 && c <= 126562)
                    : c <= 126564)
                  : (c <= 126570 || (c >= 126572 && c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c >= 126603 && c <= 126619)))))
              : (c <= 126627 || (c < 177984
                ? (c < 131072
                  ? (c < 126635
                    ? (c >= 126629 && c <= 126633)
                    : c <= 126651)
                  : (c <= 173791 || (c >= 173824 && c <= 177976)))
                : (c <= 178205 || (c < 194560
                  ? (c < 183984
                    ? (c >= 178208 && c <= 183969)
                    : c <= 191456)
                  : (c <= 195101 || (c >= 196608 && c <= 201546)))))))))))))))));
}

static inline bool sym_ident_pattern_token_character_set_2(int32_t c) {
  return (c < 43616
    ? (c < 3782
      ? (c < 2748
        ? (c < 2045
          ? (c < 1015
            ? (c < 710
              ? (c < 181
                ? (c < '_'
                  ? (c < 'A'
                    ? (c >= '0' && c <= '9')
                    : c <= 'Z')
                  : (c <= '_' || (c < 170
                    ? (c >= 'a' && c <= 'z')
                    : c <= 170)))
                : (c <= 181 || (c < 192
                  ? (c < 186
                    ? c == 183
                    : c <= 186)
                  : (c <= 214 || (c < 248
                    ? (c >= 216 && c <= 246)
                    : c <= 705)))))
              : (c <= 721 || (c < 891
                ? (c < 750
                  ? (c < 748
                    ? (c >= 736 && c <= 740)
                    : c <= 748)
                  : (c <= 750 || (c < 886
                    ? (c >= 768 && c <= 884)
                    : c <= 887)))
                : (c <= 893 || (c < 908
                  ? (c < 902
                    ? c == 895
                    : c <= 906)
                  : (c <= 908 || (c < 931
                    ? (c >= 910 && c <= 929)
                    : c <= 1013)))))))
            : (c <= 1153 || (c < 1519
              ? (c < 1425
                ? (c < 1329
                  ? (c < 1162
                    ? (c >= 1155 && c <= 1159)
                    : c <= 1327)
                  : (c <= 1366 || (c < 1376
                    ? c == 1369
                    : c <= 1416)))
                : (c <= 1469 || (c < 1476
                  ? (c < 1473
                    ? c == 1471
                    : c <= 1474)
                  : (c <= 1477 || (c < 1488
                    ? c == 1479
                    : c <= 1514)))))
              : (c <= 1522 || (c < 1770
                ? (c < 1646
                  ? (c < 1568
                    ? (c >= 1552 && c <= 1562)
                    : c <= 1641)
                  : (c <= 1747 || (c < 1759
                    ? (c >= 1749 && c <= 1756)
                    : c <= 1768)))
                : (c <= 1788 || (c < 1869
                  ? (c < 1808
                    ? c == 1791
                    : c <= 1866)
                  : (c <= 1969 || (c < 2042
                    ? (c >= 1984 && c <= 2037)
                    : c <= 2042)))))))))
          : (c <= 2045 || (c < 2558
            ? (c < 2451
              ? (c < 2200
                ? (c < 2144
                  ? (c < 2112
                    ? (c >= 2048 && c <= 2093)
                    : c <= 2139)
                  : (c <= 2154 || (c < 2185
                    ? (c >= 2160 && c <= 2183)
                    : c <= 2190)))
                : (c <= 2273 || (c < 2417
                  ? (c < 2406
                    ? (c >= 2275 && c <= 2403)
                    : c <= 2415)
                  : (c <= 2435 || (c < 2447
                    ? (c >= 2437 && c <= 2444)
                    : c <= 2448)))))
              : (c <= 2472 || (c < 2507
                ? (c < 2486
                  ? (c < 2482
                    ? (c >= 2474 && c <= 2480)
                    : c <= 2482)
                  : (c <= 2489 || (c < 2503
                    ? (c >= 2492 && c <= 2500)
                    : c <= 2504)))
                : (c <= 2510 || (c < 2527
                  ? (c < 2524
                    ? c == 2519
                    : c <= 2525)
                  : (c <= 2531 || (c < 2556
                    ? (c >= 2534 && c <= 2545)
                    : c <= 2556)))))))
            : (c <= 2558 || (c < 2635
              ? (c < 2610
                ? (c < 2575
                  ? (c < 2565
                    ? (c >= 2561 && c <= 2563)
                    : c <= 2570)
                  : (c <= 2576 || (c < 2602
                    ? (c >= 2579 && c <= 2600)
                    : c <= 2608)))
                : (c <= 2611 || (c < 2620
                  ? (c < 2616
                    ? (c >= 2613 && c <= 2614)
                    : c <= 2617)
                  : (c <= 2620 || (c < 2631
                    ? (c >= 2622 && c <= 2626)
                    : c <= 2632)))))
              : (c <= 2637 || (c < 2693
                ? (c < 2654
                  ? (c < 2649
                    ? c == 2641
                    : c <= 2652)
                  : (c <= 2654 || (c < 2689
                    ? (c >= 2662 && c <= 2677)
                    : c <= 2691)))
                : (c <= 2701 || (c < 2730
                  ? (c < 2707
                    ? (c >= 2703 && c <= 2705)
                    : c <= 2728)
                  : (c <= 2736 || (c < 2741
                    ? (c >= 2738 && c <= 2739)
                    : c <= 2745)))))))))))
        : (c <= 2757 || (c < 3168
          ? (c < 2958
            ? (c < 2866
              ? (c < 2809
                ? (c < 2768
                  ? (c < 2763
                    ? (c >= 2759 && c <= 2761)
                    : c <= 2765)
                  : (c <= 2768 || (c < 2790
                    ? (c >= 2784 && c <= 2787)
                    : c <= 2799)))
                : (c <= 2815 || (c < 2831
                  ? (c < 2821
                    ? (c >= 2817 && c <= 2819)
                    : c <= 2828)
                  : (c <= 2832 || (c < 2858
                    ? (c >= 2835 && c <= 2856)
                    : c <= 2864)))))
              : (c <= 2867 || (c < 2908
                ? (c < 2887
                  ? (c < 2876
                    ? (c >= 2869 && c <= 2873)
                    : c <= 2884)
                  : (c <= 2888 || (c < 2901
                    ? (c >= 2891 && c <= 2893)
                    : c <= 2903)))
                : (c <= 2909 || (c < 2929
                  ? (c < 2918
                    ? (c >= 2911 && c <= 2915)
                    : c <= 2927)
                  : (c <= 2929 || (c < 2949
                    ? (c >= 2946 && c <= 2947)
                    : c <= 2954)))))))
            : (c <= 2960 || (c < 3031
              ? (c < 2984
                ? (c < 2972
                  ? (c < 2969
                    ? (c >= 2962 && c <= 2965)
                    : c <= 2970)
                  : (c <= 2972 || (c < 2979
                    ? (c >= 2974 && c <= 2975)
                    : c <= 2980)))
                : (c <= 2986 || (c < 3014
                  ? (c < 3006
                    ? (c >= 2990 && c <= 3001)
                    : c <= 3010)
                  : (c <= 3016 || (c < 3024
                    ? (c >= 3018 && c <= 3021)
                    : c <= 3024)))))
              : (c <= 3031 || (c < 3132
                ? (c < 3086
                  ? (c < 3072
                    ? (c >= 3046 && c <= 3055)
                    : c <= 3084)
                  : (c <= 3088 || (c < 3114
                    ? (c >= 3090 && c <= 3112)
                    : c <= 3129)))
                : (c <= 3140 || (c < 3157
                  ? (c < 3146
                    ? (c >= 3142 && c <= 3144)
                    : c <= 3149)
                  : (c <= 3158 || (c < 3165
                    ? (c >= 3160 && c <= 3162)
                    : c <= 3165)))))))))
          : (c <= 3171 || (c < 3450
            ? (c < 3293
              ? (c < 3242
                ? (c < 3205
                  ? (c < 3200
                    ? (c >= 3174 && c <= 3183)
                    : c <= 3203)
                  : (c <= 3212 || (c < 3218
                    ? (c >= 3214 && c <= 3216)
                    : c <= 3240)))
                : (c <= 3251 || (c < 3270
                  ? (c < 3260
                    ? (c >= 3253 && c <= 3257)
                    : c <= 3268)
                  : (c <= 3272 || (c < 3285
                    ? (c >= 3274 && c <= 3277)
                    : c <= 3286)))))
              : (c <= 3294 || (c < 3346
                ? (c < 3313
                  ? (c < 3302
                    ? (c >= 3296 && c <= 3299)
                    : c <= 3311)
                  : (c <= 3314 || (c < 3342
                    ? (c >= 3328 && c <= 3340)
                    : c <= 3344)))
                : (c <= 3396 || (c < 3412
                  ? (c < 3402
                    ? (c >= 3398 && c <= 3400)
                    : c <= 3406)
                  : (c <= 3415 || (c < 3430
                    ? (c >= 3423 && c <= 3427)
                    : c <= 3439)))))))
            : (c <= 3455 || (c < 3570
              ? (c < 3520
                ? (c < 3482
                  ? (c < 3461
                    ? (c >= 3457 && c <= 3459)
                    : c <= 3478)
                  : (c <= 3505 || (c < 3517
                    ? (c >= 3507 && c <= 3515)
                    : c <= 3517)))
                : (c <= 3526 || (c < 3542
                  ? (c < 3535
                    ? c == 3530
                    : c <= 3540)
                  : (c <= 3542 || (c < 3558
                    ? (c >= 3544 && c <= 3551)
                    : c <= 3567)))))
              : (c <= 3571 || (c < 3718
                ? (c < 3664
                  ? (c < 3648
                    ? (c >= 3585 && c <= 3642)
                    : c <= 3662)
                  : (c <= 3673 || (c < 3716
                    ? (c >= 3713 && c <= 3714)
                    : c <= 3716)))
                : (c <= 3722 || (c < 3751
                  ? (c < 3749
                    ? (c >= 3724 && c <= 3747)
                    : c <= 3749)
                  : (c <= 3773 || (c >= 3776 && c <= 3780)))))))))))))
      : (c <= 3782 || (c < 8025
        ? (c < 5888
          ? (c < 4688
            ? (c < 3953
              ? (c < 3872
                ? (c < 3804
                  ? (c < 3792
                    ? (c >= 3784 && c <= 3789)
                    : c <= 3801)
                  : (c <= 3807 || (c < 3864
                    ? c == 3840
                    : c <= 3865)))
                : (c <= 3881 || (c < 3897
                  ? (c < 3895
                    ? c == 3893
                    : c <= 3895)
                  : (c <= 3897 || (c < 3913
                    ? (c >= 3902 && c <= 3911)
                    : c <= 3948)))))
              : (c <= 3972 || (c < 4256
                ? (c < 4038
                  ? (c < 3993
                    ? (c >= 3974 && c <= 3991)
                    : c <= 4028)
                  : (c <= 4038 || (c < 4176
                    ? (c >= 4096 && c <= 4169)
                    : c <= 4253)))
                : (c <= 4293 || (c < 4304
                  ? (c < 4301
                    ? c == 4295
                    : c <= 4301)
                  : (c <= 4346 || (c < 4682
                    ? (c >= 4348 && c <= 4680)
                    : c <= 4685)))))))
            : (c <= 4694 || (c < 4882
              ? (c < 4786
                ? (c < 4704
                  ? (c < 4698
                    ? c == 4696
                    : c <= 4701)
                  : (c <= 4744 || (c < 4752
                    ? (c >= 4746 && c <= 4749)
                    : c <= 4784)))
                : (c <= 4789 || (c < 4802
                  ? (c < 4800
                    ? (c >= 4792 && c <= 4798)
                    : c <= 4800)
                  : (c <= 4805 || (c < 4824
                    ? (c >= 4808 && c <= 4822)
                    : c <= 4880)))))
              : (c <= 4885 || (c < 5112
                ? (c < 4969
                  ? (c < 4957
                    ? (c >= 4888 && c <= 4954)
                    : c <= 4959)
                  : (c <= 4977 || (c < 5024
                    ? (c >= 4992 && c <= 5007)
                    : c <= 5109)))
                : (c <= 5117 || (c < 5761
                  ? (c < 5743
                    ? (c >= 5121 && c <= 5740)
                    : c <= 5759)
                  : (c <= 5786 || (c < 5870
                    ? (c >= 5792 && c <= 5866)
                    : c <= 5880)))))))))
          : (c <= 5909 || (c < 6688
            ? (c < 6176
              ? (c < 6016
                ? (c < 5984
                  ? (c < 5952
                    ? (c >= 5919 && c <= 5940)
                    : c <= 5971)
                  : (c <= 5996 || (c < 6002
                    ? (c >= 5998 && c <= 6000)
                    : c <= 6003)))
                : (c <= 6099 || (c < 6112
                  ? (c < 6108
                    ? c == 6103
                    : c <= 6109)
                  : (c <= 6121 || (c < 6159
                    ? (c >= 6155 && c <= 6157)
                    : c <= 6169)))))
              : (c <= 6264 || (c < 6470
                ? (c < 6400
                  ? (c < 6320
                    ? (c >= 6272 && c <= 6314)
                    : c <= 6389)
                  : (c <= 6430 || (c < 6448
                    ? (c >= 6432 && c <= 6443)
                    : c <= 6459)))
                : (c <= 6509 || (c < 6576
                  ? (c < 6528
                    ? (c >= 6512 && c <= 6516)
                    : c <= 6571)
                  : (c <= 6601 || (c < 6656
                    ? (c >= 6608 && c <= 6618)
                    : c <= 6683)))))))
            : (c <= 6750 || (c < 7232
              ? (c < 6847
                ? (c < 6800
                  ? (c < 6783
                    ? (c >= 6752 && c <= 6780)
                    : c <= 6793)
                  : (c <= 6809 || (c < 6832
                    ? c == 6823
                    : c <= 6845)))
                : (c <= 6862 || (c < 7019
                  ? (c < 6992
                    ? (c >= 6912 && c <= 6988)
                    : c <= 7001)
                  : (c <= 7027 || (c < 7168
                    ? (c >= 7040 && c <= 7155)
                    : c <= 7223)))))
              : (c <= 7241 || (c < 7380
                ? (c < 7312
                  ? (c < 7296
                    ? (c >= 7245 && c <= 7293)
                    : c <= 7304)
                  : (c <= 7354 || (c < 7376
                    ? (c >= 7357 && c <= 7359)
                    : c <= 7378)))
                : (c <= 7418 || (c < 7968
                  ? (c < 7960
                    ? (c >= 7424 && c <= 7957)
                    : c <= 7965)
                  : (c <= 8005 || (c < 8016
                    ? (c >= 8008 && c <= 8013)
                    : c <= 8023)))))))))))
        : (c <= 8025 || (c < 11720
          ? (c < 8458
            ? (c < 8178
              ? (c < 8126
                ? (c < 8031
                  ? (c < 8029
                    ? c == 8027
                    : c <= 8029)
                  : (c <= 8061 || (c < 8118
                    ? (c >= 8064 && c <= 8116)
                    : c <= 8124)))
                : (c <= 8126 || (c < 8144
                  ? (c < 8134
                    ? (c >= 8130 && c <= 8132)
                    : c <= 8140)
                  : (c <= 8147 || (c < 8160
                    ? (c >= 8150 && c <= 8155)
                    : c <= 8172)))))
              : (c <= 8180 || (c < 8336
                ? (c < 8276
                  ? (c < 8255
                    ? (c >= 8182 && c <= 8188)
                    : c <= 8256)
                  : (c <= 8276 || (c < 8319
                    ? c == 8305
                    : c <= 8319)))
                : (c <= 8348 || (c < 8421
                  ? (c < 8417
                    ? (c >= 8400 && c <= 8412)
                    : c <= 8417)
                  : (c <= 8432 || (c < 8455
                    ? c == 8450
                    : c <= 8455)))))))
            : (c <= 8467 || (c < 11499
              ? (c < 8490
                ? (c < 8484
                  ? (c < 8472
                    ? c == 8469
                    : c <= 8477)
                  : (c <= 8484 || (c < 8488
                    ? c == 8486
                    : c <= 8488)))
                : (c <= 8505 || (c < 8526
                  ? (c < 8517
                    ? (c >= 8508 && c <= 8511)
                    : c <= 8521)
                  : (c <= 8526 || (c < 11264
                    ? (c >= 8544 && c <= 8584)
                    : c <= 11492)))))
              : (c <= 11507 || (c < 11647
                ? (c < 11565
                  ? (c < 11559
                    ? (c >= 11520 && c <= 11557)
                    : c <= 11559)
                  : (c <= 11565 || (c < 11631
                    ? (c >= 11568 && c <= 11623)
                    : c <= 11631)))
                : (c <= 11670 || (c < 11696
                  ? (c < 11688
                    ? (c >= 11680 && c <= 11686)
                    : c <= 11694)
                  : (c <= 11702 || (c < 11712
                    ? (c >= 11704 && c <= 11710)
                    : c <= 11718)))))))))
          : (c <= 11726 || (c < 42623
            ? (c < 12540
              ? (c < 12337
                ? (c < 11744
                  ? (c < 11736
                    ? (c >= 11728 && c <= 11734)
                    : c <= 11742)
                  : (c <= 11775 || (c < 12321
                    ? (c >= 12293 && c <= 12295)
                    : c <= 12335)))
                : (c <= 12341 || (c < 12441
                  ? (c < 12353
                    ? (c >= 12344 && c <= 12348)
                    : c <= 12438)
                  : (c <= 12442 || (c < 12449
                    ? (c >= 12445 && c <= 12447)
                    : c <= 12538)))))
              : (c <= 12543 || (c < 19968
                ? (c < 12704
                  ? (c < 12593
                    ? (c >= 12549 && c <= 12591)
                    : c <= 12686)
                  : (c <= 12735 || (c < 13312
                    ? (c >= 12784 && c <= 12799)
                    : c <= 19903)))
                : (c <= 42124 || (c < 42512
                  ? (c < 42240
                    ? (c >= 42192 && c <= 42237)
                    : c <= 42508)
                  : (c <= 42539 || (c < 42612
                    ? (c >= 42560 && c <= 42607)
                    : c <= 42621)))))))
            : (c <= 42737 || (c < 43232
              ? (c < 42965
                ? (c < 42891
                  ? (c < 42786
                    ? (c >= 42775 && c <= 42783)
                    : c <= 42888)
                  : (c <= 42954 || (c < 42963
                    ? (c >= 42960 && c <= 42961)
                    : c <= 42963)))
                : (c <= 42969 || (c < 43072
                  ? (c < 43052
                    ? (c >= 42994 && c <= 43047)
                    : c <= 43052)
                  : (c <= 43123 || (c < 43216
                    ? (c >= 43136 && c <= 43205)
                    : c <= 43225)))))
              : (c <= 43255 || (c < 43471
                ? (c < 43312
                  ? (c < 43261
                    ? c == 43259
                    : c <= 43309)
                  : (c <= 43347 || (c < 43392
                    ? (c >= 43360 && c <= 43388)
                    : c <= 43456)))
                : (c <= 43481 || (c < 43584
                  ? (c < 43520
                    ? (c >= 43488 && c <= 43518)
                    : c <= 43574)
                  : (c <= 43597 || (c >= 43600 && c <= 43609)))))))))))))))
    : (c <= 43638 || (c < 71453
      ? (c < 67639
        ? (c < 65345
          ? (c < 64312
            ? (c < 43888
              ? (c < 43785
                ? (c < 43744
                  ? (c < 43739
                    ? (c >= 43642 && c <= 43714)
                    : c <= 43741)
                  : (c <= 43759 || (c < 43777
                    ? (c >= 43762 && c <= 43766)
                    : c <= 43782)))
                : (c <= 43790 || (c < 43816
                  ? (c < 43808
                    ? (c >= 43793 && c <= 43798)
                    : c <= 43814)
                  : (c <= 43822 || (c < 43868
                    ? (c >= 43824 && c <= 43866)
                    : c <= 43881)))))
              : (c <= 44010 || (c < 63744
                ? (c < 44032
                  ? (c < 44016
                    ? (c >= 44012 && c <= 44013)
                    : c <= 44025)
                  : (c <= 55203 || (c < 55243
                    ? (c >= 55216 && c <= 55238)
                    : c <= 55291)))
                : (c <= 64109 || (c < 64275
                  ? (c < 64256
                    ? (c >= 64112 && c <= 64217)
                    : c <= 64262)
                  : (c <= 64279 || (c < 64298
                    ? (c >= 64285 && c <= 64296)
                    : c <= 64310)))))))
            : (c <= 64316 || (c < 65075
              ? (c < 64612
                ? (c < 64323
                  ? (c < 64320
                    ? c == 64318
                    : c <= 64321)
                  : (c <= 64324 || (c < 64467
                    ? (c >= 64326 && c <= 64433)
                    : c <= 64605)))
                : (c <= 64829 || (c < 65008
                  ? (c < 64914
                    ? (c >= 64848 && c <= 64911)
                    : c <= 64967)
                  : (c <= 65017 || (c < 65056
                    ? (c >= 65024 && c <= 65039)
                    : c <= 65071)))))
              : (c <= 65076 || (c < 65147
                ? (c < 65139
                  ? (c < 65137
                    ? (c >= 65101 && c <= 65103)
                    : c <= 65137)
                  : (c <= 65139 || (c < 65145
                    ? c == 65143
                    : c <= 65145)))
                : (c <= 65147 || (c < 65296
                  ? (c < 65151
                    ? c == 65149
                    : c <= 65276)
                  : (c <= 65305 || (c < 65343
                    ? (c >= 65313 && c <= 65338)
                    : c <= 65343)))))))))
          : (c <= 65370 || (c < 66513
            ? (c < 65664
              ? (c < 65536
                ? (c < 65482
                  ? (c < 65474
                    ? (c >= 65382 && c <= 65470)
                    : c <= 65479)
                  : (c <= 65487 || (c < 65498
                    ? (c >= 65490 && c <= 65495)
                    : c <= 65500)))
                : (c <= 65547 || (c < 65596
                  ? (c < 65576
                    ? (c >= 65549 && c <= 65574)
                    : c <= 65594)
                  : (c <= 65597 || (c < 65616
                    ? (c >= 65599 && c <= 65613)
                    : c <= 65629)))))
              : (c <= 65786 || (c < 66304
                ? (c < 66176
                  ? (c < 66045
                    ? (c >= 65856 && c <= 65908)
                    : c <= 66045)
                  : (c <= 66204 || (c < 66272
                    ? (c >= 66208 && c <= 66256)
                    : c <= 66272)))
                : (c <= 66335 || (c < 66432
                  ? (c < 66384
                    ? (c >= 66349 && c <= 66378)
                    : c <= 66426)
                  : (c <= 66461 || (c < 66504
                    ? (c >= 66464 && c <= 66499)
                    : c <= 66511)))))))
            : (c <= 66517 || (c < 66979
              ? (c < 66864
                ? (c < 66736
                  ? (c < 66720
                    ? (c >= 66560 && c <= 66717)
                    : c <= 66729)
                  : (c <= 66771 || (c < 66816
                    ? (c >= 66776 && c <= 66811)
                    : c <= 66855)))
                : (c <= 66915 || (c < 66956
                  ? (c < 66940
                    ? (c >= 66928 && c <= 66938)
                    : c <= 66954)
                  : (c <= 66962 || (c < 66967
                    ? (c >= 66964 && c <= 66965)
                    : c <= 66977)))))
              : (c <= 66993 || (c < 67456
                ? (c < 67072
                  ? (c < 67003
                    ? (c >= 66995 && c <= 67001)
                    : c <= 67004)
                  : (c <= 67382 || (c < 67424
                    ? (c >= 67392 && c <= 67413)
                    : c <= 67431)))
                : (c <= 67461 || (c < 67584
                  ? (c < 67506
                    ? (c >= 67463 && c <= 67504)
                    : c <= 67514)
                  : (c <= 67589 || (c < 67594
                    ? c == 67592
                    : c <= 67637)))))))))))
        : (c <= 67640 || (c < 69956
          ? (c < 68448
            ? (c < 68101
              ? (c < 67828
                ? (c < 67680
                  ? (c < 67647
                    ? c == 67644
                    : c <= 67669)
                  : (c <= 67702 || (c < 67808
                    ? (c >= 67712 && c <= 67742)
                    : c <= 67826)))
                : (c <= 67829 || (c < 67968
                  ? (c < 67872
                    ? (c >= 67840 && c <= 67861)
                    : c <= 67897)
                  : (c <= 68023 || (c < 68096
                    ? (c >= 68030 && c <= 68031)
                    : c <= 68099)))))
              : (c <= 68102 || (c < 68192
                ? (c < 68121
                  ? (c < 68117
                    ? (c >= 68108 && c <= 68115)
                    : c <= 68119)
                  : (c <= 68149 || (c < 68159
                    ? (c >= 68152 && c <= 68154)
                    : c <= 68159)))
                : (c <= 68220 || (c < 68297
                  ? (c < 68288
                    ? (c >= 68224 && c <= 68252)
                    : c <= 68295)
                  : (c <= 68326 || (c < 68416
                    ? (c >= 68352 && c <= 68405)
                    : c <= 68437)))))))
            : (c <= 68466 || (c < 69424
              ? (c < 68912
                ? (c < 68736
                  ? (c < 68608
                    ? (c >= 68480 && c <= 68497)
                    : c <= 68680)
                  : (c <= 68786 || (c < 68864
                    ? (c >= 68800 && c <= 68850)
                    : c <= 68903)))
                : (c <= 68921 || (c < 69296
                  ? (c < 69291
                    ? (c >= 69248 && c <= 69289)
                    : c <= 69292)
                  : (c <= 69297 || (c < 69415
                    ? (c >= 69376 && c <= 69404)
                    : c <= 69415)))))
              : (c <= 69456 || (c < 69759
                ? (c < 69600
                  ? (c < 69552
                    ? (c >= 69488 && c <= 69509)
                    : c <= 69572)
                  : (c <= 69622 || (c < 69734
                    ? (c >= 69632 && c <= 69702)
                    : c <= 69749)))
                : (c <= 69818 || (c < 69872
                  ? (c < 69840
                    ? c == 69826
                    : c <= 69864)
                  : (c <= 69881 || (c < 69942
                    ? (c >= 69888 && c <= 69940)
                    : c <= 69951)))))))))
          : (c <= 69959 || (c < 70459
            ? (c < 70282
              ? (c < 70108
                ? (c < 70016
                  ? (c < 70006
                    ? (c >= 69968 && c <= 70003)
                    : c <= 70006)
                  : (c <= 70084 || (c < 70094
                    ? (c >= 70089 && c <= 70092)
                    : c <= 70106)))
                : (c <= 70108 || (c < 70206
                  ? (c < 70163
                    ? (c >= 70144 && c <= 70161)
                    : c <= 70199)
                  : (c <= 70206 || (c < 70280
                    ? (c >= 70272 && c <= 70278)
                    : c <= 70280)))))
              : (c <= 70285 || (c < 70405
                ? (c < 70320
                  ? (c < 70303
                    ? (c >= 70287 && c <= 70301)
                    : c <= 70312)
                  : (c <= 70378 || (c < 70400
                    ? (c >= 70384 && c <= 70393)
                    : c <= 70403)))
                : (c <= 70412 || (c < 70442
                  ? (c < 70419
                    ? (c >= 70415 && c <= 70416)
                    : c <= 70440)
                  : (c <= 70448 || (c < 70453
                    ? (c >= 70450 && c <= 70451)
                    : c <= 70457)))))))
            : (c <= 70468 || (c < 70855
              ? (c < 70502
                ? (c < 70480
                  ? (c < 70475
                    ? (c >= 70471 && c <= 70472)
                    : c <= 70477)
                  : (c <= 70480 || (c < 70493
                    ? c == 70487
                    : c <= 70499)))
                : (c <= 70508 || (c < 70736
                  ? (c < 70656
                    ? (c >= 70512 && c <= 70516)
                    : c <= 70730)
                  : (c <= 70745 || (c < 70784
                    ? (c >= 70750 && c <= 70753)
                    : c <= 70853)))))
              : (c <= 70855 || (c < 71236
                ? (c < 71096
                  ? (c < 71040
                    ? (c >= 70864 && c <= 70873)
                    : c <= 71093)
                  : (c <= 71104 || (c < 71168
                    ? (c >= 71128 && c <= 71133)
                    : c <= 71232)))
                : (c <= 71236 || (c < 71360
                  ? (c < 71296
                    ? (c >= 71248 && c <= 71257)
                    : c <= 71352)
                  : (c <= 71369 || (c >= 71424 && c <= 71450)))))))))))))
      : (c <= 71467 || (c < 119973
        ? (c < 77824
          ? (c < 72760
            ? (c < 72016
              ? (c < 71945
                ? (c < 71680
                  ? (c < 71488
                    ? (c >= 71472 && c <= 71481)
                    : c <= 71494)
                  : (c <= 71738 || (c < 71935
                    ? (c >= 71840 && c <= 71913)
                    : c <= 71942)))
                : (c <= 71945 || (c < 71960
                  ? (c < 71957
                    ? (c >= 71948 && c <= 71955)
                    : c <= 71958)
                  : (c <= 71989 || (c < 71995
                    ? (c >= 71991 && c <= 71992)
                    : c <= 72003)))))
              : (c <= 72025 || (c < 72263
                ? (c < 72154
                  ? (c < 72106
                    ? (c >= 72096 && c <= 72103)
                    : c <= 72151)
                  : (c <= 72161 || (c < 72192
                    ? (c >= 72163 && c <= 72164)
                    : c <= 72254)))
                : (c <= 72263 || (c < 72368
                  ? (c < 72349
                    ? (c >= 72272 && c <= 72345)
                    : c <= 72349)
                  : (c <= 72440 || (c < 72714
                    ? (c >= 72704 && c <= 72712)
                    : c <= 72758)))))))
            : (c <= 72768 || (c < 73056
              ? (c < 72968
                ? (c < 72850
                  ? (c < 72818
                    ? (c >= 72784 && c <= 72793)
                    : c <= 72847)
                  : (c <= 72871 || (c < 72960
                    ? (c >= 72873 && c <= 72886)
                    : c <= 72966)))
                : (c <= 72969 || (c < 73020
                  ? (c < 73018
                    ? (c >= 72971 && c <= 73014)
                    : c <= 73018)
                  : (c <= 73021 || (c < 73040
                    ? (c >= 73023 && c <= 73031)
                    : c <= 73049)))))
              : (c <= 73061 || (c < 73440
                ? (c < 73104
                  ? (c < 73066
                    ? (c >= 73063 && c <= 73064)
                    : c <= 73102)
                  : (c <= 73105 || (c < 73120
                    ? (c >= 73107 && c <= 73112)
                    : c <= 73129)))
                : (c <= 73462 || (c < 74752
                  ? (c < 73728
                    ? c == 73648
                    : c <= 74649)
                  : (c <= 74862 || (c < 77712
                    ? (c >= 74880 && c <= 75075)
                    : c <= 77808)))))))))
          : (c <= 78894 || (c < 110576
            ? (c < 93027
              ? (c < 92864
                ? (c < 92736
                  ? (c < 92160
                    ? (c >= 82944 && c <= 83526)
                    : c <= 92728)
                  : (c <= 92766 || (c < 92784
                    ? (c >= 92768 && c <= 92777)
                    : c <= 92862)))
                : (c <= 92873 || (c < 92928
                  ? (c < 92912
                    ? (c >= 92880 && c <= 92909)
                    : c <= 92916)
                  : (c <= 92982 || (c < 93008
                    ? (c >= 92992 && c <= 92995)
                    : c <= 93017)))))
              : (c <= 93047 || (c < 94176
                ? (c < 93952
                  ? (c < 93760
                    ? (c >= 93053 && c <= 93071)
                    : c <= 93823)
                  : (c <= 94026 || (c < 94095
                    ? (c >= 94031 && c <= 94087)
                    : c <= 94111)))
                : (c <= 94177 || (c < 94208
                  ? (c < 94192
                    ? (c >= 94179 && c <= 94180)
                    : c <= 94193)
                  : (c <= 100343 || (c < 101632
                    ? (c >= 100352 && c <= 101589)
                    : c <= 101640)))))))
            : (c <= 110579 || (c < 118528
              ? (c < 110960
                ? (c < 110592
                  ? (c < 110589
                    ? (c >= 110581 && c <= 110587)
                    : c <= 110590)
                  : (c <= 110882 || (c < 110948
                    ? (c >= 110928 && c <= 110930)
                    : c <= 110951)))
                : (c <= 111355 || (c < 113792
                  ? (c < 113776
                    ? (c >= 113664 && c <= 113770)
                    : c <= 113788)
                  : (c <= 113800 || (c < 113821
                    ? (c >= 113808 && c <= 113817)
                    : c <= 113822)))))
              : (c <= 118573 || (c < 119210
                ? (c < 119149
                  ? (c < 119141
                    ? (c >= 118576 && c <= 118598)
                    : c <= 119145)
                  : (c <= 119154 || (c < 119173
                    ? (c >= 119163 && c <= 119170)
                    : c <= 119179)))
                : (c <= 119213 || (c < 119894
                  ? (c < 119808
                    ? (c >= 119362 && c <= 119364)
                    : c <= 119892)
                  : (c <= 119964 || (c < 119970
                    ? (c >= 119966 && c <= 119967)
                    : c <= 119970)))))))))))
        : (c <= 119974 || (c < 124912
          ? (c < 120746
            ? (c < 120134
              ? (c < 120071
                ? (c < 119995
                  ? (c < 119982
                    ? (c >= 119977 && c <= 119980)
                    : c <= 119993)
                  : (c <= 119995 || (c < 120005
                    ? (c >= 119997 && c <= 120003)
                    : c <= 120069)))
                : (c <= 120074 || (c < 120094
                  ? (c < 120086
                    ? (c >= 120077 && c <= 120084)
                    : c <= 120092)
                  : (c <= 120121 || (c < 120128
                    ? (c >= 120123 && c <= 120126)
                    : c <= 120132)))))
              : (c <= 120134 || (c < 120572
                ? (c < 120488
                  ? (c < 120146
                    ? (c >= 120138 && c <= 120144)
                    : c <= 120485)
                  : (c <= 120512 || (c < 120540
                    ? (c >= 120514 && c <= 120538)
                    : c <= 120570)))
                : (c <= 120596 || (c < 120656
                  ? (c < 120630
                    ? (c >= 120598 && c <= 120628)
                    : c <= 120654)
                  : (c <= 120686 || (c < 120714
                    ? (c >= 120688 && c <= 120712)
                    : c <= 120744)))))))
            : (c <= 120770 || (c < 122907
              ? (c < 121476
                ? (c < 121344
                  ? (c < 120782
                    ? (c >= 120772 && c <= 120779)
                    : c <= 120831)
                  : (c <= 121398 || (c < 121461
                    ? (c >= 121403 && c <= 121452)
                    : c <= 121461)))
                : (c <= 121476 || (c < 122624
                  ? (c < 121505
                    ? (c >= 121499 && c <= 121503)
                    : c <= 121519)
                  : (c <= 122654 || (c < 122888
                    ? (c >= 122880 && c <= 122886)
                    : c <= 122904)))))
              : (c <= 122913 || (c < 123214
                ? (c < 123136
                  ? (c < 122918
                    ? (c >= 122915 && c <= 122916)
                    : c <= 122922)
                  : (c <= 123180 || (c < 123200
                    ? (c >= 123184 && c <= 123197)
                    : c <= 123209)))
                : (c <= 123214 || (c < 124896
                  ? (c < 123584
                    ? (c >= 123536 && c <= 123566)
                    : c <= 123641)
                  : (c <= 124902 || (c < 124909
                    ? (c >= 124904 && c <= 124907)
                    : c <= 124910)))))))))
          : (c <= 124926 || (c < 126557
            ? (c < 126521
              ? (c < 126469
                ? (c < 125184
                  ? (c < 125136
                    ? (c >= 124928 && c <= 125124)
                    : c <= 125142)
                  : (c <= 125259 || (c < 126464
                    ? (c >= 125264 && c <= 125273)
                    : c <= 126467)))
                : (c <= 126495 || (c < 126503
                  ? (c < 126500
                    ? (c >= 126497 && c <= 126498)
                    : c <= 126500)
                  : (c <= 126503 || (c < 126516
                    ? (c >= 126505 && c <= 126514)
                    : c <= 126519)))))
              : (c <= 126521 || (c < 126541
                ? (c < 126535
                  ? (c < 126530
                    ? c == 126523
                    : c <= 126530)
                  : (c <= 126535 || (c < 126539
                    ? c == 126537
                    : c <= 126539)))
                : (c <= 126543 || (c < 126551
                  ? (c < 126548
                    ? (c >= 126545 && c <= 126546)
                    : c <= 126548)
                  : (c <= 126551 || (c < 126555
                    ? c == 126553
                    : c <= 126555)))))))
            : (c <= 126557 || (c < 126629
              ? (c < 126580
                ? (c < 126564
                  ? (c < 126561
                    ? c == 126559
                    : c <= 126562)
                  : (c <= 126564 || (c < 126572
                    ? (c >= 126567 && c <= 126570)
                    : c <= 126578)))
                : (c <= 126583 || (c < 126592
                  ? (c < 126590
                    ? (c >= 126585 && c <= 126588)
                    : c <= 126590)
                  : (c <= 126601 || (c < 126625
                    ? (c >= 126603 && c <= 126619)
                    : c <= 126627)))))
              : (c <= 126633 || (c < 178208
                ? (c < 131072
                  ? (c < 130032
                    ? (c >= 126635 && c <= 126651)
                    : c <= 130041)
                  : (c <= 173791 || (c < 177984
                    ? (c >= 173824 && c <= 177976)
                    : c <= 178205)))
                : (c <= 183969 || (c < 196608
                  ? (c < 194560
                    ? (c >= 183984 && c <= 191456)
                    : c <= 195101)
                  : (c <= 201546 || (c >= 917760 && c <= 917999)))))))))))))))));
}

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(90);
      if (lookahead == '!') ADVANCE(139);
      if (lookahead == '%') ADVANCE(150);
      if (lookahead == '&') ADVANCE(145);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == ')') ADVANCE(119);
      if (lookahead == '*') ADVANCE(142);
      if (lookahead == '+') ADVANCE(152);
      if (lookahead == ',') ADVANCE(118);
      if (lookahead == '-') ADVANCE(137);
      if (lookahead == '.') ADVANCE(135);
      if (lookahead == '/') ADVANCE(148);
      if (lookahead == '0') ADVANCE(93);
      if (lookahead == ':') ADVANCE(129);
      if (lookahead == ';') ADVANCE(91);
      if (lookahead == '<') ADVANCE(121);
      if (lookahead == '=') ADVANCE(131);
      if (lookahead == '>') ADVANCE(126);
      if (lookahead == '@') ADVANCE(116);
      if (lookahead == '[') ADVANCE(132);
      if (lookahead == ']') ADVANCE(133);
      if (lookahead == '^') ADVANCE(166);
      if (lookahead == '_') ADVANCE(167);
      if (lookahead == '{') ADVANCE(127);
      if (lookahead == '|') ADVANCE(163);
      if (lookahead == '}') ADVANCE(128);
      if (lookahead == '~') ADVANCE(140);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(95);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ' ||
          lookahead == 133 ||
          lookahead == 8206 ||
          lookahead == 8207 ||
          lookahead == 8232 ||
          lookahead == 8233) ADVANCE(195);
      if (sym_ident_pattern_token_character_set_1(lookahead)) ADVANCE(181);
      END_STATE();
    case 1:
      if (lookahead == '!') ADVANCE(24);
      if (lookahead == '%') ADVANCE(149);
      if (lookahead == '&') ADVANCE(144);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == ')') ADVANCE(119);
      if (lookahead == '*') ADVANCE(141);
      if (lookahead == '+') ADVANCE(151);
      if (lookahead == ',') ADVANCE(118);
      if (lookahead == '-') ADVANCE(136);
      if (lookahead == '.') ADVANCE(134);
      if (lookahead == '/') ADVANCE(147);
      if (lookahead == ':') ADVANCE(129);
      if (lookahead == ';') ADVANCE(91);
      if (lookahead == '<') ADVANCE(122);
      if (lookahead == '=') ADVANCE(131);
      if (lookahead == '>') ADVANCE(126);
      if (lookahead == '[') ADVANCE(132);
      if (lookahead == ']') ADVANCE(133);
      if (lookahead == '^') ADVANCE(165);
      if (lookahead == '{') ADVANCE(127);
      if (lookahead == '|') ADVANCE(164);
      if (lookahead == '}') ADVANCE(128);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ' ||
          lookahead == 133 ||
          lookahead == 8206 ||
          lookahead == 8207 ||
          lookahead == 8232 ||
          lookahead == 8233) ADVANCE(195);
      END_STATE();
    case 2:
      if (lookahead == '!') ADVANCE(24);
      if (lookahead == '%') ADVANCE(149);
      if (lookahead == '&') ADVANCE(144);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == ')') ADVANCE(119);
      if (lookahead == '*') ADVANCE(141);
      if (lookahead == '+') ADVANCE(151);
      if (lookahead == ',') ADVANCE(118);
      if (lookahead == '-') ADVANCE(136);
      if (lookahead == '.') ADVANCE(134);
      if (lookahead == '/') ADVANCE(147);
      if (lookahead == ':') ADVANCE(129);
      if (lookahead == ';') ADVANCE(91);
      if (lookahead == '<') ADVANCE(122);
      if (lookahead == '=') ADVANCE(28);
      if (lookahead == '>') ADVANCE(126);
      if (lookahead == '[') ADVANCE(132);
      if (lookahead == ']') ADVANCE(133);
      if (lookahead == '^') ADVANCE(165);
      if (lookahead == '{') ADVANCE(127);
      if (lookahead == '|') ADVANCE(164);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ' ||
          lookahead == 133 ||
          lookahead == 8206 ||
          lookahead == 8207 ||
          lookahead == 8232 ||
          lookahead == 8233) ADVANCE(195);
      END_STATE();
    case 3:
      if (lookahead == '!') ADVANCE(24);
      if (lookahead == '%') ADVANCE(149);
      if (lookahead == '&') ADVANCE(7);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == ')') ADVANCE(119);
      if (lookahead == '*') ADVANCE(141);
      if (lookahead == '+') ADVANCE(151);
      if (lookahead == ',') ADVANCE(118);
      if (lookahead == '-') ADVANCE(136);
      if (lookahead == '.') ADVANCE(134);
      if (lookahead == '/') ADVANCE(147);
      if (lookahead == ':') ADVANCE(129);
      if (lookahead == ';') ADVANCE(91);
      if (lookahead == '<') ADVANCE(123);
      if (lookahead == '=') ADVANCE(28);
      if (lookahead == '>') ADVANCE(125);
      if (lookahead == '[') ADVANCE(132);
      if (lookahead == ']') ADVANCE(133);
      if (lookahead == '{') ADVANCE(127);
      if (lookahead == '|') ADVANCE(73);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ' ||
          lookahead == 133 ||
          lookahead == 8206 ||
          lookahead == 8207 ||
          lookahead == 8232 ||
          lookahead == 8233) ADVANCE(195);
      END_STATE();
    case 4:
      if (lookahead == '%') ADVANCE(149);
      if (lookahead == '&') ADVANCE(143);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == ')') ADVANCE(119);
      if (lookahead == '*') ADVANCE(141);
      if (lookahead == '+') ADVANCE(151);
      if (lookahead == ',') ADVANCE(118);
      if (lookahead == '-') ADVANCE(136);
      if (lookahead == '.') ADVANCE(134);
      if (lookahead == '/') ADVANCE(147);
      if (lookahead == ':') ADVANCE(129);
      if (lookahead == ';') ADVANCE(91);
      if (lookahead == '>') ADVANCE(124);
      if (lookahead == '[') ADVANCE(132);
      if (lookahead == ']') ADVANCE(133);
      if (lookahead == '^') ADVANCE(165);
      if (lookahead == '{') ADVANCE(127);
      if (lookahead == '|') ADVANCE(162);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ' ||
          lookahead == 133 ||
          lookahead == 8206 ||
          lookahead == 8207 ||
          lookahead == 8232 ||
          lookahead == 8233) ADVANCE(195);
      END_STATE();
    case 5:
      if (lookahead == '%') ADVANCE(25);
      if (lookahead == '&') ADVANCE(146);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == ')') ADVANCE(119);
      if (lookahead == '*') ADVANCE(27);
      if (lookahead == '+') ADVANCE(8);
      if (lookahead == ',') ADVANCE(118);
      if (lookahead == '-') ADVANCE(10);
      if (lookahead == '.') ADVANCE(134);
      if (lookahead == '/') ADVANCE(14);
      if (lookahead == ':') ADVANCE(129);
      if (lookahead == ';') ADVANCE(91);
      if (lookahead == '<') ADVANCE(23);
      if (lookahead == '=') ADVANCE(130);
      if (lookahead == '>') ADVANCE(33);
      if (lookahead == '[') ADVANCE(132);
      if (lookahead == ']') ADVANCE(133);
      if (lookahead == '^') ADVANCE(29);
      if (lookahead == '_') ADVANCE(88);
      if (lookahead == '{') ADVANCE(127);
      if (lookahead == '|') ADVANCE(30);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ' ||
          lookahead == 133 ||
          lookahead == 8206 ||
          lookahead == 8207 ||
          lookahead == 8232 ||
          lookahead == 8233) ADVANCE(195);
      if (sym_ident_pattern_token_character_set_1(lookahead)) ADVANCE(181);
      END_STATE();
    case 6:
      if (lookahead == '%') ADVANCE(25);
      if (lookahead == '&') ADVANCE(26);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == ')') ADVANCE(119);
      if (lookahead == '*') ADVANCE(27);
      if (lookahead == '+') ADVANCE(8);
      if (lookahead == '-') ADVANCE(9);
      if (lookahead == '.') ADVANCE(134);
      if (lookahead == '/') ADVANCE(13);
      if (lookahead == '<') ADVANCE(23);
      if (lookahead == '=') ADVANCE(130);
      if (lookahead == '>') ADVANCE(33);
      if (lookahead == '[') ADVANCE(132);
      if (lookahead == '^') ADVANCE(29);
      if (lookahead == '|') ADVANCE(30);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ' ||
          lookahead == 133 ||
          lookahead == 8206 ||
          lookahead == 8207 ||
          lookahead == 8232 ||
          lookahead == 8233) ADVANCE(195);
      END_STATE();
    case 7:
      if (lookahead == '&') ADVANCE(160);
      END_STATE();
    case 8:
      if (lookahead == '+') ADVANCE(178);
      if (lookahead == '=') ADVANCE(168);
      END_STATE();
    case 9:
      if (lookahead == '-') ADVANCE(179);
      if (lookahead == '=') ADVANCE(169);
      END_STATE();
    case 10:
      if (lookahead == '-') ADVANCE(179);
      if (lookahead == '=') ADVANCE(169);
      if (lookahead == '>') ADVANCE(180);
      END_STATE();
    case 11:
      if (lookahead == '.') ADVANCE(104);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(74);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(11);
      END_STATE();
    case 12:
      if (lookahead == '.') ADVANCE(87);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(97);
      END_STATE();
    case 13:
      if (lookahead == '/') ADVANCE(190);
      if (lookahead == '=') ADVANCE(171);
      END_STATE();
    case 14:
      if (lookahead == '/') ADVANCE(190);
      if (lookahead == '=') ADVANCE(171);
      if (lookahead == '[') ADVANCE(54);
      END_STATE();
    case 15:
      if (lookahead == '/') ADVANCE(182);
      if (lookahead == '[') ADVANCE(56);
      END_STATE();
    case 16:
      if (lookahead == '/') ADVANCE(186);
      if (lookahead == '[') ADVANCE(63);
      END_STATE();
    case 17:
      if (lookahead == '/') ADVANCE(183);
      if (lookahead == '[') ADVANCE(57);
      END_STATE();
    case 18:
      if (lookahead == '/') ADVANCE(187);
      if (lookahead == '[') ADVANCE(64);
      END_STATE();
    case 19:
      if (lookahead == '/') ADVANCE(184);
      if (lookahead == '[') ADVANCE(55);
      END_STATE();
    case 20:
      if (lookahead == '/') ADVANCE(188);
      if (lookahead == '[') ADVANCE(62);
      END_STATE();
    case 21:
      if (lookahead == '/') ADVANCE(185);
      END_STATE();
    case 22:
      if (lookahead == '/') ADVANCE(189);
      END_STATE();
    case 23:
      if (lookahead == '<') ADVANCE(31);
      END_STATE();
    case 24:
      if (lookahead == '=') ADVANCE(159);
      END_STATE();
    case 25:
      if (lookahead == '=') ADVANCE(172);
      END_STATE();
    case 26:
      if (lookahead == '=') ADVANCE(173);
      END_STATE();
    case 27:
      if (lookahead == '=') ADVANCE(170);
      END_STATE();
    case 28:
      if (lookahead == '=') ADVANCE(158);
      END_STATE();
    case 29:
      if (lookahead == '=') ADVANCE(175);
      END_STATE();
    case 30:
      if (lookahead == '=') ADVANCE(174);
      END_STATE();
    case 31:
      if (lookahead == '=') ADVANCE(177);
      END_STATE();
    case 32:
      if (lookahead == '=') ADVANCE(176);
      END_STATE();
    case 33:
      if (lookahead == '>') ADVANCE(32);
      END_STATE();
    case 34:
      if (lookahead == ']') ADVANCE(15);
      END_STATE();
    case 35:
      if (lookahead == ']') ADVANCE(21);
      END_STATE();
    case 36:
      if (lookahead == ']') ADVANCE(16);
      END_STATE();
    case 37:
      if (lookahead == ']') ADVANCE(22);
      END_STATE();
    case 38:
      if (lookahead == ']') ADVANCE(17);
      END_STATE();
    case 39:
      if (lookahead == ']') ADVANCE(18);
      END_STATE();
    case 40:
      if (lookahead == ']') ADVANCE(19);
      END_STATE();
    case 41:
      if (lookahead == ']') ADVANCE(20);
      END_STATE();
    case 42:
      if (lookahead == 'a') ADVANCE(34);
      END_STATE();
    case 43:
      if (lookahead == 'a') ADVANCE(35);
      END_STATE();
    case 44:
      if (lookahead == 'a') ADVANCE(38);
      END_STATE();
    case 45:
      if (lookahead == 'a') ADVANCE(40);
      END_STATE();
    case 46:
      if (lookahead == 'b') ADVANCE(42);
      END_STATE();
    case 47:
      if (lookahead == 'b') ADVANCE(43);
      END_STATE();
    case 48:
      if (lookahead == 'b') ADVANCE(44);
      END_STATE();
    case 49:
      if (lookahead == 'b') ADVANCE(45);
      END_STATE();
    case 50:
      if (lookahead == 'g') ADVANCE(46);
      END_STATE();
    case 51:
      if (lookahead == 'g') ADVANCE(47);
      END_STATE();
    case 52:
      if (lookahead == 'g') ADVANCE(48);
      END_STATE();
    case 53:
      if (lookahead == 'g') ADVANCE(49);
      END_STATE();
    case 54:
      if (lookahead == 'r') ADVANCE(50);
      if (lookahead == 'x') ADVANCE(65);
      END_STATE();
    case 55:
      if (lookahead == 'r') ADVANCE(51);
      END_STATE();
    case 56:
      if (lookahead == 'r') ADVANCE(52);
      END_STATE();
    case 57:
      if (lookahead == 'r') ADVANCE(53);
      END_STATE();
    case 58:
      if (lookahead == 'w') ADVANCE(37);
      END_STATE();
    case 59:
      if (lookahead == 'w') ADVANCE(36);
      END_STATE();
    case 60:
      if (lookahead == 'w') ADVANCE(39);
      END_STATE();
    case 61:
      if (lookahead == 'w') ADVANCE(41);
      END_STATE();
    case 62:
      if (lookahead == 'x') ADVANCE(66);
      END_STATE();
    case 63:
      if (lookahead == 'x') ADVANCE(67);
      END_STATE();
    case 64:
      if (lookahead == 'x') ADVANCE(68);
      END_STATE();
    case 65:
      if (lookahead == 'y') ADVANCE(69);
      END_STATE();
    case 66:
      if (lookahead == 'y') ADVANCE(70);
      END_STATE();
    case 67:
      if (lookahead == 'y') ADVANCE(71);
      END_STATE();
    case 68:
      if (lookahead == 'y') ADVANCE(72);
      END_STATE();
    case 69:
      if (lookahead == 'z') ADVANCE(59);
      END_STATE();
    case 70:
      if (lookahead == 'z') ADVANCE(58);
      END_STATE();
    case 71:
      if (lookahead == 'z') ADVANCE(60);
      END_STATE();
    case 72:
      if (lookahead == 'z') ADVANCE(61);
      END_STATE();
    case 73:
      if (lookahead == '|') ADVANCE(161);
      END_STATE();
    case 74:
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(81);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(107);
      END_STATE();
    case 75:
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(82);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(102);
      END_STATE();
    case 76:
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(83);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(105);
      END_STATE();
    case 77:
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(84);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(115);
      END_STATE();
    case 78:
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(85);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(110);
      END_STATE();
    case 79:
      if (lookahead == '+' ||
          lookahead == '-') ADVANCE(86);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(113);
      END_STATE();
    case 80:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(101);
      END_STATE();
    case 81:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(107);
      END_STATE();
    case 82:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(102);
      END_STATE();
    case 83:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(105);
      END_STATE();
    case 84:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(115);
      END_STATE();
    case 85:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(110);
      END_STATE();
    case 86:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(113);
      END_STATE();
    case 87:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(109);
      END_STATE();
    case 88:
      if (sym_ident_pattern_token_character_set_2(lookahead)) ADVANCE(181);
      END_STATE();
    case 89:
      if (eof) ADVANCE(90);
      if (lookahead == '!') ADVANCE(138);
      if (lookahead == '%') ADVANCE(149);
      if (lookahead == '&') ADVANCE(143);
      if (lookahead == '(') ADVANCE(117);
      if (lookahead == ')') ADVANCE(119);
      if (lookahead == '*') ADVANCE(141);
      if (lookahead == '+') ADVANCE(151);
      if (lookahead == ',') ADVANCE(118);
      if (lookahead == '-') ADVANCE(136);
      if (lookahead == '.') ADVANCE(80);
      if (lookahead == '/') ADVANCE(147);
      if (lookahead == '0') ADVANCE(93);
      if (lookahead == ':') ADVANCE(129);
      if (lookahead == ';') ADVANCE(91);
      if (lookahead == '<') ADVANCE(120);
      if (lookahead == '=') ADVANCE(130);
      if (lookahead == '>') ADVANCE(124);
      if (lookahead == '@') ADVANCE(116);
      if (lookahead == ']') ADVANCE(133);
      if (lookahead == '^') ADVANCE(165);
      if (lookahead == '_') ADVANCE(167);
      if (lookahead == '{') ADVANCE(127);
      if (lookahead == '|') ADVANCE(162);
      if (lookahead == '}') ADVANCE(128);
      if (lookahead == '~') ADVANCE(140);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(95);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ' ||
          lookahead == 133 ||
          lookahead == 8206 ||
          lookahead == 8207 ||
          lookahead == 8232 ||
          lookahead == 8233) ADVANCE(195);
      if (sym_ident_pattern_token_character_set_1(lookahead)) ADVANCE(181);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(aux_sym_decimal_int_literal_token1);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(aux_sym_decimal_int_literal_token1);
      if (lookahead == '.') ADVANCE(104);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(74);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(12);
      if (lookahead == 'f' ||
          lookahead == 'h') ADVANCE(98);
      if (lookahead == 'i' ||
          lookahead == 'u') ADVANCE(92);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(11);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(aux_sym_decimal_int_literal_token2);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(aux_sym_decimal_int_literal_token2);
      if (lookahead == '.') ADVANCE(104);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(74);
      if (lookahead == 'f' ||
          lookahead == 'h') ADVANCE(99);
      if (lookahead == 'i' ||
          lookahead == 'u') ADVANCE(94);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(95);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(sym_hex_int_literal);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(sym_hex_int_literal);
      if (lookahead == '.') ADVANCE(112);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(77);
      if (lookahead == 'i' ||
          lookahead == 'u') ADVANCE(96);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(97);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(aux_sym_decimal_float_literal_token1);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(aux_sym_decimal_float_literal_token2);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(aux_sym_decimal_float_literal_token3);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(aux_sym_decimal_float_literal_token3);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(75);
      if (lookahead == 'f' ||
          lookahead == 'h') ADVANCE(100);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(101);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(aux_sym_decimal_float_literal_token3);
      if (lookahead == 'f' ||
          lookahead == 'h') ADVANCE(100);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(102);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(aux_sym_decimal_float_literal_token4);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(aux_sym_decimal_float_literal_token4);
      if (lookahead == 'E' ||
          lookahead == 'e') ADVANCE(76);
      if (lookahead == 'f' ||
          lookahead == 'h') ADVANCE(103);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(101);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(aux_sym_decimal_float_literal_token4);
      if (lookahead == 'f' ||
          lookahead == 'h') ADVANCE(103);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(105);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(aux_sym_decimal_float_literal_token5);
      END_STATE();
    case 107:
      ACCEPT_TOKEN(aux_sym_decimal_float_literal_token5);
      if (lookahead == 'f' ||
          lookahead == 'h') ADVANCE(106);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(107);
      END_STATE();
    case 108:
      ACCEPT_TOKEN(aux_sym_hex_float_literal_token1);
      END_STATE();
    case 109:
      ACCEPT_TOKEN(aux_sym_hex_float_literal_token1);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(78);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(109);
      END_STATE();
    case 110:
      ACCEPT_TOKEN(aux_sym_hex_float_literal_token1);
      if (lookahead == 'f' ||
          lookahead == 'h') ADVANCE(108);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(110);
      END_STATE();
    case 111:
      ACCEPT_TOKEN(aux_sym_hex_float_literal_token2);
      END_STATE();
    case 112:
      ACCEPT_TOKEN(aux_sym_hex_float_literal_token2);
      if (lookahead == 'P' ||
          lookahead == 'p') ADVANCE(79);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(109);
      END_STATE();
    case 113:
      ACCEPT_TOKEN(aux_sym_hex_float_literal_token2);
      if (lookahead == 'f' ||
          lookahead == 'h') ADVANCE(111);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(113);
      END_STATE();
    case 114:
      ACCEPT_TOKEN(aux_sym_hex_float_literal_token3);
      END_STATE();
    case 115:
      ACCEPT_TOKEN(aux_sym_hex_float_literal_token3);
      if (lookahead == 'f' ||
          lookahead == 'h') ADVANCE(114);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(115);
      END_STATE();
    case 116:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 117:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 118:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 119:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 120:
      ACCEPT_TOKEN(anon_sym_LT);
      END_STATE();
    case 121:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '<') ADVANCE(154);
      if (lookahead == '=') ADVANCE(156);
      END_STATE();
    case 122:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '<') ADVANCE(153);
      if (lookahead == '=') ADVANCE(156);
      END_STATE();
    case 123:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '=') ADVANCE(156);
      END_STATE();
    case 124:
      ACCEPT_TOKEN(anon_sym_GT);
      END_STATE();
    case 125:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(157);
      END_STATE();
    case 126:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(157);
      if (lookahead == '>') ADVANCE(155);
      END_STATE();
    case 127:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 128:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 129:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 130:
      ACCEPT_TOKEN(anon_sym_EQ);
      END_STATE();
    case 131:
      ACCEPT_TOKEN(anon_sym_EQ);
      if (lookahead == '=') ADVANCE(158);
      END_STATE();
    case 132:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 133:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 134:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 135:
      ACCEPT_TOKEN(anon_sym_DOT);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(101);
      END_STATE();
    case 136:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 137:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '=') ADVANCE(169);
      if (lookahead == '>') ADVANCE(180);
      END_STATE();
    case 138:
      ACCEPT_TOKEN(anon_sym_BANG);
      END_STATE();
    case 139:
      ACCEPT_TOKEN(anon_sym_BANG);
      if (lookahead == '=') ADVANCE(159);
      END_STATE();
    case 140:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 141:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 142:
      ACCEPT_TOKEN(anon_sym_STAR);
      if (lookahead == '=') ADVANCE(170);
      END_STATE();
    case 143:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 144:
      ACCEPT_TOKEN(anon_sym_AMP);
      if (lookahead == '&') ADVANCE(160);
      END_STATE();
    case 145:
      ACCEPT_TOKEN(anon_sym_AMP);
      if (lookahead == '&') ADVANCE(160);
      if (lookahead == '=') ADVANCE(173);
      END_STATE();
    case 146:
      ACCEPT_TOKEN(anon_sym_AMP);
      if (lookahead == '=') ADVANCE(173);
      END_STATE();
    case 147:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '/') ADVANCE(190);
      END_STATE();
    case 148:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '/') ADVANCE(190);
      if (lookahead == '=') ADVANCE(171);
      if (lookahead == '[') ADVANCE(54);
      END_STATE();
    case 149:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 150:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      if (lookahead == '=') ADVANCE(172);
      END_STATE();
    case 151:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 152:
      ACCEPT_TOKEN(anon_sym_PLUS);
      if (lookahead == '+') ADVANCE(178);
      if (lookahead == '=') ADVANCE(168);
      END_STATE();
    case 153:
      ACCEPT_TOKEN(anon_sym_LT_LT);
      END_STATE();
    case 154:
      ACCEPT_TOKEN(anon_sym_LT_LT);
      if (lookahead == '=') ADVANCE(177);
      END_STATE();
    case 155:
      ACCEPT_TOKEN(anon_sym_GT_GT);
      END_STATE();
    case 156:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 157:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 158:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 159:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 160:
      ACCEPT_TOKEN(anon_sym_AMP_AMP);
      END_STATE();
    case 161:
      ACCEPT_TOKEN(anon_sym_PIPE_PIPE);
      END_STATE();
    case 162:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 163:
      ACCEPT_TOKEN(anon_sym_PIPE);
      if (lookahead == '=') ADVANCE(174);
      if (lookahead == '|') ADVANCE(161);
      END_STATE();
    case 164:
      ACCEPT_TOKEN(anon_sym_PIPE);
      if (lookahead == '|') ADVANCE(161);
      END_STATE();
    case 165:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 166:
      ACCEPT_TOKEN(anon_sym_CARET);
      if (lookahead == '=') ADVANCE(175);
      END_STATE();
    case 167:
      ACCEPT_TOKEN(anon_sym__);
      if (sym_ident_pattern_token_character_set_2(lookahead)) ADVANCE(181);
      END_STATE();
    case 168:
      ACCEPT_TOKEN(anon_sym_PLUS_EQ);
      END_STATE();
    case 169:
      ACCEPT_TOKEN(anon_sym_DASH_EQ);
      END_STATE();
    case 170:
      ACCEPT_TOKEN(anon_sym_STAR_EQ);
      END_STATE();
    case 171:
      ACCEPT_TOKEN(anon_sym_SLASH_EQ);
      END_STATE();
    case 172:
      ACCEPT_TOKEN(anon_sym_PERCENT_EQ);
      END_STATE();
    case 173:
      ACCEPT_TOKEN(anon_sym_AMP_EQ);
      END_STATE();
    case 174:
      ACCEPT_TOKEN(anon_sym_PIPE_EQ);
      END_STATE();
    case 175:
      ACCEPT_TOKEN(anon_sym_CARET_EQ);
      END_STATE();
    case 176:
      ACCEPT_TOKEN(anon_sym_GT_GT_EQ);
      END_STATE();
    case 177:
      ACCEPT_TOKEN(anon_sym_LT_LT_EQ);
      END_STATE();
    case 178:
      ACCEPT_TOKEN(anon_sym_PLUS_PLUS);
      END_STATE();
    case 179:
      ACCEPT_TOKEN(anon_sym_DASH_DASH);
      END_STATE();
    case 180:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 181:
      ACCEPT_TOKEN(sym_ident_pattern_token);
      if (sym_ident_pattern_token_character_set_2(lookahead)) ADVANCE(181);
      END_STATE();
    case 182:
      ACCEPT_TOKEN(anon_sym_SLASH_LBRACKrgba_RBRACK_SLASH);
      END_STATE();
    case 183:
      ACCEPT_TOKEN(anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH);
      END_STATE();
    case 184:
      ACCEPT_TOKEN(anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH);
      END_STATE();
    case 186:
      ACCEPT_TOKEN(anon_sym_SLASH_LBRACKxyzw_RBRACK_SLASH);
      END_STATE();
    case 187:
      ACCEPT_TOKEN(anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH);
      END_STATE();
    case 188:
      ACCEPT_TOKEN(anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH);
      END_STATE();
    case 189:
      ACCEPT_TOKEN(anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH);
      END_STATE();
    case 190:
      ACCEPT_TOKEN(anon_sym_SLASH_SLASH);
      END_STATE();
    case 191:
      ACCEPT_TOKEN(anon_sym_SLASH_SLASH);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(194);
      END_STATE();
    case 192:
      ACCEPT_TOKEN(aux_sym__comment_token1);
      if (lookahead == '\n') ADVANCE(195);
      if (lookahead == '/') ADVANCE(193);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ' ||
          lookahead == 133 ||
          lookahead == 8206 ||
          lookahead == 8207 ||
          lookahead == 8232 ||
          lookahead == 8233) ADVANCE(194);
      if (lookahead != 0) ADVANCE(194);
      END_STATE();
    case 193:
      ACCEPT_TOKEN(aux_sym__comment_token1);
      if (lookahead == '/') ADVANCE(191);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(194);
      END_STATE();
    case 194:
      ACCEPT_TOKEN(aux_sym__comment_token1);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(194);
      END_STATE();
    case 195:
      ACCEPT_TOKEN(sym__blankspace);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (lookahead == 'C') ADVANCE(1);
      if (lookahead == 'D') ADVANCE(2);
      if (lookahead == 'G') ADVANCE(3);
      if (lookahead == 'H') ADVANCE(4);
      if (lookahead == 'N') ADVANCE(5);
      if (lookahead == 'S') ADVANCE(6);
      if (lookahead == 'a') ADVANCE(7);
      if (lookahead == 'b') ADVANCE(8);
      if (lookahead == 'c') ADVANCE(9);
      if (lookahead == 'd') ADVANCE(10);
      if (lookahead == 'e') ADVANCE(11);
      if (lookahead == 'f') ADVANCE(12);
      if (lookahead == 'g') ADVANCE(13);
      if (lookahead == 'h') ADVANCE(14);
      if (lookahead == 'i') ADVANCE(15);
      if (lookahead == 'l') ADVANCE(16);
      if (lookahead == 'm') ADVANCE(17);
      if (lookahead == 'n') ADVANCE(18);
      if (lookahead == 'o') ADVANCE(19);
      if (lookahead == 'p') ADVANCE(20);
      if (lookahead == 'r') ADVANCE(21);
      if (lookahead == 's') ADVANCE(22);
      if (lookahead == 't') ADVANCE(23);
      if (lookahead == 'u') ADVANCE(24);
      if (lookahead == 'v') ADVANCE(25);
      if (lookahead == 'w') ADVANCE(26);
      if (lookahead == 'y') ADVANCE(27);
      END_STATE();
    case 1:
      if (lookahead == 'o') ADVANCE(28);
      END_STATE();
    case 2:
      if (lookahead == 'o') ADVANCE(29);
      END_STATE();
    case 3:
      if (lookahead == 'e') ADVANCE(30);
      END_STATE();
    case 4:
      if (lookahead == 'u') ADVANCE(31);
      END_STATE();
    case 5:
      if (lookahead == 'U') ADVANCE(32);
      END_STATE();
    case 6:
      if (lookahead == 'e') ADVANCE(33);
      END_STATE();
    case 7:
      if (lookahead == 'b') ADVANCE(34);
      if (lookahead == 'c') ADVANCE(35);
      if (lookahead == 'l') ADVANCE(36);
      if (lookahead == 'r') ADVANCE(37);
      if (lookahead == 's') ADVANCE(38);
      if (lookahead == 't') ADVANCE(39);
      if (lookahead == 'u') ADVANCE(40);
      if (lookahead == 'w') ADVANCE(41);
      END_STATE();
    case 8:
      if (lookahead == 'e') ADVANCE(42);
      if (lookahead == 'g') ADVANCE(43);
      if (lookahead == 'i') ADVANCE(44);
      if (lookahead == 'o') ADVANCE(45);
      if (lookahead == 'r') ADVANCE(46);
      if (lookahead == 'u') ADVANCE(47);
      END_STATE();
    case 9:
      if (lookahead == 'a') ADVANCE(48);
      if (lookahead == 'e') ADVANCE(49);
      if (lookahead == 'l') ADVANCE(50);
      if (lookahead == 'o') ADVANCE(51);
      if (lookahead == 'r') ADVANCE(52);
      END_STATE();
    case 10:
      if (lookahead == 'e') ADVANCE(53);
      if (lookahead == 'i') ADVANCE(54);
      if (lookahead == 'o') ADVANCE(55);
      if (lookahead == 'y') ADVANCE(56);
      END_STATE();
    case 11:
      if (lookahead == 'l') ADVANCE(57);
      if (lookahead == 'n') ADVANCE(58);
      if (lookahead == 'x') ADVANCE(59);
      END_STATE();
    case 12:
      if (lookahead == '1') ADVANCE(60);
      if (lookahead == '3') ADVANCE(61);
      if (lookahead == 'a') ADVANCE(62);
      if (lookahead == 'i') ADVANCE(63);
      if (lookahead == 'l') ADVANCE(64);
      if (lookahead == 'n') ADVANCE(65);
      if (lookahead == 'o') ADVANCE(66);
      if (lookahead == 'r') ADVANCE(67);
      if (lookahead == 'u') ADVANCE(68);
      if (lookahead == 'x') ADVANCE(69);
      END_STATE();
    case 13:
      if (lookahead == 'e') ADVANCE(70);
      if (lookahead == 'l') ADVANCE(71);
      if (lookahead == 'o') ADVANCE(72);
      if (lookahead == 'r') ADVANCE(73);
      END_STATE();
    case 14:
      if (lookahead == 'a') ADVANCE(74);
      if (lookahead == 'i') ADVANCE(75);
      END_STATE();
    case 15:
      if (lookahead == '3') ADVANCE(76);
      if (lookahead == 'd') ADVANCE(77);
      if (lookahead == 'f') ADVANCE(78);
      if (lookahead == 'm') ADVANCE(79);
      if (lookahead == 'n') ADVANCE(80);
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(81);
      if (lookahead == 'e') ADVANCE(82);
      if (lookahead == 'i') ADVANCE(83);
      if (lookahead == 'o') ADVANCE(84);
      END_STATE();
    case 17:
      if (lookahead == 'a') ADVANCE(85);
      if (lookahead == 'e') ADVANCE(86);
      if (lookahead == 'o') ADVANCE(87);
      if (lookahead == 'u') ADVANCE(88);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(89);
      if (lookahead == 'e') ADVANCE(90);
      if (lookahead == 'i') ADVANCE(91);
      if (lookahead == 'o') ADVANCE(92);
      if (lookahead == 'u') ADVANCE(93);
      END_STATE();
    case 19:
      if (lookahead == 'f') ADVANCE(94);
      if (lookahead == 'p') ADVANCE(95);
      if (lookahead == 'v') ADVANCE(96);
      END_STATE();
    case 20:
      if (lookahead == 'a') ADVANCE(97);
      if (lookahead == 'e') ADVANCE(98);
      if (lookahead == 'i') ADVANCE(99);
      if (lookahead == 'o') ADVANCE(100);
      if (lookahead == 'r') ADVANCE(101);
      if (lookahead == 't') ADVANCE(102);
      if (lookahead == 'u') ADVANCE(103);
      END_STATE();
    case 21:
      if (lookahead == '3') ADVANCE(104);
      if (lookahead == 'e') ADVANCE(105);
      if (lookahead == 'g') ADVANCE(106);
      END_STATE();
    case 22:
      if (lookahead == 'a') ADVANCE(107);
      if (lookahead == 'e') ADVANCE(108);
      if (lookahead == 'h') ADVANCE(109);
      if (lookahead == 'i') ADVANCE(110);
      if (lookahead == 'm') ADVANCE(111);
      if (lookahead == 'n') ADVANCE(112);
      if (lookahead == 't') ADVANCE(113);
      if (lookahead == 'u') ADVANCE(114);
      if (lookahead == 'w') ADVANCE(115);
      END_STATE();
    case 23:
      if (lookahead == 'a') ADVANCE(116);
      if (lookahead == 'e') ADVANCE(117);
      if (lookahead == 'h') ADVANCE(118);
      if (lookahead == 'r') ADVANCE(119);
      if (lookahead == 'y') ADVANCE(120);
      END_STATE();
    case 24:
      if (lookahead == '3') ADVANCE(121);
      if (lookahead == 'n') ADVANCE(122);
      if (lookahead == 's') ADVANCE(123);
      END_STATE();
    case 25:
      if (lookahead == 'a') ADVANCE(124);
      if (lookahead == 'e') ADVANCE(125);
      if (lookahead == 'i') ADVANCE(126);
      if (lookahead == 'o') ADVANCE(127);
      END_STATE();
    case 26:
      if (lookahead == 'g') ADVANCE(128);
      if (lookahead == 'h') ADVANCE(129);
      if (lookahead == 'i') ADVANCE(130);
      if (lookahead == 'o') ADVANCE(131);
      if (lookahead == 'r') ADVANCE(132);
      END_STATE();
    case 27:
      if (lookahead == 'i') ADVANCE(133);
      END_STATE();
    case 28:
      if (lookahead == 'm') ADVANCE(134);
      END_STATE();
    case 29:
      if (lookahead == 'm') ADVANCE(135);
      END_STATE();
    case 30:
      if (lookahead == 'o') ADVANCE(136);
      END_STATE();
    case 31:
      if (lookahead == 'l') ADVANCE(137);
      END_STATE();
    case 32:
      if (lookahead == 'L') ADVANCE(138);
      END_STATE();
    case 33:
      if (lookahead == 'l') ADVANCE(139);
      END_STATE();
    case 34:
      if (lookahead == 's') ADVANCE(140);
      END_STATE();
    case 35:
      if (lookahead == 't') ADVANCE(141);
      END_STATE();
    case 36:
      if (lookahead == 'i') ADVANCE(142);
      END_STATE();
    case 37:
      if (lookahead == 'r') ADVANCE(143);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_as);
      if (lookahead == 'm') ADVANCE(144);
      if (lookahead == 'y') ADVANCE(145);
      END_STATE();
    case 39:
      if (lookahead == 'o') ADVANCE(146);
      if (lookahead == 't') ADVANCE(147);
      END_STATE();
    case 40:
      if (lookahead == 't') ADVANCE(148);
      END_STATE();
    case 41:
      if (lookahead == 'a') ADVANCE(149);
      END_STATE();
    case 42:
      if (lookahead == 'c') ADVANCE(150);
      END_STATE();
    case 43:
      if (lookahead == 'r') ADVANCE(151);
      END_STATE();
    case 44:
      if (lookahead == 'n') ADVANCE(152);
      if (lookahead == 't') ADVANCE(153);
      END_STATE();
    case 45:
      if (lookahead == 'o') ADVANCE(154);
      END_STATE();
    case 46:
      if (lookahead == 'e') ADVANCE(155);
      END_STATE();
    case 47:
      if (lookahead == 'i') ADVANCE(156);
      END_STATE();
    case 48:
      if (lookahead == 's') ADVANCE(157);
      if (lookahead == 't') ADVANCE(158);
      END_STATE();
    case 49:
      if (lookahead == 'n') ADVANCE(159);
      END_STATE();
    case 50:
      if (lookahead == 'a') ADVANCE(160);
      END_STATE();
    case 51:
      if (lookahead == '_') ADVANCE(161);
      if (lookahead == 'h') ADVANCE(162);
      if (lookahead == 'l') ADVANCE(163);
      if (lookahead == 'm') ADVANCE(164);
      if (lookahead == 'n') ADVANCE(165);
      END_STATE();
    case 52:
      if (lookahead == 'a') ADVANCE(166);
      END_STATE();
    case 53:
      if (lookahead == 'b') ADVANCE(167);
      if (lookahead == 'c') ADVANCE(168);
      if (lookahead == 'f') ADVANCE(169);
      if (lookahead == 'l') ADVANCE(170);
      if (lookahead == 'm') ADVANCE(171);
      END_STATE();
    case 54:
      if (lookahead == 's') ADVANCE(172);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(anon_sym_do);
      END_STATE();
    case 56:
      if (lookahead == 'n') ADVANCE(173);
      END_STATE();
    case 57:
      if (lookahead == 's') ADVANCE(174);
      END_STATE();
    case 58:
      if (lookahead == 'a') ADVANCE(175);
      if (lookahead == 'u') ADVANCE(176);
      END_STATE();
    case 59:
      if (lookahead == 'p') ADVANCE(177);
      if (lookahead == 't') ADVANCE(178);
      END_STATE();
    case 60:
      if (lookahead == '6') ADVANCE(179);
      END_STATE();
    case 61:
      if (lookahead == '2') ADVANCE(180);
      END_STATE();
    case 62:
      if (lookahead == 'l') ADVANCE(181);
      END_STATE();
    case 63:
      if (lookahead == 'l') ADVANCE(182);
      if (lookahead == 'n') ADVANCE(183);
      END_STATE();
    case 64:
      if (lookahead == 'a') ADVANCE(184);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(anon_sym_fn);
      END_STATE();
    case 66:
      if (lookahead == 'r') ADVANCE(185);
      END_STATE();
    case 67:
      if (lookahead == 'a') ADVANCE(186);
      if (lookahead == 'i') ADVANCE(187);
      if (lookahead == 'o') ADVANCE(188);
      END_STATE();
    case 68:
      if (lookahead == 'n') ADVANCE(189);
      END_STATE();
    case 69:
      if (lookahead == 'g') ADVANCE(190);
      END_STATE();
    case 70:
      if (lookahead == 't') ADVANCE(191);
      END_STATE();
    case 71:
      if (lookahead == 'o') ADVANCE(192);
      END_STATE();
    case 72:
      if (lookahead == 't') ADVANCE(193);
      END_STATE();
    case 73:
      if (lookahead == 'o') ADVANCE(194);
      END_STATE();
    case 74:
      if (lookahead == 'n') ADVANCE(195);
      END_STATE();
    case 75:
      if (lookahead == 'g') ADVANCE(196);
      END_STATE();
    case 76:
      if (lookahead == '2') ADVANCE(197);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(anon_sym_id);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(anon_sym_if);
      END_STATE();
    case 79:
      if (lookahead == 'p') ADVANCE(198);
      END_STATE();
    case 80:
      if (lookahead == 'l') ADVANCE(199);
      if (lookahead == 'o') ADVANCE(200);
      if (lookahead == 's') ADVANCE(201);
      if (lookahead == 't') ADVANCE(202);
      if (lookahead == 'v') ADVANCE(203);
      END_STATE();
    case 81:
      if (lookahead == 'y') ADVANCE(204);
      END_STATE();
    case 82:
      if (lookahead == 't') ADVANCE(205);
      END_STATE();
    case 83:
      if (lookahead == 'n') ADVANCE(206);
      END_STATE();
    case 84:
      if (lookahead == 'c') ADVANCE(207);
      if (lookahead == 'o') ADVANCE(208);
      if (lookahead == 'w') ADVANCE(209);
      END_STATE();
    case 85:
      if (lookahead == 'c') ADVANCE(210);
      if (lookahead == 't') ADVANCE(211);
      END_STATE();
    case 86:
      if (lookahead == 'd') ADVANCE(212);
      if (lookahead == 't') ADVANCE(213);
      END_STATE();
    case 87:
      if (lookahead == 'd') ADVANCE(214);
      if (lookahead == 'v') ADVANCE(215);
      END_STATE();
    case 88:
      if (lookahead == 't') ADVANCE(216);
      END_STATE();
    case 89:
      if (lookahead == 'm') ADVANCE(217);
      END_STATE();
    case 90:
      if (lookahead == 'w') ADVANCE(218);
      END_STATE();
    case 91:
      if (lookahead == 'l') ADVANCE(219);
      END_STATE();
    case 92:
      if (lookahead == 'e') ADVANCE(220);
      if (lookahead == 'i') ADVANCE(221);
      if (lookahead == 'p') ADVANCE(222);
      END_STATE();
    case 93:
      if (lookahead == 'l') ADVANCE(223);
      if (lookahead == 'm') ADVANCE(224);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(anon_sym_of);
      END_STATE();
    case 95:
      if (lookahead == 'e') ADVANCE(225);
      END_STATE();
    case 96:
      if (lookahead == 'e') ADVANCE(226);
      END_STATE();
    case 97:
      if (lookahead == 'c') ADVANCE(227);
      if (lookahead == 'r') ADVANCE(228);
      if (lookahead == 's') ADVANCE(229);
      if (lookahead == 't') ADVANCE(230);
      END_STATE();
    case 98:
      if (lookahead == 'r') ADVANCE(231);
      END_STATE();
    case 99:
      if (lookahead == 'x') ADVANCE(232);
      END_STATE();
    case 100:
      if (lookahead == 's') ADVANCE(233);
      END_STATE();
    case 101:
      if (lookahead == 'e') ADVANCE(234);
      if (lookahead == 'i') ADVANCE(235);
      if (lookahead == 'o') ADVANCE(236);
      END_STATE();
    case 102:
      if (lookahead == 'r') ADVANCE(237);
      END_STATE();
    case 103:
      if (lookahead == 'b') ADVANCE(238);
      END_STATE();
    case 104:
      if (lookahead == '2') ADVANCE(239);
      END_STATE();
    case 105:
      if (lookahead == 'a') ADVANCE(240);
      if (lookahead == 'f') ADVANCE(241);
      if (lookahead == 'g') ADVANCE(242);
      if (lookahead == 'i') ADVANCE(243);
      if (lookahead == 'q') ADVANCE(244);
      if (lookahead == 's') ADVANCE(245);
      if (lookahead == 't') ADVANCE(246);
      END_STATE();
    case 106:
      if (lookahead == '3') ADVANCE(247);
      if (lookahead == 'b') ADVANCE(248);
      END_STATE();
    case 107:
      if (lookahead == 'm') ADVANCE(249);
      END_STATE();
    case 108:
      if (lookahead == 'l') ADVANCE(250);
      if (lookahead == 't') ADVANCE(251);
      END_STATE();
    case 109:
      if (lookahead == 'a') ADVANCE(252);
      END_STATE();
    case 110:
      if (lookahead == 'g') ADVANCE(253);
      if (lookahead == 'z') ADVANCE(254);
      END_STATE();
    case 111:
      if (lookahead == 'o') ADVANCE(255);
      END_STATE();
    case 112:
      if (lookahead == 'o') ADVANCE(256);
      END_STATE();
    case 113:
      if (lookahead == 'a') ADVANCE(257);
      if (lookahead == 'd') ADVANCE(258);
      if (lookahead == 'o') ADVANCE(259);
      if (lookahead == 'r') ADVANCE(260);
      END_STATE();
    case 114:
      if (lookahead == 'b') ADVANCE(261);
      if (lookahead == 'p') ADVANCE(262);
      END_STATE();
    case 115:
      if (lookahead == 'i') ADVANCE(263);
      END_STATE();
    case 116:
      if (lookahead == 'r') ADVANCE(264);
      END_STATE();
    case 117:
      if (lookahead == 'm') ADVANCE(265);
      if (lookahead == 'x') ADVANCE(266);
      END_STATE();
    case 118:
      if (lookahead == 'i') ADVANCE(267);
      if (lookahead == 'r') ADVANCE(268);
      END_STATE();
    case 119:
      if (lookahead == 'a') ADVANCE(269);
      if (lookahead == 'u') ADVANCE(270);
      if (lookahead == 'y') ADVANCE(271);
      END_STATE();
    case 120:
      if (lookahead == 'p') ADVANCE(272);
      END_STATE();
    case 121:
      if (lookahead == '2') ADVANCE(273);
      END_STATE();
    case 122:
      if (lookahead == 'i') ADVANCE(274);
      if (lookahead == 'l') ADVANCE(275);
      if (lookahead == 'o') ADVANCE(276);
      if (lookahead == 's') ADVANCE(277);
      END_STATE();
    case 123:
      if (lookahead == 'e') ADVANCE(278);
      if (lookahead == 'i') ADVANCE(279);
      END_STATE();
    case 124:
      if (lookahead == 'r') ADVANCE(280);
      END_STATE();
    case 125:
      if (lookahead == 'c') ADVANCE(281);
      if (lookahead == 'r') ADVANCE(282);
      END_STATE();
    case 126:
      if (lookahead == 'r') ADVANCE(283);
      END_STATE();
    case 127:
      if (lookahead == 'l') ADVANCE(284);
      END_STATE();
    case 128:
      if (lookahead == 's') ADVANCE(285);
      END_STATE();
    case 129:
      if (lookahead == 'e') ADVANCE(286);
      if (lookahead == 'i') ADVANCE(287);
      END_STATE();
    case 130:
      if (lookahead == 't') ADVANCE(288);
      END_STATE();
    case 131:
      if (lookahead == 'r') ADVANCE(289);
      END_STATE();
    case 132:
      if (lookahead == 'i') ADVANCE(290);
      END_STATE();
    case 133:
      if (lookahead == 'e') ADVANCE(291);
      END_STATE();
    case 134:
      if (lookahead == 'p') ADVANCE(292);
      END_STATE();
    case 135:
      if (lookahead == 'a') ADVANCE(293);
      END_STATE();
    case 136:
      if (lookahead == 'm') ADVANCE(294);
      END_STATE();
    case 137:
      if (lookahead == 'l') ADVANCE(295);
      END_STATE();
    case 138:
      if (lookahead == 'L') ADVANCE(296);
      END_STATE();
    case 139:
      if (lookahead == 'f') ADVANCE(297);
      END_STATE();
    case 140:
      if (lookahead == 't') ADVANCE(298);
      END_STATE();
    case 141:
      if (lookahead == 'i') ADVANCE(299);
      END_STATE();
    case 142:
      if (lookahead == 'a') ADVANCE(300);
      if (lookahead == 'g') ADVANCE(301);
      END_STATE();
    case 143:
      if (lookahead == 'a') ADVANCE(302);
      END_STATE();
    case 144:
      ACCEPT_TOKEN(anon_sym_asm);
      if (lookahead == '_') ADVANCE(303);
      END_STATE();
    case 145:
      if (lookahead == 'n') ADVANCE(304);
      END_STATE();
    case 146:
      if (lookahead == 'm') ADVANCE(305);
      END_STATE();
    case 147:
      if (lookahead == 'r') ADVANCE(306);
      END_STATE();
    case 148:
      if (lookahead == 'o') ADVANCE(307);
      END_STATE();
    case 149:
      if (lookahead == 'i') ADVANCE(308);
      END_STATE();
    case 150:
      if (lookahead == 'o') ADVANCE(309);
      END_STATE();
    case 151:
      if (lookahead == 'a') ADVANCE(310);
      END_STATE();
    case 152:
      if (lookahead == 'd') ADVANCE(311);
      END_STATE();
    case 153:
      if (lookahead == 'c') ADVANCE(312);
      END_STATE();
    case 154:
      if (lookahead == 'l') ADVANCE(313);
      END_STATE();
    case 155:
      if (lookahead == 'a') ADVANCE(314);
      END_STATE();
    case 156:
      if (lookahead == 'l') ADVANCE(315);
      END_STATE();
    case 157:
      if (lookahead == 'e') ADVANCE(316);
      if (lookahead == 't') ADVANCE(317);
      END_STATE();
    case 158:
      if (lookahead == 'c') ADVANCE(318);
      END_STATE();
    case 159:
      if (lookahead == 't') ADVANCE(319);
      END_STATE();
    case 160:
      if (lookahead == 's') ADVANCE(320);
      END_STATE();
    case 161:
      if (lookahead == 'a') ADVANCE(321);
      if (lookahead == 'r') ADVANCE(322);
      if (lookahead == 'y') ADVANCE(323);
      END_STATE();
    case 162:
      if (lookahead == 'e') ADVANCE(324);
      END_STATE();
    case 163:
      if (lookahead == 'u') ADVANCE(325);
      END_STATE();
    case 164:
      if (lookahead == 'm') ADVANCE(326);
      if (lookahead == 'p') ADVANCE(327);
      END_STATE();
    case 165:
      if (lookahead == 'c') ADVANCE(328);
      if (lookahead == 's') ADVANCE(329);
      if (lookahead == 't') ADVANCE(330);
      END_STATE();
    case 166:
      if (lookahead == 't') ADVANCE(331);
      END_STATE();
    case 167:
      if (lookahead == 'u') ADVANCE(332);
      END_STATE();
    case 168:
      if (lookahead == 'l') ADVANCE(333);
      END_STATE();
    case 169:
      if (lookahead == 'a') ADVANCE(334);
      END_STATE();
    case 170:
      if (lookahead == 'e') ADVANCE(335);
      END_STATE();
    case 171:
      if (lookahead == 'o') ADVANCE(336);
      END_STATE();
    case 172:
      if (lookahead == 'c') ADVANCE(337);
      END_STATE();
    case 173:
      if (lookahead == 'a') ADVANCE(338);
      END_STATE();
    case 174:
      if (lookahead == 'e') ADVANCE(339);
      END_STATE();
    case 175:
      if (lookahead == 'b') ADVANCE(340);
      END_STATE();
    case 176:
      if (lookahead == 'm') ADVANCE(341);
      END_STATE();
    case 177:
      if (lookahead == 'l') ADVANCE(342);
      if (lookahead == 'o') ADVANCE(343);
      END_STATE();
    case 178:
      if (lookahead == 'e') ADVANCE(344);
      END_STATE();
    case 179:
      ACCEPT_TOKEN(anon_sym_f16);
      END_STATE();
    case 180:
      ACCEPT_TOKEN(anon_sym_f32);
      END_STATE();
    case 181:
      if (lookahead == 'l') ADVANCE(345);
      if (lookahead == 's') ADVANCE(346);
      END_STATE();
    case 182:
      if (lookahead == 't') ADVANCE(347);
      END_STATE();
    case 183:
      if (lookahead == 'a') ADVANCE(348);
      END_STATE();
    case 184:
      if (lookahead == 't') ADVANCE(349);
      END_STATE();
    case 185:
      ACCEPT_TOKEN(anon_sym_for);
      END_STATE();
    case 186:
      if (lookahead == 'g') ADVANCE(350);
      END_STATE();
    case 187:
      if (lookahead == 'e') ADVANCE(351);
      END_STATE();
    case 188:
      if (lookahead == 'm') ADVANCE(352);
      if (lookahead == 'n') ADVANCE(353);
      END_STATE();
    case 189:
      if (lookahead == 'c') ADVANCE(354);
      END_STATE();
    case 190:
      if (lookahead == 'r') ADVANCE(355);
      END_STATE();
    case 191:
      ACCEPT_TOKEN(anon_sym_get);
      END_STATE();
    case 192:
      if (lookahead == 'b') ADVANCE(356);
      END_STATE();
    case 193:
      if (lookahead == 'o') ADVANCE(357);
      END_STATE();
    case 194:
      if (lookahead == 'u') ADVANCE(358);
      END_STATE();
    case 195:
      if (lookahead == 'd') ADVANCE(359);
      END_STATE();
    case 196:
      if (lookahead == 'h') ADVANCE(360);
      END_STATE();
    case 197:
      ACCEPT_TOKEN(anon_sym_i32);
      END_STATE();
    case 198:
      if (lookahead == 'l') ADVANCE(361);
      if (lookahead == 'o') ADVANCE(362);
      END_STATE();
    case 199:
      if (lookahead == 'i') ADVANCE(363);
      END_STATE();
    case 200:
      if (lookahead == 'u') ADVANCE(364);
      END_STATE();
    case 201:
      if (lookahead == 't') ADVANCE(365);
      END_STATE();
    case 202:
      if (lookahead == 'e') ADVANCE(366);
      END_STATE();
    case 203:
      if (lookahead == 'a') ADVANCE(367);
      END_STATE();
    case 204:
      if (lookahead == 'o') ADVANCE(368);
      END_STATE();
    case 205:
      ACCEPT_TOKEN(anon_sym_let);
      END_STATE();
    case 206:
      if (lookahead == 'e') ADVANCE(369);
      END_STATE();
    case 207:
      if (lookahead == 'a') ADVANCE(370);
      END_STATE();
    case 208:
      if (lookahead == 'p') ADVANCE(371);
      END_STATE();
    case 209:
      if (lookahead == 'p') ADVANCE(372);
      END_STATE();
    case 210:
      if (lookahead == 'r') ADVANCE(373);
      END_STATE();
    case 211:
      if (lookahead == '2') ADVANCE(374);
      if (lookahead == '3') ADVANCE(375);
      if (lookahead == '4') ADVANCE(376);
      if (lookahead == 'c') ADVANCE(377);
      END_STATE();
    case 212:
      if (lookahead == 'i') ADVANCE(378);
      END_STATE();
    case 213:
      if (lookahead == 'a') ADVANCE(379);
      END_STATE();
    case 214:
      ACCEPT_TOKEN(anon_sym_mod);
      if (lookahead == 'u') ADVANCE(380);
      END_STATE();
    case 215:
      if (lookahead == 'e') ADVANCE(381);
      END_STATE();
    case 216:
      ACCEPT_TOKEN(anon_sym_mut);
      if (lookahead == 'a') ADVANCE(382);
      END_STATE();
    case 217:
      if (lookahead == 'e') ADVANCE(383);
      END_STATE();
    case 218:
      ACCEPT_TOKEN(anon_sym_new);
      END_STATE();
    case 219:
      ACCEPT_TOKEN(anon_sym_nil);
      END_STATE();
    case 220:
      if (lookahead == 'x') ADVANCE(384);
      END_STATE();
    case 221:
      if (lookahead == 'n') ADVANCE(385);
      END_STATE();
    case 222:
      if (lookahead == 'e') ADVANCE(386);
      END_STATE();
    case 223:
      if (lookahead == 'l') ADVANCE(387);
      END_STATE();
    case 224:
      if (lookahead == '_') ADVANCE(388);
      END_STATE();
    case 225:
      if (lookahead == 'r') ADVANCE(389);
      END_STATE();
    case 226:
      if (lookahead == 'r') ADVANCE(390);
      END_STATE();
    case 227:
      if (lookahead == 'k') ADVANCE(391);
      END_STATE();
    case 228:
      if (lookahead == 't') ADVANCE(392);
      END_STATE();
    case 229:
      if (lookahead == 's') ADVANCE(393);
      END_STATE();
    case 230:
      if (lookahead == 'c') ADVANCE(394);
      END_STATE();
    case 231:
      if (lookahead == 's') ADVANCE(395);
      END_STATE();
    case 232:
      if (lookahead == 'e') ADVANCE(396);
      END_STATE();
    case 233:
      if (lookahead == 'i') ADVANCE(397);
      END_STATE();
    case 234:
      if (lookahead == 'c') ADVANCE(398);
      if (lookahead == 'm') ADVANCE(399);
      END_STATE();
    case 235:
      if (lookahead == 'v') ADVANCE(400);
      END_STATE();
    case 236:
      if (lookahead == 't') ADVANCE(401);
      END_STATE();
    case 237:
      ACCEPT_TOKEN(anon_sym_ptr);
      END_STATE();
    case 238:
      ACCEPT_TOKEN(anon_sym_pub);
      if (lookahead == 'l') ADVANCE(402);
      END_STATE();
    case 239:
      if (lookahead == 'f') ADVANCE(403);
      if (lookahead == 's') ADVANCE(404);
      if (lookahead == 'u') ADVANCE(405);
      END_STATE();
    case 240:
      if (lookahead == 'd') ADVANCE(406);
      END_STATE();
    case 241:
      ACCEPT_TOKEN(anon_sym_ref);
      END_STATE();
    case 242:
      if (lookahead == 'a') ADVANCE(407);
      if (lookahead == 'i') ADVANCE(408);
      END_STATE();
    case 243:
      if (lookahead == 'n') ADVANCE(409);
      END_STATE();
    case 244:
      if (lookahead == 'u') ADVANCE(410);
      END_STATE();
    case 245:
      if (lookahead == 'o') ADVANCE(411);
      if (lookahead == 't') ADVANCE(412);
      END_STATE();
    case 246:
      if (lookahead == 'u') ADVANCE(413);
      END_STATE();
    case 247:
      if (lookahead == '2') ADVANCE(414);
      END_STATE();
    case 248:
      if (lookahead == 'a') ADVANCE(415);
      END_STATE();
    case 249:
      if (lookahead == 'p') ADVANCE(416);
      END_STATE();
    case 250:
      if (lookahead == 'f') ADVANCE(417);
      END_STATE();
    case 251:
      ACCEPT_TOKEN(anon_sym_set);
      END_STATE();
    case 252:
      if (lookahead == 'r') ADVANCE(418);
      END_STATE();
    case 253:
      if (lookahead == 'n') ADVANCE(419);
      END_STATE();
    case 254:
      if (lookahead == 'e') ADVANCE(420);
      END_STATE();
    case 255:
      if (lookahead == 'o') ADVANCE(421);
      END_STATE();
    case 256:
      if (lookahead == 'r') ADVANCE(422);
      END_STATE();
    case 257:
      if (lookahead == 't') ADVANCE(423);
      END_STATE();
    case 258:
      ACCEPT_TOKEN(anon_sym_std);
      END_STATE();
    case 259:
      if (lookahead == 'r') ADVANCE(424);
      END_STATE();
    case 260:
      if (lookahead == 'u') ADVANCE(425);
      END_STATE();
    case 261:
      if (lookahead == 'r') ADVANCE(426);
      END_STATE();
    case 262:
      if (lookahead == 'e') ADVANCE(427);
      END_STATE();
    case 263:
      if (lookahead == 't') ADVANCE(428);
      END_STATE();
    case 264:
      if (lookahead == 'g') ADVANCE(429);
      END_STATE();
    case 265:
      if (lookahead == 'p') ADVANCE(430);
      END_STATE();
    case 266:
      if (lookahead == 't') ADVANCE(431);
      END_STATE();
    case 267:
      if (lookahead == 's') ADVANCE(432);
      END_STATE();
    case 268:
      if (lookahead == 'e') ADVANCE(433);
      if (lookahead == 'o') ADVANCE(434);
      END_STATE();
    case 269:
      if (lookahead == 'i') ADVANCE(435);
      END_STATE();
    case 270:
      if (lookahead == 'e') ADVANCE(436);
      END_STATE();
    case 271:
      ACCEPT_TOKEN(anon_sym_try);
      END_STATE();
    case 272:
      if (lookahead == 'e') ADVANCE(437);
      END_STATE();
    case 273:
      ACCEPT_TOKEN(anon_sym_u32);
      END_STATE();
    case 274:
      if (lookahead == 'f') ADVANCE(438);
      if (lookahead == 'o') ADVANCE(439);
      END_STATE();
    case 275:
      if (lookahead == 'e') ADVANCE(440);
      END_STATE();
    case 276:
      if (lookahead == 'r') ADVANCE(441);
      END_STATE();
    case 277:
      if (lookahead == 'a') ADVANCE(442);
      if (lookahead == 'i') ADVANCE(443);
      END_STATE();
    case 278:
      ACCEPT_TOKEN(anon_sym_use);
      END_STATE();
    case 279:
      if (lookahead == 'n') ADVANCE(444);
      END_STATE();
    case 280:
      ACCEPT_TOKEN(anon_sym_var);
      if (lookahead == 'y') ADVANCE(445);
      END_STATE();
    case 281:
      if (lookahead == '2') ADVANCE(446);
      if (lookahead == '3') ADVANCE(447);
      if (lookahead == '4') ADVANCE(448);
      END_STATE();
    case 282:
      if (lookahead == 't') ADVANCE(449);
      END_STATE();
    case 283:
      if (lookahead == 't') ADVANCE(450);
      END_STATE();
    case 284:
      if (lookahead == 'a') ADVANCE(451);
      END_STATE();
    case 285:
      if (lookahead == 'l') ADVANCE(452);
      END_STATE();
    case 286:
      if (lookahead == 'r') ADVANCE(453);
      END_STATE();
    case 287:
      if (lookahead == 'l') ADVANCE(454);
      END_STATE();
    case 288:
      if (lookahead == 'h') ADVANCE(455);
      END_STATE();
    case 289:
      if (lookahead == 'k') ADVANCE(456);
      END_STATE();
    case 290:
      if (lookahead == 't') ADVANCE(457);
      END_STATE();
    case 291:
      if (lookahead == 'l') ADVANCE(458);
      END_STATE();
    case 292:
      if (lookahead == 'i') ADVANCE(459);
      if (lookahead == 'u') ADVANCE(460);
      END_STATE();
    case 293:
      if (lookahead == 'i') ADVANCE(461);
      END_STATE();
    case 294:
      if (lookahead == 'e') ADVANCE(462);
      END_STATE();
    case 295:
      if (lookahead == 's') ADVANCE(463);
      END_STATE();
    case 296:
      ACCEPT_TOKEN(anon_sym_NULL);
      END_STATE();
    case 297:
      ACCEPT_TOKEN(anon_sym_Self);
      END_STATE();
    case 298:
      if (lookahead == 'r') ADVANCE(464);
      END_STATE();
    case 299:
      if (lookahead == 'v') ADVANCE(465);
      END_STATE();
    case 300:
      if (lookahead == 's') ADVANCE(466);
      END_STATE();
    case 301:
      if (lookahead == 'n') ADVANCE(467);
      END_STATE();
    case 302:
      if (lookahead == 'y') ADVANCE(468);
      END_STATE();
    case 303:
      if (lookahead == 'f') ADVANCE(469);
      END_STATE();
    case 304:
      if (lookahead == 'c') ADVANCE(470);
      END_STATE();
    case 305:
      if (lookahead == 'i') ADVANCE(471);
      END_STATE();
    case 306:
      if (lookahead == 'i') ADVANCE(472);
      END_STATE();
    case 307:
      ACCEPT_TOKEN(anon_sym_auto);
      END_STATE();
    case 308:
      if (lookahead == 't') ADVANCE(473);
      END_STATE();
    case 309:
      if (lookahead == 'm') ADVANCE(474);
      END_STATE();
    case 310:
      if (lookahead == '8') ADVANCE(475);
      END_STATE();
    case 311:
      if (lookahead == 'i') ADVANCE(476);
      END_STATE();
    case 312:
      if (lookahead == 'a') ADVANCE(477);
      END_STATE();
    case 313:
      ACCEPT_TOKEN(anon_sym_bool);
      END_STATE();
    case 314:
      if (lookahead == 'k') ADVANCE(478);
      END_STATE();
    case 315:
      if (lookahead == 't') ADVANCE(479);
      END_STATE();
    case 316:
      ACCEPT_TOKEN(anon_sym_case);
      END_STATE();
    case 317:
      ACCEPT_TOKEN(anon_sym_cast);
      END_STATE();
    case 318:
      if (lookahead == 'h') ADVANCE(480);
      END_STATE();
    case 319:
      if (lookahead == 'e') ADVANCE(481);
      if (lookahead == 'r') ADVANCE(482);
      END_STATE();
    case 320:
      if (lookahead == 's') ADVANCE(483);
      END_STATE();
    case 321:
      if (lookahead == 'w') ADVANCE(484);
      END_STATE();
    case 322:
      if (lookahead == 'e') ADVANCE(485);
      END_STATE();
    case 323:
      if (lookahead == 'i') ADVANCE(486);
      END_STATE();
    case 324:
      if (lookahead == 'r') ADVANCE(487);
      END_STATE();
    case 325:
      if (lookahead == 'm') ADVANCE(488);
      END_STATE();
    case 326:
      if (lookahead == 'o') ADVANCE(489);
      END_STATE();
    case 327:
      if (lookahead == 'i') ADVANCE(490);
      if (lookahead == 'u') ADVANCE(491);
      END_STATE();
    case 328:
      if (lookahead == 'e') ADVANCE(492);
      END_STATE();
    case 329:
      if (lookahead == 't') ADVANCE(493);
      END_STATE();
    case 330:
      if (lookahead == 'i') ADVANCE(494);
      END_STATE();
    case 331:
      if (lookahead == 'e') ADVANCE(495);
      END_STATE();
    case 332:
      if (lookahead == 'g') ADVANCE(496);
      END_STATE();
    case 333:
      if (lookahead == 't') ADVANCE(497);
      END_STATE();
    case 334:
      if (lookahead == 'u') ADVANCE(498);
      END_STATE();
    case 335:
      if (lookahead == 't') ADVANCE(499);
      END_STATE();
    case 336:
      if (lookahead == 't') ADVANCE(500);
      END_STATE();
    case 337:
      if (lookahead == 'a') ADVANCE(501);
      END_STATE();
    case 338:
      if (lookahead == 'm') ADVANCE(502);
      END_STATE();
    case 339:
      ACCEPT_TOKEN(anon_sym_else);
      END_STATE();
    case 340:
      if (lookahead == 'l') ADVANCE(503);
      END_STATE();
    case 341:
      ACCEPT_TOKEN(anon_sym_enum);
      END_STATE();
    case 342:
      if (lookahead == 'i') ADVANCE(504);
      END_STATE();
    case 343:
      if (lookahead == 'r') ADVANCE(505);
      END_STATE();
    case 344:
      if (lookahead == 'n') ADVANCE(506);
      if (lookahead == 'r') ADVANCE(507);
      END_STATE();
    case 345:
      if (lookahead == 't') ADVANCE(508);
      END_STATE();
    case 346:
      if (lookahead == 'e') ADVANCE(509);
      END_STATE();
    case 347:
      if (lookahead == 'e') ADVANCE(510);
      END_STATE();
    case 348:
      if (lookahead == 'l') ADVANCE(511);
      END_STATE();
    case 349:
      ACCEPT_TOKEN(anon_sym_flat);
      END_STATE();
    case 350:
      if (lookahead == '_') ADVANCE(512);
      if (lookahead == 'm') ADVANCE(513);
      END_STATE();
    case 351:
      if (lookahead == 'n') ADVANCE(514);
      END_STATE();
    case 352:
      ACCEPT_TOKEN(anon_sym_from);
      END_STATE();
    case 353:
      if (lookahead == 't') ADVANCE(515);
      END_STATE();
    case 354:
      if (lookahead == 't') ADVANCE(516);
      END_STATE();
    case 355:
      if (lookahead == 'o') ADVANCE(517);
      END_STATE();
    case 356:
      if (lookahead == 'a') ADVANCE(518);
      END_STATE();
    case 357:
      ACCEPT_TOKEN(anon_sym_goto);
      END_STATE();
    case 358:
      if (lookahead == 'p') ADVANCE(519);
      END_STATE();
    case 359:
      if (lookahead == 'l') ADVANCE(520);
      END_STATE();
    case 360:
      if (lookahead == 'p') ADVANCE(521);
      END_STATE();
    case 361:
      ACCEPT_TOKEN(anon_sym_impl);
      if (lookahead == 'e') ADVANCE(522);
      END_STATE();
    case 362:
      if (lookahead == 'r') ADVANCE(523);
      END_STATE();
    case 363:
      if (lookahead == 'n') ADVANCE(524);
      END_STATE();
    case 364:
      if (lookahead == 't') ADVANCE(525);
      END_STATE();
    case 365:
      if (lookahead == 'a') ADVANCE(526);
      END_STATE();
    case 366:
      if (lookahead == 'r') ADVANCE(527);
      END_STATE();
    case 367:
      if (lookahead == 'r') ADVANCE(528);
      END_STATE();
    case 368:
      if (lookahead == 'u') ADVANCE(529);
      END_STATE();
    case 369:
      if (lookahead == 'a') ADVANCE(530);
      END_STATE();
    case 370:
      if (lookahead == 'l') ADVANCE(531);
      if (lookahead == 't') ADVANCE(532);
      END_STATE();
    case 371:
      ACCEPT_TOKEN(anon_sym_loop);
      END_STATE();
    case 372:
      ACCEPT_TOKEN(anon_sym_lowp);
      END_STATE();
    case 373:
      if (lookahead == 'o') ADVANCE(533);
      END_STATE();
    case 374:
      if (lookahead == 'x') ADVANCE(534);
      END_STATE();
    case 375:
      if (lookahead == 'x') ADVANCE(535);
      END_STATE();
    case 376:
      if (lookahead == 'x') ADVANCE(536);
      END_STATE();
    case 377:
      if (lookahead == 'h') ADVANCE(537);
      END_STATE();
    case 378:
      if (lookahead == 'u') ADVANCE(538);
      END_STATE();
    case 379:
      ACCEPT_TOKEN(anon_sym_meta);
      END_STATE();
    case 380:
      if (lookahead == 'l') ADVANCE(539);
      END_STATE();
    case 381:
      ACCEPT_TOKEN(anon_sym_move);
      END_STATE();
    case 382:
      if (lookahead == 'b') ADVANCE(540);
      END_STATE();
    case 383:
      if (lookahead == 's') ADVANCE(541);
      END_STATE();
    case 384:
      if (lookahead == 'c') ADVANCE(542);
      END_STATE();
    case 385:
      if (lookahead == 'l') ADVANCE(543);
      if (lookahead == 't') ADVANCE(544);
      END_STATE();
    case 386:
      if (lookahead == 'r') ADVANCE(545);
      END_STATE();
    case 387:
      ACCEPT_TOKEN(anon_sym_null);
      if (lookahead == 'p') ADVANCE(546);
      END_STATE();
    case 388:
      if (lookahead == 'w') ADVANCE(547);
      END_STATE();
    case 389:
      if (lookahead == 'a') ADVANCE(548);
      END_STATE();
    case 390:
      if (lookahead == 'r') ADVANCE(549);
      END_STATE();
    case 391:
      if (lookahead == 'a') ADVANCE(550);
      if (lookahead == 'o') ADVANCE(551);
      END_STATE();
    case 392:
      if (lookahead == 'i') ADVANCE(552);
      END_STATE();
    case 393:
      ACCEPT_TOKEN(anon_sym_pass);
      END_STATE();
    case 394:
      if (lookahead == 'h') ADVANCE(553);
      END_STATE();
    case 395:
      if (lookahead == 'p') ADVANCE(554);
      END_STATE();
    case 396:
      if (lookahead == 'l') ADVANCE(555);
      END_STATE();
    case 397:
      if (lookahead == 't') ADVANCE(556);
      END_STATE();
    case 398:
      if (lookahead == 'i') ADVANCE(557);
      END_STATE();
    case 399:
      if (lookahead == 'e') ADVANCE(558);
      END_STATE();
    case 400:
      ACCEPT_TOKEN(anon_sym_priv);
      if (lookahead == 'a') ADVANCE(559);
      END_STATE();
    case 401:
      if (lookahead == 'e') ADVANCE(560);
      END_STATE();
    case 402:
      if (lookahead == 'i') ADVANCE(561);
      END_STATE();
    case 403:
      if (lookahead == 'l') ADVANCE(562);
      END_STATE();
    case 404:
      if (lookahead == 'i') ADVANCE(563);
      END_STATE();
    case 405:
      if (lookahead == 'i') ADVANCE(564);
      END_STATE();
    case 406:
      ACCEPT_TOKEN(anon_sym_read);
      if (lookahead == '_') ADVANCE(565);
      if (lookahead == 'o') ADVANCE(566);
      END_STATE();
    case 407:
      if (lookahead == 'r') ADVANCE(567);
      END_STATE();
    case 408:
      if (lookahead == 's') ADVANCE(568);
      END_STATE();
    case 409:
      if (lookahead == 't') ADVANCE(569);
      END_STATE();
    case 410:
      if (lookahead == 'i') ADVANCE(570);
      END_STATE();
    case 411:
      if (lookahead == 'u') ADVANCE(571);
      END_STATE();
    case 412:
      if (lookahead == 'r') ADVANCE(572);
      END_STATE();
    case 413:
      if (lookahead == 'r') ADVANCE(573);
      END_STATE();
    case 414:
      if (lookahead == 'f') ADVANCE(574);
      if (lookahead == 's') ADVANCE(575);
      if (lookahead == 'u') ADVANCE(576);
      END_STATE();
    case 415:
      if (lookahead == '1') ADVANCE(577);
      if (lookahead == '3') ADVANCE(578);
      if (lookahead == '8') ADVANCE(579);
      END_STATE();
    case 416:
      if (lookahead == 'l') ADVANCE(580);
      END_STATE();
    case 417:
      ACCEPT_TOKEN(anon_sym_self);
      END_STATE();
    case 418:
      if (lookahead == 'e') ADVANCE(581);
      END_STATE();
    case 419:
      if (lookahead == 'e') ADVANCE(582);
      END_STATE();
    case 420:
      ACCEPT_TOKEN(anon_sym_size);
      if (lookahead == 'o') ADVANCE(583);
      END_STATE();
    case 421:
      if (lookahead == 't') ADVANCE(584);
      END_STATE();
    case 422:
      if (lookahead == 'm') ADVANCE(585);
      END_STATE();
    case 423:
      if (lookahead == 'i') ADVANCE(586);
      END_STATE();
    case 424:
      if (lookahead == 'a') ADVANCE(587);
      END_STATE();
    case 425:
      if (lookahead == 'c') ADVANCE(588);
      END_STATE();
    case 426:
      if (lookahead == 'o') ADVANCE(589);
      END_STATE();
    case 427:
      if (lookahead == 'r') ADVANCE(590);
      END_STATE();
    case 428:
      if (lookahead == 'c') ADVANCE(591);
      END_STATE();
    case 429:
      if (lookahead == 'e') ADVANCE(592);
      END_STATE();
    case 430:
      if (lookahead == 'l') ADVANCE(593);
      END_STATE();
    case 431:
      if (lookahead == 'u') ADVANCE(594);
      END_STATE();
    case 432:
      ACCEPT_TOKEN(anon_sym_this);
      END_STATE();
    case 433:
      if (lookahead == 'a') ADVANCE(595);
      END_STATE();
    case 434:
      if (lookahead == 'w') ADVANCE(596);
      END_STATE();
    case 435:
      if (lookahead == 't') ADVANCE(597);
      END_STATE();
    case 436:
      ACCEPT_TOKEN(anon_sym_true);
      END_STATE();
    case 437:
      ACCEPT_TOKEN(anon_sym_type);
      if (lookahead == 'd') ADVANCE(598);
      if (lookahead == 'i') ADVANCE(599);
      if (lookahead == 'n') ADVANCE(600);
      if (lookahead == 'o') ADVANCE(601);
      END_STATE();
    case 438:
      if (lookahead == 'o') ADVANCE(602);
      END_STATE();
    case 439:
      if (lookahead == 'n') ADVANCE(603);
      END_STATE();
    case 440:
      if (lookahead == 's') ADVANCE(604);
      END_STATE();
    case 441:
      if (lookahead == 'm') ADVANCE(605);
      END_STATE();
    case 442:
      if (lookahead == 'f') ADVANCE(606);
      END_STATE();
    case 443:
      if (lookahead == 'z') ADVANCE(607);
      END_STATE();
    case 444:
      if (lookahead == 'g') ADVANCE(608);
      END_STATE();
    case 445:
      if (lookahead == 'i') ADVANCE(609);
      END_STATE();
    case 446:
      ACCEPT_TOKEN(anon_sym_vec2);
      END_STATE();
    case 447:
      ACCEPT_TOKEN(anon_sym_vec3);
      END_STATE();
    case 448:
      ACCEPT_TOKEN(anon_sym_vec4);
      END_STATE();
    case 449:
      if (lookahead == 'e') ADVANCE(610);
      END_STATE();
    case 450:
      if (lookahead == 'u') ADVANCE(611);
      END_STATE();
    case 451:
      if (lookahead == 't') ADVANCE(612);
      END_STATE();
    case 452:
      ACCEPT_TOKEN(anon_sym_wgsl);
      END_STATE();
    case 453:
      if (lookahead == 'e') ADVANCE(613);
      END_STATE();
    case 454:
      if (lookahead == 'e') ADVANCE(614);
      END_STATE();
    case 455:
      ACCEPT_TOKEN(anon_sym_with);
      END_STATE();
    case 456:
      if (lookahead == 'g') ADVANCE(615);
      END_STATE();
    case 457:
      if (lookahead == 'e') ADVANCE(616);
      END_STATE();
    case 458:
      if (lookahead == 'd') ADVANCE(617);
      END_STATE();
    case 459:
      if (lookahead == 'l') ADVANCE(618);
      END_STATE();
    case 460:
      if (lookahead == 't') ADVANCE(619);
      END_STATE();
    case 461:
      if (lookahead == 'n') ADVANCE(620);
      END_STATE();
    case 462:
      if (lookahead == 't') ADVANCE(621);
      END_STATE();
    case 463:
      if (lookahead == 'h') ADVANCE(622);
      END_STATE();
    case 464:
      if (lookahead == 'a') ADVANCE(623);
      END_STATE();
    case 465:
      if (lookahead == 'e') ADVANCE(624);
      END_STATE();
    case 466:
      ACCEPT_TOKEN(anon_sym_alias);
      END_STATE();
    case 467:
      ACCEPT_TOKEN(anon_sym_align);
      if (lookahead == 'a') ADVANCE(625);
      if (lookahead == 'o') ADVANCE(626);
      END_STATE();
    case 468:
      ACCEPT_TOKEN(anon_sym_array);
      END_STATE();
    case 469:
      if (lookahead == 'r') ADVANCE(627);
      END_STATE();
    case 470:
      ACCEPT_TOKEN(anon_sym_async);
      END_STATE();
    case 471:
      if (lookahead == 'c') ADVANCE(628);
      END_STATE();
    case 472:
      if (lookahead == 'b') ADVANCE(629);
      END_STATE();
    case 473:
      ACCEPT_TOKEN(anon_sym_await);
      END_STATE();
    case 474:
      if (lookahead == 'e') ADVANCE(630);
      END_STATE();
    case 475:
      if (lookahead == 'u') ADVANCE(631);
      END_STATE();
    case 476:
      if (lookahead == 'n') ADVANCE(632);
      END_STATE();
    case 477:
      if (lookahead == 's') ADVANCE(633);
      END_STATE();
    case 478:
      ACCEPT_TOKEN(anon_sym_break);
      END_STATE();
    case 479:
      if (lookahead == 'i') ADVANCE(634);
      END_STATE();
    case 480:
      ACCEPT_TOKEN(anon_sym_catch);
      END_STATE();
    case 481:
      if (lookahead == 'r') ADVANCE(635);
      END_STATE();
    case 482:
      if (lookahead == 'o') ADVANCE(636);
      END_STATE();
    case 483:
      ACCEPT_TOKEN(anon_sym_class);
      END_STATE();
    case 484:
      if (lookahead == 'a') ADVANCE(637);
      END_STATE();
    case 485:
      if (lookahead == 't') ADVANCE(638);
      END_STATE();
    case 486:
      if (lookahead == 'e') ADVANCE(639);
      END_STATE();
    case 487:
      if (lookahead == 'e') ADVANCE(640);
      END_STATE();
    case 488:
      if (lookahead == 'n') ADVANCE(641);
      END_STATE();
    case 489:
      if (lookahead == 'n') ADVANCE(642);
      END_STATE();
    case 490:
      if (lookahead == 'l') ADVANCE(643);
      END_STATE();
    case 491:
      if (lookahead == 't') ADVANCE(644);
      END_STATE();
    case 492:
      if (lookahead == 'p') ADVANCE(645);
      END_STATE();
    case 493:
      ACCEPT_TOKEN(anon_sym_const);
      if (lookahead == '_') ADVANCE(646);
      if (lookahead == 'e') ADVANCE(647);
      if (lookahead == 'i') ADVANCE(648);
      END_STATE();
    case 494:
      if (lookahead == 'n') ADVANCE(649);
      END_STATE();
    case 495:
      ACCEPT_TOKEN(anon_sym_crate);
      END_STATE();
    case 496:
      if (lookahead == 'g') ADVANCE(650);
      END_STATE();
    case 497:
      if (lookahead == 'y') ADVANCE(651);
      END_STATE();
    case 498:
      if (lookahead == 'l') ADVANCE(652);
      END_STATE();
    case 499:
      if (lookahead == 'e') ADVANCE(653);
      END_STATE();
    case 500:
      if (lookahead == 'e') ADVANCE(654);
      END_STATE();
    case 501:
      if (lookahead == 'r') ADVANCE(655);
      END_STATE();
    case 502:
      if (lookahead == 'i') ADVANCE(656);
      END_STATE();
    case 503:
      if (lookahead == 'e') ADVANCE(657);
      END_STATE();
    case 504:
      if (lookahead == 'c') ADVANCE(658);
      END_STATE();
    case 505:
      if (lookahead == 't') ADVANCE(659);
      END_STATE();
    case 506:
      if (lookahead == 'd') ADVANCE(660);
      END_STATE();
    case 507:
      if (lookahead == 'n') ADVANCE(661);
      END_STATE();
    case 508:
      if (lookahead == 'h') ADVANCE(662);
      END_STATE();
    case 509:
      ACCEPT_TOKEN(anon_sym_false);
      END_STATE();
    case 510:
      if (lookahead == 'r') ADVANCE(663);
      END_STATE();
    case 511:
      ACCEPT_TOKEN(anon_sym_final);
      if (lookahead == 'l') ADVANCE(664);
      END_STATE();
    case 512:
      if (lookahead == 'd') ADVANCE(665);
      END_STATE();
    case 513:
      if (lookahead == 'e') ADVANCE(666);
      END_STATE();
    case 514:
      if (lookahead == 'd') ADVANCE(667);
      END_STATE();
    case 515:
      if (lookahead == '_') ADVANCE(668);
      END_STATE();
    case 516:
      if (lookahead == 'i') ADVANCE(669);
      END_STATE();
    case 517:
      if (lookahead == 'u') ADVANCE(670);
      END_STATE();
    case 518:
      if (lookahead == 'l') ADVANCE(671);
      END_STATE();
    case 519:
      ACCEPT_TOKEN(anon_sym_group);
      if (lookahead == 's') ADVANCE(672);
      END_STATE();
    case 520:
      if (lookahead == 'e') ADVANCE(673);
      END_STATE();
    case 521:
      ACCEPT_TOKEN(anon_sym_highp);
      END_STATE();
    case 522:
      if (lookahead == 'm') ADVANCE(674);
      END_STATE();
    case 523:
      if (lookahead == 't') ADVANCE(675);
      END_STATE();
    case 524:
      if (lookahead == 'e') ADVANCE(676);
      END_STATE();
    case 525:
      ACCEPT_TOKEN(anon_sym_inout);
      END_STATE();
    case 526:
      if (lookahead == 'n') ADVANCE(677);
      END_STATE();
    case 527:
      if (lookahead == 'f') ADVANCE(678);
      if (lookahead == 'p') ADVANCE(679);
      END_STATE();
    case 528:
      if (lookahead == 'i') ADVANCE(680);
      END_STATE();
    case 529:
      if (lookahead == 't') ADVANCE(681);
      END_STATE();
    case 530:
      if (lookahead == 'r') ADVANCE(682);
      END_STATE();
    case 531:
      if (lookahead == '_') ADVANCE(683);
      END_STATE();
    case 532:
      if (lookahead == 'i') ADVANCE(684);
      END_STATE();
    case 533:
      ACCEPT_TOKEN(anon_sym_macro);
      if (lookahead == '_') ADVANCE(685);
      END_STATE();
    case 534:
      if (lookahead == '2') ADVANCE(686);
      if (lookahead == '3') ADVANCE(687);
      if (lookahead == '4') ADVANCE(688);
      END_STATE();
    case 535:
      if (lookahead == '2') ADVANCE(689);
      if (lookahead == '3') ADVANCE(690);
      if (lookahead == '4') ADVANCE(691);
      END_STATE();
    case 536:
      if (lookahead == '2') ADVANCE(692);
      if (lookahead == '3') ADVANCE(693);
      if (lookahead == '4') ADVANCE(694);
      END_STATE();
    case 537:
      ACCEPT_TOKEN(anon_sym_match);
      END_STATE();
    case 538:
      if (lookahead == 'm') ADVANCE(695);
      END_STATE();
    case 539:
      if (lookahead == 'e') ADVANCE(696);
      END_STATE();
    case 540:
      if (lookahead == 'l') ADVANCE(697);
      END_STATE();
    case 541:
      if (lookahead == 'p') ADVANCE(698);
      END_STATE();
    case 542:
      if (lookahead == 'e') ADVANCE(699);
      END_STATE();
    case 543:
      if (lookahead == 'i') ADVANCE(700);
      END_STATE();
    case 544:
      if (lookahead == 'e') ADVANCE(701);
      END_STATE();
    case 545:
      if (lookahead == 's') ADVANCE(702);
      END_STATE();
    case 546:
      if (lookahead == 't') ADVANCE(703);
      END_STATE();
    case 547:
      if (lookahead == 'o') ADVANCE(704);
      END_STATE();
    case 548:
      if (lookahead == 't') ADVANCE(705);
      END_STATE();
    case 549:
      if (lookahead == 'i') ADVANCE(706);
      END_STATE();
    case 550:
      if (lookahead == 'g') ADVANCE(707);
      END_STATE();
    case 551:
      if (lookahead == 'f') ADVANCE(708);
      END_STATE();
    case 552:
      if (lookahead == 't') ADVANCE(709);
      END_STATE();
    case 553:
      ACCEPT_TOKEN(anon_sym_patch);
      END_STATE();
    case 554:
      if (lookahead == 'e') ADVANCE(710);
      END_STATE();
    case 555:
      if (lookahead == 'f') ADVANCE(711);
      END_STATE();
    case 556:
      if (lookahead == 'i') ADVANCE(712);
      END_STATE();
    case 557:
      if (lookahead == 's') ADVANCE(713);
      END_STATE();
    case 558:
      if (lookahead == 'r') ADVANCE(714);
      END_STATE();
    case 559:
      if (lookahead == 't') ADVANCE(715);
      END_STATE();
    case 560:
      if (lookahead == 'c') ADVANCE(716);
      END_STATE();
    case 561:
      if (lookahead == 'c') ADVANCE(717);
      END_STATE();
    case 562:
      if (lookahead == 'o') ADVANCE(718);
      END_STATE();
    case 563:
      if (lookahead == 'n') ADVANCE(719);
      END_STATE();
    case 564:
      if (lookahead == 'n') ADVANCE(720);
      END_STATE();
    case 565:
      if (lookahead == 'w') ADVANCE(721);
      END_STATE();
    case 566:
      if (lookahead == 'n') ADVANCE(722);
      END_STATE();
    case 567:
      if (lookahead == 'd') ADVANCE(723);
      END_STATE();
    case 568:
      if (lookahead == 't') ADVANCE(724);
      END_STATE();
    case 569:
      if (lookahead == 'e') ADVANCE(725);
      END_STATE();
    case 570:
      if (lookahead == 'r') ADVANCE(726);
      END_STATE();
    case 571:
      if (lookahead == 'r') ADVANCE(727);
      END_STATE();
    case 572:
      if (lookahead == 'i') ADVANCE(728);
      END_STATE();
    case 573:
      if (lookahead == 'n') ADVANCE(729);
      END_STATE();
    case 574:
      if (lookahead == 'l') ADVANCE(730);
      END_STATE();
    case 575:
      if (lookahead == 'i') ADVANCE(731);
      END_STATE();
    case 576:
      if (lookahead == 'i') ADVANCE(732);
      END_STATE();
    case 577:
      if (lookahead == '6') ADVANCE(733);
      END_STATE();
    case 578:
      if (lookahead == '2') ADVANCE(734);
      END_STATE();
    case 579:
      if (lookahead == 's') ADVANCE(735);
      if (lookahead == 'u') ADVANCE(736);
      END_STATE();
    case 580:
      if (lookahead == 'e') ADVANCE(737);
      END_STATE();
    case 581:
      if (lookahead == 'd') ADVANCE(738);
      END_STATE();
    case 582:
      if (lookahead == 'd') ADVANCE(739);
      END_STATE();
    case 583:
      if (lookahead == 'f') ADVANCE(740);
      END_STATE();
    case 584:
      if (lookahead == 'h') ADVANCE(741);
      END_STATE();
    case 585:
      ACCEPT_TOKEN(anon_sym_snorm);
      END_STATE();
    case 586:
      if (lookahead == 'c') ADVANCE(742);
      END_STATE();
    case 587:
      if (lookahead == 'g') ADVANCE(743);
      END_STATE();
    case 588:
      if (lookahead == 't') ADVANCE(744);
      END_STATE();
    case 589:
      if (lookahead == 'u') ADVANCE(745);
      END_STATE();
    case 590:
      ACCEPT_TOKEN(anon_sym_super);
      END_STATE();
    case 591:
      if (lookahead == 'h') ADVANCE(746);
      END_STATE();
    case 592:
      if (lookahead == 't') ADVANCE(747);
      END_STATE();
    case 593:
      if (lookahead == 'a') ADVANCE(748);
      END_STATE();
    case 594:
      if (lookahead == 'r') ADVANCE(749);
      END_STATE();
    case 595:
      if (lookahead == 'd') ADVANCE(750);
      END_STATE();
    case 596:
      ACCEPT_TOKEN(anon_sym_throw);
      END_STATE();
    case 597:
      ACCEPT_TOKEN(anon_sym_trait);
      END_STATE();
    case 598:
      if (lookahead == 'e') ADVANCE(751);
      END_STATE();
    case 599:
      if (lookahead == 'd') ADVANCE(752);
      END_STATE();
    case 600:
      if (lookahead == 'a') ADVANCE(753);
      END_STATE();
    case 601:
      if (lookahead == 'f') ADVANCE(754);
      END_STATE();
    case 602:
      if (lookahead == 'r') ADVANCE(755);
      END_STATE();
    case 603:
      ACCEPT_TOKEN(anon_sym_union);
      END_STATE();
    case 604:
      if (lookahead == 's') ADVANCE(756);
      END_STATE();
    case 605:
      ACCEPT_TOKEN(anon_sym_unorm);
      END_STATE();
    case 606:
      if (lookahead == 'e') ADVANCE(757);
      END_STATE();
    case 607:
      if (lookahead == 'e') ADVANCE(758);
      END_STATE();
    case 608:
      ACCEPT_TOKEN(anon_sym_using);
      END_STATE();
    case 609:
      if (lookahead == 'n') ADVANCE(759);
      END_STATE();
    case 610:
      if (lookahead == 'x') ADVANCE(760);
      END_STATE();
    case 611:
      if (lookahead == 'a') ADVANCE(761);
      END_STATE();
    case 612:
      if (lookahead == 'i') ADVANCE(762);
      END_STATE();
    case 613:
      ACCEPT_TOKEN(anon_sym_where);
      END_STATE();
    case 614:
      ACCEPT_TOKEN(anon_sym_while);
      END_STATE();
    case 615:
      if (lookahead == 'r') ADVANCE(763);
      END_STATE();
    case 616:
      ACCEPT_TOKEN(anon_sym_write);
      if (lookahead == 'o') ADVANCE(764);
      END_STATE();
    case 617:
      ACCEPT_TOKEN(anon_sym_yield);
      END_STATE();
    case 618:
      if (lookahead == 'e') ADVANCE(765);
      END_STATE();
    case 619:
      if (lookahead == 'e') ADVANCE(766);
      END_STATE();
    case 620:
      if (lookahead == 'S') ADVANCE(767);
      END_STATE();
    case 621:
      if (lookahead == 'r') ADVANCE(768);
      END_STATE();
    case 622:
      if (lookahead == 'a') ADVANCE(769);
      END_STATE();
    case 623:
      if (lookahead == 'c') ADVANCE(770);
      END_STATE();
    case 624:
      ACCEPT_TOKEN(anon_sym_active);
      END_STATE();
    case 625:
      if (lookahead == 's') ADVANCE(771);
      END_STATE();
    case 626:
      if (lookahead == 'f') ADVANCE(772);
      END_STATE();
    case 627:
      if (lookahead == 'a') ADVANCE(773);
      END_STATE();
    case 628:
      ACCEPT_TOKEN(anon_sym_atomic);
      END_STATE();
    case 629:
      if (lookahead == 'u') ADVANCE(774);
      END_STATE();
    case 630:
      ACCEPT_TOKEN(anon_sym_become);
      END_STATE();
    case 631:
      if (lookahead == 'n') ADVANCE(775);
      END_STATE();
    case 632:
      if (lookahead == 'g') ADVANCE(776);
      END_STATE();
    case 633:
      if (lookahead == 't') ADVANCE(777);
      END_STATE();
    case 634:
      if (lookahead == 'n') ADVANCE(778);
      END_STATE();
    case 635:
      ACCEPT_TOKEN(anon_sym_center);
      END_STATE();
    case 636:
      if (lookahead == 'i') ADVANCE(779);
      END_STATE();
    case 637:
      if (lookahead == 'i') ADVANCE(780);
      END_STATE();
    case 638:
      if (lookahead == 'u') ADVANCE(781);
      END_STATE();
    case 639:
      if (lookahead == 'l') ADVANCE(782);
      END_STATE();
    case 640:
      if (lookahead == 'n') ADVANCE(783);
      END_STATE();
    case 641:
      if (lookahead == '_') ADVANCE(784);
      END_STATE();
    case 642:
      ACCEPT_TOKEN(anon_sym_common);
      END_STATE();
    case 643:
      if (lookahead == 'e') ADVANCE(785);
      END_STATE();
    case 644:
      if (lookahead == 'e') ADVANCE(786);
      END_STATE();
    case 645:
      if (lookahead == 't') ADVANCE(787);
      END_STATE();
    case 646:
      if (lookahead == 'a') ADVANCE(788);
      if (lookahead == 'c') ADVANCE(789);
      END_STATE();
    case 647:
      if (lookahead == 'v') ADVANCE(790);
      if (lookahead == 'x') ADVANCE(791);
      END_STATE();
    case 648:
      if (lookahead == 'n') ADVANCE(792);
      END_STATE();
    case 649:
      if (lookahead == 'u') ADVANCE(793);
      END_STATE();
    case 650:
      if (lookahead == 'e') ADVANCE(794);
      END_STATE();
    case 651:
      if (lookahead == 'p') ADVANCE(795);
      END_STATE();
    case 652:
      if (lookahead == 't') ADVANCE(796);
      END_STATE();
    case 653:
      ACCEPT_TOKEN(anon_sym_delete);
      END_STATE();
    case 654:
      ACCEPT_TOKEN(anon_sym_demote);
      if (lookahead == '_') ADVANCE(797);
      END_STATE();
    case 655:
      if (lookahead == 'd') ADVANCE(798);
      END_STATE();
    case 656:
      if (lookahead == 'c') ADVANCE(799);
      END_STATE();
    case 657:
      ACCEPT_TOKEN(anon_sym_enable);
      END_STATE();
    case 658:
      if (lookahead == 'i') ADVANCE(800);
      END_STATE();
    case 659:
      ACCEPT_TOKEN(anon_sym_export);
      END_STATE();
    case 660:
      if (lookahead == 's') ADVANCE(801);
      END_STATE();
    case 661:
      ACCEPT_TOKEN(anon_sym_extern);
      if (lookahead == 'a') ADVANCE(802);
      END_STATE();
    case 662:
      if (lookahead == 'r') ADVANCE(803);
      END_STATE();
    case 663:
      ACCEPT_TOKEN(anon_sym_filter);
      END_STATE();
    case 664:
      if (lookahead == 'y') ADVANCE(804);
      END_STATE();
    case 665:
      if (lookahead == 'e') ADVANCE(805);
      END_STATE();
    case 666:
      if (lookahead == 'n') ADVANCE(806);
      END_STATE();
    case 667:
      ACCEPT_TOKEN(anon_sym_friend);
      END_STATE();
    case 668:
      if (lookahead == 'f') ADVANCE(807);
      END_STATE();
    case 669:
      if (lookahead == 'o') ADVANCE(808);
      END_STATE();
    case 670:
      if (lookahead == 'p') ADVANCE(809);
      END_STATE();
    case 671:
      if (lookahead == '_') ADVANCE(810);
      END_STATE();
    case 672:
      if (lookahead == 'h') ADVANCE(811);
      END_STATE();
    case 673:
      ACCEPT_TOKEN(anon_sym_handle);
      END_STATE();
    case 674:
      if (lookahead == 'e') ADVANCE(812);
      END_STATE();
    case 675:
      ACCEPT_TOKEN(anon_sym_import);
      END_STATE();
    case 676:
      ACCEPT_TOKEN(anon_sym_inline);
      END_STATE();
    case 677:
      if (lookahead == 'c') ADVANCE(813);
      END_STATE();
    case 678:
      if (lookahead == 'a') ADVANCE(814);
      END_STATE();
    case 679:
      if (lookahead == 'o') ADVANCE(815);
      END_STATE();
    case 680:
      if (lookahead == 'a') ADVANCE(816);
      END_STATE();
    case 681:
      ACCEPT_TOKEN(anon_sym_layout);
      END_STATE();
    case 682:
      ACCEPT_TOKEN(anon_sym_linear);
      END_STATE();
    case 683:
      if (lookahead == 'i') ADVANCE(817);
      END_STATE();
    case 684:
      if (lookahead == 'o') ADVANCE(818);
      END_STATE();
    case 685:
      if (lookahead == 'r') ADVANCE(819);
      END_STATE();
    case 686:
      ACCEPT_TOKEN(anon_sym_mat2x2);
      END_STATE();
    case 687:
      ACCEPT_TOKEN(anon_sym_mat2x3);
      END_STATE();
    case 688:
      ACCEPT_TOKEN(anon_sym_mat2x4);
      END_STATE();
    case 689:
      ACCEPT_TOKEN(anon_sym_mat3x2);
      END_STATE();
    case 690:
      ACCEPT_TOKEN(anon_sym_mat3x3);
      END_STATE();
    case 691:
      ACCEPT_TOKEN(anon_sym_mat3x4);
      END_STATE();
    case 692:
      ACCEPT_TOKEN(anon_sym_mat4x2);
      END_STATE();
    case 693:
      ACCEPT_TOKEN(anon_sym_mat4x3);
      END_STATE();
    case 694:
      ACCEPT_TOKEN(anon_sym_mat4x4);
      END_STATE();
    case 695:
      if (lookahead == 'p') ADVANCE(820);
      END_STATE();
    case 696:
      ACCEPT_TOKEN(anon_sym_module);
      END_STATE();
    case 697:
      if (lookahead == 'e') ADVANCE(821);
      END_STATE();
    case 698:
      if (lookahead == 'a') ADVANCE(822);
      END_STATE();
    case 699:
      if (lookahead == 'p') ADVANCE(823);
      END_STATE();
    case 700:
      if (lookahead == 'n') ADVANCE(824);
      END_STATE();
    case 701:
      if (lookahead == 'r') ADVANCE(825);
      END_STATE();
    case 702:
      if (lookahead == 'p') ADVANCE(826);
      END_STATE();
    case 703:
      if (lookahead == 'r') ADVANCE(827);
      END_STATE();
    case 704:
      if (lookahead == 'r') ADVANCE(828);
      END_STATE();
    case 705:
      if (lookahead == 'o') ADVANCE(829);
      END_STATE();
    case 706:
      if (lookahead == 'd') ADVANCE(830);
      END_STATE();
    case 707:
      if (lookahead == 'e') ADVANCE(831);
      END_STATE();
    case 708:
      if (lookahead == 'f') ADVANCE(832);
      END_STATE();
    case 709:
      if (lookahead == 'i') ADVANCE(833);
      END_STATE();
    case 710:
      if (lookahead == 'c') ADVANCE(834);
      END_STATE();
    case 711:
      if (lookahead == 'r') ADVANCE(835);
      END_STATE();
    case 712:
      if (lookahead == 'o') ADVANCE(836);
      END_STATE();
    case 713:
      if (lookahead == 'e') ADVANCE(837);
      if (lookahead == 'i') ADVANCE(838);
      END_STATE();
    case 714:
      if (lookahead == 'g') ADVANCE(839);
      END_STATE();
    case 715:
      if (lookahead == 'e') ADVANCE(840);
      END_STATE();
    case 716:
      if (lookahead == 't') ADVANCE(841);
      END_STATE();
    case 717:
      ACCEPT_TOKEN(anon_sym_public);
      END_STATE();
    case 718:
      if (lookahead == 'a') ADVANCE(842);
      END_STATE();
    case 719:
      if (lookahead == 't') ADVANCE(843);
      END_STATE();
    case 720:
      if (lookahead == 't') ADVANCE(844);
      END_STATE();
    case 721:
      if (lookahead == 'r') ADVANCE(845);
      END_STATE();
    case 722:
      if (lookahead == 'l') ADVANCE(846);
      END_STATE();
    case 723:
      if (lookahead == 'l') ADVANCE(847);
      END_STATE();
    case 724:
      if (lookahead == 'e') ADVANCE(848);
      END_STATE();
    case 725:
      if (lookahead == 'r') ADVANCE(849);
      END_STATE();
    case 726:
      if (lookahead == 'e') ADVANCE(850);
      END_STATE();
    case 727:
      if (lookahead == 'c') ADVANCE(851);
      END_STATE();
    case 728:
      if (lookahead == 'c') ADVANCE(852);
      END_STATE();
    case 729:
      ACCEPT_TOKEN(anon_sym_return);
      END_STATE();
    case 730:
      if (lookahead == 'o') ADVANCE(853);
      END_STATE();
    case 731:
      if (lookahead == 'n') ADVANCE(854);
      END_STATE();
    case 732:
      if (lookahead == 'n') ADVANCE(855);
      END_STATE();
    case 733:
      if (lookahead == 'f') ADVANCE(856);
      if (lookahead == 's') ADVANCE(857);
      if (lookahead == 'u') ADVANCE(858);
      END_STATE();
    case 734:
      if (lookahead == 'f') ADVANCE(859);
      if (lookahead == 's') ADVANCE(860);
      if (lookahead == 'u') ADVANCE(861);
      END_STATE();
    case 735:
      if (lookahead == 'i') ADVANCE(862);
      if (lookahead == 'n') ADVANCE(863);
      END_STATE();
    case 736:
      if (lookahead == 'i') ADVANCE(864);
      if (lookahead == 'n') ADVANCE(865);
      END_STATE();
    case 737:
      ACCEPT_TOKEN(anon_sym_sample);
      if (lookahead == '_') ADVANCE(866);
      if (lookahead == 'r') ADVANCE(867);
      END_STATE();
    case 738:
      ACCEPT_TOKEN(anon_sym_shared);
      END_STATE();
    case 739:
      ACCEPT_TOKEN(anon_sym_signed);
      END_STATE();
    case 740:
      ACCEPT_TOKEN(anon_sym_sizeof);
      END_STATE();
    case 741:
      ACCEPT_TOKEN(anon_sym_smooth);
      END_STATE();
    case 742:
      ACCEPT_TOKEN(anon_sym_static);
      if (lookahead == '_') ADVANCE(868);
      END_STATE();
    case 743:
      if (lookahead == 'e') ADVANCE(869);
      END_STATE();
    case 744:
      ACCEPT_TOKEN(anon_sym_struct);
      END_STATE();
    case 745:
      if (lookahead == 't') ADVANCE(870);
      END_STATE();
    case 746:
      ACCEPT_TOKEN(anon_sym_switch);
      END_STATE();
    case 747:
      ACCEPT_TOKEN(anon_sym_target);
      END_STATE();
    case 748:
      if (lookahead == 't') ADVANCE(871);
      END_STATE();
    case 749:
      if (lookahead == 'e') ADVANCE(872);
      END_STATE();
    case 750:
      if (lookahead == '_') ADVANCE(873);
      END_STATE();
    case 751:
      if (lookahead == 'f') ADVANCE(874);
      END_STATE();
    case 752:
      ACCEPT_TOKEN(anon_sym_typeid);
      END_STATE();
    case 753:
      if (lookahead == 'm') ADVANCE(875);
      END_STATE();
    case 754:
      ACCEPT_TOKEN(anon_sym_typeof);
      END_STATE();
    case 755:
      if (lookahead == 'm') ADVANCE(876);
      END_STATE();
    case 756:
      ACCEPT_TOKEN(anon_sym_unless);
      END_STATE();
    case 757:
      ACCEPT_TOKEN(anon_sym_unsafe);
      END_STATE();
    case 758:
      if (lookahead == 'd') ADVANCE(877);
      END_STATE();
    case 759:
      if (lookahead == 'g') ADVANCE(878);
      END_STATE();
    case 760:
      ACCEPT_TOKEN(anon_sym_vertex);
      if (lookahead == '_') ADVANCE(879);
      END_STATE();
    case 761:
      if (lookahead == 'l') ADVANCE(880);
      END_STATE();
    case 762:
      if (lookahead == 'l') ADVANCE(881);
      END_STATE();
    case 763:
      if (lookahead == 'o') ADVANCE(882);
      END_STATE();
    case 764:
      if (lookahead == 'n') ADVANCE(883);
      END_STATE();
    case 765:
      if (lookahead == 'S') ADVANCE(884);
      END_STATE();
    case 766:
      if (lookahead == 'S') ADVANCE(885);
      END_STATE();
    case 767:
      if (lookahead == 'h') ADVANCE(886);
      END_STATE();
    case 768:
      if (lookahead == 'y') ADVANCE(887);
      END_STATE();
    case 769:
      if (lookahead == 'd') ADVANCE(888);
      END_STATE();
    case 770:
      if (lookahead == 't') ADVANCE(889);
      END_STATE();
    case 771:
      ACCEPT_TOKEN(anon_sym_alignas);
      END_STATE();
    case 772:
      ACCEPT_TOKEN(anon_sym_alignof);
      END_STATE();
    case 773:
      if (lookahead == 'g') ADVANCE(890);
      END_STATE();
    case 774:
      if (lookahead == 't') ADVANCE(891);
      END_STATE();
    case 775:
      if (lookahead == 'o') ADVANCE(892);
      END_STATE();
    case 776:
      ACCEPT_TOKEN(anon_sym_binding);
      if (lookahead == '_') ADVANCE(893);
      END_STATE();
    case 777:
      ACCEPT_TOKEN(anon_sym_bitcast);
      END_STATE();
    case 778:
      ACCEPT_TOKEN(anon_sym_builtin);
      END_STATE();
    case 779:
      if (lookahead == 'd') ADVANCE(894);
      END_STATE();
    case 780:
      if (lookahead == 't') ADVANCE(895);
      END_STATE();
    case 781:
      if (lookahead == 'r') ADVANCE(896);
      END_STATE();
    case 782:
      if (lookahead == 'd') ADVANCE(897);
      END_STATE();
    case 783:
      if (lookahead == 't') ADVANCE(898);
      END_STATE();
    case 784:
      if (lookahead == 'm') ADVANCE(899);
      END_STATE();
    case 785:
      ACCEPT_TOKEN(anon_sym_compile);
      if (lookahead == '_') ADVANCE(900);
      END_STATE();
    case 786:
      ACCEPT_TOKEN(anon_sym_compute);
      END_STATE();
    case 787:
      ACCEPT_TOKEN(anon_sym_concept);
      END_STATE();
    case 788:
      if (lookahead == 's') ADVANCE(901);
      END_STATE();
    case 789:
      if (lookahead == 'a') ADVANCE(902);
      END_STATE();
    case 790:
      if (lookahead == 'a') ADVANCE(903);
      END_STATE();
    case 791:
      if (lookahead == 'p') ADVANCE(904);
      END_STATE();
    case 792:
      if (lookahead == 'i') ADVANCE(905);
      END_STATE();
    case 793:
      if (lookahead == 'e') ADVANCE(906);
      if (lookahead == 'i') ADVANCE(907);
      END_STATE();
    case 794:
      if (lookahead == 'r') ADVANCE(908);
      END_STATE();
    case 795:
      if (lookahead == 'e') ADVANCE(909);
      END_STATE();
    case 796:
      ACCEPT_TOKEN(anon_sym_default);
      END_STATE();
    case 797:
      if (lookahead == 't') ADVANCE(910);
      END_STATE();
    case 798:
      ACCEPT_TOKEN(anon_sym_discard);
      END_STATE();
    case 799:
      if (lookahead == '_') ADVANCE(911);
      END_STATE();
    case 800:
      if (lookahead == 't') ADVANCE(912);
      END_STATE();
    case 801:
      ACCEPT_TOKEN(anon_sym_extends);
      END_STATE();
    case 802:
      if (lookahead == 'l') ADVANCE(913);
      END_STATE();
    case 803:
      if (lookahead == 'o') ADVANCE(914);
      END_STATE();
    case 804:
      ACCEPT_TOKEN(anon_sym_finally);
      END_STATE();
    case 805:
      if (lookahead == 'p') ADVANCE(915);
      END_STATE();
    case 806:
      if (lookahead == 't') ADVANCE(916);
      END_STATE();
    case 807:
      if (lookahead == 'a') ADVANCE(917);
      END_STATE();
    case 808:
      if (lookahead == 'n') ADVANCE(918);
      END_STATE();
    case 809:
      ACCEPT_TOKEN(anon_sym_fxgroup);
      END_STATE();
    case 810:
      if (lookahead == 'i') ADVANCE(919);
      END_STATE();
    case 811:
      if (lookahead == 'a') ADVANCE(920);
      END_STATE();
    case 812:
      if (lookahead == 'n') ADVANCE(921);
      END_STATE();
    case 813:
      if (lookahead == 'e') ADVANCE(922);
      END_STATE();
    case 814:
      if (lookahead == 'c') ADVANCE(923);
      END_STATE();
    case 815:
      if (lookahead == 'l') ADVANCE(924);
      END_STATE();
    case 816:
      if (lookahead == 'n') ADVANCE(925);
      END_STATE();
    case 817:
      if (lookahead == 'n') ADVANCE(926);
      END_STATE();
    case 818:
      if (lookahead == 'n') ADVANCE(927);
      END_STATE();
    case 819:
      if (lookahead == 'u') ADVANCE(928);
      END_STATE();
    case 820:
      ACCEPT_TOKEN(anon_sym_mediump);
      END_STATE();
    case 821:
      ACCEPT_TOKEN(anon_sym_mutable);
      END_STATE();
    case 822:
      if (lookahead == 'c') ADVANCE(929);
      END_STATE();
    case 823:
      if (lookahead == 't') ADVANCE(930);
      END_STATE();
    case 824:
      if (lookahead == 'e') ADVANCE(931);
      END_STATE();
    case 825:
      if (lookahead == 'p') ADVANCE(932);
      END_STATE();
    case 826:
      if (lookahead == 'e') ADVANCE(933);
      END_STATE();
    case 827:
      ACCEPT_TOKEN(anon_sym_nullptr);
      END_STATE();
    case 828:
      if (lookahead == 'k') ADVANCE(934);
      END_STATE();
    case 829:
      if (lookahead == 'r') ADVANCE(935);
      END_STATE();
    case 830:
      if (lookahead == 'e') ADVANCE(936);
      END_STATE();
    case 831:
      ACCEPT_TOKEN(anon_sym_package);
      END_STATE();
    case 832:
      if (lookahead == 's') ADVANCE(937);
      END_STATE();
    case 833:
      if (lookahead == 'o') ADVANCE(938);
      END_STATE();
    case 834:
      if (lookahead == 't') ADVANCE(939);
      END_STATE();
    case 835:
      if (lookahead == 'a') ADVANCE(940);
      END_STATE();
    case 836:
      if (lookahead == 'n') ADVANCE(941);
      END_STATE();
    case 837:
      ACCEPT_TOKEN(anon_sym_precise);
      END_STATE();
    case 838:
      if (lookahead == 'o') ADVANCE(942);
      END_STATE();
    case 839:
      if (lookahead == 'e') ADVANCE(943);
      END_STATE();
    case 840:
      ACCEPT_TOKEN(anon_sym_private);
      END_STATE();
    case 841:
      if (lookahead == 'e') ADVANCE(944);
      END_STATE();
    case 842:
      if (lookahead == 't') ADVANCE(945);
      END_STATE();
    case 843:
      ACCEPT_TOKEN(anon_sym_r32sint);
      END_STATE();
    case 844:
      ACCEPT_TOKEN(anon_sym_r32uint);
      END_STATE();
    case 845:
      if (lookahead == 'i') ADVANCE(946);
      END_STATE();
    case 846:
      if (lookahead == 'y') ADVANCE(947);
      END_STATE();
    case 847:
      if (lookahead == 'e') ADVANCE(948);
      END_STATE();
    case 848:
      if (lookahead == 'r') ADVANCE(949);
      END_STATE();
    case 849:
      if (lookahead == 'p') ADVANCE(950);
      END_STATE();
    case 850:
      if (lookahead == 's') ADVANCE(951);
      END_STATE();
    case 851:
      if (lookahead == 'e') ADVANCE(952);
      END_STATE();
    case 852:
      if (lookahead == 't') ADVANCE(953);
      END_STATE();
    case 853:
      if (lookahead == 'a') ADVANCE(954);
      END_STATE();
    case 854:
      if (lookahead == 't') ADVANCE(955);
      END_STATE();
    case 855:
      if (lookahead == 't') ADVANCE(956);
      END_STATE();
    case 856:
      if (lookahead == 'l') ADVANCE(957);
      END_STATE();
    case 857:
      if (lookahead == 'i') ADVANCE(958);
      END_STATE();
    case 858:
      if (lookahead == 'i') ADVANCE(959);
      END_STATE();
    case 859:
      if (lookahead == 'l') ADVANCE(960);
      END_STATE();
    case 860:
      if (lookahead == 'i') ADVANCE(961);
      END_STATE();
    case 861:
      if (lookahead == 'i') ADVANCE(962);
      END_STATE();
    case 862:
      if (lookahead == 'n') ADVANCE(963);
      END_STATE();
    case 863:
      if (lookahead == 'o') ADVANCE(964);
      END_STATE();
    case 864:
      if (lookahead == 'n') ADVANCE(965);
      END_STATE();
    case 865:
      if (lookahead == 'o') ADVANCE(966);
      END_STATE();
    case 866:
      if (lookahead == 'i') ADVANCE(967);
      if (lookahead == 'm') ADVANCE(968);
      END_STATE();
    case 867:
      ACCEPT_TOKEN(anon_sym_sampler);
      if (lookahead == '_') ADVANCE(969);
      END_STATE();
    case 868:
      if (lookahead == 'a') ADVANCE(970);
      if (lookahead == 'c') ADVANCE(971);
      END_STATE();
    case 869:
      ACCEPT_TOKEN(anon_sym_storage);
      END_STATE();
    case 870:
      if (lookahead == 'i') ADVANCE(972);
      END_STATE();
    case 871:
      if (lookahead == 'e') ADVANCE(973);
      END_STATE();
    case 872:
      if (lookahead == '_') ADVANCE(974);
      END_STATE();
    case 873:
      if (lookahead == 'l') ADVANCE(975);
      END_STATE();
    case 874:
      ACCEPT_TOKEN(anon_sym_typedef);
      END_STATE();
    case 875:
      if (lookahead == 'e') ADVANCE(976);
      END_STATE();
    case 876:
      ACCEPT_TOKEN(anon_sym_uniform);
      END_STATE();
    case 877:
      ACCEPT_TOKEN(anon_sym_unsized);
      END_STATE();
    case 878:
      ACCEPT_TOKEN(anon_sym_varying);
      END_STATE();
    case 879:
      if (lookahead == 'i') ADVANCE(977);
      END_STATE();
    case 880:
      ACCEPT_TOKEN(anon_sym_virtual);
      END_STATE();
    case 881:
      if (lookahead == 'e') ADVANCE(978);
      END_STATE();
    case 882:
      if (lookahead == 'u') ADVANCE(979);
      END_STATE();
    case 883:
      if (lookahead == 'l') ADVANCE(980);
      END_STATE();
    case 884:
      if (lookahead == 'h') ADVANCE(981);
      END_STATE();
    case 885:
      if (lookahead == 'h') ADVANCE(982);
      END_STATE();
    case 886:
      if (lookahead == 'a') ADVANCE(983);
      END_STATE();
    case 887:
      if (lookahead == 'S') ADVANCE(984);
      END_STATE();
    case 888:
      if (lookahead == 'e') ADVANCE(985);
      END_STATE();
    case 889:
      ACCEPT_TOKEN(anon_sym_abstract);
      END_STATE();
    case 890:
      if (lookahead == 'm') ADVANCE(986);
      END_STATE();
    case 891:
      if (lookahead == 'e') ADVANCE(987);
      END_STATE();
    case 892:
      if (lookahead == 'r') ADVANCE(988);
      END_STATE();
    case 893:
      if (lookahead == 'a') ADVANCE(989);
      END_STATE();
    case 894:
      ACCEPT_TOKEN(anon_sym_centroid);
      END_STATE();
    case 895:
      ACCEPT_TOKEN(anon_sym_co_await);
      END_STATE();
    case 896:
      if (lookahead == 'n') ADVANCE(990);
      END_STATE();
    case 897:
      ACCEPT_TOKEN(anon_sym_co_yield);
      END_STATE();
    case 898:
      ACCEPT_TOKEN(anon_sym_coherent);
      END_STATE();
    case 899:
      if (lookahead == 'a') ADVANCE(991);
      END_STATE();
    case 900:
      if (lookahead == 'f') ADVANCE(992);
      END_STATE();
    case 901:
      if (lookahead == 's') ADVANCE(993);
      END_STATE();
    case 902:
      if (lookahead == 's') ADVANCE(994);
      END_STATE();
    case 903:
      if (lookahead == 'l') ADVANCE(995);
      END_STATE();
    case 904:
      if (lookahead == 'r') ADVANCE(996);
      END_STATE();
    case 905:
      if (lookahead == 't') ADVANCE(997);
      END_STATE();
    case 906:
      ACCEPT_TOKEN(sym_continue_statement);
      END_STATE();
    case 907:
      if (lookahead == 'n') ADVANCE(998);
      END_STATE();
    case 908:
      ACCEPT_TOKEN(anon_sym_debugger);
      END_STATE();
    case 909:
      ACCEPT_TOKEN(anon_sym_decltype);
      END_STATE();
    case 910:
      if (lookahead == 'o') ADVANCE(999);
      END_STATE();
    case 911:
      if (lookahead == 'c') ADVANCE(1000);
      END_STATE();
    case 912:
      ACCEPT_TOKEN(anon_sym_explicit);
      END_STATE();
    case 913:
      ACCEPT_TOKEN(anon_sym_external);
      END_STATE();
    case 914:
      if (lookahead == 'u') ADVANCE(1001);
      END_STATE();
    case 915:
      if (lookahead == 't') ADVANCE(1002);
      END_STATE();
    case 916:
      ACCEPT_TOKEN(anon_sym_fragment);
      END_STATE();
    case 917:
      if (lookahead == 'c') ADVANCE(1003);
      END_STATE();
    case 918:
      ACCEPT_TOKEN(anon_sym_function);
      END_STATE();
    case 919:
      if (lookahead == 'n') ADVANCE(1004);
      END_STATE();
    case 920:
      if (lookahead == 'r') ADVANCE(1005);
      END_STATE();
    case 921:
      if (lookahead == 't') ADVANCE(1006);
      END_STATE();
    case 922:
      if (lookahead == '_') ADVANCE(1007);
      if (lookahead == 'o') ADVANCE(1008);
      END_STATE();
    case 923:
      if (lookahead == 'e') ADVANCE(1009);
      END_STATE();
    case 924:
      if (lookahead == 'a') ADVANCE(1010);
      END_STATE();
    case 925:
      if (lookahead == 't') ADVANCE(1011);
      END_STATE();
    case 926:
      if (lookahead == 'v') ADVANCE(1012);
      END_STATE();
    case 927:
      ACCEPT_TOKEN(anon_sym_location);
      END_STATE();
    case 928:
      if (lookahead == 'l') ADVANCE(1013);
      END_STATE();
    case 929:
      if (lookahead == 'e') ADVANCE(1014);
      END_STATE();
    case 930:
      ACCEPT_TOKEN(anon_sym_noexcept);
      END_STATE();
    case 931:
      ACCEPT_TOKEN(anon_sym_noinline);
      END_STATE();
    case 932:
      if (lookahead == 'o') ADVANCE(1015);
      END_STATE();
    case 933:
      if (lookahead == 'c') ADVANCE(1016);
      END_STATE();
    case 934:
      if (lookahead == 'g') ADVANCE(1017);
      END_STATE();
    case 935:
      ACCEPT_TOKEN(anon_sym_operator);
      END_STATE();
    case 936:
      ACCEPT_TOKEN(anon_sym_override);
      END_STATE();
    case 937:
      if (lookahead == 'e') ADVANCE(1018);
      END_STATE();
    case 938:
      if (lookahead == 'n') ADVANCE(1019);
      END_STATE();
    case 939:
      if (lookahead == 'i') ADVANCE(1020);
      END_STATE();
    case 940:
      if (lookahead == 'g') ADVANCE(1021);
      END_STATE();
    case 941:
      ACCEPT_TOKEN(anon_sym_position);
      END_STATE();
    case 942:
      if (lookahead == 'n') ADVANCE(1022);
      END_STATE();
    case 943:
      ACCEPT_TOKEN(anon_sym_premerge);
      END_STATE();
    case 944:
      if (lookahead == 'd') ADVANCE(1023);
      END_STATE();
    case 945:
      ACCEPT_TOKEN(anon_sym_r32float);
      END_STATE();
    case 946:
      if (lookahead == 't') ADVANCE(1024);
      END_STATE();
    case 947:
      ACCEPT_TOKEN(anon_sym_readonly);
      END_STATE();
    case 948:
      if (lookahead == 's') ADVANCE(1025);
      END_STATE();
    case 949:
      ACCEPT_TOKEN(anon_sym_register);
      END_STATE();
    case 950:
      if (lookahead == 'r') ADVANCE(1026);
      END_STATE();
    case 951:
      ACCEPT_TOKEN(anon_sym_requires);
      END_STATE();
    case 952:
      ACCEPT_TOKEN(anon_sym_resource);
      END_STATE();
    case 953:
      ACCEPT_TOKEN(anon_sym_restrict);
      END_STATE();
    case 954:
      if (lookahead == 't') ADVANCE(1027);
      END_STATE();
    case 955:
      ACCEPT_TOKEN(anon_sym_rg32sint);
      END_STATE();
    case 956:
      ACCEPT_TOKEN(anon_sym_rg32uint);
      END_STATE();
    case 957:
      if (lookahead == 'o') ADVANCE(1028);
      END_STATE();
    case 958:
      if (lookahead == 'n') ADVANCE(1029);
      END_STATE();
    case 959:
      if (lookahead == 'n') ADVANCE(1030);
      END_STATE();
    case 960:
      if (lookahead == 'o') ADVANCE(1031);
      END_STATE();
    case 961:
      if (lookahead == 'n') ADVANCE(1032);
      END_STATE();
    case 962:
      if (lookahead == 'n') ADVANCE(1033);
      END_STATE();
    case 963:
      if (lookahead == 't') ADVANCE(1034);
      END_STATE();
    case 964:
      if (lookahead == 'r') ADVANCE(1035);
      END_STATE();
    case 965:
      if (lookahead == 't') ADVANCE(1036);
      END_STATE();
    case 966:
      if (lookahead == 'r') ADVANCE(1037);
      END_STATE();
    case 967:
      if (lookahead == 'n') ADVANCE(1038);
      END_STATE();
    case 968:
      if (lookahead == 'a') ADVANCE(1039);
      END_STATE();
    case 969:
      if (lookahead == 'c') ADVANCE(1040);
      END_STATE();
    case 970:
      if (lookahead == 's') ADVANCE(1041);
      END_STATE();
    case 971:
      if (lookahead == 'a') ADVANCE(1042);
      END_STATE();
    case 972:
      if (lookahead == 'n') ADVANCE(1043);
      END_STATE();
    case 973:
      ACCEPT_TOKEN(anon_sym_template);
      END_STATE();
    case 974:
      if (lookahead == '1') ADVANCE(1044);
      if (lookahead == '2') ADVANCE(1045);
      if (lookahead == '3') ADVANCE(1046);
      if (lookahead == 'c') ADVANCE(1047);
      if (lookahead == 'd') ADVANCE(1048);
      if (lookahead == 'm') ADVANCE(1049);
      if (lookahead == 's') ADVANCE(1050);
      END_STATE();
    case 975:
      if (lookahead == 'o') ADVANCE(1051);
      END_STATE();
    case 976:
      ACCEPT_TOKEN(anon_sym_typename);
      END_STATE();
    case 977:
      if (lookahead == 'n') ADVANCE(1052);
      END_STATE();
    case 978:
      ACCEPT_TOKEN(anon_sym_volatile);
      END_STATE();
    case 979:
      if (lookahead == 'p') ADVANCE(1053);
      END_STATE();
    case 980:
      if (lookahead == 'y') ADVANCE(1054);
      END_STATE();
    case 981:
      if (lookahead == 'a') ADVANCE(1055);
      END_STATE();
    case 982:
      if (lookahead == 'a') ADVANCE(1056);
      END_STATE();
    case 983:
      if (lookahead == 'd') ADVANCE(1057);
      END_STATE();
    case 984:
      if (lookahead == 'h') ADVANCE(1058);
      END_STATE();
    case 985:
      if (lookahead == 'r') ADVANCE(1059);
      END_STATE();
    case 986:
      if (lookahead == 'e') ADVANCE(1060);
      END_STATE();
    case 987:
      ACCEPT_TOKEN(anon_sym_attribute);
      END_STATE();
    case 988:
      if (lookahead == 'm') ADVANCE(1061);
      END_STATE();
    case 989:
      if (lookahead == 'r') ADVANCE(1062);
      END_STATE();
    case 990:
      ACCEPT_TOKEN(anon_sym_co_return);
      END_STATE();
    case 991:
      if (lookahead == 'j') ADVANCE(1063);
      END_STATE();
    case 992:
      if (lookahead == 'r') ADVANCE(1064);
      END_STATE();
    case 993:
      if (lookahead == 'e') ADVANCE(1065);
      END_STATE();
    case 994:
      if (lookahead == 't') ADVANCE(1066);
      END_STATE();
    case 995:
      ACCEPT_TOKEN(anon_sym_consteval);
      END_STATE();
    case 996:
      ACCEPT_TOKEN(anon_sym_constexpr);
      END_STATE();
    case 997:
      ACCEPT_TOKEN(anon_sym_constinit);
      END_STATE();
    case 998:
      if (lookahead == 'g') ADVANCE(1067);
      END_STATE();
    case 999:
      if (lookahead == '_') ADVANCE(1068);
      END_STATE();
    case 1000:
      if (lookahead == 'a') ADVANCE(1069);
      END_STATE();
    case 1001:
      if (lookahead == 'g') ADVANCE(1070);
      END_STATE();
    case 1002:
      if (lookahead == 'h') ADVANCE(1071);
      END_STATE();
    case 1003:
      if (lookahead == 'i') ADVANCE(1072);
      END_STATE();
    case 1004:
      if (lookahead == 'v') ADVANCE(1073);
      END_STATE();
    case 1005:
      if (lookahead == 'e') ADVANCE(1074);
      END_STATE();
    case 1006:
      if (lookahead == 's') ADVANCE(1075);
      END_STATE();
    case 1007:
      if (lookahead == 'i') ADVANCE(1076);
      END_STATE();
    case 1008:
      if (lookahead == 'f') ADVANCE(1077);
      END_STATE();
    case 1009:
      ACCEPT_TOKEN(anon_sym_interface);
      END_STATE();
    case 1010:
      if (lookahead == 't') ADVANCE(1078);
      END_STATE();
    case 1011:
      ACCEPT_TOKEN(anon_sym_invariant);
      END_STATE();
    case 1012:
      if (lookahead == 'o') ADVANCE(1079);
      END_STATE();
    case 1013:
      if (lookahead == 'e') ADVANCE(1080);
      END_STATE();
    case 1014:
      ACCEPT_TOKEN(anon_sym_namespace);
      END_STATE();
    case 1015:
      if (lookahead == 'l') ADVANCE(1081);
      END_STATE();
    case 1016:
      if (lookahead == 't') ADVANCE(1082);
      END_STATE();
    case 1017:
      if (lookahead == 'r') ADVANCE(1083);
      END_STATE();
    case 1018:
      if (lookahead == 't') ADVANCE(1084);
      END_STATE();
    case 1019:
      ACCEPT_TOKEN(anon_sym_partition);
      END_STATE();
    case 1020:
      if (lookahead == 'v') ADVANCE(1085);
      END_STATE();
    case 1021:
      if (lookahead == 'm') ADVANCE(1086);
      END_STATE();
    case 1022:
      ACCEPT_TOKEN(anon_sym_precision);
      END_STATE();
    case 1023:
      ACCEPT_TOKEN(anon_sym_protected);
      END_STATE();
    case 1024:
      if (lookahead == 'e') ADVANCE(1087);
      END_STATE();
    case 1025:
      if (lookahead == 's') ADVANCE(1088);
      END_STATE();
    case 1026:
      if (lookahead == 'e') ADVANCE(1089);
      END_STATE();
    case 1027:
      ACCEPT_TOKEN(anon_sym_rg32float);
      END_STATE();
    case 1028:
      if (lookahead == 'a') ADVANCE(1090);
      END_STATE();
    case 1029:
      if (lookahead == 't') ADVANCE(1091);
      END_STATE();
    case 1030:
      if (lookahead == 't') ADVANCE(1092);
      END_STATE();
    case 1031:
      if (lookahead == 'a') ADVANCE(1093);
      END_STATE();
    case 1032:
      if (lookahead == 't') ADVANCE(1094);
      END_STATE();
    case 1033:
      if (lookahead == 't') ADVANCE(1095);
      END_STATE();
    case 1034:
      ACCEPT_TOKEN(anon_sym_rgba8sint);
      END_STATE();
    case 1035:
      if (lookahead == 'm') ADVANCE(1096);
      END_STATE();
    case 1036:
      ACCEPT_TOKEN(anon_sym_rgba8uint);
      END_STATE();
    case 1037:
      if (lookahead == 'm') ADVANCE(1097);
      END_STATE();
    case 1038:
      if (lookahead == 'd') ADVANCE(1098);
      END_STATE();
    case 1039:
      if (lookahead == 's') ADVANCE(1099);
      END_STATE();
    case 1040:
      if (lookahead == 'o') ADVANCE(1100);
      END_STATE();
    case 1041:
      if (lookahead == 's') ADVANCE(1101);
      END_STATE();
    case 1042:
      if (lookahead == 's') ADVANCE(1102);
      END_STATE();
    case 1043:
      if (lookahead == 'e') ADVANCE(1103);
      END_STATE();
    case 1044:
      if (lookahead == 'd') ADVANCE(1104);
      END_STATE();
    case 1045:
      if (lookahead == 'd') ADVANCE(1105);
      END_STATE();
    case 1046:
      if (lookahead == 'd') ADVANCE(1106);
      END_STATE();
    case 1047:
      if (lookahead == 'u') ADVANCE(1107);
      END_STATE();
    case 1048:
      if (lookahead == 'e') ADVANCE(1108);
      END_STATE();
    case 1049:
      if (lookahead == 'u') ADVANCE(1109);
      END_STATE();
    case 1050:
      if (lookahead == 't') ADVANCE(1110);
      END_STATE();
    case 1051:
      if (lookahead == 'c') ADVANCE(1111);
      END_STATE();
    case 1052:
      if (lookahead == 'd') ADVANCE(1112);
      END_STATE();
    case 1053:
      ACCEPT_TOKEN(anon_sym_workgroup);
      if (lookahead == '_') ADVANCE(1113);
      END_STATE();
    case 1054:
      ACCEPT_TOKEN(anon_sym_writeonly);
      END_STATE();
    case 1055:
      if (lookahead == 'd') ADVANCE(1114);
      END_STATE();
    case 1056:
      if (lookahead == 'd') ADVANCE(1115);
      END_STATE();
    case 1057:
      if (lookahead == 'e') ADVANCE(1116);
      END_STATE();
    case 1058:
      if (lookahead == 'a') ADVANCE(1117);
      END_STATE();
    case 1059:
      ACCEPT_TOKEN(anon_sym_Hullshader);
      END_STATE();
    case 1060:
      if (lookahead == 'n') ADVANCE(1118);
      END_STATE();
    case 1061:
      ACCEPT_TOKEN(anon_sym_bgra8unorm);
      END_STATE();
    case 1062:
      if (lookahead == 'r') ADVANCE(1119);
      END_STATE();
    case 1063:
      if (lookahead == 'o') ADVANCE(1120);
      END_STATE();
    case 1064:
      if (lookahead == 'a') ADVANCE(1121);
      END_STATE();
    case 1065:
      if (lookahead == 'r') ADVANCE(1122);
      END_STATE();
    case 1066:
      ACCEPT_TOKEN(anon_sym_const_cast);
      END_STATE();
    case 1067:
      ACCEPT_TOKEN(anon_sym_continuing);
      END_STATE();
    case 1068:
      if (lookahead == 'h') ADVANCE(1123);
      END_STATE();
    case 1069:
      if (lookahead == 's') ADVANCE(1124);
      END_STATE();
    case 1070:
      if (lookahead == 'h') ADVANCE(1125);
      END_STATE();
    case 1071:
      ACCEPT_TOKEN(anon_sym_frag_depth);
      END_STATE();
    case 1072:
      if (lookahead == 'n') ADVANCE(1126);
      END_STATE();
    case 1073:
      if (lookahead == 'o') ADVANCE(1127);
      END_STATE();
    case 1074:
      if (lookahead == 'd') ADVANCE(1128);
      END_STATE();
    case 1075:
      ACCEPT_TOKEN(anon_sym_implements);
      END_STATE();
    case 1076:
      if (lookahead == 'n') ADVANCE(1129);
      END_STATE();
    case 1077:
      ACCEPT_TOKEN(anon_sym_instanceof);
      END_STATE();
    case 1078:
      if (lookahead == 'e') ADVANCE(1130);
      END_STATE();
    case 1079:
      if (lookahead == 'c') ADVANCE(1131);
      END_STATE();
    case 1080:
      if (lookahead == 's') ADVANCE(1132);
      END_STATE();
    case 1081:
      if (lookahead == 'a') ADVANCE(1133);
      END_STATE();
    case 1082:
      if (lookahead == 'i') ADVANCE(1134);
      END_STATE();
    case 1083:
      if (lookahead == 'o') ADVANCE(1135);
      END_STATE();
    case 1084:
      ACCEPT_TOKEN(anon_sym_packoffset);
      END_STATE();
    case 1085:
      if (lookahead == 'e') ADVANCE(1136);
      END_STATE();
    case 1086:
      if (lookahead == 'e') ADVANCE(1137);
      END_STATE();
    case 1087:
      ACCEPT_TOKEN(anon_sym_read_write);
      END_STATE();
    case 1088:
      ACCEPT_TOKEN(anon_sym_regardless);
      END_STATE();
    case 1089:
      if (lookahead == 't') ADVANCE(1138);
      END_STATE();
    case 1090:
      if (lookahead == 't') ADVANCE(1139);
      END_STATE();
    case 1091:
      ACCEPT_TOKEN(anon_sym_rgba16sint);
      END_STATE();
    case 1092:
      ACCEPT_TOKEN(anon_sym_rgba16uint);
      END_STATE();
    case 1093:
      if (lookahead == 't') ADVANCE(1140);
      END_STATE();
    case 1094:
      ACCEPT_TOKEN(anon_sym_rgba32sint);
      END_STATE();
    case 1095:
      ACCEPT_TOKEN(anon_sym_rgba32uint);
      END_STATE();
    case 1096:
      ACCEPT_TOKEN(anon_sym_rgba8snorm);
      END_STATE();
    case 1097:
      ACCEPT_TOKEN(anon_sym_rgba8unorm);
      END_STATE();
    case 1098:
      if (lookahead == 'e') ADVANCE(1141);
      END_STATE();
    case 1099:
      if (lookahead == 'k') ADVANCE(1142);
      END_STATE();
    case 1100:
      if (lookahead == 'm') ADVANCE(1143);
      END_STATE();
    case 1101:
      if (lookahead == 'e') ADVANCE(1144);
      END_STATE();
    case 1102:
      if (lookahead == 't') ADVANCE(1145);
      END_STATE();
    case 1103:
      ACCEPT_TOKEN(anon_sym_subroutine);
      END_STATE();
    case 1104:
      ACCEPT_TOKEN(anon_sym_texture_1d);
      END_STATE();
    case 1105:
      ACCEPT_TOKEN(anon_sym_texture_2d);
      if (lookahead == '_') ADVANCE(1146);
      END_STATE();
    case 1106:
      ACCEPT_TOKEN(anon_sym_texture_3d);
      END_STATE();
    case 1107:
      if (lookahead == 'b') ADVANCE(1147);
      END_STATE();
    case 1108:
      if (lookahead == 'p') ADVANCE(1148);
      END_STATE();
    case 1109:
      if (lookahead == 'l') ADVANCE(1149);
      END_STATE();
    case 1110:
      if (lookahead == 'o') ADVANCE(1150);
      END_STATE();
    case 1111:
      if (lookahead == 'a') ADVANCE(1151);
      END_STATE();
    case 1112:
      if (lookahead == 'e') ADVANCE(1152);
      END_STATE();
    case 1113:
      if (lookahead == 'i') ADVANCE(1153);
      if (lookahead == 's') ADVANCE(1154);
      END_STATE();
    case 1114:
      if (lookahead == 'e') ADVANCE(1155);
      END_STATE();
    case 1115:
      if (lookahead == 'e') ADVANCE(1156);
      END_STATE();
    case 1116:
      if (lookahead == 'r') ADVANCE(1157);
      END_STATE();
    case 1117:
      if (lookahead == 'd') ADVANCE(1158);
      END_STATE();
    case 1118:
      if (lookahead == 't') ADVANCE(1159);
      END_STATE();
    case 1119:
      if (lookahead == 'a') ADVANCE(1160);
      END_STATE();
    case 1120:
      if (lookahead == 'r') ADVANCE(1161);
      END_STATE();
    case 1121:
      if (lookahead == 'g') ADVANCE(1162);
      END_STATE();
    case 1122:
      if (lookahead == 't') ADVANCE(1163);
      END_STATE();
    case 1123:
      if (lookahead == 'e') ADVANCE(1164);
      END_STATE();
    case 1124:
      if (lookahead == 't') ADVANCE(1165);
      END_STATE();
    case 1125:
      ACCEPT_TOKEN(anon_sym_fallthrough);
      END_STATE();
    case 1126:
      if (lookahead == 'g') ADVANCE(1166);
      END_STATE();
    case 1127:
      if (lookahead == 'c') ADVANCE(1167);
      END_STATE();
    case 1128:
      ACCEPT_TOKEN(anon_sym_groupshared);
      END_STATE();
    case 1129:
      if (lookahead == 'd') ADVANCE(1168);
      END_STATE();
    case 1130:
      ACCEPT_TOKEN(anon_sym_interpolate);
      END_STATE();
    case 1131:
      if (lookahead == 'a') ADVANCE(1169);
      END_STATE();
    case 1132:
      ACCEPT_TOKEN(anon_sym_macro_rules);
      END_STATE();
    case 1133:
      if (lookahead == 't') ADVANCE(1170);
      END_STATE();
    case 1134:
      if (lookahead == 'v') ADVANCE(1171);
      END_STATE();
    case 1135:
      if (lookahead == 'u') ADVANCE(1172);
      END_STATE();
    case 1136:
      ACCEPT_TOKEN(anon_sym_perspective);
      END_STATE();
    case 1137:
      if (lookahead == 'n') ADVANCE(1173);
      END_STATE();
    case 1138:
      if (lookahead == '_') ADVANCE(1174);
      END_STATE();
    case 1139:
      ACCEPT_TOKEN(anon_sym_rgba16float);
      END_STATE();
    case 1140:
      ACCEPT_TOKEN(anon_sym_rgba32float);
      END_STATE();
    case 1141:
      if (lookahead == 'x') ADVANCE(1175);
      END_STATE();
    case 1142:
      ACCEPT_TOKEN(anon_sym_sample_mask);
      END_STATE();
    case 1143:
      if (lookahead == 'p') ADVANCE(1176);
      END_STATE();
    case 1144:
      if (lookahead == 'r') ADVANCE(1177);
      END_STATE();
    case 1145:
      ACCEPT_TOKEN(anon_sym_static_cast);
      END_STATE();
    case 1146:
      if (lookahead == 'a') ADVANCE(1178);
      END_STATE();
    case 1147:
      if (lookahead == 'e') ADVANCE(1179);
      END_STATE();
    case 1148:
      if (lookahead == 't') ADVANCE(1180);
      END_STATE();
    case 1149:
      if (lookahead == 't') ADVANCE(1181);
      END_STATE();
    case 1150:
      if (lookahead == 'r') ADVANCE(1182);
      END_STATE();
    case 1151:
      if (lookahead == 'l') ADVANCE(1183);
      END_STATE();
    case 1152:
      if (lookahead == 'x') ADVANCE(1184);
      END_STATE();
    case 1153:
      if (lookahead == 'd') ADVANCE(1185);
      END_STATE();
    case 1154:
      if (lookahead == 'i') ADVANCE(1186);
      END_STATE();
    case 1155:
      if (lookahead == 'r') ADVANCE(1187);
      END_STATE();
    case 1156:
      if (lookahead == 'r') ADVANCE(1188);
      END_STATE();
    case 1157:
      ACCEPT_TOKEN(anon_sym_DomainShader);
      END_STATE();
    case 1158:
      if (lookahead == 'e') ADVANCE(1189);
      END_STATE();
    case 1159:
      ACCEPT_TOKEN(anon_sym_asm_fragment);
      END_STATE();
    case 1160:
      if (lookahead == 'y') ADVANCE(1190);
      END_STATE();
    case 1161:
      ACCEPT_TOKEN(anon_sym_column_major);
      END_STATE();
    case 1162:
      if (lookahead == 'm') ADVANCE(1191);
      END_STATE();
    case 1163:
      ACCEPT_TOKEN(anon_sym_const_assert);
      END_STATE();
    case 1164:
      if (lookahead == 'l') ADVANCE(1192);
      END_STATE();
    case 1165:
      ACCEPT_TOKEN(anon_sym_dynamic_cast);
      END_STATE();
    case 1166:
      ACCEPT_TOKEN(anon_sym_front_facing);
      END_STATE();
    case 1167:
      if (lookahead == 'a') ADVANCE(1193);
      END_STATE();
    case 1168:
      if (lookahead == 'e') ADVANCE(1194);
      END_STATE();
    case 1169:
      if (lookahead == 't') ADVANCE(1195);
      END_STATE();
    case 1170:
      if (lookahead == 'i') ADVANCE(1196);
      END_STATE();
    case 1171:
      if (lookahead == 'e') ADVANCE(1197);
      END_STATE();
    case 1172:
      if (lookahead == 'p') ADVANCE(1198);
      END_STATE();
    case 1173:
      if (lookahead == 't') ADVANCE(1199);
      END_STATE();
    case 1174:
      if (lookahead == 'c') ADVANCE(1200);
      END_STATE();
    case 1175:
      ACCEPT_TOKEN(anon_sym_sample_index);
      END_STATE();
    case 1176:
      if (lookahead == 'a') ADVANCE(1201);
      END_STATE();
    case 1177:
      if (lookahead == 't') ADVANCE(1202);
      END_STATE();
    case 1178:
      if (lookahead == 'r') ADVANCE(1203);
      END_STATE();
    case 1179:
      ACCEPT_TOKEN(anon_sym_texture_cube);
      if (lookahead == '_') ADVANCE(1204);
      END_STATE();
    case 1180:
      if (lookahead == 'h') ADVANCE(1205);
      END_STATE();
    case 1181:
      if (lookahead == 'i') ADVANCE(1206);
      END_STATE();
    case 1182:
      if (lookahead == 'a') ADVANCE(1207);
      END_STATE();
    case 1183:
      ACCEPT_TOKEN(anon_sym_thread_local);
      END_STATE();
    case 1184:
      ACCEPT_TOKEN(anon_sym_vertex_index);
      END_STATE();
    case 1185:
      ACCEPT_TOKEN(anon_sym_workgroup_id);
      END_STATE();
    case 1186:
      if (lookahead == 'z') ADVANCE(1208);
      END_STATE();
    case 1187:
      ACCEPT_TOKEN(anon_sym_CompileShader);
      END_STATE();
    case 1188:
      ACCEPT_TOKEN(anon_sym_ComputeShader);
      END_STATE();
    case 1189:
      if (lookahead == 'r') ADVANCE(1209);
      END_STATE();
    case 1190:
      ACCEPT_TOKEN(anon_sym_binding_array);
      END_STATE();
    case 1191:
      if (lookahead == 'e') ADVANCE(1210);
      END_STATE();
    case 1192:
      if (lookahead == 'p') ADVANCE(1211);
      END_STATE();
    case 1193:
      if (lookahead == 't') ADVANCE(1212);
      END_STATE();
    case 1194:
      if (lookahead == 'x') ADVANCE(1213);
      END_STATE();
    case 1195:
      if (lookahead == 'i') ADVANCE(1214);
      END_STATE();
    case 1196:
      if (lookahead == 'o') ADVANCE(1215);
      END_STATE();
    case 1197:
      ACCEPT_TOKEN(anon_sym_noperspective);
      END_STATE();
    case 1198:
      if (lookahead == 's') ADVANCE(1216);
      END_STATE();
    case 1199:
      ACCEPT_TOKEN(anon_sym_pixelfragment);
      END_STATE();
    case 1200:
      if (lookahead == 'a') ADVANCE(1217);
      END_STATE();
    case 1201:
      if (lookahead == 'r') ADVANCE(1218);
      END_STATE();
    case 1202:
      ACCEPT_TOKEN(anon_sym_static_assert);
      END_STATE();
    case 1203:
      if (lookahead == 'r') ADVANCE(1219);
      END_STATE();
    case 1204:
      if (lookahead == 'a') ADVANCE(1220);
      END_STATE();
    case 1205:
      if (lookahead == '_') ADVANCE(1221);
      END_STATE();
    case 1206:
      if (lookahead == 's') ADVANCE(1222);
      END_STATE();
    case 1207:
      if (lookahead == 'g') ADVANCE(1223);
      END_STATE();
    case 1208:
      if (lookahead == 'e') ADVANCE(1224);
      END_STATE();
    case 1209:
      ACCEPT_TOKEN(anon_sym_GeometryShader);
      END_STATE();
    case 1210:
      if (lookahead == 'n') ADVANCE(1225);
      END_STATE();
    case 1211:
      if (lookahead == 'e') ADVANCE(1226);
      END_STATE();
    case 1212:
      if (lookahead == 'i') ADVANCE(1227);
      END_STATE();
    case 1213:
      ACCEPT_TOKEN(anon_sym_instance_index);
      END_STATE();
    case 1214:
      if (lookahead == 'o') ADVANCE(1228);
      END_STATE();
    case 1215:
      if (lookahead == 'n') ADVANCE(1229);
      END_STATE();
    case 1216:
      ACCEPT_TOKEN(anon_sym_num_workgroups);
      END_STATE();
    case 1217:
      if (lookahead == 's') ADVANCE(1230);
      END_STATE();
    case 1218:
      if (lookahead == 'i') ADVANCE(1231);
      END_STATE();
    case 1219:
      if (lookahead == 'a') ADVANCE(1232);
      END_STATE();
    case 1220:
      if (lookahead == 'r') ADVANCE(1233);
      END_STATE();
    case 1221:
      if (lookahead == '2') ADVANCE(1234);
      if (lookahead == 'c') ADVANCE(1235);
      if (lookahead == 'm') ADVANCE(1236);
      END_STATE();
    case 1222:
      if (lookahead == 'a') ADVANCE(1237);
      END_STATE();
    case 1223:
      if (lookahead == 'e') ADVANCE(1238);
      END_STATE();
    case 1224:
      ACCEPT_TOKEN(anon_sym_workgroup_size);
      END_STATE();
    case 1225:
      if (lookahead == 't') ADVANCE(1239);
      END_STATE();
    case 1226:
      if (lookahead == 'r') ADVANCE(1240);
      END_STATE();
    case 1227:
      if (lookahead == 'o') ADVANCE(1241);
      END_STATE();
    case 1228:
      if (lookahead == 'n') ADVANCE(1242);
      END_STATE();
    case 1229:
      ACCEPT_TOKEN(anon_sym_nointerpolation);
      END_STATE();
    case 1230:
      if (lookahead == 't') ADVANCE(1243);
      END_STATE();
    case 1231:
      if (lookahead == 's') ADVANCE(1244);
      END_STATE();
    case 1232:
      if (lookahead == 'y') ADVANCE(1245);
      END_STATE();
    case 1233:
      if (lookahead == 'r') ADVANCE(1246);
      END_STATE();
    case 1234:
      if (lookahead == 'd') ADVANCE(1247);
      END_STATE();
    case 1235:
      if (lookahead == 'u') ADVANCE(1248);
      END_STATE();
    case 1236:
      if (lookahead == 'u') ADVANCE(1249);
      END_STATE();
    case 1237:
      if (lookahead == 'm') ADVANCE(1250);
      END_STATE();
    case 1238:
      if (lookahead == '_') ADVANCE(1251);
      END_STATE();
    case 1239:
      ACCEPT_TOKEN(anon_sym_compile_fragment);
      END_STATE();
    case 1240:
      ACCEPT_TOKEN(anon_sym_demote_to_helper);
      END_STATE();
    case 1241:
      if (lookahead == 'n') ADVANCE(1252);
      END_STATE();
    case 1242:
      if (lookahead == '_') ADVANCE(1253);
      END_STATE();
    case 1243:
      ACCEPT_TOKEN(anon_sym_reinterpret_cast);
      END_STATE();
    case 1244:
      if (lookahead == 'o') ADVANCE(1254);
      END_STATE();
    case 1245:
      ACCEPT_TOKEN(anon_sym_texture_2d_array);
      END_STATE();
    case 1246:
      if (lookahead == 'a') ADVANCE(1255);
      END_STATE();
    case 1247:
      ACCEPT_TOKEN(anon_sym_texture_depth_2d);
      if (lookahead == '_') ADVANCE(1256);
      END_STATE();
    case 1248:
      if (lookahead == 'b') ADVANCE(1257);
      END_STATE();
    case 1249:
      if (lookahead == 'l') ADVANCE(1258);
      END_STATE();
    case 1250:
      if (lookahead == 'p') ADVANCE(1259);
      END_STATE();
    case 1251:
      if (lookahead == '1') ADVANCE(1260);
      if (lookahead == '2') ADVANCE(1261);
      if (lookahead == '3') ADVANCE(1262);
      END_STATE();
    case 1252:
      if (lookahead == '_') ADVANCE(1263);
      END_STATE();
    case 1253:
      if (lookahead == 'i') ADVANCE(1264);
      END_STATE();
    case 1254:
      if (lookahead == 'n') ADVANCE(1265);
      END_STATE();
    case 1255:
      if (lookahead == 'y') ADVANCE(1266);
      END_STATE();
    case 1256:
      if (lookahead == 'a') ADVANCE(1267);
      END_STATE();
    case 1257:
      if (lookahead == 'e') ADVANCE(1268);
      END_STATE();
    case 1258:
      if (lookahead == 't') ADVANCE(1269);
      END_STATE();
    case 1259:
      if (lookahead == 'l') ADVANCE(1270);
      END_STATE();
    case 1260:
      if (lookahead == 'd') ADVANCE(1271);
      END_STATE();
    case 1261:
      if (lookahead == 'd') ADVANCE(1272);
      END_STATE();
    case 1262:
      if (lookahead == 'd') ADVANCE(1273);
      END_STATE();
    case 1263:
      if (lookahead == 'i') ADVANCE(1274);
      END_STATE();
    case 1264:
      if (lookahead == 'd') ADVANCE(1275);
      if (lookahead == 'n') ADVANCE(1276);
      END_STATE();
    case 1265:
      ACCEPT_TOKEN(anon_sym_sampler_comparison);
      END_STATE();
    case 1266:
      ACCEPT_TOKEN(anon_sym_texture_cube_array);
      END_STATE();
    case 1267:
      if (lookahead == 'r') ADVANCE(1277);
      END_STATE();
    case 1268:
      ACCEPT_TOKEN(anon_sym_texture_depth_cube);
      if (lookahead == '_') ADVANCE(1278);
      END_STATE();
    case 1269:
      if (lookahead == 'i') ADVANCE(1279);
      END_STATE();
    case 1270:
      if (lookahead == 'e') ADVANCE(1280);
      END_STATE();
    case 1271:
      ACCEPT_TOKEN(anon_sym_texture_storage_1d);
      END_STATE();
    case 1272:
      ACCEPT_TOKEN(anon_sym_texture_storage_2d);
      if (lookahead == '_') ADVANCE(1281);
      END_STATE();
    case 1273:
      ACCEPT_TOKEN(anon_sym_texture_storage_3d);
      END_STATE();
    case 1274:
      if (lookahead == 'd') ADVANCE(1282);
      END_STATE();
    case 1275:
      ACCEPT_TOKEN(anon_sym_local_invocation_id);
      END_STATE();
    case 1276:
      if (lookahead == 'd') ADVANCE(1283);
      END_STATE();
    case 1277:
      if (lookahead == 'r') ADVANCE(1284);
      END_STATE();
    case 1278:
      if (lookahead == 'a') ADVANCE(1285);
      END_STATE();
    case 1279:
      if (lookahead == 's') ADVANCE(1286);
      END_STATE();
    case 1280:
      if (lookahead == 'd') ADVANCE(1287);
      END_STATE();
    case 1281:
      if (lookahead == 'a') ADVANCE(1288);
      END_STATE();
    case 1282:
      ACCEPT_TOKEN(anon_sym_global_invocation_id);
      END_STATE();
    case 1283:
      if (lookahead == 'e') ADVANCE(1289);
      END_STATE();
    case 1284:
      if (lookahead == 'a') ADVANCE(1290);
      END_STATE();
    case 1285:
      if (lookahead == 'r') ADVANCE(1291);
      END_STATE();
    case 1286:
      if (lookahead == 'a') ADVANCE(1292);
      END_STATE();
    case 1287:
      if (lookahead == '_') ADVANCE(1293);
      END_STATE();
    case 1288:
      if (lookahead == 'r') ADVANCE(1294);
      END_STATE();
    case 1289:
      if (lookahead == 'x') ADVANCE(1295);
      END_STATE();
    case 1290:
      if (lookahead == 'y') ADVANCE(1296);
      END_STATE();
    case 1291:
      if (lookahead == 'r') ADVANCE(1297);
      END_STATE();
    case 1292:
      if (lookahead == 'm') ADVANCE(1298);
      END_STATE();
    case 1293:
      if (lookahead == '2') ADVANCE(1299);
      END_STATE();
    case 1294:
      if (lookahead == 'r') ADVANCE(1300);
      END_STATE();
    case 1295:
      ACCEPT_TOKEN(anon_sym_local_invocation_index);
      END_STATE();
    case 1296:
      ACCEPT_TOKEN(anon_sym_texture_depth_2d_array);
      END_STATE();
    case 1297:
      if (lookahead == 'a') ADVANCE(1301);
      END_STATE();
    case 1298:
      if (lookahead == 'p') ADVANCE(1302);
      END_STATE();
    case 1299:
      if (lookahead == 'd') ADVANCE(1303);
      END_STATE();
    case 1300:
      if (lookahead == 'a') ADVANCE(1304);
      END_STATE();
    case 1301:
      if (lookahead == 'y') ADVANCE(1305);
      END_STATE();
    case 1302:
      if (lookahead == 'l') ADVANCE(1306);
      END_STATE();
    case 1303:
      ACCEPT_TOKEN(sym_multisampled_texture_type);
      END_STATE();
    case 1304:
      if (lookahead == 'y') ADVANCE(1307);
      END_STATE();
    case 1305:
      ACCEPT_TOKEN(anon_sym_texture_depth_cube_array);
      END_STATE();
    case 1306:
      if (lookahead == 'e') ADVANCE(1308);
      END_STATE();
    case 1307:
      ACCEPT_TOKEN(anon_sym_texture_storage_2d_array);
      END_STATE();
    case 1308:
      if (lookahead == 'd') ADVANCE(1309);
      END_STATE();
    case 1309:
      if (lookahead == '_') ADVANCE(1310);
      END_STATE();
    case 1310:
      if (lookahead == '2') ADVANCE(1311);
      END_STATE();
    case 1311:
      if (lookahead == 'd') ADVANCE(1312);
      END_STATE();
    case 1312:
      ACCEPT_TOKEN(anon_sym_texture_depth_multisampled_2d);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 0, .external_lex_state = 1},
  [2] = {.lex_state = 89, .external_lex_state = 1},
  [3] = {.lex_state = 89, .external_lex_state = 1},
  [4] = {.lex_state = 89, .external_lex_state = 1},
  [5] = {.lex_state = 89, .external_lex_state = 1},
  [6] = {.lex_state = 89, .external_lex_state = 1},
  [7] = {.lex_state = 89, .external_lex_state = 1},
  [8] = {.lex_state = 89, .external_lex_state = 1},
  [9] = {.lex_state = 89, .external_lex_state = 1},
  [10] = {.lex_state = 89, .external_lex_state = 1},
  [11] = {.lex_state = 89, .external_lex_state = 1},
  [12] = {.lex_state = 89, .external_lex_state = 1},
  [13] = {.lex_state = 89, .external_lex_state = 1},
  [14] = {.lex_state = 89, .external_lex_state = 1},
  [15] = {.lex_state = 89, .external_lex_state = 1},
  [16] = {.lex_state = 89, .external_lex_state = 1},
  [17] = {.lex_state = 89, .external_lex_state = 1},
  [18] = {.lex_state = 89, .external_lex_state = 1},
  [19] = {.lex_state = 89, .external_lex_state = 1},
  [20] = {.lex_state = 89, .external_lex_state = 1},
  [21] = {.lex_state = 89, .external_lex_state = 1},
  [22] = {.lex_state = 89, .external_lex_state = 1},
  [23] = {.lex_state = 89, .external_lex_state = 1},
  [24] = {.lex_state = 89, .external_lex_state = 1},
  [25] = {.lex_state = 89, .external_lex_state = 1},
  [26] = {.lex_state = 89, .external_lex_state = 1},
  [27] = {.lex_state = 89, .external_lex_state = 1},
  [28] = {.lex_state = 89, .external_lex_state = 1},
  [29] = {.lex_state = 89, .external_lex_state = 1},
  [30] = {.lex_state = 89, .external_lex_state = 1},
  [31] = {.lex_state = 89, .external_lex_state = 1},
  [32] = {.lex_state = 89, .external_lex_state = 1},
  [33] = {.lex_state = 89, .external_lex_state = 1},
  [34] = {.lex_state = 89, .external_lex_state = 1},
  [35] = {.lex_state = 89, .external_lex_state = 1},
  [36] = {.lex_state = 89, .external_lex_state = 1},
  [37] = {.lex_state = 89, .external_lex_state = 1},
  [38] = {.lex_state = 89, .external_lex_state = 1},
  [39] = {.lex_state = 89, .external_lex_state = 1},
  [40] = {.lex_state = 89, .external_lex_state = 1},
  [41] = {.lex_state = 89, .external_lex_state = 1},
  [42] = {.lex_state = 89, .external_lex_state = 1},
  [43] = {.lex_state = 89, .external_lex_state = 1},
  [44] = {.lex_state = 89, .external_lex_state = 1},
  [45] = {.lex_state = 89, .external_lex_state = 1},
  [46] = {.lex_state = 89, .external_lex_state = 1},
  [47] = {.lex_state = 89, .external_lex_state = 1},
  [48] = {.lex_state = 89, .external_lex_state = 1},
  [49] = {.lex_state = 89, .external_lex_state = 1},
  [50] = {.lex_state = 89, .external_lex_state = 1},
  [51] = {.lex_state = 89, .external_lex_state = 1},
  [52] = {.lex_state = 89, .external_lex_state = 1},
  [53] = {.lex_state = 89, .external_lex_state = 1},
  [54] = {.lex_state = 89, .external_lex_state = 1},
  [55] = {.lex_state = 89, .external_lex_state = 1},
  [56] = {.lex_state = 89, .external_lex_state = 1},
  [57] = {.lex_state = 89, .external_lex_state = 1},
  [58] = {.lex_state = 89, .external_lex_state = 1},
  [59] = {.lex_state = 89, .external_lex_state = 1},
  [60] = {.lex_state = 89, .external_lex_state = 1},
  [61] = {.lex_state = 89, .external_lex_state = 1},
  [62] = {.lex_state = 89, .external_lex_state = 1},
  [63] = {.lex_state = 89, .external_lex_state = 1},
  [64] = {.lex_state = 89, .external_lex_state = 1},
  [65] = {.lex_state = 89, .external_lex_state = 1},
  [66] = {.lex_state = 89, .external_lex_state = 1},
  [67] = {.lex_state = 89, .external_lex_state = 1},
  [68] = {.lex_state = 89, .external_lex_state = 1},
  [69] = {.lex_state = 89, .external_lex_state = 1},
  [70] = {.lex_state = 89, .external_lex_state = 1},
  [71] = {.lex_state = 89, .external_lex_state = 1},
  [72] = {.lex_state = 89, .external_lex_state = 1},
  [73] = {.lex_state = 89, .external_lex_state = 1},
  [74] = {.lex_state = 89, .external_lex_state = 1},
  [75] = {.lex_state = 89, .external_lex_state = 1},
  [76] = {.lex_state = 89, .external_lex_state = 1},
  [77] = {.lex_state = 89, .external_lex_state = 1},
  [78] = {.lex_state = 89, .external_lex_state = 1},
  [79] = {.lex_state = 89, .external_lex_state = 1},
  [80] = {.lex_state = 89, .external_lex_state = 1},
  [81] = {.lex_state = 89, .external_lex_state = 1},
  [82] = {.lex_state = 89, .external_lex_state = 1},
  [83] = {.lex_state = 89, .external_lex_state = 1},
  [84] = {.lex_state = 89, .external_lex_state = 1},
  [85] = {.lex_state = 89, .external_lex_state = 1},
  [86] = {.lex_state = 89, .external_lex_state = 1},
  [87] = {.lex_state = 89, .external_lex_state = 1},
  [88] = {.lex_state = 89, .external_lex_state = 1},
  [89] = {.lex_state = 89, .external_lex_state = 1},
  [90] = {.lex_state = 89, .external_lex_state = 1},
  [91] = {.lex_state = 89, .external_lex_state = 1},
  [92] = {.lex_state = 89, .external_lex_state = 1},
  [93] = {.lex_state = 89, .external_lex_state = 1},
  [94] = {.lex_state = 89, .external_lex_state = 1},
  [95] = {.lex_state = 0, .external_lex_state = 1},
  [96] = {.lex_state = 0, .external_lex_state = 1},
  [97] = {.lex_state = 0, .external_lex_state = 1},
  [98] = {.lex_state = 0, .external_lex_state = 1},
  [99] = {.lex_state = 0, .external_lex_state = 1},
  [100] = {.lex_state = 0, .external_lex_state = 1},
  [101] = {.lex_state = 0, .external_lex_state = 1},
  [102] = {.lex_state = 0, .external_lex_state = 1},
  [103] = {.lex_state = 0, .external_lex_state = 1},
  [104] = {.lex_state = 0, .external_lex_state = 1},
  [105] = {.lex_state = 0, .external_lex_state = 1},
  [106] = {.lex_state = 0, .external_lex_state = 1},
  [107] = {.lex_state = 0, .external_lex_state = 1},
  [108] = {.lex_state = 0, .external_lex_state = 1},
  [109] = {.lex_state = 0, .external_lex_state = 1},
  [110] = {.lex_state = 0, .external_lex_state = 1},
  [111] = {.lex_state = 0, .external_lex_state = 1},
  [112] = {.lex_state = 0, .external_lex_state = 1},
  [113] = {.lex_state = 0, .external_lex_state = 1},
  [114] = {.lex_state = 0, .external_lex_state = 1},
  [115] = {.lex_state = 0, .external_lex_state = 1},
  [116] = {.lex_state = 0, .external_lex_state = 1},
  [117] = {.lex_state = 0, .external_lex_state = 1},
  [118] = {.lex_state = 0, .external_lex_state = 1},
  [119] = {.lex_state = 0, .external_lex_state = 1},
  [120] = {.lex_state = 1, .external_lex_state = 1},
  [121] = {.lex_state = 2, .external_lex_state = 1},
  [122] = {.lex_state = 2, .external_lex_state = 1},
  [123] = {.lex_state = 2, .external_lex_state = 1},
  [124] = {.lex_state = 2, .external_lex_state = 1},
  [125] = {.lex_state = 2, .external_lex_state = 1},
  [126] = {.lex_state = 2, .external_lex_state = 1},
  [127] = {.lex_state = 2, .external_lex_state = 1},
  [128] = {.lex_state = 2, .external_lex_state = 1},
  [129] = {.lex_state = 2, .external_lex_state = 1},
  [130] = {.lex_state = 2, .external_lex_state = 1},
  [131] = {.lex_state = 2, .external_lex_state = 1},
  [132] = {.lex_state = 2, .external_lex_state = 1},
  [133] = {.lex_state = 2, .external_lex_state = 1},
  [134] = {.lex_state = 2, .external_lex_state = 1},
  [135] = {.lex_state = 2, .external_lex_state = 1},
  [136] = {.lex_state = 2, .external_lex_state = 1},
  [137] = {.lex_state = 2, .external_lex_state = 1},
  [138] = {.lex_state = 2, .external_lex_state = 1},
  [139] = {.lex_state = 2, .external_lex_state = 1},
  [140] = {.lex_state = 2, .external_lex_state = 1},
  [141] = {.lex_state = 0, .external_lex_state = 1},
  [142] = {.lex_state = 2, .external_lex_state = 1},
  [143] = {.lex_state = 2, .external_lex_state = 1},
  [144] = {.lex_state = 2, .external_lex_state = 1},
  [145] = {.lex_state = 2, .external_lex_state = 1},
  [146] = {.lex_state = 2, .external_lex_state = 1},
  [147] = {.lex_state = 2, .external_lex_state = 1},
  [148] = {.lex_state = 5, .external_lex_state = 1},
  [149] = {.lex_state = 5, .external_lex_state = 1},
  [150] = {.lex_state = 5, .external_lex_state = 1},
  [151] = {.lex_state = 3, .external_lex_state = 1},
  [152] = {.lex_state = 3, .external_lex_state = 1},
  [153] = {.lex_state = 3, .external_lex_state = 1},
  [154] = {.lex_state = 3, .external_lex_state = 1},
  [155] = {.lex_state = 5, .external_lex_state = 1},
  [156] = {.lex_state = 5, .external_lex_state = 1},
  [157] = {.lex_state = 3, .external_lex_state = 1},
  [158] = {.lex_state = 3, .external_lex_state = 1},
  [159] = {.lex_state = 3, .external_lex_state = 1},
  [160] = {.lex_state = 0, .external_lex_state = 1},
  [161] = {.lex_state = 3, .external_lex_state = 1},
  [162] = {.lex_state = 0, .external_lex_state = 1},
  [163] = {.lex_state = 3, .external_lex_state = 1},
  [164] = {.lex_state = 3, .external_lex_state = 1},
  [165] = {.lex_state = 3, .external_lex_state = 1},
  [166] = {.lex_state = 0, .external_lex_state = 1},
  [167] = {.lex_state = 3, .external_lex_state = 1},
  [168] = {.lex_state = 3, .external_lex_state = 1},
  [169] = {.lex_state = 3, .external_lex_state = 1},
  [170] = {.lex_state = 3, .external_lex_state = 1},
  [171] = {.lex_state = 3, .external_lex_state = 1},
  [172] = {.lex_state = 3, .external_lex_state = 1},
  [173] = {.lex_state = 2, .external_lex_state = 1},
  [174] = {.lex_state = 3, .external_lex_state = 1},
  [175] = {.lex_state = 3, .external_lex_state = 1},
  [176] = {.lex_state = 3, .external_lex_state = 1},
  [177] = {.lex_state = 3, .external_lex_state = 1},
  [178] = {.lex_state = 5, .external_lex_state = 1},
  [179] = {.lex_state = 3, .external_lex_state = 1},
  [180] = {.lex_state = 3, .external_lex_state = 1},
  [181] = {.lex_state = 5, .external_lex_state = 1},
  [182] = {.lex_state = 3, .external_lex_state = 1},
  [183] = {.lex_state = 3, .external_lex_state = 1},
  [184] = {.lex_state = 3, .external_lex_state = 1},
  [185] = {.lex_state = 3, .external_lex_state = 1},
  [186] = {.lex_state = 3, .external_lex_state = 1},
  [187] = {.lex_state = 3, .external_lex_state = 1},
  [188] = {.lex_state = 3, .external_lex_state = 1},
  [189] = {.lex_state = 0, .external_lex_state = 1},
  [190] = {.lex_state = 4, .external_lex_state = 1},
  [191] = {.lex_state = 4, .external_lex_state = 1},
  [192] = {.lex_state = 4, .external_lex_state = 1},
  [193] = {.lex_state = 4, .external_lex_state = 1},
  [194] = {.lex_state = 4, .external_lex_state = 1},
  [195] = {.lex_state = 4, .external_lex_state = 1},
  [196] = {.lex_state = 4, .external_lex_state = 1},
  [197] = {.lex_state = 4, .external_lex_state = 1},
  [198] = {.lex_state = 4, .external_lex_state = 1},
  [199] = {.lex_state = 4, .external_lex_state = 1},
  [200] = {.lex_state = 4, .external_lex_state = 1},
  [201] = {.lex_state = 4, .external_lex_state = 1},
  [202] = {.lex_state = 4, .external_lex_state = 1},
  [203] = {.lex_state = 4, .external_lex_state = 1},
  [204] = {.lex_state = 4, .external_lex_state = 1},
  [205] = {.lex_state = 3, .external_lex_state = 1},
  [206] = {.lex_state = 6, .external_lex_state = 1},
  [207] = {.lex_state = 6, .external_lex_state = 1},
  [208] = {.lex_state = 6, .external_lex_state = 1},
  [209] = {.lex_state = 6, .external_lex_state = 1},
  [210] = {.lex_state = 89, .external_lex_state = 1},
  [211] = {.lex_state = 89, .external_lex_state = 1},
  [212] = {.lex_state = 89, .external_lex_state = 1},
  [213] = {.lex_state = 1, .external_lex_state = 1},
  [214] = {.lex_state = 3, .external_lex_state = 1},
  [215] = {.lex_state = 0, .external_lex_state = 1},
  [216] = {.lex_state = 6, .external_lex_state = 1},
  [217] = {.lex_state = 1, .external_lex_state = 1},
  [218] = {.lex_state = 0, .external_lex_state = 1},
  [219] = {.lex_state = 6, .external_lex_state = 1},
  [220] = {.lex_state = 3, .external_lex_state = 1},
  [221] = {.lex_state = 1, .external_lex_state = 1},
  [222] = {.lex_state = 0, .external_lex_state = 1},
  [223] = {.lex_state = 4, .external_lex_state = 1},
  [224] = {.lex_state = 4, .external_lex_state = 1},
  [225] = {.lex_state = 4, .external_lex_state = 1},
  [226] = {.lex_state = 4, .external_lex_state = 1},
  [227] = {.lex_state = 0, .external_lex_state = 1},
  [228] = {.lex_state = 0, .external_lex_state = 1},
  [229] = {.lex_state = 4, .external_lex_state = 1},
  [230] = {.lex_state = 4, .external_lex_state = 1},
  [231] = {.lex_state = 5, .external_lex_state = 1},
  [232] = {.lex_state = 0, .external_lex_state = 1},
  [233] = {.lex_state = 5, .external_lex_state = 1},
  [234] = {.lex_state = 1, .external_lex_state = 1},
  [235] = {.lex_state = 5, .external_lex_state = 1},
  [236] = {.lex_state = 5, .external_lex_state = 1},
  [237] = {.lex_state = 0, .external_lex_state = 1},
  [238] = {.lex_state = 0, .external_lex_state = 1},
  [239] = {.lex_state = 0, .external_lex_state = 1},
  [240] = {.lex_state = 0, .external_lex_state = 1},
  [241] = {.lex_state = 0, .external_lex_state = 1},
  [242] = {.lex_state = 0, .external_lex_state = 1},
  [243] = {.lex_state = 0, .external_lex_state = 1},
  [244] = {.lex_state = 0, .external_lex_state = 1},
  [245] = {.lex_state = 4, .external_lex_state = 1},
  [246] = {.lex_state = 89, .external_lex_state = 1},
  [247] = {.lex_state = 89, .external_lex_state = 1},
  [248] = {.lex_state = 89, .external_lex_state = 1},
  [249] = {.lex_state = 89, .external_lex_state = 1},
  [250] = {.lex_state = 0, .external_lex_state = 1},
  [251] = {.lex_state = 89, .external_lex_state = 1},
  [252] = {.lex_state = 89, .external_lex_state = 1},
  [253] = {.lex_state = 89, .external_lex_state = 1},
  [254] = {.lex_state = 89, .external_lex_state = 1},
  [255] = {.lex_state = 89, .external_lex_state = 1},
  [256] = {.lex_state = 89, .external_lex_state = 1},
  [257] = {.lex_state = 89, .external_lex_state = 1},
  [258] = {.lex_state = 89, .external_lex_state = 1},
  [259] = {.lex_state = 89, .external_lex_state = 1},
  [260] = {.lex_state = 89, .external_lex_state = 1},
  [261] = {.lex_state = 0, .external_lex_state = 1},
  [262] = {.lex_state = 0, .external_lex_state = 1},
  [263] = {.lex_state = 0, .external_lex_state = 1},
  [264] = {.lex_state = 89, .external_lex_state = 1},
  [265] = {.lex_state = 89, .external_lex_state = 1},
  [266] = {.lex_state = 89, .external_lex_state = 1},
  [267] = {.lex_state = 0, .external_lex_state = 1},
  [268] = {.lex_state = 0, .external_lex_state = 1},
  [269] = {.lex_state = 89, .external_lex_state = 1},
  [270] = {.lex_state = 0, .external_lex_state = 1},
  [271] = {.lex_state = 0, .external_lex_state = 1},
  [272] = {.lex_state = 0, .external_lex_state = 1},
  [273] = {.lex_state = 89, .external_lex_state = 1},
  [274] = {.lex_state = 89, .external_lex_state = 1},
  [275] = {.lex_state = 89, .external_lex_state = 1},
  [276] = {.lex_state = 0, .external_lex_state = 1},
  [277] = {.lex_state = 0, .external_lex_state = 1},
  [278] = {.lex_state = 0, .external_lex_state = 1},
  [279] = {.lex_state = 89, .external_lex_state = 1},
  [280] = {.lex_state = 89, .external_lex_state = 1},
  [281] = {.lex_state = 0, .external_lex_state = 1},
  [282] = {.lex_state = 0, .external_lex_state = 1},
  [283] = {.lex_state = 0, .external_lex_state = 1},
  [284] = {.lex_state = 0, .external_lex_state = 1},
  [285] = {.lex_state = 89, .external_lex_state = 1},
  [286] = {.lex_state = 89, .external_lex_state = 1},
  [287] = {.lex_state = 0, .external_lex_state = 1},
  [288] = {.lex_state = 0, .external_lex_state = 1},
  [289] = {.lex_state = 0, .external_lex_state = 1},
  [290] = {.lex_state = 0, .external_lex_state = 1},
  [291] = {.lex_state = 89, .external_lex_state = 1},
  [292] = {.lex_state = 0, .external_lex_state = 1},
  [293] = {.lex_state = 0, .external_lex_state = 1},
  [294] = {.lex_state = 0, .external_lex_state = 1},
  [295] = {.lex_state = 0, .external_lex_state = 1},
  [296] = {.lex_state = 0, .external_lex_state = 1},
  [297] = {.lex_state = 0, .external_lex_state = 1},
  [298] = {.lex_state = 1, .external_lex_state = 1},
  [299] = {.lex_state = 0, .external_lex_state = 1},
  [300] = {.lex_state = 0, .external_lex_state = 1},
  [301] = {.lex_state = 0, .external_lex_state = 1},
  [302] = {.lex_state = 89, .external_lex_state = 1},
  [303] = {.lex_state = 0, .external_lex_state = 1},
  [304] = {.lex_state = 0, .external_lex_state = 1},
  [305] = {.lex_state = 0, .external_lex_state = 1},
  [306] = {.lex_state = 0, .external_lex_state = 1},
  [307] = {.lex_state = 0, .external_lex_state = 1},
  [308] = {.lex_state = 0, .external_lex_state = 1},
  [309] = {.lex_state = 0, .external_lex_state = 1},
  [310] = {.lex_state = 0, .external_lex_state = 1},
  [311] = {.lex_state = 0, .external_lex_state = 1},
  [312] = {.lex_state = 0, .external_lex_state = 1},
  [313] = {.lex_state = 89, .external_lex_state = 1},
  [314] = {.lex_state = 0, .external_lex_state = 1},
  [315] = {.lex_state = 0, .external_lex_state = 1},
  [316] = {.lex_state = 0, .external_lex_state = 1},
  [317] = {.lex_state = 0, .external_lex_state = 1},
  [318] = {.lex_state = 0, .external_lex_state = 1},
  [319] = {.lex_state = 0, .external_lex_state = 1},
  [320] = {.lex_state = 0, .external_lex_state = 1},
  [321] = {.lex_state = 0, .external_lex_state = 1},
  [322] = {.lex_state = 0, .external_lex_state = 1},
  [323] = {.lex_state = 0, .external_lex_state = 1},
  [324] = {.lex_state = 0, .external_lex_state = 1},
  [325] = {.lex_state = 0, .external_lex_state = 1},
  [326] = {.lex_state = 0, .external_lex_state = 1},
  [327] = {.lex_state = 0, .external_lex_state = 1},
  [328] = {.lex_state = 0, .external_lex_state = 1},
  [329] = {.lex_state = 0, .external_lex_state = 1},
  [330] = {.lex_state = 0, .external_lex_state = 1},
  [331] = {.lex_state = 0, .external_lex_state = 1},
  [332] = {.lex_state = 0, .external_lex_state = 1},
  [333] = {.lex_state = 0, .external_lex_state = 1},
  [334] = {.lex_state = 0, .external_lex_state = 1},
  [335] = {.lex_state = 0, .external_lex_state = 1},
  [336] = {.lex_state = 0, .external_lex_state = 1},
  [337] = {.lex_state = 0, .external_lex_state = 1},
  [338] = {.lex_state = 0, .external_lex_state = 1},
  [339] = {.lex_state = 89, .external_lex_state = 1},
  [340] = {.lex_state = 0, .external_lex_state = 1},
  [341] = {.lex_state = 0, .external_lex_state = 1},
  [342] = {.lex_state = 0, .external_lex_state = 1},
  [343] = {.lex_state = 0, .external_lex_state = 1},
  [344] = {.lex_state = 0, .external_lex_state = 1},
  [345] = {.lex_state = 0, .external_lex_state = 1},
  [346] = {.lex_state = 0, .external_lex_state = 1},
  [347] = {.lex_state = 0, .external_lex_state = 1},
  [348] = {.lex_state = 0, .external_lex_state = 1},
  [349] = {.lex_state = 89, .external_lex_state = 1},
  [350] = {.lex_state = 0, .external_lex_state = 1},
  [351] = {.lex_state = 89, .external_lex_state = 1},
  [352] = {.lex_state = 0, .external_lex_state = 1},
  [353] = {.lex_state = 89, .external_lex_state = 1},
  [354] = {.lex_state = 89, .external_lex_state = 1},
  [355] = {.lex_state = 0, .external_lex_state = 1},
  [356] = {.lex_state = 89, .external_lex_state = 1},
  [357] = {.lex_state = 0, .external_lex_state = 1},
  [358] = {.lex_state = 89, .external_lex_state = 1},
  [359] = {.lex_state = 89, .external_lex_state = 1},
  [360] = {.lex_state = 0, .external_lex_state = 1},
  [361] = {.lex_state = 0, .external_lex_state = 1},
  [362] = {.lex_state = 0, .external_lex_state = 1},
  [363] = {.lex_state = 0, .external_lex_state = 1},
  [364] = {.lex_state = 0, .external_lex_state = 1},
  [365] = {.lex_state = 0, .external_lex_state = 1},
  [366] = {.lex_state = 0, .external_lex_state = 1},
  [367] = {.lex_state = 0, .external_lex_state = 1},
  [368] = {.lex_state = 0, .external_lex_state = 1},
  [369] = {.lex_state = 89, .external_lex_state = 1},
  [370] = {.lex_state = 0, .external_lex_state = 1},
  [371] = {.lex_state = 89, .external_lex_state = 1},
  [372] = {.lex_state = 0, .external_lex_state = 1},
  [373] = {.lex_state = 89, .external_lex_state = 1},
  [374] = {.lex_state = 5, .external_lex_state = 1},
  [375] = {.lex_state = 5, .external_lex_state = 1},
  [376] = {.lex_state = 0, .external_lex_state = 1},
  [377] = {.lex_state = 0, .external_lex_state = 1},
  [378] = {.lex_state = 89, .external_lex_state = 1},
  [379] = {.lex_state = 89, .external_lex_state = 1},
  [380] = {.lex_state = 89, .external_lex_state = 1},
  [381] = {.lex_state = 0, .external_lex_state = 1},
  [382] = {.lex_state = 0, .external_lex_state = 1},
  [383] = {.lex_state = 0, .external_lex_state = 1},
  [384] = {.lex_state = 0, .external_lex_state = 1},
  [385] = {.lex_state = 89, .external_lex_state = 1},
  [386] = {.lex_state = 0, .external_lex_state = 1},
  [387] = {.lex_state = 0, .external_lex_state = 1},
  [388] = {.lex_state = 89, .external_lex_state = 1},
  [389] = {.lex_state = 0, .external_lex_state = 1},
  [390] = {.lex_state = 89, .external_lex_state = 1},
  [391] = {.lex_state = 0, .external_lex_state = 1},
  [392] = {.lex_state = 89, .external_lex_state = 1},
  [393] = {.lex_state = 0, .external_lex_state = 1},
  [394] = {.lex_state = 0, .external_lex_state = 1},
  [395] = {.lex_state = 0, .external_lex_state = 1},
  [396] = {.lex_state = 0, .external_lex_state = 1},
  [397] = {.lex_state = 0, .external_lex_state = 1},
  [398] = {.lex_state = 0, .external_lex_state = 1},
  [399] = {.lex_state = 0, .external_lex_state = 1},
  [400] = {.lex_state = 0, .external_lex_state = 1},
  [401] = {.lex_state = 89, .external_lex_state = 1},
  [402] = {.lex_state = 0, .external_lex_state = 1},
  [403] = {.lex_state = 0, .external_lex_state = 1},
  [404] = {.lex_state = 0, .external_lex_state = 1},
  [405] = {.lex_state = 0, .external_lex_state = 1},
  [406] = {.lex_state = 0, .external_lex_state = 1},
  [407] = {.lex_state = 192, .external_lex_state = 1},
  [408] = {.lex_state = 0, .external_lex_state = 1},
  [409] = {.lex_state = 0, .external_lex_state = 1},
  [410] = {.lex_state = 0, .external_lex_state = 1},
  [411] = {.lex_state = 89, .external_lex_state = 1},
  [412] = {.lex_state = 89, .external_lex_state = 1},
  [413] = {.lex_state = 0, .external_lex_state = 1},
  [414] = {.lex_state = 0, .external_lex_state = 1},
  [415] = {.lex_state = 89, .external_lex_state = 1},
  [416] = {.lex_state = 0, .external_lex_state = 1},
  [417] = {.lex_state = 0, .external_lex_state = 1},
  [418] = {.lex_state = 89, .external_lex_state = 1},
  [419] = {.lex_state = 0, .external_lex_state = 1},
  [420] = {.lex_state = 0, .external_lex_state = 1},
  [421] = {.lex_state = 0, .external_lex_state = 1},
  [422] = {.lex_state = 0, .external_lex_state = 1},
  [423] = {.lex_state = 0, .external_lex_state = 1},
  [424] = {.lex_state = 0, .external_lex_state = 1},
  [425] = {.lex_state = 0, .external_lex_state = 1},
  [426] = {.lex_state = 89, .external_lex_state = 1},
  [427] = {.lex_state = 89, .external_lex_state = 1},
  [428] = {.lex_state = 89, .external_lex_state = 1},
  [429] = {.lex_state = 0, .external_lex_state = 1},
  [430] = {.lex_state = 0, .external_lex_state = 1},
  [431] = {.lex_state = 0, .external_lex_state = 1},
  [432] = {.lex_state = 0, .external_lex_state = 1},
  [433] = {.lex_state = 0, .external_lex_state = 1},
  [434] = {.lex_state = 0, .external_lex_state = 1},
  [435] = {.lex_state = 89, .external_lex_state = 1},
  [436] = {.lex_state = 0, .external_lex_state = 1},
  [437] = {.lex_state = 89, .external_lex_state = 1},
  [438] = {.lex_state = 0, .external_lex_state = 1},
  [439] = {.lex_state = 0, .external_lex_state = 1},
  [440] = {.lex_state = 0, .external_lex_state = 1},
  [441] = {.lex_state = 89, .external_lex_state = 1},
  [442] = {.lex_state = 0, .external_lex_state = 1},
  [443] = {.lex_state = 0, .external_lex_state = 1},
  [444] = {.lex_state = 89, .external_lex_state = 1},
  [445] = {.lex_state = 89, .external_lex_state = 1},
  [446] = {.lex_state = 89, .external_lex_state = 1},
  [447] = {.lex_state = 0, .external_lex_state = 1},
  [448] = {.lex_state = 0, .external_lex_state = 1},
  [449] = {.lex_state = 0, .external_lex_state = 1},
  [450] = {.lex_state = 0, .external_lex_state = 1},
  [451] = {.lex_state = 0, .external_lex_state = 1},
  [452] = {.lex_state = 89, .external_lex_state = 1},
  [453] = {.lex_state = 89, .external_lex_state = 1},
  [454] = {.lex_state = 89, .external_lex_state = 1},
  [455] = {.lex_state = 89, .external_lex_state = 1},
  [456] = {.lex_state = 89, .external_lex_state = 1},
  [457] = {.lex_state = 89, .external_lex_state = 1},
  [458] = {.lex_state = 89, .external_lex_state = 1},
  [459] = {.lex_state = 89, .external_lex_state = 1},
  [460] = {.lex_state = 0, .external_lex_state = 1},
  [461] = {.lex_state = 0, .external_lex_state = 1},
  [462] = {.lex_state = 0, .external_lex_state = 1},
  [463] = {.lex_state = 0, .external_lex_state = 1},
  [464] = {.lex_state = 0, .external_lex_state = 1},
  [465] = {.lex_state = 89, .external_lex_state = 1},
  [466] = {.lex_state = 0, .external_lex_state = 1},
  [467] = {.lex_state = 0, .external_lex_state = 1},
  [468] = {.lex_state = 0, .external_lex_state = 1},
  [469] = {.lex_state = 0, .external_lex_state = 1},
  [470] = {.lex_state = 0, .external_lex_state = 1},
  [471] = {.lex_state = 0, .external_lex_state = 1},
  [472] = {.lex_state = 0, .external_lex_state = 1},
  [473] = {.lex_state = 0, .external_lex_state = 1},
  [474] = {.lex_state = 0, .external_lex_state = 1},
  [475] = {.lex_state = 0, .external_lex_state = 1},
  [476] = {.lex_state = 0, .external_lex_state = 1},
  [477] = {.lex_state = 0, .external_lex_state = 1},
  [478] = {.lex_state = 0, .external_lex_state = 1},
  [479] = {.lex_state = 0, .external_lex_state = 1},
  [480] = {.lex_state = 0, .external_lex_state = 1},
  [481] = {.lex_state = 89, .external_lex_state = 1},
  [482] = {.lex_state = 0, .external_lex_state = 1},
  [483] = {.lex_state = 89, .external_lex_state = 1},
  [484] = {.lex_state = 0, .external_lex_state = 1},
  [485] = {.lex_state = 0, .external_lex_state = 1},
  [486] = {.lex_state = 0, .external_lex_state = 1},
  [487] = {.lex_state = 89, .external_lex_state = 1},
  [488] = {.lex_state = 0, .external_lex_state = 1},
  [489] = {.lex_state = 89, .external_lex_state = 1},
  [490] = {.lex_state = 89, .external_lex_state = 1},
  [491] = {.lex_state = 89, .external_lex_state = 1},
  [492] = {(TSStateId)(-1)},
};

enum {
  ts_external_token__block_comment = 0,
};

static const TSSymbol ts_external_scanner_symbol_map[EXTERNAL_TOKEN_COUNT] = {
  [ts_external_token__block_comment] = sym__block_comment,
};

static const bool ts_external_scanner_states[2][EXTERNAL_TOKEN_COUNT] = {
  [1] = {
    [ts_external_token__block_comment] = true,
  },
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [sym__comment] = STATE(0),
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_ident_pattern_token] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_true] = ACTIONS(1),
    [anon_sym_false] = ACTIONS(1),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(1),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(1),
    [sym_hex_int_literal] = ACTIONS(1),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(1),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(1),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(1),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(1),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(1),
    [aux_sym_hex_float_literal_token1] = ACTIONS(1),
    [aux_sym_hex_float_literal_token2] = ACTIONS(1),
    [aux_sym_hex_float_literal_token3] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [anon_sym_align] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_binding] = ACTIONS(1),
    [anon_sym_builtin] = ACTIONS(1),
    [anon_sym_const] = ACTIONS(1),
    [anon_sym_group] = ACTIONS(1),
    [anon_sym_id] = ACTIONS(1),
    [anon_sym_interpolate] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_invariant] = ACTIONS(1),
    [anon_sym_location] = ACTIONS(1),
    [anon_sym_size] = ACTIONS(1),
    [anon_sym_workgroup_size] = ACTIONS(1),
    [anon_sym_vertex] = ACTIONS(1),
    [anon_sym_fragment] = ACTIONS(1),
    [anon_sym_compute] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_array] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_struct] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_sampler] = ACTIONS(1),
    [anon_sym_sampler_comparison] = ACTIONS(1),
    [anon_sym_texture_1d] = ACTIONS(1),
    [anon_sym_texture_2d] = ACTIONS(1),
    [anon_sym_texture_2d_array] = ACTIONS(1),
    [anon_sym_texture_3d] = ACTIONS(1),
    [anon_sym_texture_cube] = ACTIONS(1),
    [anon_sym_texture_cube_array] = ACTIONS(1),
    [sym_multisampled_texture_type] = ACTIONS(1),
    [anon_sym_texture_storage_1d] = ACTIONS(1),
    [anon_sym_texture_storage_2d] = ACTIONS(1),
    [anon_sym_texture_storage_2d_array] = ACTIONS(1),
    [anon_sym_texture_storage_3d] = ACTIONS(1),
    [anon_sym_texture_depth_2d] = ACTIONS(1),
    [anon_sym_texture_depth_2d_array] = ACTIONS(1),
    [anon_sym_texture_depth_cube] = ACTIONS(1),
    [anon_sym_texture_depth_cube_array] = ACTIONS(1),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(1),
    [anon_sym_alias] = ACTIONS(1),
    [anon_sym_EQ] = ACTIONS(1),
    [anon_sym_bool] = ACTIONS(1),
    [anon_sym_f32] = ACTIONS(1),
    [anon_sym_f16] = ACTIONS(1),
    [anon_sym_i32] = ACTIONS(1),
    [anon_sym_u32] = ACTIONS(1),
    [anon_sym_ptr] = ACTIONS(1),
    [anon_sym_atomic] = ACTIONS(1),
    [anon_sym_vec2] = ACTIONS(1),
    [anon_sym_vec3] = ACTIONS(1),
    [anon_sym_vec4] = ACTIONS(1),
    [anon_sym_mat2x2] = ACTIONS(1),
    [anon_sym_mat2x3] = ACTIONS(1),
    [anon_sym_mat2x4] = ACTIONS(1),
    [anon_sym_mat3x2] = ACTIONS(1),
    [anon_sym_mat3x3] = ACTIONS(1),
    [anon_sym_mat3x4] = ACTIONS(1),
    [anon_sym_mat4x2] = ACTIONS(1),
    [anon_sym_mat4x3] = ACTIONS(1),
    [anon_sym_mat4x4] = ACTIONS(1),
    [anon_sym_let] = ACTIONS(1),
    [anon_sym_var] = ACTIONS(1),
    [anon_sym_override] = ACTIONS(1),
    [anon_sym_bitcast] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_BANG] = ACTIONS(1),
    [anon_sym_TILDE] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_AMP] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_LT_LT] = ACTIONS(1),
    [anon_sym_GT_GT] = ACTIONS(1),
    [anon_sym_LT_EQ] = ACTIONS(1),
    [anon_sym_GT_EQ] = ACTIONS(1),
    [anon_sym_EQ_EQ] = ACTIONS(1),
    [anon_sym_BANG_EQ] = ACTIONS(1),
    [anon_sym_AMP_AMP] = ACTIONS(1),
    [anon_sym_PIPE_PIPE] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_CARET] = ACTIONS(1),
    [anon_sym__] = ACTIONS(1),
    [anon_sym_PLUS_EQ] = ACTIONS(1),
    [anon_sym_DASH_EQ] = ACTIONS(1),
    [anon_sym_STAR_EQ] = ACTIONS(1),
    [anon_sym_SLASH_EQ] = ACTIONS(1),
    [anon_sym_PERCENT_EQ] = ACTIONS(1),
    [anon_sym_AMP_EQ] = ACTIONS(1),
    [anon_sym_PIPE_EQ] = ACTIONS(1),
    [anon_sym_CARET_EQ] = ACTIONS(1),
    [anon_sym_LT_LT_EQ] = ACTIONS(1),
    [anon_sym_PLUS_PLUS] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_switch] = ACTIONS(1),
    [anon_sym_case] = ACTIONS(1),
    [anon_sym_default] = ACTIONS(1),
    [anon_sym_loop] = ACTIONS(1),
    [anon_sym_for] = ACTIONS(1),
    [anon_sym_while] = ACTIONS(1),
    [anon_sym_break] = ACTIONS(1),
    [sym_continue_statement] = ACTIONS(1),
    [anon_sym_continuing] = ACTIONS(1),
    [anon_sym_return] = ACTIONS(1),
    [anon_sym_const_assert] = ACTIONS(1),
    [anon_sym_discard] = ACTIONS(1),
    [anon_sym_fn] = ACTIONS(1),
    [anon_sym_DASH_GT] = ACTIONS(1),
    [anon_sym_enable] = ACTIONS(1),
    [anon_sym_perspective] = ACTIONS(1),
    [anon_sym_linear] = ACTIONS(1),
    [anon_sym_flat] = ACTIONS(1),
    [anon_sym_center] = ACTIONS(1),
    [anon_sym_centroid] = ACTIONS(1),
    [anon_sym_sample] = ACTIONS(1),
    [anon_sym_vertex_index] = ACTIONS(1),
    [anon_sym_instance_index] = ACTIONS(1),
    [anon_sym_position] = ACTIONS(1),
    [anon_sym_front_facing] = ACTIONS(1),
    [anon_sym_frag_depth] = ACTIONS(1),
    [anon_sym_local_invocation_id] = ACTIONS(1),
    [anon_sym_local_invocation_index] = ACTIONS(1),
    [anon_sym_global_invocation_id] = ACTIONS(1),
    [anon_sym_workgroup_id] = ACTIONS(1),
    [anon_sym_num_workgroups] = ACTIONS(1),
    [anon_sym_sample_index] = ACTIONS(1),
    [anon_sym_sample_mask] = ACTIONS(1),
    [anon_sym_read] = ACTIONS(1),
    [anon_sym_write] = ACTIONS(1),
    [anon_sym_read_write] = ACTIONS(1),
    [anon_sym_function] = ACTIONS(1),
    [anon_sym_private] = ACTIONS(1),
    [anon_sym_workgroup] = ACTIONS(1),
    [anon_sym_uniform] = ACTIONS(1),
    [anon_sym_storage] = ACTIONS(1),
    [anon_sym_rgba8unorm] = ACTIONS(1),
    [anon_sym_rgba8snorm] = ACTIONS(1),
    [anon_sym_rgba8uint] = ACTIONS(1),
    [anon_sym_rgba8sint] = ACTIONS(1),
    [anon_sym_rgba16uint] = ACTIONS(1),
    [anon_sym_rgba16sint] = ACTIONS(1),
    [anon_sym_rgba16float] = ACTIONS(1),
    [anon_sym_r32uint] = ACTIONS(1),
    [anon_sym_r32sint] = ACTIONS(1),
    [anon_sym_r32float] = ACTIONS(1),
    [anon_sym_rg32uint] = ACTIONS(1),
    [anon_sym_rg32sint] = ACTIONS(1),
    [anon_sym_rg32float] = ACTIONS(1),
    [anon_sym_rgba32uint] = ACTIONS(1),
    [anon_sym_rgba32sint] = ACTIONS(1),
    [anon_sym_rgba32float] = ACTIONS(1),
    [anon_sym_bgra8unorm] = ACTIONS(1),
    [anon_sym_SLASH_LBRACKrgba_RBRACK_SLASH] = ACTIONS(1),
    [anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH] = ACTIONS(1),
    [anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH] = ACTIONS(1),
    [anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH] = ACTIONS(1),
    [anon_sym_SLASH_LBRACKxyzw_RBRACK_SLASH] = ACTIONS(1),
    [anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH] = ACTIONS(1),
    [anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH] = ACTIONS(1),
    [anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH] = ACTIONS(1),
    [anon_sym_CompileShader] = ACTIONS(1),
    [anon_sym_ComputeShader] = ACTIONS(1),
    [anon_sym_DomainShader] = ACTIONS(1),
    [anon_sym_GeometryShader] = ACTIONS(1),
    [anon_sym_Hullshader] = ACTIONS(1),
    [anon_sym_NULL] = ACTIONS(1),
    [anon_sym_Self] = ACTIONS(1),
    [anon_sym_abstract] = ACTIONS(1),
    [anon_sym_active] = ACTIONS(1),
    [anon_sym_alignas] = ACTIONS(1),
    [anon_sym_alignof] = ACTIONS(1),
    [anon_sym_as] = ACTIONS(1),
    [anon_sym_asm] = ACTIONS(1),
    [anon_sym_asm_fragment] = ACTIONS(1),
    [anon_sym_async] = ACTIONS(1),
    [anon_sym_attribute] = ACTIONS(1),
    [anon_sym_auto] = ACTIONS(1),
    [anon_sym_await] = ACTIONS(1),
    [anon_sym_become] = ACTIONS(1),
    [anon_sym_binding_array] = ACTIONS(1),
    [anon_sym_cast] = ACTIONS(1),
    [anon_sym_catch] = ACTIONS(1),
    [anon_sym_class] = ACTIONS(1),
    [anon_sym_co_await] = ACTIONS(1),
    [anon_sym_co_return] = ACTIONS(1),
    [anon_sym_co_yield] = ACTIONS(1),
    [anon_sym_coherent] = ACTIONS(1),
    [anon_sym_column_major] = ACTIONS(1),
    [anon_sym_common] = ACTIONS(1),
    [anon_sym_compile] = ACTIONS(1),
    [anon_sym_compile_fragment] = ACTIONS(1),
    [anon_sym_concept] = ACTIONS(1),
    [anon_sym_const_cast] = ACTIONS(1),
    [anon_sym_consteval] = ACTIONS(1),
    [anon_sym_constexpr] = ACTIONS(1),
    [anon_sym_constinit] = ACTIONS(1),
    [anon_sym_crate] = ACTIONS(1),
    [anon_sym_debugger] = ACTIONS(1),
    [anon_sym_decltype] = ACTIONS(1),
    [anon_sym_delete] = ACTIONS(1),
    [anon_sym_demote] = ACTIONS(1),
    [anon_sym_demote_to_helper] = ACTIONS(1),
    [anon_sym_do] = ACTIONS(1),
    [anon_sym_dynamic_cast] = ACTIONS(1),
    [anon_sym_enum] = ACTIONS(1),
    [anon_sym_explicit] = ACTIONS(1),
    [anon_sym_export] = ACTIONS(1),
    [anon_sym_extends] = ACTIONS(1),
    [anon_sym_extern] = ACTIONS(1),
    [anon_sym_external] = ACTIONS(1),
    [anon_sym_fallthrough] = ACTIONS(1),
    [anon_sym_filter] = ACTIONS(1),
    [anon_sym_final] = ACTIONS(1),
    [anon_sym_finally] = ACTIONS(1),
    [anon_sym_friend] = ACTIONS(1),
    [anon_sym_from] = ACTIONS(1),
    [anon_sym_fxgroup] = ACTIONS(1),
    [anon_sym_get] = ACTIONS(1),
    [anon_sym_goto] = ACTIONS(1),
    [anon_sym_groupshared] = ACTIONS(1),
    [anon_sym_handle] = ACTIONS(1),
    [anon_sym_highp] = ACTIONS(1),
    [anon_sym_impl] = ACTIONS(1),
    [anon_sym_implements] = ACTIONS(1),
    [anon_sym_import] = ACTIONS(1),
    [anon_sym_inline] = ACTIONS(1),
    [anon_sym_inout] = ACTIONS(1),
    [anon_sym_instanceof] = ACTIONS(1),
    [anon_sym_interface] = ACTIONS(1),
    [anon_sym_layout] = ACTIONS(1),
    [anon_sym_lowp] = ACTIONS(1),
    [anon_sym_macro] = ACTIONS(1),
    [anon_sym_macro_rules] = ACTIONS(1),
    [anon_sym_match] = ACTIONS(1),
    [anon_sym_mediump] = ACTIONS(1),
    [anon_sym_meta] = ACTIONS(1),
    [anon_sym_mod] = ACTIONS(1),
    [anon_sym_module] = ACTIONS(1),
    [anon_sym_move] = ACTIONS(1),
    [anon_sym_mut] = ACTIONS(1),
    [anon_sym_mutable] = ACTIONS(1),
    [anon_sym_namespace] = ACTIONS(1),
    [anon_sym_new] = ACTIONS(1),
    [anon_sym_nil] = ACTIONS(1),
    [anon_sym_noexcept] = ACTIONS(1),
    [anon_sym_noinline] = ACTIONS(1),
    [anon_sym_nointerpolation] = ACTIONS(1),
    [anon_sym_noperspective] = ACTIONS(1),
    [anon_sym_null] = ACTIONS(1),
    [anon_sym_nullptr] = ACTIONS(1),
    [anon_sym_of] = ACTIONS(1),
    [anon_sym_operator] = ACTIONS(1),
    [anon_sym_package] = ACTIONS(1),
    [anon_sym_packoffset] = ACTIONS(1),
    [anon_sym_partition] = ACTIONS(1),
    [anon_sym_pass] = ACTIONS(1),
    [anon_sym_patch] = ACTIONS(1),
    [anon_sym_pixelfragment] = ACTIONS(1),
    [anon_sym_precise] = ACTIONS(1),
    [anon_sym_precision] = ACTIONS(1),
    [anon_sym_premerge] = ACTIONS(1),
    [anon_sym_priv] = ACTIONS(1),
    [anon_sym_protected] = ACTIONS(1),
    [anon_sym_pub] = ACTIONS(1),
    [anon_sym_public] = ACTIONS(1),
    [anon_sym_readonly] = ACTIONS(1),
    [anon_sym_ref] = ACTIONS(1),
    [anon_sym_regardless] = ACTIONS(1),
    [anon_sym_register] = ACTIONS(1),
    [anon_sym_reinterpret_cast] = ACTIONS(1),
    [anon_sym_requires] = ACTIONS(1),
    [anon_sym_resource] = ACTIONS(1),
    [anon_sym_restrict] = ACTIONS(1),
    [anon_sym_self] = ACTIONS(1),
    [anon_sym_set] = ACTIONS(1),
    [anon_sym_shared] = ACTIONS(1),
    [anon_sym_signed] = ACTIONS(1),
    [anon_sym_sizeof] = ACTIONS(1),
    [anon_sym_smooth] = ACTIONS(1),
    [anon_sym_snorm] = ACTIONS(1),
    [anon_sym_static] = ACTIONS(1),
    [anon_sym_static_assert] = ACTIONS(1),
    [anon_sym_static_cast] = ACTIONS(1),
    [anon_sym_std] = ACTIONS(1),
    [anon_sym_subroutine] = ACTIONS(1),
    [anon_sym_super] = ACTIONS(1),
    [anon_sym_target] = ACTIONS(1),
    [anon_sym_template] = ACTIONS(1),
    [anon_sym_this] = ACTIONS(1),
    [anon_sym_thread_local] = ACTIONS(1),
    [anon_sym_throw] = ACTIONS(1),
    [anon_sym_trait] = ACTIONS(1),
    [anon_sym_try] = ACTIONS(1),
    [anon_sym_type] = ACTIONS(1),
    [anon_sym_typedef] = ACTIONS(1),
    [anon_sym_typeid] = ACTIONS(1),
    [anon_sym_typename] = ACTIONS(1),
    [anon_sym_typeof] = ACTIONS(1),
    [anon_sym_union] = ACTIONS(1),
    [anon_sym_unless] = ACTIONS(1),
    [anon_sym_unorm] = ACTIONS(1),
    [anon_sym_unsafe] = ACTIONS(1),
    [anon_sym_unsized] = ACTIONS(1),
    [anon_sym_use] = ACTIONS(1),
    [anon_sym_using] = ACTIONS(1),
    [anon_sym_varying] = ACTIONS(1),
    [anon_sym_virtual] = ACTIONS(1),
    [anon_sym_volatile] = ACTIONS(1),
    [anon_sym_wgsl] = ACTIONS(1),
    [anon_sym_where] = ACTIONS(1),
    [anon_sym_with] = ACTIONS(1),
    [anon_sym_writeonly] = ACTIONS(1),
    [anon_sym_yield] = ACTIONS(1),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [1] = {
    [sym_translation_unit] = STATE(480),
    [sym_global_directive] = STATE(232),
    [sym_attribute] = STATE(117),
    [sym_struct_decl] = STATE(241),
    [sym_type_alias_decl] = STATE(478),
    [sym_variable_decl] = STATE(358),
    [sym_global_variable_decl] = STATE(478),
    [sym_global_constant_decl] = STATE(478),
    [sym_const_assert_statement] = STATE(478),
    [sym_function_decl] = STATE(241),
    [sym_function_header] = STATE(347),
    [sym_enable_directive] = STATE(227),
    [sym__comment] = STATE(1),
    [aux_sym_translation_unit_repeat1] = STATE(141),
    [aux_sym_translation_unit_repeat2] = STATE(166),
    [aux_sym_struct_member_repeat1] = STATE(262),
    [ts_builtin_sym_end] = ACTIONS(7),
    [anon_sym_SEMI] = ACTIONS(9),
    [anon_sym_AT] = ACTIONS(11),
    [anon_sym_const] = ACTIONS(13),
    [anon_sym_struct] = ACTIONS(15),
    [anon_sym_alias] = ACTIONS(17),
    [anon_sym_var] = ACTIONS(19),
    [anon_sym_override] = ACTIONS(21),
    [anon_sym_const_assert] = ACTIONS(23),
    [anon_sym_fn] = ACTIONS(25),
    [anon_sym_enable] = ACTIONS(27),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [2] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(303),
    [sym_case_selector] = STATE(315),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(2),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_LBRACE] = ACTIONS(49),
    [anon_sym_COLON] = ACTIONS(49),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_default] = ACTIONS(75),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [3] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(303),
    [sym_case_selector] = STATE(315),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(3),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_LBRACE] = ACTIONS(77),
    [anon_sym_COLON] = ACTIONS(77),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_default] = ACTIONS(75),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [4] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(303),
    [sym_case_selectors] = STATE(332),
    [sym_case_selector] = STATE(301),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(4),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_default] = ACTIONS(75),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [5] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_expression_comma_list] = STATE(476),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(329),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(5),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_RPAREN] = ACTIONS(79),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [6] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(303),
    [sym_case_selector] = STATE(315),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(6),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_default] = ACTIONS(75),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [7] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_expression_comma_list] = STATE(468),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(329),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(7),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_RPAREN] = ACTIONS(81),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [8] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_expression_comma_list] = STATE(436),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(329),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(8),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_RPAREN] = ACTIONS(83),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [9] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(381),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(9),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_RPAREN] = ACTIONS(85),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [10] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(439),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(10),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_SEMI] = ACTIONS(87),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [11] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(470),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(11),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_SEMI] = ACTIONS(89),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [12] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(326),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(12),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_RPAREN] = ACTIONS(91),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [13] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(381),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(13),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_RPAREN] = ACTIONS(93),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [14] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(413),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(14),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_SEMI] = ACTIONS(95),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [15] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(328),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(15),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_RPAREN] = ACTIONS(91),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [16] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(432),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(16),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [17] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(472),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(17),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [18] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(447),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(18),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [19] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(406),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(19),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [20] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(469),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(20),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [21] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(394),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(21),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [22] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(473),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(22),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [23] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(460),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(23),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [24] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(363),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(24),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [25] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(467),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(25),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [26] = {
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_variable_statement] = STATE(397),
    [sym_variable_decl] = STATE(351),
    [sym_call_phrase] = STATE(387),
    [sym_callable] = STATE(340),
    [sym_lhs_expression] = STATE(219),
    [sym_core_lhs_expression] = STATE(206),
    [sym_compound_statement] = STATE(87),
    [sym_assignment_statement] = STATE(386),
    [sym_increment_statement] = STATE(386),
    [sym_decrement_statement] = STATE(386),
    [sym_if_statement] = STATE(87),
    [sym_if_clause] = STATE(72),
    [sym_switch_statement] = STATE(87),
    [sym_loop_statement] = STATE(87),
    [sym_for_statement] = STATE(87),
    [sym_while_statement] = STATE(87),
    [sym_break_statement] = STATE(397),
    [sym_continuing_statement] = STATE(486),
    [sym_return_statement] = STATE(397),
    [sym_func_call_statement] = STATE(397),
    [sym_const_assert_statement] = STATE(397),
    [sym_statement] = STATE(85),
    [sym_variable_updating_statement] = STATE(397),
    [sym_ident] = STATE(209),
    [sym__comment] = STATE(26),
    [aux_sym_compound_statement_repeat1] = STATE(28),
    [sym_ident_pattern_token] = ACTIONS(97),
    [anon_sym_SEMI] = ACTIONS(99),
    [anon_sym_LPAREN] = ACTIONS(101),
    [anon_sym_const] = ACTIONS(103),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_LBRACE] = ACTIONS(105),
    [anon_sym_RBRACE] = ACTIONS(107),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_let] = ACTIONS(103),
    [anon_sym_var] = ACTIONS(109),
    [anon_sym_STAR] = ACTIONS(111),
    [anon_sym_AMP] = ACTIONS(111),
    [anon_sym__] = ACTIONS(113),
    [anon_sym_if] = ACTIONS(115),
    [anon_sym_switch] = ACTIONS(117),
    [anon_sym_loop] = ACTIONS(119),
    [anon_sym_for] = ACTIONS(121),
    [anon_sym_while] = ACTIONS(123),
    [anon_sym_break] = ACTIONS(125),
    [sym_continue_statement] = ACTIONS(127),
    [anon_sym_continuing] = ACTIONS(129),
    [anon_sym_return] = ACTIONS(131),
    [anon_sym_const_assert] = ACTIONS(133),
    [anon_sym_discard] = ACTIONS(127),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [27] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(424),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(27),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [28] = {
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_variable_statement] = STATE(397),
    [sym_variable_decl] = STATE(351),
    [sym_call_phrase] = STATE(387),
    [sym_callable] = STATE(340),
    [sym_lhs_expression] = STATE(219),
    [sym_core_lhs_expression] = STATE(206),
    [sym_compound_statement] = STATE(87),
    [sym_assignment_statement] = STATE(386),
    [sym_increment_statement] = STATE(386),
    [sym_decrement_statement] = STATE(386),
    [sym_if_statement] = STATE(87),
    [sym_if_clause] = STATE(72),
    [sym_switch_statement] = STATE(87),
    [sym_loop_statement] = STATE(87),
    [sym_for_statement] = STATE(87),
    [sym_while_statement] = STATE(87),
    [sym_break_statement] = STATE(397),
    [sym_continuing_statement] = STATE(442),
    [sym_return_statement] = STATE(397),
    [sym_func_call_statement] = STATE(397),
    [sym_const_assert_statement] = STATE(397),
    [sym_statement] = STATE(85),
    [sym_variable_updating_statement] = STATE(397),
    [sym_ident] = STATE(209),
    [sym__comment] = STATE(28),
    [aux_sym_compound_statement_repeat1] = STATE(41),
    [sym_ident_pattern_token] = ACTIONS(97),
    [anon_sym_SEMI] = ACTIONS(99),
    [anon_sym_LPAREN] = ACTIONS(101),
    [anon_sym_const] = ACTIONS(103),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_LBRACE] = ACTIONS(105),
    [anon_sym_RBRACE] = ACTIONS(135),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_let] = ACTIONS(103),
    [anon_sym_var] = ACTIONS(109),
    [anon_sym_STAR] = ACTIONS(111),
    [anon_sym_AMP] = ACTIONS(111),
    [anon_sym__] = ACTIONS(113),
    [anon_sym_if] = ACTIONS(115),
    [anon_sym_switch] = ACTIONS(117),
    [anon_sym_loop] = ACTIONS(119),
    [anon_sym_for] = ACTIONS(121),
    [anon_sym_while] = ACTIONS(123),
    [anon_sym_break] = ACTIONS(125),
    [sym_continue_statement] = ACTIONS(127),
    [anon_sym_continuing] = ACTIONS(129),
    [anon_sym_return] = ACTIONS(131),
    [anon_sym_const_assert] = ACTIONS(133),
    [anon_sym_discard] = ACTIONS(127),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [29] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(381),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(29),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [30] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(343),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(30),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [31] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(331),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(31),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [32] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(475),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(32),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [33] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(474),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(33),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [34] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(471),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(34),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [35] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(479),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(35),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [36] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(393),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(36),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [37] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(336),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(37),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [38] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(308),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(38),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [39] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(409),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(39),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [40] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(144),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(250),
    [sym_short_circuit_and_expression] = STATE(430),
    [sym_short_circuit_or_expression] = STATE(429),
    [sym_binary_or_expression] = STATE(428),
    [sym_binary_and_expression] = STATE(427),
    [sym_binary_xor_expression] = STATE(426),
    [sym_bitwise_expression] = STATE(288),
    [sym_expression] = STATE(364),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(40),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [41] = {
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_variable_statement] = STATE(397),
    [sym_variable_decl] = STATE(351),
    [sym_call_phrase] = STATE(387),
    [sym_callable] = STATE(340),
    [sym_lhs_expression] = STATE(219),
    [sym_core_lhs_expression] = STATE(206),
    [sym_compound_statement] = STATE(87),
    [sym_assignment_statement] = STATE(386),
    [sym_increment_statement] = STATE(386),
    [sym_decrement_statement] = STATE(386),
    [sym_if_statement] = STATE(87),
    [sym_if_clause] = STATE(72),
    [sym_switch_statement] = STATE(87),
    [sym_loop_statement] = STATE(87),
    [sym_for_statement] = STATE(87),
    [sym_while_statement] = STATE(87),
    [sym_break_statement] = STATE(397),
    [sym_return_statement] = STATE(397),
    [sym_func_call_statement] = STATE(397),
    [sym_const_assert_statement] = STATE(397),
    [sym_statement] = STATE(85),
    [sym_variable_updating_statement] = STATE(397),
    [sym_ident] = STATE(209),
    [sym__comment] = STATE(41),
    [aux_sym_compound_statement_repeat1] = STATE(41),
    [sym_ident_pattern_token] = ACTIONS(137),
    [anon_sym_SEMI] = ACTIONS(140),
    [anon_sym_LPAREN] = ACTIONS(143),
    [anon_sym_const] = ACTIONS(146),
    [anon_sym_array] = ACTIONS(149),
    [anon_sym_LBRACE] = ACTIONS(152),
    [anon_sym_RBRACE] = ACTIONS(155),
    [anon_sym_sampler] = ACTIONS(157),
    [anon_sym_sampler_comparison] = ACTIONS(157),
    [anon_sym_texture_1d] = ACTIONS(160),
    [anon_sym_texture_2d] = ACTIONS(160),
    [anon_sym_texture_2d_array] = ACTIONS(160),
    [anon_sym_texture_3d] = ACTIONS(160),
    [anon_sym_texture_cube] = ACTIONS(160),
    [anon_sym_texture_cube_array] = ACTIONS(160),
    [sym_multisampled_texture_type] = ACTIONS(163),
    [anon_sym_texture_storage_1d] = ACTIONS(166),
    [anon_sym_texture_storage_2d] = ACTIONS(166),
    [anon_sym_texture_storage_2d_array] = ACTIONS(166),
    [anon_sym_texture_storage_3d] = ACTIONS(166),
    [anon_sym_texture_depth_2d] = ACTIONS(169),
    [anon_sym_texture_depth_2d_array] = ACTIONS(169),
    [anon_sym_texture_depth_cube] = ACTIONS(169),
    [anon_sym_texture_depth_cube_array] = ACTIONS(169),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(169),
    [anon_sym_bool] = ACTIONS(172),
    [anon_sym_f32] = ACTIONS(172),
    [anon_sym_f16] = ACTIONS(172),
    [anon_sym_i32] = ACTIONS(172),
    [anon_sym_u32] = ACTIONS(172),
    [anon_sym_ptr] = ACTIONS(175),
    [anon_sym_atomic] = ACTIONS(178),
    [anon_sym_vec2] = ACTIONS(181),
    [anon_sym_vec3] = ACTIONS(181),
    [anon_sym_vec4] = ACTIONS(181),
    [anon_sym_mat2x2] = ACTIONS(184),
    [anon_sym_mat2x3] = ACTIONS(184),
    [anon_sym_mat2x4] = ACTIONS(184),
    [anon_sym_mat3x2] = ACTIONS(184),
    [anon_sym_mat3x3] = ACTIONS(184),
    [anon_sym_mat3x4] = ACTIONS(184),
    [anon_sym_mat4x2] = ACTIONS(184),
    [anon_sym_mat4x3] = ACTIONS(184),
    [anon_sym_mat4x4] = ACTIONS(184),
    [anon_sym_let] = ACTIONS(146),
    [anon_sym_var] = ACTIONS(187),
    [anon_sym_STAR] = ACTIONS(190),
    [anon_sym_AMP] = ACTIONS(190),
    [anon_sym__] = ACTIONS(193),
    [anon_sym_if] = ACTIONS(196),
    [anon_sym_switch] = ACTIONS(199),
    [anon_sym_loop] = ACTIONS(202),
    [anon_sym_for] = ACTIONS(205),
    [anon_sym_while] = ACTIONS(208),
    [anon_sym_break] = ACTIONS(211),
    [sym_continue_statement] = ACTIONS(214),
    [anon_sym_continuing] = ACTIONS(217),
    [anon_sym_return] = ACTIONS(219),
    [anon_sym_const_assert] = ACTIONS(222),
    [anon_sym_discard] = ACTIONS(214),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [42] = {
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_variable_statement] = STATE(397),
    [sym_variable_decl] = STATE(351),
    [sym_call_phrase] = STATE(387),
    [sym_callable] = STATE(340),
    [sym_lhs_expression] = STATE(219),
    [sym_core_lhs_expression] = STATE(206),
    [sym_compound_statement] = STATE(87),
    [sym_assignment_statement] = STATE(386),
    [sym_increment_statement] = STATE(386),
    [sym_decrement_statement] = STATE(386),
    [sym_if_statement] = STATE(87),
    [sym_if_clause] = STATE(72),
    [sym_switch_statement] = STATE(87),
    [sym_loop_statement] = STATE(87),
    [sym_for_statement] = STATE(87),
    [sym_while_statement] = STATE(87),
    [sym_break_statement] = STATE(397),
    [sym_break_if_statement] = STATE(404),
    [sym_return_statement] = STATE(397),
    [sym_func_call_statement] = STATE(397),
    [sym_const_assert_statement] = STATE(397),
    [sym_statement] = STATE(85),
    [sym_variable_updating_statement] = STATE(397),
    [sym_ident] = STATE(209),
    [sym__comment] = STATE(42),
    [aux_sym_compound_statement_repeat1] = STATE(41),
    [sym_ident_pattern_token] = ACTIONS(97),
    [anon_sym_SEMI] = ACTIONS(99),
    [anon_sym_LPAREN] = ACTIONS(101),
    [anon_sym_const] = ACTIONS(103),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_LBRACE] = ACTIONS(105),
    [anon_sym_RBRACE] = ACTIONS(225),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_let] = ACTIONS(103),
    [anon_sym_var] = ACTIONS(109),
    [anon_sym_STAR] = ACTIONS(111),
    [anon_sym_AMP] = ACTIONS(111),
    [anon_sym__] = ACTIONS(113),
    [anon_sym_if] = ACTIONS(115),
    [anon_sym_switch] = ACTIONS(117),
    [anon_sym_loop] = ACTIONS(119),
    [anon_sym_for] = ACTIONS(121),
    [anon_sym_while] = ACTIONS(123),
    [anon_sym_break] = ACTIONS(227),
    [sym_continue_statement] = ACTIONS(127),
    [anon_sym_return] = ACTIONS(131),
    [anon_sym_const_assert] = ACTIONS(133),
    [anon_sym_discard] = ACTIONS(127),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [43] = {
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_variable_statement] = STATE(397),
    [sym_variable_decl] = STATE(351),
    [sym_call_phrase] = STATE(387),
    [sym_callable] = STATE(340),
    [sym_lhs_expression] = STATE(219),
    [sym_core_lhs_expression] = STATE(206),
    [sym_compound_statement] = STATE(87),
    [sym_assignment_statement] = STATE(386),
    [sym_increment_statement] = STATE(386),
    [sym_decrement_statement] = STATE(386),
    [sym_if_statement] = STATE(87),
    [sym_if_clause] = STATE(72),
    [sym_switch_statement] = STATE(87),
    [sym_loop_statement] = STATE(87),
    [sym_for_statement] = STATE(87),
    [sym_while_statement] = STATE(87),
    [sym_break_statement] = STATE(397),
    [sym_break_if_statement] = STATE(420),
    [sym_return_statement] = STATE(397),
    [sym_func_call_statement] = STATE(397),
    [sym_const_assert_statement] = STATE(397),
    [sym_statement] = STATE(85),
    [sym_variable_updating_statement] = STATE(397),
    [sym_ident] = STATE(209),
    [sym__comment] = STATE(43),
    [aux_sym_compound_statement_repeat1] = STATE(42),
    [sym_ident_pattern_token] = ACTIONS(97),
    [anon_sym_SEMI] = ACTIONS(99),
    [anon_sym_LPAREN] = ACTIONS(101),
    [anon_sym_const] = ACTIONS(103),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_LBRACE] = ACTIONS(105),
    [anon_sym_RBRACE] = ACTIONS(229),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_let] = ACTIONS(103),
    [anon_sym_var] = ACTIONS(109),
    [anon_sym_STAR] = ACTIONS(111),
    [anon_sym_AMP] = ACTIONS(111),
    [anon_sym__] = ACTIONS(113),
    [anon_sym_if] = ACTIONS(115),
    [anon_sym_switch] = ACTIONS(117),
    [anon_sym_loop] = ACTIONS(119),
    [anon_sym_for] = ACTIONS(121),
    [anon_sym_while] = ACTIONS(123),
    [anon_sym_break] = ACTIONS(227),
    [sym_continue_statement] = ACTIONS(127),
    [anon_sym_return] = ACTIONS(131),
    [anon_sym_const_assert] = ACTIONS(133),
    [anon_sym_discard] = ACTIONS(127),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [44] = {
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_variable_statement] = STATE(397),
    [sym_variable_decl] = STATE(351),
    [sym_call_phrase] = STATE(387),
    [sym_callable] = STATE(340),
    [sym_lhs_expression] = STATE(219),
    [sym_core_lhs_expression] = STATE(206),
    [sym_compound_statement] = STATE(87),
    [sym_assignment_statement] = STATE(386),
    [sym_increment_statement] = STATE(386),
    [sym_decrement_statement] = STATE(386),
    [sym_if_statement] = STATE(87),
    [sym_if_clause] = STATE(72),
    [sym_switch_statement] = STATE(87),
    [sym_loop_statement] = STATE(87),
    [sym_for_statement] = STATE(87),
    [sym_while_statement] = STATE(87),
    [sym_break_statement] = STATE(397),
    [sym_return_statement] = STATE(397),
    [sym_func_call_statement] = STATE(397),
    [sym_const_assert_statement] = STATE(397),
    [sym_statement] = STATE(85),
    [sym_variable_updating_statement] = STATE(397),
    [sym_ident] = STATE(209),
    [sym__comment] = STATE(44),
    [aux_sym_compound_statement_repeat1] = STATE(45),
    [sym_ident_pattern_token] = ACTIONS(97),
    [anon_sym_SEMI] = ACTIONS(99),
    [anon_sym_LPAREN] = ACTIONS(101),
    [anon_sym_const] = ACTIONS(103),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_LBRACE] = ACTIONS(105),
    [anon_sym_RBRACE] = ACTIONS(231),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_let] = ACTIONS(103),
    [anon_sym_var] = ACTIONS(109),
    [anon_sym_STAR] = ACTIONS(111),
    [anon_sym_AMP] = ACTIONS(111),
    [anon_sym__] = ACTIONS(113),
    [anon_sym_if] = ACTIONS(115),
    [anon_sym_switch] = ACTIONS(117),
    [anon_sym_loop] = ACTIONS(119),
    [anon_sym_for] = ACTIONS(121),
    [anon_sym_while] = ACTIONS(123),
    [anon_sym_break] = ACTIONS(125),
    [sym_continue_statement] = ACTIONS(127),
    [anon_sym_return] = ACTIONS(131),
    [anon_sym_const_assert] = ACTIONS(133),
    [anon_sym_discard] = ACTIONS(127),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [45] = {
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_variable_statement] = STATE(397),
    [sym_variable_decl] = STATE(351),
    [sym_call_phrase] = STATE(387),
    [sym_callable] = STATE(340),
    [sym_lhs_expression] = STATE(219),
    [sym_core_lhs_expression] = STATE(206),
    [sym_compound_statement] = STATE(87),
    [sym_assignment_statement] = STATE(386),
    [sym_increment_statement] = STATE(386),
    [sym_decrement_statement] = STATE(386),
    [sym_if_statement] = STATE(87),
    [sym_if_clause] = STATE(72),
    [sym_switch_statement] = STATE(87),
    [sym_loop_statement] = STATE(87),
    [sym_for_statement] = STATE(87),
    [sym_while_statement] = STATE(87),
    [sym_break_statement] = STATE(397),
    [sym_return_statement] = STATE(397),
    [sym_func_call_statement] = STATE(397),
    [sym_const_assert_statement] = STATE(397),
    [sym_statement] = STATE(85),
    [sym_variable_updating_statement] = STATE(397),
    [sym_ident] = STATE(209),
    [sym__comment] = STATE(45),
    [aux_sym_compound_statement_repeat1] = STATE(41),
    [sym_ident_pattern_token] = ACTIONS(97),
    [anon_sym_SEMI] = ACTIONS(99),
    [anon_sym_LPAREN] = ACTIONS(101),
    [anon_sym_const] = ACTIONS(103),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_LBRACE] = ACTIONS(105),
    [anon_sym_RBRACE] = ACTIONS(233),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_let] = ACTIONS(103),
    [anon_sym_var] = ACTIONS(109),
    [anon_sym_STAR] = ACTIONS(111),
    [anon_sym_AMP] = ACTIONS(111),
    [anon_sym__] = ACTIONS(113),
    [anon_sym_if] = ACTIONS(115),
    [anon_sym_switch] = ACTIONS(117),
    [anon_sym_loop] = ACTIONS(119),
    [anon_sym_for] = ACTIONS(121),
    [anon_sym_while] = ACTIONS(123),
    [anon_sym_break] = ACTIONS(125),
    [sym_continue_statement] = ACTIONS(127),
    [anon_sym_return] = ACTIONS(131),
    [anon_sym_const_assert] = ACTIONS(133),
    [anon_sym_discard] = ACTIONS(127),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [46] = {
    [sym_bool_literal] = STATE(202),
    [sym_int_literal] = STATE(202),
    [sym_decimal_int_literal] = STATE(192),
    [sym_float_literal] = STATE(202),
    [sym_decimal_float_literal] = STATE(191),
    [sym_hex_float_literal] = STATE(191),
    [sym_literal] = STATE(198),
    [sym_array_type_specifier] = STATE(254),
    [sym_element_count_expression] = STATE(456),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(225),
    [sym_call_expression] = STATE(198),
    [sym_call_phrase] = STATE(197),
    [sym_callable] = STATE(341),
    [sym_paren_expression] = STATE(198),
    [sym_unary_expression] = STATE(248),
    [sym_singular_expression] = STATE(212),
    [sym_multiplicative_expression] = STATE(274),
    [sym_additive_expression] = STATE(302),
    [sym_binary_or_expression] = STATE(458),
    [sym_binary_and_expression] = STATE(481),
    [sym_binary_xor_expression] = STATE(459),
    [sym_bitwise_expression] = STATE(455),
    [sym_ident] = STATE(190),
    [sym__comment] = STATE(46),
    [sym_ident_pattern_token] = ACTIONS(235),
    [anon_sym_true] = ACTIONS(237),
    [anon_sym_false] = ACTIONS(237),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(239),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(239),
    [sym_hex_int_literal] = ACTIONS(241),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(245),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(243),
    [aux_sym_hex_float_literal_token1] = ACTIONS(247),
    [aux_sym_hex_float_literal_token2] = ACTIONS(249),
    [aux_sym_hex_float_literal_token3] = ACTIONS(247),
    [anon_sym_LPAREN] = ACTIONS(251),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(253),
    [anon_sym_DASH] = ACTIONS(255),
    [anon_sym_BANG] = ACTIONS(255),
    [anon_sym_TILDE] = ACTIONS(255),
    [anon_sym_STAR] = ACTIONS(255),
    [anon_sym_AMP] = ACTIONS(255),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [47] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(173),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(268),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(47),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [48] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(173),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(179),
    [sym_additive_expression] = STATE(205),
    [sym_shift_expression] = STATE(214),
    [sym_relational_expression] = STATE(267),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(48),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [49] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(213),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(221),
    [sym_additive_expression] = STATE(234),
    [sym_shift_expression] = STATE(263),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(49),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [50] = {
    [sym_bool_literal] = STATE(202),
    [sym_int_literal] = STATE(202),
    [sym_decimal_int_literal] = STATE(192),
    [sym_float_literal] = STATE(202),
    [sym_decimal_float_literal] = STATE(191),
    [sym_hex_float_literal] = STATE(191),
    [sym_literal] = STATE(198),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(225),
    [sym_call_expression] = STATE(198),
    [sym_call_phrase] = STATE(197),
    [sym_callable] = STATE(341),
    [sym_paren_expression] = STATE(198),
    [sym_unary_expression] = STATE(286),
    [sym_singular_expression] = STATE(212),
    [sym_multiplicative_expression] = STATE(275),
    [sym_ident] = STATE(190),
    [sym__comment] = STATE(50),
    [sym_ident_pattern_token] = ACTIONS(235),
    [anon_sym_true] = ACTIONS(237),
    [anon_sym_false] = ACTIONS(237),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(239),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(239),
    [sym_hex_int_literal] = ACTIONS(241),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(245),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(243),
    [aux_sym_hex_float_literal_token1] = ACTIONS(247),
    [aux_sym_hex_float_literal_token2] = ACTIONS(249),
    [aux_sym_hex_float_literal_token3] = ACTIONS(247),
    [anon_sym_LPAREN] = ACTIONS(251),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(253),
    [anon_sym_DASH] = ACTIONS(255),
    [anon_sym_BANG] = ACTIONS(255),
    [anon_sym_TILDE] = ACTIONS(255),
    [anon_sym_STAR] = ACTIONS(255),
    [anon_sym_AMP] = ACTIONS(255),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [51] = {
    [sym_bool_literal] = STATE(165),
    [sym_int_literal] = STATE(165),
    [sym_decimal_int_literal] = STATE(169),
    [sym_float_literal] = STATE(165),
    [sym_decimal_float_literal] = STATE(164),
    [sym_hex_float_literal] = STATE(164),
    [sym_literal] = STATE(158),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(157),
    [sym_call_expression] = STATE(158),
    [sym_call_phrase] = STATE(174),
    [sym_callable] = STATE(370),
    [sym_paren_expression] = STATE(158),
    [sym_unary_expression] = STATE(188),
    [sym_singular_expression] = STATE(187),
    [sym_multiplicative_expression] = STATE(180),
    [sym_ident] = STATE(154),
    [sym__comment] = STATE(51),
    [sym_ident_pattern_token] = ACTIONS(257),
    [anon_sym_true] = ACTIONS(259),
    [anon_sym_false] = ACTIONS(259),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(261),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(261),
    [sym_hex_int_literal] = ACTIONS(263),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(265),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(265),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(265),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(267),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(265),
    [aux_sym_hex_float_literal_token1] = ACTIONS(269),
    [aux_sym_hex_float_literal_token2] = ACTIONS(271),
    [aux_sym_hex_float_literal_token3] = ACTIONS(269),
    [anon_sym_LPAREN] = ACTIONS(273),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(275),
    [anon_sym_DASH] = ACTIONS(277),
    [anon_sym_BANG] = ACTIONS(277),
    [anon_sym_TILDE] = ACTIONS(277),
    [anon_sym_STAR] = ACTIONS(277),
    [anon_sym_AMP] = ACTIONS(277),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [52] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(188),
    [sym_singular_expression] = STATE(146),
    [sym_multiplicative_expression] = STATE(217),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(52),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [53] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(143),
    [sym_singular_expression] = STATE(146),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(53),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [54] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(256),
    [sym_singular_expression] = STATE(146),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(54),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [55] = {
    [sym_bool_literal] = STATE(202),
    [sym_int_literal] = STATE(202),
    [sym_decimal_int_literal] = STATE(192),
    [sym_float_literal] = STATE(202),
    [sym_decimal_float_literal] = STATE(191),
    [sym_hex_float_literal] = STATE(191),
    [sym_literal] = STATE(198),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(225),
    [sym_call_expression] = STATE(198),
    [sym_call_phrase] = STATE(197),
    [sym_callable] = STATE(341),
    [sym_paren_expression] = STATE(198),
    [sym_unary_expression] = STATE(285),
    [sym_singular_expression] = STATE(212),
    [sym_ident] = STATE(190),
    [sym__comment] = STATE(55),
    [sym_ident_pattern_token] = ACTIONS(235),
    [anon_sym_true] = ACTIONS(237),
    [anon_sym_false] = ACTIONS(237),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(239),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(239),
    [sym_hex_int_literal] = ACTIONS(241),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(245),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(243),
    [aux_sym_hex_float_literal_token1] = ACTIONS(247),
    [aux_sym_hex_float_literal_token2] = ACTIONS(249),
    [aux_sym_hex_float_literal_token3] = ACTIONS(247),
    [anon_sym_LPAREN] = ACTIONS(251),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(253),
    [anon_sym_DASH] = ACTIONS(255),
    [anon_sym_BANG] = ACTIONS(255),
    [anon_sym_TILDE] = ACTIONS(255),
    [anon_sym_STAR] = ACTIONS(255),
    [anon_sym_AMP] = ACTIONS(255),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [56] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(186),
    [sym_singular_expression] = STATE(146),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(56),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [57] = {
    [sym_bool_literal] = STATE(165),
    [sym_int_literal] = STATE(165),
    [sym_decimal_int_literal] = STATE(169),
    [sym_float_literal] = STATE(165),
    [sym_decimal_float_literal] = STATE(164),
    [sym_hex_float_literal] = STATE(164),
    [sym_literal] = STATE(158),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(157),
    [sym_call_expression] = STATE(158),
    [sym_call_phrase] = STATE(174),
    [sym_callable] = STATE(370),
    [sym_paren_expression] = STATE(158),
    [sym_unary_expression] = STATE(183),
    [sym_singular_expression] = STATE(187),
    [sym_ident] = STATE(154),
    [sym__comment] = STATE(57),
    [sym_ident_pattern_token] = ACTIONS(257),
    [anon_sym_true] = ACTIONS(259),
    [anon_sym_false] = ACTIONS(259),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(261),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(261),
    [sym_hex_int_literal] = ACTIONS(263),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(265),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(265),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(265),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(267),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(265),
    [aux_sym_hex_float_literal_token1] = ACTIONS(269),
    [aux_sym_hex_float_literal_token2] = ACTIONS(271),
    [aux_sym_hex_float_literal_token3] = ACTIONS(269),
    [anon_sym_LPAREN] = ACTIONS(273),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(275),
    [anon_sym_DASH] = ACTIONS(277),
    [anon_sym_BANG] = ACTIONS(277),
    [anon_sym_TILDE] = ACTIONS(277),
    [anon_sym_STAR] = ACTIONS(277),
    [anon_sym_AMP] = ACTIONS(277),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [58] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(220),
    [sym_singular_expression] = STATE(146),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(58),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [59] = {
    [sym_bool_literal] = STATE(202),
    [sym_int_literal] = STATE(202),
    [sym_decimal_int_literal] = STATE(192),
    [sym_float_literal] = STATE(202),
    [sym_decimal_float_literal] = STATE(191),
    [sym_hex_float_literal] = STATE(191),
    [sym_literal] = STATE(198),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(225),
    [sym_call_expression] = STATE(198),
    [sym_call_phrase] = STATE(197),
    [sym_callable] = STATE(341),
    [sym_paren_expression] = STATE(198),
    [sym_unary_expression] = STATE(253),
    [sym_singular_expression] = STATE(212),
    [sym_ident] = STATE(190),
    [sym__comment] = STATE(59),
    [sym_ident_pattern_token] = ACTIONS(235),
    [anon_sym_true] = ACTIONS(237),
    [anon_sym_false] = ACTIONS(237),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(239),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(239),
    [sym_hex_int_literal] = ACTIONS(241),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(245),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(243),
    [aux_sym_hex_float_literal_token1] = ACTIONS(247),
    [aux_sym_hex_float_literal_token2] = ACTIONS(249),
    [aux_sym_hex_float_literal_token3] = ACTIONS(247),
    [anon_sym_LPAREN] = ACTIONS(251),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(253),
    [anon_sym_DASH] = ACTIONS(255),
    [anon_sym_BANG] = ACTIONS(255),
    [anon_sym_TILDE] = ACTIONS(255),
    [anon_sym_STAR] = ACTIONS(255),
    [anon_sym_AMP] = ACTIONS(255),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [60] = {
    [sym_bool_literal] = STATE(202),
    [sym_int_literal] = STATE(202),
    [sym_decimal_int_literal] = STATE(192),
    [sym_float_literal] = STATE(202),
    [sym_decimal_float_literal] = STATE(191),
    [sym_hex_float_literal] = STATE(191),
    [sym_literal] = STATE(198),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(225),
    [sym_call_expression] = STATE(198),
    [sym_call_phrase] = STATE(197),
    [sym_callable] = STATE(341),
    [sym_paren_expression] = STATE(198),
    [sym_unary_expression] = STATE(256),
    [sym_singular_expression] = STATE(212),
    [sym_ident] = STATE(190),
    [sym__comment] = STATE(60),
    [sym_ident_pattern_token] = ACTIONS(235),
    [anon_sym_true] = ACTIONS(237),
    [anon_sym_false] = ACTIONS(237),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(239),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(239),
    [sym_hex_int_literal] = ACTIONS(241),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(245),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(243),
    [aux_sym_hex_float_literal_token1] = ACTIONS(247),
    [aux_sym_hex_float_literal_token2] = ACTIONS(249),
    [aux_sym_hex_float_literal_token3] = ACTIONS(247),
    [anon_sym_LPAREN] = ACTIONS(251),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(253),
    [anon_sym_DASH] = ACTIONS(255),
    [anon_sym_BANG] = ACTIONS(255),
    [anon_sym_TILDE] = ACTIONS(255),
    [anon_sym_STAR] = ACTIONS(255),
    [anon_sym_AMP] = ACTIONS(255),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [61] = {
    [sym_bool_literal] = STATE(202),
    [sym_int_literal] = STATE(202),
    [sym_decimal_int_literal] = STATE(192),
    [sym_float_literal] = STATE(202),
    [sym_decimal_float_literal] = STATE(191),
    [sym_hex_float_literal] = STATE(191),
    [sym_literal] = STATE(198),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(245),
    [sym_call_expression] = STATE(198),
    [sym_call_phrase] = STATE(197),
    [sym_callable] = STATE(341),
    [sym_paren_expression] = STATE(198),
    [sym_unary_expression] = STATE(253),
    [sym_singular_expression] = STATE(212),
    [sym_ident] = STATE(190),
    [sym__comment] = STATE(61),
    [sym_ident_pattern_token] = ACTIONS(97),
    [anon_sym_true] = ACTIONS(237),
    [anon_sym_false] = ACTIONS(237),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(239),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(239),
    [sym_hex_int_literal] = ACTIONS(241),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(245),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(243),
    [aux_sym_hex_float_literal_token1] = ACTIONS(247),
    [aux_sym_hex_float_literal_token2] = ACTIONS(249),
    [aux_sym_hex_float_literal_token3] = ACTIONS(247),
    [anon_sym_LPAREN] = ACTIONS(251),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(253),
    [anon_sym_DASH] = ACTIONS(279),
    [anon_sym_BANG] = ACTIONS(279),
    [anon_sym_TILDE] = ACTIONS(279),
    [anon_sym_STAR] = ACTIONS(279),
    [anon_sym_AMP] = ACTIONS(279),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [62] = {
    [sym_bool_literal] = STATE(135),
    [sym_int_literal] = STATE(135),
    [sym_decimal_int_literal] = STATE(138),
    [sym_float_literal] = STATE(135),
    [sym_decimal_float_literal] = STATE(133),
    [sym_hex_float_literal] = STATE(133),
    [sym_literal] = STATE(126),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(121),
    [sym_call_expression] = STATE(126),
    [sym_call_phrase] = STATE(131),
    [sym_callable] = STATE(340),
    [sym_paren_expression] = STATE(126),
    [sym_unary_expression] = STATE(257),
    [sym_singular_expression] = STATE(146),
    [sym_ident] = STATE(122),
    [sym__comment] = STATE(62),
    [sym_ident_pattern_token] = ACTIONS(29),
    [anon_sym_true] = ACTIONS(31),
    [anon_sym_false] = ACTIONS(31),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(33),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(33),
    [sym_hex_int_literal] = ACTIONS(35),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(37),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(39),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(37),
    [aux_sym_hex_float_literal_token1] = ACTIONS(41),
    [aux_sym_hex_float_literal_token2] = ACTIONS(43),
    [aux_sym_hex_float_literal_token3] = ACTIONS(41),
    [anon_sym_LPAREN] = ACTIONS(45),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(71),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_BANG] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [63] = {
    [sym_bool_literal] = STATE(202),
    [sym_int_literal] = STATE(202),
    [sym_decimal_int_literal] = STATE(192),
    [sym_float_literal] = STATE(202),
    [sym_decimal_float_literal] = STATE(191),
    [sym_hex_float_literal] = STATE(191),
    [sym_literal] = STATE(198),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(225),
    [sym_call_expression] = STATE(198),
    [sym_call_phrase] = STATE(197),
    [sym_callable] = STATE(341),
    [sym_paren_expression] = STATE(198),
    [sym_unary_expression] = STATE(257),
    [sym_singular_expression] = STATE(212),
    [sym_ident] = STATE(190),
    [sym__comment] = STATE(63),
    [sym_ident_pattern_token] = ACTIONS(235),
    [anon_sym_true] = ACTIONS(237),
    [anon_sym_false] = ACTIONS(237),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(239),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(239),
    [sym_hex_int_literal] = ACTIONS(241),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(245),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(243),
    [aux_sym_hex_float_literal_token1] = ACTIONS(247),
    [aux_sym_hex_float_literal_token2] = ACTIONS(249),
    [aux_sym_hex_float_literal_token3] = ACTIONS(247),
    [anon_sym_LPAREN] = ACTIONS(251),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(253),
    [anon_sym_DASH] = ACTIONS(255),
    [anon_sym_BANG] = ACTIONS(255),
    [anon_sym_TILDE] = ACTIONS(255),
    [anon_sym_STAR] = ACTIONS(255),
    [anon_sym_AMP] = ACTIONS(255),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [64] = {
    [sym_bool_literal] = STATE(202),
    [sym_int_literal] = STATE(202),
    [sym_decimal_int_literal] = STATE(192),
    [sym_float_literal] = STATE(202),
    [sym_decimal_float_literal] = STATE(191),
    [sym_hex_float_literal] = STATE(191),
    [sym_literal] = STATE(198),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(225),
    [sym_call_expression] = STATE(198),
    [sym_call_phrase] = STATE(197),
    [sym_callable] = STATE(341),
    [sym_paren_expression] = STATE(198),
    [sym_unary_expression] = STATE(211),
    [sym_singular_expression] = STATE(212),
    [sym_ident] = STATE(190),
    [sym__comment] = STATE(64),
    [sym_ident_pattern_token] = ACTIONS(235),
    [anon_sym_true] = ACTIONS(237),
    [anon_sym_false] = ACTIONS(237),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(239),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(239),
    [sym_hex_int_literal] = ACTIONS(241),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(245),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(243),
    [aux_sym_hex_float_literal_token1] = ACTIONS(247),
    [aux_sym_hex_float_literal_token2] = ACTIONS(249),
    [aux_sym_hex_float_literal_token3] = ACTIONS(247),
    [anon_sym_LPAREN] = ACTIONS(251),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(253),
    [anon_sym_DASH] = ACTIONS(255),
    [anon_sym_BANG] = ACTIONS(255),
    [anon_sym_TILDE] = ACTIONS(255),
    [anon_sym_STAR] = ACTIONS(255),
    [anon_sym_AMP] = ACTIONS(255),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [65] = {
    [sym_bool_literal] = STATE(202),
    [sym_int_literal] = STATE(202),
    [sym_decimal_int_literal] = STATE(192),
    [sym_float_literal] = STATE(202),
    [sym_decimal_float_literal] = STATE(191),
    [sym_hex_float_literal] = STATE(191),
    [sym_literal] = STATE(198),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(245),
    [sym_call_expression] = STATE(198),
    [sym_call_phrase] = STATE(197),
    [sym_callable] = STATE(341),
    [sym_paren_expression] = STATE(198),
    [sym_unary_expression] = STATE(211),
    [sym_singular_expression] = STATE(212),
    [sym_ident] = STATE(190),
    [sym__comment] = STATE(65),
    [sym_ident_pattern_token] = ACTIONS(97),
    [anon_sym_true] = ACTIONS(237),
    [anon_sym_false] = ACTIONS(237),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(239),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(239),
    [sym_hex_int_literal] = ACTIONS(241),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(243),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(245),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(243),
    [aux_sym_hex_float_literal_token1] = ACTIONS(247),
    [aux_sym_hex_float_literal_token2] = ACTIONS(249),
    [aux_sym_hex_float_literal_token3] = ACTIONS(247),
    [anon_sym_LPAREN] = ACTIONS(251),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(253),
    [anon_sym_DASH] = ACTIONS(279),
    [anon_sym_BANG] = ACTIONS(279),
    [anon_sym_TILDE] = ACTIONS(279),
    [anon_sym_STAR] = ACTIONS(279),
    [anon_sym_AMP] = ACTIONS(279),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [66] = {
    [sym_bool_literal] = STATE(165),
    [sym_int_literal] = STATE(165),
    [sym_decimal_int_literal] = STATE(169),
    [sym_float_literal] = STATE(165),
    [sym_decimal_float_literal] = STATE(164),
    [sym_hex_float_literal] = STATE(164),
    [sym_literal] = STATE(158),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(157),
    [sym_call_expression] = STATE(158),
    [sym_call_phrase] = STATE(174),
    [sym_callable] = STATE(370),
    [sym_paren_expression] = STATE(158),
    [sym_unary_expression] = STATE(220),
    [sym_singular_expression] = STATE(187),
    [sym_ident] = STATE(154),
    [sym__comment] = STATE(66),
    [sym_ident_pattern_token] = ACTIONS(257),
    [anon_sym_true] = ACTIONS(259),
    [anon_sym_false] = ACTIONS(259),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(261),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(261),
    [sym_hex_int_literal] = ACTIONS(263),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(265),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(265),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(265),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(267),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(265),
    [aux_sym_hex_float_literal_token1] = ACTIONS(269),
    [aux_sym_hex_float_literal_token2] = ACTIONS(271),
    [aux_sym_hex_float_literal_token3] = ACTIONS(269),
    [anon_sym_LPAREN] = ACTIONS(273),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(275),
    [anon_sym_DASH] = ACTIONS(277),
    [anon_sym_BANG] = ACTIONS(277),
    [anon_sym_TILDE] = ACTIONS(277),
    [anon_sym_STAR] = ACTIONS(277),
    [anon_sym_AMP] = ACTIONS(277),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [67] = {
    [sym_bool_literal] = STATE(165),
    [sym_int_literal] = STATE(165),
    [sym_decimal_int_literal] = STATE(169),
    [sym_float_literal] = STATE(165),
    [sym_decimal_float_literal] = STATE(164),
    [sym_hex_float_literal] = STATE(164),
    [sym_literal] = STATE(158),
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_primary_expression] = STATE(157),
    [sym_call_expression] = STATE(158),
    [sym_call_phrase] = STATE(174),
    [sym_callable] = STATE(370),
    [sym_paren_expression] = STATE(158),
    [sym_unary_expression] = STATE(186),
    [sym_singular_expression] = STATE(187),
    [sym_ident] = STATE(154),
    [sym__comment] = STATE(67),
    [sym_ident_pattern_token] = ACTIONS(257),
    [anon_sym_true] = ACTIONS(259),
    [anon_sym_false] = ACTIONS(259),
    [aux_sym_decimal_int_literal_token1] = ACTIONS(261),
    [aux_sym_decimal_int_literal_token2] = ACTIONS(261),
    [sym_hex_int_literal] = ACTIONS(263),
    [aux_sym_decimal_float_literal_token1] = ACTIONS(265),
    [aux_sym_decimal_float_literal_token2] = ACTIONS(265),
    [aux_sym_decimal_float_literal_token3] = ACTIONS(265),
    [aux_sym_decimal_float_literal_token4] = ACTIONS(267),
    [aux_sym_decimal_float_literal_token5] = ACTIONS(265),
    [aux_sym_hex_float_literal_token1] = ACTIONS(269),
    [aux_sym_hex_float_literal_token2] = ACTIONS(271),
    [aux_sym_hex_float_literal_token3] = ACTIONS(269),
    [anon_sym_LPAREN] = ACTIONS(273),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_bitcast] = ACTIONS(275),
    [anon_sym_DASH] = ACTIONS(277),
    [anon_sym_BANG] = ACTIONS(277),
    [anon_sym_TILDE] = ACTIONS(277),
    [anon_sym_STAR] = ACTIONS(277),
    [anon_sym_AMP] = ACTIONS(277),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [68] = {
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_variable_statement] = STATE(485),
    [sym_variable_decl] = STATE(351),
    [sym_call_phrase] = STATE(387),
    [sym_callable] = STATE(340),
    [sym_lhs_expression] = STATE(219),
    [sym_core_lhs_expression] = STATE(206),
    [sym_assignment_statement] = STATE(386),
    [sym_increment_statement] = STATE(386),
    [sym_decrement_statement] = STATE(386),
    [sym_for_header] = STATE(484),
    [sym_for_init] = STATE(482),
    [sym_func_call_statement] = STATE(485),
    [sym_variable_updating_statement] = STATE(485),
    [sym_ident] = STATE(209),
    [sym__comment] = STATE(68),
    [sym_ident_pattern_token] = ACTIONS(97),
    [anon_sym_SEMI] = ACTIONS(281),
    [anon_sym_LPAREN] = ACTIONS(101),
    [anon_sym_const] = ACTIONS(103),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_let] = ACTIONS(103),
    [anon_sym_var] = ACTIONS(109),
    [anon_sym_STAR] = ACTIONS(111),
    [anon_sym_AMP] = ACTIONS(111),
    [anon_sym__] = ACTIONS(113),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [69] = {
    [sym__comment] = STATE(69),
    [ts_builtin_sym_end] = ACTIONS(283),
    [sym_ident_pattern_token] = ACTIONS(285),
    [anon_sym_SEMI] = ACTIONS(283),
    [anon_sym_AT] = ACTIONS(283),
    [anon_sym_LPAREN] = ACTIONS(283),
    [anon_sym_const] = ACTIONS(285),
    [anon_sym_array] = ACTIONS(285),
    [anon_sym_struct] = ACTIONS(285),
    [anon_sym_LBRACE] = ACTIONS(283),
    [anon_sym_RBRACE] = ACTIONS(283),
    [anon_sym_sampler] = ACTIONS(285),
    [anon_sym_sampler_comparison] = ACTIONS(285),
    [anon_sym_texture_1d] = ACTIONS(285),
    [anon_sym_texture_2d] = ACTIONS(285),
    [anon_sym_texture_2d_array] = ACTIONS(285),
    [anon_sym_texture_3d] = ACTIONS(285),
    [anon_sym_texture_cube] = ACTIONS(285),
    [anon_sym_texture_cube_array] = ACTIONS(285),
    [sym_multisampled_texture_type] = ACTIONS(285),
    [anon_sym_texture_storage_1d] = ACTIONS(285),
    [anon_sym_texture_storage_2d] = ACTIONS(285),
    [anon_sym_texture_storage_2d_array] = ACTIONS(285),
    [anon_sym_texture_storage_3d] = ACTIONS(285),
    [anon_sym_texture_depth_2d] = ACTIONS(285),
    [anon_sym_texture_depth_2d_array] = ACTIONS(285),
    [anon_sym_texture_depth_cube] = ACTIONS(285),
    [anon_sym_texture_depth_cube_array] = ACTIONS(285),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(285),
    [anon_sym_alias] = ACTIONS(285),
    [anon_sym_bool] = ACTIONS(285),
    [anon_sym_f32] = ACTIONS(285),
    [anon_sym_f16] = ACTIONS(285),
    [anon_sym_i32] = ACTIONS(285),
    [anon_sym_u32] = ACTIONS(285),
    [anon_sym_ptr] = ACTIONS(285),
    [anon_sym_atomic] = ACTIONS(285),
    [anon_sym_vec2] = ACTIONS(285),
    [anon_sym_vec3] = ACTIONS(285),
    [anon_sym_vec4] = ACTIONS(285),
    [anon_sym_mat2x2] = ACTIONS(285),
    [anon_sym_mat2x3] = ACTIONS(285),
    [anon_sym_mat2x4] = ACTIONS(285),
    [anon_sym_mat3x2] = ACTIONS(285),
    [anon_sym_mat3x3] = ACTIONS(285),
    [anon_sym_mat3x4] = ACTIONS(285),
    [anon_sym_mat4x2] = ACTIONS(285),
    [anon_sym_mat4x3] = ACTIONS(285),
    [anon_sym_mat4x4] = ACTIONS(285),
    [anon_sym_let] = ACTIONS(285),
    [anon_sym_var] = ACTIONS(285),
    [anon_sym_override] = ACTIONS(285),
    [anon_sym_STAR] = ACTIONS(283),
    [anon_sym_AMP] = ACTIONS(283),
    [anon_sym__] = ACTIONS(285),
    [anon_sym_if] = ACTIONS(285),
    [anon_sym_else] = ACTIONS(285),
    [anon_sym_switch] = ACTIONS(285),
    [anon_sym_case] = ACTIONS(285),
    [anon_sym_default] = ACTIONS(285),
    [anon_sym_loop] = ACTIONS(285),
    [anon_sym_for] = ACTIONS(285),
    [anon_sym_while] = ACTIONS(285),
    [anon_sym_break] = ACTIONS(285),
    [sym_continue_statement] = ACTIONS(285),
    [anon_sym_continuing] = ACTIONS(285),
    [anon_sym_return] = ACTIONS(285),
    [anon_sym_const_assert] = ACTIONS(285),
    [anon_sym_discard] = ACTIONS(285),
    [anon_sym_fn] = ACTIONS(285),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [70] = {
    [sym__comment] = STATE(70),
    [ts_builtin_sym_end] = ACTIONS(287),
    [sym_ident_pattern_token] = ACTIONS(289),
    [anon_sym_SEMI] = ACTIONS(287),
    [anon_sym_AT] = ACTIONS(287),
    [anon_sym_LPAREN] = ACTIONS(287),
    [anon_sym_const] = ACTIONS(289),
    [anon_sym_array] = ACTIONS(289),
    [anon_sym_struct] = ACTIONS(289),
    [anon_sym_LBRACE] = ACTIONS(287),
    [anon_sym_RBRACE] = ACTIONS(287),
    [anon_sym_sampler] = ACTIONS(289),
    [anon_sym_sampler_comparison] = ACTIONS(289),
    [anon_sym_texture_1d] = ACTIONS(289),
    [anon_sym_texture_2d] = ACTIONS(289),
    [anon_sym_texture_2d_array] = ACTIONS(289),
    [anon_sym_texture_3d] = ACTIONS(289),
    [anon_sym_texture_cube] = ACTIONS(289),
    [anon_sym_texture_cube_array] = ACTIONS(289),
    [sym_multisampled_texture_type] = ACTIONS(289),
    [anon_sym_texture_storage_1d] = ACTIONS(289),
    [anon_sym_texture_storage_2d] = ACTIONS(289),
    [anon_sym_texture_storage_2d_array] = ACTIONS(289),
    [anon_sym_texture_storage_3d] = ACTIONS(289),
    [anon_sym_texture_depth_2d] = ACTIONS(289),
    [anon_sym_texture_depth_2d_array] = ACTIONS(289),
    [anon_sym_texture_depth_cube] = ACTIONS(289),
    [anon_sym_texture_depth_cube_array] = ACTIONS(289),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(289),
    [anon_sym_alias] = ACTIONS(289),
    [anon_sym_bool] = ACTIONS(289),
    [anon_sym_f32] = ACTIONS(289),
    [anon_sym_f16] = ACTIONS(289),
    [anon_sym_i32] = ACTIONS(289),
    [anon_sym_u32] = ACTIONS(289),
    [anon_sym_ptr] = ACTIONS(289),
    [anon_sym_atomic] = ACTIONS(289),
    [anon_sym_vec2] = ACTIONS(289),
    [anon_sym_vec3] = ACTIONS(289),
    [anon_sym_vec4] = ACTIONS(289),
    [anon_sym_mat2x2] = ACTIONS(289),
    [anon_sym_mat2x3] = ACTIONS(289),
    [anon_sym_mat2x4] = ACTIONS(289),
    [anon_sym_mat3x2] = ACTIONS(289),
    [anon_sym_mat3x3] = ACTIONS(289),
    [anon_sym_mat3x4] = ACTIONS(289),
    [anon_sym_mat4x2] = ACTIONS(289),
    [anon_sym_mat4x3] = ACTIONS(289),
    [anon_sym_mat4x4] = ACTIONS(289),
    [anon_sym_let] = ACTIONS(289),
    [anon_sym_var] = ACTIONS(289),
    [anon_sym_override] = ACTIONS(289),
    [anon_sym_STAR] = ACTIONS(287),
    [anon_sym_AMP] = ACTIONS(287),
    [anon_sym__] = ACTIONS(289),
    [anon_sym_if] = ACTIONS(289),
    [anon_sym_else] = ACTIONS(289),
    [anon_sym_switch] = ACTIONS(289),
    [anon_sym_case] = ACTIONS(289),
    [anon_sym_default] = ACTIONS(289),
    [anon_sym_loop] = ACTIONS(289),
    [anon_sym_for] = ACTIONS(289),
    [anon_sym_while] = ACTIONS(289),
    [anon_sym_break] = ACTIONS(289),
    [sym_continue_statement] = ACTIONS(289),
    [anon_sym_continuing] = ACTIONS(289),
    [anon_sym_return] = ACTIONS(289),
    [anon_sym_const_assert] = ACTIONS(289),
    [anon_sym_discard] = ACTIONS(289),
    [anon_sym_fn] = ACTIONS(289),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [71] = {
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_call_phrase] = STATE(387),
    [sym_callable] = STATE(340),
    [sym_lhs_expression] = STATE(219),
    [sym_core_lhs_expression] = STATE(206),
    [sym_assignment_statement] = STATE(386),
    [sym_increment_statement] = STATE(386),
    [sym_decrement_statement] = STATE(386),
    [sym_for_update] = STATE(417),
    [sym_func_call_statement] = STATE(416),
    [sym_variable_updating_statement] = STATE(416),
    [sym_ident] = STATE(209),
    [sym__comment] = STATE(71),
    [sym_ident_pattern_token] = ACTIONS(97),
    [anon_sym_LPAREN] = ACTIONS(101),
    [anon_sym_RPAREN] = ACTIONS(291),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_STAR] = ACTIONS(111),
    [anon_sym_AMP] = ACTIONS(111),
    [anon_sym__] = ACTIONS(113),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [72] = {
    [sym_else_if_clause] = STATE(78),
    [sym_else_clause] = STATE(81),
    [sym__comment] = STATE(72),
    [aux_sym_if_statement_repeat1] = STATE(74),
    [sym_ident_pattern_token] = ACTIONS(293),
    [anon_sym_SEMI] = ACTIONS(295),
    [anon_sym_LPAREN] = ACTIONS(295),
    [anon_sym_const] = ACTIONS(293),
    [anon_sym_array] = ACTIONS(293),
    [anon_sym_LBRACE] = ACTIONS(295),
    [anon_sym_RBRACE] = ACTIONS(295),
    [anon_sym_sampler] = ACTIONS(293),
    [anon_sym_sampler_comparison] = ACTIONS(293),
    [anon_sym_texture_1d] = ACTIONS(293),
    [anon_sym_texture_2d] = ACTIONS(293),
    [anon_sym_texture_2d_array] = ACTIONS(293),
    [anon_sym_texture_3d] = ACTIONS(293),
    [anon_sym_texture_cube] = ACTIONS(293),
    [anon_sym_texture_cube_array] = ACTIONS(293),
    [sym_multisampled_texture_type] = ACTIONS(293),
    [anon_sym_texture_storage_1d] = ACTIONS(293),
    [anon_sym_texture_storage_2d] = ACTIONS(293),
    [anon_sym_texture_storage_2d_array] = ACTIONS(293),
    [anon_sym_texture_storage_3d] = ACTIONS(293),
    [anon_sym_texture_depth_2d] = ACTIONS(293),
    [anon_sym_texture_depth_2d_array] = ACTIONS(293),
    [anon_sym_texture_depth_cube] = ACTIONS(293),
    [anon_sym_texture_depth_cube_array] = ACTIONS(293),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(293),
    [anon_sym_bool] = ACTIONS(293),
    [anon_sym_f32] = ACTIONS(293),
    [anon_sym_f16] = ACTIONS(293),
    [anon_sym_i32] = ACTIONS(293),
    [anon_sym_u32] = ACTIONS(293),
    [anon_sym_ptr] = ACTIONS(293),
    [anon_sym_atomic] = ACTIONS(293),
    [anon_sym_vec2] = ACTIONS(293),
    [anon_sym_vec3] = ACTIONS(293),
    [anon_sym_vec4] = ACTIONS(293),
    [anon_sym_mat2x2] = ACTIONS(293),
    [anon_sym_mat2x3] = ACTIONS(293),
    [anon_sym_mat2x4] = ACTIONS(293),
    [anon_sym_mat3x2] = ACTIONS(293),
    [anon_sym_mat3x3] = ACTIONS(293),
    [anon_sym_mat3x4] = ACTIONS(293),
    [anon_sym_mat4x2] = ACTIONS(293),
    [anon_sym_mat4x3] = ACTIONS(293),
    [anon_sym_mat4x4] = ACTIONS(293),
    [anon_sym_let] = ACTIONS(293),
    [anon_sym_var] = ACTIONS(293),
    [anon_sym_STAR] = ACTIONS(295),
    [anon_sym_AMP] = ACTIONS(295),
    [anon_sym__] = ACTIONS(293),
    [anon_sym_if] = ACTIONS(293),
    [anon_sym_else] = ACTIONS(297),
    [anon_sym_switch] = ACTIONS(293),
    [anon_sym_loop] = ACTIONS(293),
    [anon_sym_for] = ACTIONS(293),
    [anon_sym_while] = ACTIONS(293),
    [anon_sym_break] = ACTIONS(293),
    [sym_continue_statement] = ACTIONS(293),
    [anon_sym_continuing] = ACTIONS(293),
    [anon_sym_return] = ACTIONS(293),
    [anon_sym_const_assert] = ACTIONS(293),
    [anon_sym_discard] = ACTIONS(293),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [73] = {
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_call_phrase] = STATE(387),
    [sym_callable] = STATE(340),
    [sym_lhs_expression] = STATE(219),
    [sym_core_lhs_expression] = STATE(206),
    [sym_assignment_statement] = STATE(386),
    [sym_increment_statement] = STATE(386),
    [sym_decrement_statement] = STATE(386),
    [sym_for_update] = STATE(389),
    [sym_func_call_statement] = STATE(416),
    [sym_variable_updating_statement] = STATE(416),
    [sym_ident] = STATE(209),
    [sym__comment] = STATE(73),
    [sym_ident_pattern_token] = ACTIONS(97),
    [anon_sym_LPAREN] = ACTIONS(101),
    [anon_sym_RPAREN] = ACTIONS(299),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_STAR] = ACTIONS(111),
    [anon_sym_AMP] = ACTIONS(111),
    [anon_sym__] = ACTIONS(113),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [74] = {
    [sym_else_if_clause] = STATE(78),
    [sym_else_clause] = STATE(91),
    [sym__comment] = STATE(74),
    [aux_sym_if_statement_repeat1] = STATE(76),
    [sym_ident_pattern_token] = ACTIONS(301),
    [anon_sym_SEMI] = ACTIONS(303),
    [anon_sym_LPAREN] = ACTIONS(303),
    [anon_sym_const] = ACTIONS(301),
    [anon_sym_array] = ACTIONS(301),
    [anon_sym_LBRACE] = ACTIONS(303),
    [anon_sym_RBRACE] = ACTIONS(303),
    [anon_sym_sampler] = ACTIONS(301),
    [anon_sym_sampler_comparison] = ACTIONS(301),
    [anon_sym_texture_1d] = ACTIONS(301),
    [anon_sym_texture_2d] = ACTIONS(301),
    [anon_sym_texture_2d_array] = ACTIONS(301),
    [anon_sym_texture_3d] = ACTIONS(301),
    [anon_sym_texture_cube] = ACTIONS(301),
    [anon_sym_texture_cube_array] = ACTIONS(301),
    [sym_multisampled_texture_type] = ACTIONS(301),
    [anon_sym_texture_storage_1d] = ACTIONS(301),
    [anon_sym_texture_storage_2d] = ACTIONS(301),
    [anon_sym_texture_storage_2d_array] = ACTIONS(301),
    [anon_sym_texture_storage_3d] = ACTIONS(301),
    [anon_sym_texture_depth_2d] = ACTIONS(301),
    [anon_sym_texture_depth_2d_array] = ACTIONS(301),
    [anon_sym_texture_depth_cube] = ACTIONS(301),
    [anon_sym_texture_depth_cube_array] = ACTIONS(301),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(301),
    [anon_sym_bool] = ACTIONS(301),
    [anon_sym_f32] = ACTIONS(301),
    [anon_sym_f16] = ACTIONS(301),
    [anon_sym_i32] = ACTIONS(301),
    [anon_sym_u32] = ACTIONS(301),
    [anon_sym_ptr] = ACTIONS(301),
    [anon_sym_atomic] = ACTIONS(301),
    [anon_sym_vec2] = ACTIONS(301),
    [anon_sym_vec3] = ACTIONS(301),
    [anon_sym_vec4] = ACTIONS(301),
    [anon_sym_mat2x2] = ACTIONS(301),
    [anon_sym_mat2x3] = ACTIONS(301),
    [anon_sym_mat2x4] = ACTIONS(301),
    [anon_sym_mat3x2] = ACTIONS(301),
    [anon_sym_mat3x3] = ACTIONS(301),
    [anon_sym_mat3x4] = ACTIONS(301),
    [anon_sym_mat4x2] = ACTIONS(301),
    [anon_sym_mat4x3] = ACTIONS(301),
    [anon_sym_mat4x4] = ACTIONS(301),
    [anon_sym_let] = ACTIONS(301),
    [anon_sym_var] = ACTIONS(301),
    [anon_sym_STAR] = ACTIONS(303),
    [anon_sym_AMP] = ACTIONS(303),
    [anon_sym__] = ACTIONS(301),
    [anon_sym_if] = ACTIONS(301),
    [anon_sym_else] = ACTIONS(297),
    [anon_sym_switch] = ACTIONS(301),
    [anon_sym_loop] = ACTIONS(301),
    [anon_sym_for] = ACTIONS(301),
    [anon_sym_while] = ACTIONS(301),
    [anon_sym_break] = ACTIONS(301),
    [sym_continue_statement] = ACTIONS(301),
    [anon_sym_continuing] = ACTIONS(301),
    [anon_sym_return] = ACTIONS(301),
    [anon_sym_const_assert] = ACTIONS(301),
    [anon_sym_discard] = ACTIONS(301),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [75] = {
    [sym_array_type_specifier] = STATE(254),
    [sym_texture_and_sampler_types] = STATE(254),
    [sym_sampler_type] = STATE(260),
    [sym_sampled_texture_type] = STATE(445),
    [sym_storage_texture_type] = STATE(435),
    [sym_depth_texture_type] = STATE(260),
    [sym_type_specifier_without_ident] = STATE(434),
    [sym_vec_prefix] = STATE(339),
    [sym_mat_prefix] = STATE(339),
    [sym_call_phrase] = STATE(387),
    [sym_callable] = STATE(340),
    [sym_lhs_expression] = STATE(219),
    [sym_core_lhs_expression] = STATE(206),
    [sym_assignment_statement] = STATE(386),
    [sym_increment_statement] = STATE(386),
    [sym_decrement_statement] = STATE(386),
    [sym_for_update] = STATE(391),
    [sym_func_call_statement] = STATE(416),
    [sym_variable_updating_statement] = STATE(416),
    [sym_ident] = STATE(209),
    [sym__comment] = STATE(75),
    [sym_ident_pattern_token] = ACTIONS(97),
    [anon_sym_LPAREN] = ACTIONS(101),
    [anon_sym_RPAREN] = ACTIONS(305),
    [anon_sym_array] = ACTIONS(47),
    [anon_sym_sampler] = ACTIONS(51),
    [anon_sym_sampler_comparison] = ACTIONS(51),
    [anon_sym_texture_1d] = ACTIONS(53),
    [anon_sym_texture_2d] = ACTIONS(53),
    [anon_sym_texture_2d_array] = ACTIONS(53),
    [anon_sym_texture_3d] = ACTIONS(53),
    [anon_sym_texture_cube] = ACTIONS(53),
    [anon_sym_texture_cube_array] = ACTIONS(53),
    [sym_multisampled_texture_type] = ACTIONS(55),
    [anon_sym_texture_storage_1d] = ACTIONS(57),
    [anon_sym_texture_storage_2d] = ACTIONS(57),
    [anon_sym_texture_storage_2d_array] = ACTIONS(57),
    [anon_sym_texture_storage_3d] = ACTIONS(57),
    [anon_sym_texture_depth_2d] = ACTIONS(59),
    [anon_sym_texture_depth_2d_array] = ACTIONS(59),
    [anon_sym_texture_depth_cube] = ACTIONS(59),
    [anon_sym_texture_depth_cube_array] = ACTIONS(59),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(59),
    [anon_sym_bool] = ACTIONS(61),
    [anon_sym_f32] = ACTIONS(61),
    [anon_sym_f16] = ACTIONS(61),
    [anon_sym_i32] = ACTIONS(61),
    [anon_sym_u32] = ACTIONS(61),
    [anon_sym_ptr] = ACTIONS(63),
    [anon_sym_atomic] = ACTIONS(65),
    [anon_sym_vec2] = ACTIONS(67),
    [anon_sym_vec3] = ACTIONS(67),
    [anon_sym_vec4] = ACTIONS(67),
    [anon_sym_mat2x2] = ACTIONS(69),
    [anon_sym_mat2x3] = ACTIONS(69),
    [anon_sym_mat2x4] = ACTIONS(69),
    [anon_sym_mat3x2] = ACTIONS(69),
    [anon_sym_mat3x3] = ACTIONS(69),
    [anon_sym_mat3x4] = ACTIONS(69),
    [anon_sym_mat4x2] = ACTIONS(69),
    [anon_sym_mat4x3] = ACTIONS(69),
    [anon_sym_mat4x4] = ACTIONS(69),
    [anon_sym_STAR] = ACTIONS(111),
    [anon_sym_AMP] = ACTIONS(111),
    [anon_sym__] = ACTIONS(113),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [76] = {
    [sym_else_if_clause] = STATE(78),
    [sym__comment] = STATE(76),
    [aux_sym_if_statement_repeat1] = STATE(76),
    [sym_ident_pattern_token] = ACTIONS(307),
    [anon_sym_SEMI] = ACTIONS(309),
    [anon_sym_LPAREN] = ACTIONS(309),
    [anon_sym_const] = ACTIONS(307),
    [anon_sym_array] = ACTIONS(307),
    [anon_sym_LBRACE] = ACTIONS(309),
    [anon_sym_RBRACE] = ACTIONS(309),
    [anon_sym_sampler] = ACTIONS(307),
    [anon_sym_sampler_comparison] = ACTIONS(307),
    [anon_sym_texture_1d] = ACTIONS(307),
    [anon_sym_texture_2d] = ACTIONS(307),
    [anon_sym_texture_2d_array] = ACTIONS(307),
    [anon_sym_texture_3d] = ACTIONS(307),
    [anon_sym_texture_cube] = ACTIONS(307),
    [anon_sym_texture_cube_array] = ACTIONS(307),
    [sym_multisampled_texture_type] = ACTIONS(307),
    [anon_sym_texture_storage_1d] = ACTIONS(307),
    [anon_sym_texture_storage_2d] = ACTIONS(307),
    [anon_sym_texture_storage_2d_array] = ACTIONS(307),
    [anon_sym_texture_storage_3d] = ACTIONS(307),
    [anon_sym_texture_depth_2d] = ACTIONS(307),
    [anon_sym_texture_depth_2d_array] = ACTIONS(307),
    [anon_sym_texture_depth_cube] = ACTIONS(307),
    [anon_sym_texture_depth_cube_array] = ACTIONS(307),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(307),
    [anon_sym_bool] = ACTIONS(307),
    [anon_sym_f32] = ACTIONS(307),
    [anon_sym_f16] = ACTIONS(307),
    [anon_sym_i32] = ACTIONS(307),
    [anon_sym_u32] = ACTIONS(307),
    [anon_sym_ptr] = ACTIONS(307),
    [anon_sym_atomic] = ACTIONS(307),
    [anon_sym_vec2] = ACTIONS(307),
    [anon_sym_vec3] = ACTIONS(307),
    [anon_sym_vec4] = ACTIONS(307),
    [anon_sym_mat2x2] = ACTIONS(307),
    [anon_sym_mat2x3] = ACTIONS(307),
    [anon_sym_mat2x4] = ACTIONS(307),
    [anon_sym_mat3x2] = ACTIONS(307),
    [anon_sym_mat3x3] = ACTIONS(307),
    [anon_sym_mat3x4] = ACTIONS(307),
    [anon_sym_mat4x2] = ACTIONS(307),
    [anon_sym_mat4x3] = ACTIONS(307),
    [anon_sym_mat4x4] = ACTIONS(307),
    [anon_sym_let] = ACTIONS(307),
    [anon_sym_var] = ACTIONS(307),
    [anon_sym_STAR] = ACTIONS(309),
    [anon_sym_AMP] = ACTIONS(309),
    [anon_sym__] = ACTIONS(307),
    [anon_sym_if] = ACTIONS(307),
    [anon_sym_else] = ACTIONS(311),
    [anon_sym_switch] = ACTIONS(307),
    [anon_sym_loop] = ACTIONS(307),
    [anon_sym_for] = ACTIONS(307),
    [anon_sym_while] = ACTIONS(307),
    [anon_sym_break] = ACTIONS(307),
    [sym_continue_statement] = ACTIONS(307),
    [anon_sym_continuing] = ACTIONS(307),
    [anon_sym_return] = ACTIONS(307),
    [anon_sym_const_assert] = ACTIONS(307),
    [anon_sym_discard] = ACTIONS(307),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [77] = {
    [sym__comment] = STATE(77),
    [sym_ident_pattern_token] = ACTIONS(314),
    [anon_sym_SEMI] = ACTIONS(316),
    [anon_sym_LPAREN] = ACTIONS(316),
    [anon_sym_const] = ACTIONS(314),
    [anon_sym_array] = ACTIONS(314),
    [anon_sym_LBRACE] = ACTIONS(316),
    [anon_sym_RBRACE] = ACTIONS(316),
    [anon_sym_sampler] = ACTIONS(314),
    [anon_sym_sampler_comparison] = ACTIONS(314),
    [anon_sym_texture_1d] = ACTIONS(314),
    [anon_sym_texture_2d] = ACTIONS(314),
    [anon_sym_texture_2d_array] = ACTIONS(314),
    [anon_sym_texture_3d] = ACTIONS(314),
    [anon_sym_texture_cube] = ACTIONS(314),
    [anon_sym_texture_cube_array] = ACTIONS(314),
    [sym_multisampled_texture_type] = ACTIONS(314),
    [anon_sym_texture_storage_1d] = ACTIONS(314),
    [anon_sym_texture_storage_2d] = ACTIONS(314),
    [anon_sym_texture_storage_2d_array] = ACTIONS(314),
    [anon_sym_texture_storage_3d] = ACTIONS(314),
    [anon_sym_texture_depth_2d] = ACTIONS(314),
    [anon_sym_texture_depth_2d_array] = ACTIONS(314),
    [anon_sym_texture_depth_cube] = ACTIONS(314),
    [anon_sym_texture_depth_cube_array] = ACTIONS(314),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(314),
    [anon_sym_bool] = ACTIONS(314),
    [anon_sym_f32] = ACTIONS(314),
    [anon_sym_f16] = ACTIONS(314),
    [anon_sym_i32] = ACTIONS(314),
    [anon_sym_u32] = ACTIONS(314),
    [anon_sym_ptr] = ACTIONS(314),
    [anon_sym_atomic] = ACTIONS(314),
    [anon_sym_vec2] = ACTIONS(314),
    [anon_sym_vec3] = ACTIONS(314),
    [anon_sym_vec4] = ACTIONS(314),
    [anon_sym_mat2x2] = ACTIONS(314),
    [anon_sym_mat2x3] = ACTIONS(314),
    [anon_sym_mat2x4] = ACTIONS(314),
    [anon_sym_mat3x2] = ACTIONS(314),
    [anon_sym_mat3x3] = ACTIONS(314),
    [anon_sym_mat3x4] = ACTIONS(314),
    [anon_sym_mat4x2] = ACTIONS(314),
    [anon_sym_mat4x3] = ACTIONS(314),
    [anon_sym_mat4x4] = ACTIONS(314),
    [anon_sym_let] = ACTIONS(314),
    [anon_sym_var] = ACTIONS(314),
    [anon_sym_STAR] = ACTIONS(316),
    [anon_sym_AMP] = ACTIONS(316),
    [anon_sym__] = ACTIONS(314),
    [anon_sym_if] = ACTIONS(314),
    [anon_sym_else] = ACTIONS(314),
    [anon_sym_switch] = ACTIONS(314),
    [anon_sym_loop] = ACTIONS(314),
    [anon_sym_for] = ACTIONS(314),
    [anon_sym_while] = ACTIONS(314),
    [anon_sym_break] = ACTIONS(314),
    [sym_continue_statement] = ACTIONS(314),
    [anon_sym_continuing] = ACTIONS(314),
    [anon_sym_return] = ACTIONS(314),
    [anon_sym_const_assert] = ACTIONS(314),
    [anon_sym_discard] = ACTIONS(314),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [78] = {
    [sym__comment] = STATE(78),
    [sym_ident_pattern_token] = ACTIONS(318),
    [anon_sym_SEMI] = ACTIONS(320),
    [anon_sym_LPAREN] = ACTIONS(320),
    [anon_sym_const] = ACTIONS(318),
    [anon_sym_array] = ACTIONS(318),
    [anon_sym_LBRACE] = ACTIONS(320),
    [anon_sym_RBRACE] = ACTIONS(320),
    [anon_sym_sampler] = ACTIONS(318),
    [anon_sym_sampler_comparison] = ACTIONS(318),
    [anon_sym_texture_1d] = ACTIONS(318),
    [anon_sym_texture_2d] = ACTIONS(318),
    [anon_sym_texture_2d_array] = ACTIONS(318),
    [anon_sym_texture_3d] = ACTIONS(318),
    [anon_sym_texture_cube] = ACTIONS(318),
    [anon_sym_texture_cube_array] = ACTIONS(318),
    [sym_multisampled_texture_type] = ACTIONS(318),
    [anon_sym_texture_storage_1d] = ACTIONS(318),
    [anon_sym_texture_storage_2d] = ACTIONS(318),
    [anon_sym_texture_storage_2d_array] = ACTIONS(318),
    [anon_sym_texture_storage_3d] = ACTIONS(318),
    [anon_sym_texture_depth_2d] = ACTIONS(318),
    [anon_sym_texture_depth_2d_array] = ACTIONS(318),
    [anon_sym_texture_depth_cube] = ACTIONS(318),
    [anon_sym_texture_depth_cube_array] = ACTIONS(318),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(318),
    [anon_sym_bool] = ACTIONS(318),
    [anon_sym_f32] = ACTIONS(318),
    [anon_sym_f16] = ACTIONS(318),
    [anon_sym_i32] = ACTIONS(318),
    [anon_sym_u32] = ACTIONS(318),
    [anon_sym_ptr] = ACTIONS(318),
    [anon_sym_atomic] = ACTIONS(318),
    [anon_sym_vec2] = ACTIONS(318),
    [anon_sym_vec3] = ACTIONS(318),
    [anon_sym_vec4] = ACTIONS(318),
    [anon_sym_mat2x2] = ACTIONS(318),
    [anon_sym_mat2x3] = ACTIONS(318),
    [anon_sym_mat2x4] = ACTIONS(318),
    [anon_sym_mat3x2] = ACTIONS(318),
    [anon_sym_mat3x3] = ACTIONS(318),
    [anon_sym_mat3x4] = ACTIONS(318),
    [anon_sym_mat4x2] = ACTIONS(318),
    [anon_sym_mat4x3] = ACTIONS(318),
    [anon_sym_mat4x4] = ACTIONS(318),
    [anon_sym_let] = ACTIONS(318),
    [anon_sym_var] = ACTIONS(318),
    [anon_sym_STAR] = ACTIONS(320),
    [anon_sym_AMP] = ACTIONS(320),
    [anon_sym__] = ACTIONS(318),
    [anon_sym_if] = ACTIONS(318),
    [anon_sym_else] = ACTIONS(318),
    [anon_sym_switch] = ACTIONS(318),
    [anon_sym_loop] = ACTIONS(318),
    [anon_sym_for] = ACTIONS(318),
    [anon_sym_while] = ACTIONS(318),
    [anon_sym_break] = ACTIONS(318),
    [sym_continue_statement] = ACTIONS(318),
    [anon_sym_continuing] = ACTIONS(318),
    [anon_sym_return] = ACTIONS(318),
    [anon_sym_const_assert] = ACTIONS(318),
    [anon_sym_discard] = ACTIONS(318),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
  [79] = {
    [sym__comment] = STATE(79),
    [sym_ident_pattern_token] = ACTIONS(322),
    [anon_sym_SEMI] = ACTIONS(324),
    [anon_sym_LPAREN] = ACTIONS(324),
    [anon_sym_const] = ACTIONS(322),
    [anon_sym_array] = ACTIONS(322),
    [anon_sym_LBRACE] = ACTIONS(324),
    [anon_sym_RBRACE] = ACTIONS(324),
    [anon_sym_sampler] = ACTIONS(322),
    [anon_sym_sampler_comparison] = ACTIONS(322),
    [anon_sym_texture_1d] = ACTIONS(322),
    [anon_sym_texture_2d] = ACTIONS(322),
    [anon_sym_texture_2d_array] = ACTIONS(322),
    [anon_sym_texture_3d] = ACTIONS(322),
    [anon_sym_texture_cube] = ACTIONS(322),
    [anon_sym_texture_cube_array] = ACTIONS(322),
    [sym_multisampled_texture_type] = ACTIONS(322),
    [anon_sym_texture_storage_1d] = ACTIONS(322),
    [anon_sym_texture_storage_2d] = ACTIONS(322),
    [anon_sym_texture_storage_2d_array] = ACTIONS(322),
    [anon_sym_texture_storage_3d] = ACTIONS(322),
    [anon_sym_texture_depth_2d] = ACTIONS(322),
    [anon_sym_texture_depth_2d_array] = ACTIONS(322),
    [anon_sym_texture_depth_cube] = ACTIONS(322),
    [anon_sym_texture_depth_cube_array] = ACTIONS(322),
    [anon_sym_texture_depth_multisampled_2d] = ACTIONS(322),
    [anon_sym_bool] = ACTIONS(322),
    [anon_sym_f32] = ACTIONS(322),
    [anon_sym_f16] = ACTIONS(322),
    [anon_sym_i32] = ACTIONS(322),
    [anon_sym_u32] = ACTIONS(322),
    [anon_sym_ptr] = ACTIONS(322),
    [anon_sym_atomic] = ACTIONS(322),
    [anon_sym_vec2] = ACTIONS(322),
    [anon_sym_vec3] = ACTIONS(322),
    [anon_sym_vec4] = ACTIONS(322),
    [anon_sym_mat2x2] = ACTIONS(322),
    [anon_sym_mat2x3] = ACTIONS(322),
    [anon_sym_mat2x4] = ACTIONS(322),
    [anon_sym_mat3x2] = ACTIONS(322),
    [anon_sym_mat3x3] = ACTIONS(322),
    [anon_sym_mat3x4] = ACTIONS(322),
    [anon_sym_mat4x2] = ACTIONS(322),
    [anon_sym_mat4x3] = ACTIONS(322),
    [anon_sym_mat4x4] = ACTIONS(322),
    [anon_sym_let] = ACTIONS(322),
    [anon_sym_var] = ACTIONS(322),
    [anon_sym_STAR] = ACTIONS(324),
    [anon_sym_AMP] = ACTIONS(324),
    [anon_sym__] = ACTIONS(322),
    [anon_sym_if] = ACTIONS(322),
    [anon_sym_else] = ACTIONS(322),
    [anon_sym_switch] = ACTIONS(322),
    [anon_sym_loop] = ACTIONS(322),
    [anon_sym_for] = ACTIONS(322),
    [anon_sym_while] = ACTIONS(322),
    [anon_sym_break] = ACTIONS(322),
    [sym_continue_statement] = ACTIONS(322),
    [anon_sym_continuing] = ACTIONS(322),
    [anon_sym_return] = ACTIONS(322),
    [anon_sym_const_assert] = ACTIONS(322),
    [anon_sym_discard] = ACTIONS(322),
    [anon_sym_SLASH_SLASH] = ACTIONS(3),
    [sym__blankspace] = ACTIONS(5),
    [sym__block_comment] = ACTIONS(5),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(80), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(328), 6,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(326), 54,
      anon_sym_const,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_let,
      anon_sym_var,
      anon_sym__,
      anon_sym_if,
      anon_sym_switch,
      anon_sym_loop,
      anon_sym_for,
      anon_sym_while,
      anon_sym_break,
      sym_continue_statement,
      anon_sym_continuing,
      anon_sym_return,
      anon_sym_const_assert,
      anon_sym_discard,
      sym_ident_pattern_token,
  [75] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(81), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(303), 6,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(301), 54,
      anon_sym_const,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_let,
      anon_sym_var,
      anon_sym__,
      anon_sym_if,
      anon_sym_switch,
      anon_sym_loop,
      anon_sym_for,
      anon_sym_while,
      anon_sym_break,
      sym_continue_statement,
      anon_sym_continuing,
      anon_sym_return,
      anon_sym_const_assert,
      anon_sym_discard,
      sym_ident_pattern_token,
  [150] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(82), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(332), 6,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(330), 54,
      anon_sym_const,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_let,
      anon_sym_var,
      anon_sym__,
      anon_sym_if,
      anon_sym_switch,
      anon_sym_loop,
      anon_sym_for,
      anon_sym_while,
      anon_sym_break,
      sym_continue_statement,
      anon_sym_continuing,
      anon_sym_return,
      anon_sym_const_assert,
      anon_sym_discard,
      sym_ident_pattern_token,
  [225] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(83), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(336), 6,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(334), 54,
      anon_sym_const,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_let,
      anon_sym_var,
      anon_sym__,
      anon_sym_if,
      anon_sym_switch,
      anon_sym_loop,
      anon_sym_for,
      anon_sym_while,
      anon_sym_break,
      sym_continue_statement,
      anon_sym_continuing,
      anon_sym_return,
      anon_sym_const_assert,
      anon_sym_discard,
      sym_ident_pattern_token,
  [300] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(84), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(340), 6,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(338), 54,
      anon_sym_const,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_let,
      anon_sym_var,
      anon_sym__,
      anon_sym_if,
      anon_sym_switch,
      anon_sym_loop,
      anon_sym_for,
      anon_sym_while,
      anon_sym_break,
      sym_continue_statement,
      anon_sym_continuing,
      anon_sym_return,
      anon_sym_const_assert,
      anon_sym_discard,
      sym_ident_pattern_token,
  [375] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(85), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(344), 6,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(342), 54,
      anon_sym_const,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_let,
      anon_sym_var,
      anon_sym__,
      anon_sym_if,
      anon_sym_switch,
      anon_sym_loop,
      anon_sym_for,
      anon_sym_while,
      anon_sym_break,
      sym_continue_statement,
      anon_sym_continuing,
      anon_sym_return,
      anon_sym_const_assert,
      anon_sym_discard,
      sym_ident_pattern_token,
  [450] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(86), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(348), 6,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(346), 54,
      anon_sym_const,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_let,
      anon_sym_var,
      anon_sym__,
      anon_sym_if,
      anon_sym_switch,
      anon_sym_loop,
      anon_sym_for,
      anon_sym_while,
      anon_sym_break,
      sym_continue_statement,
      anon_sym_continuing,
      anon_sym_return,
      anon_sym_const_assert,
      anon_sym_discard,
      sym_ident_pattern_token,
  [525] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(87), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(352), 6,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(350), 54,
      anon_sym_const,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_let,
      anon_sym_var,
      anon_sym__,
      anon_sym_if,
      anon_sym_switch,
      anon_sym_loop,
      anon_sym_for,
      anon_sym_while,
      anon_sym_break,
      sym_continue_statement,
      anon_sym_continuing,
      anon_sym_return,
      anon_sym_const_assert,
      anon_sym_discard,
      sym_ident_pattern_token,
  [600] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(88), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(356), 6,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(354), 54,
      anon_sym_const,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_let,
      anon_sym_var,
      anon_sym__,
      anon_sym_if,
      anon_sym_switch,
      anon_sym_loop,
      anon_sym_for,
      anon_sym_while,
      anon_sym_break,
      sym_continue_statement,
      anon_sym_continuing,
      anon_sym_return,
      anon_sym_const_assert,
      anon_sym_discard,
      sym_ident_pattern_token,
  [675] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(89), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(360), 6,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(358), 54,
      anon_sym_const,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_let,
      anon_sym_var,
      anon_sym__,
      anon_sym_if,
      anon_sym_switch,
      anon_sym_loop,
      anon_sym_for,
      anon_sym_while,
      anon_sym_break,
      sym_continue_statement,
      anon_sym_continuing,
      anon_sym_return,
      anon_sym_const_assert,
      anon_sym_discard,
      sym_ident_pattern_token,
  [750] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(90), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(364), 6,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(362), 54,
      anon_sym_const,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_let,
      anon_sym_var,
      anon_sym__,
      anon_sym_if,
      anon_sym_switch,
      anon_sym_loop,
      anon_sym_for,
      anon_sym_while,
      anon_sym_break,
      sym_continue_statement,
      anon_sym_continuing,
      anon_sym_return,
      anon_sym_const_assert,
      anon_sym_discard,
      sym_ident_pattern_token,
  [825] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(91), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(368), 6,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(366), 54,
      anon_sym_const,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_let,
      anon_sym_var,
      anon_sym__,
      anon_sym_if,
      anon_sym_switch,
      anon_sym_loop,
      anon_sym_for,
      anon_sym_while,
      anon_sym_break,
      sym_continue_statement,
      anon_sym_continuing,
      anon_sym_return,
      anon_sym_const_assert,
      anon_sym_discard,
      sym_ident_pattern_token,
  [900] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(92), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(372), 12,
      aux_sym_decimal_float_literal_token1,
      aux_sym_decimal_float_literal_token2,
      aux_sym_decimal_float_literal_token3,
      aux_sym_decimal_float_literal_token5,
      aux_sym_hex_float_literal_token1,
      aux_sym_hex_float_literal_token3,
      anon_sym_LPAREN,
      anon_sym_DASH,
      anon_sym_BANG,
      anon_sym_TILDE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(370), 47,
      anon_sym_true,
      anon_sym_false,
      aux_sym_decimal_int_literal_token1,
      aux_sym_decimal_int_literal_token2,
      sym_hex_int_literal,
      aux_sym_decimal_float_literal_token4,
      aux_sym_hex_float_literal_token2,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_bitcast,
      sym_ident_pattern_token,
  [974] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(93), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(376), 12,
      aux_sym_decimal_float_literal_token1,
      aux_sym_decimal_float_literal_token2,
      aux_sym_decimal_float_literal_token3,
      aux_sym_decimal_float_literal_token5,
      aux_sym_hex_float_literal_token1,
      aux_sym_hex_float_literal_token3,
      anon_sym_LPAREN,
      anon_sym_DASH,
      anon_sym_BANG,
      anon_sym_TILDE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(374), 47,
      anon_sym_true,
      anon_sym_false,
      aux_sym_decimal_int_literal_token1,
      aux_sym_decimal_int_literal_token2,
      sym_hex_int_literal,
      aux_sym_decimal_float_literal_token4,
      aux_sym_hex_float_literal_token2,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_bitcast,
      sym_ident_pattern_token,
  [1048] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(94), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(380), 12,
      aux_sym_decimal_float_literal_token1,
      aux_sym_decimal_float_literal_token2,
      aux_sym_decimal_float_literal_token3,
      aux_sym_decimal_float_literal_token5,
      aux_sym_hex_float_literal_token1,
      aux_sym_hex_float_literal_token3,
      anon_sym_LPAREN,
      anon_sym_DASH,
      anon_sym_BANG,
      anon_sym_TILDE,
      anon_sym_STAR,
      anon_sym_AMP,
    ACTIONS(378), 47,
      anon_sym_true,
      anon_sym_false,
      aux_sym_decimal_int_literal_token1,
      aux_sym_decimal_int_literal_token2,
      sym_hex_int_literal,
      aux_sym_decimal_float_literal_token4,
      aux_sym_hex_float_literal_token2,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_bitcast,
      sym_ident_pattern_token,
  [1122] = 25,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      sym_ident_pattern_token,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(95), 1,
      sym__comment,
    STATE(98), 1,
      aux_sym_struct_member_repeat1,
    STATE(117), 1,
      sym_attribute,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    STATE(450), 1,
      sym_type_specifier,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [1230] = 25,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      sym_ident_pattern_token,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(96), 1,
      sym__comment,
    STATE(112), 1,
      aux_sym_struct_member_repeat1,
    STATE(117), 1,
      sym_attribute,
    STATE(410), 1,
      sym_type_specifier,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [1338] = 25,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      sym_ident_pattern_token,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(96), 1,
      aux_sym_struct_member_repeat1,
    STATE(97), 1,
      sym__comment,
    STATE(117), 1,
      sym_attribute,
    STATE(433), 1,
      sym_type_specifier,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [1446] = 25,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(29), 1,
      sym_ident_pattern_token,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(98), 1,
      sym__comment,
    STATE(112), 1,
      aux_sym_struct_member_repeat1,
    STATE(117), 1,
      sym_attribute,
    STATE(433), 1,
      sym_type_specifier,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [1554] = 22,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(29), 1,
      sym_ident_pattern_token,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(99), 1,
      sym__comment,
    STATE(403), 1,
      sym_type_specifier,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [1653] = 22,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(29), 1,
      sym_ident_pattern_token,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(100), 1,
      sym__comment,
    STATE(376), 1,
      sym_type_specifier,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [1752] = 22,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(29), 1,
      sym_ident_pattern_token,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(101), 1,
      sym__comment,
    STATE(379), 1,
      sym_type_specifier,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [1851] = 22,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(235), 1,
      sym_ident_pattern_token,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(102), 1,
      sym__comment,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    STATE(487), 1,
      sym_type_specifier,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [1950] = 22,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(235), 1,
      sym_ident_pattern_token,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(103), 1,
      sym__comment,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    STATE(483), 1,
      sym_type_specifier,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [2049] = 22,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(29), 1,
      sym_ident_pattern_token,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(104), 1,
      sym__comment,
    STATE(342), 1,
      sym_type_specifier,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [2148] = 22,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(235), 1,
      sym_ident_pattern_token,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(105), 1,
      sym__comment,
    STATE(356), 1,
      sym_type_specifier,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [2247] = 22,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(29), 1,
      sym_ident_pattern_token,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(106), 1,
      sym__comment,
    STATE(362), 1,
      sym_type_specifier,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [2346] = 22,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(29), 1,
      sym_ident_pattern_token,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(107), 1,
      sym__comment,
    STATE(365), 1,
      sym_type_specifier,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [2445] = 22,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(235), 1,
      sym_ident_pattern_token,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(108), 1,
      sym__comment,
    STATE(371), 1,
      sym_type_specifier,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [2544] = 22,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(235), 1,
      sym_ident_pattern_token,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(109), 1,
      sym__comment,
    STATE(412), 1,
      sym_type_specifier,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [2643] = 22,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(235), 1,
      sym_ident_pattern_token,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(110), 1,
      sym__comment,
    STATE(418), 1,
      sym_type_specifier,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [2742] = 22,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(55), 1,
      sym_multisampled_texture_type,
    ACTIONS(63), 1,
      anon_sym_ptr,
    ACTIONS(65), 1,
      anon_sym_atomic,
    ACTIONS(235), 1,
      sym_ident_pattern_token,
    ACTIONS(382), 1,
      anon_sym_array,
    STATE(111), 1,
      sym__comment,
    STATE(415), 1,
      sym_type_specifier,
    STATE(435), 1,
      sym_storage_texture_type,
    STATE(445), 1,
      sym_sampled_texture_type,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(51), 2,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
    STATE(254), 2,
      sym_array_type_specifier,
      sym_texture_and_sampler_types,
    STATE(260), 2,
      sym_sampler_type,
      sym_depth_texture_type,
    STATE(269), 2,
      sym_type_specifier_without_ident,
      sym_ident,
    STATE(390), 2,
      sym_vec_prefix,
      sym_mat_prefix,
    ACTIONS(67), 3,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
    ACTIONS(57), 4,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
    ACTIONS(59), 5,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
    ACTIONS(61), 5,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
    ACTIONS(53), 6,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
    ACTIONS(69), 9,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
  [2841] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(386), 1,
      anon_sym_AT,
    STATE(117), 1,
      sym_attribute,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(112), 2,
      sym__comment,
      aux_sym_struct_member_repeat1,
    ACTIONS(384), 42,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_var,
      anon_sym_override,
      anon_sym_fn,
      sym_ident_pattern_token,
  [2903] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(391), 1,
      anon_sym_AT,
    STATE(113), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(389), 42,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_var,
      anon_sym_override,
      anon_sym_fn,
      sym_ident_pattern_token,
  [2961] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(395), 1,
      anon_sym_AT,
    STATE(114), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(393), 42,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_var,
      anon_sym_override,
      anon_sym_fn,
      sym_ident_pattern_token,
  [3019] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(399), 1,
      anon_sym_AT,
    STATE(115), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(397), 42,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_var,
      anon_sym_override,
      anon_sym_fn,
      sym_ident_pattern_token,
  [3077] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(403), 1,
      anon_sym_AT,
    STATE(116), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(401), 42,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_var,
      anon_sym_override,
      anon_sym_fn,
      sym_ident_pattern_token,
  [3135] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(407), 1,
      anon_sym_AT,
    STATE(117), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(405), 42,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_var,
      anon_sym_override,
      anon_sym_fn,
      sym_ident_pattern_token,
  [3193] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(411), 1,
      anon_sym_AT,
    STATE(118), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(409), 42,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_var,
      anon_sym_override,
      anon_sym_fn,
      sym_ident_pattern_token,
  [3251] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(415), 1,
      anon_sym_AT,
    STATE(119), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(413), 42,
      anon_sym_array,
      anon_sym_sampler,
      anon_sym_sampler_comparison,
      anon_sym_texture_1d,
      anon_sym_texture_2d,
      anon_sym_texture_2d_array,
      anon_sym_texture_3d,
      anon_sym_texture_cube,
      anon_sym_texture_cube_array,
      sym_multisampled_texture_type,
      anon_sym_texture_storage_1d,
      anon_sym_texture_storage_2d,
      anon_sym_texture_storage_2d_array,
      anon_sym_texture_storage_3d,
      anon_sym_texture_depth_2d,
      anon_sym_texture_depth_2d_array,
      anon_sym_texture_depth_cube,
      anon_sym_texture_depth_cube_array,
      anon_sym_texture_depth_multisampled_2d,
      anon_sym_bool,
      anon_sym_f32,
      anon_sym_f16,
      anon_sym_i32,
      anon_sym_u32,
      anon_sym_ptr,
      anon_sym_atomic,
      anon_sym_vec2,
      anon_sym_vec3,
      anon_sym_vec4,
      anon_sym_mat2x2,
      anon_sym_mat2x3,
      anon_sym_mat2x4,
      anon_sym_mat3x2,
      anon_sym_mat3x3,
      anon_sym_mat3x4,
      anon_sym_mat4x2,
      anon_sym_mat4x3,
      anon_sym_mat4x4,
      anon_sym_var,
      anon_sym_override,
      anon_sym_fn,
      sym_ident_pattern_token,
  [3309] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(120), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(419), 6,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_EQ,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(417), 23,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3353] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(425), 1,
      anon_sym_LBRACK,
    ACTIONS(427), 1,
      anon_sym_DOT,
    STATE(121), 1,
      sym__comment,
    STATE(147), 1,
      sym_component_or_swizzle_specifier,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(423), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(421), 19,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3401] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(431), 1,
      anon_sym_LPAREN,
    STATE(122), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(433), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(429), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3445] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(425), 1,
      anon_sym_LBRACK,
    ACTIONS(427), 1,
      anon_sym_DOT,
    STATE(123), 1,
      sym__comment,
    STATE(145), 1,
      sym_component_or_swizzle_specifier,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(437), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(435), 19,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3493] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(425), 1,
      anon_sym_LBRACK,
    ACTIONS(427), 1,
      anon_sym_DOT,
    STATE(124), 1,
      sym__comment,
    STATE(142), 1,
      sym_component_or_swizzle_specifier,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(441), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(439), 19,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3541] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(125), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(445), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(443), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3582] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(126), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(433), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(429), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3623] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(127), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(449), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(447), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3664] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(128), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(453), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(451), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3705] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(129), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(457), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(455), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3746] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(130), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(461), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(459), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3787] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(131), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(465), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(463), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3828] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(132), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(469), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(467), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3869] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(133), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(473), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(471), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3910] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(134), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(477), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(475), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3951] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(135), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(481), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(479), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [3992] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(136), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(485), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(483), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [4033] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(137), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(489), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(487), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [4074] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(138), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(493), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(491), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [4115] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(139), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(497), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(495), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [4156] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(140), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(501), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(499), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [4197] = 24,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(9), 1,
      anon_sym_SEMI,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(13), 1,
      anon_sym_const,
    ACTIONS(15), 1,
      anon_sym_struct,
    ACTIONS(17), 1,
      anon_sym_alias,
    ACTIONS(19), 1,
      anon_sym_var,
    ACTIONS(21), 1,
      anon_sym_override,
    ACTIONS(23), 1,
      anon_sym_const_assert,
    ACTIONS(25), 1,
      anon_sym_fn,
    ACTIONS(27), 1,
      anon_sym_enable,
    ACTIONS(503), 1,
      ts_builtin_sym_end,
    STATE(117), 1,
      sym_attribute,
    STATE(141), 1,
      sym__comment,
    STATE(162), 1,
      aux_sym_translation_unit_repeat2,
    STATE(218), 1,
      aux_sym_translation_unit_repeat1,
    STATE(227), 1,
      sym_enable_directive,
    STATE(232), 1,
      sym_global_directive,
    STATE(262), 1,
      aux_sym_struct_member_repeat1,
    STATE(347), 1,
      sym_function_header,
    STATE(358), 1,
      sym_variable_decl,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(241), 2,
      sym_struct_decl,
      sym_function_decl,
    STATE(478), 4,
      sym_type_alias_decl,
      sym_global_variable_decl,
      sym_global_constant_decl,
      sym_const_assert_statement,
  [4275] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(142), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(507), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(505), 19,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [4314] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(143), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(511), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(509), 19,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [4353] = 9,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(517), 1,
      anon_sym_AMP,
    ACTIONS(521), 1,
      anon_sym_PIPE,
    ACTIONS(523), 1,
      anon_sym_CARET,
    STATE(144), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(519), 2,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
    ACTIONS(515), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(513), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [4400] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(145), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(441), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(439), 19,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [4439] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(146), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(527), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(525), 19,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [4478] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(147), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(531), 5,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_AMP,
      anon_sym_SLASH,
      anon_sym_PIPE,
    ACTIONS(529), 19,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
      anon_sym_CARET,
  [4517] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(441), 1,
      anon_sym_AMP,
    ACTIONS(533), 1,
      anon_sym_LBRACK,
    ACTIONS(535), 1,
      anon_sym_DOT,
    STATE(148), 1,
      sym__comment,
    STATE(181), 1,
      sym_component_or_swizzle_specifier,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(439), 19,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_EQ,
      anon_sym_RBRACK,
      anon_sym_PLUS_EQ,
      anon_sym_DASH_EQ,
      anon_sym_STAR_EQ,
      anon_sym_SLASH_EQ,
      anon_sym_PERCENT_EQ,
      anon_sym_AMP_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_GT_GT_EQ,
      anon_sym_LT_LT_EQ,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH_DASH,
  [4561] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(437), 1,
      anon_sym_AMP,
    ACTIONS(533), 1,
      anon_sym_LBRACK,
    ACTIONS(535), 1,
      anon_sym_DOT,
    STATE(149), 1,
      sym__comment,
    STATE(178), 1,
      sym_component_or_swizzle_specifier,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(435), 19,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_EQ,
      anon_sym_RBRACK,
      anon_sym_PLUS_EQ,
      anon_sym_DASH_EQ,
      anon_sym_STAR_EQ,
      anon_sym_SLASH_EQ,
      anon_sym_PERCENT_EQ,
      anon_sym_AMP_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_GT_GT_EQ,
      anon_sym_LT_LT_EQ,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH_DASH,
  [4605] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(419), 1,
      anon_sym_AMP,
    STATE(150), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(417), 22,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_PLUS_EQ,
      anon_sym_DASH_EQ,
      anon_sym_STAR_EQ,
      anon_sym_SLASH_EQ,
      anon_sym_PERCENT_EQ,
      anon_sym_AMP_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_GT_GT_EQ,
      anon_sym_LT_LT_EQ,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH_DASH,
  [4643] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(151), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(419), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(417), 19,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [4680] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(537), 1,
      anon_sym_LBRACK,
    ACTIONS(539), 1,
      anon_sym_DOT,
    STATE(152), 1,
      sym__comment,
    STATE(184), 1,
      sym_component_or_swizzle_specifier,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(441), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(439), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [4723] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(537), 1,
      anon_sym_LBRACK,
    ACTIONS(539), 1,
      anon_sym_DOT,
    STATE(153), 1,
      sym__comment,
    STATE(182), 1,
      sym_component_or_swizzle_specifier,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(437), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(435), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [4766] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(431), 1,
      anon_sym_LPAREN,
    STATE(154), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(433), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(429), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [4805] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(445), 1,
      anon_sym_AMP,
    STATE(155), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(443), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_PLUS_EQ,
      anon_sym_DASH_EQ,
      anon_sym_STAR_EQ,
      anon_sym_SLASH_EQ,
      anon_sym_PERCENT_EQ,
      anon_sym_AMP_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_GT_GT_EQ,
      anon_sym_LT_LT_EQ,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH_DASH,
  [4842] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(453), 1,
      anon_sym_AMP,
    STATE(156), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(451), 21,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_PLUS_EQ,
      anon_sym_DASH_EQ,
      anon_sym_STAR_EQ,
      anon_sym_SLASH_EQ,
      anon_sym_PERCENT_EQ,
      anon_sym_AMP_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_GT_GT_EQ,
      anon_sym_LT_LT_EQ,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH_DASH,
  [4879] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(537), 1,
      anon_sym_LBRACK,
    ACTIONS(539), 1,
      anon_sym_DOT,
    STATE(157), 1,
      sym__comment,
    STATE(185), 1,
      sym_component_or_swizzle_specifier,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(423), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(421), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [4922] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(158), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(433), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(429), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [4958] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(159), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(449), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(447), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [4994] = 19,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(541), 1,
      ts_builtin_sym_end,
    ACTIONS(543), 1,
      anon_sym_SEMI,
    ACTIONS(546), 1,
      anon_sym_AT,
    ACTIONS(549), 1,
      anon_sym_const,
    ACTIONS(552), 1,
      anon_sym_struct,
    ACTIONS(555), 1,
      anon_sym_alias,
    ACTIONS(558), 1,
      anon_sym_var,
    ACTIONS(561), 1,
      anon_sym_override,
    ACTIONS(564), 1,
      anon_sym_const_assert,
    ACTIONS(567), 1,
      anon_sym_fn,
    STATE(117), 1,
      sym_attribute,
    STATE(262), 1,
      aux_sym_struct_member_repeat1,
    STATE(347), 1,
      sym_function_header,
    STATE(358), 1,
      sym_variable_decl,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(160), 2,
      sym__comment,
      aux_sym_translation_unit_repeat2,
    STATE(241), 2,
      sym_struct_decl,
      sym_function_decl,
    STATE(478), 4,
      sym_type_alias_decl,
      sym_global_variable_decl,
      sym_global_constant_decl,
      sym_const_assert_statement,
  [5058] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(161), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(485), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(483), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5094] = 20,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(9), 1,
      anon_sym_SEMI,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(13), 1,
      anon_sym_const,
    ACTIONS(15), 1,
      anon_sym_struct,
    ACTIONS(17), 1,
      anon_sym_alias,
    ACTIONS(19), 1,
      anon_sym_var,
    ACTIONS(21), 1,
      anon_sym_override,
    ACTIONS(23), 1,
      anon_sym_const_assert,
    ACTIONS(25), 1,
      anon_sym_fn,
    ACTIONS(570), 1,
      ts_builtin_sym_end,
    STATE(117), 1,
      sym_attribute,
    STATE(160), 1,
      aux_sym_translation_unit_repeat2,
    STATE(162), 1,
      sym__comment,
    STATE(262), 1,
      aux_sym_struct_member_repeat1,
    STATE(347), 1,
      sym_function_header,
    STATE(358), 1,
      sym_variable_decl,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(241), 2,
      sym_struct_decl,
      sym_function_decl,
    STATE(478), 4,
      sym_type_alias_decl,
      sym_global_variable_decl,
      sym_global_constant_decl,
      sym_const_assert_statement,
  [5160] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(163), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(469), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(467), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5196] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(164), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(473), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(471), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5232] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(165), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(481), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(479), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5268] = 20,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(9), 1,
      anon_sym_SEMI,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(13), 1,
      anon_sym_const,
    ACTIONS(15), 1,
      anon_sym_struct,
    ACTIONS(17), 1,
      anon_sym_alias,
    ACTIONS(19), 1,
      anon_sym_var,
    ACTIONS(21), 1,
      anon_sym_override,
    ACTIONS(23), 1,
      anon_sym_const_assert,
    ACTIONS(25), 1,
      anon_sym_fn,
    ACTIONS(503), 1,
      ts_builtin_sym_end,
    STATE(117), 1,
      sym_attribute,
    STATE(160), 1,
      aux_sym_translation_unit_repeat2,
    STATE(166), 1,
      sym__comment,
    STATE(262), 1,
      aux_sym_struct_member_repeat1,
    STATE(347), 1,
      sym_function_header,
    STATE(358), 1,
      sym_variable_decl,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(241), 2,
      sym_struct_decl,
      sym_function_decl,
    STATE(478), 4,
      sym_type_alias_decl,
      sym_global_variable_decl,
      sym_global_constant_decl,
      sym_const_assert_statement,
  [5334] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(167), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(501), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(499), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5370] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(168), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(497), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(495), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5406] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(169), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(493), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(491), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5442] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(170), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(489), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(487), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5478] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(171), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(477), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(475), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5514] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(172), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(461), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(459), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5550] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(173), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(519), 2,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
    ACTIONS(515), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(513), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5588] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(174), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(465), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(463), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5624] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(175), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(457), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(455), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5660] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(176), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(453), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(451), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5696] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(177), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(445), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(443), 18,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5732] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(441), 1,
      anon_sym_AMP,
    STATE(178), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(439), 19,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_EQ,
      anon_sym_RBRACK,
      anon_sym_PLUS_EQ,
      anon_sym_DASH_EQ,
      anon_sym_STAR_EQ,
      anon_sym_SLASH_EQ,
      anon_sym_PERCENT_EQ,
      anon_sym_AMP_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_GT_GT_EQ,
      anon_sym_LT_LT_EQ,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH_DASH,
  [5767] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(578), 1,
      anon_sym_SLASH,
    STATE(67), 1,
      sym_multiplicative_operator,
    STATE(179), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(574), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(576), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(572), 14,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5808] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(578), 1,
      anon_sym_SLASH,
    STATE(67), 1,
      sym_multiplicative_operator,
    STATE(180), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(576), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(582), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(580), 14,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5849] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(507), 1,
      anon_sym_AMP,
    STATE(181), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(505), 19,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_EQ,
      anon_sym_RBRACK,
      anon_sym_PLUS_EQ,
      anon_sym_DASH_EQ,
      anon_sym_STAR_EQ,
      anon_sym_SLASH_EQ,
      anon_sym_PERCENT_EQ,
      anon_sym_AMP_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_GT_GT_EQ,
      anon_sym_LT_LT_EQ,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH_DASH,
  [5884] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(182), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(441), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(439), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5918] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(183), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(511), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(509), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5952] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(184), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(507), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(505), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [5986] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(185), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(531), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(529), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [6020] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(186), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(586), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(584), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [6054] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(187), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(527), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(525), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [6088] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(188), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(515), 3,
      anon_sym_LT,
      anon_sym_GT,
      anon_sym_SLASH,
    ACTIONS(513), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [6122] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(189), 1,
      sym__comment,
    STATE(431), 1,
      sym_texel_format,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(588), 17,
      anon_sym_rgba8unorm,
      anon_sym_rgba8snorm,
      anon_sym_rgba8uint,
      anon_sym_rgba8sint,
      anon_sym_rgba16uint,
      anon_sym_rgba16sint,
      anon_sym_rgba16float,
      anon_sym_r32uint,
      anon_sym_r32sint,
      anon_sym_r32float,
      anon_sym_rg32uint,
      anon_sym_rg32sint,
      anon_sym_rg32float,
      anon_sym_rgba32uint,
      anon_sym_rgba32sint,
      anon_sym_rgba32float,
      anon_sym_bgra8unorm,
  [6155] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(431), 1,
      anon_sym_LPAREN,
    ACTIONS(433), 1,
      anon_sym_SLASH,
    STATE(190), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(429), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6190] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(473), 1,
      anon_sym_SLASH,
    STATE(191), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(471), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6222] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(493), 1,
      anon_sym_SLASH,
    STATE(192), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(491), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6254] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(461), 1,
      anon_sym_SLASH,
    STATE(193), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(459), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6286] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(477), 1,
      anon_sym_SLASH,
    STATE(194), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(475), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6318] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(489), 1,
      anon_sym_SLASH,
    STATE(195), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(487), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6350] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(469), 1,
      anon_sym_SLASH,
    STATE(196), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(467), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6382] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(465), 1,
      anon_sym_SLASH,
    STATE(197), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(463), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6414] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(433), 1,
      anon_sym_SLASH,
    STATE(198), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(429), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6446] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(501), 1,
      anon_sym_SLASH,
    STATE(199), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(499), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6478] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(457), 1,
      anon_sym_SLASH,
    STATE(200), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(455), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6510] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(485), 1,
      anon_sym_SLASH,
    STATE(201), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(483), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6542] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(481), 1,
      anon_sym_SLASH,
    STATE(202), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(479), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6574] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(497), 1,
      anon_sym_SLASH,
    STATE(203), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(495), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6606] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(449), 1,
      anon_sym_SLASH,
    STATE(204), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(447), 16,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_LBRACK,
      anon_sym_RBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6638] = 7,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(51), 1,
      sym_additive_operator,
    STATE(205), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(592), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(594), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(590), 12,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [6674] = 7,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(533), 1,
      anon_sym_LBRACK,
    ACTIONS(535), 1,
      anon_sym_DOT,
    STATE(206), 1,
      sym__comment,
    STATE(216), 1,
      sym_component_or_swizzle_specifier,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(596), 13,
      anon_sym_EQ,
      anon_sym_PLUS_EQ,
      anon_sym_DASH_EQ,
      anon_sym_STAR_EQ,
      anon_sym_SLASH_EQ,
      anon_sym_PERCENT_EQ,
      anon_sym_AMP_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_GT_GT_EQ,
      anon_sym_LT_LT_EQ,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH_DASH,
  [6709] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(207), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(598), 16,
      anon_sym_RPAREN,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_PLUS_EQ,
      anon_sym_DASH_EQ,
      anon_sym_STAR_EQ,
      anon_sym_SLASH_EQ,
      anon_sym_PERCENT_EQ,
      anon_sym_AMP_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_GT_GT_EQ,
      anon_sym_LT_LT_EQ,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH_DASH,
  [6738] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(208), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(600), 16,
      anon_sym_RPAREN,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_PLUS_EQ,
      anon_sym_DASH_EQ,
      anon_sym_STAR_EQ,
      anon_sym_SLASH_EQ,
      anon_sym_PERCENT_EQ,
      anon_sym_AMP_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_GT_GT_EQ,
      anon_sym_LT_LT_EQ,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH_DASH,
  [6767] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(431), 1,
      anon_sym_LPAREN,
    STATE(209), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(598), 15,
      anon_sym_EQ,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_PLUS_EQ,
      anon_sym_DASH_EQ,
      anon_sym_STAR_EQ,
      anon_sym_SLASH_EQ,
      anon_sym_PERCENT_EQ,
      anon_sym_AMP_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_GT_GT_EQ,
      anon_sym_LT_LT_EQ,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH_DASH,
  [6798] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(531), 1,
      anon_sym_SLASH,
    STATE(210), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(529), 14,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6828] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(511), 1,
      anon_sym_SLASH,
    STATE(211), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(509), 14,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6858] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(527), 1,
      anon_sym_SLASH,
    STATE(212), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(525), 14,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [6888] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(515), 1,
      anon_sym_SLASH,
    STATE(213), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(602), 2,
      anon_sym_LT_LT,
      anon_sym_GT_GT,
    ACTIONS(513), 12,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [6920] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(214), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(606), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(608), 4,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
    ACTIONS(604), 8,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [6951] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(612), 1,
      anon_sym_builtin,
    ACTIONS(616), 1,
      anon_sym_interpolate,
    ACTIONS(618), 1,
      anon_sym_workgroup_size,
    STATE(215), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(614), 5,
      anon_sym_const,
      anon_sym_invariant,
      anon_sym_vertex,
      anon_sym_fragment,
      anon_sym_compute,
    ACTIONS(610), 6,
      anon_sym_align,
      anon_sym_binding,
      anon_sym_group,
      anon_sym_id,
      anon_sym_location,
      anon_sym_size,
  [6986] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(216), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(620), 14,
      anon_sym_RPAREN,
      anon_sym_EQ,
      anon_sym_PLUS_EQ,
      anon_sym_DASH_EQ,
      anon_sym_STAR_EQ,
      anon_sym_SLASH_EQ,
      anon_sym_PERCENT_EQ,
      anon_sym_AMP_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_GT_GT_EQ,
      anon_sym_LT_LT_EQ,
      anon_sym_PLUS_PLUS,
      anon_sym_DASH_DASH,
  [7013] = 7,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(578), 1,
      anon_sym_SLASH,
    STATE(56), 1,
      sym_multiplicative_operator,
    STATE(217), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(576), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(580), 10,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [7046] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(624), 1,
      anon_sym_const,
    ACTIONS(626), 1,
      anon_sym_enable,
    STATE(227), 1,
      sym_enable_directive,
    STATE(232), 1,
      sym_global_directive,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(218), 2,
      sym__comment,
      aux_sym_translation_unit_repeat1,
    ACTIONS(622), 9,
      ts_builtin_sym_end,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_struct,
      anon_sym_alias,
      anon_sym_var,
      anon_sym_override,
      anon_sym_const_assert,
      anon_sym_fn,
  [7081] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(629), 1,
      anon_sym_EQ,
    ACTIONS(633), 1,
      anon_sym_PLUS_PLUS,
    ACTIONS(635), 1,
      anon_sym_DASH_DASH,
    STATE(24), 1,
      sym_compound_assignment_operator,
    STATE(219), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(631), 10,
      anon_sym_PLUS_EQ,
      anon_sym_DASH_EQ,
      anon_sym_STAR_EQ,
      anon_sym_SLASH_EQ,
      anon_sym_PERCENT_EQ,
      anon_sym_AMP_EQ,
      anon_sym_PIPE_EQ,
      anon_sym_CARET_EQ,
      anon_sym_GT_GT_EQ,
      anon_sym_LT_LT_EQ,
  [7116] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(220), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(639), 2,
      anon_sym_LT,
      anon_sym_GT,
    ACTIONS(637), 12,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_LT_EQ,
      anon_sym_GT_EQ,
      anon_sym_EQ_EQ,
      anon_sym_BANG_EQ,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [7145] = 7,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(578), 1,
      anon_sym_SLASH,
    STATE(56), 1,
      sym_multiplicative_operator,
    STATE(221), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(576), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(572), 10,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_DASH,
      anon_sym_PLUS,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [7178] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(222), 1,
      sym__comment,
    STATE(331), 1,
      sym_builtin_value_name,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(641), 12,
      anon_sym_vertex_index,
      anon_sym_instance_index,
      anon_sym_position,
      anon_sym_front_facing,
      anon_sym_frag_depth,
      anon_sym_local_invocation_id,
      anon_sym_local_invocation_index,
      anon_sym_global_invocation_id,
      anon_sym_workgroup_id,
      anon_sym_num_workgroups,
      anon_sym_sample_index,
      anon_sym_sample_mask,
  [7206] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(419), 1,
      anon_sym_SLASH,
    STATE(223), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(417), 12,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [7234] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(437), 1,
      anon_sym_SLASH,
    ACTIONS(643), 1,
      anon_sym_LBRACK,
    ACTIONS(645), 1,
      anon_sym_DOT,
    STATE(224), 1,
      sym__comment,
    STATE(247), 1,
      sym_component_or_swizzle_specifier,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(435), 8,
      anon_sym_GT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [7267] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(423), 1,
      anon_sym_SLASH,
    ACTIONS(643), 1,
      anon_sym_LBRACK,
    ACTIONS(645), 1,
      anon_sym_DOT,
    STATE(210), 1,
      sym_component_or_swizzle_specifier,
    STATE(225), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(421), 8,
      anon_sym_GT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [7300] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(441), 1,
      anon_sym_SLASH,
    ACTIONS(643), 1,
      anon_sym_LBRACK,
    ACTIONS(645), 1,
      anon_sym_DOT,
    STATE(226), 1,
      sym__comment,
    STATE(246), 1,
      sym_component_or_swizzle_specifier,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(439), 8,
      anon_sym_GT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [7333] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(649), 1,
      anon_sym_const,
    STATE(227), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(647), 10,
      ts_builtin_sym_end,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_struct,
      anon_sym_alias,
      anon_sym_var,
      anon_sym_override,
      anon_sym_const_assert,
      anon_sym_fn,
      anon_sym_enable,
  [7359] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(653), 1,
      anon_sym_const,
    STATE(228), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(651), 10,
      ts_builtin_sym_end,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_struct,
      anon_sym_alias,
      anon_sym_var,
      anon_sym_override,
      anon_sym_const_assert,
      anon_sym_fn,
      anon_sym_enable,
  [7385] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(453), 1,
      anon_sym_SLASH,
    STATE(229), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(451), 10,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [7411] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(445), 1,
      anon_sym_SLASH,
    STATE(230), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(443), 10,
      anon_sym_GT,
      anon_sym_LBRACK,
      anon_sym_DOT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [7437] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(655), 1,
      sym_ident_pattern_token,
    STATE(231), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(224), 2,
      sym_member_ident,
      sym_swizzle_name,
    ACTIONS(657), 8,
      anon_sym_SLASH_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
  [7465] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(661), 1,
      anon_sym_const,
    STATE(232), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(659), 10,
      ts_builtin_sym_end,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_struct,
      anon_sym_alias,
      anon_sym_var,
      anon_sym_override,
      anon_sym_const_assert,
      anon_sym_fn,
      anon_sym_enable,
  [7491] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(663), 1,
      sym_ident_pattern_token,
    STATE(233), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(123), 2,
      sym_member_ident,
      sym_swizzle_name,
    ACTIONS(665), 8,
      anon_sym_SLASH_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
  [7519] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(52), 1,
      sym_additive_operator,
    STATE(234), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(594), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
    ACTIONS(590), 8,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [7547] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(667), 1,
      sym_ident_pattern_token,
    STATE(235), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(149), 2,
      sym_member_ident,
      sym_swizzle_name,
    ACTIONS(669), 8,
      anon_sym_SLASH_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
  [7575] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(671), 1,
      sym_ident_pattern_token,
    STATE(236), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(153), 2,
      sym_member_ident,
      sym_swizzle_name,
    ACTIONS(673), 8,
      anon_sym_SLASH_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_LBRACKrgba_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
      anon_sym_SLASH_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_LBRACKxyzw_RBRACK_SLASH,
  [7603] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(677), 1,
      anon_sym_const,
    STATE(237), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(675), 9,
      ts_builtin_sym_end,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_struct,
      anon_sym_alias,
      anon_sym_var,
      anon_sym_override,
      anon_sym_const_assert,
      anon_sym_fn,
  [7628] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(681), 1,
      anon_sym_const,
    STATE(238), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(679), 9,
      ts_builtin_sym_end,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_struct,
      anon_sym_alias,
      anon_sym_var,
      anon_sym_override,
      anon_sym_const_assert,
      anon_sym_fn,
  [7653] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(685), 1,
      anon_sym_const,
    STATE(239), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(683), 9,
      ts_builtin_sym_end,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_struct,
      anon_sym_alias,
      anon_sym_var,
      anon_sym_override,
      anon_sym_const_assert,
      anon_sym_fn,
  [7678] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(689), 1,
      anon_sym_const,
    STATE(240), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(687), 9,
      ts_builtin_sym_end,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_struct,
      anon_sym_alias,
      anon_sym_var,
      anon_sym_override,
      anon_sym_const_assert,
      anon_sym_fn,
  [7703] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(693), 1,
      anon_sym_const,
    STATE(241), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(691), 9,
      ts_builtin_sym_end,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_struct,
      anon_sym_alias,
      anon_sym_var,
      anon_sym_override,
      anon_sym_const_assert,
      anon_sym_fn,
  [7728] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(697), 1,
      anon_sym_const,
    STATE(242), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(695), 9,
      ts_builtin_sym_end,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_struct,
      anon_sym_alias,
      anon_sym_var,
      anon_sym_override,
      anon_sym_const_assert,
      anon_sym_fn,
  [7753] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(699), 1,
      anon_sym_const,
    STATE(243), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(541), 9,
      ts_builtin_sym_end,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_struct,
      anon_sym_alias,
      anon_sym_var,
      anon_sym_override,
      anon_sym_const_assert,
      anon_sym_fn,
  [7778] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(703), 1,
      anon_sym_const,
    STATE(244), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(701), 9,
      ts_builtin_sym_end,
      anon_sym_SEMI,
      anon_sym_AT,
      anon_sym_struct,
      anon_sym_alias,
      anon_sym_var,
      anon_sym_override,
      anon_sym_const_assert,
      anon_sym_fn,
  [7803] = 7,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(533), 1,
      anon_sym_LBRACK,
    ACTIONS(535), 1,
      anon_sym_DOT,
    STATE(210), 1,
      sym_component_or_swizzle_specifier,
    STATE(245), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(421), 7,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_AMP,
  [7832] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(507), 1,
      anon_sym_SLASH,
    STATE(246), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(505), 8,
      anon_sym_GT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [7856] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(441), 1,
      anon_sym_SLASH,
    STATE(247), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(439), 8,
      anon_sym_GT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_AMP,
      anon_sym_PERCENT,
      anon_sym_PLUS,
      anon_sym_PIPE,
      anon_sym_CARET,
  [7880] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(515), 1,
      anon_sym_SLASH,
    ACTIONS(523), 1,
      anon_sym_CARET,
    ACTIONS(705), 1,
      anon_sym_AMP,
    ACTIONS(707), 1,
      anon_sym_PIPE,
    STATE(248), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(513), 5,
      anon_sym_GT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
  [7910] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(249), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(709), 8,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [7931] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(713), 1,
      anon_sym_AMP_AMP,
    ACTIONS(715), 1,
      anon_sym_PIPE_PIPE,
    STATE(250), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(711), 6,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
  [7956] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(251), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(717), 8,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [7977] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(252), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(719), 8,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [7998] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(723), 1,
      anon_sym_AMP,
    STATE(253), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(721), 7,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
  [8021] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(254), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(725), 8,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [8042] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(255), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(727), 8,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [8063] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(729), 1,
      anon_sym_CARET,
    STATE(256), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(721), 7,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
  [8086] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(731), 1,
      anon_sym_PIPE,
    STATE(257), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(721), 7,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
  [8109] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(258), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(733), 8,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [8130] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(259), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(735), 8,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [8151] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(260), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(737), 8,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [8172] = 11,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    ACTIONS(741), 1,
      anon_sym_RPAREN,
    STATE(117), 1,
      sym_attribute,
    STATE(261), 1,
      sym__comment,
    STATE(292), 1,
      aux_sym_struct_member_repeat1,
    STATE(312), 1,
      sym_param,
    STATE(448), 1,
      sym_param_list,
    STATE(449), 1,
      sym_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [8207] = 11,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(19), 1,
      anon_sym_var,
    ACTIONS(25), 1,
      anon_sym_fn,
    ACTIONS(743), 1,
      anon_sym_override,
    STATE(112), 1,
      aux_sym_struct_member_repeat1,
    STATE(117), 1,
      sym_attribute,
    STATE(262), 1,
      sym__comment,
    STATE(349), 1,
      sym_variable_decl,
    STATE(350), 1,
      sym_function_header,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [8242] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(263), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(745), 8,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
      anon_sym_AMP_AMP,
      anon_sym_PIPE_PIPE,
  [8263] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(264), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(747), 8,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [8284] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(265), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(749), 8,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [8305] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(266), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(751), 8,
      anon_sym_SEMI,
      anon_sym_LPAREN,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [8326] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(755), 1,
      anon_sym_PIPE_PIPE,
    STATE(267), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(753), 6,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
  [8348] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(757), 1,
      anon_sym_AMP_AMP,
    STATE(268), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(753), 6,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
  [8370] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(269), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(759), 7,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_GT,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_EQ,
  [8390] = 10,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    ACTIONS(761), 1,
      anon_sym_RPAREN,
    STATE(117), 1,
      sym_attribute,
    STATE(270), 1,
      sym__comment,
    STATE(292), 1,
      aux_sym_struct_member_repeat1,
    STATE(377), 1,
      sym_param,
    STATE(449), 1,
      sym_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [8422] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(763), 1,
      anon_sym_RBRACE,
    ACTIONS(765), 1,
      anon_sym_case,
    ACTIONS(768), 1,
      anon_sym_default,
    STATE(318), 1,
      sym_switch_body,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(271), 2,
      sym__comment,
      aux_sym_switch_statement_repeat1,
    STATE(319), 2,
      sym_case_clause,
      sym_default_alone_clause,
  [8450] = 10,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    ACTIONS(771), 1,
      anon_sym_RPAREN,
    STATE(117), 1,
      sym_attribute,
    STATE(272), 1,
      sym__comment,
    STATE(292), 1,
      aux_sym_struct_member_repeat1,
    STATE(377), 1,
      sym_param,
    STATE(449), 1,
      sym_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [8482] = 9,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(101), 1,
      anon_sym_LPAREN,
    ACTIONS(773), 1,
      sym_ident_pattern_token,
    STATE(206), 1,
      sym_core_lhs_expression,
    STATE(207), 1,
      sym_ident,
    STATE(216), 1,
      sym_lhs_expression,
    STATE(273), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(111), 2,
      anon_sym_STAR,
      anon_sym_AMP,
  [8512] = 7,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(578), 1,
      anon_sym_SLASH,
    STATE(55), 1,
      sym_multiplicative_operator,
    STATE(274), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(576), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(572), 3,
      anon_sym_GT,
      anon_sym_DASH,
      anon_sym_PLUS,
  [8538] = 7,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(578), 1,
      anon_sym_SLASH,
    STATE(55), 1,
      sym_multiplicative_operator,
    STATE(275), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(576), 2,
      anon_sym_STAR,
      anon_sym_PERCENT,
    ACTIONS(580), 3,
      anon_sym_GT,
      anon_sym_DASH,
      anon_sym_PLUS,
  [8564] = 10,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(663), 1,
      sym_ident_pattern_token,
    ACTIONS(775), 1,
      anon_sym_RBRACE,
    STATE(117), 1,
      sym_attribute,
    STATE(276), 1,
      sym__comment,
    STATE(293), 1,
      aux_sym_struct_member_repeat1,
    STATE(344), 1,
      sym_struct_member,
    STATE(396), 1,
      sym_member_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [8596] = 10,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(663), 1,
      sym_ident_pattern_token,
    ACTIONS(777), 1,
      anon_sym_RBRACE,
    STATE(117), 1,
      sym_attribute,
    STATE(277), 1,
      sym__comment,
    STATE(293), 1,
      aux_sym_struct_member_repeat1,
    STATE(344), 1,
      sym_struct_member,
    STATE(396), 1,
      sym_member_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [8628] = 9,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(779), 1,
      anon_sym_RBRACE,
    ACTIONS(781), 1,
      anon_sym_case,
    ACTIONS(783), 1,
      anon_sym_default,
    STATE(271), 1,
      aux_sym_switch_statement_repeat1,
    STATE(278), 1,
      sym__comment,
    STATE(318), 1,
      sym_switch_body,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(319), 2,
      sym_case_clause,
      sym_default_alone_clause,
  [8658] = 9,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(101), 1,
      anon_sym_LPAREN,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    STATE(207), 1,
      sym_ident,
    STATE(279), 1,
      sym__comment,
    STATE(298), 1,
      sym_core_lhs_expression,
    STATE(451), 1,
      sym_lhs_expression,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(785), 2,
      anon_sym_STAR,
      anon_sym_AMP,
  [8688] = 9,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(101), 1,
      anon_sym_LPAREN,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    STATE(207), 1,
      sym_ident,
    STATE(216), 1,
      sym_lhs_expression,
    STATE(280), 1,
      sym__comment,
    STATE(298), 1,
      sym_core_lhs_expression,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(785), 2,
      anon_sym_STAR,
      anon_sym_AMP,
  [8718] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(281), 1,
      sym__comment,
    STATE(354), 1,
      sym_address_space,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(787), 5,
      anon_sym_function,
      anon_sym_private,
      anon_sym_workgroup,
      anon_sym_uniform,
      anon_sym_storage,
  [8739] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(282), 1,
      sym__comment,
    STATE(414), 1,
      sym_address_space,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(787), 5,
      anon_sym_function,
      anon_sym_private,
      anon_sym_workgroup,
      anon_sym_uniform,
      anon_sym_storage,
  [8760] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(781), 1,
      anon_sym_case,
    ACTIONS(783), 1,
      anon_sym_default,
    STATE(278), 1,
      aux_sym_switch_statement_repeat1,
    STATE(283), 1,
      sym__comment,
    STATE(318), 1,
      sym_switch_body,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(319), 2,
      sym_case_clause,
      sym_default_alone_clause,
  [8787] = 9,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(663), 1,
      sym_ident_pattern_token,
    STATE(117), 1,
      sym_attribute,
    STATE(284), 1,
      sym__comment,
    STATE(293), 1,
      aux_sym_struct_member_repeat1,
    STATE(344), 1,
      sym_struct_member,
    STATE(396), 1,
      sym_member_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [8816] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(586), 1,
      anon_sym_SLASH,
    STATE(285), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(584), 5,
      anon_sym_GT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
  [8837] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(515), 1,
      anon_sym_SLASH,
    STATE(286), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(513), 5,
      anon_sym_GT,
      anon_sym_DASH,
      anon_sym_STAR,
      anon_sym_PERCENT,
      anon_sym_PLUS,
  [8858] = 9,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    STATE(117), 1,
      sym_attribute,
    STATE(287), 1,
      sym__comment,
    STATE(292), 1,
      aux_sym_struct_member_repeat1,
    STATE(377), 1,
      sym_param,
    STATE(449), 1,
      sym_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [8887] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(288), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(711), 6,
      anon_sym_SEMI,
      anon_sym_COMMA,
      anon_sym_RPAREN,
      anon_sym_LBRACE,
      anon_sym_COLON,
      anon_sym_RBRACK,
  [8906] = 9,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(663), 1,
      sym_ident_pattern_token,
    STATE(117), 1,
      sym_attribute,
    STATE(289), 1,
      sym__comment,
    STATE(293), 1,
      aux_sym_struct_member_repeat1,
    STATE(311), 1,
      sym_struct_member,
    STATE(396), 1,
      sym_member_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [8935] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(91), 1,
      anon_sym_RPAREN,
    STATE(290), 1,
      sym__comment,
    STATE(327), 1,
      sym_interpolation_sample_name,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(789), 3,
      anon_sym_center,
      anon_sym_centroid,
      anon_sym_sample,
  [8957] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    ACTIONS(791), 1,
      anon_sym_LT,
    STATE(291), 1,
      sym__comment,
    STATE(307), 1,
      sym_variable_qualifier,
    STATE(313), 1,
      sym_ident,
    STATE(388), 1,
      sym_optionally_typed_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [8983] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    STATE(112), 1,
      aux_sym_struct_member_repeat1,
    STATE(117), 1,
      sym_attribute,
    STATE(292), 1,
      sym__comment,
    STATE(402), 1,
      sym_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9009] = 8,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(11), 1,
      anon_sym_AT,
    ACTIONS(663), 1,
      sym_ident_pattern_token,
    STATE(112), 1,
      aux_sym_struct_member_repeat1,
    STATE(117), 1,
      sym_attribute,
    STATE(293), 1,
      sym__comment,
    STATE(488), 1,
      sym_member_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9035] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(294), 1,
      sym__comment,
    STATE(306), 1,
      sym_interpolation_type_name,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(793), 3,
      anon_sym_perspective,
      anon_sym_linear,
      anon_sym_flat,
  [9054] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(795), 1,
      anon_sym_read,
    STATE(295), 1,
      sym__comment,
    STATE(411), 1,
      sym_access_mode,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(797), 2,
      anon_sym_write,
      anon_sym_read_write,
  [9075] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(795), 1,
      anon_sym_read,
    STATE(296), 1,
      sym__comment,
    STATE(452), 1,
      sym_access_mode,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(797), 2,
      anon_sym_write,
      anon_sym_read_write,
  [9096] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(799), 1,
      anon_sym_COMMA,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(802), 2,
      anon_sym_LBRACE,
      anon_sym_COLON,
    STATE(297), 2,
      sym__comment,
      aux_sym_case_selectors_repeat1,
  [9115] = 7,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(425), 1,
      anon_sym_LBRACK,
    ACTIONS(427), 1,
      anon_sym_DOT,
    ACTIONS(596), 1,
      anon_sym_RPAREN,
    STATE(216), 1,
      sym_component_or_swizzle_specifier,
    STATE(298), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9138] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(795), 1,
      anon_sym_read,
    STATE(299), 1,
      sym__comment,
    STATE(490), 1,
      sym_access_mode,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(797), 2,
      anon_sym_write,
      anon_sym_read_write,
  [9159] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(804), 1,
      anon_sym_COMMA,
    STATE(297), 1,
      aux_sym_case_selectors_repeat1,
    STATE(300), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(77), 2,
      anon_sym_LBRACE,
      anon_sym_COLON,
  [9180] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(806), 1,
      anon_sym_COMMA,
    STATE(300), 1,
      aux_sym_case_selectors_repeat1,
    STATE(301), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(808), 2,
      anon_sym_LBRACE,
      anon_sym_COLON,
  [9201] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(810), 1,
      anon_sym_GT,
    STATE(50), 1,
      sym_additive_operator,
    STATE(302), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(594), 2,
      anon_sym_DASH,
      anon_sym_PLUS,
  [9222] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(303), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(812), 3,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON,
  [9238] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(304), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(814), 3,
      anon_sym_RBRACE,
      anon_sym_case,
      anon_sym_default,
  [9254] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(816), 1,
      anon_sym_COMMA,
    ACTIONS(819), 1,
      anon_sym_RBRACE,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(305), 2,
      sym__comment,
      aux_sym_struct_body_decl_repeat1,
  [9272] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(821), 1,
      anon_sym_COMMA,
    ACTIONS(823), 1,
      anon_sym_RPAREN,
    STATE(113), 1,
      sym_attrib_end,
    STATE(306), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9292] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    STATE(307), 1,
      sym__comment,
    STATE(313), 1,
      sym_ident,
    STATE(359), 1,
      sym_optionally_typed_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9312] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(823), 1,
      anon_sym_RPAREN,
    ACTIONS(825), 1,
      anon_sym_COMMA,
    STATE(113), 1,
      sym_attrib_end,
    STATE(308), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9332] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(827), 1,
      anon_sym_COMMA,
    ACTIONS(830), 1,
      anon_sym_RPAREN,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(309), 2,
      sym__comment,
      aux_sym_expression_comma_list_repeat1,
  [9350] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(85), 1,
      anon_sym_RPAREN,
    ACTIONS(832), 1,
      anon_sym_COMMA,
    STATE(309), 1,
      aux_sym_expression_comma_list_repeat1,
    STATE(310), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9370] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(834), 1,
      anon_sym_COMMA,
    ACTIONS(836), 1,
      anon_sym_RBRACE,
    STATE(311), 1,
      sym__comment,
    STATE(325), 1,
      aux_sym_struct_body_decl_repeat1,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9390] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(838), 1,
      anon_sym_COMMA,
    ACTIONS(840), 1,
      anon_sym_RPAREN,
    STATE(312), 1,
      sym__comment,
    STATE(320), 1,
      aux_sym_param_list_repeat1,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9410] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(844), 1,
      anon_sym_COLON,
    STATE(313), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(842), 2,
      anon_sym_SEMI,
      anon_sym_EQ,
  [9428] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(846), 1,
      anon_sym_COMMA,
    ACTIONS(849), 1,
      anon_sym_RPAREN,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    STATE(314), 2,
      sym__comment,
      aux_sym_param_list_repeat1,
  [9446] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(315), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(802), 3,
      anon_sym_COMMA,
      anon_sym_LBRACE,
      anon_sym_COLON,
  [9462] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(316), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(851), 3,
      anon_sym_RBRACE,
      anon_sym_case,
      anon_sym_default,
  [9478] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(105), 1,
      anon_sym_LBRACE,
    ACTIONS(853), 1,
      anon_sym_COLON,
    STATE(304), 1,
      sym_compound_statement,
    STATE(317), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9498] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(318), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(855), 3,
      anon_sym_RBRACE,
      anon_sym_case,
      anon_sym_default,
  [9514] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(319), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(857), 3,
      anon_sym_RBRACE,
      anon_sym_case,
      anon_sym_default,
  [9530] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(761), 1,
      anon_sym_RPAREN,
    ACTIONS(859), 1,
      anon_sym_COMMA,
    STATE(314), 1,
      aux_sym_param_list_repeat1,
    STATE(320), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9550] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    STATE(313), 1,
      sym_ident,
    STATE(321), 1,
      sym__comment,
    STATE(457), 1,
      sym_optionally_typed_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9570] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    STATE(313), 1,
      sym_ident,
    STATE(322), 1,
      sym__comment,
    STATE(454), 1,
      sym_optionally_typed_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9590] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(323), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(861), 3,
      anon_sym_RBRACE,
      anon_sym_case,
      anon_sym_default,
  [9606] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(324), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(863), 3,
      anon_sym_RBRACE,
      anon_sym_case,
      anon_sym_default,
  [9622] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(777), 1,
      anon_sym_RBRACE,
    ACTIONS(865), 1,
      anon_sym_COMMA,
    STATE(305), 1,
      aux_sym_struct_body_decl_repeat1,
    STATE(325), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9642] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(823), 1,
      anon_sym_RPAREN,
    ACTIONS(867), 1,
      anon_sym_COMMA,
    STATE(118), 1,
      sym_attrib_end,
    STATE(326), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9662] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(823), 1,
      anon_sym_RPAREN,
    ACTIONS(869), 1,
      anon_sym_COMMA,
    STATE(118), 1,
      sym_attrib_end,
    STATE(327), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9682] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(823), 1,
      anon_sym_RPAREN,
    ACTIONS(869), 1,
      anon_sym_COMMA,
    STATE(119), 1,
      sym_attrib_end,
    STATE(328), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9702] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(871), 1,
      anon_sym_COMMA,
    ACTIONS(873), 1,
      anon_sym_RPAREN,
    STATE(310), 1,
      aux_sym_expression_comma_list_repeat1,
    STATE(329), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9722] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    STATE(313), 1,
      sym_ident,
    STATE(330), 1,
      sym__comment,
    STATE(380), 1,
      sym_optionally_typed_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9742] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(823), 1,
      anon_sym_RPAREN,
    ACTIONS(869), 1,
      anon_sym_COMMA,
    STATE(113), 1,
      sym_attrib_end,
    STATE(331), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9762] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(105), 1,
      anon_sym_LBRACE,
    ACTIONS(875), 1,
      anon_sym_COLON,
    STATE(324), 1,
      sym_compound_statement,
    STATE(332), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9782] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    STATE(313), 1,
      sym_ident,
    STATE(333), 1,
      sym__comment,
    STATE(385), 1,
      sym_optionally_typed_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9802] = 6,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(105), 1,
      anon_sym_LBRACE,
    ACTIONS(877), 1,
      anon_sym_if,
    STATE(80), 1,
      sym_compound_statement,
    STATE(334), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9822] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(273), 1,
      anon_sym_LPAREN,
    STATE(161), 1,
      sym_paren_expression,
    STATE(335), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9839] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(105), 1,
      anon_sym_LBRACE,
    STATE(79), 1,
      sym_compound_statement,
    STATE(336), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9856] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(337), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(879), 2,
      anon_sym_SEMI,
      anon_sym_RPAREN,
  [9871] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(338), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(881), 2,
      anon_sym_SEMI,
      anon_sym_RPAREN,
  [9886] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(431), 1,
      anon_sym_LPAREN,
    ACTIONS(883), 1,
      anon_sym_LT,
    STATE(339), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9903] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(885), 1,
      anon_sym_LPAREN,
    STATE(127), 1,
      sym_argument_expression_list,
    STATE(340), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9920] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(887), 1,
      anon_sym_LPAREN,
    STATE(204), 1,
      sym_argument_expression_list,
    STATE(341), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9937] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(342), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(889), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [9952] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(105), 1,
      anon_sym_LBRACE,
    STATE(88), 1,
      sym_compound_statement,
    STATE(343), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [9969] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(344), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(819), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [9984] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(105), 1,
      anon_sym_LBRACE,
    STATE(323), 1,
      sym_compound_statement,
    STATE(345), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10001] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(891), 1,
      anon_sym_LBRACE,
    STATE(238), 1,
      sym_struct_body_decl,
    STATE(346), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10018] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(105), 1,
      anon_sym_LBRACE,
    STATE(242), 1,
      sym_compound_statement,
    STATE(347), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10035] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(893), 1,
      anon_sym_SEMI,
    ACTIONS(895), 1,
      anon_sym_if,
    STATE(348), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10052] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(897), 1,
      anon_sym_SEMI,
    ACTIONS(899), 1,
      anon_sym_EQ,
    STATE(349), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10069] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(105), 1,
      anon_sym_LBRACE,
    STATE(244), 1,
      sym_compound_statement,
    STATE(350), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10086] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(901), 1,
      anon_sym_SEMI,
    ACTIONS(903), 1,
      anon_sym_EQ,
    STATE(351), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10103] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(905), 1,
      anon_sym_f16,
    STATE(352), 1,
      sym__comment,
    STATE(421), 1,
      sym_extension_name,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10120] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(353), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(907), 2,
      anon_sym_COMMA,
      anon_sym_GT,
  [10135] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(909), 1,
      anon_sym_COMMA,
    ACTIONS(911), 1,
      anon_sym_GT,
    STATE(354), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10152] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    STATE(355), 1,
      sym__comment,
    STATE(423), 1,
      sym_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10169] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(913), 1,
      anon_sym_COMMA,
    ACTIONS(915), 1,
      anon_sym_GT,
    STATE(356), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10186] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(917), 1,
      anon_sym_LBRACE,
    STATE(357), 1,
      sym__comment,
    STATE(443), 1,
      sym_continuing_compound_statement,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10203] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(919), 1,
      anon_sym_SEMI,
    ACTIONS(921), 1,
      anon_sym_EQ,
    STATE(358), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10220] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(359), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(923), 2,
      anon_sym_SEMI,
      anon_sym_EQ,
  [10235] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(360), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(925), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [10250] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(105), 1,
      anon_sym_LBRACE,
    STATE(316), 1,
      sym_compound_statement,
    STATE(361), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10267] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(362), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(927), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [10282] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(363), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(929), 2,
      anon_sym_SEMI,
      anon_sym_RPAREN,
  [10297] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(105), 1,
      anon_sym_LBRACE,
    STATE(77), 1,
      sym_compound_statement,
    STATE(364), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10314] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(365), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(931), 2,
      anon_sym_COMMA,
      anon_sym_RBRACE,
  [10329] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    STATE(366), 1,
      sym__comment,
    STATE(453), 1,
      sym_ident,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10346] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(739), 1,
      sym_ident_pattern_token,
    STATE(346), 1,
      sym_ident,
    STATE(367), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10363] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(105), 1,
      anon_sym_LBRACE,
    STATE(84), 1,
      sym_compound_statement,
    STATE(368), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10380] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(369), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(933), 2,
      anon_sym_LPAREN,
      anon_sym_LT,
  [10395] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(935), 1,
      anon_sym_LPAREN,
    STATE(159), 1,
      sym_argument_expression_list,
    STATE(370), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10412] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(937), 1,
      anon_sym_COMMA,
    ACTIONS(939), 1,
      anon_sym_GT,
    STATE(371), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10429] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(45), 1,
      anon_sym_LPAREN,
    STATE(136), 1,
      sym_paren_expression,
    STATE(372), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10446] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(373), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(941), 2,
      anon_sym_LPAREN,
      anon_sym_LT,
  [10461] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(943), 1,
      anon_sym_LBRACE,
    ACTIONS(945), 1,
      anon_sym_DASH_GT,
    STATE(374), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10478] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(947), 1,
      anon_sym_LBRACE,
    ACTIONS(949), 1,
      anon_sym_DASH_GT,
    STATE(375), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10495] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(376), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(951), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [10510] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(377), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(849), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [10525] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(431), 1,
      anon_sym_LPAREN,
    ACTIONS(953), 1,
      anon_sym_LT,
    STATE(378), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10542] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(379), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(955), 2,
      anon_sym_SEMI,
      anon_sym_EQ,
  [10557] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(957), 1,
      anon_sym_SEMI,
    ACTIONS(959), 1,
      anon_sym_EQ,
    STATE(380), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10574] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(381), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(830), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [10589] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(382), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(961), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [10604] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(383), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(963), 2,
      anon_sym_COMMA,
      anon_sym_RPAREN,
  [10619] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(251), 1,
      anon_sym_LPAREN,
    STATE(201), 1,
      sym_paren_expression,
    STATE(384), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10636] = 5,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(965), 1,
      anon_sym_SEMI,
    ACTIONS(967), 1,
      anon_sym_EQ,
    STATE(385), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10653] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(386), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(969), 2,
      anon_sym_SEMI,
      anon_sym_RPAREN,
  [10668] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(387), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(971), 2,
      anon_sym_SEMI,
      anon_sym_RPAREN,
  [10683] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    STATE(388), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
    ACTIONS(973), 2,
      anon_sym_SEMI,
      anon_sym_EQ,
  [10698] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(305), 1,
      anon_sym_RPAREN,
    STATE(389), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10712] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(883), 1,
      anon_sym_LT,
    STATE(390), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10726] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(975), 1,
      anon_sym_RPAREN,
    STATE(391), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10740] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(953), 1,
      anon_sym_LT,
    STATE(392), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10754] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(977), 1,
      anon_sym_SEMI,
    STATE(393), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10768] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(979), 1,
      anon_sym_SEMI,
    STATE(394), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10782] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(981), 1,
      anon_sym_RBRACE,
    STATE(395), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10796] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(983), 1,
      anon_sym_COLON,
    STATE(396), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10810] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(985), 1,
      anon_sym_SEMI,
    STATE(397), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10824] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(893), 1,
      anon_sym_SEMI,
    STATE(398), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10838] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(987), 1,
      anon_sym_LPAREN,
    STATE(399), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10852] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(989), 1,
      anon_sym_LBRACE,
    STATE(400), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10866] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(629), 1,
      anon_sym_EQ,
    STATE(401), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10880] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(991), 1,
      anon_sym_COLON,
    STATE(402), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10894] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(993), 1,
      anon_sym_SEMI,
    STATE(403), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10908] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(995), 1,
      anon_sym_RBRACE,
    STATE(404), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10922] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(997), 1,
      anon_sym_RBRACE,
    STATE(405), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10936] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(999), 1,
      anon_sym_SEMI,
    STATE(406), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10950] = 5,
    ACTIONS(5), 1,
      sym__block_comment,
    ACTIONS(1001), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1003), 1,
      aux_sym__comment_token1,
    ACTIONS(1005), 1,
      sym__blankspace,
    STATE(407), 1,
      sym__comment,
  [10966] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1007), 1,
      sym_ident_pattern_token,
    STATE(408), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10980] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1009), 1,
      anon_sym_RPAREN,
    STATE(409), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [10994] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1011), 1,
      anon_sym_LBRACE,
    STATE(410), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11008] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1013), 1,
      anon_sym_GT,
    STATE(411), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11022] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1015), 1,
      anon_sym_GT,
    STATE(412), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11036] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1017), 1,
      anon_sym_SEMI,
    STATE(413), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11050] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1019), 1,
      anon_sym_COMMA,
    STATE(414), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11064] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1021), 1,
      anon_sym_GT,
    STATE(415), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11078] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1023), 1,
      anon_sym_RPAREN,
    STATE(416), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11092] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(299), 1,
      anon_sym_RPAREN,
    STATE(417), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11106] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1025), 1,
      anon_sym_GT,
    STATE(418), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11120] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1027), 1,
      anon_sym_COMMA,
    STATE(419), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11134] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(225), 1,
      anon_sym_RBRACE,
    STATE(420), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11148] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1029), 1,
      anon_sym_SEMI,
    STATE(421), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11162] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1031), 1,
      anon_sym_SEMI,
    STATE(422), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11176] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1033), 1,
      anon_sym_LPAREN,
    STATE(423), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11190] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1035), 1,
      anon_sym_SEMI,
    STATE(424), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11204] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1037), 1,
      anon_sym_RBRACE,
    STATE(425), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11218] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1039), 1,
      anon_sym_CARET,
    STATE(426), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11232] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1041), 1,
      anon_sym_AMP,
    STATE(427), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11246] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1043), 1,
      anon_sym_PIPE,
    STATE(428), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11260] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1045), 1,
      anon_sym_PIPE_PIPE,
    STATE(429), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11274] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1047), 1,
      anon_sym_AMP_AMP,
    STATE(430), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11288] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1049), 1,
      anon_sym_COMMA,
    STATE(431), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11302] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1051), 1,
      anon_sym_RBRACK,
    STATE(432), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11316] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1053), 1,
      anon_sym_LBRACE,
    STATE(433), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11330] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(431), 1,
      anon_sym_LPAREN,
    STATE(434), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11344] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1055), 1,
      anon_sym_LT,
    STATE(435), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11358] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1057), 1,
      anon_sym_RPAREN,
    STATE(436), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11372] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1059), 1,
      anon_sym_LT,
    STATE(437), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11386] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(877), 1,
      anon_sym_if,
    STATE(438), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11400] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(95), 1,
      anon_sym_SEMI,
    STATE(439), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11414] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1061), 1,
      anon_sym_RBRACE,
    STATE(440), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11428] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1063), 1,
      anon_sym_LT,
    STATE(441), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11442] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1065), 1,
      anon_sym_RBRACE,
    STATE(442), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11456] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1067), 1,
      anon_sym_RBRACE,
    STATE(443), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11470] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1069), 1,
      anon_sym_LT,
    STATE(444), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11484] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1071), 1,
      anon_sym_LT,
    STATE(445), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11498] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1073), 1,
      anon_sym_LT,
    STATE(446), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11512] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1075), 1,
      anon_sym_SEMI,
    STATE(447), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11526] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1077), 1,
      anon_sym_RPAREN,
    STATE(448), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11540] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1079), 1,
      anon_sym_COLON,
    STATE(449), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11554] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1081), 1,
      anon_sym_LBRACE,
    STATE(450), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11568] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1083), 1,
      anon_sym_RPAREN,
    STATE(451), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11582] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1085), 1,
      anon_sym_GT,
    STATE(452), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11596] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1087), 1,
      anon_sym_EQ,
    STATE(453), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11610] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1089), 1,
      anon_sym_EQ,
    STATE(454), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11624] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(810), 1,
      anon_sym_GT,
    STATE(455), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11638] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1091), 1,
      anon_sym_GT,
    STATE(456), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11652] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(959), 1,
      anon_sym_EQ,
    STATE(457), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11666] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1093), 1,
      anon_sym_PIPE,
    STATE(458), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11680] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1095), 1,
      anon_sym_CARET,
    STATE(459), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11694] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1097), 1,
      anon_sym_RPAREN,
    STATE(460), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11708] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1099), 1,
      anon_sym_LPAREN,
    STATE(461), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11722] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1101), 1,
      anon_sym_LPAREN,
    STATE(462), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11736] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1103), 1,
      anon_sym_LPAREN,
    STATE(463), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11750] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1105), 1,
      anon_sym_LPAREN,
    STATE(464), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11764] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1107), 1,
      anon_sym_LT,
    STATE(465), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11778] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1109), 1,
      sym_ident_pattern_token,
    STATE(466), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11792] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1111), 1,
      anon_sym_RBRACK,
    STATE(467), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11806] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1113), 1,
      anon_sym_RPAREN,
    STATE(468), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11820] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1115), 1,
      anon_sym_LBRACE,
    STATE(469), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11834] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1117), 1,
      anon_sym_SEMI,
    STATE(470), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11848] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1119), 1,
      anon_sym_SEMI,
    STATE(471), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11862] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1121), 1,
      anon_sym_RPAREN,
    STATE(472), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11876] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1123), 1,
      anon_sym_SEMI,
    STATE(473), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11890] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1125), 1,
      anon_sym_SEMI,
    STATE(474), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11904] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1127), 1,
      anon_sym_RBRACK,
    STATE(475), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11918] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1129), 1,
      anon_sym_RPAREN,
    STATE(476), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11932] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(91), 1,
      anon_sym_RPAREN,
    STATE(477), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11946] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1131), 1,
      anon_sym_SEMI,
    STATE(478), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11960] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1133), 1,
      anon_sym_RBRACK,
    STATE(479), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11974] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1135), 1,
      ts_builtin_sym_end,
    STATE(480), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [11988] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1137), 1,
      anon_sym_AMP,
    STATE(481), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [12002] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1139), 1,
      anon_sym_SEMI,
    STATE(482), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [12016] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1141), 1,
      anon_sym_GT,
    STATE(483), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [12030] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1143), 1,
      anon_sym_RPAREN,
    STATE(484), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [12044] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1145), 1,
      anon_sym_SEMI,
    STATE(485), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [12058] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(135), 1,
      anon_sym_RBRACE,
    STATE(486), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [12072] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1147), 1,
      anon_sym_GT,
    STATE(487), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [12086] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1149), 1,
      anon_sym_COLON,
    STATE(488), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [12100] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1151), 1,
      anon_sym_GT,
    STATE(489), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [12114] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1153), 1,
      anon_sym_GT,
    STATE(490), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [12128] = 4,
    ACTIONS(3), 1,
      anon_sym_SLASH_SLASH,
    ACTIONS(1155), 1,
      anon_sym_LT,
    STATE(491), 1,
      sym__comment,
    ACTIONS(5), 2,
      sym__block_comment,
      sym__blankspace,
  [12142] = 1,
    ACTIONS(1157), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(80)] = 0,
  [SMALL_STATE(81)] = 75,
  [SMALL_STATE(82)] = 150,
  [SMALL_STATE(83)] = 225,
  [SMALL_STATE(84)] = 300,
  [SMALL_STATE(85)] = 375,
  [SMALL_STATE(86)] = 450,
  [SMALL_STATE(87)] = 525,
  [SMALL_STATE(88)] = 600,
  [SMALL_STATE(89)] = 675,
  [SMALL_STATE(90)] = 750,
  [SMALL_STATE(91)] = 825,
  [SMALL_STATE(92)] = 900,
  [SMALL_STATE(93)] = 974,
  [SMALL_STATE(94)] = 1048,
  [SMALL_STATE(95)] = 1122,
  [SMALL_STATE(96)] = 1230,
  [SMALL_STATE(97)] = 1338,
  [SMALL_STATE(98)] = 1446,
  [SMALL_STATE(99)] = 1554,
  [SMALL_STATE(100)] = 1653,
  [SMALL_STATE(101)] = 1752,
  [SMALL_STATE(102)] = 1851,
  [SMALL_STATE(103)] = 1950,
  [SMALL_STATE(104)] = 2049,
  [SMALL_STATE(105)] = 2148,
  [SMALL_STATE(106)] = 2247,
  [SMALL_STATE(107)] = 2346,
  [SMALL_STATE(108)] = 2445,
  [SMALL_STATE(109)] = 2544,
  [SMALL_STATE(110)] = 2643,
  [SMALL_STATE(111)] = 2742,
  [SMALL_STATE(112)] = 2841,
  [SMALL_STATE(113)] = 2903,
  [SMALL_STATE(114)] = 2961,
  [SMALL_STATE(115)] = 3019,
  [SMALL_STATE(116)] = 3077,
  [SMALL_STATE(117)] = 3135,
  [SMALL_STATE(118)] = 3193,
  [SMALL_STATE(119)] = 3251,
  [SMALL_STATE(120)] = 3309,
  [SMALL_STATE(121)] = 3353,
  [SMALL_STATE(122)] = 3401,
  [SMALL_STATE(123)] = 3445,
  [SMALL_STATE(124)] = 3493,
  [SMALL_STATE(125)] = 3541,
  [SMALL_STATE(126)] = 3582,
  [SMALL_STATE(127)] = 3623,
  [SMALL_STATE(128)] = 3664,
  [SMALL_STATE(129)] = 3705,
  [SMALL_STATE(130)] = 3746,
  [SMALL_STATE(131)] = 3787,
  [SMALL_STATE(132)] = 3828,
  [SMALL_STATE(133)] = 3869,
  [SMALL_STATE(134)] = 3910,
  [SMALL_STATE(135)] = 3951,
  [SMALL_STATE(136)] = 3992,
  [SMALL_STATE(137)] = 4033,
  [SMALL_STATE(138)] = 4074,
  [SMALL_STATE(139)] = 4115,
  [SMALL_STATE(140)] = 4156,
  [SMALL_STATE(141)] = 4197,
  [SMALL_STATE(142)] = 4275,
  [SMALL_STATE(143)] = 4314,
  [SMALL_STATE(144)] = 4353,
  [SMALL_STATE(145)] = 4400,
  [SMALL_STATE(146)] = 4439,
  [SMALL_STATE(147)] = 4478,
  [SMALL_STATE(148)] = 4517,
  [SMALL_STATE(149)] = 4561,
  [SMALL_STATE(150)] = 4605,
  [SMALL_STATE(151)] = 4643,
  [SMALL_STATE(152)] = 4680,
  [SMALL_STATE(153)] = 4723,
  [SMALL_STATE(154)] = 4766,
  [SMALL_STATE(155)] = 4805,
  [SMALL_STATE(156)] = 4842,
  [SMALL_STATE(157)] = 4879,
  [SMALL_STATE(158)] = 4922,
  [SMALL_STATE(159)] = 4958,
  [SMALL_STATE(160)] = 4994,
  [SMALL_STATE(161)] = 5058,
  [SMALL_STATE(162)] = 5094,
  [SMALL_STATE(163)] = 5160,
  [SMALL_STATE(164)] = 5196,
  [SMALL_STATE(165)] = 5232,
  [SMALL_STATE(166)] = 5268,
  [SMALL_STATE(167)] = 5334,
  [SMALL_STATE(168)] = 5370,
  [SMALL_STATE(169)] = 5406,
  [SMALL_STATE(170)] = 5442,
  [SMALL_STATE(171)] = 5478,
  [SMALL_STATE(172)] = 5514,
  [SMALL_STATE(173)] = 5550,
  [SMALL_STATE(174)] = 5588,
  [SMALL_STATE(175)] = 5624,
  [SMALL_STATE(176)] = 5660,
  [SMALL_STATE(177)] = 5696,
  [SMALL_STATE(178)] = 5732,
  [SMALL_STATE(179)] = 5767,
  [SMALL_STATE(180)] = 5808,
  [SMALL_STATE(181)] = 5849,
  [SMALL_STATE(182)] = 5884,
  [SMALL_STATE(183)] = 5918,
  [SMALL_STATE(184)] = 5952,
  [SMALL_STATE(185)] = 5986,
  [SMALL_STATE(186)] = 6020,
  [SMALL_STATE(187)] = 6054,
  [SMALL_STATE(188)] = 6088,
  [SMALL_STATE(189)] = 6122,
  [SMALL_STATE(190)] = 6155,
  [SMALL_STATE(191)] = 6190,
  [SMALL_STATE(192)] = 6222,
  [SMALL_STATE(193)] = 6254,
  [SMALL_STATE(194)] = 6286,
  [SMALL_STATE(195)] = 6318,
  [SMALL_STATE(196)] = 6350,
  [SMALL_STATE(197)] = 6382,
  [SMALL_STATE(198)] = 6414,
  [SMALL_STATE(199)] = 6446,
  [SMALL_STATE(200)] = 6478,
  [SMALL_STATE(201)] = 6510,
  [SMALL_STATE(202)] = 6542,
  [SMALL_STATE(203)] = 6574,
  [SMALL_STATE(204)] = 6606,
  [SMALL_STATE(205)] = 6638,
  [SMALL_STATE(206)] = 6674,
  [SMALL_STATE(207)] = 6709,
  [SMALL_STATE(208)] = 6738,
  [SMALL_STATE(209)] = 6767,
  [SMALL_STATE(210)] = 6798,
  [SMALL_STATE(211)] = 6828,
  [SMALL_STATE(212)] = 6858,
  [SMALL_STATE(213)] = 6888,
  [SMALL_STATE(214)] = 6920,
  [SMALL_STATE(215)] = 6951,
  [SMALL_STATE(216)] = 6986,
  [SMALL_STATE(217)] = 7013,
  [SMALL_STATE(218)] = 7046,
  [SMALL_STATE(219)] = 7081,
  [SMALL_STATE(220)] = 7116,
  [SMALL_STATE(221)] = 7145,
  [SMALL_STATE(222)] = 7178,
  [SMALL_STATE(223)] = 7206,
  [SMALL_STATE(224)] = 7234,
  [SMALL_STATE(225)] = 7267,
  [SMALL_STATE(226)] = 7300,
  [SMALL_STATE(227)] = 7333,
  [SMALL_STATE(228)] = 7359,
  [SMALL_STATE(229)] = 7385,
  [SMALL_STATE(230)] = 7411,
  [SMALL_STATE(231)] = 7437,
  [SMALL_STATE(232)] = 7465,
  [SMALL_STATE(233)] = 7491,
  [SMALL_STATE(234)] = 7519,
  [SMALL_STATE(235)] = 7547,
  [SMALL_STATE(236)] = 7575,
  [SMALL_STATE(237)] = 7603,
  [SMALL_STATE(238)] = 7628,
  [SMALL_STATE(239)] = 7653,
  [SMALL_STATE(240)] = 7678,
  [SMALL_STATE(241)] = 7703,
  [SMALL_STATE(242)] = 7728,
  [SMALL_STATE(243)] = 7753,
  [SMALL_STATE(244)] = 7778,
  [SMALL_STATE(245)] = 7803,
  [SMALL_STATE(246)] = 7832,
  [SMALL_STATE(247)] = 7856,
  [SMALL_STATE(248)] = 7880,
  [SMALL_STATE(249)] = 7910,
  [SMALL_STATE(250)] = 7931,
  [SMALL_STATE(251)] = 7956,
  [SMALL_STATE(252)] = 7977,
  [SMALL_STATE(253)] = 7998,
  [SMALL_STATE(254)] = 8021,
  [SMALL_STATE(255)] = 8042,
  [SMALL_STATE(256)] = 8063,
  [SMALL_STATE(257)] = 8086,
  [SMALL_STATE(258)] = 8109,
  [SMALL_STATE(259)] = 8130,
  [SMALL_STATE(260)] = 8151,
  [SMALL_STATE(261)] = 8172,
  [SMALL_STATE(262)] = 8207,
  [SMALL_STATE(263)] = 8242,
  [SMALL_STATE(264)] = 8263,
  [SMALL_STATE(265)] = 8284,
  [SMALL_STATE(266)] = 8305,
  [SMALL_STATE(267)] = 8326,
  [SMALL_STATE(268)] = 8348,
  [SMALL_STATE(269)] = 8370,
  [SMALL_STATE(270)] = 8390,
  [SMALL_STATE(271)] = 8422,
  [SMALL_STATE(272)] = 8450,
  [SMALL_STATE(273)] = 8482,
  [SMALL_STATE(274)] = 8512,
  [SMALL_STATE(275)] = 8538,
  [SMALL_STATE(276)] = 8564,
  [SMALL_STATE(277)] = 8596,
  [SMALL_STATE(278)] = 8628,
  [SMALL_STATE(279)] = 8658,
  [SMALL_STATE(280)] = 8688,
  [SMALL_STATE(281)] = 8718,
  [SMALL_STATE(282)] = 8739,
  [SMALL_STATE(283)] = 8760,
  [SMALL_STATE(284)] = 8787,
  [SMALL_STATE(285)] = 8816,
  [SMALL_STATE(286)] = 8837,
  [SMALL_STATE(287)] = 8858,
  [SMALL_STATE(288)] = 8887,
  [SMALL_STATE(289)] = 8906,
  [SMALL_STATE(290)] = 8935,
  [SMALL_STATE(291)] = 8957,
  [SMALL_STATE(292)] = 8983,
  [SMALL_STATE(293)] = 9009,
  [SMALL_STATE(294)] = 9035,
  [SMALL_STATE(295)] = 9054,
  [SMALL_STATE(296)] = 9075,
  [SMALL_STATE(297)] = 9096,
  [SMALL_STATE(298)] = 9115,
  [SMALL_STATE(299)] = 9138,
  [SMALL_STATE(300)] = 9159,
  [SMALL_STATE(301)] = 9180,
  [SMALL_STATE(302)] = 9201,
  [SMALL_STATE(303)] = 9222,
  [SMALL_STATE(304)] = 9238,
  [SMALL_STATE(305)] = 9254,
  [SMALL_STATE(306)] = 9272,
  [SMALL_STATE(307)] = 9292,
  [SMALL_STATE(308)] = 9312,
  [SMALL_STATE(309)] = 9332,
  [SMALL_STATE(310)] = 9350,
  [SMALL_STATE(311)] = 9370,
  [SMALL_STATE(312)] = 9390,
  [SMALL_STATE(313)] = 9410,
  [SMALL_STATE(314)] = 9428,
  [SMALL_STATE(315)] = 9446,
  [SMALL_STATE(316)] = 9462,
  [SMALL_STATE(317)] = 9478,
  [SMALL_STATE(318)] = 9498,
  [SMALL_STATE(319)] = 9514,
  [SMALL_STATE(320)] = 9530,
  [SMALL_STATE(321)] = 9550,
  [SMALL_STATE(322)] = 9570,
  [SMALL_STATE(323)] = 9590,
  [SMALL_STATE(324)] = 9606,
  [SMALL_STATE(325)] = 9622,
  [SMALL_STATE(326)] = 9642,
  [SMALL_STATE(327)] = 9662,
  [SMALL_STATE(328)] = 9682,
  [SMALL_STATE(329)] = 9702,
  [SMALL_STATE(330)] = 9722,
  [SMALL_STATE(331)] = 9742,
  [SMALL_STATE(332)] = 9762,
  [SMALL_STATE(333)] = 9782,
  [SMALL_STATE(334)] = 9802,
  [SMALL_STATE(335)] = 9822,
  [SMALL_STATE(336)] = 9839,
  [SMALL_STATE(337)] = 9856,
  [SMALL_STATE(338)] = 9871,
  [SMALL_STATE(339)] = 9886,
  [SMALL_STATE(340)] = 9903,
  [SMALL_STATE(341)] = 9920,
  [SMALL_STATE(342)] = 9937,
  [SMALL_STATE(343)] = 9952,
  [SMALL_STATE(344)] = 9969,
  [SMALL_STATE(345)] = 9984,
  [SMALL_STATE(346)] = 10001,
  [SMALL_STATE(347)] = 10018,
  [SMALL_STATE(348)] = 10035,
  [SMALL_STATE(349)] = 10052,
  [SMALL_STATE(350)] = 10069,
  [SMALL_STATE(351)] = 10086,
  [SMALL_STATE(352)] = 10103,
  [SMALL_STATE(353)] = 10120,
  [SMALL_STATE(354)] = 10135,
  [SMALL_STATE(355)] = 10152,
  [SMALL_STATE(356)] = 10169,
  [SMALL_STATE(357)] = 10186,
  [SMALL_STATE(358)] = 10203,
  [SMALL_STATE(359)] = 10220,
  [SMALL_STATE(360)] = 10235,
  [SMALL_STATE(361)] = 10250,
  [SMALL_STATE(362)] = 10267,
  [SMALL_STATE(363)] = 10282,
  [SMALL_STATE(364)] = 10297,
  [SMALL_STATE(365)] = 10314,
  [SMALL_STATE(366)] = 10329,
  [SMALL_STATE(367)] = 10346,
  [SMALL_STATE(368)] = 10363,
  [SMALL_STATE(369)] = 10380,
  [SMALL_STATE(370)] = 10395,
  [SMALL_STATE(371)] = 10412,
  [SMALL_STATE(372)] = 10429,
  [SMALL_STATE(373)] = 10446,
  [SMALL_STATE(374)] = 10461,
  [SMALL_STATE(375)] = 10478,
  [SMALL_STATE(376)] = 10495,
  [SMALL_STATE(377)] = 10510,
  [SMALL_STATE(378)] = 10525,
  [SMALL_STATE(379)] = 10542,
  [SMALL_STATE(380)] = 10557,
  [SMALL_STATE(381)] = 10574,
  [SMALL_STATE(382)] = 10589,
  [SMALL_STATE(383)] = 10604,
  [SMALL_STATE(384)] = 10619,
  [SMALL_STATE(385)] = 10636,
  [SMALL_STATE(386)] = 10653,
  [SMALL_STATE(387)] = 10668,
  [SMALL_STATE(388)] = 10683,
  [SMALL_STATE(389)] = 10698,
  [SMALL_STATE(390)] = 10712,
  [SMALL_STATE(391)] = 10726,
  [SMALL_STATE(392)] = 10740,
  [SMALL_STATE(393)] = 10754,
  [SMALL_STATE(394)] = 10768,
  [SMALL_STATE(395)] = 10782,
  [SMALL_STATE(396)] = 10796,
  [SMALL_STATE(397)] = 10810,
  [SMALL_STATE(398)] = 10824,
  [SMALL_STATE(399)] = 10838,
  [SMALL_STATE(400)] = 10852,
  [SMALL_STATE(401)] = 10866,
  [SMALL_STATE(402)] = 10880,
  [SMALL_STATE(403)] = 10894,
  [SMALL_STATE(404)] = 10908,
  [SMALL_STATE(405)] = 10922,
  [SMALL_STATE(406)] = 10936,
  [SMALL_STATE(407)] = 10950,
  [SMALL_STATE(408)] = 10966,
  [SMALL_STATE(409)] = 10980,
  [SMALL_STATE(410)] = 10994,
  [SMALL_STATE(411)] = 11008,
  [SMALL_STATE(412)] = 11022,
  [SMALL_STATE(413)] = 11036,
  [SMALL_STATE(414)] = 11050,
  [SMALL_STATE(415)] = 11064,
  [SMALL_STATE(416)] = 11078,
  [SMALL_STATE(417)] = 11092,
  [SMALL_STATE(418)] = 11106,
  [SMALL_STATE(419)] = 11120,
  [SMALL_STATE(420)] = 11134,
  [SMALL_STATE(421)] = 11148,
  [SMALL_STATE(422)] = 11162,
  [SMALL_STATE(423)] = 11176,
  [SMALL_STATE(424)] = 11190,
  [SMALL_STATE(425)] = 11204,
  [SMALL_STATE(426)] = 11218,
  [SMALL_STATE(427)] = 11232,
  [SMALL_STATE(428)] = 11246,
  [SMALL_STATE(429)] = 11260,
  [SMALL_STATE(430)] = 11274,
  [SMALL_STATE(431)] = 11288,
  [SMALL_STATE(432)] = 11302,
  [SMALL_STATE(433)] = 11316,
  [SMALL_STATE(434)] = 11330,
  [SMALL_STATE(435)] = 11344,
  [SMALL_STATE(436)] = 11358,
  [SMALL_STATE(437)] = 11372,
  [SMALL_STATE(438)] = 11386,
  [SMALL_STATE(439)] = 11400,
  [SMALL_STATE(440)] = 11414,
  [SMALL_STATE(441)] = 11428,
  [SMALL_STATE(442)] = 11442,
  [SMALL_STATE(443)] = 11456,
  [SMALL_STATE(444)] = 11470,
  [SMALL_STATE(445)] = 11484,
  [SMALL_STATE(446)] = 11498,
  [SMALL_STATE(447)] = 11512,
  [SMALL_STATE(448)] = 11526,
  [SMALL_STATE(449)] = 11540,
  [SMALL_STATE(450)] = 11554,
  [SMALL_STATE(451)] = 11568,
  [SMALL_STATE(452)] = 11582,
  [SMALL_STATE(453)] = 11596,
  [SMALL_STATE(454)] = 11610,
  [SMALL_STATE(455)] = 11624,
  [SMALL_STATE(456)] = 11638,
  [SMALL_STATE(457)] = 11652,
  [SMALL_STATE(458)] = 11666,
  [SMALL_STATE(459)] = 11680,
  [SMALL_STATE(460)] = 11694,
  [SMALL_STATE(461)] = 11708,
  [SMALL_STATE(462)] = 11722,
  [SMALL_STATE(463)] = 11736,
  [SMALL_STATE(464)] = 11750,
  [SMALL_STATE(465)] = 11764,
  [SMALL_STATE(466)] = 11778,
  [SMALL_STATE(467)] = 11792,
  [SMALL_STATE(468)] = 11806,
  [SMALL_STATE(469)] = 11820,
  [SMALL_STATE(470)] = 11834,
  [SMALL_STATE(471)] = 11848,
  [SMALL_STATE(472)] = 11862,
  [SMALL_STATE(473)] = 11876,
  [SMALL_STATE(474)] = 11890,
  [SMALL_STATE(475)] = 11904,
  [SMALL_STATE(476)] = 11918,
  [SMALL_STATE(477)] = 11932,
  [SMALL_STATE(478)] = 11946,
  [SMALL_STATE(479)] = 11960,
  [SMALL_STATE(480)] = 11974,
  [SMALL_STATE(481)] = 11988,
  [SMALL_STATE(482)] = 12002,
  [SMALL_STATE(483)] = 12016,
  [SMALL_STATE(484)] = 12030,
  [SMALL_STATE(485)] = 12044,
  [SMALL_STATE(486)] = 12058,
  [SMALL_STATE(487)] = 12072,
  [SMALL_STATE(488)] = 12086,
  [SMALL_STATE(489)] = 12100,
  [SMALL_STATE(490)] = 12114,
  [SMALL_STATE(491)] = 12128,
  [SMALL_STATE(492)] = 12142,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(407),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [7] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_translation_unit, 0),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(241),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(215),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(321),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(367),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(366),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(291),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(330),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(355),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(352),
  [29] = {.entry = {.count = 1, .reusable = false}}, SHIFT(120),
  [31] = {.entry = {.count = 1, .reusable = false}}, SHIFT(134),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(137),
  [35] = {.entry = {.count = 1, .reusable = false}}, SHIFT(138),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(139),
  [39] = {.entry = {.count = 1, .reusable = false}}, SHIFT(139),
  [41] = {.entry = {.count = 1, .reusable = true}}, SHIFT(140),
  [43] = {.entry = {.count = 1, .reusable = false}}, SHIFT(140),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [47] = {.entry = {.count = 1, .reusable = false}}, SHIFT(378),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_case_selectors, 3),
  [51] = {.entry = {.count = 1, .reusable = false}}, SHIFT(252),
  [53] = {.entry = {.count = 1, .reusable = false}}, SHIFT(446),
  [55] = {.entry = {.count = 1, .reusable = false}}, SHIFT(445),
  [57] = {.entry = {.count = 1, .reusable = false}}, SHIFT(444),
  [59] = {.entry = {.count = 1, .reusable = false}}, SHIFT(259),
  [61] = {.entry = {.count = 1, .reusable = false}}, SHIFT(254),
  [63] = {.entry = {.count = 1, .reusable = false}}, SHIFT(441),
  [65] = {.entry = {.count = 1, .reusable = false}}, SHIFT(390),
  [67] = {.entry = {.count = 1, .reusable = false}}, SHIFT(373),
  [69] = {.entry = {.count = 1, .reusable = false}}, SHIFT(369),
  [71] = {.entry = {.count = 1, .reusable = false}}, SHIFT(437),
  [73] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [75] = {.entry = {.count = 1, .reusable = false}}, SHIFT(303),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_case_selectors, 2),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(193),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(172),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(130),
  [85] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression_comma_list, 2),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_return_statement, 1),
  [91] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [93] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression_comma_list, 3),
  [95] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [97] = {.entry = {.count = 1, .reusable = false}}, SHIFT(150),
  [99] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [101] = {.entry = {.count = 1, .reusable = true}}, SHIFT(279),
  [103] = {.entry = {.count = 1, .reusable = false}}, SHIFT(322),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [107] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [109] = {.entry = {.count = 1, .reusable = false}}, SHIFT(291),
  [111] = {.entry = {.count = 1, .reusable = true}}, SHIFT(273),
  [113] = {.entry = {.count = 1, .reusable = false}}, SHIFT(401),
  [115] = {.entry = {.count = 1, .reusable = false}}, SHIFT(40),
  [117] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [119] = {.entry = {.count = 1, .reusable = false}}, SHIFT(400),
  [121] = {.entry = {.count = 1, .reusable = false}}, SHIFT(399),
  [123] = {.entry = {.count = 1, .reusable = false}}, SHIFT(30),
  [125] = {.entry = {.count = 1, .reusable = false}}, SHIFT(398),
  [127] = {.entry = {.count = 1, .reusable = false}}, SHIFT(397),
  [129] = {.entry = {.count = 1, .reusable = false}}, SHIFT(357),
  [131] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [133] = {.entry = {.count = 1, .reusable = false}}, SHIFT(27),
  [135] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [137] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(150),
  [140] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(87),
  [143] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(279),
  [146] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(322),
  [149] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(378),
  [152] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(44),
  [155] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_compound_statement_repeat1, 2),
  [157] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(252),
  [160] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(446),
  [163] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(445),
  [166] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(444),
  [169] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(259),
  [172] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(254),
  [175] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(441),
  [178] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(390),
  [181] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(373),
  [184] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(369),
  [187] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(291),
  [190] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(273),
  [193] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(401),
  [196] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(40),
  [199] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(20),
  [202] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(400),
  [205] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(399),
  [208] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(30),
  [211] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(398),
  [214] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(397),
  [217] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2),
  [219] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(11),
  [222] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 2), SHIFT_REPEAT(27),
  [225] = {.entry = {.count = 1, .reusable = true}}, SHIFT(405),
  [227] = {.entry = {.count = 1, .reusable = false}}, SHIFT(348),
  [229] = {.entry = {.count = 1, .reusable = true}}, SHIFT(425),
  [231] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [233] = {.entry = {.count = 1, .reusable = true}}, SHIFT(70),
  [235] = {.entry = {.count = 1, .reusable = false}}, SHIFT(223),
  [237] = {.entry = {.count = 1, .reusable = false}}, SHIFT(194),
  [239] = {.entry = {.count = 1, .reusable = false}}, SHIFT(195),
  [241] = {.entry = {.count = 1, .reusable = false}}, SHIFT(192),
  [243] = {.entry = {.count = 1, .reusable = true}}, SHIFT(203),
  [245] = {.entry = {.count = 1, .reusable = false}}, SHIFT(203),
  [247] = {.entry = {.count = 1, .reusable = true}}, SHIFT(199),
  [249] = {.entry = {.count = 1, .reusable = false}}, SHIFT(199),
  [251] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [253] = {.entry = {.count = 1, .reusable = false}}, SHIFT(465),
  [255] = {.entry = {.count = 1, .reusable = true}}, SHIFT(64),
  [257] = {.entry = {.count = 1, .reusable = false}}, SHIFT(151),
  [259] = {.entry = {.count = 1, .reusable = false}}, SHIFT(171),
  [261] = {.entry = {.count = 1, .reusable = false}}, SHIFT(170),
  [263] = {.entry = {.count = 1, .reusable = false}}, SHIFT(169),
  [265] = {.entry = {.count = 1, .reusable = true}}, SHIFT(168),
  [267] = {.entry = {.count = 1, .reusable = false}}, SHIFT(168),
  [269] = {.entry = {.count = 1, .reusable = true}}, SHIFT(167),
  [271] = {.entry = {.count = 1, .reusable = false}}, SHIFT(167),
  [273] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [275] = {.entry = {.count = 1, .reusable = false}}, SHIFT(491),
  [277] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [279] = {.entry = {.count = 1, .reusable = true}}, SHIFT(65),
  [281] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [283] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_compound_statement, 2),
  [285] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_compound_statement, 2),
  [287] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_compound_statement, 3),
  [289] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_compound_statement, 3),
  [291] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_header, 2),
  [293] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 1),
  [295] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 1),
  [297] = {.entry = {.count = 1, .reusable = false}}, SHIFT(334),
  [299] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_header, 3),
  [301] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 2),
  [303] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 2),
  [305] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_header, 4),
  [307] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_if_statement_repeat1, 2),
  [309] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_if_statement_repeat1, 2),
  [311] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_if_statement_repeat1, 2), SHIFT_REPEAT(438),
  [314] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_clause, 3),
  [316] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_clause, 3),
  [318] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_if_statement_repeat1, 1),
  [320] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_if_statement_repeat1, 1),
  [322] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_else_if_clause, 4),
  [324] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_else_if_clause, 4),
  [326] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_else_clause, 2),
  [328] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_else_clause, 2),
  [330] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_switch_statement, 5),
  [332] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_switch_statement, 5),
  [334] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_loop_statement, 4),
  [336] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_loop_statement, 4),
  [338] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_for_statement, 5),
  [340] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_statement, 5),
  [342] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_compound_statement_repeat1, 1),
  [344] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_compound_statement_repeat1, 1),
  [346] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_loop_statement, 3),
  [348] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_loop_statement, 3),
  [350] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_statement, 1),
  [352] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 1),
  [354] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_while_statement, 3),
  [356] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_while_statement, 3),
  [358] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_statement, 2),
  [360] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 2),
  [362] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_loop_statement, 5),
  [364] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_loop_statement, 5),
  [366] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if_statement, 3),
  [368] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if_statement, 3),
  [370] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_additive_operator, 1),
  [372] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_additive_operator, 1),
  [374] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_multiplicative_operator, 1),
  [376] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_multiplicative_operator, 1),
  [378] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_compound_assignment_operator, 1),
  [380] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_compound_assignment_operator, 1),
  [382] = {.entry = {.count = 1, .reusable = false}}, SHIFT(392),
  [384] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_struct_member_repeat1, 2),
  [386] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_struct_member_repeat1, 2), SHIFT_REPEAT(215),
  [389] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attribute, 5),
  [391] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 5),
  [393] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attribute, 2),
  [395] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 2),
  [397] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attrib_end, 2),
  [399] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrib_end, 2),
  [401] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attrib_end, 1),
  [403] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attrib_end, 1),
  [405] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_struct_member_repeat1, 1),
  [407] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_struct_member_repeat1, 1),
  [409] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attribute, 7),
  [411] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 7),
  [413] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_attribute, 9),
  [415] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_attribute, 9),
  [417] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ident, 1),
  [419] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ident, 1),
  [421] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_singular_expression, 1),
  [423] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_singular_expression, 1),
  [425] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [427] = {.entry = {.count = 1, .reusable = true}}, SHIFT(233),
  [429] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_primary_expression, 1),
  [431] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_callable, 1),
  [433] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_primary_expression, 1),
  [435] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_component_or_swizzle_specifier, 2),
  [437] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_component_or_swizzle_specifier, 2),
  [439] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_component_or_swizzle_specifier, 3),
  [441] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_component_or_swizzle_specifier, 3),
  [443] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_swizzle_name, 1),
  [445] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_swizzle_name, 1),
  [447] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call_phrase, 2),
  [449] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call_phrase, 2),
  [451] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_member_ident, 1),
  [453] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_member_ident, 1),
  [455] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_expression_list, 3),
  [457] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_argument_expression_list, 3),
  [459] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_argument_expression_list, 2),
  [461] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_argument_expression_list, 2),
  [463] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_call_expression, 1),
  [465] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_call_expression, 1),
  [467] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_paren_expression, 3),
  [469] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_paren_expression, 3),
  [471] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_float_literal, 1),
  [473] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_float_literal, 1),
  [475] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bool_literal, 1),
  [477] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_bool_literal, 1),
  [479] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_literal, 1),
  [481] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_literal, 1),
  [483] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_primary_expression, 5),
  [485] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_primary_expression, 5),
  [487] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decimal_int_literal, 1),
  [489] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_decimal_int_literal, 1),
  [491] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_int_literal, 1),
  [493] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_int_literal, 1),
  [495] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decimal_float_literal, 1),
  [497] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_decimal_float_literal, 1),
  [499] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hex_float_literal, 1),
  [501] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_hex_float_literal, 1),
  [503] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_translation_unit, 1),
  [505] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_component_or_swizzle_specifier, 4),
  [507] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_component_or_swizzle_specifier, 4),
  [509] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_expression, 2),
  [511] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_expression, 2),
  [513] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_multiplicative_expression, 1),
  [515] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_multiplicative_expression, 1),
  [517] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_and_expression, 1),
  [519] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [521] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binary_or_expression, 1),
  [523] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_xor_expression, 1),
  [525] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unary_expression, 1),
  [527] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unary_expression, 1),
  [529] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_singular_expression, 2),
  [531] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_singular_expression, 2),
  [533] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [535] = {.entry = {.count = 1, .reusable = true}}, SHIFT(235),
  [537] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [539] = {.entry = {.count = 1, .reusable = true}}, SHIFT(236),
  [541] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat2, 2),
  [543] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat2, 2), SHIFT_REPEAT(241),
  [546] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat2, 2), SHIFT_REPEAT(215),
  [549] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat2, 2), SHIFT_REPEAT(321),
  [552] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat2, 2), SHIFT_REPEAT(367),
  [555] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat2, 2), SHIFT_REPEAT(366),
  [558] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat2, 2), SHIFT_REPEAT(291),
  [561] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat2, 2), SHIFT_REPEAT(330),
  [564] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat2, 2), SHIFT_REPEAT(27),
  [567] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat2, 2), SHIFT_REPEAT(355),
  [570] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_translation_unit, 2),
  [572] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_additive_expression, 1),
  [574] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_additive_expression, 1),
  [576] = {.entry = {.count = 1, .reusable = true}}, SHIFT(93),
  [578] = {.entry = {.count = 1, .reusable = false}}, SHIFT(93),
  [580] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_additive_expression, 3),
  [582] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_additive_expression, 3),
  [584] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_multiplicative_expression, 3),
  [586] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_multiplicative_expression, 3),
  [588] = {.entry = {.count = 1, .reusable = true}}, SHIFT(419),
  [590] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_shift_expression, 1),
  [592] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_shift_expression, 1),
  [594] = {.entry = {.count = 1, .reusable = true}}, SHIFT(92),
  [596] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_lhs_expression, 1),
  [598] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_core_lhs_expression, 1),
  [600] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_core_lhs_expression, 3),
  [602] = {.entry = {.count = 1, .reusable = true}}, SHIFT(58),
  [604] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_relational_expression, 1),
  [606] = {.entry = {.count = 1, .reusable = false}}, SHIFT(49),
  [608] = {.entry = {.count = 1, .reusable = true}}, SHIFT(49),
  [610] = {.entry = {.count = 1, .reusable = true}}, SHIFT(464),
  [612] = {.entry = {.count = 1, .reusable = true}}, SHIFT(463),
  [614] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [616] = {.entry = {.count = 1, .reusable = true}}, SHIFT(462),
  [618] = {.entry = {.count = 1, .reusable = true}}, SHIFT(461),
  [620] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_lhs_expression, 2),
  [622] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat1, 2),
  [624] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 2),
  [626] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat1, 2), SHIFT_REPEAT(352),
  [629] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [631] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [633] = {.entry = {.count = 1, .reusable = true}}, SHIFT(337),
  [635] = {.entry = {.count = 1, .reusable = true}}, SHIFT(338),
  [637] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_shift_expression, 3),
  [639] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_shift_expression, 3),
  [641] = {.entry = {.count = 1, .reusable = true}}, SHIFT(383),
  [643] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [645] = {.entry = {.count = 1, .reusable = true}}, SHIFT(231),
  [647] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_directive, 1),
  [649] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_global_directive, 1),
  [651] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_enable_directive, 3),
  [653] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_enable_directive, 3),
  [655] = {.entry = {.count = 1, .reusable = true}}, SHIFT(229),
  [657] = {.entry = {.count = 1, .reusable = true}}, SHIFT(230),
  [659] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat1, 1),
  [661] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat1, 1),
  [663] = {.entry = {.count = 1, .reusable = true}}, SHIFT(128),
  [665] = {.entry = {.count = 1, .reusable = true}}, SHIFT(125),
  [667] = {.entry = {.count = 1, .reusable = true}}, SHIFT(156),
  [669] = {.entry = {.count = 1, .reusable = true}}, SHIFT(155),
  [671] = {.entry = {.count = 1, .reusable = true}}, SHIFT(176),
  [673] = {.entry = {.count = 1, .reusable = true}}, SHIFT(177),
  [675] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_body_decl, 3),
  [677] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_struct_body_decl, 3),
  [679] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_decl, 3),
  [681] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_struct_decl, 3),
  [683] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_body_decl, 4),
  [685] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_struct_body_decl, 4),
  [687] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_body_decl, 5),
  [689] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_struct_body_decl, 5),
  [691] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_translation_unit_repeat2, 1),
  [693] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat2, 1),
  [695] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_decl, 2),
  [697] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_decl, 2),
  [699] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_translation_unit_repeat2, 2),
  [701] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_decl, 3),
  [703] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_function_decl, 3),
  [705] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_and_expression, 1),
  [707] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_or_expression, 1),
  [709] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_specifier_without_ident, 4),
  [711] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 1),
  [713] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_short_circuit_and_expression, 1),
  [715] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_short_circuit_or_expression, 1),
  [717] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_texture_and_sampler_types, 4),
  [719] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sampler_type, 1),
  [721] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bitwise_expression, 3),
  [723] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_and_expression, 3),
  [725] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_specifier_without_ident, 1),
  [727] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_specifier_without_ident, 8),
  [729] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_xor_expression, 3),
  [731] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binary_or_expression, 3),
  [733] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type_specifier, 4),
  [735] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_depth_texture_type, 1),
  [737] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_texture_and_sampler_types, 1),
  [739] = {.entry = {.count = 1, .reusable = true}}, SHIFT(120),
  [741] = {.entry = {.count = 1, .reusable = true}}, SHIFT(374),
  [743] = {.entry = {.count = 1, .reusable = true}}, SHIFT(333),
  [745] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_relational_expression, 3),
  [747] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_array_type_specifier, 6),
  [749] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_specifier_without_ident, 6),
  [751] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_texture_and_sampler_types, 6),
  [753] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 3),
  [755] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_short_circuit_or_expression, 3),
  [757] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_short_circuit_and_expression, 3),
  [759] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_specifier, 1),
  [761] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_param_list, 2),
  [763] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_switch_statement_repeat1, 2),
  [765] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_switch_statement_repeat1, 2), SHIFT_REPEAT(4),
  [768] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_switch_statement_repeat1, 2), SHIFT_REPEAT(317),
  [771] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_param_list, 3),
  [773] = {.entry = {.count = 1, .reusable = true}}, SHIFT(150),
  [775] = {.entry = {.count = 1, .reusable = true}}, SHIFT(240),
  [777] = {.entry = {.count = 1, .reusable = true}}, SHIFT(239),
  [779] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [781] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [783] = {.entry = {.count = 1, .reusable = true}}, SHIFT(317),
  [785] = {.entry = {.count = 1, .reusable = true}}, SHIFT(280),
  [787] = {.entry = {.count = 1, .reusable = true}}, SHIFT(353),
  [789] = {.entry = {.count = 1, .reusable = true}}, SHIFT(360),
  [791] = {.entry = {.count = 1, .reusable = true}}, SHIFT(281),
  [793] = {.entry = {.count = 1, .reusable = true}}, SHIFT(382),
  [795] = {.entry = {.count = 1, .reusable = false}}, SHIFT(489),
  [797] = {.entry = {.count = 1, .reusable = true}}, SHIFT(489),
  [799] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_case_selectors_repeat1, 2), SHIFT_REPEAT(6),
  [802] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_case_selectors_repeat1, 2),
  [804] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [806] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [808] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_case_selectors, 1),
  [810] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_element_count_expression, 1),
  [812] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_case_selector, 1),
  [814] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_default_alone_clause, 2),
  [816] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_struct_body_decl_repeat1, 2), SHIFT_REPEAT(284),
  [819] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_struct_body_decl_repeat1, 2),
  [821] = {.entry = {.count = 1, .reusable = true}}, SHIFT(290),
  [823] = {.entry = {.count = 1, .reusable = true}}, SHIFT(116),
  [825] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [827] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_expression_comma_list_repeat1, 2), SHIFT_REPEAT(29),
  [830] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_expression_comma_list_repeat1, 2),
  [832] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [834] = {.entry = {.count = 1, .reusable = true}}, SHIFT(277),
  [836] = {.entry = {.count = 1, .reusable = true}}, SHIFT(237),
  [838] = {.entry = {.count = 1, .reusable = true}}, SHIFT(270),
  [840] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_param_list, 1),
  [842] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_optionally_typed_ident, 1),
  [844] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [846] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_param_list_repeat1, 2), SHIFT_REPEAT(287),
  [849] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_param_list_repeat1, 2),
  [851] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_case_clause, 4),
  [853] = {.entry = {.count = 1, .reusable = true}}, SHIFT(345),
  [855] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_switch_statement_repeat1, 1),
  [857] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_switch_body, 1),
  [859] = {.entry = {.count = 1, .reusable = true}}, SHIFT(272),
  [861] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_default_alone_clause, 3),
  [863] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_case_clause, 3),
  [865] = {.entry = {.count = 1, .reusable = true}}, SHIFT(276),
  [867] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [869] = {.entry = {.count = 1, .reusable = true}}, SHIFT(477),
  [871] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [873] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression_comma_list, 1),
  [875] = {.entry = {.count = 1, .reusable = true}}, SHIFT(361),
  [877] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [879] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_increment_statement, 2),
  [881] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_decrement_statement, 2),
  [883] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [885] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [887] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [889] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_param, 4),
  [891] = {.entry = {.count = 1, .reusable = true}}, SHIFT(289),
  [893] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_break_statement, 1),
  [895] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [897] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_variable_decl, 2),
  [899] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [901] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_statement, 1),
  [903] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [905] = {.entry = {.count = 1, .reusable = true}}, SHIFT(422),
  [907] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_address_space, 1),
  [909] = {.entry = {.count = 1, .reusable = true}}, SHIFT(299),
  [911] = {.entry = {.count = 1, .reusable = true}}, SHIFT(408),
  [913] = {.entry = {.count = 1, .reusable = true}}, SHIFT(295),
  [915] = {.entry = {.count = 1, .reusable = true}}, SHIFT(265),
  [917] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [919] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_variable_decl, 1),
  [921] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [923] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_decl, 3),
  [925] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interpolation_sample_name, 1),
  [927] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_member, 4),
  [929] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_assignment_statement, 3),
  [931] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_struct_member, 3),
  [933] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_mat_prefix, 1),
  [935] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [937] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [939] = {.entry = {.count = 1, .reusable = true}}, SHIFT(258),
  [941] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_vec_prefix, 1),
  [943] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_header, 4),
  [945] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [947] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_header, 5),
  [949] = {.entry = {.count = 1, .reusable = true}}, SHIFT(97),
  [951] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_param, 3),
  [953] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [955] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_optionally_typed_ident, 3),
  [957] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_constant_decl, 2),
  [959] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [961] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_interpolation_type_name, 1),
  [963] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_builtin_value_name, 1),
  [965] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_constant_decl, 3),
  [967] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [969] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_updating_statement, 1),
  [971] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func_call_statement, 1),
  [973] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_decl, 2),
  [975] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_header, 5),
  [977] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_constant_decl, 4),
  [979] = {.entry = {.count = 1, .reusable = true}}, SHIFT(395),
  [981] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_break_if_statement, 4),
  [983] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [985] = {.entry = {.count = 1, .reusable = true}}, SHIFT(89),
  [987] = {.entry = {.count = 1, .reusable = true}}, SHIFT(68),
  [989] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [991] = {.entry = {.count = 1, .reusable = true}}, SHIFT(104),
  [993] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_alias_decl, 4),
  [995] = {.entry = {.count = 1, .reusable = true}}, SHIFT(440),
  [997] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_continuing_compound_statement, 3),
  [999] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_variable_decl, 3),
  [1001] = {.entry = {.count = 1, .reusable = false}}, SHIFT(407),
  [1003] = {.entry = {.count = 1, .reusable = false}}, SHIFT(492),
  [1005] = {.entry = {.count = 1, .reusable = false}}, SHIFT_EXTRA(),
  [1007] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_qualifier, 3),
  [1009] = {.entry = {.count = 1, .reusable = true}}, SHIFT(132),
  [1011] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_header, 8),
  [1013] = {.entry = {.count = 1, .reusable = true}}, SHIFT(255),
  [1015] = {.entry = {.count = 1, .reusable = true}}, SHIFT(251),
  [1017] = {.entry = {.count = 1, .reusable = true}}, SHIFT(75),
  [1019] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [1021] = {.entry = {.count = 1, .reusable = true}}, SHIFT(249),
  [1023] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_update, 1),
  [1025] = {.entry = {.count = 1, .reusable = true}}, SHIFT(372),
  [1027] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_texel_format, 1),
  [1029] = {.entry = {.count = 1, .reusable = true}}, SHIFT(228),
  [1031] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_extension_name, 1),
  [1033] = {.entry = {.count = 1, .reusable = true}}, SHIFT(261),
  [1035] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_const_assert_statement, 2),
  [1037] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_continuing_compound_statement, 2),
  [1039] = {.entry = {.count = 1, .reusable = true}}, SHIFT(54),
  [1041] = {.entry = {.count = 1, .reusable = true}}, SHIFT(61),
  [1043] = {.entry = {.count = 1, .reusable = true}}, SHIFT(62),
  [1045] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [1047] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [1049] = {.entry = {.count = 1, .reusable = true}}, SHIFT(296),
  [1051] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [1053] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_header, 7),
  [1055] = {.entry = {.count = 1, .reusable = true}}, SHIFT(189),
  [1057] = {.entry = {.count = 1, .reusable = true}}, SHIFT(129),
  [1059] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [1061] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_continuing_compound_statement, 4),
  [1063] = {.entry = {.count = 1, .reusable = true}}, SHIFT(282),
  [1065] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [1067] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_continuing_statement, 2),
  [1069] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_storage_texture_type, 1),
  [1071] = {.entry = {.count = 1, .reusable = true}}, SHIFT(109),
  [1073] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_sampled_texture_type, 1),
  [1075] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_statement, 4),
  [1077] = {.entry = {.count = 1, .reusable = true}}, SHIFT(375),
  [1079] = {.entry = {.count = 1, .reusable = true}}, SHIFT(100),
  [1081] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_function_header, 6),
  [1083] = {.entry = {.count = 1, .reusable = true}}, SHIFT(208),
  [1085] = {.entry = {.count = 1, .reusable = true}}, SHIFT(266),
  [1087] = {.entry = {.count = 1, .reusable = true}}, SHIFT(99),
  [1089] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [1091] = {.entry = {.count = 1, .reusable = true}}, SHIFT(264),
  [1093] = {.entry = {.count = 1, .reusable = true}}, SHIFT(63),
  [1095] = {.entry = {.count = 1, .reusable = true}}, SHIFT(60),
  [1097] = {.entry = {.count = 1, .reusable = true}}, SHIFT(163),
  [1099] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [1101] = {.entry = {.count = 1, .reusable = true}}, SHIFT(294),
  [1103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(222),
  [1105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [1107] = {.entry = {.count = 1, .reusable = true}}, SHIFT(102),
  [1109] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_qualifier, 5),
  [1111] = {.entry = {.count = 1, .reusable = true}}, SHIFT(148),
  [1113] = {.entry = {.count = 1, .reusable = true}}, SHIFT(175),
  [1115] = {.entry = {.count = 1, .reusable = true}}, SHIFT(283),
  [1117] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_return_statement, 2),
  [1119] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_constant_decl, 5),
  [1121] = {.entry = {.count = 1, .reusable = true}}, SHIFT(196),
  [1123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_global_variable_decl, 4),
  [1125] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_variable_statement, 3),
  [1127] = {.entry = {.count = 1, .reusable = true}}, SHIFT(152),
  [1129] = {.entry = {.count = 1, .reusable = true}}, SHIFT(200),
  [1131] = {.entry = {.count = 1, .reusable = true}}, SHIFT(243),
  [1133] = {.entry = {.count = 1, .reusable = true}}, SHIFT(226),
  [1135] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [1137] = {.entry = {.count = 1, .reusable = true}}, SHIFT(59),
  [1139] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [1141] = {.entry = {.count = 1, .reusable = true}}, SHIFT(335),
  [1143] = {.entry = {.count = 1, .reusable = true}}, SHIFT(368),
  [1145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_for_init, 1),
  [1147] = {.entry = {.count = 1, .reusable = true}}, SHIFT(384),
  [1149] = {.entry = {.count = 1, .reusable = true}}, SHIFT(106),
  [1151] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_access_mode, 1),
  [1153] = {.entry = {.count = 1, .reusable = true}}, SHIFT(466),
  [1155] = {.entry = {.count = 1, .reusable = true}}, SHIFT(103),
  [1157] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__comment, 2),
};

#ifdef __cplusplus
extern "C" {
#endif
void *tree_sitter_wgsl_external_scanner_create(void);
void tree_sitter_wgsl_external_scanner_destroy(void *);
bool tree_sitter_wgsl_external_scanner_scan(void *, TSLexer *, const bool *);
unsigned tree_sitter_wgsl_external_scanner_serialize(void *, char *);
void tree_sitter_wgsl_external_scanner_deserialize(void *, const char *, unsigned);

#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_wgsl(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_ident_pattern_token,
    .external_scanner = {
      &ts_external_scanner_states[0][0],
      ts_external_scanner_symbol_map,
      tree_sitter_wgsl_external_scanner_create,
      tree_sitter_wgsl_external_scanner_destroy,
      tree_sitter_wgsl_external_scanner_scan,
      tree_sitter_wgsl_external_scanner_serialize,
      tree_sitter_wgsl_external_scanner_deserialize,
    },
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
