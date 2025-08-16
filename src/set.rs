//! The `IndexSet` struct is an index-keyed set built above any type implementing `IndexStore`.
//!
//! The `IndexOrdSet`, `IndexChunkSet`, and `IndexSliceSet` implement more efficient set operations, by leaning on
//! refinements of the `IndexStore` trait.

use core::{
    cmp::{self, Ordering},
    iter::FusedIterator,
    ops::{self, Bound},
};

#[cfg(feature = "nightly")]
use core::ops::Try;

use crate::{
    Never,
    index::{IndexBackward, IndexCollection, IndexForward, IndexOrdered, IndexStore, IndexView},
};

/// A set of indexes.
#[derive(Clone, Copy, Debug)]
pub struct IndexSet<S> {
    store: S,
}

/// A set of indexes.
#[derive(Clone, Copy, Debug)]
pub struct IndexOrdSet<S> {
    store: S,
}

//
//  Construction.
//

impl<S> IndexSet<S>
where
    S: IndexCollection,
{
    /// Returns the span of index values which MAY be inserted.
    ///
    /// Attempts to insert values outside this span WILL fail, possibly via panicking or aborting.
    #[inline(always)]
    pub fn span() -> (Bound<S::Index>, Bound<S::Index>) {
        S::span()
    }

    /// Creates a new, empty, instance.
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_store(S::new())
    }

    /// Creates a new, empty, instance, with appropriate capacity for storing the span if possible.
    ///
    /// This is purely a _best effort_ method, as not all collections allow reserving extra space.
    #[inline(always)]
    pub fn with_span(range: (Bound<S::Index>, Bound<S::Index>)) -> Self {
        Self::with_store(S::with_span(range))
    }

    /// Creates a new instance from the original store.
    #[inline(always)]
    pub const fn with_store(store: S) -> Self {
        Self { store }
    }
}

impl<S> IndexOrdSet<S>
where
    S: IndexCollection + IndexOrdered,
{
    /// Returns the span of index values which MAY be inserted.
    ///
    /// Attempts to insert values outside this span WILL fail, possibly via panicking or aborting.
    #[inline(always)]
    pub fn span() -> (Bound<S::Index>, Bound<S::Index>) {
        S::span()
    }

    /// Creates a new, empty, instance.
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_store(S::new())
    }

    /// Creates a new, empty, instance, with appropriate capacity for storing up to `n` indexes if possible.
    ///
    /// This is purely a _best effort_ method, as not all collections allow reserving extra space, and some may have a
    /// a maximum capacity that is below `n` anyway.
    #[inline(always)]
    pub fn with_span(range: (Bound<S::Index>, Bound<S::Index>)) -> Self {
        Self::with_store(S::with_span(range))
    }

    /// Creates a new instance from the original store.
    #[inline(always)]
    pub const fn with_store(store: S) -> Self {
        Self { store }
    }
}

impl<S> Default for IndexSet<S>
where
    S: IndexCollection,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Default for IndexOrdSet<S>
where
    S: IndexCollection + IndexOrdered,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<A, S> FromIterator<A> for IndexSet<S>
where
    S: IndexCollection<Index = A> + IndexStore<Index = A, InsertionError = Never>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        let mut this = Self::new();

        this.extend(iter);

        this
    }
}

impl<A, S> FromIterator<A> for IndexOrdSet<S>
where
    S: IndexCollection<Index = A> + IndexOrdered<Index = A> + IndexStore<Index = A, InsertionError = Never>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        let mut this = Self::new();

        this.extend(iter);

        this
    }
}

#[cfg(test)]
mod construction_tests;

//
//  Deconstruction operations.
//

impl<S> IndexSet<S> {
    /// Returns a reference to the underlying store.
    pub fn as_store(&self) -> &S {
        &self.store
    }

    /// Returns a mutable reference to the underlying store.
    pub fn as_store_mut(&mut self) -> &mut S {
        &mut self.store
    }

    /// Returns the store.
    pub fn into_store(self) -> S {
        self.store
    }
}

impl<S> IndexOrdSet<S> {
    /// Returns a reference to the underlying store.
    pub fn as_store(&self) -> &S {
        &self.store
    }

    /// Returns a mutable reference to the underlying store.
    pub fn as_store_mut(&mut self) -> &mut S {
        &mut self.store
    }

    /// Returns the store.
    pub fn into_store(self) -> S {
        self.store
    }
}

//
//  View Operations.
//

impl<S> IndexSet<S>
where
    S: IndexView,
{
    /// Returns whether the set is empty, or not.
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    /// Returns the number of indexes in the set.
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// Returns whether the index is contained in the set.
    pub fn contains(&self, index: S::Index) -> bool {
        self.store.contains(index)
    }
}

impl<S> IndexOrdSet<S>
where
    S: IndexView,
{
    /// Returns whether the set is empty, or not.
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    /// Returns the number of indexes in the set.
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// Returns whether the index is contained in the set.
    pub fn contains(&self, index: S::Index) -> bool {
        self.store.contains(index)
    }
}

#[cfg(test)]
mod view_tests;

//
//  Store operations.
//

impl<S> IndexSet<S>
where
    S: IndexStore,
{
    /// Removes all indexes from the set.
    pub fn clear(&mut self) {
        self.store.clear()
    }

    /// Inserts the index in the set, returns whether it is newly inserted.
    pub fn insert(&mut self, index: S::Index) -> Result<bool, S::InsertionError> {
        self.store.insert(index)
    }

    /// Removes the index from the set, returns whether it was in the set prior to removal.
    pub fn remove(&mut self, index: S::Index) -> bool {
        self.store.remove(index)
    }
}

impl<S> IndexOrdSet<S>
where
    S: IndexStore,
{
    /// Removes all indexes from the set.
    pub fn clear(&mut self) {
        self.store.clear()
    }

    /// Inserts the index in the set, returns whether it is newly inserted.
    pub fn insert(&mut self, index: S::Index) -> Result<bool, S::InsertionError> {
        self.store.insert(index)
    }

    /// Removes the index from the set, returns whether it was in the set prior to removal.
    pub fn remove(&mut self, index: S::Index) -> bool {
        self.store.remove(index)
    }
}

impl<A, S> Extend<A> for IndexSet<S>
where
    S: IndexStore<Index = A, InsertionError = Never>,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = A>,
    {
        for index in iter {
            let _ = self.insert(index);
        }
    }

    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, index: A) {
        let _ = self.insert(index);
    }
}

