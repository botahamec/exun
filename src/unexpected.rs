use core::fmt::{self, Debug, Display};

#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

#[cfg(feature = "std")]
use std::error::Error;

pub trait Errorable: Display + Debug + Send + Sync {}
impl<T: Display + Debug + Send + Sync + ?Sized> Errorable for T {}

#[derive(Debug)]
enum ErrorTy {
	Message(Box<dyn Errorable + 'static>),
	#[cfg(feature = "std")]
	Error(Box<dyn Error + Send + Sync + 'static>),
}

#[derive(Debug)]
pub struct UnexpectedError {
	internal: ErrorTy,
}

impl Display for UnexpectedError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self.internal {
			ErrorTy::Message(m) => Display::fmt(&m, f),
			#[cfg(feature = "std")]
			ErrorTy::Error(e) => Display::fmt(&e, f),
		}
	}
}

#[cfg(feature = "std")]
impl<T: Error + Send + Sync + 'static> From<T> for UnexpectedError {
	fn from(e: T) -> Self {
		Self::new(e)
	}
}

impl UnexpectedError {
	/// Create a new `UnexpectedError` from any [`Error`] type.
	///
	/// The error must be thread-safe and `'static` so that the
	/// `UnexpectedError` will be too.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use exun::*;
	///
	/// let err = UnexpectedError::new(core::fmt::Error);
	/// ```
	#[cfg(feature = "std")]
	pub fn new<E: Error + Send + Sync + 'static>(e: E) -> Self {
		Self {
			internal: ErrorTy::Error(Box::new(e)),
		}
	}

	/// Create an error message from a printable error message.
	///
	/// If the argument implements [`Error`], prefer [`UnexpectedError::new`]
	/// instead, which preserves the source.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use exun::*;
	///
	/// let err = UnexpectedError::msg("failed");
	/// ```
	pub fn msg<E: Display + Debug + Send + Sync + 'static>(e: E) -> Self {
		Self {
			internal: ErrorTy::Message(Box::new(e)),
		}
	}
}
