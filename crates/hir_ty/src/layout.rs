//! <https://www.w3.org/TR/WGSL/#memory-layouts>

use hir_def::data::LocalFieldId;
use la_arena::ArenaMap;

use crate::{
    database::HirDatabase,
    ty::{ArraySize, ArrayType, ScalarType, TyKind, Type, VecSize, VectorType},
};

type Bytes = u32;

const fn round_up(
    multiple: Bytes,
    num: Bytes,
) -> Bytes {
    num.div_ceil(multiple) * multiple
}

/// All address spaces except uniform have the same constraints as the storage address space.
///
/// <https://www.w3.org/TR/WGSL/#address-space-layout-constraints>
#[derive(Clone, Copy)]
pub enum LayoutAddressSpace {
    Uniform,
    Other,
}

impl ArrayType {
    pub fn stride(
        &self,
        address_space: LayoutAddressSpace,
        database: &dyn HirDatabase,
    ) -> Option<Bytes> {
        let stride = round_up(
            self.inner.align(address_space, database)?,
            self.inner.size(address_space, database)?,
        );
        match address_space {
            LayoutAddressSpace::Other => Some(stride),
            LayoutAddressSpace::Uniform => Some(round_up(16, stride)),
        }
    }
}

impl Type {
    pub fn align(
        self,
        address_space: LayoutAddressSpace,
        database: &dyn HirDatabase,
    ) -> Option<Bytes> {
        self.kind(database).align_of(address_space, database)
    }

    pub fn size(
        self,
        address_space: LayoutAddressSpace,
        database: &dyn HirDatabase,
    ) -> Option<Bytes> {
        self.kind(database).size_of(address_space, database)
    }
}

impl TyKind {
    /// <https://www.w3.org/TR/WGSL/#alignof>
    pub fn align_of(
        &self,
        address_space: LayoutAddressSpace,
        database: &dyn HirDatabase,
    ) -> Option<Bytes> {
        #[expect(
            clippy::match_same_arms,
            reason = "a match arm corresponds to a table row in the specification"
        )]
        match self {
            // <https://www.w3.org/TR/WGSL/#why-is-bool-4-bytes>
            Self::Scalar(ScalarType::Bool) => Some(4),
            Self::Scalar(ScalarType::I32 | ScalarType::U32 | ScalarType::F32) => Some(4),
            Self::Scalar(ScalarType::F16) => Some(2),
            Self::Atomic(_) => Some(4),
            Self::Vector(VectorType {
                size: VecSize::Two,
                component_type,
            }) if matches!(
                component_type.kind(database),
                Self::Scalar(
                    ScalarType::Bool | ScalarType::I32 | ScalarType::U32 | ScalarType::F32
                )
            ) =>
            {
                Some(8)
            },
            Self::Vector(VectorType {
                size: VecSize::Two,
                component_type,
            }) if matches!(component_type.kind(database), Self::Scalar(ScalarType::F16)) => Some(4),
            Self::Vector(VectorType {
                size: VecSize::Three,
                component_type,
            }) if matches!(
                component_type.kind(database),
                Self::Scalar(
                    ScalarType::Bool | ScalarType::I32 | ScalarType::U32 | ScalarType::F32
                )
            ) =>
            {
                Some(16)
            },
            Self::Vector(VectorType {
                size: VecSize::Three,
                component_type,
            }) if matches!(component_type.kind(database), Self::Scalar(ScalarType::F16)) => Some(8),
            Self::Vector(VectorType {
                size: VecSize::Four,
                component_type,
            }) if matches!(
                component_type.kind(database),
                Self::Scalar(
                    ScalarType::Bool | ScalarType::I32 | ScalarType::U32 | ScalarType::F32
                )
            ) =>
            {
                Some(16)
            },
            Self::Vector(VectorType {
                size: VecSize::Four,
                component_type,
            }) if matches!(component_type.kind(database), Self::Scalar(ScalarType::F16)) => Some(8),
            Self::Matrix(matrix_type) => Self::Vector(VectorType {
                size: matrix_type.rows,
                component_type: matrix_type.inner,
            })
            .align_of(address_space, database),
            Self::Struct(r#struct) => {
                let fields = &database.field_types(*r#struct).0;
                let (align, _) =
                    struct_member_layout(fields, database, LayoutAddressSpace::Other, |_, _| {})?;

                Some(match address_space {
                    LayoutAddressSpace::Other => align,
                    LayoutAddressSpace::Uniform => round_up(16, align),
                })
            },
            Self::Array(array) => {
                let inner_align = array.inner.align(address_space, database)?;
                Some(match address_space {
                    LayoutAddressSpace::Other => inner_align,
                    LayoutAddressSpace::Uniform => round_up(16, inner_align),
                })
            },
            Self::Error
            | Self::Scalar(ScalarType::AbstractFloat | ScalarType::AbstractInt)
            | Self::Vector(_)
            | Self::Texture(_)
            | Self::Sampler(_)
            | Self::Reference(_)
            | Self::Pointer(_)
            | Self::BoundVariable(_)
            | Self::StorageTypeOfTexelFormat(_) => None,
        }
    }

    /// <https://www.w3.org/TR/WGSL/#sizeof>
    ///
    /// # Panics
    ///
    /// Panics if the size of the array exceeds.
    pub fn size_of(
        &self,
        address_space: LayoutAddressSpace,
        database: &dyn HirDatabase,
    ) -> Option<Bytes> {
        #[expect(
            clippy::match_same_arms,
            reason = "a match arm corresponds to a table row in the specification"
        )]
        match self {
            Self::Scalar(ScalarType::Bool) => Some(4),
            Self::Scalar(ScalarType::I32 | ScalarType::U32 | ScalarType::F32) => Some(4),
            Self::Scalar(ScalarType::F16) => Some(2),
            Self::Atomic(_) => Some(4),
            Self::Vector(VectorType {
                size: VecSize::Two,
                component_type,
            }) if matches!(
                component_type.kind(database),
                Self::Scalar(
                    ScalarType::Bool | ScalarType::I32 | ScalarType::U32 | ScalarType::F32
                )
            ) =>
            {
                Some(8)
            },
            Self::Vector(VectorType {
                size: VecSize::Two,
                component_type,
            }) if matches!(component_type.kind(database), Self::Scalar(ScalarType::F16)) => Some(4),
            Self::Vector(VectorType {
                size: VecSize::Three,
                component_type,
            }) if matches!(
                component_type.kind(database),
                Self::Scalar(
                    ScalarType::Bool | ScalarType::I32 | ScalarType::U32 | ScalarType::F32
                )
            ) =>
            {
                Some(12)
            },
            Self::Vector(VectorType {
                size: VecSize::Four,
                component_type,
            }) if matches!(component_type.kind(database), Self::Scalar(ScalarType::F16)) => Some(6),
            Self::Vector(VectorType {
                size: VecSize::Four,
                component_type,
            }) if matches!(
                component_type.kind(database),
                Self::Scalar(
                    ScalarType::Bool | ScalarType::I32 | ScalarType::U32 | ScalarType::F32
                )
            ) =>
            {
                Some(16)
            },
            Self::Vector(VectorType {
                size: VecSize::Three,
                component_type,
            }) if matches!(component_type.kind(database), Self::Scalar(ScalarType::F16)) => Some(8),
            Self::Matrix(matrix_type) => Self::Vector(VectorType {
                size: matrix_type.rows,
                component_type: matrix_type.inner,
            })
            .size_of(address_space, database),
            Self::Struct(r#struct) => {
                let fields = &database.field_types(*r#struct).0;
                let (_, size) =
                    struct_member_layout(fields, database, LayoutAddressSpace::Other, |_, _| {})?;
                Some(size)
            },
            Self::Array(array) => match array.size {
                ArraySize::Constant(size) => {
                    let stride = array.stride(address_space, database)?;
                    Some(Bytes::try_from(size).unwrap() * stride)
                },
                ArraySize::Dynamic => None,
            },
            Self::Error
            | Self::Scalar(ScalarType::AbstractFloat | ScalarType::AbstractInt)
            | Self::Vector(_)
            | Self::Texture(_)
            | Self::Sampler(_)
            | Self::Reference(_)
            | Self::Pointer(_)
            | Self::BoundVariable(_)
            | Self::StorageTypeOfTexelFormat(_) => None,
        }
    }
}