impl<A, S> Extend<A> for IndexOrdSet<S>
where
    S: IndexStore<Index = A, InsertionError = Never>,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = A>,
    {
        for index in iter {
            let _ = self.insert(index);
        }
    }

    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, index: A) {
        let _ = self.insert(index);
    }
}

#[cfg(test)]
mod store_tests;

//
//  Inclusion operations.
//

impl<S> IndexSet<S>
where
    S: IndexForward,
{
    /// Returns whether `self` and `other` are disjoint, ie do not have any index in common.
    pub fn is_disjoint<OS>(&self, other: &IndexSet<OS>) -> bool
    where
        OS: IndexView<Index = S::Index>,
    {
        self.iter().all(|index| !other.contains(index))
    }

    /// Returns whether `self` is a subset of `other`, ie whether all elements of `self` are contained in `other`.
    ///
    /// If `self` is a subset of `other`, then `other` is a superset of `self`, and vice-versa.
    pub fn is_subset<OS>(&self, other: &IndexSet<OS>) -> bool
    where
        OS: IndexView<Index = S::Index>,
    {
        self.iter().all(|index| other.contains(index))
    }

    /// Returns whether `self` is a superset of `other`, ie whether all elements of `other` are contained in `self`.
    ///
    /// If `self` is a superset of `other`, then `other` is a subset of `self`, and vice-versa.
    pub fn is_superset<OS>(&self, other: &IndexSet<OS>) -> bool
    where
        OS: IndexForward<Index = S::Index>,
    {
        other.is_subset(self)
    }
}

impl<S> IndexOrdSet<S>
where
    S: IndexForward,
{
    /// Returns whether `self` and `other` are disjoint, ie do not have any index in common.
    pub fn is_disjoint<OS>(&self, other: &IndexOrdSet<OS>) -> bool
    where
        OS: IndexView<Index = S::Index>,
    {
        self.iter().all(|index| !other.contains(index))
    }

    /// Returns whether `self` is a subset of `other`, ie whether all elements of `self` are contained in `other`.
    ///
    /// If `self` is a subset of `other`, then `other` is a superset of `self`, and vice-versa.
    pub fn is_subset<OS>(&self, other: &IndexOrdSet<OS>) -> bool
    where
        OS: IndexView<Index = S::Index>,
    {
        self.iter().all(|index| other.contains(index))
    }

    /// Returns whether `self` is a superset of `other`, ie whether all elements of `other` are contained in `self`.
    ///
    /// If `self` is a superset of `other`, then `other` is a subset of `self`, and vice-versa.
    pub fn is_superset<OS>(&self, other: &IndexOrdSet<OS>) -> bool
    where
        OS: IndexForward<Index = S::Index>,
    {
        other.is_subset(self)
    }
}

#[cfg(test)]
mod inclusion_tests;

//
//  Entry API.
//

impl<S> IndexSet<S>
where
    S: IndexStore,
{
    /// Returns the entry.
    pub fn entry(&mut self, index: S::Index) -> Entry<'_, S::Index, S> {
        if self.contains(index) {
            Entry::Occupied(OccupiedEntry {
                index,
                store: &mut self.store,
            })
        } else {
            Entry::Vacant(VacantEntry {
                index,
                store: &mut self.store,
            })
        }
    }
}

impl<S> IndexOrdSet<S>
where
    S: IndexStore,
{
    /// Returns the entry.
    pub fn entry(&mut self, index: S::Index) -> Entry<'_, S::Index, S> {
        if self.contains(index) {
            Entry::Occupied(OccupiedEntry {
                index,
                store: &mut self.store,
            })
        } else {
            Entry::Vacant(VacantEntry {
                index,
                store: &mut self.store,
            })
        }
    }
}

/// A view into a single entry in a set, which may either be vacant or occupied.
pub enum Entry<'a, I, S> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, I, S>),
    /// A vacant entry.
    Vacant(VacantEntry<'a, I, S>),
}

impl<'a, I, S> Entry<'a, I, S>
where
    I: Copy,
    S: IndexStore<Index = I>,
{
    /// Returns the index.
    pub fn get(&self) -> I {
        match self {
            Self::Occupied(o) => o.get(),
            Self::Vacant(v) => v.get(),
        }
    }

    /// Inserts the index, and returns an `OccupiedEntry`.
    pub fn insert(self) -> Result<OccupiedEntry<'a, I, S>, S::InsertionError> {
        match self {
            Self::Occupied(o) => Ok(o),
            Self::Vacant(VacantEntry { index, store }) => {
                let _inserted = store.insert(index)?;

                debug_assert!(_inserted);

                Ok(OccupiedEntry { index, store })
            }
        }
    }

    /// Ensures the index is in the set.
    pub fn or_insert(self) -> Result<(), S::InsertionError> {
        if let Self::Vacant(v) = self {
            v.insert()?;
        }

        Ok(())
    }
}

/// An occupied entry in a set.
pub struct OccupiedEntry<'a, I, S> {
    index: I,
    store: &'a mut S,
}

impl<'a, I, S> OccupiedEntry<'a, I, S>
where
    I: Copy,
    S: IndexStore<Index = I>,
{
    /// Returns the index.
    pub fn get(&self) -> I {
        self.index
    }

    /// Removes the index from the set.
    pub fn remove(self) -> I {
        let _removed = self.store.remove(self.index);

        debug_assert!(_removed);

        self.index
    }
}

/// A vacant entry in a set.
pub struct VacantEntry<'a, I, S> {
    index: I,
    store: &'a mut S,
}

impl<'a, I, S> VacantEntry<'a, I, S>
where
    I: Copy,
    S: IndexStore<Index = I>,
{
    /// Returns the index.
    pub fn get(&self) -> I {
        self.index
    }

    /// Consumes the entry and returns the index.
    pub fn into_value(self) -> I {
        self.index
    }

    /// Inserts the index in the store.
    pub fn insert(self) -> Result<(), S::InsertionError> {
        let _inserted = self.store.insert(self.index)?;

        debug_assert!(_inserted);

        Ok(())
    }
}

#[cfg(test)]
mod entry_tests;

//
//  Iterator operations: just iteration.
//

impl<S> IndexSet<S>
where
    S: IndexForward,
{
    /// Returns an iterator over the indexes in the set.
    pub fn iter(&self) -> Iter<'_, S::Index, S> {
        Iter {
            next: self.store.first(),
            yielded: 0,
            store: &self.store,
        }
    }

    /// Returns an iterator over the indexes in the set.
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> IntoIter<S::Index, S> {
        IntoIter {
            next: self.store.first(),
            yielded: 0,
            store: self.store,
        }
    }
}

