use core::fmt::{self, Debug, Display};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::boxed::Box;
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::String;

#[cfg(feature = "std")]
use std::error::Error;

pub trait Errorable: Display + Debug + Send + Sync {}
impl<T: Display + Debug + Send + Sync + ?Sized> Errorable for T {}

#[derive(Debug)]
enum ErrorTy {
	None,
	#[cfg(feature = "alloc")]
	Message(Box<dyn Errorable + 'static>),
	#[cfg(feature = "std")]
	Error(Box<dyn Error + Send + Sync + 'static>),
}

/// A wrapper for an error that isn't expected to occur.
///
/// This implements [`From<T>`] where `T` implements [`Error`], [`Send`],
/// [`Sync`] and `'static` for easy conversion. Because of this, it cannot
/// itself implement [`Error`]. If you need a type that implements [`Error`]
/// but doesn't implement `From<Error>`, use [`UnexpectedError`].
#[derive(Debug)]
pub struct RawUnexpected {
	internal: ErrorTy,
}

impl Display for RawUnexpected {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.internal {
			ErrorTy::None => Display::fmt("Called `unexpect` on a `None` value", f),
			#[cfg(feature = "alloc")]
			ErrorTy::Message(m) => Display::fmt(&m, f),
			#[cfg(feature = "std")]
			ErrorTy::Error(e) => Display::fmt(&e, f),
		}
	}
}

#[cfg(feature = "std")]
impl<T: Error + Send + Sync + 'static> From<T> for RawUnexpected {
	fn from(e: T) -> Self {
		Self::new(e)
	}
}

impl RawUnexpected {
	/// Create a new `RawUnexpected` from any [`Error`] type.
	///
	/// The error must be thread-safe and `'static` so that the
	/// `RawUnexpected` will be too.
	///
	/// # Examples
	///
	/// ```
	/// use exun::*;
	///
	/// let x = RawUnexpected::new(core::fmt::Error);
	/// ```
	#[cfg(feature = "std")]
	#[must_use]
	pub fn new<E: Error + Send + Sync + 'static>(error: E) -> Self {
		Self {
			internal: ErrorTy::Error(Box::new(error)),
		}
	}

	/// Create a new `RawUnexpected` from a printable error message.
	///
	/// If the argument implements [`Error`], prefer [`RawUnexpected::new`]
	/// instead, which preserves the source.
	///
	/// # Examples
	///
	/// ```
	/// use exun::*;
	///
	/// let x = RawUnexpected::msg("failed");
	/// ```
	#[cfg(feature = "alloc")]
	#[must_use]
	pub fn msg<E: Display + Debug + Send + Sync + 'static>(error: E) -> Self {
		Self {
			internal: ErrorTy::Message(Box::new(error)),
		}
	}

	/// Create a new `RawUnexpected` that is simply empty.
	///
	/// This is used for converting an [`Option<T>`] to a
	/// [`Result<T, RawUnexpected>`].
	///
	/// # Examples
	///
	/// ```
	/// use exun::*;
	///
	/// let x = RawUnexpected::none();
	/// ```
	#[must_use]
	pub fn none() -> Self {
		Self {
			internal: ErrorTy::None,
		}
	}

	/// Get the original error.
	///
	/// This will return [`None`] if `self` was created using
	/// [`RawUnexpected::msg`].
	///
	/// # Examples
	///
	/// ```
	/// use exun::*;
	///
	/// let x = RawUnexpected::new(core::fmt::Error);
	/// assert!(x.source().is_some());
	///
	/// let x = RawUnexpected::msg("failed");
	/// assert!(x.source().is_none());
	/// ```
	#[must_use]
	#[cfg(feature = "std")]
	pub fn source(&self) -> Option<&(dyn Error + 'static)> {
		match &self.internal {
			ErrorTy::None => None,
			#[cfg(feature = "alloc")]
			ErrorTy::Message(_) => None,
			#[cfg(feature = "std")]
			ErrorTy::Error(e) => Some(&**e),
		}
	}
}

/// An error that isn't expected to occur.
///
/// This implements [`Error`]. Because of this, it cannot implement
/// `From<Error>`. If that's something you need, try [`RawUnexpected`].
#[derive(Debug)]
pub struct UnexpectedError(RawUnexpected);

impl UnexpectedError {
	/// Create a new `UnexpectedError` from any [`Error`] type.
	///
	/// The error must be thread-safe and `'static` so that the
	/// `UnexpectedError` will be too.
	///
	/// # Examples
	///
	/// ```
	/// use exun::*;
	///
	/// let x = UnexpectedError::new(core::fmt::Error);
	/// ```
	#[cfg(feature = "std")]
	#[must_use]
	pub fn new<E: Error + Send + Sync + 'static>(error: E) -> Self {
		Self(RawUnexpected::new(error))
	}

	/// Create a new `UnexpectedError` from a printable error message.
	///
	/// If the argument implements [`Error`], prefer [`UnexpectedError::new`]
	/// instead, which preserves the source.
	///
	/// # Examples
	///
	/// ```
	/// use exun::*;
	///
	/// let x = UnexpectedError::msg("failed");
	/// ```
	#[cfg(feature = "alloc")]
	#[must_use]
	pub fn msg<E: Display + Debug + Send + Sync + 'static>(error: E) -> Self {
		Self(RawUnexpected::msg(error))
	}

	/// Create a new `RawUnexpected` that is simply empty.
	///
	/// This is used for converting an [`Option<T>`] to a
	/// [`Result<T, RawUnexpected>`].
	///
	/// # Examples
	///
	/// ```
	/// use exun::*;
	///
	/// let x = UnexpectedError::none();
	/// ```
	#[must_use]
	pub fn none() -> Self {
		Self(RawUnexpected::none())
	}
}

impl From<RawUnexpected> for UnexpectedError {
	fn from(ru: RawUnexpected) -> Self {
		Self(ru)
	}
}

#[cfg(feature = "alloc")]
impl From<&'static str> for UnexpectedError {
	fn from(value: &'static str) -> Self {
		Self(RawUnexpected::msg(value))
	}
}

#[cfg(feature = "alloc")]
impl From<String> for UnexpectedError {
	fn from(value: String) -> Self {
		Self(RawUnexpected::msg(value))
	}
}

impl Display for UnexpectedError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

#[cfg(feature = "std")]
impl Error for UnexpectedError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		self.0.source()
	}
}

impl AsRef<RawUnexpected> for UnexpectedError {
	fn as_ref(&self) -> &RawUnexpected {
		&self.0
	}
}
