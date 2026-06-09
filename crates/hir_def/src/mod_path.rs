//! A lowering for `import`-paths (more generally, paths without angle-bracketed segments).

use std::{
    fmt::{self, Display as _},
    iter, range,
};

use smallvec::SmallVec;
use syntax::{
    AstNode,
    ast::{self, ImportRelative},
};

use crate::{database::DefDatabase, item_tree::Name};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModPath {
    kind: PathKind,
    segments: SmallVec<[Name; 1]>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PathKind {
    /// Either a library when used like `import foo::bar` or a plain variable name when used inline `foo`.
    Plain,
    /// `self::` is `Super(0)`.
    Super(u8),
    /// `package::`.
    Package,
}

impl PathKind {
    pub const SELF: Self = Self::Super(0);

    #[must_use]
    pub fn from_src(relative: Option<ast::ImportRelative>) -> Self {
        match relative {
            Some(ImportRelative::ImportPackageRelative(_)) => Self::Package,
            Some(ImportRelative::ImportSuperRelative(import_super)) => {
                Self::Super(import_super.super_count())
            },
            None => Self::Plain,
        }
    }
}

impl ModPath {
    /// The WESL grammar guarantees that only valid paths can be in the syntax tree.
    #[must_use]
    pub fn from_src(path: &ast::Path) -> Self {
        convert_path(path)
    }

    pub fn from_segments<Segments: IntoIterator<Item = Name>>(
        kind: PathKind,
        segments: Segments,
    ) -> Self {
        let mut segments: SmallVec<_> = segments.into_iter().collect();
        segments.shrink_to_fit();
        Self { kind, segments }
    }

    /// Creates a `ModPath` from a `PathKind`, with no extra path segments.
    #[must_use]
    pub const fn from_kind(kind: PathKind) -> Self {
        Self {
            kind,
            segments: SmallVec::new_const(),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> PathKind {
        self.kind
    }

    pub fn set_kind(
        &mut self,
        kind: PathKind,
    ) {
        self.kind = kind;
    }

    #[must_use]
    pub fn segments(&self) -> &[Name] {
        &self.segments
    }

    pub fn push_segment(
        &mut self,
        segment: Name,
    ) {
        self.segments.push(segment);
    }

    pub fn pop_segment(&mut self) -> Option<Name> {
        self.segments.pop()
    }

    /// Returns the number of segments in the path (counting special segments like `$crate` and
    /// `super`).
    #[must_use]
    pub fn len(&self) -> usize {
        self.segments.len()
            + match self.kind {
                PathKind::Plain => 0,
                PathKind::Super(levels) => usize::from(levels),
                PathKind::Package => 1,
            }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn textual_len(&self) -> usize {
        let base = match self.kind {
            PathKind::Plain => 0,
            PathKind::SELF => "self".len(),
            PathKind::Super(levels) => "super".len() * usize::from(levels),
            PathKind::Package => "crate".len(),
        };
        self.segments()
            .iter()
            .map(|segment| segment.as_str().len())
            .fold(base, core::ops::Add::add)
    }

    #[must_use]
    pub fn is_self(&self) -> bool {
        self.kind == PathKind::SELF && self.segments.is_empty()
    }

    /// If this path is a single identifier, like `foo`, return its name.
    #[must_use]
    pub fn as_ident(&self) -> Option<&Name> {
        if self.kind != PathKind::Plain {
            return None;
        }

        match &*self.segments {
            [name] => Some(name),
            _ => None,
        }
    }

    pub fn display_iter(&self) -> impl Iterator<Item = &str> {
        ModPathDisplayIter {
            kind: self.kind,
            segments: &self.segments,
            segment_index: 0,
        }
    }
}

impl Extend<Name> for ModPath {
    fn extend<T: IntoIterator<Item = Name>>(
        &mut self,
        iter: T,
    ) {
        self.segments.extend(iter);
    }
}

impl fmt::Display for ModPath {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let mut segments = self.display_iter();
        let Some(first_segment) = segments.next() else {
            return Ok(());
        };
        f.write_str(first_segment)?;
        while let Some(segment) = segments.next() {
            f.write_str("::")?;
            f.write_str(segment)?;
        }
        Ok(())
    }
}

impl From<Name> for ModPath {
    fn from(name: Name) -> Self {
        Self::from_segments(PathKind::Plain, iter::once(name))
    }
}

struct ModPathDisplayIter<'path> {
    kind: PathKind,
    segments: &'path SmallVec<[Name; 1]>,
    segment_index: usize,
}
impl<'path> Iterator for ModPathDisplayIter<'path> {
    type Item = &'path str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.kind {
            PathKind::Plain => {
                let name = self.segments.get(self.segment_index)?;
                self.segment_index += 1;
                Some(name.as_str())
            },
            PathKind::Super(level) => {
                if level == 0 {
                    self.kind = PathKind::Plain;
                    Some("self")
                } else if level == 1 {
                    self.kind = PathKind::Plain;
                    Some("super")
                } else {
                    self.kind = PathKind::Super(level - 1);
                    Some("super")
                }
            },
            PathKind::Package => {
                self.kind = PathKind::Plain;
                Some("package")
            },
        }
    }
}

fn convert_path(path: &ast::Path) -> ModPath {
    let kind = PathKind::from_src(path.relative());

    let mut segments: SmallVec<_> = path
        .segments()
        .map(|segment| Name::from(segment.text()))
        .collect();
    segments.shrink_to_fit();
    ModPath { kind, segments }
}