impl<S> IndexOrdSet<S>
where
    S: IndexForward,
{
    /// Returns an iterator over the indexes in the set.
    pub fn iter(&self) -> Iter<'_, S::Index, S> {
        Iter {
            next: self.store.first(),
            yielded: 0,
            store: &self.store,
        }
    }

    /// Returns an iterator over the indexes in the set.
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> IntoIter<S::Index, S> {
        IntoIter {
            next: self.store.first(),
            yielded: 0,
            store: self.store,
        }
    }
}

impl<S> IndexSet<S>
where
    S: IndexBackward,
{
    /// Returns an iterator over the indexes in the set.
    pub fn iter_rev(&self) -> IterRev<'_, S::Index, S> {
        IterRev {
            next: self.store.last(),
            yielded: 0,
            store: &self.store,
        }
    }

    /// Returns an iterator over the indexes in the set.
    pub fn into_iter_rev(self) -> IntoIterRev<S::Index, S> {
        IntoIterRev {
            next: self.store.last(),
            yielded: 0,
            store: self.store,
        }
    }
}

impl<S> IndexOrdSet<S>
where
    S: IndexBackward,
{
    /// Returns an iterator over the indexes in the set.
    pub fn iter_rev(&self) -> IterRev<'_, S::Index, S> {
        IterRev {
            next: self.store.last(),
            yielded: 0,
            store: &self.store,
        }
    }

    /// Returns an iterator over the indexes in the set.
    pub fn into_iter_rev(self) -> IntoIterRev<S::Index, S> {
        IntoIterRev {
            next: self.store.last(),
            yielded: 0,
            store: self.store,
        }
    }
}

impl<'a, S> IntoIterator for &'a IndexSet<S>
where
    S: IndexForward,
{
    type Item = S::Index;
    type IntoIter = Iter<'a, S::Index, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<S> IntoIterator for IndexSet<S>
where
    S: IndexForward,
{
    type Item = S::Index;
    type IntoIter = IntoIter<S::Index, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, S> IntoIterator for &'a IndexOrdSet<S>
where
    S: IndexForward,
{
    type Item = S::Index;
    type IntoIter = Iter<'a, S::Index, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<S> IntoIterator for IndexOrdSet<S>
where
    S: IndexForward,
{
    type Item = S::Index;
    type IntoIter = IntoIter<S::Index, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

/// Iterator over the elements of S.
pub struct Iter<'a, I, S> {
    next: Option<I>,
    yielded: usize,
    store: &'a S,
}

impl<'a, I, S> Iterator for Iter<'a, I, S>
where
    I: Copy,
    S: IndexForward<Index = I>,
{
    type Item = I;

    fn count(self) -> usize {
        self.len()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.len();

        (length, Some(length))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next.take()?;

        self.yielded += 1;
        self.next = self.store.next_after(result);

        Some(result)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if let Some(n) = n.checked_sub(1) {
            let index = self.next.take()?;

            match self.store.nth_after(n, index) {
                Ok(next) => {
                    self.next = Some(next);
                    self.yielded += n;
                }
                Err(remainder) => {
                    self.yielded += n - remainder.get();
                }
            }
        }

        self.next()
    }

    #[cfg(feature = "nightly")]
    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: Try<Output = B>,
    {
        let Some(index) = self.next.take() else {
            return R::from_output(init);
        };

        self.yielded = self.store.len();

        let init = f(init, index)?;

        self.store.try_fold_after(index, init, f)
    }
}

impl<'a, I, S> ExactSizeIterator for Iter<'a, I, S>
where
    I: Copy,
    S: IndexForward<Index = I>,
{
    fn len(&self) -> usize {
        self.store.len() - self.yielded
    }

    #[cfg(feature = "nightly")]
    fn is_empty(&self) -> bool {
        self.store.len() == self.yielded
    }
}

impl<'a, I, S> FusedIterator for Iter<'a, I, S>
where
    I: Copy,
    S: IndexForward<Index = I>,
{
}

/// Iterator over the elements of S, in reverse order.
pub struct IterRev<'a, I, S> {
    next: Option<I>,
    yielded: usize,
    store: &'a S,
}

impl<'a, I, S> Iterator for IterRev<'a, I, S>
where
    I: Copy,
    S: IndexBackward<Index = I>,
{
    type Item = I;

    fn count(self) -> usize {
        self.len()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.len();

        (length, Some(length))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next.take()?;

        self.yielded += 1;
        self.next = self.store.next_before(result);

        Some(result)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if let Some(n) = n.checked_sub(1) {
            let index = self.next.take()?;

            match self.store.nth_before(n, index) {
                Ok(next) => {
                    self.next = Some(next);
                    self.yielded += n;
                }
                Err(remainder) => {
                    self.yielded += n - remainder.get();
                }
            }
        }

        self.next()
    }

    #[cfg(feature = "nightly")]
    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: Try<Output = B>,
    {
        let Some(index) = self.next.take() else {
            return R::from_output(init);
        };

        self.yielded = self.store.len();

        let init = f(init, index)?;

        self.store.try_fold_before(index, init, f)
    }
}

impl<'a, I, S> ExactSizeIterator for IterRev<'a, I, S>
where
    I: Copy,
    S: IndexBackward<Index = I>,
{
    fn len(&self) -> usize {
        self.store.len() - self.yielded
    }

    #[cfg(feature = "nightly")]
    fn is_empty(&self) -> bool {
        self.store.len() == self.yielded
    }
}

impl<'a, I, S> FusedIterator for IterRev<'a, I, S>
where
    I: Copy,
    S: IndexBackward<Index = I>,
{
}

/// Iterator over the elements of S.
pub struct IntoIter<I, S> {
    next: Option<I>,
    yielded: usize,
    store: S,
}

impl<I, S> Iterator for IntoIter<I, S>
where
    I: Copy,
    S: IndexForward<Index = I>,
{
    type Item = I;

    fn count(self) -> usize {
        self.len()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.len();

        (length, Some(length))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next.take()?;

        self.yielded += 1;
        self.next = self.store.next_after(result);

        Some(result)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if let Some(n) = n.checked_sub(1) {
            let index = self.next.take()?;

            match self.store.nth_after(n, index) {
                Ok(next) => {
                    self.next = Some(next);
                    self.yielded += n;
                }
                Err(remainder) => {
                    self.yielded += n - remainder.get();
                }
            }
        }

        self.next()
    }

    #[cfg(feature = "nightly")]
    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: Try<Output = B>,
    {
        let Some(index) = self.next.take() else {
            return R::from_output(init);
        };

        self.yielded = self.store.len();

        let init = f(init, index)?;

        self.store.try_fold_after(index, init, f)
    }
}

impl<I, S> ExactSizeIterator for IntoIter<I, S>
where
    I: Copy,
    S: IndexForward<Index = I>,
{
    fn len(&self) -> usize {
        self.store.len() - self.yielded
    }

    #[cfg(feature = "nightly")]
    fn is_empty(&self) -> bool {
        self.store.len() == self.yielded
    }
}

impl<I, S> FusedIterator for IntoIter<I, S>
where
    I: Copy,
    S: IndexForward<Index = I>,
{
}

/// Iterator over the elements of S, in reverse order.
pub struct IntoIterRev<I, S> {
    next: Option<I>,
    yielded: usize,
    store: S,
}

impl<I, S> Iterator for IntoIterRev<I, S>
where
    I: Copy,
    S: IndexBackward<Index = I>,
{
    type Item = I;

    fn count(self) -> usize {
        self.len()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.len();

        (length, Some(length))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next.take()?;

        self.yielded += 1;
        self.next = self.store.next_before(result);

        Some(result)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if let Some(n) = n.checked_sub(1) {
            let index = self.next.take()?;

            match self.store.nth_before(n, index) {
                Ok(next) => {
                    self.next = Some(next);
                    self.yielded += n;
                }
                Err(remainder) => {
                    self.yielded += n - remainder.get();
                }
            }
        }

        self.next()
    }

    #[cfg(feature = "nightly")]
    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: Try<Output = B>,
    {
        let Some(index) = self.next.take() else {
            return R::from_output(init);
        };

        self.yielded = self.store.len();

        let init = f(init, index)?;

        self.store.try_fold_after(index, init, f)
    }
}

impl<I, S> ExactSizeIterator for IntoIterRev<I, S>
where
    I: Copy,
    S: IndexBackward<Index = I>,
{
    fn len(&self) -> usize {
        self.store.len() - self.yielded
    }

    #[cfg(feature = "nightly")]
    fn is_empty(&self) -> bool {
        self.store.len() == self.yielded
    }
}

impl<I, S> FusedIterator for IntoIterRev<I, S>
where
    I: Copy,
    S: IndexBackward<Index = I>,
{
}

#[cfg(test)]
mod basic_iteration_tests;

//
//  Iterator operations: drain, erase_if, retain.
//

impl<S> IndexSet<S>
where
    S: IndexForward + IndexStore,
{
    /// Clears the set, returning all elements as an iterator.
    pub fn drain(&mut self) -> Drain<'_, S::Index, S> {
        Drain {
            next: self.store.first(),
            yielded: 0,
            store: &mut self.store,
        }
    }

    /// Creates an iterator which uses a closure to determine if an element should be removed.
    ///
    /// The elements returned by the iterator are removed from the set.
    pub fn extract_if<F>(&mut self, pred: F) -> ExtractIf<'_, S::Index, S, F> {
        ExtractIf {
            pred,
            next: self.store.first(),
            passed: 0,
            store: &mut self.store,
        }
    }

    /// Retains only the elements specified by the predicate.
    pub fn retain<F>(&mut self, mut pred: F)
    where
        F: FnMut(S::Index) -> bool,
    {
        let mut cursor = self.store.first();

        while let Some(index) = cursor {
            if !pred(index) {
                self.store.remove(index);
            }

            cursor = self.store.next_after(index);
        }
    }
}

