#[cfg(feature = "std")]
use std::error::Error;

use crate::{unexpected::Errorable, RawUnexpected};

mod sealed {
	pub trait Sealed {}
	impl<T, E> Sealed for Result<T, E> {}
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
