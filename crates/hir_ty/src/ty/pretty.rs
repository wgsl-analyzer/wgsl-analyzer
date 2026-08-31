use std::fmt::{self, Write as _};

use base_db::{CapabilitiesInput, TextRange, TextSize};
use hir_def::signature::StructSignature;
use itertools::Itertools as _;
use wgsl_types::{tplt::AccelerationStructureTags, ty::SamplerType};

use super::{Type, TypeKind};
use crate::{
    db::HirDatabase,
    function::FunctionDetails,
    infer::{TypeExpectation, TypeExpectationInner},
    ty::{ArraySize, ScalarType, TextureKind},
};

#[derive(Debug, Clone, Copy, Default)]
pub enum TypeVerbosity {
    Full, // ref<uniform, f32, read_write>,
    #[default]
    Compact, // ref<f32>,
    Inner, // f32
}

pub fn pretty_type_expectation(
    db: &dyn HirDatabase,
    r#type: TypeExpectation,
) -> String {
    pretty_type_expectation_with_verbosity(db, r#type, TypeVerbosity::default())
}

pub fn pretty_type_expectation_with_verbosity(
    db: &dyn HirDatabase,
    r#type: TypeExpectation,
    verbosity: TypeVerbosity,
) -> String {
    let mut str = String::new();

    match r#type {
        TypeExpectation::Type(r#type) => {
            _ = write_type_expectation_inner(db, r#type, false, &mut str, verbosity);
        },
        TypeExpectation::Any => _ = write!(&mut str, "any"),
    }
    str
}

fn write_type_expectation_inner(
    db: &dyn HirDatabase,
    inner: TypeExpectationInner,
    or_vec: bool,
    buffer: &mut String,
    verbosity: TypeVerbosity,
) -> fmt::Result {
    match inner {
        TypeExpectationInner::Exact(r#type) => {
            write_type(db, r#type, buffer, verbosity)?;
            if or_vec {
                write!(buffer, " or vecN<")?;
                write_type(db, r#type, buffer, verbosity)?;
                write!(buffer, ">")?;
            }
        },
        TypeExpectationInner::IntegerScalar => {
            write!(buffer, "i32 or u32")?;
            if CapabilitiesInput::get_capabilities(db).shader_int64 {
                write!(buffer, " or i64 or u64")?;
            }
        },
        TypeExpectationInner::IntegerIndex => {
            write!(buffer, "i32 or u32")?;
        },
    }
    Ok(())
}

pub fn pretty_type(
    db: &dyn HirDatabase,
    r#type: Type,
) -> String {
    pretty_type_with_verbosity(db, r#type, TypeVerbosity::default())
}

/// Pretty-print a type.
///
/// # Panics
///
/// Panics if writing to the internal buffer fails.
pub fn pretty_type_with_verbosity(
    db: &dyn HirDatabase,
    r#type: Type,
    verbosity: TypeVerbosity,
) -> String {
    let mut str = String::new();
    write_type(db, r#type, &mut str, verbosity).unwrap();
    str
}

pub fn pretty_fn(
    db: &dyn HirDatabase,
    function: &FunctionDetails,
) -> String {
    pretty_fn_with_verbosity(db, function, TypeVerbosity::default())
}

/// Pretty-print a function.
///
/// # Panics
///
/// Panics if writing into the internal buffer fails.
pub fn pretty_fn_with_verbosity(
    db: &dyn HirDatabase,
    function: &FunctionDetails,
    verbosity: TypeVerbosity,
) -> String {
    let mut str = String::new();
    pretty_fn_inner(db, function, &mut str, verbosity).unwrap();
    str
}

fn pretty_fn_inner(
    db: &dyn HirDatabase,
    function: &FunctionDetails,
    buffer: &mut String,
    verbosity: TypeVerbosity,
) -> fmt::Result {
    pretty_fn_inner_with_offsets(db, function, buffer, verbosity, None)
}

/// Pretty-print a function signature, optionally recording byte-offset
/// ranges for each parameter into `param_offsets`.
///
/// # Panics
///
/// Panics if writing into the internal buffer fails.
pub fn pretty_fn_inner_with_offsets(
    db: &dyn HirDatabase,
    function: &FunctionDetails,
    buffer: &mut String,
    verbosity: TypeVerbosity,
    mut param_offsets: Option<&mut Vec<TextRange>>,
) -> fmt::Result {
    write!(buffer, "fn {name}(", name = function.name.as_str())?;
    for (index, (param_type, param_name)) in function.parameters_with_names().enumerate() {
        if index != 0 {
            buffer.push_str(", ");
        }

        #[expect(
            clippy::cast_possible_truncation,
            clippy::as_conversions,
            reason = "buffer length will not exceed u32::MAX in practice"
        )]
        let start = buffer.len() as u32;
        if !param_name.is_empty() && !hir_def::item_tree::Name::is_missing(param_name) {
            write!(buffer, "{param_name}: ")?;
        }
        write_type(db, param_type, buffer, verbosity)?;

        if let Some(ref mut offsets) = param_offsets {
            #[expect(
                clippy::cast_possible_truncation,
                clippy::as_conversions,
                reason = "buffer length will not exceed u32::MAX in practice"
            )]
            let range = TextRange::new(TextSize::from(start), TextSize::from(buffer.len() as u32));
            offsets.push(range);
        }
    }
    write!(buffer, ")")?;
    if let Some(return_type) = function.return_type {
        buffer.push_str(" -> ");
        write_type(db, return_type, buffer, verbosity)?;
    }
    Ok(())
}

#[expect(clippy::too_many_lines, reason = "long but simple (recursive) match")]
fn write_type(
    db: &dyn HirDatabase,
    r#type: Type,
    formatter: &mut String,
    verbosity: TypeVerbosity,
) -> fmt::Result {
    match r#type.kind(db) {
        TypeKind::Error => write!(formatter, "[error]"),
        TypeKind::Scalar(ScalarType::Bool) => write!(formatter, "bool"),
        TypeKind::Scalar(ScalarType::AbstractInt) => write!(formatter, "integer"),
        TypeKind::Scalar(ScalarType::AbstractFloat) => write!(formatter, "float"),
        TypeKind::Scalar(ScalarType::I32) => write!(formatter, "i32"),
        TypeKind::Scalar(ScalarType::U32) => write!(formatter, "u32"),
        TypeKind::Scalar(ScalarType::I64) => write!(formatter, "i64"),
        TypeKind::Scalar(ScalarType::U64) => write!(formatter, "u64"),
        TypeKind::Scalar(ScalarType::F32) => write!(formatter, "f32"),
        TypeKind::Scalar(ScalarType::F16) => write!(formatter, "f16"),
        TypeKind::Atomic(atomic) => {
            write!(formatter, "atomic<")?;
            write_type(db, atomic.inner, formatter, verbosity)?;
            write!(formatter, ">")?;
            Ok(())
        },
        TypeKind::Vector(vector_type) => {
            write!(formatter, "vec{}<", vector_type.size)?;
            write_type(db, vector_type.component_type, formatter, verbosity)?;
            write!(formatter, ">")?;
            Ok(())
        },
        TypeKind::Matrix(matrix_type) => {
            write!(
                formatter,
                "mat{}x{}<",
                matrix_type.columns, matrix_type.rows
            )?;
            write_type(db, matrix_type.inner, formatter, verbosity)?;
            write!(formatter, ">")?;
            Ok(())
        },
        TypeKind::Struct(r#struct) => {
            let data = StructSignature::of(db, r#struct);
            write!(formatter, "{}", data.name.as_str())
        },
        TypeKind::BuiltinStruct(builtin_struct) => {
            write!(formatter, "{}", builtin_struct.name)
        },
        TypeKind::Array(array_type) => {
            if array_type.binding_array {
                write!(formatter, "binding_array<")?;
            } else {
                write!(formatter, "array<")?;
            }
            write_type(db, array_type.inner, formatter, verbosity)?;
            match array_type.size {
                ArraySize::Constant(value) => write!(formatter, ", {value}")?,
                ArraySize::Dynamic => {},
            }
            write!(formatter, ">")
        },
        TypeKind::Texture(texture_type) => {
            let value = match texture_type.kind {
                TextureKind::Sampled(r#type) => format!(
                    "texture_{}{}{}<{}>",
                    if texture_type.multisampled {
                        "multisampled_"
                    } else {
                        ""
                    },
                    texture_type.dimension,
                    if texture_type.arrayed { "_array" } else { "" },
                    pretty_type(db, r#type),
                ),
                TextureKind::Storage(format, mode) => format!(
                    "texture_storage_{}{}{}<{format},{mode}>",
                    if texture_type.multisampled {
                        "multisampled_"
                    } else {
                        ""
                    },
                    texture_type.dimension,
                    if texture_type.arrayed { "_array" } else { "" },
                ),
                TextureKind::Depth => format!(
                    "texture_depth_{}{}{}",
                    if texture_type.multisampled {
                        "multisampled_"
                    } else {
                        ""
                    },
                    texture_type.dimension,
                    if texture_type.arrayed { "_array" } else { "" },
                ),
                TextureKind::External => "texture_external".into(),
            };
            write!(formatter, "{value}")?;
            Ok(())
        },
        TypeKind::Sampler(SamplerType::Sampler) => {
            write!(formatter, "sampler")?;
            Ok(())
        },
        TypeKind::Sampler(SamplerType::SamplerComparison) => {
            write!(formatter, "sampler_comparison")?;
            Ok(())
        },
        TypeKind::Reference(reference) => match verbosity {
            TypeVerbosity::Full => {
                write!(formatter, "ref<{}, ", reference.address_space)?;
                write_type(db, reference.inner, formatter, verbosity)?;
                write!(formatter, ", {}>", reference.access_mode)?;
                Ok(())
            },
            TypeVerbosity::Compact => {
                write!(formatter, "ref<")?;
                write_type(db, reference.inner, formatter, verbosity)?;
                write!(formatter, ">")
            },
            TypeVerbosity::Inner => write_type(db, reference.inner, formatter, verbosity),
        },
        TypeKind::Pointer(pointer) => match verbosity {
            TypeVerbosity::Full => {
                write!(formatter, "ptr<{}, ", pointer.address_space)?;
                write_type(db, pointer.inner, formatter, verbosity)?;
                write!(formatter, ", {}>", pointer.access_mode)
            },
            TypeVerbosity::Compact | TypeVerbosity::Inner => {
                write!(formatter, "ptr<")?;
                write_type(db, pointer.inner, formatter, verbosity)?;
                write!(formatter, ">")
            },
        },
        TypeKind::AccelerationStructure(tags) => {
            write!(formatter, "acceleration_structure")?;
            if let Some(tags) = tags {
                write!(formatter, "<{}>", display_tags(&tags))?;
            }
            Ok(())
        },
        TypeKind::RayQuery(tags) => {
            write!(formatter, "ray_query")?;
            if let Some(tags) = tags {
                write!(formatter, "<{}>", display_tags(&tags))?;
            }
            Ok(())
        },
    }
}

fn display_tags(tags: &AccelerationStructureTags) -> String {
    tags.tags()
        .iter()
        .map(|tag| match tag {
            wgsl_types::syntax::AccelerationStructureTag::VertexReturn => "vertex_return",
        })
        .join(", ")
}
