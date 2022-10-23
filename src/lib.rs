#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![doc = include_str!("../README.md")]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

mod exun;
#[cfg(feature = "alloc")]
mod result;
#[cfg(feature = "alloc")]
mod unexpected;

pub use crate::exun::{Expected, Exun, Unexpected};
#[cfg(feature = "std")]
pub use result::ResultErrorExt;
#[cfg(feature = "alloc")]
pub use result::ResultMsgExt;
#[cfg(feature = "alloc")]
pub use unexpected::{RawUnexpected, UnexpectedError};

#[cfg(feature = "alloc")]
pub type Expect<E> = Exun<E, RawUnexpected>;
