//! Unit tests for entry operations.

mod index_set {
    use std::collections::HashSet;

    use crate::set::{Entry, IndexSet};

    type Victim = IndexSet<HashSet<u8>>;

    #[test]
    fn entry() {
        const IN: u8 = 1;
        const OUT: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        {
            let entry = victim.entry(IN);

            assert!(matches!(entry, Entry::Occupied(_)));
        }

        assert_eq!(primes.as_store(), victim.as_store());

        {
            let entry = victim.entry(OUT);

            assert!(matches!(entry, Entry::Vacant(_)));
        }

        assert_eq!(primes.as_store(), victim.as_store());
    }

    #[test]
    fn entry_get() {
        const IN: u8 = 1;
        const OUT: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        {
            let entry = victim.entry(IN);

            assert_eq!(IN, entry.get());
        }

        assert_eq!(primes.as_store(), victim.as_store());

        {
            let entry = victim.entry(OUT);

            assert_eq!(OUT, entry.get());
        }

        assert_eq!(primes.as_store(), victim.as_store());
    }

    #[test]
    fn entry_insert() {
        const IN: u8 = 1;
        const OUT: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        {
            let occupied = victim.entry(IN).insert().unwrap();

            assert_eq!(IN, occupied.get());
        }

        assert_eq!(primes.as_store(), victim.as_store());

        {
            let occupied = victim.entry(OUT).insert().unwrap();

            assert_eq!(OUT, occupied.get());
        }

        assert!(primes.as_store().iter().all(|index| victim.as_store().contains(index)));
        assert!(victim.as_store().contains(&OUT));
    }

    #[test]
    fn entry_or_insert() {
        const IN: u8 = 1;
        const OUT: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        victim.entry(IN).or_insert().unwrap();

        assert_eq!(primes.as_store(), victim.as_store());

        victim.entry(OUT).or_insert().unwrap();

        assert!(primes.as_store().iter().all(|index| victim.as_store().contains(index)));
        assert!(victim.as_store().contains(&OUT));
    }

    #[test]
    fn occupied_get() {
        const IN: u8 = 3;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        assert_eq!(IN, victim.entry(IN).insert().unwrap().get());

        assert_eq!(primes.as_store(), victim.as_store());
    }

    #[test]
    fn occupied_remove() {
        const NEW: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        assert_eq!(NEW, victim.entry(NEW).insert().unwrap().remove());

        assert_eq!(primes.as_store(), victim.as_store());
    }

    #[test]
    fn vacant_get() {
        const NEW: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        {
            let Entry::Vacant(vacant) = victim.entry(NEW) else {
                unreachable!()
            };

            assert_eq!(NEW, vacant.get());
        }

        assert_eq!(primes.as_store(), victim.as_store());
    }

    #[test]
    fn vacant_into_value() {
        const NEW: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        {
            let Entry::Vacant(vacant) = victim.entry(NEW) else {
                unreachable!()
            };

            assert_eq!(NEW, vacant.into_value());
        }

        assert_eq!(primes.as_store(), victim.as_store());
    }

    #[test]
    fn vacant_insert() {
        const NEW: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        {
            let Entry::Vacant(vacant) = victim.entry(NEW) else {
                unreachable!()
            };

            vacant.insert().unwrap();
        }

        assert!(primes.as_store().iter().all(|index| victim.as_store().contains(index)));
        assert!(victim.as_store().contains(&NEW));
    }
} // mod index_set

mod index_ord_set {
    use alloc::collections::BTreeSet;

    use crate::set::{Entry, IndexOrdSet};

    type Victim = IndexOrdSet<BTreeSet<u8>>;

    #[test]
    fn entry() {
        const IN: u8 = 1;
        const OUT: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        {
            let entry = victim.entry(IN);

            assert!(matches!(entry, Entry::Occupied(_)));
        }

        assert_eq!(primes.as_store(), victim.as_store());

        {
            let entry = victim.entry(OUT);

            assert!(matches!(entry, Entry::Vacant(_)));
        }

        assert_eq!(primes.as_store(), victim.as_store());
    }

    #[test]
    fn entry_get() {
        const IN: u8 = 1;
        const OUT: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        {
            let entry = victim.entry(IN);

            assert_eq!(IN, entry.get());
        }

        assert_eq!(primes.as_store(), victim.as_store());

        {
            let entry = victim.entry(OUT);

            assert_eq!(OUT, entry.get());
        }

        assert_eq!(primes.as_store(), victim.as_store());
    }

    #[test]
    fn entry_insert() {
        const IN: u8 = 1;
        const OUT: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        {
            let occupied = victim.entry(IN).insert().unwrap();

            assert_eq!(IN, occupied.get());
        }

        assert_eq!(primes.as_store(), victim.as_store());

        {
            let occupied = victim.entry(OUT).insert().unwrap();

            assert_eq!(OUT, occupied.get());
        }

        assert!(primes.as_store().iter().all(|index| victim.as_store().contains(index)));
        assert!(victim.as_store().contains(&OUT));
    }

    #[test]
    fn entry_or_insert() {
        const IN: u8 = 1;
        const OUT: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        victim.entry(IN).or_insert().unwrap();

        assert_eq!(primes.as_store(), victim.as_store());

        victim.entry(OUT).or_insert().unwrap();

        assert!(primes.as_store().iter().all(|index| victim.as_store().contains(index)));
        assert!(victim.as_store().contains(&OUT));
    }

    #[test]
    fn occupied_get() {
        const IN: u8 = 3;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        assert_eq!(IN, victim.entry(IN).insert().unwrap().get());

        assert_eq!(primes.as_store(), victim.as_store());
    }

    #[test]
    fn occupied_remove() {
        const NEW: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        assert_eq!(NEW, victim.entry(NEW).insert().unwrap().remove());

        assert_eq!(primes.as_store(), victim.as_store());
    }

    #[test]
    fn vacant_get() {
        const NEW: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        {
            let Entry::Vacant(vacant) = victim.entry(NEW) else {
                unreachable!()
            };

            assert_eq!(NEW, vacant.get());
        }

        assert_eq!(primes.as_store(), victim.as_store());
    }

    #[test]
    fn vacant_into_value() {
        const NEW: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        {
            let Entry::Vacant(vacant) = victim.entry(NEW) else {
                unreachable!()
            };

            assert_eq!(NEW, vacant.into_value());
        }

        assert_eq!(primes.as_store(), victim.as_store());
    }

    #[test]
    fn vacant_insert() {
        const NEW: u8 = 4;

        let primes = Victim::from_iter([1, 2, 3, 5]);

        let mut victim = primes.clone();

        {
            let Entry::Vacant(vacant) = victim.entry(NEW) else {
                unreachable!()
            };

            vacant.insert().unwrap();
        }

        assert!(primes.as_store().iter().all(|index| victim.as_store().contains(index)));
        assert!(victim.as_store().contains(&NEW));
    }
} // mod index_ord_set
