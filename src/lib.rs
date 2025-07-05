//! A collection of building blocks for index-keyed sets & maps.
//!
//! #   Organization
//!
//! The library is essentially divided in 2 parts:
//!
//! -   Traits which make a type compatible for storing indexes or values.
//! -   `IndexSet` and `IndexMap`, which provide rich set/map functionality built atop those traits.

//  Attributes.
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
//  Features (library).
#![cfg_attr(feature = "nightly", feature(exact_size_is_empty))]
#![cfg_attr(feature = "nightly", feature(extend_one))]
#![cfg_attr(feature = "nightly", feature(iter_advance_by))]
#![cfg_attr(feature = "nightly", feature(try_trait_v2))]
//  Lints.
#![deny(missing_docs)]

#[cfg(any(feature = "alloc", test))]
extern crate alloc;

pub mod index;
pub mod set;
pub mod vault;
