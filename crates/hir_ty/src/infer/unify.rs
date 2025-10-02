use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;
use wgsl_types::syntax::AccessMode;

use crate::{
    database::HirDatabase,
    ty::{
        BoundVar, MatrixType, ScalarType, TexelFormat, TextureKind, TextureType, TyKind, Type,
        VecSize, VectorType,
    },
};

#[derive(Default)]
pub struct UnificationTable {
    types: FxHashMap<BoundVar, Type>,
    vec_sizes: FxHashMap<BoundVar, VecSize>,
    texel_formats: FxHashMap<BoundVar, TexelFormat>,
}

impl UnificationTable {
    fn set_vec_size(
        &mut self,
        var: BoundVar,
        vec_size: VecSize,
    ) -> Result<(), ()> {
        match self.vec_sizes.entry(var) {
            Entry::Occupied(entry) if *entry.get() == vec_size => Ok(()),
            Entry::Occupied(_) => Err(()),
            Entry::Vacant(entry) => {
                entry.insert(vec_size);
                Ok(())
            },
        }
    }

    fn set_type(
        &mut self,
        var: BoundVar,
        r#type: Type,
    ) -> Result<(), ()> {
        match self.types.entry(var) {
            Entry::Occupied(entry) if *entry.get() == r#type => Ok(()),
            Entry::Occupied(_) => Err(()),
            Entry::Vacant(entry) => {
                entry.insert(r#type);
                Ok(())
            },
        }
    }

    fn set_texel_format(
        &mut self,
        var: BoundVar,
        format: TexelFormat,
    ) -> Result<(), ()> {
        match self.texel_formats.entry(var) {
            Entry::Occupied(entry) if *entry.get() == format => Ok(()),
            Entry::Occupied(_) => Err(()),
            Entry::Vacant(entry) => {
                entry.insert(format);
                Ok(())
            },
        }
    }

    pub fn resolve(
        &self,
        database: &dyn HirDatabase,
        r#type: Type,
    ) -> Type {
        match r#type.kind(database) {
            TyKind::BoundVar(var) => *self.types.get(&var).expect("type var not constrained"),
            TyKind::Vector(VectorType {
                size,
                component_type: inner,
            }) => {
                let size = match size {
                    VecSize::BoundVar(size_var) => *self
                        .vec_sizes
                        .get(&size_var)
                        .expect("vec size var not constrained"),
                    (VecSize::Two | VecSize::Three | VecSize::Four) => size,
                };
                let inner = self.resolve(database, inner);
                TyKind::Vector(VectorType {
                    size,
                    component_type: inner,
                })
                .intern(database)
            },
            TyKind::Matrix(mat) => {
                let columns = match mat.columns {
                    VecSize::BoundVar(var) => self.vec_sizes[&var],
                    other @ (VecSize::Two | VecSize::Three | VecSize::Four) => other,
                };
                let rows = match mat.rows {
                    VecSize::BoundVar(var) => self.vec_sizes[&var],
                    other @ (VecSize::Two | VecSize::Three | VecSize::Four) => other,
                };

                let inner = self.resolve(database, mat.inner);
                TyKind::Matrix(MatrixType {
                    columns,
                    rows,
                    inner,
                })
                .intern(database)
            },
            TyKind::Texture(TextureType {
                kind: TextureKind::Storage(TexelFormat::BoundVar(var), mode),
                dimension,
                arrayed,
                multisampled,
            }) => {
                let format = self.texel_formats[&var];

                TyKind::Texture(TextureType {
                    kind: TextureKind::Storage(format, mode),
                    dimension,
                    arrayed,
                    multisampled,
                })
                .intern(database)
            },
            TyKind::Texture(TextureType {
                kind: TextureKind::Sampled(sampled_ty),
                dimension,
                arrayed,
                multisampled,
            }) => {
                let sampled_ty = self.resolve(database, sampled_ty);
                TyKind::Texture(TextureType {
                    kind: TextureKind::Sampled(sampled_ty),
                    dimension,
                    arrayed,
                    multisampled,
                })
                .intern(database)
            },
            TyKind::StorageTypeOfTexelFormat(var) => {
                let format = self.texel_formats[&var];
                storage_type_of_texel_format(database, format)
            },
            TyKind::Error
            | TyKind::Scalar(_)
            | TyKind::Atomic(_)
            | TyKind::Struct(_)
            | TyKind::Array(_)
            | TyKind::Texture(_)
            | TyKind::Sampler(_)
            | TyKind::Reference(_)
            | TyKind::Pointer(_) => r#type,
        }
    }
}

