use std::iter;

use hir_def::database::DefDatabase as _;
use hir_ty::ty::TypeKind;

use super::Completions;
use crate::{
    context::{CompletionContext, ImmediateLocation},
    item::{CompletionItem, CompletionItemKind},
};

pub(crate) fn complete_dot(
    accumulator: &mut Completions,
    context: &CompletionContext<'_>,
) -> Option<()> {
    let Some(ImmediateLocation::FieldAccess { expression }) = &context.completion_location else {
        return Some(());
    };
    match context
        .semantics
        .analyze(
            context
                .container
                .and_then(hir::ChildContainer::as_def_with_body_id)?,
        )
        .type_of_expression(&expression.expression()?)?
        .kind(context.database)
        .unref(context.database)
        .as_ref()
    {
        TypeKind::Vector(vector) => {
            vector_completions(accumulator, context, expression, vector);
            Some(())
        },
        TypeKind::Struct(r#struct) => {
            struct_completions(accumulator, context, *r#struct);
            Some(())
        },
        TypeKind::Error
        | TypeKind::Scalar(_)
        | TypeKind::Atomic(_)
        | TypeKind::Matrix(_)
        | TypeKind::Array(_)
        | TypeKind::Texture(_)
        | TypeKind::Sampler(_)
        | TypeKind::Reference(_)
        | TypeKind::Pointer(_)
        | TypeKind::BoundVariable(_)
        | TypeKind::StorageTypeOfTexelFormat(_) => None,
    }
}

fn struct_completions(
    accumulator: &mut Completions,
    context: &CompletionContext<'_>,
    r#struct: hir_def::database::StructId,
) {
    let field_completion_item = |name| {
        CompletionItem::new(CompletionItemKind::Field, context.source_range(), name)
            .build(context.database)
    };

    let r#struct = context.database.struct_data(r#struct).0;
    let items = r#struct
        .fields()
        .iter()
        .map(|(_, field)| field.name.as_str())
        .map(field_completion_item);
    accumulator.add_all(items);
}

fn vector_completions(
    accumulator: &mut Completions,
    context: &CompletionContext<'_>,
    expression: &syntax::ast::FieldExpression,
    vector_type: &hir_ty::ty::VectorType,
) {
    let field_text = expression
        .field()
        .map(|name| name.text().to_owned())
        // It should never be `None` because `x.$0` gets parsed as `Some("")`.
        .unwrap_or_default();

    if is_swizzleable(&field_text) {
        let size: usize = vector_type.size.as_u8().into();
        debug_assert!(
            (MIN_VECTOR_SIZE..=MAX_VECTOR_SIZE).contains(&size),
            "Invalid vector size: {size}"
        );
        let possible_swizzles = possible_swizzles(size, &field_text);
        let suggestions = possible_swizzles.map(|label| {
            let binding =
                CompletionItem::new(CompletionItemKind::Field, context.source_range(), label);
            binding.build(context.database)
        });
        accumulator.add_all(suggestions);
    }
}

/// Tells whether swizzle completions are valid.
fn is_swizzleable(field_text: &str) -> bool {
    if !(0..=MAX_VECTOR_SIZE).contains(&field_text.len()) {
        return false;
    }

    let is_rgba = field_text
        .chars()
        .all(|character| matches!(character, 'r' | 'g' | 'b' | 'a'));

    let is_xyzw = field_text
        .chars()
        .all(|character| matches!(character, 'x' | 'y' | 'z' | 'w'));

    is_rgba || is_xyzw
}

/// <https://www.w3.org/TR/WGSL/#vector>
const MIN_VECTOR_SIZE: usize = 2;

/// <https://www.w3.org/TR/WGSL/#vector>
const MAX_VECTOR_SIZE: usize = 4;

/// <https://www.w3.org/TR/WGSL/#syntax-swizzle_name>
const SWIZZLE_SETS: &[&str] = &["xyzw", "rgba"];

/// Return all possible valid swizzles that are compatible with what has already been typed.
fn possible_swizzles(
    max_length: usize,
    field_text: &str,
) -> impl Iterator<Item = String> {
    SWIZZLE_SETS
        .iter()
        .filter_map(move |swizzle_set| swizzler(swizzle_set, field_text, max_length))
        .flat_map(iter::IntoIterator::into_iter)
        .chain(iter::once(field_text.to_owned()))
        .filter(|swizzle| !swizzle.is_empty())
}

/// Given a set of swizzle characters relevant source info, return valid longer swizzles.
fn swizzler(
    swizzle: &&str,
    field_text: &str,
    max_length: usize,
) -> Option<impl iter::Iterator<Item = String>> {
    // Do not show "rgb" swizzles for "xyz"
    // and do not suggest further changes for invalid swizzles
    let characters_allowed = field_text.is_empty()
        || (field_text.len() < MAX_VECTOR_SIZE
            && swizzle
                .chars()
                .any(|character| field_text.contains(character)));

    characters_allowed.then(|| {
        swizzle[0..max_length]
            .chars()
            .map(move |next_character| format!("{field_text}{next_character}"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_swizzleable_valid() {
        assert!(is_swizzleable("r"));
        assert!(is_swizzleable("rgba"));
        assert!(is_swizzleable("yxx"));
    }

    #[test]
    fn is_swizzleable_invalid() {
        assert!(!is_swizzleable("rgbaa"));
        assert!(!is_swizzleable("rgbab"));
        assert!(!is_swizzleable("rxgba"));
        assert!(!is_swizzleable("bx"));
    }

    #[test]
    fn possible_swizzles_is_correct() {
        // empty
        let swizzles: Vec<_> = possible_swizzles(2, "").collect();
        assert_eq!(swizzles, vec!["x", "y", "r", "g"]);

        let swizzles: Vec<_> = possible_swizzles(3, "").collect();
        assert_eq!(swizzles, vec!["x", "y", "z", "r", "g", "b"]);

        let swizzles: Vec<_> = possible_swizzles(4, "").collect();
        assert_eq!(swizzles, vec!["x", "y", "z", "w", "r", "g", "b", "a"]);

        // x
        let swizzles: Vec<_> = possible_swizzles(2, "x").collect();
        assert_eq!(swizzles, vec!["xx", "xy", "x"]);

        let swizzles: Vec<_> = possible_swizzles(3, "x").collect();
        assert_eq!(swizzles, vec!["xx", "xy", "xz", "x"]);

        let swizzles: Vec<_> = possible_swizzles(4, "x").collect();
        assert_eq!(swizzles, vec!["xx", "xy", "xz", "xw", "x"]);

        // xy
        let swizzles: Vec<_> = possible_swizzles(2, "xy").collect();
        assert_eq!(swizzles, vec!["xyx", "xyy", "xy"]);

        let swizzles: Vec<_> = possible_swizzles(3, "xy").collect();
        assert_eq!(swizzles, vec!["xyx", "xyy", "xyz", "xy"]);

        let swizzles: Vec<_> = possible_swizzles(4, "xy").collect();
        assert_eq!(swizzles, vec!["xyx", "xyy", "xyz", "xyw", "xy"]);

        // xyx
        let swizzles: Vec<_> = possible_swizzles(2, "xyx").collect();
        assert_eq!(swizzles, vec!["xyxx", "xyxy", "xyx"]);

        let swizzles: Vec<_> = possible_swizzles(3, "xyx").collect();
        assert_eq!(swizzles, vec!["xyxx", "xyxy", "xyxz", "xyx"]);

        let swizzles: Vec<_> = possible_swizzles(4, "xyx").collect();
        assert_eq!(swizzles, vec!["xyxx", "xyxy", "xyxz", "xyxw", "xyx"]);
    }

    #[test]
    fn swizzler_is_correct() {
        let swizzles: Vec<_> = swizzler(&"abcd", "a", 2).unwrap().collect();
        assert_eq!(swizzles, vec!["aa", "ab"]);

        let swizzles: Vec<_> = swizzler(&"abcd", "a", 2).unwrap().collect();
        assert_eq!(swizzles, vec!["aa", "ab"]);

        let swizzles: Vec<_> = swizzler(&"abcd", "a", 4).unwrap().collect();
        assert_eq!(swizzles, vec!["aa", "ab", "ac", "ad"]);

        let swizzles: Vec<_> = swizzler(&"abcd", "d", 2).unwrap().collect();
        assert_eq!(swizzles, vec!["da", "db"]);

        assert!(swizzler(&"abcd", "e", 2).is_none());
    }
}
