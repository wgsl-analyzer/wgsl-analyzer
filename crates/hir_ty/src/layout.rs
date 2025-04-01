use hir_def::data::LocalFieldId;
use la_arena::ArenaMap;

use crate::{
    db::HirDatabase,
    ty::{ArraySize, ArrayType, ScalarType, Ty, TyKind, VecSize},
};

type Bytes = u32;

fn round_up(
    multiple: Bytes,
    num: Bytes,
) -> Bytes {
    num.div_ceil(multiple) * multiple
}

#[test]
fn test_round_up() {
    assert_eq!(round_up(16, 10), 16);
    assert_eq!(round_up(16, 16), 16);
    assert_eq!(round_up(32, 17), 32);
    assert_eq!(round_up(32, 35), 64);
    assert_eq!(round_up(32, 102), 128);
}

#[derive(Clone, Copy)]
pub enum LayoutAddressSpace {
    Storage,
    Uniform,
}

impl ArrayType {
    pub fn stride(
        &self,
        address_space: LayoutAddressSpace,
        db: &dyn HirDatabase,
    ) -> Option<Bytes> {
        let stride = round_up(
            self.inner.align(address_space, db)?,
            self.inner.size(address_space, db)?,
        );
        match address_space {
            LayoutAddressSpace::Storage => Some(stride),
            LayoutAddressSpace::Uniform => Some(round_up(16, stride)),
        }
    }
}

impl Ty {
    pub fn align(
        &self,
        address_space: LayoutAddressSpace,
        db: &dyn HirDatabase,
    ) -> Option<Bytes> {
        self.kind(db).align(address_space, db)
    }

    pub fn size(
        &self,
        address_space: LayoutAddressSpace,
        db: &dyn HirDatabase,
    ) -> Option<Bytes> {
        self.kind(db).size(address_space, db)
    }
}

impl TyKind {
    pub fn align(
        &self,
        address_space: LayoutAddressSpace,
        db: &dyn HirDatabase,
    ) -> Option<Bytes> {
        Some(match self {
            TyKind::Scalar(ScalarType::I32 | ScalarType::U32 | ScalarType::F32) => 4,
            TyKind::Scalar(ScalarType::Bool) => return None,
            TyKind::Atomic(_) => 4,
            TyKind::Vector(v) => match v.size {
                VecSize::Two => 8,
                VecSize::Three => 16,
                VecSize::Four => 16,
                VecSize::BoundVar(_) => return None,
            },
            TyKind::Matrix(m) => match m.rows {
                VecSize::Two => 8,
                VecSize::Three => 16,
                VecSize::Four => 16,
                VecSize::BoundVar(_) => return None,
            },
            TyKind::Struct(r#struct) => {
                let fields = db.field_types(*r#struct);
                let (align, _) =
                    struct_member_layout(&fields, db, LayoutAddressSpace::Storage, |_, _| {})?;

                match address_space {
                    LayoutAddressSpace::Storage => align,
                    LayoutAddressSpace::Uniform => round_up(16, align),
                }
            },
            TyKind::Array(array) => {
                let inner_align = array.inner.align(address_space, db)?;
                match address_space {
                    LayoutAddressSpace::Storage => inner_align,
                    LayoutAddressSpace::Uniform => round_up(16, inner_align),
                }
            },
            _ => return None,
        })
    }

    pub fn size(
        &self,
        address_space: LayoutAddressSpace,
        db: &dyn HirDatabase,
    ) -> Option<Bytes> {
        Some(match self {
            TyKind::Scalar(ScalarType::I32 | ScalarType::U32 | ScalarType::F32) => 4,
            TyKind::Scalar(ScalarType::Bool) => return None,
            TyKind::Atomic(_) => 4,
            TyKind::Vector(v) => match v.size {
                VecSize::Two => 8,
                VecSize::Three => 12,
                VecSize::Four => 16,
                VecSize::BoundVar(_) => return None,
            },
            TyKind::Matrix(m) => {
                let n = m.columns.as_u8() as Bytes;
                let (vec_align, vec_size) = match m.columns {
                    VecSize::Two => (8, 8),
                    VecSize::Three => (16, 12),
                    VecSize::Four => (16, 16),
                    VecSize::BoundVar(_) => return None,
                };

                round_up(vec_align, vec_size) * n
            },
            TyKind::Struct(r#struct) => {
                let fields = db.field_types(*r#struct);
                let (_, size) =
                    struct_member_layout(&fields, db, LayoutAddressSpace::Storage, |_, _| {})?;
                size
            },
            TyKind::Array(array) => match array.size {
                ArraySize::Constant(n) => {
                    let stride = array.stride(address_space, db)?;
                    n as Bytes * stride
                },
                ArraySize::Dynamic => return None,
            },
            _ => return None,
        })
    }
}

pub struct FieldLayout {
    pub offset: Bytes,
    pub align: Bytes,
    pub size: Bytes,
}

/// Returns the (align, size) of the struct, and calls `on_field` for every field
pub fn struct_member_layout<R>(
    fields: &ArenaMap<LocalFieldId, Ty>,
    db: &dyn HirDatabase,
    address_space: LayoutAddressSpace,
    mut on_field: impl FnMut(LocalFieldId, FieldLayout) -> R,
) -> Option<(Bytes, Bytes)> {
    let mut struct_align = Bytes::MIN;

    let mut offset = 0;
    let mut last_member_size = None;

    for (field_id, &field) in fields.iter() {
        let custom_align = None; // TODO handle @align @size
        let custom_size = None;

        let align = custom_align.or_else(|| field.align(address_space, db))?;
        let size = custom_size.or_else(|| field.align(address_space, db))?;

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
        LayoutAddressSpace::Storage => struct_align,
        LayoutAddressSpace::Uniform => round_up(16, struct_align),
    };

    Some((struct_align, struct_size))
}
