//! A collection of index vaults for common needs.

#[cfg(any(feature = "alloc", test))]
mod btree_set;

#[cfg(any(feature = "std", test))]
mod hash_set;
