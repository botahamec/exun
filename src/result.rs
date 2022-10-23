#[cfg(feature = "std")]
use std::error::Error;

use crate::{unexpected::Errorable, RawUnexpected};

mod sealed {
	pub trait Sealed {}
	impl<T, E> Sealed for Result<T, E> {}
}

use sealed::Sealed;

#[cfg(feature = "std")]
pub trait ResultErrorExt<T>: Sealed {
	/// Converts `Result<T, E>` to `Result<T, RawUnexpected>`.
	#[allow(clippy::missing_errors_doc)]
	fn unexpect(self) -> Result<T, RawUnexpected>;
}

#[cfg(feature = "std")]
impl<T, E: Error + Send + Sync + 'static> ResultErrorExt<T> for Result<T, E> {
	fn unexpect(self) -> Result<T, RawUnexpected> {
		self.map_err(RawUnexpected::new)
	}
}

pub trait ResultMsgExt<T>: Sealed {
	/// Converts `Result<T, E>` to `Result<T, RawUnExpected>`.
	///
	/// This is provided for compatibility with `no_std`. If your type
	/// implements [`Error`], then you should prefer that instead.
	#[allow(clippy::missing_errors_doc)]
	fn unexpect_msg(self) -> Result<T, RawUnexpected>;
}

impl<T, E: Errorable + 'static> ResultMsgExt<T> for Result<T, E> {
	fn unexpect_msg(self) -> Result<T, RawUnexpected> {
		self.map_err(RawUnexpected::msg)
	}
}
