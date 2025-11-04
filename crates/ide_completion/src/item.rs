use std::fmt;

use base_db::TextRange;
use ide_db::text_edit::TextEdit;
use ide_db::{RootDatabase, SnippetCap};
use itertools::Itertools as _;
use smallvec::SmallVec;
use smol_str::{SmolStr, format_smolstr};
use std::mem;
use stdx::never;


/// `CompletionItem` describes a single completion entity which expands to 1 or more entries in the
/// editor pop-up.
///
/// It is basically a POD with various properties. To construct a [`CompletionItem`],
/// use [`Builder::new`] method and the [`Builder`] struct.
#[derive(Clone)]
#[non_exhaustive]
pub struct CompletionItem {
    /// Label in the completion pop up which identifies completion.
    pub label: CompletionItemLabel,

    /// Range of identifier that is being completed.
    ///
    /// It should be used primarily for UI, but we also use this to convert
    /// generic `TextEdit` into LSP's completion edit (see conv.rs).
    ///
    /// `source_range` must contain the completion offset. `text_edit` should
    /// start with what `source_range` points to, or `VSCode` will filter out the
    /// completion silently.
    pub source_range: TextRange,
    /// What happens when user selects this item.
    ///
    /// Typically, replaces `source_range` with new identifier.
    pub text_edit: TextEdit,
    pub is_snippet: bool,

    /// What item (struct, function, etc) are we completing.
    pub kind: CompletionItemKind,

    /// Lookup is used to check if completion item indeed can complete current
    /// ident.
    ///
    /// That is, in `foo.bar$0` lookup of `abracadabra` will be accepted (it
    /// contains `bar` sub sequence), and `quux` will rejected.
    pub lookup: SmolStr,

    /// Additional info to show in the UI pop up.
    pub detail: Option<String>,
    // pub documentation: Option<Documentation>,
    /// Whether this item is marked as deprecated
    pub deprecated: bool,

    /// If completing a function call, ask the editor to show parameter popup
    /// after completion.
    pub trigger_call_info: bool,

    /// We use this to sort completion. Relevance records facts like "do the
    /// types align precisely?". We can't sort by relevances directly, they are
    /// only partially ordered.
    ///
    /// Note that Relevance ignores fuzzy match score. We compute Relevance for
    /// all possible items, and then separately build an ordered completion list
    /// based on relevance and fuzzy matching with the already typed identifier.
    pub relevance: CompletionRelevance,

