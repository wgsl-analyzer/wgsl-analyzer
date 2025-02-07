use always_assert::never;
use base_db::TextRange;
use text_edit::TextEdit;

/// `CompletionItem` describes a single completion variant in the editor pop-up.
/// It is basically a POD with various properties. To construct a
/// `CompletionItem`, use `new` method and the `Builder` struct.
#[derive(Clone, Debug)]
pub struct CompletionItem {
    /// Label in the completion pop up which identifies completion.
    label: String,

    /// Range of identifier that is being completed.
    ///
    /// It should be used primarily for UI, but we also use this to convert
    /// generic TextEdit into LSP's completion edit (see conv.rs).
    ///
    /// `source_range` must contain the completion offset. `insert_text` should
    /// start with what `source_range` points to, or VSCode will filter out the
    /// completion silently.
    source_range: TextRange,

    /// What happens when user selects this item.
    ///
    /// Typically, replaces `source_range` with new identifier.
    text_edit: TextEdit,

    /// Whether the item is a snippet.
    is_snippet: bool,

    /// What item (struct, function, etc) are we completing.
    kind: CompletionItemKind,

    /// Lookup is used to check if completion item indeed can complete current
    /// ident.
    ///
    /// That is, in `foo.bar$0` lookup of `abracadabra` will be accepted (it
    /// contains `bar` sub sequence), and `quux` will rejected.
    lookup: Option<String>,

    /// Additional info to show in the UI pop up.
    detail: Option<String>,

    /// We use this to sort completion. Relevance records facts like "do the
    /// types align precisely?". We can't sort by relevances directly, they are
    /// only partially ordered.
    ///
    /// Note that Relevance ignores fuzzy match score. We compute Relevance for
    /// all possible items, and then separately build an ordered completion list
    /// based on relevance and fuzzy matching with the already typed identifier.
    relevance: CompletionRelevance,
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
    /// This is set in cases like these:
    ///
    /// ```ignore
    /// fn f(spam: String) {}
    /// fn main {
    ///     let spam = 92;
    ///     f($0) // name of local matches the name of param
    /// }
    /// ```
    pub exact_name_match: bool,
    /// See CompletionRelevanceTypeMatch doc comments for cases where this is set.
    pub type_match: Option<CompletionRelevanceTypeMatch>,
    /// This is set in cases like these:
    ///
    /// ```ignore
    /// fn foo(a: u32) {
    ///     let b = 0;
    ///     $0 // `a` and `b` are local
    /// }
    /// ```
    pub is_local: bool,
    /// This is set in cases like these:
    ///
    /// ```ignore
    /// (a > b).not$0
    /// ```
    ///
    /// Basically, we want to guarantee that postfix snippets always takes
    /// precedence over everything else.
    pub exact_postfix_snippet_match: bool,

    // builtins are shown with relatively low priority
    pub is_builtin: bool,

    /// swizzles should be displayed in the correct order
    pub swizzle_index: Option<usize>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CompletionRelevanceTypeMatch {
    /// This is set in cases like these:
    ///
    /// ```ignore
    /// enum Option<T> { Some(T), None }
    /// fn f(a: Option<u32>) {}
    /// fn main {
    ///     f(Option::N$0) // type `Option<T>` could unify with `Option<u32>`
    /// }
    /// ```
    CouldUnify,
    /// This is set in cases like these:
    ///
    /// ```ignore
    /// fn f(spam: String) {}
    /// fn main {
    ///     let foo = String::new();
    ///     f($0) // type of local matches the type of param
    /// }
    /// ```
    Exact,
}

impl CompletionRelevance {
    /// Provides a relevance score. Higher values are more relevant.
    ///
    /// The absolute value of the relevance score is not meaningful, for
    /// example a value of 0 doesn't mean "not relevant", rather
    /// it means "least relevant". The score value should only be used
    /// for relative ordering.
    ///
    /// See is_relevant if you need to make some judgment about score
    /// in an absolute sense.
    pub fn score(&self) -> u32 {
        let mut score = u32::MAX / 2;

        if self.exact_name_match {
            score -= 200;
        }
        score -= match self.type_match {
            Some(CompletionRelevanceTypeMatch::Exact) => 400,
            Some(CompletionRelevanceTypeMatch::CouldUnify) => 300,
            None => 0,
        };
        if self.is_local {
            score -= 100;
        }
        if self.exact_postfix_snippet_match {
            score -= 10000;
        }

        if let Some(index) = self.swizzle_index {
            score += index as u32 + 10;
        }

        if self.is_builtin {
            score += 100;
        }

        score
    }
}

impl CompletionItem {
    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new(
        kind: CompletionItemKind,
        source_range: TextRange,
        label: impl Into<String>,
    ) -> Builder {
        let label = label.into();
        Builder {
            source_range,
            label,
            insert_text: None,
            is_snippet: false,
            // trait_name: None,
            detail: None,
            // documentation: None,
            lookup: None,
            kind,
            text_edit: None,
            // deprecated: false,
            // trigger_call_info: None,
            relevance: CompletionRelevance::default(),
            // ref_match: None,
            // imports_to_add: Default::default(),
        }
    }

    /// What user sees in pop-up in the UI.
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn source_range(&self) -> TextRange {
        self.source_range
    }

    pub fn text_edit(&self) -> &TextEdit {
        &self.text_edit
    }

    /// Whether `text_edit` is a snippet (contains `$0` markers).
    pub fn is_snippet(&self) -> bool {
        self.is_snippet
    }

