/// Visibility of an item, with the path resolved.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Visibility {
    /// Visibility is limited to the current file.
    File,
    /// Visibility is unrestricted.
    Public,
}
