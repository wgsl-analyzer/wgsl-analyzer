use super::{Ty, TyKind};
use crate::{
    infer::{TypeExpectation, TypeExpectationInner},
    ty::{ArraySize, ScalarType, TextureKind},
    HirDatabase,
};
use std::fmt::Write;

pub fn pretty_type_expectation(db: &dyn HirDatabase, ty: TypeExpectation) -> String {
    let mut str = String::new();

    match ty {
        TypeExpectation::Type(ty) => {
            let _ = write_type_expectation_inner(db, ty, false, &mut str);
        }
        TypeExpectation::TypeOrVecOf(inner) => {
            let _ = write_type_expectation_inner(db, inner, true, &mut str);
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
) -> std::fmt::Result {
    match inner {
        TypeExpectationInner::Exact(ty) => {
            write_ty(db, ty, f)?;
            if or_vec {
                write!(f, " or vecN<")?;
                write_ty(db, ty, f)?;
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
    let mut str = String::new();
    write_ty(db, ty, &mut str).unwrap();
    str
}

fn write_ty(db: &dyn HirDatabase, ty: Ty, f: &mut String) -> std::fmt::Result {
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
            write_ty(db, atomic.inner, f)?;
            write!(f, ">")
        }
        TyKind::Vector(t) => {
            write!(f, "vec{}<", t.size)?;
            write_ty(db, t.inner, f)?;
            write!(f, ">")
        }
        TyKind::Matrix(t) => {
            write!(f, "mat{}x{}<", t.columns, t.rows)?;
            write_ty(db, t.inner, f)?;
            write!(f, ">")
        }
        TyKind::Struct(strukt) => {
            let data = db.struct_data(strukt);
            write!(f, "{}", data.name.as_str())
        }
        TyKind::Array(t) => {
            write!(f, "array<")?;
            write_ty(db, t.inner, f)?;
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
        TyKind::Ref(t) => {
            write!(f, "ref<{}, ", t.storage_class)?;
            write_ty(db, t.inner, f)?;
            write!(f, ", {}>", t.access_mode)
        }
        TyKind::Ptr(t) => {
            write!(f, "ptr<{}, ", t.storage_class)?;
            write_ty(db, t.inner, f)?;
            write!(f, ", {}>", t.access_mode)
        }
        TyKind::Function(function) => {
            write!(f, "fn(")?;
            for (i, &param) in function.parameters.iter().enumerate() {
                if i != 0 {
                    f.push_str(", ");
                }
                write_ty(db, param, f)?;
            }
            write!(f, ")")?;
            if let Some(ret) = function.return_type {
                f.push_str(" -> ");
                write_ty(db, ret, f)?;
            }
            Ok(())
        }
        TyKind::BoundVar(var) => {
            write!(f, "{}", ('T'..).nth(var.index).unwrap())
        }
        TyKind::BuiltinFn(builtin) => {
            write!(f, "<builtin> {}", builtin.lookup(db).name.as_str())
        }
        TyKind::StorageTypeOfTexelFormat(var) => {
            write!(f, "{}::StorageType", ('F'..).nth(var.index).unwrap())
        }
    }
}
