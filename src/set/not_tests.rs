//! Unit tests for negation operations.

mod index_set {
    use crate::{chunk::UnsignedChunk, index::IndexView, set::IndexSet};

    type Victim = IndexSet<UnsignedChunk<u16>>;

    #[test]
    fn as_not() {
        const EMPTY: [u8; 0] = [];
        const SOME: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        {
            let victim: Victim = EMPTY.into_iter().collect();

            assert!(victim.as_not().contains(0));
            assert!(victim.as_not().contains(1));
        }

        {
            let victim: Victim = SOME.into_iter().collect();

            assert!(victim.as_not().contains(0));
            assert!(!victim.as_not().contains(1));
        }
    }
} // mod index_set

mod index_ord_set {
    use crate::{chunk::UnsignedChunk, index::IndexView, set::IndexOrdSet};

    type Victim = IndexOrdSet<UnsignedChunk<u16>>;

    #[test]
    fn contains() {
        const EMPTY: [u8; 0] = [];
        const SOME: [u8; 7] = [1, 2, 3, 5, 7, 11, 13];

        {
            let victim: Victim = EMPTY.into_iter().collect();

            assert!(victim.as_not().contains(0));
            assert!(victim.as_not().contains(1));
        }

        {
            let victim: Victim = SOME.into_iter().collect();

            assert!(victim.as_not().contains(0));
            assert!(!victim.as_not().contains(1));
        }
    }
} // mod index_ord_set

mod index_chunked_set {
    use crate::{
        chunk::{ArrayChunk, UnsignedChunk},
        index::IndexView,
        set::IndexChunkedSet,
    };

    type Victim = IndexChunkedSet<ArrayChunk<UnsignedChunk<u8>, 2>>;

    #[test]
    fn contains() {
        const EMPTY: [u16; 0] = [];
        const SOME: [u16; 7] = [1, 2, 3, 5, 7, 11, 13];

        {
            let victim: Victim = EMPTY.into_iter().collect();

            assert!(victim.as_not().contains(0));
            assert!(victim.as_not().contains(1));
        }

        {
            let victim: Victim = SOME.into_iter().collect();

            assert!(victim.as_not().contains(0));
            assert!(!victim.as_not().contains(1));
        }
    }
} // mod index_chunked_set
