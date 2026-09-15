//! This file is a port of only the necessary features from <https://github.com/chris-morgan/anymap> version 1.0.0-beta.2 for use within wgsl-analyzer.
//!
//! Copyright © 2014–2022 Chris Morgan.
//! COPYING: <https://github.com/chris-morgan/anymap/blob/master/COPYING>
//! Note that the license is changed from `Blue Oak Model 1.0.0 or MIT or Apache-2.0` to `MIT OR Apache-2.0`.
//!
//! This implementation provides a safe and convenient store for one value of each type.
//!
//! Your starting point is [`Map`]. It has an example.
//!
//! # Cargo features
//!
//! This implementation has two independent features, each of which provides an implementation providing
//! types `Map`, `AnyMap`, `OccupiedEntry`, `VacantEntry`, `Entry` and `RawMap`:
//!
//! - **std** (default, *enabled* in this build):
//!   an implementation using `std::collections::hash_map`, placed in the crate root
//!   (for example, `anymap::AnyMap`).

#![warn(missing_docs, unused_results)]

use core::hash::Hasher;

/// A hasher designed to eke a little more speed out, given `TypeId`'s known characteristics.
///
/// Specifically, this is a no-op hasher that expects to be fed a u64's worth of
/// randomly-distributed bits. It works well for `TypeId` (eliminating start-up time, so that my
/// `get_missing` benchmark is ~30ns rather than ~900ns, and being a good deal faster after that, so
/// that my `insert_and_get_on_260_types` benchmark is ~12μs instead of ~21.5μs), but will
/// panic in debug mode and always emit zeros in release mode for any other sorts of inputs, so
/// yeah, do not use it!
#[derive(Default)]
pub struct TypeIdHasher {
    value: u64,
}

impl Hasher for TypeIdHasher {
    #[inline]
    fn write(
        &mut self,
        bytes: &[u8],
    ) {
        // This expects to receive exactly one 64-bit value, and there is no realistic chance of
        // that changing, but I do not want to depend on something that is not expressly part of the
        // contract for safety. But I am OK with release builds putting everything in one bucket
        // if it *did* change (and debug builds panicking).
        debug_assert_eq!(bytes.len(), 8);
        #[expect(clippy::host_endian_bytes, reason = "not relevant")]
        if let Ok(array) = bytes.try_into() {
            self.value = u64::from_ne_bytes(array);
        }
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.value
    }
}

use core::any::{Any, TypeId};
use core::hash::BuildHasherDefault;
use core::marker::PhantomData;

use ::std::collections::hash_map;

/// Raw access to the underlying `HashMap`.
///
/// This alias is provided for convenience because of the ugly third generic parameter.
#[expect(clippy::disallowed_types, reason = "Uses a custom hasher")]
pub type RawMap<A> = hash_map::HashMap<TypeId, Box<A>, BuildHasherDefault<TypeIdHasher>>;

/// A collection containing zero or one values for any given type and allowing convenient,
/// type-safe access to those values.
///
/// The type parameter `A` allows you to use a different value type; normally you will want
/// it to be `core::any::Any` (also known as `std::any::Any`), but there are other choices:
///
/// - You can add on `+ Send` or `+ Send + Sync` (for example, `Map<dyn Any + Send>`)
///   to add those auto traits.
///
/// Cumulatively, there are thus six forms of map:
///
/// - `[Map]<dyn [core::any::Any]>`,
///   also spelled [`AnyMap`] for convenience.
/// - `[Map]<dyn [core::any::Any] + Send>`
/// - `[Map]<dyn [core::any::Any] + Send + Sync>`
///
/// ## Example
///
/// Here, the [`AnyMap`] convenience alias is used;
/// the first line could use `[anymap::Map][Map]::<[core::any::Any]>::default()`
/// instead if desired.
///
/// ```rust
/// # use stdx::anymap;
/// let mut data = anymap::AnyMap::default();
/// assert_eq!(data.get(), None::<&i32>);
/// ```
///
/// Values containing non-static references are not permitted.
#[derive(Debug)]
pub struct Map<A: ?Sized + Downcast = dyn Any> {
    raw: RawMap<A>,
}

/// The most common type of `Map`: just using `Any`; `[Map]<dyn [Any]>`.
///
/// Why is this a separate type alias rather than a default value for `Map<A>`?
/// `Map::default()` does not seem to be happy to infer that it should go with the default
/// value. It is a bit sad, really. Ah well, I guess this approach will do.
pub type AnyMap = Map<dyn Any>;

