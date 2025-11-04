use std::collections::hash_map::Entry;

use rustc_hash::FxHashMap;
use wgsl_types::syntax::AccessMode;

use crate::{
    database::HirDatabase,
    ty::{
        BoundVariable, MatrixType, ScalarType, TexelFormat, TextureKind, TextureType, Type,
        TypeKind, VecSize, VectorType,
    },
};

#[derive(Default)]
pub struct UnificationTable {
    types: FxHashMap<BoundVariable, Type>,
    vec_sizes: FxHashMap<BoundVariable, VecSize>,
    texel_formats: FxHashMap<BoundVariable, TexelFormat>,
}

impl UnificationTable {
    fn set_vec_size(
        &mut self,
        variable: BoundVariable,
        vec_size: VecSize,
    ) -> Result<(), ()> {
        match self.vec_sizes.entry(variable) {
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
        variable: BoundVariable,
        r#type: Type,
        database: &dyn HirDatabase,
    ) -> Result<(), ()> {
        match self.types.entry(variable) {
            Entry::Occupied(entry) if *entry.get() == r#type => Ok(()),
            Entry::Occupied(mut entry) => {
                // abstract number conversions
                if entry.get().is_convertible_to(r#type, database) {
                    *entry.get_mut() = r#type;
                    Ok(())
                } else if r#type.is_convertible_to(*entry.get(), database) {
                    Ok(())
                } else {
                    Err(())
                }
            },
            Entry::Vacant(entry) => {
                entry.insert(r#type);
                Ok(())
            },
        }
    }

    fn set_texel_format(
        &mut self,
        variable: BoundVariable,
        format: TexelFormat,
    ) -> Result<(), ()> {
        match self.texel_formats.entry(variable) {
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
            TypeKind::BoundVariable(variable) => {
                *self.types.get(&variable).expect("type var not constrained")
            },
            TypeKind::Vector(VectorType {
                size,
                component_type: inner,
            }) => {
                let size = match size {
                    VecSize::BoundVariable(size_var) => *self
                        .vec_sizes
                        .get(&size_var)
                        .expect("vec size var not constrained"),
                    VecSize::Two | VecSize::Three | VecSize::Four => size,
                };
                let inner = self.resolve(database, inner);
                TypeKind::Vector(VectorType {
                    size,
                    component_type: inner,
                })
                .intern(database)
            },
            TypeKind::Matrix(matrix) => {
                let columns = match matrix.columns {
                    VecSize::BoundVariable(variable) => self.vec_sizes[&variable],
                    other @ (VecSize::Two | VecSize::Three | VecSize::Four) => other,
                };
                let rows = match matrix.rows {
                    VecSize::BoundVariable(variable) => self.vec_sizes[&variable],
                    other @ (VecSize::Two | VecSize::Three | VecSize::Four) => other,
                };

                let inner = self.resolve(database, matrix.inner);
                TypeKind::Matrix(MatrixType {
                    columns,
                    rows,
                    inner,
                })
                .intern(database)
            },
            #[expect(
                deprecated,
                reason = "TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559"
            )]
            TypeKind::Texture(TextureType {
                kind: TextureKind::Storage(TexelFormat::BoundVariable(variable), mode),
                dimension,
                arrayed,
                multisampled,
            }) => {
                let format = self.texel_formats[&variable];

                TypeKind::Texture(TextureType {
                    kind: TextureKind::Storage(format, mode),
                    dimension,
                    arrayed,
                    multisampled,
                })
                .intern(database)
            },
            TypeKind::Texture(TextureType {
                kind: TextureKind::Sampled(sampled_type),
                dimension,
                arrayed,
                multisampled,
            }) => {
                let sampled_type = self.resolve(database, sampled_type);
                TypeKind::Texture(TextureType {
                    kind: TextureKind::Sampled(sampled_type),
                    dimension,
                    arrayed,
                    multisampled,
                })
                .intern(database)
            },
            TypeKind::StorageTypeOfTexelFormat(variable) => {
                let format = self.texel_formats[&variable];
                storage_type_of_texel_format(database, format)
            },
            TypeKind::Error
            | TypeKind::Scalar(_)
            | TypeKind::Atomic(_)
            | TypeKind::Struct(_)
            | TypeKind::Array(_)
            | TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Reference(_)
            | TypeKind::Pointer(_) => r#type,
        }
    }
}

