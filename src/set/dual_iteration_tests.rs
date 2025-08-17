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
    fn difference() {
        let empty = Victim::from_iter(EMPTY);
        let primes = Victim::from_iter(PRIMES);
        let evens = Victim::from_iter(EVENS);
        let odds = Victim::from_iter(ODDS);

        helper::assert_iterator(empty.difference(&primes), EMPTY);
        helper::assert_iterator(empty.difference(&evens), EMPTY);
        helper::assert_iterator(empty.difference(&odds), EMPTY);

        helper::assert_iterator(primes.difference(&empty), PRIMES);
        helper::assert_iterator(primes.difference(&evens), ODD_PRIMES);
        helper::assert_iterator(primes.difference(&odds), EVEN_PRIMES);

        helper::assert_iterator(primes.difference(&primes), EMPTY);
        helper::assert_iterator(evens.difference(&evens), EMPTY);
        helper::assert_iterator(odds.difference(&odds), EMPTY);
    }

    #[test]
    fn symmetric_difference() {
        let empty = Victim::from_iter(EMPTY);
        let primes = Victim::from_iter(PRIMES);
        let evens = Victim::from_iter(EVENS);
        let odds = Victim::from_iter(ODDS);

        helper::assert_iterator(empty.symmetric_difference(&primes), PRIMES);
        helper::assert_iterator(empty.symmetric_difference(&evens), EVENS);
        helper::assert_iterator(empty.symmetric_difference(&odds), ODDS);

        helper::assert_iterator(primes.symmetric_difference(&empty), PRIMES);
        helper::assert_iterator(evens.symmetric_difference(&empty), EVENS);
        helper::assert_iterator(odds.symmetric_difference(&empty), ODDS);

        helper::assert_iterator(primes.symmetric_difference(&evens), [1, 3, 5, 4, 6, 8]);
        helper::assert_iterator(evens.symmetric_difference(&odds), [2, 4, 6, 8, 1, 3, 5, 7]);
        helper::assert_iterator(odds.symmetric_difference(&evens), [1, 3, 5, 7, 2, 4, 6, 8]);

        helper::assert_iterator(primes.symmetric_difference(&primes), EMPTY);
        helper::assert_iterator(evens.symmetric_difference(&evens), EMPTY);
        helper::assert_iterator(odds.symmetric_difference(&odds), EMPTY);
    }

    #[test]
    fn intersection() {
        let empty = Victim::from_iter(EMPTY);
        let primes = Victim::from_iter(PRIMES);
        let evens = Victim::from_iter(EVENS);
        let odds = Victim::from_iter(ODDS);

        helper::assert_iterator(empty.intersection(&empty), EMPTY);
        helper::assert_iterator(empty.intersection(&primes), EMPTY);
        helper::assert_iterator(empty.intersection(&evens), EMPTY);
        helper::assert_iterator(empty.intersection(&odds), EMPTY);

        helper::assert_iterator(primes.intersection(&empty), EMPTY);
        helper::assert_iterator(evens.intersection(&empty), EMPTY);
        helper::assert_iterator(odds.intersection(&empty), EMPTY);

        helper::assert_iterator(primes.intersection(&primes), PRIMES);
        helper::assert_iterator(evens.intersection(&evens), EVENS);
        helper::assert_iterator(odds.intersection(&odds), ODDS);

        helper::assert_iterator(primes.intersection(&evens), EVEN_PRIMES);
        helper::assert_iterator(evens.intersection(&primes), EVEN_PRIMES);
        helper::assert_iterator(primes.intersection(&odds), ODD_PRIMES);
        helper::assert_iterator(odds.intersection(&primes), ODD_PRIMES);
        helper::assert_iterator(evens.intersection(&odds), EMPTY);
        helper::assert_iterator(odds.intersection(&evens), EMPTY);
    }

    #[test]
    fn union() {
        let empty = Victim::from_iter(EMPTY);
        let primes = Victim::from_iter(PRIMES);
        let evens = Victim::from_iter(EVENS);
        let odds = Victim::from_iter(ODDS);

        helper::assert_iterator(empty.union(&empty), EMPTY);
        helper::assert_iterator(empty.union(&primes), PRIMES);
        helper::assert_iterator(empty.union(&evens), EVENS);
        helper::assert_iterator(empty.union(&odds), ODDS);

        helper::assert_iterator(primes.union(&empty), PRIMES);
        helper::assert_iterator(evens.union(&empty), EVENS);
        helper::assert_iterator(odds.union(&empty), ODDS);

        helper::assert_iterator(primes.union(&primes), PRIMES);
        helper::assert_iterator(evens.union(&evens), EVENS);
        helper::assert_iterator(odds.union(&odds), ODDS);

        helper::assert_iterator(primes.union(&evens), [1, 2, 3, 5, 4, 6, 8]);
        helper::assert_iterator(evens.union(&primes), [2, 4, 6, 8, 1, 3, 5]);

        helper::assert_iterator(primes.union(&odds), [1, 2, 3, 5, 7]);
        helper::assert_iterator(odds.union(&primes), [1, 3, 5, 7, 2]);

        helper::assert_iterator(evens.union(&odds), [2, 4, 6, 8, 1, 3, 5, 7]);
        helper::assert_iterator(odds.union(&evens), [1, 3, 5, 7, 2, 4, 6, 8]);
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
    fn difference() {
        let empty = Victim::from_iter(EMPTY);
        let primes = Victim::from_iter(PRIMES);
        let evens = Victim::from_iter(EVENS);
        let odds = Victim::from_iter(ODDS);

        helper::assert_iterator(empty.difference(&primes), EMPTY);
        helper::assert_iterator(empty.difference(&evens), EMPTY);
        helper::assert_iterator(empty.difference(&odds), EMPTY);

        helper::assert_iterator(primes.difference(&empty), PRIMES);
        helper::assert_iterator(primes.difference(&evens), ODD_PRIMES);
        helper::assert_iterator(primes.difference(&odds), EVEN_PRIMES);

        helper::assert_iterator(primes.difference(&primes), EMPTY);
        helper::assert_iterator(evens.difference(&evens), EMPTY);
        helper::assert_iterator(odds.difference(&odds), EMPTY);
    }

    #[test]
    fn symmetric_difference() {
        let empty = Victim::from_iter(EMPTY);
        let primes = Victim::from_iter(PRIMES);
        let evens = Victim::from_iter(EVENS);
        let odds = Victim::from_iter(ODDS);

        helper::assert_iterator(empty.symmetric_difference(&primes), PRIMES);
        helper::assert_iterator(empty.symmetric_difference(&evens), EVENS);
        helper::assert_iterator(empty.symmetric_difference(&odds), ODDS);

        helper::assert_iterator(primes.symmetric_difference(&empty), PRIMES);
        helper::assert_iterator(evens.symmetric_difference(&empty), EVENS);
        helper::assert_iterator(odds.symmetric_difference(&empty), ODDS);

        helper::assert_iterator(primes.symmetric_difference(&evens), [1, 3, 4, 5, 6, 8]);
        helper::assert_iterator(evens.symmetric_difference(&odds), [1, 2, 3, 4, 5, 6, 7, 8]);
        helper::assert_iterator(odds.symmetric_difference(&evens), [1, 2, 3, 4, 5, 6, 7, 8]);

        helper::assert_iterator(primes.symmetric_difference(&primes), EMPTY);
        helper::assert_iterator(evens.symmetric_difference(&evens), EMPTY);
        helper::assert_iterator(odds.symmetric_difference(&odds), EMPTY);
    }

    #[test]
    fn intersection() {
        let empty = Victim::from_iter(EMPTY);
        let primes = Victim::from_iter(PRIMES);
        let evens = Victim::from_iter(EVENS);
        let odds = Victim::from_iter(ODDS);

        helper::assert_iterator(empty.intersection(&empty), EMPTY);
        helper::assert_iterator(empty.intersection(&primes), EMPTY);
        helper::assert_iterator(empty.intersection(&evens), EMPTY);
        helper::assert_iterator(empty.intersection(&odds), EMPTY);

        helper::assert_iterator(primes.intersection(&empty), EMPTY);
        helper::assert_iterator(evens.intersection(&empty), EMPTY);
        helper::assert_iterator(odds.intersection(&empty), EMPTY);

        helper::assert_iterator(primes.intersection(&primes), PRIMES);
        helper::assert_iterator(evens.intersection(&evens), EVENS);
        helper::assert_iterator(odds.intersection(&odds), ODDS);

        helper::assert_iterator(primes.intersection(&evens), EVEN_PRIMES);
        helper::assert_iterator(evens.intersection(&primes), EVEN_PRIMES);
        helper::assert_iterator(primes.intersection(&odds), ODD_PRIMES);
        helper::assert_iterator(odds.intersection(&primes), ODD_PRIMES);
        helper::assert_iterator(evens.intersection(&odds), EMPTY);
        helper::assert_iterator(odds.intersection(&evens), EMPTY);
    }

    #[test]
    fn union() {
        let empty = Victim::from_iter(EMPTY);
        let primes = Victim::from_iter(PRIMES);
        let evens = Victim::from_iter(EVENS);
        let odds = Victim::from_iter(ODDS);

        helper::assert_iterator(empty.union(&empty), EMPTY);
        helper::assert_iterator(empty.union(&primes), PRIMES);
        helper::assert_iterator(empty.union(&evens), EVENS);
        helper::assert_iterator(empty.union(&odds), ODDS);

        helper::assert_iterator(primes.union(&empty), PRIMES);
        helper::assert_iterator(evens.union(&empty), EVENS);
        helper::assert_iterator(odds.union(&empty), ODDS);

        helper::assert_iterator(primes.union(&primes), PRIMES);
        helper::assert_iterator(evens.union(&evens), EVENS);
        helper::assert_iterator(odds.union(&odds), ODDS);

        helper::assert_iterator(primes.union(&evens), [1, 2, 3, 4, 5, 6, 8]);
        helper::assert_iterator(evens.union(&primes), [1, 2, 3, 4, 5, 6, 8]);

        helper::assert_iterator(primes.union(&odds), [1, 2, 3, 5, 7]);
        helper::assert_iterator(odds.union(&primes), [1, 2, 3, 5, 7]);

        helper::assert_iterator(evens.union(&odds), [1, 2, 3, 4, 5, 6, 7, 8]);
        helper::assert_iterator(odds.union(&evens), [1, 2, 3, 4, 5, 6, 7, 8]);
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
    fn difference() {
        let empty = Victim::from_iter(EMPTY);
        let primes = Victim::from_iter(PRIMES);
        let evens = Victim::from_iter(EVENS);
        let odds = Victim::from_iter(ODDS);

        helper::assert_iterator(empty.difference(&primes), EMPTY);
        helper::assert_iterator(empty.difference(&evens), EMPTY);
        helper::assert_iterator(empty.difference(&odds), EMPTY);

        helper::assert_iterator(primes.difference(&empty), PRIMES);
        helper::assert_iterator(primes.difference(&evens), ODD_PRIMES);
        helper::assert_iterator(primes.difference(&odds), EVEN_PRIMES);

        helper::assert_iterator(primes.difference(&primes), EMPTY);
        helper::assert_iterator(evens.difference(&evens), EMPTY);
        helper::assert_iterator(odds.difference(&odds), EMPTY);
    }

    #[test]
    fn symmetric_difference() {
        let empty = Victim::from_iter(EMPTY);
        let primes = Victim::from_iter(PRIMES);
        let evens = Victim::from_iter(EVENS);
        let odds = Victim::from_iter(ODDS);

        helper::assert_iterator(empty.symmetric_difference(&primes), PRIMES);
        helper::assert_iterator(empty.symmetric_difference(&evens), EVENS);
        helper::assert_iterator(empty.symmetric_difference(&odds), ODDS);

        helper::assert_iterator(primes.symmetric_difference(&empty), PRIMES);
        helper::assert_iterator(evens.symmetric_difference(&empty), EVENS);
        helper::assert_iterator(odds.symmetric_difference(&empty), ODDS);

        helper::assert_iterator(primes.symmetric_difference(&evens), [1, 3, 4, 5, 6, 8]);
        helper::assert_iterator(evens.symmetric_difference(&odds), [1, 2, 3, 4, 5, 6, 7, 8]);
        helper::assert_iterator(odds.symmetric_difference(&evens), [1, 2, 3, 4, 5, 6, 7, 8]);

        helper::assert_iterator(primes.symmetric_difference(&primes), EMPTY);
        helper::assert_iterator(evens.symmetric_difference(&evens), EMPTY);
        helper::assert_iterator(odds.symmetric_difference(&odds), EMPTY);
    }

    #[test]
    fn intersection() {
        let empty = Victim::from_iter(EMPTY);
        let primes = Victim::from_iter(PRIMES);
        let evens = Victim::from_iter(EVENS);
        let odds = Victim::from_iter(ODDS);

        helper::assert_iterator(empty.intersection(&empty), EMPTY);
        helper::assert_iterator(empty.intersection(&primes), EMPTY);
        helper::assert_iterator(empty.intersection(&evens), EMPTY);
        helper::assert_iterator(empty.intersection(&odds), EMPTY);

        helper::assert_iterator(primes.intersection(&empty), EMPTY);
        helper::assert_iterator(evens.intersection(&empty), EMPTY);
        helper::assert_iterator(odds.intersection(&empty), EMPTY);

        helper::assert_iterator(primes.intersection(&primes), PRIMES);
        helper::assert_iterator(evens.intersection(&evens), EVENS);
        helper::assert_iterator(odds.intersection(&odds), ODDS);

        helper::assert_iterator(primes.intersection(&evens), EVEN_PRIMES);
        helper::assert_iterator(evens.intersection(&primes), EVEN_PRIMES);
        helper::assert_iterator(primes.intersection(&odds), ODD_PRIMES);
        helper::assert_iterator(odds.intersection(&primes), ODD_PRIMES);
        helper::assert_iterator(evens.intersection(&odds), EMPTY);
        helper::assert_iterator(odds.intersection(&evens), EMPTY);
    }

    #[test]
    fn union() {
        let empty = Victim::from_iter(EMPTY);
        let primes = Victim::from_iter(PRIMES);
        let evens = Victim::from_iter(EVENS);
        let odds = Victim::from_iter(ODDS);

        helper::assert_iterator(empty.union(&empty), EMPTY);
        helper::assert_iterator(empty.union(&primes), PRIMES);
        helper::assert_iterator(empty.union(&evens), EVENS);
        helper::assert_iterator(empty.union(&odds), ODDS);

        helper::assert_iterator(primes.union(&empty), PRIMES);
        helper::assert_iterator(evens.union(&empty), EVENS);
        helper::assert_iterator(odds.union(&empty), ODDS);

        helper::assert_iterator(primes.union(&primes), PRIMES);
        helper::assert_iterator(evens.union(&evens), EVENS);
        helper::assert_iterator(odds.union(&odds), ODDS);

        helper::assert_iterator(primes.union(&evens), [1, 2, 3, 4, 5, 6, 8]);
        helper::assert_iterator(evens.union(&primes), [1, 2, 3, 4, 5, 6, 8]);

        helper::assert_iterator(primes.union(&odds), [1, 2, 3, 5, 7]);
        helper::assert_iterator(odds.union(&primes), [1, 2, 3, 5, 7]);

        helper::assert_iterator(evens.union(&odds), [1, 2, 3, 4, 5, 6, 7, 8]);
        helper::assert_iterator(odds.union(&evens), [1, 2, 3, 4, 5, 6, 7, 8]);
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
