#[cfg(feature = "std")]
use std::error::Error;

use crate::{unexpected::Errorable, UnexpectedError};

#[cfg(feature = "std")]
pub trait ResultErrorExt<T> {
	/// Converts `Result<T, E>` to `Result<T, UnexpectedError>`.
	#[allow(clippy::missing_errors_doc)]
	fn unexpect(self) -> Result<T, UnexpectedError>;
}

#[cfg(feature = "std")]
impl<T, E: Error + Send + Sync + 'static> ResultErrorExt<T> for Result<T, E> {
	fn unexpect(self) -> Result<T, UnexpectedError> {
		self.map_err(UnexpectedError::new)
	}
}

pub trait ResultMsgExt<T> {
	/// Converts `Result<T, E>` to `Result<T, UnexpectedError>`.
	///
	/// This is provided for compatibility with `no_std`. If your type
	/// implements [`Error`], then you should prefer that instead.
	#[allow(clippy::missing_errors_doc)]
	fn unexpect_msg(self) -> Result<T, UnexpectedError>;
}

impl<T, E: Errorable + 'static> ResultMsgExt<T> for Result<T, E> {
	fn unexpect_msg(self) -> Result<T, UnexpectedError> {
		self.map_err(UnexpectedError::msg)
	}
}
