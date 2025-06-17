use hir_ty::ty::TyKind;
use itertools::Itertools;

use super::Completions;
use crate::{
    context::{CompletionContext, ImmediateLocation},
    item::{CompletionItem, CompletionItemKind, CompletionRelevance},
};

pub(crate) fn complete_dot(
    accumulator: &mut Completions,
    ctx: &CompletionContext,
) -> Option<()> {
    let field_expression = match &ctx.completion_location {
        Some(ImmediateLocation::FieldAccess { expression }) => expression,
        _ => return Some(()),
    };
    let sa = ctx.sema.analyze(ctx.container?);
    let r#type = sa.type_of_expression(&field_expression.expression()?)?;

    let field_completion_item =
        |name| CompletionItem::new(CompletionItemKind::Field, ctx.source_range(), name).build();

    match r#type.kind(ctx.db).unref(ctx.db).as_ref() {
        TyKind::Vector(vec) => {
            let size = vec.size.as_u8() as usize;
            let field_text = field_expression
                .name_ref()
                .map(|name| name.text().to_string())
                .unwrap_or_default();

            let is_swizzle = field_text.is_empty()
                || field_text
                    .chars()
                    .all(|c| matches!(c, 'x' | 'y' | 'z' | 'w' | 'r' | 'g' | 'b' | 'a'));

            if is_swizzle {
                let swizzle = swizzle_items(
                    size,
                    ctx,
                    &field_text,
                    &[["x", "y", "z", "w"], ["r", "g", "b", "a"]],
                );
                accumulator.add_all(swizzle);
            }
        },
        TyKind::Matrix(_) => return None,
        TyKind::Struct(r#struct) => {
            let r#struct = ctx.db.struct_data(*r#struct);
            let items = r#struct
                .fields()
                .iter()
                .map(|(_, field)| field.name.as_str())
                .map(field_completion_item);
            accumulator.add_all(items);
        },
        _ => return None,
    };

    Some(())
}

fn swizzle_items<'a>(
    size: usize,
    ctx: &'a CompletionContext,
    field_text: &'a str,
    sets: &'a [[&'a str; 4]],
) -> impl Iterator<Item = CompletionItem> + 'a {
    let swizzle = move |set: &'a [&'a str; 4]| {
        // Don't show "rgb" swizzles for "xyz"
        // And don't suggest further changes for long texts
        let chars_allowed = field_text.is_empty()
            || (field_text.len() < 4 && set.iter().any(|v| field_text.contains(v)));

        if chars_allowed {
            either::Either::Left(set[0..size].iter().map(move |v| format!("{field_text}{v}")))
        } else {
            either::Either::Right(std::iter::empty())
        }
    };
    sets.iter()
        .flat_map(swizzle)
        .chain(std::iter::once(field_text.to_string()))
        .enumerate()
        .map(move |(i, label)| {
            CompletionItem::new(CompletionItemKind::Field, ctx.source_range(), label)
                .with_relevance(CompletionRelevance {
                    swizzle_index: Some(i),
                    ..Default::default()
                })
                .build()
        })
}
