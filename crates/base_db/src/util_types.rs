pub use rowan::{TextRange, TextSize};
use vfs::FileId;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct FilePosition {
    pub file_id: FileId,
    pub offset: TextSize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FileRange {
    pub file_id: FileId,
    pub range: TextRange,
}

#[derive(Debug)]
pub struct RangeInfo<T> {
    pub range: TextRange,
    pub info: T,
}

impl<T> RangeInfo<T> {
    pub const fn new(
        range: TextRange,
        info: T,
    ) -> Self {
        Self { range, info }
    }
}
