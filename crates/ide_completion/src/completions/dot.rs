use hir_ty::ty::TyKind;
use itertools::Itertools;

use super::Completions;
use crate::{
    context::{CompletionContext, ImmediateLocation},
    item::{CompletionItem, CompletionItemKind, CompletionRelevance},
};

pub(crate) fn complete_dot(
    accumulator: &mut Completions,
    context: &CompletionContext,
) -> Option<()> {
    let Some(ImmediateLocation::FieldAccess { expression }) = &context.completion_location else {
        return Some(());
    };
    let source_analyzer = context.sema.analyze(context.container?);
    let r#type = source_analyzer.type_of_expression(&expression.expression()?)?;

    let field_completion_item =
        |name| CompletionItem::new(CompletionItemKind::Field, context.source_range(), name).build();

    match r#type.kind(context.db).unref(context.db).as_ref() {
        TyKind::Vector(vector) => {
            let size = vector.size.as_u8() as usize;
            debug_assert!(
                (MIN_VECTOR_SIZE..=MAX_VECTOR_SIZE).contains(&size),
                "Invalid vector size: {size}"
            );
            let field_text = expression
                .name_ref()
                .map(|name| name.text().to_string())
				// It should never be `None` because `x.$0` gets parsed as `Some("")`.
                .unwrap_or_default();

            if is_swizzleable(&field_text) {
                let possible_swizzles = possible_swizzles(size, &field_text);
                let suggestions = possible_swizzles.enumerate().map(move |(index, label)| {
                    CompletionItem::new(CompletionItemKind::Field, context.source_range(), label)
                        .with_relevance(CompletionRelevance {
                            swizzle_index: Some(index),
                            ..Default::default()
                        })
                        .build()
                });
                accumulator.add_all(suggestions);
            }
        },
        TyKind::Matrix(_) => return None,
        TyKind::Struct(r#struct) => {
            let r#struct = context.db.struct_data(*r#struct);
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

fn is_swizzleable(field_text: &str) -> bool {
    if !(0..=MAX_VECTOR_SIZE).contains(&field_text.len()) {
        return false;
    }

    let is_rgba = field_text
        .chars()
        .all(|c| matches!(c, 'r' | 'g' | 'b' | 'a'));
    let is_xyzw = field_text
        .chars()
        .all(|c| matches!(c, 'x' | 'y' | 'z' | 'w'));

    is_rgba || is_xyzw
}

const MIN_VECTOR_SIZE: usize = 2;
const MAX_VECTOR_SIZE: usize = 4;

/// https://www.w3.org/TR/WGSL/#syntax-swizzle_name
const SWIZZLE_SETS: &[&str] = &["xyzw", "rgba"];

fn possible_swizzles(
    max_length: usize,
    field_text: &str,
) -> impl Iterator<Item = String> {
    SWIZZLE_SETS
        .iter()
        .filter_map(move |swizzle_set| swizzler(swizzle_set, field_text, max_length))
        .flat_map(|swizzle| swizzle.into_iter())
        .chain(std::iter::once(field_text.to_string()))
        .filter(|swizzle| !swizzle.is_empty())
}

fn swizzler(
    swizzle: &&str,
    field_text: &str,
    max_length: usize,
) -> Option<impl std::iter::Iterator<Item = String>> {
    // Do not show "rgb" swizzles for "xyz"
    // and do not suggest further changes for invalid swizzles
    let characters_allowed = field_text.is_empty()
        || (field_text.len() < MAX_VECTOR_SIZE && swizzle.chars().any(|v| field_text.contains(v)));

    if characters_allowed {
        Some(
            swizzle[0..max_length]
                .chars()
                .map(move |next_character| format!("{field_text}{next_character}")),
        )
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn valid_swizzle_string() -> impl Strategy<Value = String> {
        prop_oneof![
            prop::collection::vec(prop::sample::select(vec!['r', 'g', 'b', 'a']), 0..=4)
                .prop_map(|v| v.into_iter().collect()),
            prop::collection::vec(prop::sample::select(vec!['x', 'y', 'z', 'w']), 0..=4)
                .prop_map(|v| v.into_iter().collect()),
        ]
    }

    proptest! {
        #[test]
        fn accepts_valid_swizzles(s in valid_swizzle_string()) {
            prop_assert!(is_swizzleable(&s), "Expected '{}' to be valid", s);
        }
    }

    #[test]
    fn test_possible_swizzles() {
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
    fn test_swizzler() {
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
