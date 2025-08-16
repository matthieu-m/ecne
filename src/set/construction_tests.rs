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
    }

    #[test]
    fn default() {
        let victim = Victim::default();

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());
    }

    #[test]
    fn from_iterator_empty() {
        const EMPTY: [u8; 0] = [];
        const SOME: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        {
            let victim: Victim = EMPTY.into_iter().collect();

            assert!(victim.is_empty());
            assert_eq!(0, victim.len());
        }

        {
            let victim: Victim = SOME.into_iter().collect();

            assert!(!victim.is_empty());
            assert_eq!(SOME.len(), victim.len());
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
    }

    #[test]
    fn default() {
        let victim = Victim::new();

        assert!(victim.is_empty());
        assert_eq!(0, victim.len());
    }

    #[test]
    fn from_iterator_empty() {
        const EMPTY: [u8; 0] = [];
        const SOME: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        {
            let victim: Victim = EMPTY.into_iter().collect();

            assert!(victim.is_empty());
            assert_eq!(0, victim.len());
        }

        {
            let victim: Victim = SOME.into_iter().collect();

            assert!(!victim.is_empty());
            assert_eq!(SOME.len(), victim.len());
        }
    }
} // index_ord_set