impl<S> IndexOrdSet<S>
where
    S: IndexForward + IndexStore,
{
    /// Clears the set, returning all elements as an iterator.
    pub fn drain(&mut self) -> Drain<'_, S::Index, S> {
        Drain {
            next: self.store.first(),
            yielded: 0,
            store: &mut self.store,
        }
    }

    /// Creates an iterator which uses a closure to determine if an element should be removed.
    ///
    /// The elements returned by the iterator are removed from the set.
    pub fn extract_if<F>(&mut self, pred: F) -> ExtractIf<'_, S::Index, S, F> {
        ExtractIf {
            pred,
            next: self.store.first(),
            passed: 0,
            store: &mut self.store,
        }
    }

    /// Retains only the elements specified by the predicate.
    pub fn retain<F>(&mut self, mut pred: F)
    where
        F: FnMut(S::Index) -> bool,
    {
        let mut cursor = self.store.first();

        while let Some(index) = cursor {
            if !pred(index) {
                self.store.remove(index);
            }

            cursor = self.store.next_after(index);
        }
    }
}

/// A draining iterator over the items of an `IndexSet`.
pub struct Drain<'a, I, S>
where
    S: IndexStore,
{
    next: Option<I>,
    yielded: usize,
    store: &'a mut S,
}

impl<'a, I, S> Drop for Drain<'a, I, S>
where
    S: IndexStore,
{
    fn drop(&mut self) {
        self.store.clear();
    }
}

impl<'a, I, S> Iterator for Drain<'a, I, S>
where
    I: Copy,
    S: IndexForward<Index = I> + IndexStore<Index = I>,
{
    type Item = S::Index;

    fn count(self) -> usize {
        self.len()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.len();

        (length, Some(length))
    }

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.take()?;

        self.yielded += 1;
        self.next = self.store.next_after(next);

        Some(next)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if let Some(n) = n.checked_sub(1) {
            let index = self.next.take()?;

            match self.store.nth_after(n, index) {
                Ok(next) => {
                    self.next = Some(next);
                    self.yielded += n;
                }
                Err(remainder) => {
                    self.yielded += n - remainder.get();
                }
            }
        }

        self.next()
    }
}

impl<'a, I, S> ExactSizeIterator for Drain<'a, I, S>
where
    I: Copy,
    S: IndexForward<Index = I> + IndexStore<Index = I>,
{
    fn len(&self) -> usize {
        self.store.len() - self.yielded
    }

    #[cfg(feature = "nightly")]
    fn is_empty(&self) -> bool {
        self.store.len() == self.yielded
    }
}

impl<'a, I, S> FusedIterator for Drain<'a, I, S>
where
    I: Copy,
    S: IndexForward<Index = I> + IndexStore<Index = I>,
{
}

