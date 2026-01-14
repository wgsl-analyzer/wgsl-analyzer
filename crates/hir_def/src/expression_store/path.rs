use crate::{item_tree::Name, mod_path::ModPath};

/// A colon separated path. Does not include generics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path(pub ModPath);

impl Path {
    #[must_use]
    pub const fn mod_path(&self) -> &ModPath {
        &self.0
    }
    #[must_use]
    pub fn missing() -> Self {
        Self::from(Name::missing())
    }
}

impl From<Name> for Path {
    fn from(name: Name) -> Self {
        Self(ModPath::from_segments(
            crate::mod_path::PathKind::Plain,
            std::iter::once(name),
        ))
    }
}
