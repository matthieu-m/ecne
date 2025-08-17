//! Unit tests for individual operations.

mod index_set {
    use std::collections::BTreeSet;

    use crate::set::IndexSet;

    use super::helper;

    type Victim = IndexSet<BTreeSet<u8>>;

    #[test]
    fn forward_iter() {
        const INDEXES: [u8; 4] = [1, 2, 3, 5];

        let victim = Victim::from_iter(INDEXES);

        helper::assert_exact_iterator(victim.iter(), INDEXES);
    }

    #[test]
    fn forward_into_iter() {
        const INDEXES: [u8; 4] = [1, 2, 3, 5];

        let victim = Victim::from_iter(INDEXES);

        helper::assert_exact_iterator(victim.into_iter(), INDEXES);
    }

    #[test]
    fn backward_iter() {
        const INDEXES: [u8; 4] = [1, 2, 3, 5];

        let victim = Victim::from_iter(INDEXES);

        helper::assert_exact_iterator(victim.iter_rev(), INDEXES.into_iter().rev());
    }

    #[test]
    fn backward_into_iter() {
        const INDEXES: [u8; 4] = [1, 2, 3, 5];

        let victim = Victim::from_iter(INDEXES);

        helper::assert_exact_iterator(victim.into_iter_rev(), INDEXES.into_iter().rev());
    }
} // mod index_set

mod index_ord_set {
    use alloc::collections::BTreeSet;

    use crate::set::IndexOrdSet;

    use super::helper;

    type Victim = IndexOrdSet<BTreeSet<u8>>;

    #[test]
    fn forward_iter() {
        const INDEXES: [u8; 4] = [1, 2, 3, 5];

        let victim = Victim::from_iter(INDEXES);

        helper::assert_exact_iterator(victim.iter(), INDEXES);
    }

    #[test]
    fn forward_into_iter() {
        const INDEXES: [u8; 4] = [1, 2, 3, 5];

        let victim = Victim::from_iter(INDEXES);

        helper::assert_exact_iterator(victim.into_iter(), INDEXES);
    }

    #[test]
    fn backward_iter() {
        const INDEXES: [u8; 4] = [1, 2, 3, 5];

        let victim = Victim::from_iter(INDEXES);

        helper::assert_exact_iterator(victim.iter_rev(), INDEXES.into_iter().rev());
    }

    #[test]
    fn backward_into_iter() {
        const INDEXES: [u8; 4] = [1, 2, 3, 5];

        let victim = Victim::from_iter(INDEXES);

        helper::assert_exact_iterator(victim.into_iter_rev(), INDEXES.into_iter().rev());
    }
} // mod index_ord_set

mod index_chunked_set {
    use crate::{
        chunk::{ArrayChunk, UnsignedChunk},
        set::IndexChunkedSet,
    };

    use super::helper;

    type Victim = IndexChunkedSet<ArrayChunk<UnsignedChunk<u8>, 2>>;

    #[test]
    fn forward_iter() {
        const INDEXES: [u16; 4] = [1, 2, 3, 5];

        let victim = Victim::from_iter(INDEXES);

        helper::assert_exact_iterator(victim.iter(), INDEXES);
    }

    #[test]
    fn forward_into_iter() {
        const INDEXES: [u16; 4] = [1, 2, 3, 5];

        let victim = Victim::from_iter(INDEXES);

        helper::assert_exact_iterator(victim.into_iter(), INDEXES);
    }

    #[test]
    fn backward_iter() {
        const INDEXES: [u16; 4] = [1, 2, 3, 5];

        let victim = Victim::from_iter(INDEXES);

        helper::assert_exact_iterator(victim.iter_rev(), INDEXES.into_iter().rev());
    }

    #[test]
    fn backward_into_iter() {
        const INDEXES: [u16; 4] = [1, 2, 3, 5];

        let victim = Victim::from_iter(INDEXES);

        helper::assert_exact_iterator(victim.into_iter_rev(), INDEXES.into_iter().rev());
    }
} // mod index_chunked_set

mod helper {
    use core::fmt;

    #[track_caller]
    pub(super) fn assert_exact_iterator<I, E>(mut victim: I, expected: E)
    where
        I: ExactSizeIterator<Item: fmt::Debug + Eq>,
        E: IntoIterator<IntoIter: ExactSizeIterator<Item = I::Item>>,
    {
        let mut i = 0;
        let mut expected = expected.into_iter();

        loop {
            #[cfg(feature = "nightly")]
            assert_eq!(expected.is_empty(), victim.is_empty(), "{i}");

            assert_eq!(expected.len(), victim.len(), "{i}");

            let expected = expected.next();
            let victim = victim.next();

            assert_eq!(expected, victim);

            i += 1;

            if expected.is_none() {
                break;
            }
        }

        #[cfg(feature = "nightly")]
        assert_eq!(expected.is_empty(), victim.is_empty(), "{i}");

        assert_eq!(expected.len(), victim.len(), "{i}");
    }
} // mod helper
