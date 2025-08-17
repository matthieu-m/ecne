//! Unit tests for difference/symmetric_difference/intersection/union.

mod index_set {
    use std::collections::BTreeSet;

    use crate::set::IndexSet;

    use super::helper;

    type Victim = IndexSet<BTreeSet<u8>>;

    const EMPTY: [u8; 0] = [];
    const PRIMES: [u8; 4] = [1, 2, 3, 5];
    const EVENS: [u8; 4] = [2, 4, 6, 8];
    const ODDS: [u8; 4] = [1, 3, 5, 7];

    const EVEN_PRIMES: [u8; 1] = [2];
    const ODD_PRIMES: [u8; 3] = [1, 3, 5];

    #[test]
    fn bitand_assign() {
        #[track_caller]
        fn assert_bitand_assign<V, O, E>(victim: V, other: O, expected: E)
        where
            V: IntoIterator<Item = u8>,
            O: IntoIterator<Item = u8>,
            E: IntoIterator<Item = u8>,
        {
            let mut victim = Victim::from_iter(victim);
            let other = Victim::from_iter(other);

            victim.bitand_assign(&other);

            helper::assert_iterator(victim.iter(), expected);
        }

        assert_bitand_assign(EMPTY, EMPTY, EMPTY);
        assert_bitand_assign(EMPTY, PRIMES, EMPTY);
        assert_bitand_assign(EMPTY, EVENS, EMPTY);
        assert_bitand_assign(EMPTY, ODDS, EMPTY);

        assert_bitand_assign(PRIMES, EMPTY, EMPTY);
        assert_bitand_assign(EVENS, EMPTY, EMPTY);
        assert_bitand_assign(ODDS, EMPTY, EMPTY);

        assert_bitand_assign(PRIMES, PRIMES, PRIMES);
        assert_bitand_assign(EVENS, EVENS, EVENS);
        assert_bitand_assign(ODDS, ODDS, ODDS);

        assert_bitand_assign(PRIMES, EVENS, EVEN_PRIMES);
        assert_bitand_assign(EVENS, PRIMES, EVEN_PRIMES);
        assert_bitand_assign(PRIMES, ODDS, ODD_PRIMES);
        assert_bitand_assign(ODDS, PRIMES, ODD_PRIMES);
        assert_bitand_assign(EVENS, ODDS, EMPTY);
        assert_bitand_assign(ODDS, EVENS, EMPTY);
    }

