#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

mod exun;
#[cfg(feature = "alloc")]
mod unexpected;

pub use exun::*;
#[cfg(feature = "alloc")]
pub use unexpected::UnexpectedError;
