//! Unit tests for overall operations.

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
} // mod index_ord_set