/// An extractor iterator.
pub struct ExtractIf<'a, I, S, F> {
    pred: F,
    next: Option<I>,
    passed: usize,
    store: &'a mut S,
}

impl<'a, I, S, F> Iterator for ExtractIf<'a, I, S, F>
where
    I: Copy,
    S: IndexForward<Index = I> + IndexStore<Index = I>,
    F: FnMut(I) -> bool,
{
    type Item = S::Index;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.store.len() - self.passed;

        (0, Some(length))
    }

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(index) = self.next
            && !(self.pred)(index)
        {
            self.passed += 1;
            self.next = self.store.next_after(index);
        }

        let index = self.next.take()?;

        self.store.remove(index);

        self.passed += 1;
        self.next = self.store.next_after(index);

        Some(index)
    }
}

impl<'a, I, S, F> FusedIterator for ExtractIf<'a, I, S, F>
where
    I: Copy,
    S: IndexForward<Index = I> + IndexStore<Index = I>,
    F: FnMut(I) -> bool,
{
}

//  FIXME: implement chunk versions of the above.

#[cfg(test)]
mod extract_iteration_tests;

//
//  Iterator operations: difference, symmetric difference, intersection, union.
//

impl<S> IndexSet<S>
where
    S: IndexForward,
{
    /// Returns the indexes that are in `self`, but not `other`.
    pub fn difference<'a, OS>(&'a self, other: &'a IndexSet<OS>) -> Difference<'a, S::Index, S, OS>
    where
        OS: IndexStore<Index = S::Index>,
    {
        Difference {
            next: self.store.first(),
            passed: 0,
            left: &self.store,
            right: &other.store,
        }
    }

    /// Returns the indexes that are in `self` or in `other`, but not in both.
    pub fn symmetric_difference<'a, OS>(&'a self, other: &'a IndexSet<OS>) -> SymmetricDifference<'a, S::Index, S, OS>
    where
        OS: IndexForward<Index = S::Index>,
    {
        SymmetricDifference {
            next_left: self.store.first(),
            next_right: other.store.first(),
            passed: 0,
            left: &self.store,
            right: &other.store,
        }
    }

    /// Returns the indexes that are both in `self` and in `other`.
    ///
    /// Performance: if a set is known to contain less indexes than the other, then this set is used as `self`.
    pub fn intersection<'a, OS>(&'a self, other: &'a IndexSet<OS>) -> Intersection<'a, S::Index, S, OS>
    where
        OS: IndexStore<Index = S::Index>,
    {
        Intersection {
            next: self.store.first(),
            passed: 0,
            left: &self.store,
            right: &other.store,
        }
    }

    /// Returns the indexes that are in either one of `self` or `other`.
    pub fn union<'a, OS>(&'a self, other: &'a IndexSet<OS>) -> Union<'a, S::Index, S, OS>
    where
        OS: IndexForward<Index = S::Index>,
    {
        Union {
            next_left: self.store.first(),
            next_right: other.store.first(),
            passed: 0,
            left: &self.store,
            right: &other.store,
        }
    }
}

impl<S> IndexOrdSet<S>
where
    S: IndexOrdered,
{
    /// Returns the indexes that are in `self`, but not `other`.
    pub fn difference<'a, OS>(&'a self, other: &'a IndexOrdSet<OS>) -> Difference<'a, S::Index, S, OS>
    where
        OS: IndexStore<Index = S::Index>,
    {
        Difference {
            next: self.store.first(),
            passed: 0,
            left: &self.store,
            right: &other.store,
        }
    }

    /// Returns the indexes that are in `self` or in `other`, but not in both.
    ///
    /// Takes advantage of iteration over `self` and `other` being ordered to minimize the number of operations.
    pub fn symmetric_difference<'a, OS>(
        &'a self,
        other: &'a IndexOrdSet<OS>,
    ) -> SymmetricDifferenceOrd<'a, S::Index, S, OS>
    where
        OS: IndexOrdered<Index = S::Index>,
    {
        SymmetricDifferenceOrd {
            next_left: self.store.first(),
            next_right: other.store.first(),
            left: &self.store,
            right: &other.store,
        }
    }

    /// Returns the indexes that are both in `self` and in `other`.
    ///
    /// Takes advantage of iteration over `self` and `other` being ordered to minimize the number of operations.
    pub fn intersection<'a, OS>(&'a self, other: &'a IndexOrdSet<OS>) -> IntersectionOrd<'a, S::Index, S, OS>
    where
        OS: IndexOrdered<Index = S::Index>,
    {
        IntersectionOrd {
            next_left: self.store.first(),
            next_right: other.store.first(),
            left: &self.store,
            right: &other.store,
        }
    }

    /// Returns the indexes that are in either one of `self` or `other`.
    ///
    /// Takes advantage of iteration over `self` and `other` being ordered to minimize the number of operations.
    pub fn union<'a, OS>(&'a self, other: &'a IndexOrdSet<OS>) -> UnionOrd<'a, S::Index, S, OS>
    where
        OS: IndexOrdered<Index = S::Index>,
    {
        UnionOrd {
            next_left: self.store.first(),
            next_right: other.store.first(),
            left: &self.store,
            right: &other.store,
        }
    }
}

/// Iterator over the elements in L that are not in R.
pub struct Difference<'a, I, L, R> {
    next: Option<I>,
    passed: usize,
    left: &'a L,
    right: &'a R,
}

impl<'a, I, L, R> Iterator for Difference<'a, I, L, R>
where
    I: Copy,
    L: IndexForward<Index = I>,
    R: IndexStore<Index = I>,
{
    type Item = I;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.left.len() - self.passed;

        (0, Some(length))
    }

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(index) = self.next
            && self.right.contains(index)
        {
            self.passed += 1;
            self.next = self.left.next_after(index);
        }

        let result = self.next.take()?;

        self.passed += 1;
        self.next = self.left.next_after(result);

        Some(result)
    }
}

impl<'a, I, L, R> FusedIterator for Difference<'a, I, L, R>
where
    I: Copy,
    L: IndexForward<Index = I>,
    R: IndexStore<Index = I>,
{
}

/// Iterator over the elements in L xor in R.
pub struct SymmetricDifference<'a, I, L, R> {
    next_left: Option<I>,
    next_right: Option<I>,
    passed: usize,
    left: &'a L,
    right: &'a R,
}

