use hir_def::data::LocalFieldId;
use la_arena::ArenaMap;

use crate::{
    ty::{ArraySize, ScalarType, Ty, TyKind, VecSize},
    HirDatabase,
};

type Bytes = u32;

fn round_up(multiple: Bytes, num: Bytes) -> Bytes {
    (num + multiple - 1) / multiple * multiple
}

impl Ty {
    pub fn align(&self, db: &dyn HirDatabase) -> Option<Bytes> {
        self.kind(db).align(db)
    }
    pub fn size(&self, db: &dyn HirDatabase) -> Option<Bytes> {
        self.kind(db).size(db)
    }
}

impl TyKind {
    pub fn align(&self, db: &dyn HirDatabase) -> Option<Bytes> {
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
            TyKind::Struct(strukt) => {
                let fields = db.field_types(*strukt);
                let (align, _) = struct_member_layout(&fields, db, |_, _| {})?;
                align
            }
            TyKind::Array(array) => array.inner.align(db)?,
            _ => return None,
        })
    }

    pub fn size(&self, db: &dyn HirDatabase) -> Option<Bytes> {
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
            }
            TyKind::Struct(strukt) => {
                let fields = db.field_types(*strukt);
                let (_, size) = struct_member_layout(&fields, db, |_, _| {})?;
                size
            }
            TyKind::Array(array) => match array.size {
                ArraySize::Const(n) => {
                    let element_size = round_up(array.inner.align(db)?, array.inner.align(db)?);
                    n as Bytes * element_size
                }
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
    mut on_field: impl FnMut(LocalFieldId, FieldLayout) -> R,
) -> Option<(Bytes, Bytes)> {
    let mut struct_align = Bytes::MIN;

    let mut offset = 0;
    let mut last_member_size = None;

    for (field_id, &field) in fields.iter() {
        let custom_align = None; // TODO handle @align @size
        let custom_size = None;

        let align = custom_align.or_else(|| field.align(db))?;
        let size = custom_size.or_else(|| field.align(db))?;

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

    Some((struct_align, struct_size))
}