    // /// Indicates that a reference or mutable reference to this variable is a
    // /// possible match.
    // // FIXME: We shouldn't expose Mutability here (that is HIR types at all), its fine for now though
    // // until we have more splitting completions in which case we should think about
    // // generalizing this. See https://github.com/rust-lang/rust-analyzer/issues/12571
    // pub ref_match: Option<(CompletionItemRefMode, TextSize)>,
    /// The import data to add to completion's edits.
    pub import_to_add: SmallVec<[String; 1]>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompletionItemLabel {
    /// The primary label for the completion item.
    pub primary: SmolStr,
    /// The left detail for the completion item, usually rendered right next to the primary label.
    pub detail_left: Option<String>,
    /// The right detail for the completion item, usually rendered right aligned at the end of the completion item.
    pub detail_right: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionItemKind {
    Field,
    Function,
    Variable,
    Keyword,
    Snippet,
    Constant,
    Struct,
    Module,
    TypeAlias,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct CompletionRelevance {
    /// This is set when the identifier being completed matches up with the name that is expected,
    /// like in a function argument.
    ///
    /// ```ignore
    /// fn foo(spam: String) {}
    /// fn main() {
    ///     let spam = 92;
    ///     foo($0) // name of local matches the name of param
    /// }
    /// ```
    pub exact_name_match: bool,
    /// See [`CompletionRelevanceTypeMatch`].
    pub type_match: Option<CompletionRelevanceTypeMatch>,
    /// Set for local variables.
    ///
    /// ```ignore
    /// fn foo(a: u32) {
    ///     let b = 0;
    ///     $0 // `a` and `b` are local
    /// }
    /// ```
    pub is_local: bool,
    /// This is set when an import is suggested in a use item whose name is already imported.
    pub is_name_already_imported: bool,
    /// This is set for completions that will insert a `use` item.
    pub requires_import: bool,
    /// Set for item completions that are private but in the workspace.
    pub is_private_editable: bool,
    /// Set for postfix snippet item completions
    pub postfix_match: Option<CompletionRelevancePostfixMatch>,
    /// This is set for items that are function (associated or method)
    pub function: Option<CompletionRelevanceFn>,
    /// true when there is an `await.method()` or `iter().method()` completion.
    pub is_skipping_completion: bool,

    // builtins are shown with relatively low priority
    pub is_builtin: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CompletionRelevancePostfixMatch {
    /// Set in cases when item is postfix, but not exact
    NonExact,
    /// This is set in cases like these:
    ///
    /// ```ignore
    /// (a > b).not$0
    /// ```
    ///
    /// Basically, we want to guarantee that postfix snippets always takes
    /// precedence over everything else.
    Exact,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CompletionRelevanceFn {
    pub has_params: bool,
    pub has_self_param: bool,
    pub return_type: CompletionRelevanceReturnType,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CompletionRelevanceReturnType {
    Other,
    /// Returns the Self type of the impl/trait
    DirectConstructor,
    /// Returns something that indirectly constructs the `Self` type of the impl/trait e.g. `Result<Self, ()>`, `Option<Self>`
    Constructor,
    /// Returns a possible builder for the type
    Builder,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CompletionRelevanceTypeMatch {
    /// This is set in cases like these:
    ///
    /// ```ignore
    /// enum Option<T> { Some(T), None }
    /// fn foo(a: Option<u32>) {}
    /// fn main {
    ///     foo(Option::N$0) // type `Option<T>` could unify with `Option<u32>`
    /// }
    /// ```
    CouldUnify,
    /// This is set in cases like these:
    ///
    /// ```ignore
    /// fn foo(spam: String) {}
    /// fn main {
    ///     let foo = String::new();
    ///     foo($0) // type of local matches the type of parameter
    /// }
    /// ```
    Exact,
}

impl CompletionRelevance {
    /// Provides a relevance score. Higher values are more relevant.
    ///
    /// The absolute value of the relevance score is not meaningful, for
    /// example a value of `BASE_SCORE` doesn't mean "not relevant", rather
    /// it means "least relevant". The score value should only be used
    /// for relative ordering.
    ///
    /// See `is_relevant` if you need to make some judgment about score
    /// in an absolute sense.
    ///
    /// # Panics
    ///
    ///
    const BASE_SCORE: u32 = u32::MAX.checked_div(2).unwrap();

    #[must_use]
    pub fn score(self) -> u32 {
        let mut score = Self::BASE_SCORE;
        let Self {
            exact_name_match,
            type_match,
            is_local,
            is_name_already_imported,
            requires_import,
            is_private_editable,
            postfix_match,
            function,
            is_skipping_completion,
            is_builtin,
        } = self;

        // only applicable for completions within use items
        // lower rank for conflicting import names
        if is_name_already_imported {
            score -= 1;
        }
        // slightly prefer locals
        if is_local {
            score += 1;
        }

        // lower rank private things
        if !is_private_editable {
            score += 1;
        }

        // Lower rank for completions that skip `await` and `iter()`.
        if is_skipping_completion {
            score -= 7;
        }

        // lower rank for items that need an import
        if requires_import {
            score -= 1;
        }
        if exact_name_match {
            score += 20;
        }
        match postfix_match {
            Some(CompletionRelevancePostfixMatch::Exact) => score += 100,
            Some(CompletionRelevancePostfixMatch::NonExact) => score -= 5,
            None => (),
        }
        score += match type_match {
            Some(CompletionRelevanceTypeMatch::Exact) => 18,
            Some(CompletionRelevanceTypeMatch::CouldUnify) => 5,
            None => 0,
        };
        if self.is_builtin {
            score += 100;
        }
        if let Some(function) = function {
            let mut fn_score = match function.return_type {
                CompletionRelevanceReturnType::DirectConstructor => 15,
                CompletionRelevanceReturnType::Builder => 10,
                CompletionRelevanceReturnType::Constructor => 5,
                CompletionRelevanceReturnType::Other => 0_u32,
            };

            // When a fn is bumped due to return type:
            // Bump Constructor or Builder methods with no arguments,
            // over them than with self arguments
            if function.has_params {
                // bump associated functions
                fn_score = fn_score.saturating_sub(1);
            } else if function.has_self_param {
                // downgrade methods (below Constructor)
                fn_score = fn_score.min(1);
            }

            score += fn_score;
        }
        score
    }

    /// Returns true when the score for this threshold is above
    /// some threshold such that we think it is especially likely
    /// to be relevant.
    #[must_use]
    pub fn is_relevant(self) -> bool {
        self.score() > Self::BASE_SCORE
    }
}

impl CompletionItem {
    #[expect(clippy::new_ret_no_self, reason = "builder")]
    pub(crate) fn new(
        kind: impl Into<CompletionItemKind>,
        source_range: TextRange,
        label: impl Into<SmolStr>,
        // edition: Edition,
    ) -> Builder {
        let label = label.into();
        Builder {
            source_range,
            trait_name: None,
            doc_aliases: vec![],
            label,
            insert_text: None,
            is_snippet: false,
            // documentation: None,
            detail: None,
            lookup: None,
            kind: kind.into(),
            text_edit: None,
            deprecated: false,
            trigger_call_info: false,
            // ref_match: None,
            // imports_to_add: Default::default(),
            relevance: CompletionRelevance::default(),
            // edition,
        }
    }

    /// What user sees in pop-up in the UI.
    #[must_use]
    pub const fn label(&self) -> &CompletionItemLabel {
        &self.label
    }

    #[must_use]
    pub const fn source_range(&self) -> TextRange {
        self.source_range
    }

    #[must_use]
    pub const fn text_edit(&self) -> &TextEdit {
        &self.text_edit
    }

    /// Whether `text_edit` is a snippet (contains `$0` markers).
    #[must_use]
    pub const fn is_snippet(&self) -> bool {
        self.is_snippet
    }

    /// Short one-line additional information, like a type
    #[must_use]
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    // /// A doc-comment
    // pub fn documentation(&self) -> Option<Documentation> {
    //     self.documentation.clone()
    // }

    /// What string is used for filtering.
    #[must_use]
    pub fn lookup(&self) -> &str {
        self.lookup.as_str()
    }

    #[must_use]
    pub const fn kind(&self) -> CompletionItemKind {
        self.kind
    }

    // pub fn deprecated(&self) -> bool {
    //     self.deprecated
    // }

    #[must_use]
    pub const fn relevance(&self) -> CompletionRelevance {
        self.relevance
    }

    // pub fn trigger_call_info(&self) -> bool {
    //     self.trigger_call_info
    // }

    // pub fn ref_match(&self) -> Option<(Mutability, CompletionRelevance)> {
    // // Relevance of the ref match should be the same as the original
    // // match, but with exact type match set because self.ref_match
    // // is only set if there is an exact type match.
    // let mut relevance = self.relevance;
    // relevance.type_match = Some(CompletionRelevanceTypeMatch::Exact);

    // self.ref_match.map(|mutability| (mutability, relevance))
    // }

    // pub fn imports_to_add(&self) -> &[ImportEdit] {
    // &self.import_to_add
    // }
}

/// A helper to make `CompletionItem`s.
#[must_use]
#[derive(Clone)]
pub(crate) struct Builder {
    source_range: TextRange,
    // imports_to_add: SmallVec<[LocatedImport; 1]>,
    trait_name: Option<SmolStr>,
    doc_aliases: Vec<SmolStr>,
    label: SmolStr,
    insert_text: Option<String>,
    is_snippet: bool,
    detail: Option<String>,
    // documentation: Option<Documentation>,
    lookup: Option<SmolStr>,
    kind: CompletionItemKind,
    text_edit: Option<TextEdit>,
    deprecated: bool,
    trigger_call_info: bool,
    relevance: CompletionRelevance,
    // ref_match: Option<(CompletionItemRefMode, TextSize)>,
    // edition: Edition,
}

// We use custom debug for CompletionItem to make snapshot tests more readable.
impl fmt::Debug for CompletionItem {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let mut debug_struct = formatter.debug_struct("CompletionItem");
        debug_struct
            .field("label", &self.label.primary)
            .field("detail_left", &self.label.detail_left)
            .field("detail_right", &self.label.detail_right)
            .field("source_range", &self.source_range);
        if self.text_edit.len() == 1 {
            let atom = self.text_edit.iter().next().unwrap();
            debug_struct.field("delete", &atom.delete);
            debug_struct.field("insert", &atom.insert);
        } else {
            debug_struct.field("text_edit", &self.text_edit);
        }
        debug_struct.field("kind", &self.kind);
        if self.lookup() != self.label.primary {
            debug_struct.field("lookup", &self.lookup());
        }
        if let Some(detail) = &self.detail {
            debug_struct.field("detail", &detail);
        }
        // if let Some(documentation) = &self.documentation {
        //     s.field("documentation", &documentation);
        // }
        if self.deprecated {
            debug_struct.field("deprecated", &true);
        }

        if self.relevance != CompletionRelevance::default() {
            debug_struct.field("relevance", &self.relevance);
        }

        // if let Some((ref_mode, offset)) = self.ref_match {
        //     let prefix = match ref_mode {
        //         CompletionItemRefMode::Reference(mutability) => match mutability {
        //             Mutability::Shared => "&",
        //             Mutability::Mut => "&mut ",
        //         },
        //         CompletionItemRefMode::Dereference => "*",
        //     };
        //     s.field("ref_match", &format!("{prefix}@{offset:?}"));
        // }
        if self.trigger_call_info {
            debug_struct.field("trigger_call_info", &true);
        }
        debug_struct.finish_non_exhaustive()
    }
}

impl Builder {
    pub(crate) fn build(
        self,
        database: &RootDatabase,
    ) -> CompletionItem {
        let _p = tracing::info_span!("item::Builder::build").entered();

        let label = self.label;
        let mut lookup = self.lookup.unwrap_or_else(|| label.clone());
        let insert_text = self.insert_text.unwrap_or_else(|| label.to_string());

        let mut detail_left = None;
        if !self.doc_aliases.is_empty() {
            let doc_aliases = self.doc_aliases.iter().join(", ");
            detail_left = Some(format!("(alias {doc_aliases})"));
            let lookup_doc_aliases = self
                .doc_aliases
                .iter()
                // Don't include aliases in `lookup` that aren't valid identifiers as including
                // them results in weird completion filtering behavior e.g. `Partial>` matching
                // `PartialOrd` because it has an alias of ">".
                .filter(|alias| {
                    let mut chars = alias.chars();
                    chars.next().is_some_and(char::is_alphabetic)
                        && chars.all(|character| character.is_alphanumeric() || character == '_')
                })
                // Deliberately concatenated without separators as adding separators e.g.
                // `alias1, alias2` results in LSP clients continuing to display the completion even
                // after typing a comma or space.
                .join("");
            if !lookup_doc_aliases.is_empty() {
                lookup = format_smolstr!("{lookup}{lookup_doc_aliases}");
            }
        }
        // if let [import_edit] = &*self.imports_to_add {
        //     // snippets can have multiple imports, but normal completions only have up to one
        //     let detail_left = detail_left.get_or_insert_with(String::new);
        //     format_to!(
        //         detail_left,
        //         "{}(use {})",
        //         if detail_left.is_empty() { "" } else { " " },
        //         import_edit.import_path.display(database, self.edition)
        //     );
        // } else if let Some(trait_name) = self.trait_name {
        //     let detail_left = detail_left.get_or_insert_with(String::new);
        //     format_to!(
        //         detail_left,
        //         "{}(as {trait_name})",
        //         if detail_left.is_empty() { "" } else { " " },
        //     );
        // }

        let text_edit = match self.text_edit {
            Some(text_edit) => text_edit,
            None => TextEdit::replace(self.source_range, insert_text),
        };

        // let import_to_add = self
        //     .imports_to_add
        //     .into_iter()
        //     .map(|import| import.import_path.display(database, self.edition).to_string())
        //     .collect();

        CompletionItem {
            label: CompletionItemLabel {
                primary: label,
                detail_left,
                detail_right: self.detail.clone(),
            },
            source_range: self.source_range,
            text_edit,
            is_snippet: self.is_snippet,
            kind: self.kind,
            // documentation: self.documentation,
            lookup,
            detail: self.detail,
            deprecated: self.deprecated,
            trigger_call_info: self.trigger_call_info,
            relevance: self.relevance,
            // ref_match: self.ref_match,
            import_to_add: SmallVec::default(),
        }
    }

    pub(crate) fn lookup_by(
        &mut self,
        lookup: impl Into<SmolStr>,
    ) -> &mut Self {
        self.lookup = Some(lookup.into());
        self
    }

    pub(crate) fn label(
        &mut self,
        label: impl Into<SmolStr>,
    ) -> &mut Self {
        self.label = label.into();
        self
    }

    pub(crate) fn trait_name(
        &mut self,
        trait_name: SmolStr,
    ) -> &mut Self {
        self.trait_name = Some(trait_name);
        self
    }

    pub(crate) fn doc_aliases(
        &mut self,
        doc_aliases: Vec<SmolStr>,
    ) -> &mut Self {
        self.doc_aliases = doc_aliases;
        self
    }

    pub(crate) fn insert_text(
        &mut self,
        insert_text: impl Into<String>,
    ) -> &mut Self {
        self.insert_text = Some(insert_text.into());
        self
    }

    pub(crate) fn insert_snippet(
        &mut self,
        cap: SnippetCap,
        snippet: impl Into<String>,
    ) -> &mut Self {
        self.is_snippet = true;
        self.insert_text(snippet)
    }

    pub(crate) fn text_edit(
        &mut self,
        edit: TextEdit,
    ) -> &mut Self {
        self.text_edit = Some(edit);
        self
    }

    pub(crate) fn snippet_edit(
        &mut self,
        _cap: SnippetCap,
        edit: TextEdit,
    ) -> &mut Self {
        self.is_snippet = true;
        self.text_edit(edit)
    }

    pub(crate) fn detail(
        &mut self,
        detail: impl Into<String>,
    ) -> &mut Self {
        self.set_detail(Some(detail))
    }

    pub(crate) fn set_detail(
        &mut self,
        detail: Option<impl Into<String>>,
    ) -> &mut Self {
        self.detail = detail.map(Into::into);
        if let Some(detail) = &self.detail
            && never!(detail.contains('\n'), "multiline detail:\n{}", detail)
        {
            self.detail = Some(detail.split('\n').next().unwrap().to_owned());
        }
        self
    }

    // #[allow(unused)]
    // pub(crate) fn documentation(
    //     &mut self,
    //     docs: Documentation,
    // ) -> &mut Builder {
    //     self.set_documentation(Some(docs))
    // }

    // pub(crate) fn set_documentation(
    //     &mut self,
    //     docs: Option<Documentation>,
    // ) -> &mut Builder {
    //     self.documentation = docs;
    //     self
    // }

    pub(crate) const fn set_deprecated(
        &mut self,
        deprecated: bool,
    ) -> &mut Self {
        self.deprecated = deprecated;
        self
    }

    pub(crate) const fn set_relevance(
        &mut self,
        relevance: CompletionRelevance,
    ) -> &mut Self {
        self.relevance = relevance;
        self
    }

    pub(crate) fn with_relevance(
        &mut self,
        relevance: impl FnOnce(CompletionRelevance) -> CompletionRelevance,
    ) -> &mut Self {
        self.relevance = relevance(mem::take(&mut self.relevance));
        self
    }

    pub(crate) const fn trigger_call_info(&mut self) -> &mut Self {
        self.trigger_call_info = true;
        self
    }

    // pub(crate) fn add_import(
    //     &mut self,
    //     import_to_add: LocatedImport,
    // ) -> &mut Builder {
    //     self.imports_to_add.push(import_to_add);
    //     self
    // }

    // pub(crate) fn ref_match(
    //     &mut self,
    //     ref_mode: CompletionItemRefMode,
    //     offset: TextSize,
    // ) -> &mut Builder {
    //     self.ref_match = Some((ref_mode, offset));
    //     self
    // }
}
