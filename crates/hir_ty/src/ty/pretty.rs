use std::fmt::Write as _;

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
            let _ = write_type_expectation_inner(database, r#type, false, &mut str, verbosity);
        },
        TypeExpectation::TypeOrVecOf(inner) => {
            let _ = write_type_expectation_inner(database, inner, true, &mut str, verbosity);
        },
        TypeExpectation::None => unreachable!(),
    }
    str
}

fn write_type_expectation_inner(
    database: &dyn HirDatabase,
    inner: TypeExpectationInner,
    or_vec: bool,
    f: &mut String,
    verbosity: TypeVerbosity,
) -> std::fmt::Result {
    match inner {
        TypeExpectationInner::Exact(r#type) => {
            write_ty(database, r#type, f, verbosity)?;
            if or_vec {
                write!(f, " or vecN<")?;
                write_ty(database, r#type, f, verbosity)?;
                write!(f, ">")?;
            }
        },
        TypeExpectationInner::I32OrF32 => {
            write!(f, "i32 or f32")?;
        },
        TypeExpectationInner::NumericScalar => write!(f, "i32, u32, or f32")?,
        TypeExpectationInner::IntegerScalar => write!(f, "i32 or u32")?,
    }
    Ok(())
}

pub fn pretty_type(
    database: &dyn HirDatabase,
    r#type: Type,
) -> String {
    pretty_type_with_verbosity(database, r#type, TypeVerbosity::default())
}

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
    f: &mut String,
    verbosity: TypeVerbosity,
) -> std::fmt::Result {
    write!(f, "fn(")?;
    for (i, parameter) in function.parameters().enumerate() {
        if i != 0 {
            f.push_str(", ");
        }
        write_ty(database, parameter, f, verbosity)?;
    }
    write!(f, ")")?;
    if let Some(return_type) = function.return_type {
        f.push_str(" -> ");
        write_ty(database, return_type, f, verbosity)?;
    }
    Ok(())
}

fn write_ty(
    database: &dyn HirDatabase,
    r#type: Type,
    f: &mut String,
    verbosity: TypeVerbosity,
) -> std::fmt::Result {
    match r#type.kind(database) {
        TyKind::Error => write!(f, "[error]"),
        TyKind::Scalar(scalar) => {
            let s = match scalar {
                ScalarType::Bool => "bool",
                ScalarType::I32 => "i32",
                ScalarType::U32 => "u32",
                ScalarType::F32 => "f32",
            };
            write!(f, "{s}")
        },
        TyKind::Atomic(atomic) => {
            write!(f, "atomic<")?;
            write_ty(database, atomic.inner, f, verbosity)?;
            write!(f, ">")
        },
        TyKind::Vector(t) => {
            write!(f, "vec{}<", t.size)?;
            write_ty(database, t.inner, f, verbosity)?;
            write!(f, ">")
        },
        TyKind::Matrix(t) => {
            write!(f, "mat{}x{}<", t.columns, t.rows)?;
            write_ty(database, t.inner, f, verbosity)?;
            write!(f, ">")
        },
        TyKind::Struct(r#struct) => {
            let data = database.struct_data(r#struct);
            write!(f, "{}", data.name.as_str())
        },
        TyKind::Array(t) => {
            write!(f, "array<")?;
            write_ty(database, t.inner, f, verbosity)?;
            match t.size {
                ArraySize::Constant(value) => write!(f, ", {value}")?,
                ArraySize::Dynamic => {},
            }
            write!(f, ">")
        },
        TyKind::Texture(e) => {
            let value = match e.kind {
                TextureKind::Sampled(r#type) => format!(
                    "texture_{}{}{}<{}>",
                    if e.multisampled { "multisampled_" } else { "" },
                    e.dimension,
                    if e.arrayed { "_array" } else { "" },
                    pretty_type(database, r#type),
                ),
                TextureKind::Storage(format, mode) => format!(
                    "texture_storage_{}{}{}<{},{}>",
                    if e.multisampled { "multisampled_" } else { "" },
                    e.dimension,
                    if e.arrayed { "_array" } else { "" },
                    format,
                    mode,
                ),
                TextureKind::Depth => format!(
                    "texture_depth_{}{}{}",
                    if e.multisampled { "multisampled_" } else { "" },
                    e.dimension,
                    if e.arrayed { "_array" } else { "" },
                ),
                TextureKind::External => "texture_external".into(),
            };
            write!(f, "{value}")
        },
        TyKind::Sampler(sampler) => {
            if sampler.comparison {
                write!(f, "sampler_comparison")
            } else {
                write!(f, "sampler")
            }
        },
        TyKind::Reference(t) => match verbosity {
            TypeVerbosity::Full => {
                write!(f, "ref<{}, ", t.address_space)?;
                write_ty(database, t.inner, f, verbosity)?;
                write!(f, ", {}>", t.access_mode)
            },
            TypeVerbosity::Compact => {
                write!(f, "ref<")?;
                write_ty(database, t.inner, f, verbosity)?;
                write!(f, ">")
            },
            TypeVerbosity::Inner => write_ty(database, t.inner, f, verbosity),
        },
        TyKind::Pointer(t) => match verbosity {
            TypeVerbosity::Full => {
                write!(f, "ptr<{}, ", t.address_space)?;
                write_ty(database, t.inner, f, verbosity)?;
                write!(f, ", {}>", t.access_mode)
            },
            TypeVerbosity::Compact | TypeVerbosity::Inner => {
                write!(f, "ptr<")?;
                write_ty(database, t.inner, f, verbosity)?;
                write!(f, ">")
            },
        },
        TyKind::BoundVar(var) => {
            write!(f, "{}", ('T'..).nth(var.index).unwrap())
        },
        TyKind::StorageTypeOfTexelFormat(var) => {
            write!(f, "{}::StorageType", ('F'..).nth(var.index).unwrap())
        },
    }
}
