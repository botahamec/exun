use core::fmt::{self, Debug, Display};

#[cfg(feature = "std")]
use std::error::Error;

#[cfg(feature = "alloc")]
use crate::{RawUnexpected, UnexpectedError};

pub use Exun::{Expected, Unexpected};

/// `Exun` is a type that represents either the expected error type
/// ([`Expected`]) or an unexpected type ([`Unexpected`]).
///
/// See the [crate documentation](crate) for details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Exun<E, U> {
	/// Contains the expected type
	Expected(E),
	/// Contains an unexpected type
	Unexpected(U),
}

impl<E: Display, U: Display> Display for Exun<E, U> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Expected(e) => e.fmt(f),
			Unexpected(u) => u.fmt(f),
		}
	}
}

#[cfg(feature = "std")]
impl<E: Error + 'static, U: Error + 'static> Error for Exun<E, U> {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Expected(ref e) => Some(e),
			Unexpected(ref u) => Some(u),
		}
	}
}

#[cfg(feature = "std")]
impl<E: Error + 'static> Error for Exun<E, RawUnexpected> {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Expected(ref e) => Some(e),
			Unexpected(ref u) => u.source(),
		}
	}
}

#[cfg(feature = "std")]
impl<E: Error, U> From<E> for Exun<E, U> {
	fn from(e: E) -> Self {
		Expected(e)
	}
}

#[cfg(feature = "alloc")]
impl<E> From<RawUnexpected> for Exun<E, RawUnexpected> {
	fn from(ue: RawUnexpected) -> Self {
		Unexpected(ue)
	}
}

#[cfg(feature = "alloc")]
impl<E> From<RawUnexpected> for Exun<E, UnexpectedError> {
	fn from(ue: RawUnexpected) -> Self {
		Unexpected(ue.into())
	}
}

impl<E, U> Exun<E, U> {
	/// Converts from `Exun<E, U>` to [`Option<E>`].
	///
	/// Converts `self` into an [`Option<E>`], consuming `self`, and discarding
	/// the unexpected value, if any.
	///
	/// # Examples
	///
	/// ```
	/// use exun::*;
	///
	/// let x: Exun<i32, &str> = Expected(2);
	/// assert_eq!(x.expected(), Some(2));
	///
	/// let x: Exun<i32, &str> = Unexpected("Nothing here");
	/// assert_eq!(x.expected(), None);
	/// ```
	#[allow(clippy::missing_const_for_fn)]
	pub fn expected(self) -> Option<E> {
		match self {
			Expected(e) => Some(e),
			Unexpected(_) => None,
		}
	}

	/// Converts from `Exun<E, U>` to [`Option<U>`].
	///
	/// Converts `self` into an [`Option<U>`], consuming `self`, and discarding
	/// the expected value, if any.
	///
	/// # Examples
	///
	/// ```
	/// use exun::*;
	///
	/// let x: Exun<i32, &str> = Expected(2);
	/// assert_eq!(x.unexpected(), None);
	///
	/// let x: Exun<i32, &str> = Unexpected("Nothing here");
	/// assert_eq!(x.unexpected(), Some("Nothing here"));
	/// ```
	#[allow(clippy::missing_const_for_fn)]
	pub fn unexpected(self) -> Option<U> {
		match self {
			Expected(_) => None,
			Unexpected(u) => Some(u),
		}
	}

	/// Converts from `&mut Exun<E, U>` to `Exun<&mut E, &mut U>`.
	///
	/// # Examples
	///
	/// ```
	/// use exun::*;
	///
	/// fn mutate(r: &mut Exun<i32, i32>) {
	///     match r.as_mut() {
	///         Expected(e) => *e = 42,
	///         Unexpected(u) => *u = 0,
	///     }
	/// }
	///
	/// let mut x = Expected(2);
	/// mutate(&mut x);
	/// assert_eq!(x.unwrap(), 42);
	///
	/// let mut x = Unexpected(13);
	/// mutate(&mut x);
	/// assert_eq!(x.unwrap_unexpected(), 0);
	/// ```
	pub fn as_mut(&mut self) -> Exun<&mut E, &mut U> {
		match self {
			Expected(ref mut e) => Expected(e),
			Unexpected(ref mut u) => Unexpected(u),
		}
	}

	/// Maps a `Exun<E, U>` to `Exun<T, U>` by applying a function to a
	/// contained [`Expected`] value, leaving an [`Unexpected`] value
	/// untouched.
	///
	/// This function can be used to compose the results of two functions.
	///
	/// # Examples
	///
	/// ```
	/// use exun::*;
	///
	/// let x: Exun<i32, &str> = Expected(2);
	/// assert_eq!(x.map(|i| i * 10), Expected(20));
	///
	/// let x: Exun<i32, &str> = Unexpected("unexpected");
	/// assert_eq!(x.map(|i| i * 10), Unexpected("unexpected"));
	/// ```
	pub fn map<T, F: FnOnce(E) -> T>(self, op: F) -> Exun<T, U> {
		match self {
			Expected(e) => Expected(op(e)),
			Unexpected(u) => Unexpected(u),
		}
	}

