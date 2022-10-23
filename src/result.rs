#[cfg(feature = "std")]
use std::error::Error;

use crate::{unexpected::Errorable, UnexpectedError};

#[cfg(feature = "std")]
trait ResultErrorExt<T> {
	fn unexpect(self) -> Result<T, UnexpectedError>;
}

#[cfg(feature = "std")]
impl<T, E: Error + Send + Sync + 'static> ResultErrorExt<T> for Result<T, E> {
	fn unexpect(self) -> Result<T, UnexpectedError> {
		self.map_err(UnexpectedError::new)
	}
}

trait ResultMsgExt<T> {
	fn unexpect_msg(self) -> Result<T, UnexpectedError>;
}

impl<T, E: Errorable + 'static> ResultMsgExt<T> for Result<T, E> {
	fn unexpect_msg(self) -> Result<T, UnexpectedError> {
		self.map_err(UnexpectedError::msg)
	}
}
