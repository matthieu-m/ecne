//! Unit tests for drain/extract_if/retain operations.

mod index_set {
    use std::collections::BTreeSet;

    use crate::set::IndexSet;

    use super::helper;

    type Victim = IndexSet<BTreeSet<u8>>;

    const EMPTY: [u8; 0] = [];
    const PRIMES: [u8; 4] = [1, 2, 3, 5];
    const EVEN_PRIMES: [u8; 1] = [2];
    const ODD_PRIMES: [u8; 3] = [1, 3, 5];

    #[test]
    fn drain() {
        let mut victim = Victim::from_iter(PRIMES);

        helper::assert_iterator(victim.drain(), PRIMES);
        helper::assert_exact_iterator(victim.iter(), EMPTY);
    }

    #[test]
    fn extract_if() {
        {
            let mut victim = Victim::from_iter(PRIMES);

            helper::assert_iterator(victim.extract_if(|_| false), EMPTY);
            helper::assert_exact_iterator(victim.iter(), PRIMES);
        }

        {
            let mut victim = Victim::from_iter(PRIMES);

            helper::assert_iterator(victim.extract_if(|_| true), PRIMES);
            helper::assert_exact_iterator(victim.iter(), EMPTY);
        }

        {
            let mut victim = Victim::from_iter(PRIMES);

            helper::assert_iterator(victim.extract_if(|i: u8| i.is_multiple_of(2)), EVEN_PRIMES);
            helper::assert_exact_iterator(victim.iter(), ODD_PRIMES);
        }

        {
            let mut victim = Victim::from_iter(PRIMES);

            helper::assert_iterator(victim.extract_if(|i: u8| !i.is_multiple_of(2)), ODD_PRIMES);
            helper::assert_exact_iterator(victim.iter(), EVEN_PRIMES);
        }
    }

    #[test]
    fn retain() {
        {
            let mut victim = Victim::from_iter(PRIMES);

            victim.retain(|_| true);

            helper::assert_exact_iterator(victim.iter(), PRIMES);
        }

        {
            let mut victim = Victim::from_iter(PRIMES);

            victim.retain(|_| false);

            helper::assert_exact_iterator(victim.iter(), EMPTY);
        }

        {
            let mut victim = Victim::from_iter(PRIMES);

            victim.retain(|i| !i.is_multiple_of(2));

            helper::assert_exact_iterator(victim.iter(), ODD_PRIMES);
        }

        {
            let mut victim = Victim::from_iter(PRIMES);

            victim.retain(|i| i.is_multiple_of(2));

            helper::assert_exact_iterator(victim.iter(), EVEN_PRIMES);
        }
    }
} // mod index_set

mod index_ord_set {
    use alloc::collections::BTreeSet;

    use crate::set::IndexOrdSet;

    use super::helper;

    type Victim = IndexOrdSet<BTreeSet<u8>>;

    const EMPTY: [u8; 0] = [];
    const PRIMES: [u8; 4] = [1, 2, 3, 5];
    const EVEN_PRIMES: [u8; 1] = [2];
    const ODD_PRIMES: [u8; 3] = [1, 3, 5];

    #[test]
    fn drain() {
        let mut victim = Victim::from_iter(PRIMES);

        helper::assert_iterator(victim.drain(), PRIMES);
        helper::assert_exact_iterator(victim.iter(), EMPTY);
    }

    #[test]
    fn extract_if() {
        {
            let mut victim = Victim::from_iter(PRIMES);

            helper::assert_iterator(victim.extract_if(|_| false), EMPTY);
            helper::assert_exact_iterator(victim.iter(), PRIMES);
        }

        {
            let mut victim = Victim::from_iter(PRIMES);

            helper::assert_iterator(victim.extract_if(|_| true), PRIMES);
            helper::assert_exact_iterator(victim.iter(), EMPTY);
        }

        {
            let mut victim = Victim::from_iter(PRIMES);

            helper::assert_iterator(victim.extract_if(|i: u8| i.is_multiple_of(2)), EVEN_PRIMES);
            helper::assert_exact_iterator(victim.iter(), ODD_PRIMES);
        }

        {
            let mut victim = Victim::from_iter(PRIMES);

            helper::assert_iterator(victim.extract_if(|i: u8| !i.is_multiple_of(2)), ODD_PRIMES);
            helper::assert_exact_iterator(victim.iter(), EVEN_PRIMES);
        }
    }

    #[test]
    fn retain() {
        {
            let mut victim = Victim::from_iter(PRIMES);

            victim.retain(|_| true);

            helper::assert_exact_iterator(victim.iter(), PRIMES);
        }

        {
            let mut victim = Victim::from_iter(PRIMES);

            victim.retain(|_| false);

            helper::assert_exact_iterator(victim.iter(), EMPTY);
        }

        {
            let mut victim = Victim::from_iter(PRIMES);

            victim.retain(|i| !i.is_multiple_of(2));

            helper::assert_exact_iterator(victim.iter(), ODD_PRIMES);
        }

        {
            let mut victim = Victim::from_iter(PRIMES);

            victim.retain(|i| i.is_multiple_of(2));

            helper::assert_exact_iterator(victim.iter(), EVEN_PRIMES);
        }
    }
} // mod index_ord_set

mod helper {
    #[track_caller]
    pub(super) fn assert_iterator<I, E>(mut victim: I, expected: E)
    where
        I: Iterator<Item = u8>,
        E: IntoIterator<Item = u8>,
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

    #[track_caller]
    pub(super) fn assert_exact_iterator<I, E>(mut victim: I, expected: E)
    where
        I: ExactSizeIterator<Item = u8>,
        E: IntoIterator<IntoIter: ExactSizeIterator<Item = u8>>,
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