	/// Maps a `Exun<E, U>` to `Exun<E, T>` by applying a function to a
	/// contained [`Unexpected`] value, leaving an [`Expected`] value
	/// untouched.
	///
	/// This function can be used to pass through an expected result while
	/// handling an error.
	///
	/// # Examples
	///
	/// ```
	/// use exun::*;
	///
	/// fn stringify(x: u32) -> String { format!("error code: {x}") }
	///
	/// let x: Exun<u32, u32> = Expected(2);
	/// assert_eq!(x.map_unexpected(stringify), Expected(2));
	///
	/// let x: Exun<u32, u32> = Unexpected(13);
	/// assert_eq!(x.map_unexpected(stringify), Unexpected("error code: 13".to_string()));
	/// ```
	pub fn map_unexpected<T, F: FnOnce(U) -> T>(self, op: F) -> Exun<E, T> {
		match self {
			Expected(e) => Expected(e),
			Unexpected(u) => Unexpected(op(u)),
		}
	}

	/// Returns the [`Expected`] value, consuming the `self` value.
	///
	/// Because this function may panic, its use is generally discouraged.
	/// Instead, prefer to use pattern matching and handle the [`Unexpected`]
	/// case explicitly.
	///
	/// # Panics
	///
	/// Panics if the value is an [`Unexpected`] value, with a panic message
	/// including the passed message, and the content of the [`Unexpected`]
	/// value.
	///
	/// # Examples
	///
	/// ```should_panic
	/// use exun::*;
	///
	/// let x: Exun<u32, &str> = Exun::Unexpected("error");
	/// x.expect("Testing expect"); // panics with "testing expect: error"
	/// ```
	///
	/// # Recommended Message Style
	///
	/// We recommend that `expect` messages are used to describe the reason you
	/// *expect* the `Exun` should be `Expected`.
	///
	/// ```should_panic
	/// let path = std::env::var("IMPORTANT_PATH")
	///     .expect("env variable `IMPORTANT_PATH` should be set by test.sh");
	/// ```
	///
	/// **Hint:** If you're having trouble remembering how to phrase expect
	/// error messages, remember to focus on the word "should" as in "env
	/// variable set by blah" or "the given binary should be available and
	/// executable by the current user".
	///
	/// For more detail on expect message styles and the reasoning behind the
	/// recommendation please refer to the section on
	/// ["Common Message Styles"](https://doc.rust-lang.org/stable/std/error/index.html#common-message-styles)
	/// in the [`std::error`](https://doc.rust-lang.org/stable/std/error/index.html)
	/// module docs.
	pub fn expect(self, msg: &str) -> E
	where
		U: Debug,
	{
		match self {
			Self::Expected(e) => e,
			Self::Unexpected(e) => panic!("{}: {:?}", msg, e),
		}
	}

	/// Returns the contained [`Expected`] value, consuming the `self` value.
	///
	/// Because this function may panic, its use is generally discouraged.
	/// Instead, prefer to use pattern matching and handle the [`Unexpected`]
	/// case explicitly.
	///
	/// # Panics
	///
	/// Panics if the value is [`Unexpected`], with an panic message provided
	/// by the [`Unexpected`]'s value.
	///
	/// # Examples
	///
	/// ```
	/// use exun::*;
	///
	/// let x: Exun<u32, &str> = Expected(2);
	/// assert_eq!(x.unwrap(), 2);
	/// ```
	///
	/// ```should_panic
	/// use exun::*;
	///
	/// let x: Exun<u32, &str> = Unexpected("emergency failure");
	/// x.unwrap(); // panics with `emergency failure`
	/// ```
	pub fn unwrap(self) -> E
	where
		U: Debug,
	{
		match self {
			Expected(e) => e,
			Unexpected(u) => panic!("called `Expect::unwrap` on an `Unexpected` value: {:?}", u),
		}
	}

	/// Returns the contained [`Unexpected`] value, consuming the `self` value.
	///
	/// # Panics
	///
	/// Panics if the value is [`Expected`], with an panic message provided by
	/// the [`Expected`]'s value.
	///
	/// # Examples
	///
	/// ```should_panic
	/// use exun::*;
	///
	/// let x: Exun<u32, &str> = Expected(2);
	/// x.unwrap_unexpected(); // panics with `2`
	/// ```
	///
	/// ```
	/// use exun::*;
	///
	/// let x: Exun<u32, &str> = Unexpected("emergency failure");
	/// assert_eq!(x.unwrap_unexpected(), "emergency failure");
	/// ```
	pub fn unwrap_unexpected(self) -> U
	where
		E: Debug,
	{
		match self {
			Expected(e) => panic!(
				"called `Expect::unwrap_unexpected` on an `Expected` value: {:?}",
				e
			),
			Unexpected(u) => u,
		}
	}
}
