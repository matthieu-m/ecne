//! Unit tests for inclusion operations.

mod index_set {
    use std::collections::BTreeSet;

    use crate::set::IndexSet;

    type Victim = IndexSet<BTreeSet<u8>>;

    #[test]
    fn is_disjoint() {
        let primes = Victim::from_iter([1, 2, 3, 5]);
        let evens = Victim::from_iter([2, 4, 6, 8]);
        let perfects = Victim::from_iter([36]);

        assert!(!primes.is_disjoint(&evens));
        assert!(!evens.is_disjoint(&primes));

        assert!(primes.is_disjoint(&perfects));
        assert!(perfects.is_disjoint(&primes));
    }

    #[test]
    fn is_subset_superset() {
        let primes = Victim::from_iter([1, 2, 3, 5, 7]);
        let odds = Victim::from_iter([1, 3, 5, 7]);
        let evens = Victim::from_iter([2, 4, 6, 8]);

        assert!(odds.is_subset(&odds));
        assert!(odds.is_superset(&odds));

        assert!(odds.is_subset(&primes));
        assert!(!odds.is_superset(&primes));

        assert!(primes.is_superset(&odds));
        assert!(!primes.is_subset(&odds));

        assert!(!primes.is_subset(&evens));
        assert!(!primes.is_superset(&evens));
        assert!(!evens.is_subset(&primes));
        assert!(!evens.is_superset(&primes));
    }
} // mod index_set

mod index_ord_set {
    use alloc::collections::BTreeSet;

    use crate::set::IndexOrdSet;

    type Victim = IndexOrdSet<BTreeSet<u8>>;

    #[test]
    fn is_disjoint() {
        let primes = Victim::from_iter([1, 2, 3, 5]);
        let evens = Victim::from_iter([2, 4, 6, 8]);
        let perfects = Victim::from_iter([36]);

        assert!(!primes.is_disjoint(&evens));
        assert!(!evens.is_disjoint(&primes));

        assert!(primes.is_disjoint(&perfects));
        assert!(perfects.is_disjoint(&primes));
    }

    #[test]
    fn is_subset_superset() {
        let primes = Victim::from_iter([1, 2, 3, 5, 7]);
        let odds = Victim::from_iter([1, 3, 5, 7]);
        let evens = Victim::from_iter([2, 4, 6, 8]);

        assert!(odds.is_subset(&odds));
        assert!(odds.is_superset(&odds));

        assert!(odds.is_subset(&primes));
        assert!(!odds.is_superset(&primes));

        assert!(primes.is_superset(&odds));
        assert!(!primes.is_subset(&odds));

        assert!(!primes.is_subset(&evens));
        assert!(!primes.is_superset(&evens));
        assert!(!evens.is_subset(&primes));
        assert!(!evens.is_superset(&primes));
    }
} // mod index_ord_set

mod index_chunked_set {
    use crate::{
        chunk::{ArrayChunk, UnsignedChunk},
        set::IndexChunkedSet,
    };

    type Victim = IndexChunkedSet<ArrayChunk<UnsignedChunk<u8>, 2>>;

    #[test]
    fn is_disjoint() {
        let primes = Victim::from_iter([1, 2, 3, 5]);
        let evens = Victim::from_iter([2, 4, 6, 8]);
        let perfects = Victim::from_iter([36]);

        assert!(!primes.is_disjoint(&evens));
        assert!(!evens.is_disjoint(&primes));

        assert!(primes.is_disjoint(&perfects));
        assert!(perfects.is_disjoint(&primes));
    }

    #[test]
    fn is_subset_superset() {
        let primes = Victim::from_iter([1, 2, 3, 5, 7]);
        let odds = Victim::from_iter([1, 3, 5, 7]);
        let evens = Victim::from_iter([2, 4, 6, 8]);

        assert!(odds.is_subset(&odds));
        assert!(odds.is_superset(&odds));

        assert!(odds.is_subset(&primes));
        assert!(!odds.is_superset(&primes));

        assert!(primes.is_superset(&odds));
        assert!(!primes.is_subset(&odds));

        assert!(!primes.is_subset(&evens));
        assert!(!primes.is_superset(&evens));
        assert!(!evens.is_subset(&primes));
        assert!(!evens.is_superset(&primes));
    }
} // mod index_chunked_set