impl<A: ?Sized + Downcast> Default for Map<A> {
    #[inline]
    fn default() -> Self {
        Self {
            raw: RawMap::with_hasher(BuildHasherDefault::default()),
        }
    }
}

impl<A: ?Sized + Downcast> Map<A> {
    /// Returns a reference to the value stored in the collection for the type `T`,
    /// if it exists.
    #[inline]
    #[must_use]
    pub fn get<T>(&self) -> Option<&T>
    where
        T: IntoBox<A>,
    {
        self.raw
            .get(&TypeId::of::<T>())
        	// SAFETY: T does match the trait object because `T: IntoBox<A>`.
            .map(|any| unsafe { any.downcast_unchecked_ref::<T>() })
    }

    /// Gets the entry for the given type in the collection for in-place manipulation.
    #[inline]
    pub fn entry<T>(&mut self) -> Entry<'_, A, T>
    where
        T: IntoBox<A>,
    {
        match self.raw.entry(TypeId::of::<T>()) {
            hash_map::Entry::Occupied(entry) => Entry::Occupied(OccupiedEntry {
                inner: entry,
                type_: PhantomData,
            }),
            hash_map::Entry::Vacant(entry) => Entry::Vacant(VacantEntry {
                inner: entry,
                type_: PhantomData,
            }),
        }
    }
}

/// A view into a single occupied location in an `Map`.
pub struct OccupiedEntry<'map, A: ?Sized + Downcast, V: 'map> {
    inner: hash_map::OccupiedEntry<'map, TypeId, Box<A>>,
    type_: PhantomData<V>,
}

/// A view into a single empty location in an `Map`.
pub struct VacantEntry<'map, A: ?Sized + Downcast, V: 'map> {
    inner: hash_map::VacantEntry<'map, TypeId, Box<A>>,
    type_: PhantomData<V>,
}

/// A view into a single location in an `Map`, which may be vacant or occupied.
pub enum Entry<'map, A: ?Sized + Downcast, V> {
    /// An occupied Entry.
    Occupied(OccupiedEntry<'map, A, V>),
    /// A vacant Entry.
    Vacant(VacantEntry<'map, A, V>),
}

impl<'map, A: ?Sized + Downcast, V: IntoBox<A>> Entry<'map, A, V> {
    /// Ensures a value is in the entry by inserting the result of the default function if
    /// empty, and returns a mutable reference to the value in the entry.
    #[inline]
    pub fn or_insert_with<Default>(
        self,
        default: Default,
    ) -> &'map mut V
    where
        Default: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(inner) => inner.into_mut(),
            Entry::Vacant(inner) => inner.insert(default()),
        }
    }
}

impl<'map, A: ?Sized + Downcast, V: IntoBox<A>> OccupiedEntry<'map, A, V> {
    /// Converts the `OccupiedEntry` into a mutable reference to the value in the entry
    /// with a lifetime bound to the collection itself.
    #[inline]
    #[must_use]
    pub fn into_mut(self) -> &'map mut V {
        // SAFETY: T does match the trait object because `V: IntoBox<A>`.
        unsafe { self.inner.into_mut().downcast_unchecked_mut() }
    }
}

impl<'map, A: ?Sized + Downcast, V: IntoBox<A>> VacantEntry<'map, A, V> {
    /// Sets the value of the entry with the `VacantEntry`'s key,
    /// and returns a mutable reference to it.
    #[inline]
    pub fn insert(
        self,
        value: V,
    ) -> &'map mut V {
        // SAFETY: T does match the trait object because `V: IntoBox<A>`.
        unsafe { self.inner.insert(value.into_box()).downcast_unchecked_mut() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varieties() {
        fn assert_send<T>()
        where
            T: Send,
        {
        }
        fn assert_sync<T>()
        where
            T: Sync,
        {
        }
        fn assert_debug<T>()
        where
            T: ::core::fmt::Debug,
        {
        }
        assert_send::<Map<dyn Any + Send>>();
        assert_send::<Map<dyn Any + Send + Sync>>();
        assert_sync::<Map<dyn Any + Send + Sync>>();
        assert_debug::<Map<dyn Any>>();
        assert_debug::<Map<dyn Any + Send>>();
        assert_debug::<Map<dyn Any + Send + Sync>>();
    }

    #[test]
    fn type_id_hasher() {
        use core::any::TypeId;
        use core::hash::Hash as _;
        fn verify_hashing_with(type_id: TypeId) {
            let mut hasher = TypeIdHasher::default();
            type_id.hash(&mut hasher);
            _ = hasher.finish();
        }
        // Pick a variety of types, just to demonstrate that it is all sane. Normal, zero-sized, unsized, &c.
        verify_hashing_with(TypeId::of::<usize>());
        verify_hashing_with(TypeId::of::<()>());
        verify_hashing_with(TypeId::of::<str>());
        verify_hashing_with(TypeId::of::<&str>());
        verify_hashing_with(TypeId::of::<Vec<u8>>());
    }
}

/// Methods for downcasting from an `Any`-like trait object.
///
/// This should only be implemented on trait objects for subtraits of `Any`, though you can
/// implement it for other types and it will work fine, so long as your implementation is correct.
pub trait Downcast {
    /// Gets the `TypeId` of `self`.
    fn type_id(&self) -> TypeId;

