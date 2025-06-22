//! Representation of a `TextEdit`.
//!
//! `rust-analyzer` never mutates text itself and only sends diffs to clients,
//! so `TextEdit` is the ultimate representation of the work done by
//! rust-analyzer.

use itertools::Itertools as _;
use rowan::{TextRange, TextSize};
use std::{cmp::max, iter, slice, vec};

use crate::source_change::ChangeAnnotationId;

/// A single "atomic" change to text
///
/// Must not overlap with other [`InsertDelete`]s
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InsertDelete {
    pub insert: String,
    /// Refers to offsets in the original text
    pub delete: TextRange,
}

#[derive(Default, Debug, Clone)]
pub struct TextEdit {
    /// Invariant: disjoint and sorted by `delete`.
    insert_deletes: Vec<InsertDelete>,
    annotation: Option<ChangeAnnotationId>,
}

#[derive(Debug, Default, Clone)]
pub struct TextEditBuilder {
    insert_deletes: Vec<InsertDelete>,
    annotation: Option<ChangeAnnotationId>,
}

impl InsertDelete {
    #[must_use]
    pub const fn insert(
        offset: TextSize,
        text: String,
    ) -> Self {
        Self::replace(TextRange::empty(offset), text)
    }
    #[must_use]
    pub const fn delete(range: TextRange) -> Self {
        Self::replace(range, String::new())
    }
    #[must_use]
    pub const fn replace(
        range: TextRange,
        replace_with: String,
    ) -> Self {
        Self {
            delete: range,
            insert: replace_with,
        }
    }

    pub fn apply(
        &self,
        text: &mut String,
    ) {
        let start: usize = self.delete.start().into();
        let end: usize = self.delete.end().into();
        text.replace_range(start..end, &self.insert);
    }
}

impl TextEdit {
    #[must_use]
    pub fn builder() -> TextEditBuilder {
        TextEditBuilder::default()
    }

    #[must_use]
    pub fn insert(
        offset: TextSize,
        text: String,
    ) -> Self {
        let mut builder = Self::builder();
        builder.insert(offset, text);
        builder.finish()
    }

    #[must_use]
    pub fn delete(range: TextRange) -> Self {
        let mut builder = Self::builder();
        builder.delete(range);
        builder.finish()
    }

    #[must_use]
    pub fn replace(
        range: TextRange,
        replace_with: String,
    ) -> Self {
        let mut builder = Self::builder();
        builder.replace(range, replace_with);
        builder.finish()
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.insert_deletes.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.insert_deletes.is_empty()
    }

    pub fn iter(&self) -> slice::Iter<'_, InsertDelete> {
        self.into_iter()
    }

    pub fn apply(
        &self,
        text: &mut String,
    ) {
        match self.len() {
            0 => return,
            1 => {
                self.insert_deletes[0].apply(text);
                return;
            },
            _ => (),
        }

        let text_size = TextSize::of(&*text);
        let mut total_len = text_size;
        let mut max_total_len = text_size;
        for insert_delete in &self.insert_deletes {
            total_len += TextSize::of(&insert_delete.insert);
            total_len -= insert_delete.delete.len();
            max_total_len = max(max_total_len, total_len);
        }

        if let Some(additional) = max_total_len.checked_sub(text_size) {
            text.reserve(additional.into());
        }

        for insert_delete in self.insert_deletes.iter().rev() {
            insert_delete.apply(text);
        }

        debug_assert!(TextSize::of(&*text) == total_len);
    }

    pub fn union(
        &mut self,
        other: Self,
    ) -> Result<(), Self> {
        let iter_merge = self.iter().merge_by(other.iter(), |left, right| {
            left.delete.start() <= right.delete.start()
        });
        if !check_disjoint(&mut iter_merge.clone()) {
            return Err(other);
        }

        // Only dedup deletions and replacements, keep all insertions
        self.insert_deletes = iter_merge
            .dedup_by(|first, second| first == second && !first.delete.is_empty())
            .cloned()
            .collect();
        Ok(())
    }

    #[must_use]
    pub fn apply_to_offset(
        &self,
        offset: TextSize,
    ) -> Option<TextSize> {
        let mut result = offset;
        for insert_delete in &self.insert_deletes {
            if insert_delete.delete.start() >= offset {
                break;
            }
            if offset < insert_delete.delete.end() {
                return None;
            }
            result += TextSize::of(&insert_delete.insert);
            result -= insert_delete.delete.len();
        }
        Some(result)
    }

    // pub(crate) fn set_annotation(
    //     &mut self,
    //     conflict_annotation: Option<ChangeAnnotationId>,
    // ) {
    //     self.annotation = conflict_annotation;
    // }

    #[must_use]
    pub const fn change_annotation(&self) -> Option<ChangeAnnotationId> {
        self.annotation
    }
}

impl IntoIterator for TextEdit {
    type Item = InsertDelete;
    type IntoIter = vec::IntoIter<InsertDelete>;

    fn into_iter(self) -> Self::IntoIter {
        self.insert_deletes.into_iter()
    }
}

impl<'item> IntoIterator for &'item TextEdit {
    type Item = &'item InsertDelete;
    type IntoIter = slice::Iter<'item, InsertDelete>;

    fn into_iter(self) -> Self::IntoIter {
        self.insert_deletes.iter()
    }
}

