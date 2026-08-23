//! <https://www.w3.org/TR/WGSL/#memory-layouts>
//! .

use hir_def::signature::LocalFieldId;
use la_arena::ArenaMap;
use wgsl_types::syntax::AddressSpace;

use crate::{
    db::HirDatabase,
    ty::{ArraySize, ArrayType, ScalarType, Type, TypeKind, VecSize, VectorType},
};

type Bytes = u32;

const fn round_up(
    multiple: Bytes,
    num: Bytes,
) -> Bytes {
    num.div_ceil(multiple) * multiple
}

impl ArrayType {
    pub fn stride(
        &self,
        address_space: AddressSpace,
        db: &dyn HirDatabase,
    ) -> Option<Bytes> {
        let stride = round_up(
            self.inner.align(address_space, db)?,
            self.inner.size(address_space, db)?,
        );
        if address_space == AddressSpace::Uniform {
            Some(round_up(16, stride))
        } else {
            Some(stride)
        }
    }
}

impl Type {
    pub fn align(
        self,
        address_space: AddressSpace,
        db: &dyn HirDatabase,
    ) -> Option<Bytes> {
        self.kind(db).align_of(address_space, db)
    }

    pub fn size(
        self,
        address_space: AddressSpace,
        db: &dyn HirDatabase,
    ) -> Option<Bytes> {
        self.kind(db).size_of(address_space, db)
    }
}

