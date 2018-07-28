//! A suite of non-cryptographic hash functions for Rust.
//!
//! # Example
//!
//! ```rust
//! use std::hash::{Hash, Hasher};
//!
//! use fasthash::{metro, MetroHasher};
//!
//! fn hash<T: Hash>(t: &T) -> u64 {
//!     let mut s: MetroHasher = Default::default();
//!     t.hash(&mut s);
//!     s.finish()
//! }
//!
//! let h = metro::hash64(b"hello world\xff");
//!
//! assert_eq!(h, hash(&"hello world"));
//! ```
//!
//! By default, `HashMap` uses a hashing algorithm selected to
//! provide resistance against `HashDoS` attacks.
//! The hashing algorithm can be replaced on a per-`HashMap` basis
//! using the `HashMap::with_hasher` or
//! `HashMap::with_capacity_and_hasher` methods.
//!
//! It also cowork with `HashMap` or `HashSet`, act as a hash function
//!
//! ```rust
//! use std::collections::HashSet;
//!
//! use fasthash::spooky::SpookyHash128;
//!
//! let mut set = HashSet::with_hasher(SpookyHash128 {});
//! set.insert(2);
//! ```
//!
//! Or use `RandomState<CityHash64>` with a random seed.
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! use fasthash::RandomState;
//! use fasthash::city::CityHash64;
//!
//! let s = RandomState::<CityHash64>::new();
//! let mut map = HashMap::with_hasher(s);
//!
//! assert_eq!(map.insert(37, "a"), None);
//! assert_eq!(map.is_empty(), false);
//!
//! map.insert(37, "b");
//! assert_eq!(map.insert(37, "c"), Some("b"));
//! assert_eq!(map[&37], "c");
//! ```
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![warn(missing_docs)]

extern crate extprim;
#[macro_use]
extern crate extprim_literals;
extern crate fasthash_sys as ffi;
extern crate rand;
extern crate seahash;
extern crate xoroshiro128;

#[macro_use]
mod hasher;
pub mod city;
pub mod farm;
pub mod lookup3;
pub mod metro;
pub mod mum;
pub mod murmur;
pub mod murmur2;
pub mod murmur3;
pub mod sea;
pub mod spooky;
pub mod t1ha;
pub mod xx;

pub use hasher::{
    BufHasher, FastHash, FastHasher, Fingerprint, HasherExt, RandomState, Seed, StreamHasher,
};

#[cfg(not(feature = "sse42"))]
pub use city::{CityHasher128 as CityHasherExt, CityHasher64 as CityHasher};
#[cfg(feature = "sse42")]
pub use city::{CityHasher64 as CityHasher, CityHasherCrc128 as CityHasherExt};

pub use farm::{FarmHasher128 as FarmHasherExt, FarmHasher64 as FarmHasher};
pub use lookup3::Lookup3Hasher;

#[cfg(feature = "sse42")]
pub use metro::{MetroHasher128Crc_1 as MetroHasherExt, MetroHasher64Crc_1 as MetroHasher};
#[cfg(not(feature = "sse42"))]
pub use metro::{MetroHasher128_1 as MetroHasherExt, MetroHasher64_1 as MetroHasher};

pub use mum::MumHasher;
pub use murmur::MurmurHasher;
pub use murmur2::Murmur2Hasher_x64_64 as Murmur2Hasher;
pub use murmur3::{
    Murmur3Hasher_x64_128 as Murmur3Hasher, Murmur3Hasher_x64_128 as Murmur3HasherExt,
};
#[doc(no_inline)]
pub use sea::SeaHasher64 as SeaHasher;
pub use spooky::{SpookyHasher128 as SpookyHasher, SpookyHasher128 as SpookyHasherExt};

pub use t1ha::{
    T1ha0Hasher32Be as T1ha32BeHasher, T1ha0Hasher32Le as T1ha32LeHasher,
    T1ha1Hasher64Be as T1ha64BeHasher, T1ha1Hasher64Le as T1ha64LeHasher,
    T1ha2Hasher128 as T1haHasherExt, T1ha2Hasher64 as T1haHasher,
};

pub use xx::XXHasher64 as XXHasher;
