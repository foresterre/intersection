//! A crate to find the intersection between a collection of sets. Convenient when your
//! collection of sets consists of more than 2 sets.
//!
//! There are two options:
//! * With [`HashSet`], use [`hash_set::intersection`].
//! * With [`BTreeSet`], use [`btree_set::intersection`].
//!
//! **Why use this over [`HashSet::intersection`] or [`BTreeSet::intersection`]?**
//!
//! The standard library intersection methods require a bit of ceremony when you want to intersect 3
//! or more sets with each other. These `intersection(other: &Set)` methods produce a lazy iterator,
//! which you then have to wrap into a `HashSet::from_iter` again. An alternative way is to use
//! the `BitAnd` implementation of either set. In both cases it requires a bit of boilerplate
//! which this library does for you =).
//!
//! [`HashSet::intersection`]: https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html#method.intersection
//! [`BTreeSet::intersection`]: https://doc.rust-lang.org/alloc/collections/btree_set/struct.BTreeSet.html#method.intersection
//! [`HashSet`]: https://doc.rust-lang.org/std/collections/hash_set/struct.HashSet.html#method.intersection
//! [`hash_set::intersection`]: hash_set::intersection
//! [`BTreeSet`]: https://doc.rust-lang.org/alloc/collections/btree_set/struct.BTreeSet.html
//! [`btree_set::intersection`]: btree_set::intersection

pub mod hash_set {
    use std::collections::HashSet;
    use std::hash::Hash;

    /// Takes the intersection between the given `Set` iterables, with common type `T`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use intersection::hash_set;
    ///
    /// let tokens = "hello,world,common";
    /// let sets = tokens.split(',') // split to words, each word is a set.
    ///     .map(|word| word.chars()); // split word to chars, each char is an element of the set.
    ///
    /// // we look for the common letters between the 3 words
    /// let common_letters = hash_set::intersection(sets);
    ///
    /// let expected = HashSet::from(['o']);
    /// assert_eq!(common_letters, expected);
    /// ```
    pub fn intersection<I, Set, T>(sets: I) -> HashSet<T>
    where
        I: IntoIterator<Item = Set>,
        Set: IntoIterator<Item = T>,
        T: Clone + Eq + Hash,
    {
        sets.into_iter()
            .map(|set| {
                let set: HashSet<T> = HashSet::from_iter(set);
                set
            })
            .reduce(|l, r| &l & &r)
            .unwrap_or_default()
    }
}

pub mod btree_set {
    use std::collections::BTreeSet;

    /// Takes the intersection between the given `Set` iterables, with common type `T`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use intersection::{btree_set};
    ///
    /// let tokens = "harry,hairy,happy";
    /// let sets = tokens.split(',') // split to words, each word is a set.
    ///     .map(|word| word.chars()); // split word to chars, each char is an element of the set.
    ///
    /// // we look for the common letters between the 3 words
    /// let common_letters = btree_set::intersection(sets);
    ///
    /// let expected = BTreeSet::from(['h', 'a', 'y']);
    /// assert_eq!(common_letters, expected);
    /// ```
    pub fn intersection<I, Set, T>(sets: I) -> BTreeSet<T>
    where
        I: IntoIterator<Item = Set>,
        Set: IntoIterator<Item = T>,
        T: Clone + Ord,
    {
        sets.into_iter()
            .map(|set| {
                let set: BTreeSet<T> = BTreeSet::from_iter(set);
                set
            })
            .reduce(|l, r| &l & &r)
            .unwrap_or_default()
    }
}