impl<'a, I, L, R> Iterator for SymmetricDifference<'a, I, L, R>
where
    I: Copy,
    L: IndexForward<Index = I>,
    R: IndexForward<Index = I>,
{
    type Item = I;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.left.len() + self.right.len() - self.passed))
    }

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(index) = self.next_left
            && self.right.contains(index)
        {
            self.passed += 1;
            self.next_left = self.left.next_after(index);
        }

        if let Some(result) = self.next_left.take() {
            self.passed += 1;
            self.next_left = self.left.next_after(result);

            return Some(result);
        }

        while let Some(index) = self.next_right
            && self.left.contains(index)
        {
            self.passed += 1;
            self.next_right = self.right.next_after(index);
        }

        let result = self.next_right.take()?;

        self.passed += 1;
        self.next_right = self.right.next_after(result);

        Some(result)
    }
}

impl<'a, I, L, R> FusedIterator for SymmetricDifference<'a, I, L, R>
where
    I: Copy,
    L: IndexForward<Index = I>,
    R: IndexForward<Index = I>,
{
}

/// Iterator over the element in L and in R.
pub struct Intersection<'a, I, L, R> {
    next: Option<I>,
    passed: usize,
    left: &'a L,
    right: &'a R,
}

impl<'a, I, L, R> Iterator for Intersection<'a, I, L, R>
where
    I: Copy,
    L: IndexForward<Index = I>,
    R: IndexStore<Index = I>,
{
    type Item = I;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = self.left.len() - self.passed;
        let right = self.right.len();

        (0, Some(cmp::min(left, right)))
    }

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(index) = self.next
            && !self.right.contains(index)
        {
            self.passed += 1;
            self.next = self.left.next_after(index);
        }

        let result = self.next.take()?;

        self.passed += 1;
        self.next = self.left.next_after(result);

        Some(result)
    }
}

impl<'a, I, L, R> FusedIterator for Intersection<'a, I, L, R>
where
    I: Copy,
    L: IndexForward<Index = I>,
    R: IndexStore<Index = I>,
{
}

/// Iterator over the elements in L or in R.
pub struct Union<'a, I, L, R> {
    next_left: Option<I>,
    next_right: Option<I>,
    passed: usize,
    left: &'a L,
    right: &'a R,
}

impl<'a, I, L, R> Iterator for Union<'a, I, L, R>
where
    I: Copy,
    L: IndexForward<Index = I>,
    R: IndexForward<Index = I>,
{
    type Item = I;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.left.len() + self.right.len() - self.passed))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(result) = self.next_left {
            self.passed += 1;
            self.next_left = self.left.next_after(result);

            return Some(result);
        }

        //  Elements in `self.left` were already returned, so skip them.
        while let Some(index) = self.next_right
            && self.left.contains(index)
        {
            self.passed += 1;
            self.next_right = self.right.next_after(index);
        }

        let result = self.next_right.take()?;

        self.passed += 1;
        self.next_right = self.right.next_after(result);

        Some(result)
    }
}

impl<'a, I, L, R> FusedIterator for Union<'a, I, L, R>
where
    I: Copy,
    L: IndexForward<Index = I>,
    R: IndexForward<Index = I>,
{
}

/// Iterator over the elements in L xor in R.
pub struct SymmetricDifferenceOrd<'a, I, L, R> {
    next_left: Option<I>,
    next_right: Option<I>,
    left: &'a L,
    right: &'a R,
}

impl<'a, I, L, R> Iterator for SymmetricDifferenceOrd<'a, I, L, R>
where
    I: Copy + Eq + Ord,
    L: IndexOrdered<Index = I>,
    R: IndexOrdered<Index = I>,
{
    type Item = I;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = self.next_left.map(|_| self.left.len()).unwrap_or(0);
        let right = self.next_right.map(|_| self.right.len()).unwrap_or(0);

        (0, Some(left + right))
    }

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match (self.next_left, self.next_right) {
                (None, None) => return None,
                (Some(next_left), None) => {
                    self.next_left = self.left.next_after(next_left);

                    return Some(next_left);
                }
                (None, Some(next_right)) => {
                    self.next_right = self.right.next_after(next_right);

                    return Some(next_right);
                }
                (Some(next_left), Some(next_right)) => match next_left.cmp(&next_right) {
                    Ordering::Equal => {
                        self.next_left = self.left.next_after(next_left);
                        self.next_right = self.right.next_after(next_right);

                        continue;
                    }
                    Ordering::Less => {
                        self.next_left = self.left.next_after(next_left);

                        return Some(next_left);
                    }
                    Ordering::Greater => {
                        self.next_right = self.right.next_after(next_right);

                        return Some(next_right);
                    }
                },
            }
        }
    }
}

/// Iterator over the element in L and in R.
pub struct IntersectionOrd<'a, I, L, R> {
    next_left: Option<I>,
    next_right: Option<I>,
    left: &'a L,
    right: &'a R,
}

impl<'a, I, L, R> Iterator for IntersectionOrd<'a, I, L, R>
where
    I: Copy + Eq + Ord,
    L: IndexOrdered<Index = I>,
    R: IndexOrdered<Index = I>,
{
    type Item = I;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = self.next_left.map(|_| self.left.len()).unwrap_or(0);
        let right = self.next_right.map(|_| self.right.len()).unwrap_or(0);

        (0, Some(cmp::min(left, right)))
    }

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_left = self.next_left.take()?;
            let next_right = self.next_right.take()?;

            match next_left.cmp(&next_right) {
                Ordering::Equal => {
                    self.next_left = self.left.next_after(next_left);
                    self.next_right = self.right.next_after(next_right);

                    return Some(next_left);
                }
                Ordering::Less => {
                    self.next_left = self.left.next_after(next_right);
                    self.next_right = self.right.next_after(next_right);

                    if self.left.contains(next_right) {
                        return Some(next_right);
                    }

                    continue;
                }
                Ordering::Greater => {
                    self.next_left = self.left.next_after(next_left);
                    self.next_right = self.right.next_after(next_left);

                    if self.right.contains(next_left) {
                        return Some(next_left);
                    }

                    continue;
                }
            }
        }
    }
}

/// Iterator over the elements in L or in R.
pub struct UnionOrd<'a, I, L, R> {
    next_left: Option<I>,
    next_right: Option<I>,
    left: &'a L,
    right: &'a R,
}