    /// Short one-line additional information, like a type
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    // /// A doc-comment
    // pub fn documentation(&self) -> Option<Documentation> {
    //     self.documentation.clone()
    // }
    /// What string is used for filtering.
    pub fn lookup(&self) -> &str {
        self.lookup.as_deref().unwrap_or(&self.label)
    }

    pub fn kind(&self) -> CompletionItemKind {
        self.kind
    }

    // pub fn deprecated(&self) -> bool {
    //     self.deprecated
    // }

    pub fn relevance(&self) -> CompletionRelevance {
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
    // imports_to_add: SmallVec<[ImportEdit; 1]>,
    // trait_name: Option<String>,
    label: String,
    insert_text: Option<String>,
    is_snippet: bool,
    detail: Option<String>,
    // documentation: Option<Documentation>,
    lookup: Option<String>,
    kind: CompletionItemKind,
    text_edit: Option<TextEdit>,
    // deprecated: bool,
    // trigger_call_info: Option<bool>,
    relevance: CompletionRelevance,
    // ref_match: Option<Mutability>,
}

impl Builder {
    pub(crate) fn build(self) -> CompletionItem {
        let label = self.label;
        let lookup = self.lookup;
        let insert_text = self.insert_text;

        /*if let [import_edit] = &*self.imports_to_add {
            // snippets can have multiple imports, but normal completions only have up to one
            if let Some(original_path) = import_edit.import.original_path.as_ref() {
                lookup = lookup.or_else(|| Some(label.clone()));
                insert_text = insert_text.or_else(|| Some(label.clone()));
                format_to!(label, " (use {})", original_path)
            }
        } else if let Some(trait_name) = self.trait_name {
            insert_text = insert_text.or_else(|| Some(label.clone()));
            format_to!(label, " (as {})", trait_name)
        }*/

        let text_edit = match self.text_edit {
            Some(it) => it,
            None => TextEdit::replace(
                self.source_range,
                insert_text.unwrap_or_else(|| label.clone()),
            ),
        };

        CompletionItem {
            source_range: self.source_range,
            label,
            text_edit,
            is_snippet: self.is_snippet,
            detail: self.detail,
            // documentation: self.documentation,
            lookup,
            kind: self.kind,
            // deprecated: self.deprecated,
            // trigger_call_info: self.trigger_call_info.unwrap_or(false),
            relevance: self.relevance,
            // ref_match: self.ref_match,
            // import_to_add: self.imports_to_add,
        }
    }

    pub(crate) fn lookup_by(
        &mut self,
        lookup: impl Into<String>,
    ) -> &mut Builder {
        self.lookup = Some(lookup.into());
        self
    }

    pub(crate) fn label(
        &mut self,
        label: impl Into<String>,
    ) -> &mut Builder {
        self.label = label.into();
        self
    }

    // pub(crate) fn trait_name(&mut self, trait_name: impl Into<String>) -> &mut Builder {
    //     self.trait_name = Some(trait_name.into());
    //     self
    // }
    pub(crate) fn insert_text(
        &mut self,
        insert_text: impl Into<String>,
    ) -> &mut Builder {
        self.insert_text = Some(insert_text.into());
        self
    }

    // pub(crate) fn insert_snippet(
    //     &mut self,
    //     cap: SnippetCap,
    //     snippet: impl Into<String>,
    // ) -> &mut Builder {
    //     let _ = cap;
    //     self.is_snippet = true;
    //     self.insert_text(snippet)
    // }
    pub(crate) fn text_edit(
        &mut self,
        edit: TextEdit,
    ) -> &mut Builder {
        self.text_edit = Some(edit);
        self
    }

    // pub(crate) fn snippet_edit(&mut self, _cap: SnippetCap, edit: TextEdit) -> &mut Builder {
    //     self.is_snippet = true;
    //     self.text_edit(edit)
    // }
    pub(crate) fn detail(
        &mut self,
        detail: impl Into<String>,
    ) -> &mut Builder {
        self.set_detail(Some(detail))
    }

    pub(crate) fn set_detail(
        &mut self,
        detail: Option<impl Into<String>>,
    ) -> &mut Builder {
        self.detail = detail.map(Into::into);
        if let Some(detail) = &self.detail {
            if never!(detail.contains('\n'), "multiline detail:\n{}", detail) {
                self.detail = Some(detail.split_once('\n').map(|x| x.0).unwrap().to_string());
            }
        }
        self
    }

    // #[allow(unused)]
    // pub(crate) fn documentation(&mut self, docs: Documentation) -> &mut Builder {
    //     self.set_documentation(Some(docs))
    // }
    // pub(crate) fn set_documentation(&mut self, docs: Option<Documentation>) -> &mut Builder {
    //     self.documentation = docs.map(Into::into);
    //     self
    // }
    // pub(crate) fn set_deprecated(&mut self, deprecated: bool) -> &mut Builder {
    //     self.deprecated = deprecated;
    //     self
    // }
    pub(crate) fn set_relevance(
        &mut self,
        relevance: CompletionRelevance,
    ) -> &mut Builder {
        self.relevance = relevance;
        self
    }

    pub(crate) fn with_relevance(
        mut self,
        relevance: CompletionRelevance,
    ) -> Builder {
        self.set_relevance(relevance);
        self
    }
    // pub(crate) fn trigger_call_info(&mut self) -> &mut Builder {
    //     self.trigger_call_info = Some(true);
    //     self
    // }
    // pub(crate) fn add_import(&mut self, import_to_add: ImportEdit) -> &mut Builder {
    //     self.imports_to_add.push(import_to_add);
    //     self
    // }
    // pub(crate) fn ref_match(&mut self, mutability: Mutability) -> &mut Builder {
    //     self.ref_match = Some(mutability);
    //     self
    // }
}
