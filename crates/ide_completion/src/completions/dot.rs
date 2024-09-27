use hir_ty::ty::TyKind;
use itertools::Itertools;

use crate::{
    context::{CompletionContext, ImmediateLocation},
    item::{CompletionItem, CompletionItemKind, CompletionRelevance},
};

use super::Completions;

pub(crate) fn complete_dot(acc: &mut Completions, ctx: &CompletionContext) -> Option<()> {
    let field_expr = match &ctx.completion_location {
        Some(ImmediateLocation::FieldAccess { expr }) => expr,
        _ => return Some(()),
    };
    let sa = ctx.sema.analyze(ctx.container?);
    let ty = sa.type_of_expr(&field_expr.expr()?)?;

    let field_completion_item =
        |name| CompletionItem::new(CompletionItemKind::Field, ctx.source_range(), name).build();

    match ty.kind(ctx.db).unref(ctx.db).as_ref() {
        TyKind::Vector(vec) => {
            let size = vec.size.as_u8() as usize;
            let swizzle = swizzle_items(size, ctx, &[["x", "y", "z", "w"], ["r", "g", "b", "a"]]);
            acc.add_all(swizzle);
        }
        TyKind::Matrix(_) => return None,
        TyKind::Struct(strukt) => {
            let strukt = ctx.db.struct_data(*strukt);
            let items = strukt
                .fields()
                .iter()
                .map(|(_, field)| field.name.as_str())
                .map(field_completion_item);
            acc.add_all(items);
        }
        _ => return None,
    };

    Some(())
}

fn swizzle_items<'a>(
    size: usize,
    ctx: &'a CompletionContext,
    sets: &'a [[&'a str; 4]],
) -> impl Iterator<Item = CompletionItem> + 'a {
    let swizzle = move |set: &'a [&'a str; 4]| {
        (1..=4).flat_map(move |n| {
            (std::iter::repeat_with(|| set[0..size].iter()).take(n))
                .multi_cartesian_product()
                .map(|result| result.into_iter().copied().collect::<String>())
        })
    };
    sets.iter()
        .flat_map(swizzle)
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
