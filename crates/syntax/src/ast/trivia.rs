use parser::{SyntaxKind, SyntaxNode, SyntaxToken};

use crate::{AstNode as _, AstToken};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Comment {
    syntax: SyntaxToken,
}
impl AstToken for Comment {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LineEndingComment || kind == SyntaxKind::BlockComment
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}

impl Comment {
    #[must_use]
    pub fn kind(&self) -> CommentKind {
        CommentKind::from_text(self.text())
    }

    #[must_use]
    pub fn is_doc(&self) -> bool {
        self.kind().doc.is_some()
    }

    #[must_use]
    pub fn is_inner(&self) -> bool {
        self.kind().doc == Some(CommentPlacement::Inner)
    }

    #[must_use]
    pub fn is_outer(&self) -> bool {
        self.kind().doc == Some(CommentPlacement::Outer)
    }

    /// Extracts the prefix of the comment.
    ///
    /// Precondition:
    /// The comment node needs to have text that is a comment.
    ///
    /// # Panics
    /// When called with text that does not immediately start with a comment marker.
    #[must_use]
    pub fn prefix(&self) -> &'static str {
        let &(prefix, _) = CommentKind::BY_PREFIX
            .iter()
            .find(|&(prefix, kind)| self.kind() == *kind && self.text().starts_with(prefix))
            .unwrap();
        prefix
    }

    /// Returns the textual content of a doc comment node as a single string with prefix and suffix
    /// removed.
    #[must_use]
    pub fn doc_comment(&self) -> Option<&str> {
        let kind = self.kind();
        match kind {
            CommentKind {
                shape,
                doc: Some(_),
            } => {
                let prefix = kind.prefix();
                let text = &self.text()[prefix.len()..];
                let text = if shape == CommentShape::Block {
                    text.strip_suffix("*/").unwrap_or(text)
                } else {
                    text
                };
                Some(text)
            },
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CommentKind {
    pub shape: CommentShape,
    pub doc: Option<CommentPlacement>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommentShape {
    Line,
    Block,
}

impl CommentShape {
    #[must_use]
    pub fn is_line(self) -> bool {
        self == Self::Line
    }

    #[must_use]
    pub fn is_block(self) -> bool {
        self == Self::Block
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommentPlacement {
    Inner,
    Outer,
}

impl CommentKind {
    const BY_PREFIX: [(&'static str, Self); 9] = [
        (
            "/**/",
            Self {
                shape: CommentShape::Block,
                doc: None,
            },
        ),
        (
            "/***",
            Self {
                shape: CommentShape::Block,
                doc: None,
            },
        ),
        (
            "////",
            Self {
                shape: CommentShape::Line,
                doc: None,
            },
        ),
        (
            "///",
            Self {
                shape: CommentShape::Line,
                doc: Some(CommentPlacement::Outer),
            },
        ),
        (
            "//!",
            Self {
                shape: CommentShape::Line,
                doc: Some(CommentPlacement::Inner),
            },
        ),
        (
            "/**",
            Self {
                shape: CommentShape::Block,
                doc: Some(CommentPlacement::Outer),
            },
        ),
        (
            "/*!",
            Self {
                shape: CommentShape::Block,
                doc: Some(CommentPlacement::Inner),
            },
        ),
        (
            "//",
            Self {
                shape: CommentShape::Line,
                doc: None,
            },
        ),
        (
            "/*",
            Self {
                shape: CommentShape::Block,
                doc: None,
            },
        ),
    ];

    /// Constructs a [`CommentKind`] from text.
    ///
    /// Precondition:
    /// The text needs to start with a comment.
    ///
    /// # Panics
    /// When called with text that does not immediately start with a comment marker.
    #[must_use]
    pub(crate) fn from_text(text: &str) -> Self {
        let &(_prefix, kind) = Self::BY_PREFIX
            .iter()
            .find(|&(prefix, _kind)| text.starts_with(prefix))
            .unwrap();
        kind
    }

    /// Extracts the prefix of the comment.
    ///
    /// # Panics
    /// Cannot panic, by construction [`Self::BY_PREFIX`] should cover all cases
    #[must_use]
    pub fn prefix(self) -> &'static str {
        let &(prefix, _) = Self::BY_PREFIX
            .iter()
            .rev()
            .find(|(_, kind)| *kind == self)
            .unwrap();
        prefix
    }
}

#[derive(Clone, Debug)]
pub struct Whitespace {
    syntax: SyntaxToken,
}
impl AstToken for Whitespace {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Blankspace
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
impl Whitespace {
    #[must_use]
    pub fn spans_multiple_lines(&self) -> bool {
        let text = self.text();
        text.find('\n')
            .is_some_and(|index| text[index + 1..].contains('\n'))
    }
}