impl<'a, I, L, R> Iterator for UnionOrd<'a, I, L, R>
where
    I: Copy + Eq + Ord,
    L: IndexOrdered<Index = I>,
    R: IndexOrdered<Index = I>,
{
    type Item = I;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = self.next_left.map(|_| self.left.len()).unwrap_or(0);
        let right = self.next_right.map(|_| self.right.len()).unwrap_or(0);

        (0, Some(left + right))
    }

    fn next(&mut self) -> Option<Self::Item> {
        match (self.next_left, self.next_right) {
            (None, None) => None,
            (Some(next_left), None) => {
                self.next_left = self.left.next_after(next_left);

                Some(next_left)
            }
            (None, Some(next_right)) => {
                self.next_right = self.right.next_after(next_right);

                Some(next_right)
            }
            (Some(next_left), Some(next_right)) => match next_left.cmp(&next_right) {
                Ordering::Equal => {
                    self.next_left = self.left.next_after(next_left);
                    self.next_right = self.right.next_after(next_right);

                    Some(next_left)
                }
                Ordering::Less => {
                    self.next_left = self.left.next_after(next_left);

                    Some(next_left)
                }
                Ordering::Greater => {
                    self.next_right = self.right.next_after(next_right);

                    Some(next_right)
                }
            },
        }
    }
}

//  FIXME: implement chunk versions of the above.

#[cfg(test)]
mod dual_iteration_tests;

//
//  Bitwise operations.
//

impl<S> IndexSet<S>
where
    S: IndexStore,
{
    /// Removes all indexes of `self` not contained in `other`.
    pub fn bitand_assign<OS>(&mut self, other: &IndexSet<OS>)
    where
        S: IndexForward,
        OS: IndexView<Index = S::Index>,
    {
        self.retain(|index| other.contains(index));
    }

    /// Inserts all indexes of `other` not contained in `self`.
    pub fn bitor_assign<OS>(&mut self, other: &IndexSet<OS>)
    where
        S: IndexStore<InsertionError = Never>,
        OS: IndexForward<Index = S::Index>,
    {
        other.iter().for_each(|index| {
            let _ = self.store.insert(index);
        });
    }

    /// Removes all indexes of `other` from `self`.
    pub fn sub_assign<OS>(&mut self, other: &IndexSet<OS>)
    where
        OS: IndexForward<Index = S::Index>,
    {
        other.iter().for_each(|index| {
            self.store.remove(index);
        });
    }
}

impl<S> IndexOrdSet<S>
where
    S: IndexOrdered,
{
    /// Removes all indexes of `self` not contained in `other`.
    pub fn bitand_assign<OS>(&mut self, other: &IndexOrdSet<OS>)
    where
        S: IndexStore,
        OS: IndexView<Index = S::Index>,
    {
        self.retain(|index| other.contains(index));
    }

    /// Inserts all indexes of `other` not contained in `self`.
    pub fn bitor_assign<OS>(&mut self, other: &IndexOrdSet<OS>)
    where
        S: IndexStore<InsertionError = Never>,
        OS: IndexForward<Index = S::Index>,
    {
        other.iter().for_each(|index| {
            let _ = self.store.insert(index);
        });
    }

    /// Removes all indexes of `other` from `self`.
    pub fn sub_assign<OS>(&mut self, other: &IndexOrdSet<OS>)
    where
        S: IndexStore,
        OS: IndexForward<Index = S::Index>,
    {
        other.iter().for_each(|index| {
            self.store.remove(index);
        });
    }

    /// Inserts all indexes of `other` not contained in `self`, while removing all indexes of `self` also contained in
    /// `other`.
    pub fn bitxor_assign<OS>(&mut self, other: &IndexOrdSet<OS>)
    where
        S: IndexStore<InsertionError = Never>,
        OS: IndexOrdered<Index = S::Index>,
    {
        let mut next_self = self.store.first();
        let mut next_other = other.store.first();

        loop {
            match (next_self, next_other) {
                (_, None) => break,
                (None, Some(_)) => {
                    while let Some(that) = next_other {
                        let _ = self.store.insert(that);

                        next_other = other.store.next_after(that);
                    }

                    break;
                }
                (Some(this), Some(that)) => match this.cmp(&that) {
                    Ordering::Equal => {
                        self.store.remove(this);

                        next_self = self.store.next_after(this);
                        next_other = other.store.next_after(that);
                    }
                    Ordering::Less => {
                        next_self = self.store.next_after(this);
                    }
                    Ordering::Greater => {
                        let _ = self.store.insert(that);

                        next_other = other.store.next_after(that);
                    }
                },
            }
        }
    }
}

//  FIXME: implement chunk versions of the above.

#[cfg(test)]
mod bitwise_tests;

//
//  Bitwise operators: IndexSet.
//

impl<S, OS> ops::BitAndAssign<IndexSet<OS>> for IndexSet<S>
where
    S: IndexForward + IndexStore,
    OS: IndexStore<Index = S::Index>,
{
    fn bitand_assign(&mut self, other: IndexSet<OS>) {
        self.bitand_assign(&other);
    }
}

impl<S, OS> ops::BitAndAssign<&IndexSet<OS>> for IndexSet<S>
where
    S: IndexForward + IndexStore,
    OS: IndexStore<Index = S::Index>,
{
    fn bitand_assign(&mut self, other: &IndexSet<OS>) {
        self.bitand_assign(other);
    }
}

impl<S, OS> ops::BitOrAssign<IndexSet<OS>> for IndexSet<S>
where
    S: IndexStore<InsertionError = Never>,
    OS: IndexForward<Index = S::Index>,
{
    fn bitor_assign(&mut self, other: IndexSet<OS>) {
        self.bitor_assign(&other);
    }
}

impl<S, OS> ops::BitOrAssign<&IndexSet<OS>> for IndexSet<S>
where
    S: IndexStore<InsertionError = Never>,
    OS: IndexForward<Index = S::Index>,
{
    fn bitor_assign(&mut self, other: &IndexSet<OS>) {
        self.bitor_assign(other);
    }
}

impl<S, OS> ops::SubAssign<IndexSet<OS>> for IndexSet<S>
where
    S: IndexStore,
    OS: IndexForward<Index = S::Index>,
{
    fn sub_assign(&mut self, other: IndexSet<OS>) {
        self.sub_assign(&other);
    }
}

impl<S, OS> ops::SubAssign<&IndexSet<OS>> for IndexSet<S>
where
    S: IndexStore,
    OS: IndexForward<Index = S::Index>,
{
    fn sub_assign(&mut self, other: &IndexSet<OS>) {
        self.sub_assign(other);
    }
}

