pub struct ExperimentalCondCompMode {
    pub unscope_compound_statements_create_indent: bool,
    pub dedent_condcomp_with_body: bool,
    pub dedent_condcomp_without_body: bool,
    pub condcomp_body_braces_on_same_line: bool,
}

/// These are awful settings because as condcomps with and without body are indented differently, this leads to
/// ```wesl
/// fn main() {
///     @if(true)
///     let a = x;
///     @elif(false)
///     let a = x;
/// @else {
///     let a = x;
/// }
/// }
/// ```
const AWFUL_SETTINGS: ExperimentalCondCompMode = ExperimentalCondCompMode {
    unscope_compound_statements_create_indent: false,
    dedent_condcomp_with_body: true,
    dedent_condcomp_without_body: false,
    condcomp_body_braces_on_same_line: true,
};

/// These are very basic and in my (Mona's) opinion most readable and beautiful.
/// The only weirdness with them is that we get
/// indented global declarations at points like
/// ```wesl
/// @group(2) @binding(1)
/// var aa: bb;
/// @if (true) {
///     struct AA {...}
///     @group(2) @binding(1)
///     var aa: AA;
/// } @else {
///     @group(2) @binding(1)
///     var aa: bb;
/// }
///
/// ```
const CONVENTIONAL_BORING_SETTINGS: ExperimentalCondCompMode = ExperimentalCondCompMode {
    unscope_compound_statements_create_indent: true,
    dedent_condcomp_with_body: false,
    dedent_condcomp_without_body: false,
    condcomp_body_braces_on_same_line: true,
};

/// Looks quite nice on its own, however once code gets a bit more
/// involved, and there are fors or ifs present, this gets very
/// hard to glance.
/// ```wesl
/// @elif(MULTISAMPLE)
/// {
/// let aaaaaaaaaa = aaaababeaf * vec2<f32>(falkjsekjf);
/// var bbbbbb = efekfjalekjflaskjflakjlkjalkjlkjslkfjljk;
/// let elfkjalkjlkj = alkjslejflsjflskjlskjelkjselfj;
/// for(var sample = 1; sample < sample_count; sample += 1) {
///     result = alkjalskjef(aljlksjef(alkejlskfjelklj), 1.0);
/// }
/// return result;
/// }
/// @else
/// {
/// return elfkjasl(aseflkjaslekj(malkj, asefkr, alskfjlske));
/// }
/// ```
/// See bevy's `downsample_dept.wesl`.
/// See bevy's `debug_overlay.wesl`.
const ORIGINAL_PROPOSAL_SETTINGS: ExperimentalCondCompMode = ExperimentalCondCompMode {
    unscope_compound_statements_create_indent: false,
    dedent_condcomp_with_body: false,
    dedent_condcomp_without_body: false,
    condcomp_body_braces_on_same_line: false,
};

/// I don't *hate* this, but it also destroys code's ability to be
/// glanced. Functions get broken up by dedented lines alot and it
/// stops being clear where a function begins and ends.
/// See bevy's `meshlet_mesh_material.wesl`.
/// See bevy's `pbr_fragment.wesl`.
/// See bevy's `pbr_prepass_functions.wesl`.
const DEDENTATION_OVERLOAD: ExperimentalCondCompMode = ExperimentalCondCompMode {
    unscope_compound_statements_create_indent: false,
    dedent_condcomp_with_body: true,
    dedent_condcomp_without_body: true,
    condcomp_body_braces_on_same_line: true,
};

pub const TEMP_EXPERIMENTAL_CONDCOMP_MODE: ExperimentalCondCompMode = CONVENTIONAL_BORING_SETTINGS;
