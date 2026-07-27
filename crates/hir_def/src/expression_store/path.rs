use crate::{
    item_tree::Name,
    mod_path::{ModPath, PathKind},
};

/// A colon separated path. Does not include generics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path(pub ModPath);

impl Path {
    #[must_use]
    pub const fn mod_path(&self) -> &ModPath {
        &self.0
    }

    #[must_use]
    pub const fn kind(&self) -> PathKind {
        self.0.kind()
    }

    #[must_use]
    pub fn segments(&self) -> &[Name] {
        self.0.segments()
    }

    #[must_use]
    pub fn missing() -> Self {
        Self(ModPath::from(Name::missing()))
    }
}
