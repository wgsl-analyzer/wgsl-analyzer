use std::fmt;

use arrayvec::ArrayVec;
use base_db::TextRange;
use smol_str::SmolStr;
use vfs::FileId;

/// `NavigationTarget` represents an element in the editor's UI which you can
/// click on to navigate to a particular piece of code.
///
/// Typically, a `NavigationTarget` corresponds to some element in the source
/// code, like a function or a struct, but this is not strictly required.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NavigationTarget {
    pub file_id: FileId,
    /// Range which encompasses the whole element.
    ///
    /// Should include body, doc comments, attributes, etc.
    ///
    /// Clients should use this range to answer "is the cursor inside the
    /// element?" question.
    pub full_range: TextRange,
    /// A "most interesting" range within the `full_range`.
    ///
    /// Typically, `full_range` is the whole syntax node, including doc
    /// comments, and `focus_range` is the range of the identifier.
    ///
    /// Clients should place the cursor on this range when navigating to this target.
    ///
    /// This range must be contained within [`Self::full_range`].
    pub focus_range: Option<TextRange>,
    // // FIXME: Symbol
    // pub name: SmolStr,
    // pub kind: Option<SymbolKind>,
    // FIXME: Symbol
    pub container_name: Option<SmolStr>,
    pub description: Option<String>,
    // pub docs: Option<Documentation>,
    /// In addition to a `name` field, a `NavigationTarget` may also be aliased
    /// In such cases we want a `NavigationTarget` to be accessible by its alias
    // FIXME: Symbol
    pub alias: Option<SmolStr>,
}

impl fmt::Debug for NavigationTarget {
    fn fmt(
        &self,
        #[expect(clippy::min_ident_chars, reason = "trait impl")] f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let mut debug_struct = f.debug_struct("NavigationTarget");
        macro_rules! opt {
            ($($name:ident)*) => {$(
                if let Some(value) = &self.$name {
                    debug_struct.field(stringify!($name), value);
                }
            )*}
        }
        debug_struct
            .field("file_id", &self.file_id)
            .field("full_range", &self.full_range);
        opt!(focus_range);
        // debug_struct.field("name", &self.name);
        // opt!(kind container_name description docs);
        opt!(container_name description);
        debug_struct.finish_non_exhaustive()
    }
}

impl NavigationTarget {
    pub(crate) const fn from_syntax(
        file_id: FileId,
        // name: SmolStr,
        full_range: TextRange,
        focus_range: Option<TextRange>,
        // kind: SymbolKind,
    ) -> Self {
        Self {
            file_id,
            // name,
            // kind: Some(kind),
            full_range,
            focus_range,
            container_name: None,
            description: None,
            // docs: None,
            alias: None,
        }
    }

    #[must_use]
    pub fn focus_or_full_range(&self) -> TextRange {
        self.focus_range.unwrap_or(self.full_range)
    }
}

#[derive(Debug)]
pub struct UpmappingResult<T> {
    /// The macro call site.
    pub call_site: T,
    /// The macro definition site, if relevant.
    pub def_site: Option<T>,
}

impl<T> UpmappingResult<T> {
    pub fn call_site(self) -> T {
        self.call_site
    }

    pub fn collect<FI: FromIterator<T>>(self) -> FI {
        FI::from_iter(self)
    }
}

impl<T> IntoIterator for UpmappingResult<T> {
    type Item = T;

    type IntoIter = <ArrayVec<T, 2> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.def_site
            .into_iter()
            .chain(Some(self.call_site))
            .collect::<ArrayVec<_, 2>>()
            .into_iter()
    }
}

impl<T> UpmappingResult<T> {
    pub(crate) fn map<U>(
        self,
        function: impl Fn(T) -> U,
    ) -> UpmappingResult<U> {
        UpmappingResult {
            call_site: function(self.call_site),
            def_site: self.def_site.map(function),
        }
    }
}