impl<S, OS> ops::BitAnd<IndexSet<OS>> for IndexSet<S>
where
    S: IndexForward + IndexStore,
    OS: IndexStore<Index = S::Index>,
{
    type Output = Self;

    fn bitand(mut self, other: IndexSet<OS>) -> Self::Output {
        self.bitand_assign(&other);

        self
    }
}

impl<S, OS> ops::BitAnd<&IndexSet<OS>> for IndexSet<S>
where
    S: IndexForward + IndexStore,
    OS: IndexStore<Index = S::Index>,
{
    type Output = Self;

    fn bitand(mut self, other: &IndexSet<OS>) -> Self::Output {
        self.bitand_assign(other);

        self
    }
}

impl<S, OS> ops::BitOr<IndexSet<OS>> for IndexSet<S>
where
    S: IndexStore<InsertionError = Never>,
    OS: IndexForward<Index = S::Index>,
{
    type Output = Self;

    fn bitor(mut self, other: IndexSet<OS>) -> Self::Output {
        self.bitor_assign(&other);

        self
    }
}

impl<S, OS> ops::BitOr<&IndexSet<OS>> for IndexSet<S>
where
    S: IndexStore<InsertionError = Never>,
    OS: IndexForward<Index = S::Index>,
{
    type Output = Self;

    fn bitor(mut self, other: &IndexSet<OS>) -> Self::Output {
        self.bitor_assign(other);

        self
    }
}

impl<S, OS> ops::Sub<IndexSet<OS>> for IndexSet<S>
where
    S: IndexStore,
    OS: IndexForward<Index = S::Index>,
{
    type Output = Self;

    fn sub(mut self, other: IndexSet<OS>) -> Self::Output {
        self.sub_assign(&other);

        self
    }
}

impl<S, OS> ops::Sub<&IndexSet<OS>> for IndexSet<S>
where
    S: IndexStore,
    OS: IndexForward<Index = S::Index>,
{
    type Output = Self;

    fn sub(mut self, other: &IndexSet<OS>) -> Self::Output {
        self.sub_assign(other);

        self
    }
}

//
//  Bitwise operators: IndexOrdSet.
//

impl<S, OS> ops::BitAndAssign<IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore,
    OS: IndexStore<Index = S::Index>,
{
    fn bitand_assign(&mut self, other: IndexOrdSet<OS>) {
        self.bitand_assign(&other);
    }
}

impl<S, OS> ops::BitAndAssign<&IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore,
    OS: IndexStore<Index = S::Index>,
{
    fn bitand_assign(&mut self, other: &IndexOrdSet<OS>) {
        self.bitand_assign(other);
    }
}

impl<S, OS> ops::BitOrAssign<IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore<InsertionError = Never>,
    OS: IndexForward<Index = S::Index>,
{
    fn bitor_assign(&mut self, other: IndexOrdSet<OS>) {
        self.bitor_assign(&other);
    }
}

impl<S, OS> ops::BitOrAssign<&IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore<InsertionError = Never>,
    OS: IndexForward<Index = S::Index>,
{
    fn bitor_assign(&mut self, other: &IndexOrdSet<OS>) {
        self.bitor_assign(other);
    }
}

impl<S, OS> ops::BitXorAssign<IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore<InsertionError = Never>,
    OS: IndexOrdered<Index = S::Index>,
{
    fn bitxor_assign(&mut self, other: IndexOrdSet<OS>) {
        self.bitxor_assign(&other);
    }
}

impl<S, OS> ops::BitXorAssign<&IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore<InsertionError = Never>,
    OS: IndexOrdered<Index = S::Index>,
{
    fn bitxor_assign(&mut self, other: &IndexOrdSet<OS>) {
        self.bitxor_assign(other);
    }
}

impl<S, OS> ops::SubAssign<IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore,
    OS: IndexForward<Index = S::Index>,
{
    fn sub_assign(&mut self, other: IndexOrdSet<OS>) {
        self.sub_assign(&other);
    }
}

impl<S, OS> ops::SubAssign<&IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore,
    OS: IndexForward<Index = S::Index>,
{
    fn sub_assign(&mut self, other: &IndexOrdSet<OS>) {
        self.sub_assign(other);
    }
}

impl<S, OS> ops::BitAnd<IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore,
    OS: IndexView<Index = S::Index>,
{
    type Output = Self;

    fn bitand(mut self, other: IndexOrdSet<OS>) -> Self::Output {
        self.bitand_assign(&other);

        self
    }
}

impl<S, OS> ops::BitAnd<&IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore,
    OS: IndexView<Index = S::Index>,
{
    type Output = Self;

    fn bitand(mut self, other: &IndexOrdSet<OS>) -> Self::Output {
        self.bitand_assign(other);

        self
    }
}

impl<S, OS> ops::BitOr<IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore<InsertionError = Never>,
    OS: IndexForward<Index = S::Index>,
{
    type Output = Self;

    fn bitor(mut self, other: IndexOrdSet<OS>) -> Self::Output {
        self.bitor_assign(&other);

        self
    }
}

impl<S, OS> ops::BitOr<&IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore<InsertionError = Never>,
    OS: IndexForward<Index = S::Index>,
{
    type Output = Self;

    fn bitor(mut self, other: &IndexOrdSet<OS>) -> Self::Output {
        self.bitor_assign(other);

        self
    }
}

impl<S, OS> ops::Sub<IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore,
    OS: IndexForward<Index = S::Index>,
{
    type Output = Self;

    fn sub(mut self, other: IndexOrdSet<OS>) -> Self::Output {
        self.sub_assign(&other);

        self
    }
}

impl<S, OS> ops::Sub<&IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore,
    OS: IndexForward<Index = S::Index>,
{
    type Output = Self;

    fn sub(mut self, other: &IndexOrdSet<OS>) -> Self::Output {
        self.sub_assign(other);

        self
    }
}

impl<S, OS> ops::BitXor<IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore<InsertionError = Never>,
    OS: IndexOrdered<Index = S::Index>,
{
    type Output = Self;

    fn bitxor(mut self, other: IndexOrdSet<OS>) -> Self::Output {
        self.bitxor_assign(&other);

        self
    }
}

impl<S, OS> ops::BitXor<&IndexOrdSet<OS>> for IndexOrdSet<S>
where
    S: IndexOrdered + IndexStore<InsertionError = Never>,
    OS: IndexOrdered<Index = S::Index>,
{
    type Output = Self;

    fn bitxor(mut self, other: &IndexOrdSet<OS>) -> Self::Output {
        self.bitxor_assign(other);

        self
    }
}

//  FIXME: tests