    #[test]
    fn bitor_assign() {
        #[track_caller]
        fn assert_bitor_assign<V, O, E>(victim: V, other: O, expected: E)
        where
            V: IntoIterator<Item = u8>,
            O: IntoIterator<Item = u8>,
            E: IntoIterator<Item = u8>,
        {
            let mut victim = Victim::from_iter(victim);
            let other = Victim::from_iter(other);

            victim.bitor_assign(&other);

            helper::assert_iterator(victim.iter(), expected);
        }

        assert_bitor_assign(EMPTY, EMPTY, EMPTY);
        assert_bitor_assign(EMPTY, PRIMES, PRIMES);
        assert_bitor_assign(EMPTY, EVENS, EVENS);
        assert_bitor_assign(EMPTY, ODDS, ODDS);

        assert_bitor_assign(PRIMES, EMPTY, PRIMES);
        assert_bitor_assign(EVENS, EMPTY, EVENS);
        assert_bitor_assign(ODDS, EMPTY, ODDS);

        assert_bitor_assign(PRIMES, PRIMES, PRIMES);
        assert_bitor_assign(EVENS, EVENS, EVENS);
        assert_bitor_assign(ODDS, ODDS, ODDS);

        assert_bitor_assign(PRIMES, EVENS, [1, 2, 3, 4, 5, 6, 8]);
        assert_bitor_assign(EVENS, PRIMES, [1, 2, 3, 4, 5, 6, 8]);

        assert_bitor_assign(PRIMES, ODDS, [1, 2, 3, 5, 7]);
        assert_bitor_assign(ODDS, PRIMES, [1, 2, 3, 5, 7]);

        assert_bitor_assign(EVENS, ODDS, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_bitor_assign(ODDS, EVENS, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn sub_assign() {
        #[track_caller]
        fn assert_sub_assign<V, O, E>(victim: V, other: O, expected: E)
        where
            V: IntoIterator<Item = u8>,
            O: IntoIterator<Item = u8>,
            E: IntoIterator<Item = u8>,
        {
            let mut victim = Victim::from_iter(victim);
            let other = Victim::from_iter(other);

            victim.sub_assign(&other);

            helper::assert_iterator(victim.iter(), expected);
        }

        assert_sub_assign(EMPTY, EMPTY, EMPTY);
        assert_sub_assign(EMPTY, PRIMES, EMPTY);
        assert_sub_assign(EMPTY, EVENS, EMPTY);
        assert_sub_assign(EMPTY, ODDS, EMPTY);

        assert_sub_assign(PRIMES, EMPTY, PRIMES);
        assert_sub_assign(EVENS, EMPTY, EVENS);
        assert_sub_assign(ODDS, EMPTY, ODDS);

        assert_sub_assign(PRIMES, PRIMES, EMPTY);
        assert_sub_assign(EVENS, EVENS, EMPTY);
        assert_sub_assign(ODDS, ODDS, EMPTY);

        assert_sub_assign(PRIMES, EVENS, ODD_PRIMES);
        assert_sub_assign(EVENS, PRIMES, [4, 6, 8]);

        assert_sub_assign(PRIMES, ODDS, EVEN_PRIMES);
        assert_sub_assign(ODDS, PRIMES, [7]);

        assert_sub_assign(EVENS, ODDS, EVENS);
        assert_sub_assign(ODDS, EVENS, ODDS);
    }
} // mod index_set

mod index_ord_set {
    use alloc::collections::BTreeSet;

    use crate::set::IndexOrdSet;

    use super::helper;

    type Victim = IndexOrdSet<BTreeSet<u8>>;

    const EMPTY: [u8; 0] = [];
    const PRIMES: [u8; 4] = [1, 2, 3, 5];
    const EVENS: [u8; 4] = [2, 4, 6, 8];
    const ODDS: [u8; 4] = [1, 3, 5, 7];

    const EVEN_PRIMES: [u8; 1] = [2];
    const ODD_PRIMES: [u8; 3] = [1, 3, 5];

    #[test]
    fn bitand_assign() {
        #[track_caller]
        fn assert_bitand_assign<V, O, E>(victim: V, other: O, expected: E)
        where
            V: IntoIterator<Item = u8>,
            O: IntoIterator<Item = u8>,
            E: IntoIterator<Item = u8>,
        {
            let mut victim = Victim::from_iter(victim);
            let other = Victim::from_iter(other);

            victim.bitand_assign(&other);

            helper::assert_iterator(victim.iter(), expected);
        }

        assert_bitand_assign(EMPTY, EMPTY, EMPTY);
        assert_bitand_assign(EMPTY, PRIMES, EMPTY);
        assert_bitand_assign(EMPTY, EVENS, EMPTY);
        assert_bitand_assign(EMPTY, ODDS, EMPTY);

        assert_bitand_assign(PRIMES, EMPTY, EMPTY);
        assert_bitand_assign(EVENS, EMPTY, EMPTY);
        assert_bitand_assign(ODDS, EMPTY, EMPTY);

        assert_bitand_assign(PRIMES, PRIMES, PRIMES);
        assert_bitand_assign(EVENS, EVENS, EVENS);
        assert_bitand_assign(ODDS, ODDS, ODDS);

        assert_bitand_assign(PRIMES, EVENS, EVEN_PRIMES);
        assert_bitand_assign(EVENS, PRIMES, EVEN_PRIMES);
        assert_bitand_assign(PRIMES, ODDS, ODD_PRIMES);
        assert_bitand_assign(ODDS, PRIMES, ODD_PRIMES);
        assert_bitand_assign(EVENS, ODDS, EMPTY);
        assert_bitand_assign(ODDS, EVENS, EMPTY);
    }

    #[test]
    fn bitor_assign() {
        #[track_caller]
        fn assert_bitor_assign<V, O, E>(victim: V, other: O, expected: E)
        where
            V: IntoIterator<Item = u8>,
            O: IntoIterator<Item = u8>,
            E: IntoIterator<Item = u8>,
        {
            let mut victim = Victim::from_iter(victim);
            let other = Victim::from_iter(other);

            victim.bitor_assign(&other);

            helper::assert_iterator(victim.iter(), expected);
        }

        assert_bitor_assign(EMPTY, EMPTY, EMPTY);
        assert_bitor_assign(EMPTY, PRIMES, PRIMES);
        assert_bitor_assign(EMPTY, EVENS, EVENS);
        assert_bitor_assign(EMPTY, ODDS, ODDS);

        assert_bitor_assign(PRIMES, EMPTY, PRIMES);
        assert_bitor_assign(EVENS, EMPTY, EVENS);
        assert_bitor_assign(ODDS, EMPTY, ODDS);

        assert_bitor_assign(PRIMES, PRIMES, PRIMES);
        assert_bitor_assign(EVENS, EVENS, EVENS);
        assert_bitor_assign(ODDS, ODDS, ODDS);

        assert_bitor_assign(PRIMES, EVENS, [1, 2, 3, 4, 5, 6, 8]);
        assert_bitor_assign(EVENS, PRIMES, [1, 2, 3, 4, 5, 6, 8]);

        assert_bitor_assign(PRIMES, ODDS, [1, 2, 3, 5, 7]);
        assert_bitor_assign(ODDS, PRIMES, [1, 2, 3, 5, 7]);

        assert_bitor_assign(EVENS, ODDS, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_bitor_assign(ODDS, EVENS, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn sub_assign() {
        #[track_caller]
        fn assert_sub_assign<V, O, E>(victim: V, other: O, expected: E)
        where
            V: IntoIterator<Item = u8>,
            O: IntoIterator<Item = u8>,
            E: IntoIterator<Item = u8>,
        {
            let mut victim = Victim::from_iter(victim);
            let other = Victim::from_iter(other);

            victim.sub_assign(&other);

            helper::assert_iterator(victim.iter(), expected);
        }

        assert_sub_assign(EMPTY, EMPTY, EMPTY);
        assert_sub_assign(EMPTY, PRIMES, EMPTY);
        assert_sub_assign(EMPTY, EVENS, EMPTY);
        assert_sub_assign(EMPTY, ODDS, EMPTY);

        assert_sub_assign(PRIMES, EMPTY, PRIMES);
        assert_sub_assign(EVENS, EMPTY, EVENS);
        assert_sub_assign(ODDS, EMPTY, ODDS);

        assert_sub_assign(PRIMES, PRIMES, EMPTY);
        assert_sub_assign(EVENS, EVENS, EMPTY);
        assert_sub_assign(ODDS, ODDS, EMPTY);

        assert_sub_assign(PRIMES, EVENS, ODD_PRIMES);
        assert_sub_assign(EVENS, PRIMES, [4, 6, 8]);

        assert_sub_assign(PRIMES, ODDS, EVEN_PRIMES);
        assert_sub_assign(ODDS, PRIMES, [7]);

        assert_sub_assign(EVENS, ODDS, EVENS);
        assert_sub_assign(ODDS, EVENS, ODDS);
    }

    #[test]
    fn bitxor_assign() {
        #[track_caller]
        fn assert_bitxor_assign<V, O, E>(victim: V, other: O, expected: E)
        where
            V: IntoIterator<Item = u8>,
            O: IntoIterator<Item = u8>,
            E: IntoIterator<Item = u8>,
        {
            let mut victim = Victim::from_iter(victim);
            let other = Victim::from_iter(other);

            victim.bitxor_assign(&other);

            helper::assert_iterator(victim.iter(), expected);
        }

        assert_bitxor_assign(EMPTY, PRIMES, PRIMES);
        assert_bitxor_assign(EMPTY, EVENS, EVENS);
        assert_bitxor_assign(EMPTY, ODDS, ODDS);

        assert_bitxor_assign(PRIMES, EMPTY, PRIMES);
        assert_bitxor_assign(EVENS, EMPTY, EVENS);
        assert_bitxor_assign(ODDS, EMPTY, ODDS);

        assert_bitxor_assign(PRIMES, EVENS, [1, 3, 4, 5, 6, 8]);
        assert_bitxor_assign(EVENS, ODDS, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_bitxor_assign(ODDS, EVENS, [1, 2, 3, 4, 5, 6, 7, 8]);

        assert_bitxor_assign(PRIMES, PRIMES, EMPTY);
        assert_bitxor_assign(EVENS, EVENS, EMPTY);
        assert_bitxor_assign(ODDS, ODDS, EMPTY);
    }
} // mod index_ord_set

mod index_chunked_set {
    use crate::{
        chunk::{ArrayChunk, UnsignedChunk},
        set::IndexChunkedSet,
    };

    use super::helper;

    type Victim = IndexChunkedSet<ArrayChunk<UnsignedChunk<u8>, 2>>;

    const EMPTY: [u16; 0] = [];
    const PRIMES: [u16; 4] = [1, 2, 3, 5];
    const EVENS: [u16; 4] = [2, 4, 6, 8];
    const ODDS: [u16; 4] = [1, 3, 5, 7];

    const EVEN_PRIMES: [u16; 1] = [2];
    const ODD_PRIMES: [u16; 3] = [1, 3, 5];

    #[test]
    fn bitand_assign() {
        #[track_caller]
        fn assert_bitand_assign<V, O, E>(victim: V, other: O, expected: E)
        where
            V: IntoIterator<Item = u16>,
            O: IntoIterator<Item = u16>,
            E: IntoIterator<Item = u16>,
        {
            let mut victim = Victim::from_iter(victim);
            let other = Victim::from_iter(other);

            victim.bitand_assign(&other);

            helper::assert_iterator(victim.iter(), expected);
        }

        assert_bitand_assign(EMPTY, EMPTY, EMPTY);
        assert_bitand_assign(EMPTY, PRIMES, EMPTY);
        assert_bitand_assign(EMPTY, EVENS, EMPTY);
        assert_bitand_assign(EMPTY, ODDS, EMPTY);

        assert_bitand_assign(PRIMES, EMPTY, EMPTY);
        assert_bitand_assign(EVENS, EMPTY, EMPTY);
        assert_bitand_assign(ODDS, EMPTY, EMPTY);

        assert_bitand_assign(PRIMES, PRIMES, PRIMES);
        assert_bitand_assign(EVENS, EVENS, EVENS);
        assert_bitand_assign(ODDS, ODDS, ODDS);

        assert_bitand_assign(PRIMES, EVENS, EVEN_PRIMES);
        assert_bitand_assign(EVENS, PRIMES, EVEN_PRIMES);
        assert_bitand_assign(PRIMES, ODDS, ODD_PRIMES);
        assert_bitand_assign(ODDS, PRIMES, ODD_PRIMES);
        assert_bitand_assign(EVENS, ODDS, EMPTY);
        assert_bitand_assign(ODDS, EVENS, EMPTY);
    }

    #[test]
    fn bitor_assign() {
        #[track_caller]
        fn assert_bitor_assign<V, O, E>(victim: V, other: O, expected: E)
        where
            V: IntoIterator<Item = u16>,
            O: IntoIterator<Item = u16>,
            E: IntoIterator<Item = u16>,
        {
            let mut victim = Victim::from_iter(victim);
            let other = Victim::from_iter(other);

            victim.bitor_assign(&other);

            helper::assert_iterator(victim.iter(), expected);
        }

        assert_bitor_assign(EMPTY, EMPTY, EMPTY);
        assert_bitor_assign(EMPTY, PRIMES, PRIMES);
        assert_bitor_assign(EMPTY, EVENS, EVENS);
        assert_bitor_assign(EMPTY, ODDS, ODDS);

        assert_bitor_assign(PRIMES, EMPTY, PRIMES);
        assert_bitor_assign(EVENS, EMPTY, EVENS);
        assert_bitor_assign(ODDS, EMPTY, ODDS);

        assert_bitor_assign(PRIMES, PRIMES, PRIMES);
        assert_bitor_assign(EVENS, EVENS, EVENS);
        assert_bitor_assign(ODDS, ODDS, ODDS);

        assert_bitor_assign(PRIMES, EVENS, [1, 2, 3, 4, 5, 6, 8]);
        assert_bitor_assign(EVENS, PRIMES, [1, 2, 3, 4, 5, 6, 8]);

        assert_bitor_assign(PRIMES, ODDS, [1, 2, 3, 5, 7]);
        assert_bitor_assign(ODDS, PRIMES, [1, 2, 3, 5, 7]);

        assert_bitor_assign(EVENS, ODDS, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_bitor_assign(ODDS, EVENS, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn sub_assign() {
        #[track_caller]
        fn assert_sub_assign<V, O, E>(victim: V, other: O, expected: E)
        where
            V: IntoIterator<Item = u16>,
            O: IntoIterator<Item = u16>,
            E: IntoIterator<Item = u16>,
        {
            let mut victim = Victim::from_iter(victim);
            let other = Victim::from_iter(other);

            victim.sub_assign(&other);

            helper::assert_iterator(victim.iter(), expected);
        }

        assert_sub_assign(EMPTY, EMPTY, EMPTY);
        assert_sub_assign(EMPTY, PRIMES, EMPTY);
        assert_sub_assign(EMPTY, EVENS, EMPTY);
        assert_sub_assign(EMPTY, ODDS, EMPTY);

        assert_sub_assign(PRIMES, EMPTY, PRIMES);
        assert_sub_assign(EVENS, EMPTY, EVENS);
        assert_sub_assign(ODDS, EMPTY, ODDS);

        assert_sub_assign(PRIMES, PRIMES, EMPTY);
        assert_sub_assign(EVENS, EVENS, EMPTY);
        assert_sub_assign(ODDS, ODDS, EMPTY);

        assert_sub_assign(PRIMES, EVENS, ODD_PRIMES);
        assert_sub_assign(EVENS, PRIMES, [4, 6, 8]);

        assert_sub_assign(PRIMES, ODDS, EVEN_PRIMES);
        assert_sub_assign(ODDS, PRIMES, [7]);

        assert_sub_assign(EVENS, ODDS, EVENS);
        assert_sub_assign(ODDS, EVENS, ODDS);
    }

    #[test]
    fn bitxor_assign() {
        #[track_caller]
        fn assert_bitxor_assign<V, O, E>(victim: V, other: O, expected: E)
        where
            V: IntoIterator<Item = u16>,
            O: IntoIterator<Item = u16>,
            E: IntoIterator<Item = u16>,
        {
            let mut victim = Victim::from_iter(victim);
            let other = Victim::from_iter(other);

            victim.bitxor_assign(&other);

            helper::assert_iterator(victim.iter(), expected);
        }

        assert_bitxor_assign(EMPTY, PRIMES, PRIMES);
        assert_bitxor_assign(EMPTY, EVENS, EVENS);
        assert_bitxor_assign(EMPTY, ODDS, ODDS);

        assert_bitxor_assign(PRIMES, EMPTY, PRIMES);
        assert_bitxor_assign(EVENS, EMPTY, EVENS);
        assert_bitxor_assign(ODDS, EMPTY, ODDS);

        assert_bitxor_assign(PRIMES, EVENS, [1, 3, 4, 5, 6, 8]);
        assert_bitxor_assign(EVENS, ODDS, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_bitxor_assign(ODDS, EVENS, [1, 2, 3, 4, 5, 6, 7, 8]);

        assert_bitxor_assign(PRIMES, PRIMES, EMPTY);
        assert_bitxor_assign(EVENS, EVENS, EMPTY);
        assert_bitxor_assign(ODDS, ODDS, EMPTY);
    }
} // mod index_chunked_set

mod helper {
    use core::fmt;

    #[track_caller]
    pub(super) fn assert_iterator<I, E>(mut victim: I, expected: E)
    where
        I: Iterator<Item: fmt::Debug + Eq>,
        E: IntoIterator<Item = I::Item>,
    {
        let mut expected = expected.into_iter();

        loop {
            let expected = expected.next();
            let victim = victim.next();

            assert_eq!(expected, victim);

            if expected.is_none() {
                break;
            }
        }
    }
} // mod helper
