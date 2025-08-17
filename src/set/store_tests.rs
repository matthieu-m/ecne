//! Unit tests for store operations.

mod index_set {
    use std::collections::HashSet;

    use crate::set::IndexSet;

    type Victim = IndexSet<HashSet<u8>>;

    #[test]
    fn clear() {
        const INDEXES: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        let mut victim: Victim = INDEXES.into_iter().collect();

        victim.clear();

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());
    }

    #[test]
    fn insert_remove() {
        const INDEX: u8 = 42;

        let mut victim = Victim::new();

        for _ in 0..3 {
            assert!(!victim.contains(INDEX));

            assert!(victim.insert(INDEX).unwrap());
            assert!(!victim.insert(INDEX).unwrap());
            assert!(!victim.insert(INDEX).unwrap());

            assert!(victim.remove(INDEX));
            assert!(!victim.remove(INDEX));
            assert!(!victim.remove(INDEX));
        }
    }

    #[test]
    fn extend() {
        const EMPTY: [u8; 0] = [];
        const SOME: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        {
            let mut victim = Victim::new();

            victim.extend(EMPTY);

            assert!(victim.is_empty());
            assert!(!victim.contains(0));
            assert!(!victim.contains(1));
        }

        {
            let mut victim = Victim::new();

            victim.extend(SOME);

            assert!(!victim.is_empty());
            assert_eq!(SOME.len(), victim.len());

            assert!(!victim.contains(0));

            for index in SOME {
                assert!(victim.contains(index));
            }
        }
    }
} // mod index_set

mod index_ord_set {
    use alloc::collections::BTreeSet;

    use crate::set::IndexOrdSet;

    type Victim = IndexOrdSet<BTreeSet<u8>>;

    #[test]
    fn clear() {
        const INDEXES: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        let mut victim: Victim = INDEXES.into_iter().collect();

        victim.clear();

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());
    }

    #[test]
    fn insert_remove() {
        const INDEX: u8 = 42;

        let mut victim = Victim::new();

        for _ in 0..3 {
            assert!(!victim.contains(INDEX));

            assert!(victim.insert(INDEX).unwrap());
            assert!(!victim.insert(INDEX).unwrap());
            assert!(!victim.insert(INDEX).unwrap());

            assert!(victim.remove(INDEX));
            assert!(!victim.remove(INDEX));
            assert!(!victim.remove(INDEX));
        }
    }

    #[test]
    fn extend() {
        const EMPTY: [u8; 0] = [];
        const SOME: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        {
            let mut victim = Victim::new();

            victim.extend(EMPTY);

            assert!(victim.is_empty());
            assert!(!victim.contains(0));
            assert!(!victim.contains(1));
        }

        {
            let mut victim = Victim::new();

            victim.extend(SOME);

            assert!(!victim.is_empty());
            assert_eq!(SOME.len(), victim.len());

            assert!(!victim.contains(0));

            for index in SOME {
                assert!(victim.contains(index));
            }
        }
    }
} // mod index_ord_set

mod index_chunked_set {
    use crate::{
        chunk::{ArrayChunk, UnsignedChunk},
        set::IndexChunkedSet,
    };

    type Victim = IndexChunkedSet<ArrayChunk<UnsignedChunk<u8>, 8>>;

    #[test]
    fn clear() {
        const INDEXES: [u16; 7] = [1, 2, 3, 5, 7, 11, 13];

        let mut victim: Victim = INDEXES.into_iter().collect();

        victim.clear();

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());
    }

    #[test]
    fn insert_remove() {
        const INDEX: u16 = 42;

        let mut victim = Victim::new();

        for _ in 0..3 {
            assert!(!victim.contains(INDEX));

            assert!(victim.insert(INDEX).unwrap());
            assert!(!victim.insert(INDEX).unwrap());
            assert!(!victim.insert(INDEX).unwrap());

            assert!(victim.remove(INDEX));
            assert!(!victim.remove(INDEX));
            assert!(!victim.remove(INDEX));
        }
    }

    #[test]
    fn extend() {
        const EMPTY: [u16; 0] = [];
        const SOME: [u16; 7] = [1, 2, 3, 5, 7, 11, 13];

        {
            let mut victim = Victim::new();

            victim.extend(EMPTY);

            assert!(victim.is_empty());
            assert!(!victim.contains(0));
            assert!(!victim.contains(1));
        }

        {
            let mut victim = Victim::new();

            victim.extend(SOME);

            assert!(!victim.is_empty());
            assert_eq!(SOME.len(), victim.len());

            assert!(!victim.contains(0));

            for index in SOME {
                assert!(victim.contains(index));
            }
        }
    }
} // mod index_chunked_set
