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
    #[expect(clippy::len_without_is_empty, reason = "makes no sense for this type")]
    pub const fn len(&self) -> usize {
        1 + self.rest.len()
    }

    #[inline]
    pub fn into_last(mut self) -> T {
        self.rest.pop().unwrap_or(self.first)
    }
}

#[cfg(test)]
mod tests {
    use super::NonEmptyVec;

    #[test]
    fn non_empty_vec() {
        let mut vec = NonEmptyVec::new(1);
        assert_eq!(vec.len(), 1);
        assert_eq!(*vec.last_mut(), 1);
        assert_eq!(vec.pop(), None);
        assert_eq!(vec.into_last(), 1);

        let mut vec = NonEmptyVec::new(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);
        assert_eq!(*vec.last_mut(), 3);
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec.len(), 2);
        assert_eq!(*vec.last_mut(), 2);
        assert_eq!(vec.into_last(), 2);

        let mut vec = NonEmptyVec::new(1);
        vec.push(2);
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(*vec.last_mut(), 1);
        assert_eq!(vec.pop(), None);
        assert_eq!(vec.into_last(), 1);
    }
}