    // Note the bound through these downcast methods is 'static, rather than the inexpressible
    // concept of Self-but-as-a-trait (where Self is `dyn Trait`). This is sufficient, exceeding
    // TypeId's requirements. Sure, you *can* do CloneAny.downcast_unchecked::<NotClone>() and the
    // type system will not protect you, but that does not introduce any unsafety: the method is
    // already unsafe because you can specify the wrong type, and if this were exposing safe
    // downcasting, CloneAny.downcast::<NotClone>() would just return an error, which is just as
    // correct.
    //
    // Now in theory we could also add T: ?Sized, but that does not play nicely with the common
    // implementation, so I am doing without it.

    /// Downcast from `&Any` to `&T`, without checking the type matches.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `T` matches the trait object, on pain of *undefined behavior*.
    unsafe fn downcast_unchecked_ref<T>(&self) -> &T
    where
        T: 'static;

    /// Downcast from `&mut Any` to `&mut T`, without checking the type matches.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `T` matches the trait object, on pain of *undefined behavior*.
    unsafe fn downcast_unchecked_mut<T>(&mut self) -> &mut T
    where
        T: 'static;
}

/// A trait for the conversion of an object into a boxed trait object.
pub trait IntoBox<A: ?Sized + Downcast>: Any {
    /// Convert self into the appropriate boxed form.
    fn into_box(self) -> Box<A>;
}

macro_rules! implement {
    ($any_trait:ident $(+ $auto_traits:ident)*) => {
        impl Downcast for dyn $any_trait $(+ $auto_traits)* {
            #[inline]
            fn type_id(&self) -> TypeId {
                self.type_id()
            }

            /// Returns a reference to the underlying value without checking its type.
            ///
            /// # Safety
            ///
            /// The caller **must** ensure that the actual type of the underlying object is `T`.
            /// If the type is incorrect, this will result in undefined behavior due to an invalid cast.
            ///
            /// This method performs an unchecked cast from the trait object to the concrete type.
            #[inline]
            unsafe fn downcast_unchecked_ref<T>(&self) -> &T where T: 'static {
                // SAFETY:
                // The caller guarantees that `self` is a `T`. We cast from a trait object to T accordingly.
                unsafe { &*std::ptr::from_ref::<Self>(self).cast::<T>() }
            }

            /// Returns a mutable reference to the underlying value without checking its type.
            ///
            /// # Safety
            ///
            /// The caller **must** ensure that the actual type of the underlying object is `T`.
            /// If the type is incorrect, this will result in undefined behavior due to an invalid cast.
            ///
            /// This method performs an unchecked cast from the trait object to the concrete type.
            #[inline]
            unsafe fn downcast_unchecked_mut<T>(&mut self) -> &mut T where T: 'static {
                // SAFETY:
                // The caller guarantees that `self` is a `T`. We cast from a trait object to T accordingly.
                unsafe { &mut *std::ptr::from_mut::<Self>(self).cast::<T>() }
            }
        }

        impl<T: $any_trait $(+ $auto_traits)*> IntoBox<dyn $any_trait $(+ $auto_traits)*> for T {
            #[inline]
            fn into_box(self) -> Box<dyn $any_trait $(+ $auto_traits)*> {
                Box::new(self)
            }
        }
    }
}

implement!(Any);
implement!(Any + Send);
implement!(Any + Send + Sync);
