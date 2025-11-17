use parser::{SyntaxKind, SyntaxNode, SyntaxToken};

use crate::{AstNode, AstToken};

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
    pub fn kind(&self) -> CommentKind {
        CommentKind::from_text(self.text())
    }

    pub fn is_doc(&self) -> bool {
        self.kind().doc.is_some()
    }

    pub fn is_inner(&self) -> bool {
        self.kind().doc == Some(CommentPlacement::Inner)
    }

    pub fn is_outer(&self) -> bool {
        self.kind().doc == Some(CommentPlacement::Outer)
    }

    pub fn prefix(&self) -> &'static str {
        let &(prefix, _kind) = CommentKind::BY_PREFIX
            .iter()
            .find(|&(prefix, kind)| self.kind() == *kind && self.text().starts_with(prefix))
            .unwrap();
        prefix
    }

    /// Returns the textual content of a doc comment node as a single string with prefix and suffix
    /// removed.
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
    pub fn is_line(self) -> bool {
        self == CommentShape::Line
    }

    pub fn is_block(self) -> bool {
        self == CommentShape::Block
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommentPlacement {
    Inner,
    Outer,
}

impl CommentKind {
    const BY_PREFIX: [(&'static str, CommentKind); 9] = [
        (
            "/**/",
            CommentKind {
                shape: CommentShape::Block,
                doc: None,
            },
        ),
        (
            "/***",
            CommentKind {
                shape: CommentShape::Block,
                doc: None,
            },
        ),
        (
            "////",
            CommentKind {
                shape: CommentShape::Line,
                doc: None,
            },
        ),
        (
            "///",
            CommentKind {
                shape: CommentShape::Line,
                doc: Some(CommentPlacement::Outer),
            },
        ),
        (
            "//!",
            CommentKind {
                shape: CommentShape::Line,
                doc: Some(CommentPlacement::Inner),
            },
        ),
        (
            "/**",
            CommentKind {
                shape: CommentShape::Block,
                doc: Some(CommentPlacement::Outer),
            },
        ),
        (
            "/*!",
            CommentKind {
                shape: CommentShape::Block,
                doc: Some(CommentPlacement::Inner),
            },
        ),
        (
            "//",
            CommentKind {
                shape: CommentShape::Line,
                doc: None,
            },
        ),
        (
            "/*",
            CommentKind {
                shape: CommentShape::Block,
                doc: None,
            },
        ),
    ];

    pub(crate) fn from_text(text: &str) -> CommentKind {
        let &(_prefix, kind) = CommentKind::BY_PREFIX
            .iter()
            .find(|&(prefix, _kind)| text.starts_with(prefix))
            .unwrap();
        kind
    }

    pub fn prefix(&self) -> &'static str {
        let &(prefix, _) = CommentKind::BY_PREFIX
            .iter()
            .rev()
            .find(|(_, kind)| kind == self)
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
    pub fn spans_multiple_lines(&self) -> bool {
        let text = self.text();
        text.find('\n')
            .is_some_and(|idx| text[idx + 1..].contains('\n'))
    }
}