pub struct FieldLayout {
    pub offset: Bytes,
    pub align: Bytes,
    pub size: Bytes,
}

/// Returns the (align, size) of the struct, and calls `on_field` for every field
pub fn struct_member_layout<Result, Function: FnMut(LocalFieldId, FieldLayout) -> Result>(
    fields: &ArenaMap<LocalFieldId, Type>,
    database: &dyn HirDatabase,
    address_space: LayoutAddressSpace,
    mut on_field: Function,
) -> Option<(Bytes, Bytes)> {
    let mut struct_align = Bytes::MIN;

    let mut offset = 0;
    let mut last_member_size = None;

    for (field_id, &field) in fields.iter() {
        // TODO: handle @align and @size
        // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/678
        let custom_align = None;
        let custom_size = None;

        let align = custom_align.or_else(|| field.align(address_space, database))?;
        let size = custom_size.or_else(|| field.align(address_space, database))?;

        struct_align = struct_align.max(align);

        on_field(
            field_id,
            FieldLayout {
                offset,
                align,
                size,
            },
        );

        let new_offset = round_up(align, offset + size);
        last_member_size = Some(size);
        offset = new_offset;
    }

    let just_past_last_member = offset + last_member_size?;
    let struct_size = round_up(struct_align, just_past_last_member);

    let struct_align = match address_space {
        LayoutAddressSpace::Other => struct_align,
        LayoutAddressSpace::Uniform => round_up(16, struct_align),
    };

    Some((struct_align, struct_size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[expect(
        clippy::decimal_literal_representation,
        reason = "literal is more clear"
    )]
    fn round_up_is_correct() {
        assert_eq!(round_up(16, 10), 16);
        assert_eq!(round_up(16, 16), 16);
        assert_eq!(round_up(32, 17), 32);
        assert_eq!(round_up(32, 35), 64);
        assert_eq!(round_up(32, 102), 128);
    }
}
