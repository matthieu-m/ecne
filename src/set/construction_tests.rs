//! Unit tests for constructing sets.

mod index_set {
    use std::collections::HashSet;

    use crate::set::IndexSet;

    type Victim = IndexSet<HashSet<u8>>;

    #[test]
    fn new() {
        let victim = Victim::new();

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());
        assert_eq!(0, victim.capacity());
    }

    #[test]
    fn with_capacity() {
        const CAPACITY: usize = 42;

        let victim = Victim::with_capacity(CAPACITY);

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());
        assert!(CAPACITY < victim.capacity(), "{CAPACITY} < {}", victim.capacity());
    }

    #[test]
    fn default() {
        let victim = Victim::default();

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());
        assert_eq!(0, victim.capacity());
    }

    #[test]
    fn from_iterator_empty() {
        const EMPTY: [u8; 0] = [];
        const SOME: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        {
            let victim: Victim = EMPTY.into_iter().collect();

            assert!(victim.is_empty());
            assert_eq!(0, victim.len());
            assert_eq!(0, victim.capacity());
        }

        {
            let victim: Victim = SOME.into_iter().collect();

            assert!(!victim.is_empty());
            assert_eq!(SOME.len(), victim.len());
            assert!(
                SOME.len() <= victim.capacity(),
                "{} > {}",
                SOME.len(),
                victim.capacity()
            );
        }
    }
} // index_set

mod index_ord_set {
    use alloc::collections::BTreeSet;

    use crate::set::IndexOrdSet;

    type Victim = IndexOrdSet<BTreeSet<u8>>;

    #[test]
    fn new() {
        let victim = Victim::new();

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());
        assert_eq!(0, victim.capacity());
    }

    #[test]
    fn with_capacity() {
        const CAPACITY: usize = 42;

        let victim = Victim::with_capacity(CAPACITY);

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());
        assert_eq!(0, victim.capacity());
    }

    #[test]
    fn default() {
        let victim = Victim::new();

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());
        assert_eq!(0, victim.capacity());
    }

    #[test]
    fn from_iterator_empty() {
        const EMPTY: [u8; 0] = [];
        const SOME: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        {
            let victim: Victim = EMPTY.into_iter().collect();

            assert!(victim.is_empty());
            assert_eq!(0, victim.len());
            assert_eq!(0, victim.capacity());
        }

        {
            let victim: Victim = SOME.into_iter().collect();

            assert!(!victim.is_empty());
            assert_eq!(SOME.len(), victim.len());
            assert!(
                SOME.len() <= victim.capacity(),
                "{} > {}",
                SOME.len(),
                victim.capacity()
            );
        }
    }
} // index_ord_set
