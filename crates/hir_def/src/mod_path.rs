//! A lowering for `import`-paths (more generally, paths without angle-bracketed segments).

use std::{
    fmt::{self, Display as _},
    iter,
};

use crate::{database::DefDatabase, item_tree::Name};
use smallvec::SmallVec;
use syntax::{
    AstNode,
    ast::{self, ImportRelative},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModPath {
    pub kind: PathKind,
    segments: SmallVec<[Name; 1]>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PathKind {
    Plain,
    /// `self::` is `Super(0)`
    Super(u8),
    Package,
}

impl PathKind {
    pub const SELF: PathKind = PathKind::Super(0);
    pub fn from_src(relative: Option<ast::ImportRelative>) -> PathKind {
        match relative {
            Some(ImportRelative::ImportPackageRelative(_)) => PathKind::Package,
            Some(ImportRelative::ImportSuperRelative(import_super)) => {
                PathKind::Super(import_super.super_count())
            },
            None => PathKind::Plain,
        }
    }
}

impl ModPath {
    /// The WESL grammar guarantees that only valid paths can be in the syntax tree.
    pub fn from_src(path: ast::Path) -> ModPath {
        convert_path(path)
    }

    pub fn from_segments(
        kind: PathKind,
        segments: impl IntoIterator<Item = Name>,
    ) -> ModPath {
        let mut segments: SmallVec<_> = segments.into_iter().collect();
        segments.shrink_to_fit();
        ModPath { kind, segments }
    }

    /// Creates a `ModPath` from a `PathKind`, with no extra path segments.
    pub const fn from_kind(kind: PathKind) -> ModPath {
        ModPath {
            kind,
            segments: SmallVec::new_const(),
        }
    }

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
    pub fn len(&self) -> usize {
        self.segments.len()
            + match self.kind {
                PathKind::Plain => 0,
                PathKind::Super(i) => i as usize,
                PathKind::Package => 1,
            }
    }

    pub fn textual_len(&self) -> usize {
        let base = match self.kind {
            PathKind::Plain => 0,
            PathKind::SELF => "self".len(),
            PathKind::Super(i) => "super".len() * i as usize,
            PathKind::Package => "crate".len(),
        };
        self.segments()
            .iter()
            .map(|segment| segment.as_str().len())
            .fold(base, core::ops::Add::add)
    }

    pub fn is_ident(&self) -> bool {
        self.as_ident().is_some()
    }

    pub fn is_self(&self) -> bool {
        self.kind == PathKind::SELF && self.segments.is_empty()
    }

    /// If this path is a single identifier, like `foo`, return its name.
    pub fn as_ident(&self) -> Option<&Name> {
        if self.kind != PathKind::Plain {
            return None;
        }

        match &*self.segments {
            [name] => Some(name),
            _ => None,
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
        display_fmt_path(self, f)
    }
}

impl From<Name> for ModPath {
    fn from(name: Name) -> ModPath {
        ModPath::from_segments(PathKind::Plain, iter::once(name))
    }
}

fn display_fmt_path(
    path: &ModPath,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let mut first_segment = true;
    let mut add_segment = |s| -> fmt::Result {
        if !first_segment {
            f.write_str("::")?;
        }
        first_segment = false;
        f.write_str(s)?;
        Ok(())
    };
    match path.kind {
        PathKind::Plain => {},
        PathKind::SELF => add_segment("self")?,
        PathKind::Super(n) => {
            for _ in 0..n {
                add_segment("super")?;
            }
        },
        PathKind::Package => add_segment("crate")?,
    }
    for segment in &path.segments {
        if !first_segment {
            f.write_str("::")?;
        }
        first_segment = false;
        fmt::Display::fmt(segment.as_str(), f)?;
    }
    Ok(())
}

fn convert_path(path: ast::Path) -> ModPath {
    let kind = PathKind::from_src(path.relative());

    let mut segments: SmallVec<_> = path
        .segments()
        .map(|segment| Name::from(segment.text()))
        .into_iter()
        .collect();
    segments.shrink_to_fit();
    ModPath { kind, segments }
}