// found type should not contain bound variables
#[expect(
    clippy::too_many_lines,
    reason = "This long match is not easily broken up"
)]
pub fn unify(
    database: &dyn HirDatabase,
    table: &mut UnificationTable,
    expected: Type,
    found: Type,
) -> Result<(), ()> {
    let expected_kind = expected.kind(database);
    let found_kind = found.kind(database);

    match expected_kind {
        TypeKind::BoundVariable(variable) => {
            table.set_type(variable, found, database)?;
            Ok(())
        },
        TypeKind::Vector(VectorType {
            size,
            component_type: inner,
        }) => match found_kind {
            TypeKind::Vector(found_vec) => {
                unify(database, table, inner, found_vec.component_type)?;
                if let VecSize::BoundVariable(vec_size_var) = size {
                    table.set_vec_size(vec_size_var, found_vec.size)?;
                } else if size != found_vec.size {
                    return Err(());
                }
                Ok(())
            },
            TypeKind::Error
            | TypeKind::Scalar(_)
            | TypeKind::Atomic(_)
            | TypeKind::Matrix(_)
            | TypeKind::Struct(_)
            | TypeKind::Array(_)
            | TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Reference(_)
            | TypeKind::Pointer(_)
            | TypeKind::BoundVariable(_)
            | TypeKind::StorageTypeOfTexelFormat(_) => Err(()),
        },
        TypeKind::Matrix(MatrixType {
            columns,
            rows,
            inner,
        }) => match found_kind {
            TypeKind::Matrix(found_mat) => {
                unify(database, table, inner, found_mat.inner)?;

                if let VecSize::BoundVariable(variable) = columns {
                    table.set_vec_size(variable, found_mat.columns)?;
                } else if columns != found_mat.columns {
                    return Err(());
                }

                if let VecSize::BoundVariable(variable) = rows {
                    table.set_vec_size(variable, found_mat.rows)?;
                } else if rows != found_mat.rows {
                    return Err(());
                }

                Ok(())
            },
            TypeKind::Error
            | TypeKind::Scalar(_)
            | TypeKind::Atomic(_)
            | TypeKind::Vector(_)
            | TypeKind::Struct(_)
            | TypeKind::Array(_)
            | TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Reference(_)
            | TypeKind::Pointer(_)
            | TypeKind::BoundVariable(_)
            | TypeKind::StorageTypeOfTexelFormat(_) => Err(()),
        },
        TypeKind::Pointer(pointer) => match found_kind {
            TypeKind::Pointer(found_pointer) => {
                unify(database, table, pointer.inner, found_pointer.inner)?;

                Ok(())
            },
            TypeKind::Error
            | TypeKind::Scalar(_)
            | TypeKind::Atomic(_)
            | TypeKind::Vector(_)
            | TypeKind::Matrix(_)
            | TypeKind::Struct(_)
            | TypeKind::Array(_)
            | TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Reference(_)
            | TypeKind::BoundVariable(_)
            | TypeKind::StorageTypeOfTexelFormat(_) => Err(()),
        },
        TypeKind::Array(array) => match found_kind {
            TypeKind::Array(found_array) => {
                unify(database, table, array.inner, found_array.inner)?;

                Ok(())
            },
            TypeKind::Error
            | TypeKind::Scalar(_)
            | TypeKind::Atomic(_)
            | TypeKind::Vector(_)
            | TypeKind::Matrix(_)
            | TypeKind::Struct(_)
            | TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Reference(_)
            | TypeKind::Pointer(_)
            | TypeKind::BoundVariable(_)
            | TypeKind::StorageTypeOfTexelFormat(_) => Err(()),
        },
        TypeKind::Atomic(atomic) => match found_kind {
            TypeKind::Atomic(found_atomic) => {
                unify(database, table, atomic.inner, found_atomic.inner)?;

                Ok(())
            },
            TypeKind::Error
            | TypeKind::Scalar(_)
            | TypeKind::Vector(_)
            | TypeKind::Matrix(_)
            | TypeKind::Struct(_)
            | TypeKind::Array(_)
            | TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Reference(_)
            | TypeKind::Pointer(_)
            | TypeKind::BoundVariable(_)
            | TypeKind::StorageTypeOfTexelFormat(_) => Err(()),
        },
        TypeKind::Texture(TextureType {
            kind: TextureKind::Storage(format, mode),
            arrayed,
            multisampled,
            dimension,
        }) => match found_kind {
            TypeKind::Texture(TextureType {
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

                #[expect(
                    deprecated,
                    reason = "TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559"
                )]
                match format {
                    TexelFormat::Any => {},
                    TexelFormat::BoundVariable(variable) => {
                        table.set_texel_format(variable, format_2)?;
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
                    | TexelFormat::Rg32float
                    | TexelFormat::Bgra8unorm => {
                        if format != format_2 {
                            return Err(());
                        }
                    },
                }
                match (mode, mode_2) {
                    (AccessMode::Read, AccessMode::ReadWrite | AccessMode::Read)
                    | (AccessMode::ReadWrite, AccessMode::ReadWrite)
                    | (AccessMode::Write, AccessMode::ReadWrite | AccessMode::Write) => {},
                    (AccessMode::Write | AccessMode::ReadWrite, AccessMode::Read)
                    | (AccessMode::Read | AccessMode::ReadWrite, AccessMode::Write) => {
                        return Err(());
                    },
                    // TODO: naga atomics
                    // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/677
                    (AccessMode::Atomic, _) | (_, AccessMode::Atomic) => return Err(()),
                }

                Ok(())
            },
            TypeKind::Error
            | TypeKind::Scalar(_)
            | TypeKind::Atomic(_)
            | TypeKind::Vector(_)
            | TypeKind::Matrix(_)
            | TypeKind::Struct(_)
            | TypeKind::Array(_)
            | TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Reference(_)
            | TypeKind::Pointer(_)
            | TypeKind::BoundVariable(_)
            | TypeKind::StorageTypeOfTexelFormat(_) => Err(()),
        },
        TypeKind::StorageTypeOfTexelFormat(format) => {
            let format = table.texel_formats[&format];
            let storage_type = storage_type_of_texel_format(database, format);

            if storage_type != found {
                return Err(());
            }

            Ok(())
        },
        TypeKind::Texture(TextureType {
            kind: TextureKind::Sampled(sampled_type),
            arrayed,
            multisampled,
            dimension,
        }) => match found_kind {
            TypeKind::Texture(TextureType {
                kind: TextureKind::Sampled(found_sampled_type),
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

                unify(database, table, sampled_type, found_sampled_type)?;

                Ok(())
            },
            TypeKind::Error
            | TypeKind::Scalar(_)
            | TypeKind::Atomic(_)
            | TypeKind::Vector(_)
            | TypeKind::Matrix(_)
            | TypeKind::Struct(_)
            | TypeKind::Array(_)
            | TypeKind::Texture(_)
            | TypeKind::Sampler(_)
            | TypeKind::Reference(_)
            | TypeKind::Pointer(_)
            | TypeKind::BoundVariable(_)
            | TypeKind::StorageTypeOfTexelFormat(_) => Err(()),
        },
        TypeKind::Error
        | TypeKind::Scalar(_)
        | TypeKind::Struct(_)
        | TypeKind::Texture(_)
        | TypeKind::Sampler(_)
        | TypeKind::Reference(_) => {
            // Only 1 direction is checked for now
            // Since "expected" cannot be an abstract type,
            // nor can it contain type variables
            if found.is_convertible_to(expected, database) {
                Ok(())
            } else {
                Err(())
            }
        },
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
        | TexelFormat::Rg32float
        | TexelFormat::Bgra8unorm => ScalarType::F32,
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
        #[expect(
            deprecated,
            clippy::unreachable,
            reason = "TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/559"
        )]
        TexelFormat::BoundVariable(_) | TexelFormat::Any => {
            unreachable!("why is this unreachable?")
        },
    };
    TypeKind::Vector(VectorType {
        size: VecSize::Four,
        component_type: TypeKind::Scalar(channel_type).intern(database),
    })
    .intern(database)
}
