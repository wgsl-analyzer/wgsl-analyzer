//! WGSL attribute definitions.
//!
//! Single source of truth for attribute names, syntax, descriptions, and spec links.
//! Used by hover, completions, and any other IDE feature that needs attribute metadata.
//!
//! See <https://www.w3.org/TR/WGSL/#attributes>.

/// Metadata for a single WGSL attribute.
pub struct WgslAttribute {
    /// The attribute name (without `@`), e.g. `"group"`.
    pub name: &'static str,
    /// The syntax including `@`, e.g. `"@group(expression)"`.
    pub syntax: &'static str,
    /// A human-readable description of what the attribute does.
    pub description: &'static str,
    /// The anchor in the WGSL spec, e.g. `"group-attr"`.
    pub spec_anchor: &'static str,
}

impl WgslAttribute {
    /// Returns the full URL to the WGSL spec section for this attribute.
    #[must_use]
    pub fn spec_url(&self) -> String {
        format!("https://www.w3.org/TR/WGSL/#{}", self.spec_anchor)
    }
}

/// All WGSL attributes defined in the spec.
pub static WGSL_ATTRIBUTES: &[WgslAttribute] = &[
    WgslAttribute {
        name: "align",
        syntax: "@align(expression)",
        description: "Specifies the byte alignment of a struct member. Must be a power of 2.",
        spec_anchor: "align-attr",
    },
    WgslAttribute {
        name: "binding",
        syntax: "@binding(expression)",
        description: "Specifies the binding number of a resource variable in a bind group. Used together with `@group`.",
        spec_anchor: "binding-attr",
    },
    WgslAttribute {
        name: "builtin",
        syntax: "@builtin(builtin_value)",
        description: "Specifies that a function parameter or struct member corresponds to a built-in value (e.g., `position`, `vertex_index`, `front_facing`).",
        spec_anchor: "builtin-attr",
    },
    WgslAttribute {
        name: "compute",
        syntax: "@compute",
        description: "Declares a function as a compute shader entry point.",
        spec_anchor: "compute-attr",
    },
    WgslAttribute {
        name: "const",
        syntax: "@const",
        description: "Declares a function as a const-expression function, meaning it can be evaluated at shader creation time.",
        spec_anchor: "const-attr",
    },
    WgslAttribute {
        name: "diagnostic",
        syntax: "@diagnostic(severity, rule)",
        description: "Controls the severity of a diagnostic rule. Severity can be `error`, `warning`, `info`, or `off`.",
        spec_anchor: "diagnostic-attr",
    },
    WgslAttribute {
        name: "fragment",
        syntax: "@fragment",
        description: "Declares a function as a fragment shader entry point.",
        spec_anchor: "fragment-attr",
    },
    WgslAttribute {
        name: "group",
        syntax: "@group(expression)",
        description: "Specifies the bind group index of a resource variable. Used together with `@binding`.",
        spec_anchor: "group-attr",
    },
    WgslAttribute {
        name: "id",
        syntax: "@id(expression)",
        description: "Specifies a numeric identifier for an `override` declaration, used for pipeline-overridable constants.",
        spec_anchor: "id-attr",
    },
    WgslAttribute {
        name: "interpolate",
        syntax: "@interpolate(type, sampling)",
        description: "Specifies how user-defined IO is interpolated. Type can be `perspective`, `linear`, or `flat`. Sampling can be `center`, `centroid`, or `sample`.",
        spec_anchor: "interpolate-attr",
    },
    WgslAttribute {
        name: "invariant",
        syntax: "@invariant",
        description: "Indicates that the value of a `@builtin(position)` output must be invariant across different shader invocations with the same input.",
        spec_anchor: "invariant-attr",
    },
    WgslAttribute {
        name: "location",
        syntax: "@location(expression)",
        description: "Specifies the location number for user-defined IO (inter-stage variables between vertex and fragment shaders).",
        spec_anchor: "location-attr",
    },
    WgslAttribute {
        name: "must_use",
        syntax: "@must_use",
        description: "Indicates that the return value of a function must be used by the caller.",
        spec_anchor: "must-use-attr",
    },
    WgslAttribute {
        name: "size",
        syntax: "@size(expression)",
        description: "Specifies the byte size of a struct member, including any trailing padding.",
        spec_anchor: "size-attr",
    },
    WgslAttribute {
        name: "vertex",
        syntax: "@vertex",
        description: "Declares a function as a vertex shader entry point.",
        spec_anchor: "vertex-attr",
    },
    WgslAttribute {
        name: "workgroup_size",
        syntax: "@workgroup_size(x, y, z)",
        description: "Specifies the workgroup dimensions for a compute shader. `y` and `z` default to 1 if omitted.",
        spec_anchor: "workgroup-size-attr",
    },
];

/// Look up a WGSL attribute by name.
#[must_use]
pub fn find_attribute(name: &str) -> Option<&'static WgslAttribute> {
    WGSL_ATTRIBUTES.iter().find(|attr| attr.name == name)
}