impl TextEditBuilder {
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.insert_deletes.is_empty()
    }
    pub fn replace(
        &mut self,
        range: TextRange,
        replace_with: String,
    ) {
        self.insert_delete(InsertDelete::replace(range, replace_with));
    }
    pub fn delete(
        &mut self,
        range: TextRange,
    ) {
        self.insert_delete(InsertDelete::delete(range));
    }

    pub fn insert(
        &mut self,
        offset: TextSize,
        text: String,
    ) {
        self.insert_delete(InsertDelete::insert(offset, text));
    }

    #[must_use]
    pub fn finish(self) -> TextEdit {
        let Self {
            mut insert_deletes,
            annotation,
        } = self;
        assert_disjoint_or_equal(&mut insert_deletes);
        insert_deletes = coalesce_insert_deletes(insert_deletes);
        TextEdit {
            insert_deletes,
            annotation,
        }
    }

    #[must_use]
    pub fn invalidates_offset(
        &self,
        offset: TextSize,
    ) -> bool {
        self.insert_deletes
            .iter()
            .any(|insert_delete| insert_delete.delete.contains_inclusive(offset))
    }

    pub fn insert_delete(
        &mut self,
        insert_delete: InsertDelete,
    ) {
        self.insert_deletes.push(insert_delete);
        if self.insert_deletes.len() <= 16 {
            assert_disjoint_or_equal(&mut self.insert_deletes);
        }
    }
}

fn assert_disjoint_or_equal(insert_deletes: &mut [InsertDelete]) {
    assert!(check_disjoint_and_sort(insert_deletes));
}

fn check_disjoint_and_sort(insert_deletes: &mut [InsertDelete]) -> bool {
    insert_deletes
        .sort_by_key(|insert_delete| (insert_delete.delete.start(), insert_delete.delete.end()));
    check_disjoint(&mut insert_deletes.iter())
}

fn check_disjoint<'item, I>(insert_deletes: &mut I) -> bool
where
    I: iter::Iterator<Item = &'item InsertDelete> + Clone,
{
    #[expect(clippy::suspicious_operation_groupings, reason = "intentional logic")]
    insert_deletes
        .clone()
        .zip(insert_deletes.skip(1))
        .all(|(left, right)| (left.delete.end() <= right.delete.start()) || left == right)
}

fn coalesce_insert_deletes(insert_deletes: Vec<InsertDelete>) -> Vec<InsertDelete> {
    insert_deletes
        .into_iter()
        .coalesce(|mut first, second| {
            if first.delete.end() == second.delete.start() {
                first.insert.push_str(&second.insert);
                first.delete = TextRange::new(first.delete.start(), second.delete.end());
                Ok(first)
            } else {
                Err((first, second))
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::{TextEdit, TextEditBuilder, TextRange};

    fn range(
        start: u32,
        end: u32,
    ) -> TextRange {
        TextRange::new(start.into(), end.into())
    }

    #[test]
    fn test_apply() {
        let mut text = "_11h1_2222_xx3333_4444_6666".to_owned();
        let mut builder = TextEditBuilder::default();
        builder.replace(range(3, 4), "1".to_owned());
        builder.delete(range(11, 13));
        builder.insert(22.into(), "_5555".to_owned());

        let text_edit = builder.finish();
        text_edit.apply(&mut text);

        assert_eq!(text, "_1111_2222_3333_4444_5555_6666");
    }

    #[test]
    fn test_union() {
        let mut edit1 = TextEdit::delete(range(7, 11));
        let mut builder = TextEditBuilder::default();
        builder.delete(range(1, 5));
        builder.delete(range(13, 17));

        let edit2 = builder.finish();
        edit1.union(edit2).unwrap();
        assert_eq!(edit1.insert_deletes.len(), 3);
    }

    #[test]
    fn test_union_with_duplicates() {
        let mut builder1 = TextEditBuilder::default();
        builder1.delete(range(7, 11));
        builder1.delete(range(13, 17));

        let mut builder2 = TextEditBuilder::default();
        builder2.delete(range(1, 5));
        builder2.delete(range(13, 17));

        let mut edit1 = builder1.finish();
        let edit2 = builder2.finish();
        edit1.union(edit2).unwrap();
        assert_eq!(edit1.insert_deletes.len(), 3);
    }

    #[test]
    fn test_union_panics() {
        let mut edit1 = TextEdit::delete(range(7, 11));
        let edit2 = TextEdit::delete(range(9, 13));
        assert!(edit1.union(edit2).is_err());
    }

    #[test]
    fn test_coalesce_disjoint() {
        let mut builder = TextEditBuilder::default();
        builder.replace(range(1, 3), "aa".into());
        builder.replace(range(5, 7), "bb".into());
        let edit = builder.finish();

        assert_eq!(edit.insert_deletes.len(), 2);
    }

    #[test]
    fn test_coalesce_adjacent() {
        let mut builder = TextEditBuilder::default();
        builder.replace(range(1, 3), "aa".into());
        builder.replace(range(3, 5), "bb".into());

        let edit = builder.finish();
        assert_eq!(edit.insert_deletes.len(), 1);
        assert_eq!(edit.insert_deletes[0].insert, "aabb");
        assert_eq!(edit.insert_deletes[0].delete, range(1, 5));
    }

    #[test]
    fn test_coalesce_adjacent_series() {
        let mut builder = TextEditBuilder::default();
        builder.replace(range(1, 3), "au".into());
        builder.replace(range(3, 5), "www".into());
        builder.replace(range(5, 8), String::new());
        builder.replace(range(8, 9), "ub".into());

        let edit = builder.finish();
        assert_eq!(edit.insert_deletes.len(), 1);
        assert_eq!(edit.insert_deletes[0].insert, "auwwwub");
        assert_eq!(edit.insert_deletes[0].delete, range(1, 9));
    }
}