impl TypeKind {
    #[expect(clippy::doc_paragraphs_missing_punctuation, reason = "false positive")]
    /// <https://www.w3.org/TR/WGSL/#alignof>
    pub fn align_of(
        &self,
        address_space: AddressSpace,
        db: &dyn HirDatabase,
    ) -> Option<Bytes> {
        #[expect(
            clippy::match_same_arms,
            reason = "a match arm corresponds to a table row in the specification"
        )]
        match self {
            // <https://www.w3.org/TR/WGSL/#why-is-bool-4-bytes>
            Self::Scalar(ScalarType::Bool) => Some(4),
            Self::Scalar(ScalarType::I32 | ScalarType::U32 | ScalarType::F32) => Some(4),
            // SHADER_INT64
            Self::Scalar(ScalarType::I64 | ScalarType::U64) => Some(8),
            Self::Scalar(ScalarType::F16) => Some(2),
            Self::Atomic(_) => Some(4),
            Self::Vector(VectorType {
                size: VecSize::Two,
                component_type,
            }) if matches!(
                component_type.kind(db),
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
            }) if matches!(component_type.kind(db), Self::Scalar(ScalarType::F16)) => Some(4),
            Self::Vector(VectorType {
                size: VecSize::Three,
                component_type,
            }) if matches!(
                component_type.kind(db),
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
            }) if matches!(component_type.kind(db), Self::Scalar(ScalarType::F16)) => Some(8),
            Self::Vector(VectorType {
                size: VecSize::Four,
                component_type,
            }) if matches!(
                component_type.kind(db),
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
            }) if matches!(component_type.kind(db), Self::Scalar(ScalarType::F16)) => Some(8),
            Self::Matrix(matrix_type) => Self::Vector(VectorType {
                size: matrix_type.rows,
                component_type: matrix_type.inner,
            })
            .align_of(address_space, db),
            Self::Struct(r#struct) => {
                let fields = &db.field_types(*r#struct).0;
                let (align, _) =
                    struct_member_layout(fields, db, AddressSpace::Storage, |_, _, _| {})?;
                Some(if address_space == AddressSpace::Uniform {
                    round_up(16, align)
                } else {
                    align
                })
            },
            Self::Array(array) => {
                let inner_align = array.inner.align(address_space, db)?;
                Some(if address_space == AddressSpace::Uniform {
                    round_up(16, inner_align)
                } else {
                    inner_align
                })
            },
            Self::Error
            | Self::Scalar(ScalarType::AbstractFloat | ScalarType::AbstractInt)
            | Self::Vector(_)
            | Self::BuiltinStruct(_)
            | Self::Texture(_)
            | Self::Sampler(_)
            | Self::AccelerationStructure(_)
            | Self::Reference(_)
            | Self::Pointer(_) => None,
        }
    }

    #[expect(clippy::doc_paragraphs_missing_punctuation, reason = "false positive")]
    /// <https://www.w3.org/TR/WGSL/#sizeof>
    ///
    /// # Panics
    ///
    /// Panics if the size of the array exceeds u32.
    pub fn size_of(
        &self,
        address_space: AddressSpace,
        db: &dyn HirDatabase,
    ) -> Option<Bytes> {
        #[expect(
            clippy::match_same_arms,
            reason = "a match arm corresponds to a table row in the specification"
        )]
        match self {
            Self::Scalar(ScalarType::Bool) => Some(4),
            Self::Scalar(ScalarType::I32 | ScalarType::U32 | ScalarType::F32) => Some(4),
            // SHADER_INT64
            Self::Scalar(ScalarType::I64 | ScalarType::U64) => Some(8),
            Self::Scalar(ScalarType::F16) => Some(2),
            Self::Atomic(_) => Some(4),
            Self::Vector(VectorType {
                size: VecSize::Two,
                component_type,
            }) if matches!(
                component_type.kind(db),
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
            }) if matches!(component_type.kind(db), Self::Scalar(ScalarType::F16)) => Some(4),
            Self::Vector(VectorType {
                size: VecSize::Three,
                component_type,
            }) if matches!(
                component_type.kind(db),
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
            }) if matches!(component_type.kind(db), Self::Scalar(ScalarType::F16)) => Some(6),
            Self::Vector(VectorType {
                size: VecSize::Four,
                component_type,
            }) if matches!(
                component_type.kind(db),
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
            }) if matches!(component_type.kind(db), Self::Scalar(ScalarType::F16)) => Some(8),
            Self::Matrix(matrix_type) => Self::Vector(VectorType {
                size: matrix_type.rows,
                component_type: matrix_type.inner,
            })
            .size_of(address_space, db),
            Self::Struct(r#struct) => {
                let fields = &db.field_types(*r#struct).0;
                let (_, size) =
                    struct_member_layout(fields, db, AddressSpace::Storage, |_, _, _| {})?;
                Some(size)
            },
            Self::Array(array) => match array.size {
                ArraySize::Constant(size) => {
                    let stride = array.stride(address_space, db)?;
                    Some(size.get().checked_mul(stride).unwrap())
                },
                ArraySize::Dynamic => None,
            },
            Self::Error
            | Self::Scalar(ScalarType::AbstractFloat | ScalarType::AbstractInt)
            | Self::BuiltinStruct(_)
            | Self::Vector(_)
            | Self::Texture(_)
            | Self::Sampler(_)
            | Self::AccelerationStructure(_)
            | Self::Reference(_)
            | Self::Pointer(_) => None,
        }
    }
}

pub struct FieldLayout {
    pub offset: Bytes,
    pub align: Bytes,
    pub size: Bytes,
}

/// Returns the (align, size) of the struct, and calls `on_field` for every field.
pub fn struct_member_layout<Result, Function>(
    fields: &ArenaMap<LocalFieldId, Type>,
    db: &dyn HirDatabase,
    address_space: AddressSpace,
    mut on_field: Function,
) -> Option<(Bytes, Bytes)>
where
    Function: FnMut(LocalFieldId, Type, FieldLayout) -> Result,
{
    let mut struct_align = Bytes::MIN;

    let mut offset = 0;
    let mut last_member_size = None;

    for (field_id, &field) in fields.iter() {
        // TODO: handle @align and @size
        // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/678
        let custom_align = None;
        let custom_size = None;

        let align = custom_align.or_else(|| field.align(address_space, db))?;
        let size = custom_size.or_else(|| field.size(address_space, db))?;

        struct_align = struct_align.max(align);

        on_field(
            field_id,
            field,
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
    let struct_align = if address_space == AddressSpace::Uniform {
        round_up(16, struct_align)
    } else {
        struct_align
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
