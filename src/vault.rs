//! A collection of index vaults for common needs.

#[cfg(any(feature = "alloc", test))]
mod btree_set;

#[cfg(any(feature = "alloc", test))]
mod dynamic_chunk_store;

#[cfg(any(feature = "std", test))]
mod hash_set;

#[cfg(any(feature = "alloc", test))]
pub use dynamic_chunk_store::DynamicChunkStore;
