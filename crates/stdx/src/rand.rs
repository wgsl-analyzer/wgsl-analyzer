//! We don't use `rand` because that is too many things for us.
//!
//! `oorandom` is used instead, but it's missing these two utilities.
//! Switching to `fastrand` or our own small PRNG may be good because only xor-shift is needed.

pub fn shuffle<T, RandIndex>(
    slice: &mut [T],
    mut rand_index: RandIndex,
) where
    RandIndex: FnMut(usize) -> usize,
{
    let mut remaining = slice.len() - 1;
    while remaining > 0 {
        let index = rand_index(remaining);
        slice.swap(remaining, index);
        remaining -= 1;
    }
}

#[must_use]
pub fn seed() -> u64 {
    use std::hash::{BuildHasher as _, Hasher as _};
    #[expect(clippy::disallowed_types, reason = "TODO")]
    std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish()
}
