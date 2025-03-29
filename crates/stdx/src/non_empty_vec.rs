//! See [`NonEmptyVec`].

/// A [`Vec`] that is guaranteed to at least contain one element.
pub struct NonEmptyVec<T> {
    first: T,
    rest: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    #[inline]
    pub const fn new(first: T) -> Self {
        Self {
            first,
            rest: Vec::new(),
        }
    }

    #[inline]
    pub fn last_mut(&mut self) -> &mut T {
        self.rest.last_mut().unwrap_or(&mut self.first)
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.rest.pop()
    }

    #[inline]
    pub fn push(
        &mut self,
        value: T,
    ) {
        self.rest.push(value);
    }

    #[inline]
    pub fn length(&self) -> usize {
        1 + self.rest.len()
    }

    #[inline]
    #[deprecated(note = "you probably did not mean to use this")]
    pub const fn is_empty(&self) -> bool {
        false
    }

    #[inline]
    pub fn into_last(mut self) -> T {
        self.rest.pop().unwrap_or(self.first)
    }
}
