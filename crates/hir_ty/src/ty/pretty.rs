use super::{Ty, TyKind};
use crate::{
    function::FunctionDetails,
    infer::{TypeExpectation, TypeExpectationInner},
    ty::{ArraySize, ScalarType, TextureKind},
    HirDatabase,
};
use std::fmt::Write;

#[derive(Debug, Clone, Copy)]
#[derive(Default)]
pub enum TypeVerbosity {
    Full,    // ref<uniform, f32, read_write>,
    #[default]
    Compact, // ref<f32>,
    Inner,   // f32
}


pub fn pretty_type_expectation(db: &dyn HirDatabase, ty: TypeExpectation) -> String {
    pretty_type_expectation_with_verbosity(db, ty, TypeVerbosity::default())
}

pub fn pretty_type_expectation_with_verbosity(
    db: &dyn HirDatabase,
    ty: TypeExpectation,
    verbosity: TypeVerbosity,
) -> String {
    let mut str = String::new();

    match ty {
        TypeExpectation::Type(ty) => {
            let _ = write_type_expectation_inner(db, ty, false, &mut str, verbosity);
        }
        TypeExpectation::TypeOrVecOf(inner) => {
            let _ = write_type_expectation_inner(db, inner, true, &mut str, verbosity);
        }
        TypeExpectation::None => unreachable!(),
    }
    str
}

fn write_type_expectation_inner(
    db: &dyn HirDatabase,
    inner: TypeExpectationInner,
    or_vec: bool,
    f: &mut String,
    verbosity: TypeVerbosity,
) -> std::fmt::Result {
    match inner {
        TypeExpectationInner::Exact(ty) => {
            write_ty(db, ty, f, verbosity)?;
            if or_vec {
                write!(f, " or vecN<")?;
                write_ty(db, ty, f, verbosity)?;
                write!(f, ">")?;
            }
        }
        TypeExpectationInner::I32OrF32 => {
            write!(f, "i32 or f32")?;
        }
        TypeExpectationInner::NumericScalar => write!(f, "i32, u32 or f32")?,
        TypeExpectationInner::IntegerScalar => write!(f, "i32 or u32")?,
    }
    Ok(())
}

pub fn pretty_type(db: &dyn HirDatabase, ty: Ty) -> String {
    pretty_type_with_verbosity(db, ty, TypeVerbosity::default())
}

pub fn pretty_type_with_verbosity(
    db: &dyn HirDatabase,
    ty: Ty,
    verbosity: TypeVerbosity,
) -> String {
    let mut str = String::new();
    write_ty(db, ty, &mut str, verbosity).unwrap();
    str
}

pub fn pretty_fn(db: &dyn HirDatabase, function: &FunctionDetails) -> String {
    pretty_fn_with_verbosity(db, function, TypeVerbosity::default())
}
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
    f: &mut String,
    verbosity: TypeVerbosity,
) -> std::fmt::Result {
    write!(f, "fn(")?;
    for (i, param) in function.parameters().enumerate() {
        if i != 0 {
            f.push_str(", ");
        }
        write_ty(db, param, f, verbosity)?;
    }
    write!(f, ")")?;
    if let Some(ret) = function.return_type {
        f.push_str(" -> ");
        write_ty(db, ret, f, verbosity)?;
    }
    Ok(())
}

fn write_ty(
    db: &dyn HirDatabase,
    ty: Ty,
    f: &mut String,
    verbosity: TypeVerbosity,
) -> std::fmt::Result {
    match ty.kind(db) {
        TyKind::Error => write!(f, "[error]"),
        TyKind::Scalar(scalar) => {
            let s = match scalar {
                ScalarType::Bool => "bool",
                ScalarType::I32 => "i32",
                ScalarType::U32 => "u32",
                ScalarType::F32 => "f32",
            };
            write!(f, "{}", s)
        }
        TyKind::Atomic(atomic) => {
            write!(f, "atomic<")?;
            write_ty(db, atomic.inner, f, verbosity)?;
            write!(f, ">")
        }
        TyKind::Vector(t) => {
            write!(f, "vec{}<", t.size)?;
            write_ty(db, t.inner, f, verbosity)?;
            write!(f, ">")
        }
        TyKind::Matrix(t) => {
            write!(f, "mat{}x{}<", t.columns, t.rows)?;
            write_ty(db, t.inner, f, verbosity)?;
            write!(f, ">")
        }
        TyKind::Struct(strukt) => {
            let data = db.struct_data(strukt);
            write!(f, "{}", data.name.as_str())
        }
        TyKind::Array(t) => {
            write!(f, "array<")?;
            write_ty(db, t.inner, f, verbosity)?;
            match t.size {
                ArraySize::Const(val) => write!(f, ", {}", val)?,
                ArraySize::Dynamic => {}
            }
            write!(f, ">")
        }
        TyKind::Texture(e) => {
            let val = match e.kind {
                TextureKind::Sampled(ty) => format!(
                    "texture_{}{}{}<{}>",
                    if e.multisampled { "multisampled_" } else { "" },
                    e.dimension,
                    if e.arrayed { "_array" } else { "" },
                    pretty_type(db, ty),
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
            write!(f, "{}", val)
        }
        TyKind::Sampler(sampler) => match sampler.comparison {
            true => write!(f, "sampler_comparison"),
            false => write!(f, "sampler"),
        },
        TyKind::Ref(t) => match verbosity {
            TypeVerbosity::Full => {
                write!(f, "ref<{}, ", t.storage_class)?;
                write_ty(db, t.inner, f, verbosity)?;
                write!(f, ", {}>", t.access_mode)
            }
            TypeVerbosity::Compact => {
                write!(f, "ref<")?;
                write_ty(db, t.inner, f, verbosity)?;
                write!(f, ">")
            }
            TypeVerbosity::Inner => write_ty(db, t.inner, f, verbosity),
        },
        TyKind::Ptr(t) => match verbosity {
            TypeVerbosity::Full => {
                write!(f, "ptr<{}, ", t.storage_class)?;
                write_ty(db, t.inner, f, verbosity)?;
                write!(f, ", {}>", t.access_mode)
            }
            TypeVerbosity::Compact | TypeVerbosity::Inner => {
                write!(f, "ptr<")?;
                write_ty(db, t.inner, f, verbosity)?;
                write!(f, ">")
            }
        },
        TyKind::BoundVar(var) => {
            write!(f, "{}", ('T'..).nth(var.index).unwrap())
        }
        TyKind::StorageTypeOfTexelFormat(var) => {
            write!(f, "{}::StorageType", ('F'..).nth(var.index).unwrap())
        }
    }
}
