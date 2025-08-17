use std::fmt::{self, Write as _};

use super::{TyKind, Type};
use crate::{
    database::HirDatabase,
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
    database: &dyn HirDatabase,
    r#type: TypeExpectation,
) -> String {
    pretty_type_expectation_with_verbosity(database, r#type, TypeVerbosity::default())
}

pub fn pretty_type_expectation_with_verbosity(
    database: &dyn HirDatabase,
    r#type: TypeExpectation,
    verbosity: TypeVerbosity,
) -> String {
    let mut str = String::new();

    match r#type {
        TypeExpectation::Type(r#type) => {
            _ = write_type_expectation_inner(database, &r#type, false, &mut str, verbosity);
        },
        TypeExpectation::TypeOrVecOf(inner) => {
            _ = write_type_expectation_inner(database, &inner, true, &mut str, verbosity);
        },
        #[expect(clippy::unreachable, reason = "TODO")]
        TypeExpectation::None => unreachable!(),
    }
    str
}

fn write_type_expectation_inner(
    database: &dyn HirDatabase,
    inner: &TypeExpectationInner,
    or_vec: bool,
    buffer: &mut String,
    verbosity: TypeVerbosity,
) -> fmt::Result {
    match inner {
        TypeExpectationInner::Exact(r#type) => {
            write_ty(database, *r#type, buffer, verbosity)?;
            if or_vec {
                write!(buffer, " or vecN<")?;
                write_ty(database, *r#type, buffer, verbosity)?;
                write!(buffer, ">")?;
            }
        },
        TypeExpectationInner::I32OrF32 => {
            write!(buffer, "i32 or f32")?;
        },
        TypeExpectationInner::NumericScalar => write!(buffer, "i32, u32, or f32")?,
        TypeExpectationInner::IntegerScalar => write!(buffer, "i32 or u32")?,
    }
    Ok(())
}

pub fn pretty_type(
    database: &dyn HirDatabase,
    r#type: Type,
) -> String {
    pretty_type_with_verbosity(database, r#type, TypeVerbosity::default())
}

/// Pretty-print a type.
///
/// # Panics
///
/// Panics if writing to the internal buffer fails.
pub fn pretty_type_with_verbosity(
    database: &dyn HirDatabase,
    r#type: Type,
    verbosity: TypeVerbosity,
) -> String {
    let mut str = String::new();
    write_ty(database, r#type, &mut str, verbosity).unwrap();
    str
}

pub fn pretty_fn(
    database: &dyn HirDatabase,
    function: &FunctionDetails,
) -> String {
    pretty_fn_with_verbosity(database, function, TypeVerbosity::default())
}

/// Pretty-print a function.
///
/// # Panics
///
/// Panics if writing into the internal buffer fails.
pub fn pretty_fn_with_verbosity(
    database: &dyn HirDatabase,
    function: &FunctionDetails,
    verbosity: TypeVerbosity,
) -> String {
    let mut str = String::new();
    pretty_fn_inner(database, function, &mut str, verbosity).unwrap();
    str
}

fn pretty_fn_inner(
    database: &dyn HirDatabase,
    function: &FunctionDetails,
    buffer: &mut String,
    verbosity: TypeVerbosity,
) -> fmt::Result {
    write!(buffer, "fn(")?;
    for (index, parameter) in function.parameters().enumerate() {
        if index != 0 {
            buffer.push_str(", ");
        }
        write_ty(database, parameter, buffer, verbosity)?;
    }
    write!(buffer, ")")?;
    if let Some(return_type) = function.return_type {
        buffer.push_str(" -> ");
        write_ty(database, return_type, buffer, verbosity)?;
    }
    Ok(())
}

#[expect(clippy::too_many_lines, reason = "TODO")]
fn write_ty(
    database: &dyn HirDatabase,
    r#type: Type,
    formatter: &mut String,
    verbosity: TypeVerbosity,
) -> fmt::Result {
    match r#type.kind(database) {
        TyKind::Error => write!(formatter, "[error]"),
        TyKind::Scalar(scalar) => {
            let string = match scalar {
                ScalarType::Bool => "bool",
                // TODO: Is this reachable?
                ScalarType::AbstractInt => "integer",
                ScalarType::AbstractFloat => "float",
                ScalarType::I32 => "i32",
                ScalarType::U32 => "u32",
                ScalarType::F32 => "f32",
                ScalarType::F16 => "f16",
            };
            write!(formatter, "{string}")
        },
        TyKind::Atomic(atomic) => {
            write!(formatter, "atomic<")?;
            write_ty(database, atomic.inner, formatter, verbosity)?;
            write!(formatter, ">")
        },
        TyKind::Vector(vector_type) => {
            write!(formatter, "vec{}<", vector_type.size)?;
            write_ty(database, vector_type.component_type, formatter, verbosity)?;
            write!(formatter, ">")
        },
        TyKind::Matrix(matrix_type) => {
            write!(
                formatter,
                "mat{}x{}<",
                matrix_type.columns, matrix_type.rows
            )?;
            write_ty(database, matrix_type.inner, formatter, verbosity)?;
            write!(formatter, ">")
        },
        TyKind::Struct(r#struct) => {
            let data = database.struct_data(r#struct).0;
            write!(formatter, "{}", data.name.as_str())
        },
        TyKind::Array(array_type) => {
            write!(formatter, "array<")?;
            write_ty(database, array_type.inner, formatter, verbosity)?;
            match array_type.size {
                ArraySize::Constant(value) => write!(formatter, ", {value}")?,
                ArraySize::Dynamic => {},
            }
            write!(formatter, ">")
        },
        TyKind::Texture(texture_type) => {
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
                    pretty_type(database, r#type),
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
            write!(formatter, "{value}")
        },
        TyKind::Sampler(sampler) => {
            if sampler.comparison {
                write!(formatter, "sampler_comparison")
            } else {
                write!(formatter, "sampler")
            }
        },
        TyKind::Reference(reference) => match verbosity {
            TypeVerbosity::Full => {
                write!(formatter, "ref<{}, ", reference.address_space)?;
                write_ty(database, reference.inner, formatter, verbosity)?;
                write!(formatter, ", {}>", reference.access_mode)
            },
            TypeVerbosity::Compact => {
                write!(formatter, "ref<")?;
                write_ty(database, reference.inner, formatter, verbosity)?;
                write!(formatter, ">")
            },
            TypeVerbosity::Inner => write_ty(database, reference.inner, formatter, verbosity),
        },
        TyKind::Pointer(pointer) => match verbosity {
            TypeVerbosity::Full => {
                write!(formatter, "ptr<{}, ", pointer.address_space)?;
                write_ty(database, pointer.inner, formatter, verbosity)?;
                write!(formatter, ", {}>", pointer.access_mode)
            },
            TypeVerbosity::Compact | TypeVerbosity::Inner => {
                write!(formatter, "ptr<")?;
                write_ty(database, pointer.inner, formatter, verbosity)?;
                write!(formatter, ">")
            },
        },
        TyKind::BoundVar(var) => {
            write!(formatter, "{}", ('T'..).nth(var.index).unwrap())
        },
        TyKind::StorageTypeOfTexelFormat(var) => {
            write!(
                formatter,
                "{}::StorageType",
                ('F'..).nth(var.index).unwrap()
            )
        },
    }
}
