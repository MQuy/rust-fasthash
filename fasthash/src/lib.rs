//! A suite of non-cryptographic hash functions for Rust.
//!
//! # Example
//!
//! ```
//! use std::hash::{Hash, Hasher};
//!
//! use fasthash::{metro, MetroHasher};
//!
//! fn hash<T: Hash>(t: &T) -> u64 {
//!     let mut s = MetroHasher::new();
//!     t.hash(&mut s);
//!     s.finish()
//! }
//!
//! let h = metro::hash64(b"hello world\xff");
//!
//! assert_eq!(h, hash(&"hello world"));
//! ```
//!

extern crate extprim;
extern crate fasthash_sys as ffi;

#[macro_use]
mod hasher;
pub mod city;
pub mod farm;
pub mod metro;
pub mod mum;
pub mod murmur;
pub mod murmur2;
pub mod murmur3;
pub mod spooky;
pub mod t1ha;
pub mod xx;

pub use hasher::{Fingerprint, FastHash, HasherExt};

pub use city::{CityHasher64 as CityHasher, CityHasher128 as CityHasherExt};
pub use farm::{FarmHasher64 as FarmHasher, FarmHasher128 as FarmHasherExt};
pub use metro::{MetroHasher64_1 as MetroHasher, MetroHasher128_1 as MetroHasherExt};
pub use mum::MumHasher;
pub use murmur::MurmurHasher;
pub use murmur2::Murmur2Hasher_x64_64 as Murmur2Hasher;
pub use murmur3::{Murmur3Hasher_x86_32 as Murmur3Hasher, Murmur3Hasher_x64_128 as Murmur3HasherExt};
pub use spooky::{SpookyHasher64 as SpookyHasher, SpookyHasher128 as SpookyHasherExt};
pub use t1ha::T1ha64BeHasher as T1haHasher;
pub use xx::XXHasher64 as XXHasher;
