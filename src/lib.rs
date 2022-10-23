#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
//! There are many errors we don't expect to occur. But what if we're wrong? We
//! don't want our programs to panic because of that. We also don't want to spend
//! so much time handling unexpected errors. That's what this crate is for. You
//! keep your unexpected errors, and don't worry about them until later.
//!
//! * This crate works in `no-std`, although most features (besides [`Exun`]) require
//! `alloc`.
//!
//! * [`Exun`] is an error type. It'll hold on to your [`Unexpected`] error if you have
//! one, so you can figure out what to do with it later. If the error is
//! [`Expected`], then it'll hold onto that too.
//!
//! * [`RawUnexpected`] bottles up all of your unexpected errors. There's also
//! [`UnexpectedError`], which implements [`Error`].
//!
//! * [`Expect`] is a type alias for [`Exun<E, RawUnexpected>`].
//!
//! * Clearly mark errors that you don't expect to occur by calling
//! `Result::unexpect`. If the error type doesn't implement `Error`, you can still
//! use `Result::unexpect_msg`, as long as it implements
//! `Debug + Display + Send + Sync + 'static`.
//!
//! ## Usage
//!
//! The only pre-requisite is Rust 1.41.1.
//!
//! For standard features:
//!
//! ```toml
//! [dependencies]
//! # ...
//! exun = "0.1"
//! ```
//!
//! The following features are enabled by default:
//!
//! * `std`: This automatically enables `alloc`. It's used for the standard
//! library's [`Error`] type. Using this type allows more errors to be converted
//! into [`Exun`] and [`RawUnexpected`] errors automatically, and it's needed for
//! `Result::unexpect`.
//!
//! * `alloc`: This is needed for [`Expect`], [`RawUnexpected`] and
//! [`UnexpectedError`], as well as `Result::unexpected_msg`.
//!
//! To disable these features:
//!
//! ```toml
//! [dependencies]
//! # ...
//! exun = { version = "0.1", default-features = false }
//! ```
//!
//! If you'd like to use `alloc` but not `std`:
//!
//! ```toml
//! [dependencies]
//! # ...
//! exun = { version = "0.1", default-features = false, features = ["alloc"] }
//! ```
//!
//! ## Examples
//!
//! ```
//! use exun::*;
//!
//! fn foo(num: &str) -> Result<i32, RawUnexpected> {
//!     // we use `unexpect` to indicate that we don't expect this error to occur
//!     let num = num.parse::<i32>().unexpect()?;
//!     Ok(num)
//! }
//! ```
//!
//! ```
//! use std::error::Error;
//! use std::fmt::{self, Display};
//!
//! use exun::*;
//!
//! #[derive(Debug)]
//! struct NoNumberError;
//!
//! impl Display for NoNumberError {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         write!(f, "no number provided")
//!     }
//! }
//!
//! impl Error for NoNumberError {}
//!
//! fn foo(num: Option<&str>) -> Result<i32, Expect<NoNumberError>> {
//!     let num = num.ok_or(NoNumberError)?; // we expect that this may return an error
//!     let num = num.parse::<i32>().unexpect()?; // but we think the number is otherwise parsable
//!     Ok(num)
//! }
//! ```
//!
//! ```
//! use std::error::Error;
//! use std::fmt::{self, Display};
//! use std::num::ParseIntError;
//!
//! use exun::*;
//!
//! #[derive(Debug)]
//! struct NoNumberError;
//!
//! impl Display for NoNumberError {
//!     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!         write!(f, "no number provided")
//!     }
//! }
//!
//! impl Error for NoNumberError {}
//!
//! fn foo(num: Option<&str>) -> Result<i32, Exun<&str, ParseIntError>> {
//!     // we expect it possible to not get a number, so we handle it as such
//!     let num = match num {
//!         Some(num) => num,
//!         None => return Err(Expected("no number provided")),
//!     };
//!
//!     // however, we expect that the number is otherwise parsable
//!     match num.parse() {
//!         Ok(int) => Ok(int),
//!         Err(e) => Err(Unexpected(e))
//!     }
//! }
//! ```
//!
//! [`Error`]: `std::error::Error
//!

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

mod exun;
#[cfg(feature = "alloc")]
mod result;
#[cfg(feature = "alloc")]
mod unexpected;

pub use crate::exun::Exun;
#[cfg(feature = "std")]
pub use result::ResultErrorExt;
#[cfg(feature = "alloc")]
pub use result::ResultMsgExt;
#[cfg(feature = "alloc")]
pub use unexpected::{RawUnexpected, UnexpectedError};
pub use Exun::{Expected, Unexpected};

#[cfg(feature = "alloc")]
pub type Expect<E> = Exun<E, RawUnexpected>;
