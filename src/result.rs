use core::fmt::Debug;

#[cfg(feature = "std")]
use std::error::Error;

use crate::{unexpected::Errorable, Exun, RawUnexpected};

mod sealed {
	pub trait Sealed {}
	impl<T, E> Sealed for Result<T, E> {}
	impl<T> Sealed for Option<T> {}
}

use sealed::Sealed;

/// Provides [`Result::unexpect`]
///
/// [`Result::unexpect`]: `ResultErrorExt::unexpect`
#[cfg(feature = "std")]
pub trait ResultErrorExt<T>: Sealed {
	/// Converts [`Result<T, E>`] to [`Result<T, RawUnexpected>`].
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use exun::*;
	/// use core::fmt::Error;
	///
	/// let res: Result<i32, Error> = Err(Error);
	/// let res: Result<i32, RawUnexpected> = res.unexpect();
	/// ```
	///
	/// Use with the try operator
	///
	/// ```
	/// use exun::*;
	/// use core::fmt::Error;
	///
	/// fn foo() -> Result<i32, UnexpectedError> {
	///     let res: Result<i32, Error> = Err(Error);
	///     Ok(res.unexpect()?)
	/// }
	/// ```
	///
	/// Use with the try operator and [`Exun`]
	///
	/// ```
	/// use exun::*;
	/// use core::fmt::Error;
	///
	/// fn foo() -> Result<i32, Exun<(), UnexpectedError>> {
	///     let res: Result<i32, Error> = Err(Error);
	///     Ok(res.unexpect()?)
	/// }
	/// ```
	///
	/// [`Exun`]: `crate::Exun`
	#[allow(clippy::missing_errors_doc)]
	fn unexpect(self) -> Result<T, RawUnexpected>;
}

#[cfg(feature = "std")]
impl<T, E: Error + Send + Sync + 'static> ResultErrorExt<T> for Result<T, E> {
	fn unexpect(self) -> Result<T, RawUnexpected> {
		self.map_err(RawUnexpected::new)
	}
}

impl<T> ResultErrorExt<T> for Result<T, RawUnexpected> {
	fn unexpect(self) -> Self {
		self
	}
}

impl<T> ResultErrorExt<T> for Option<T> {
	fn unexpect(self) -> Result<T, RawUnexpected> {
		self.ok_or_else(RawUnexpected::none)
	}
}

/// Provides [`Result::unexpect_msg`]
///
/// [`Result::unexpect_msg`]: `ResultMsgExt::unexpect_msg`
pub trait ResultMsgExt<T>: Sealed {
	/// Converts [`Result<T, E>`] to [`Result<T, RawUnexpected>`].
	///
	/// This is provided for compatibility with `no_std`. If your type
	/// implements [`Error`], then you should prefer that instead.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use exun::*;
	///
	/// let res: Result<i32, &str> = Err("failure");
	/// let res: Result<i32, RawUnexpected> = res.unexpect_msg();
	/// ```
	///
	/// Use with the try operator
	///
	/// ```
	/// use exun::*;
	///
	/// fn foo() -> Result<i32, UnexpectedError> {
	///     let res: Result<i32, &str> = Err("failure");
	///     Ok(res.unexpect_msg()?)
	/// }
	/// ```
	///
	/// Use with the try operator and [`Exun`]
	///
	/// ```
	/// use exun::*;
	///
	/// fn foo() -> Result<i32, Exun<(), UnexpectedError>> {
	///     let res: Result<i32, &str> = Err("failure");
	///     Ok(res.unexpect_msg()?)
	/// }
	/// ```
	///
	/// [`Exun`]: `crate::Exun`
	#[allow(clippy::missing_errors_doc)]
	fn unexpect_msg(self) -> Result<T, RawUnexpected>;
}

impl<T, E: Errorable + 'static> ResultMsgExt<T> for Result<T, E> {
	fn unexpect_msg(self) -> Result<T, RawUnexpected> {
		self.map_err(RawUnexpected::msg)
	}
}

pub trait ResultExunExt<T, E, U>: Sealed {
	/// Converts [`Result<T, Exun<E, U>>`] to [`Option<E>`].
	///
	/// Converts self into an [`Option<E>`], consuming `self`, and discarding
	/// success value and the unexpected error, if any.
	///
	/// # Examples
	///
	/// ```
	/// use exun::{Expected, Exun, ResultExunExt};
	///
	/// let x: Result<u32, Exun<&str, &str>> = Ok(2);
	/// assert_eq!(x.expected_err(), None);
	///
	/// let x: Result<u32, Exun<&str, &str>> = Err(Expected("expected"));
	/// assert_eq!(x.expected_err(), Some("expected"));
	/// ```
	fn expected_err(self) -> Option<E>;

	/// Converts [`Result<T, Exun<E, U>>`] to [`Option<U>`].
	///
	/// Converts self into an [`Option<U>`], consuming `self`, and discarding
	/// success value and the expected error, if any.
	///
	/// # Examples
	///
	/// ```
	/// use exun::{Exun, ResultExunExt, Unexpected};
	///
	/// let x: Result<u32, Exun<&str, &str>> = Ok(2);
	/// assert_eq!(x.unexpected_err(), None);
	///
	/// let x: Result<u32, Exun<&str, &str>> = Err(Unexpected("unexpected"));
	/// assert_eq!(x.unexpected_err(), Some("unexpected"));
	/// ```
	fn unexpected_err(self) -> Option<U>;

