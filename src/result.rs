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

trait ResultExunExt<T, E, U>: Sealed {
	fn expected_err(self) -> Option<E>;

	fn unexpected_err(self) -> Option<U>;

	fn map_expected_err<F>(self, op: impl FnOnce(E) -> F) -> Result<T, Exun<F, U>>;

	fn map_unexpected_err<F>(self, op: impl FnOnce(U) -> F) -> Result<T, Exun<E, F>>;

	fn unwrap_result(self) -> Result<T, E>
	where
		U: Debug;

	fn unwrap_expected_err(self) -> E
	where
		T: Debug,
		U: Debug;

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