// found type should not contain bound variables
#[expect(clippy::too_many_lines, reason = "TODO")]
pub fn unify(
    database: &dyn HirDatabase,
    table: &mut UnificationTable,
    expected: Type,
    found: Type,
) -> Result<(), ()> {
    let expected_kind = expected.kind(database);
    let found_kind = found.kind(database);

    match expected_kind {
        TyKind::BoundVar(var) => {
            table.set_type(var, found)?;
            Ok(())
        },
        TyKind::Vector(VectorType {
            size,
            component_type: inner,
        }) => match found_kind {
            TyKind::Vector(found_vec) => {
                unify(database, table, inner, found_vec.component_type)?;
                if let VecSize::BoundVar(vec_size_var) = size {
                    table.set_vec_size(vec_size_var, found_vec.size)?;
                } else if size != found_vec.size {
                    return Err(());
                }
                Ok(())
            },
            TyKind::Error
            | TyKind::Scalar(_)
            | TyKind::Atomic(_)
            | TyKind::Matrix(_)
            | TyKind::Struct(_)
            | TyKind::Array(_)
            | TyKind::Texture(_)
            | TyKind::Sampler(_)
            | TyKind::Reference(_)
            | TyKind::Pointer(_)
            | TyKind::BoundVar(_)
            | TyKind::StorageTypeOfTexelFormat(_) => Err(()),
        },
        TyKind::Matrix(MatrixType {
            columns,
            rows,
            inner,
        }) => match found_kind {
            TyKind::Matrix(found_mat) => {
                unify(database, table, inner, found_mat.inner)?;

                if let VecSize::BoundVar(var) = columns {
                    table.set_vec_size(var, found_mat.columns)?;
                } else if columns != found_mat.columns {
                    return Err(());
                }

                if let VecSize::BoundVar(var) = rows {
                    table.set_vec_size(var, found_mat.rows)?;
                } else if rows != found_mat.rows {
                    return Err(());
                }

                Ok(())
            },
            TyKind::Error
            | TyKind::Scalar(_)
            | TyKind::Atomic(_)
            | TyKind::Vector(_)
            | TyKind::Struct(_)
            | TyKind::Array(_)
            | TyKind::Texture(_)
            | TyKind::Sampler(_)
            | TyKind::Reference(_)
            | TyKind::Pointer(_)
            | TyKind::BoundVar(_)
            | TyKind::StorageTypeOfTexelFormat(_) => Err(()),
        },
        TyKind::Pointer(pointer) => match found_kind {
            TyKind::Pointer(found_pointer) => {
                unify(database, table, pointer.inner, found_pointer.inner)?;

                Ok(())
            },
            TyKind::Error
            | TyKind::Scalar(_)
            | TyKind::Atomic(_)
            | TyKind::Vector(_)
            | TyKind::Matrix(_)
            | TyKind::Struct(_)
            | TyKind::Array(_)
            | TyKind::Texture(_)
            | TyKind::Sampler(_)
            | TyKind::Reference(_)
            | TyKind::BoundVar(_)
            | TyKind::StorageTypeOfTexelFormat(_) => Err(()),
        },
        TyKind::Array(array) => match found_kind {
            TyKind::Array(found_array) => {
                unify(database, table, array.inner, found_array.inner)?;

                Ok(())
            },
            TyKind::Error
            | TyKind::Scalar(_)
            | TyKind::Atomic(_)
            | TyKind::Vector(_)
            | TyKind::Matrix(_)
            | TyKind::Struct(_)
            | TyKind::Texture(_)
            | TyKind::Sampler(_)
            | TyKind::Reference(_)
            | TyKind::Pointer(_)
            | TyKind::BoundVar(_)
            | TyKind::StorageTypeOfTexelFormat(_) => Err(()),
        },
        TyKind::Atomic(atomic) => match found_kind {
            TyKind::Atomic(found_atomic) => {
                unify(database, table, atomic.inner, found_atomic.inner)?;

                Ok(())
            },
            TyKind::Error
            | TyKind::Scalar(_)
            | TyKind::Vector(_)
            | TyKind::Matrix(_)
            | TyKind::Struct(_)
            | TyKind::Array(_)
            | TyKind::Texture(_)
            | TyKind::Sampler(_)
            | TyKind::Reference(_)
            | TyKind::Pointer(_)
            | TyKind::BoundVar(_)
            | TyKind::StorageTypeOfTexelFormat(_) => Err(()),
        },
        TyKind::Texture(TextureType {
            kind: TextureKind::Storage(format, mode),
            arrayed,
            multisampled,
            dimension,
        }) => match found_kind {
            TyKind::Texture(TextureType {
                kind: TextureKind::Storage(format_2, mode_2),
                arrayed: arrayed_2,
                multisampled: multisampled_2,
                dimension: dimension_2,
            }) => {
                if arrayed != arrayed_2
                    || multisampled != multisampled_2
                    || dimension != dimension_2
                {
                    return Err(());
                }

                match format {
                    TexelFormat::Any => {},
                    TexelFormat::BoundVar(var) => {
                        table.set_texel_format(var, format_2)?;
                    },
                    TexelFormat::Rgba8unorm
                    | TexelFormat::Rgba8snorm
                    | TexelFormat::Rgba8uint
                    | TexelFormat::Rgba8sint
                    | TexelFormat::Rgba16uint
                    | TexelFormat::Rgba16sint
                    | TexelFormat::Rgba16float
                    | TexelFormat::Rgba32uint
                    | TexelFormat::Rgba32sint
                    | TexelFormat::Rgba32float
                    | TexelFormat::R32uint
                    | TexelFormat::R32sint
                    | TexelFormat::R32float
                    | TexelFormat::Rg32uint
                    | TexelFormat::Rg32sint
                    | TexelFormat::Rg32float => {
                        if format != format_2 {
                            return Err(());
                        }
                    },
                }
                match (mode, mode_2) {
                    (AccessMode::Read, AccessMode::ReadWrite | AccessMode::Read)
                    | (AccessMode::ReadWrite, AccessMode::ReadWrite)
                    | (AccessMode::Write, AccessMode::ReadWrite | AccessMode::Write) => {},
                    #[expect(clippy::unreachable, reason = "TODO")]
                    (AccessMode::Write | AccessMode::ReadWrite, AccessMode::Read)
                    | (AccessMode::Read | AccessMode::ReadWrite, AccessMode::Write) => {
                        return Err(());
                    },
                    (AccessMode::Atomic, _) | (_, AccessMode::Atomic) => todo!("What's this?"),
                }

                Ok(())
            },
            TyKind::Error
            | TyKind::Scalar(_)
            | TyKind::Atomic(_)
            | TyKind::Vector(_)
            | TyKind::Matrix(_)
            | TyKind::Struct(_)
            | TyKind::Array(_)
            | TyKind::Texture(_)
            | TyKind::Sampler(_)
            | TyKind::Reference(_)
            | TyKind::Pointer(_)
            | TyKind::BoundVar(_)
            | TyKind::StorageTypeOfTexelFormat(_) => Err(()),
        },
        TyKind::StorageTypeOfTexelFormat(format) => {
            let format = table.texel_formats[&format];
            let storage_type = storage_type_of_texel_format(database, format);

            if storage_type != found {
                return Err(());
            }

            Ok(())
        },

        TyKind::Texture(TextureType {
            kind: TextureKind::Sampled(sampled_ty),
            arrayed,
            multisampled,
            dimension,
        }) => match found_kind {
            TyKind::Texture(TextureType {
                kind: TextureKind::Sampled(found_sampled_ty),
                arrayed: arrayed_2,
                multisampled: multisampled_2,
                dimension: dimension_2,
            }) => {
                if arrayed != arrayed_2
                    || multisampled != multisampled_2
                    || dimension != dimension_2
                {
                    return Err(());
                }

                unify(database, table, sampled_ty, found_sampled_ty)?;

                Ok(())
            },
            TyKind::Error
            | TyKind::Scalar(_)
            | TyKind::Atomic(_)
            | TyKind::Vector(_)
            | TyKind::Matrix(_)
            | TyKind::Struct(_)
            | TyKind::Array(_)
            | TyKind::Texture(_)
            | TyKind::Sampler(_)
            | TyKind::Reference(_)
            | TyKind::Pointer(_)
            | TyKind::BoundVar(_)
            | TyKind::StorageTypeOfTexelFormat(_) => Err(()),
        },
        TyKind::Error
        | TyKind::Scalar(_)
        | TyKind::Struct(_)
        | TyKind::Texture(_)
        | TyKind::Sampler(_)
        | TyKind::Reference(_)
            if expected == found =>
        {
            Ok(())
        },
        TyKind::Error
        | TyKind::Scalar(_)
        | TyKind::Struct(_)
        | TyKind::Texture(_)
        | TyKind::Sampler(_)
        | TyKind::Reference(_) => Err(()),
    }
}

fn storage_type_of_texel_format(
    database: &dyn HirDatabase,
    format: TexelFormat,
) -> Type {
    let channel_type = match format {
        TexelFormat::Rgba8unorm
        | TexelFormat::Rgba8snorm
        | TexelFormat::Rgba16float
        | TexelFormat::Rgba32float
        | TexelFormat::R32float
        | TexelFormat::Rg32float => ScalarType::F32,
        TexelFormat::Rgba8sint
        | TexelFormat::Rgba16sint
        | TexelFormat::Rgba32sint
        | TexelFormat::R32sint
        | TexelFormat::Rg32sint => ScalarType::I32,
        TexelFormat::Rgba8uint
        | TexelFormat::Rgba16uint
        | TexelFormat::Rgba32uint
        | TexelFormat::R32uint
        | TexelFormat::Rg32uint => ScalarType::U32,

        #[expect(clippy::unreachable, reason = "TODO")]
        TexelFormat::BoundVar(_) | TexelFormat::Any => unreachable!(),
    };
    TyKind::Vector(VectorType {
        size: VecSize::Four,
        component_type: TyKind::Scalar(channel_type).intern(database),
    })
    .intern(database)
}