	/// Maps a [`Result<T, Exun<E, U>>`] to `Result<T, Exun<F, U>>` by applying
	/// a function to a contained `Err(Expected)` value, leaving the `Ok` and
	/// `Err(Unexpected)` values untouched.
	///
	/// This function can be used to pass through a successful result while
	/// handling an expected error.
	///
	/// # Examples
	///
	/// ```
	/// use exun::{Exun, ResultExunExt, Expected};
	///
	/// fn stringify(x: u32) -> String { format!("error code: {x}") }
	///
	/// let x: Result<u32, Exun<u32, &str>> = Ok(2);
	/// assert_eq!(x.map_expected_err(stringify), Ok(2));
	///
	/// let x: Result<u32, Exun<u32, &str>> = Err(Expected(13));
	/// assert_eq!(x.map_expected_err(stringify), Err(Expected("error code: 13".to_string())));
	/// ```
	fn map_expected_err<F>(self, op: impl FnOnce(E) -> F) -> Result<T, Exun<F, U>>;

	/// Maps a [`Result<T, Exun<E, U>>`] to `Result<T, Exun<E, F>>` by applying
	/// a function to a contained `Err(Unexpected)` value, leaving the `Ok` and
	/// `Err(Expected)` values untouched.
	///
	/// This function can be used to pass through a successful result while
	/// handling an unexpected error.
	///
	/// # Examples
	///
	/// ```
	/// use exun::{Exun, ResultExunExt, Unexpected};
	///
	/// fn stringify(x: &str) -> String { format!("error: {x}") }
	///
	/// let x: Result<u32, Exun<u32, &str>> = Ok(2);
	/// assert_eq!(x.map_unexpected_err(stringify), Ok(2));
	///
	/// let x: Result<u32, Exun<u32, &str>> = Err(Unexpected("hi"));
	/// assert_eq!(x.map_unexpected_err(stringify), Err(Unexpected("error: hi".to_string())));
	/// ```
	fn map_unexpected_err<F>(self, op: impl FnOnce(U) -> F) -> Result<T, Exun<E, F>>;

	/// Converts [`Result<T, Exun<E, U>>`] to `Result<T, E>`, consuming the
	/// self value.
	///
	/// Because this function may panic, its use is generally discouraged.
	/// Instead, prefer to use pattern matching and handle the [`Unexpected`]
	/// case explicitly.
	///
	/// # Panics
	///
	/// Panics if the value is an [`Unexpected`], with a panic message provided
	/// by the [`Unexpected`]'s value.
	///
	/// # Examples
	///
	/// ```
	/// use exun::{Exun, ResultExunExt};
	///
	/// let x: Result<u32, Exun<&str, &str>> = Ok(2);
	/// assert_eq!(x.unwrap_result(), Ok(2));
	/// ```
	///
	/// [`Unexpected`]: crate::Unexpected
	fn unwrap_result(self) -> Result<T, E>
	where
		U: Debug;

	/// Returns the contained [`Expected`] value, consuming the `self` value.
	///
	/// Because this function may panic, its use is generally discouraged.
	/// Instead, prefer to use pattern matching and handle the [`Unexpected`]
	/// case explicitly.
	///
	/// # Panics
	///
	/// Panics if the value is an [`Unexpected`], with a panic message provided
	/// by the [`Unexpected`]'s value.
	///
	/// # Examples
	///
	/// ```
	/// use exun::{Expected, Exun, ResultExunExt};
	///
	/// let x: Result<u32, Exun<&str, &str>> = Err(Expected("failure"));
	/// assert_eq!(x.unwrap_expected_err(), "failure");
	/// ```
	///
	/// [`Expected`]: crate::Expected
	/// [`Unexpected`]: crate::Unexpected
	fn unwrap_expected_err(self) -> E
	where
		T: Debug,
		U: Debug;

	/// Returns the contained [`Unexpected`] value, consuming the `self` value.
	///
	/// Because this function may panic, its use is generally discouraged.
	/// Instead, prefer to use pattern matching and handle the [`Expected`]
	/// case explicitly.
	///
	/// # Panics
	///
	/// Panics if the value is an [`Expected`], with a panic message provided
	/// by the [`Expected`]'s value.
	///
	/// # Examples
	///
	/// ```
	/// use exun::{Exun, ResultExunExt, Unexpected};
	///
	/// let x: Result<u32, Exun<&str, &str>> = Err(Unexpected("failure"));
	/// assert_eq!(x.unwrap_unexpected_err(), "failure");
	/// ```
	///
	/// [`Expected`]: crate::Expected
	/// [`Unexpected`]: crate::Unexpected
	fn unwrap_unexpected_err(self) -> U
	where
		T: Debug,
		E: Debug;
}

impl<T, E, U> ResultExunExt<T, E, U> for Result<T, Exun<E, U>> {
	fn expected_err(self) -> Option<E> {
		self.err()?.expected()
	}

	fn unexpected_err(self) -> Option<U> {
		self.err()?.unexpected()
	}

	fn map_expected_err<F>(self, op: impl FnOnce(E) -> F) -> Result<T, Exun<F, U>> {
		self.map_err(|e| e.map(op))
	}

	fn map_unexpected_err<F>(self, op: impl FnOnce(U) -> F) -> Result<T, Exun<E, F>> {
		self.map_err(|e| e.map_unexpected(op))
	}

	fn unwrap_result(self) -> Result<T, E>
	where
		U: Debug,
	{
		match self {
			Ok(value) => Ok(value),
			Err(error) => Err(error.unwrap()),
		}
	}

	fn unwrap_expected_err(self) -> E
	where
		T: Debug,
		U: Debug,
	{
		self.unwrap_err().unwrap()
	}

	fn unwrap_unexpected_err(self) -> U
	where
		T: Debug,
		E: Debug,
	{
		self.unwrap_err().unwrap_unexpected()
	}
}
