use core::fmt::{self, Debug, Display};

#[cfg(feature = "std")]
use std::error::Error;

pub use Exun::{Expected, Unexpected};

/// `Expect` is a type that represents either the expected error type
/// ([`Expected`]) or an unexpected error ([`Unexpected`]).
///
/// See the [crate documentation](self) for details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Exun<E, U> {
	/// Contains the expected error type
	Expected(E),
	/// Contains an unexpected error
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

impl<E, U> From<E> for Exun<E, U> {
	fn from(e: E) -> Self {
		Expected(e)
	}
}

impl<E, U> Exun<E, U> {
	/// Converts from `Expect<E, U>` to [`Option<E>`].
	///
	/// Converts `self` into an [`Option<E>`], consuming `self`, and discarding
	/// the unexpected value, if any.
	///
	/// # Examples
	/// Basic usage:
	/// ```
	/// use exun::*;
	///
	/// let x: Expect<i32, &str> = Expected(2);
	/// assert_eq!(x.expected(), Some(2));
	///
	/// let x: Expect<i32, &str> = Unexpected("Nothing here");
	/// assert_eq!(x.expected(), None);
	/// ```
	#[allow(clippy::missing_const_for_fn)]
	pub fn expected(self) -> Option<E> {
		match self {
			Expected(e) => Some(e),
			Unexpected(_) => None,
		}
	}

	/// Converts from `Expect<E, U>` to [`Option<U>`].
	///
	/// Converts `self` into an [`Option<U>`], consuming `self`, and discarding
	/// the expected value, if any.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use exun::*;
	///
	/// let x: Expect<i32, &str> = Expected(2);
	/// assert_eq!(x.unexpected(), None);
	///
	/// let x: Expect<i32, &str> = Unexpected("Nothing here");
	/// assert_eq!(x.unexpected(), Some("Nothing here"));
	/// ```
	#[allow(clippy::missing_const_for_fn)]
	pub fn unexpected(self) -> Option<U> {
		match self {
			Expected(_) => None,
			Unexpected(u) => Some(u),
		}
	}

	/// Converts from `&mut Expect<E, U>` to `Expect<&mut E, &mut U>`.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use exun::*;
	///
	/// fn mutate(r: &mut Expect<i32, i32>) {
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

	/// Maps a `Expect<E, U>` to `Expect<T, U>` by applying a function to a
	/// contained [`Expected`] value, leaving an [`Unexpected`] value
	/// untouched.
	///
	/// This function can be used to compose the results of two functions.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use exun::*;
	///
	/// let x: Expect<i32, &str> = Expected(2);
	/// assert_eq!(x.map(|i| i * 10), Expected(20));
	///
	/// let x: Expect<i32, &str> = Unexpected("unexpected");
	/// assert_eq!(x.map(|i| i * 10), Unexpected("unexpected"));
	/// ```
	pub fn map<T, F: FnOnce(E) -> T>(self, op: F) -> Exun<T, U> {
		match self {
			Expected(e) => Expected(op(e)),
			Unexpected(u) => Unexpected(u),
		}
	}

	/// Maps a `Expect<E, U>` to `Expect<E, T>` by applying a function to a
	/// contained [`Unexpected`] value, leaving an [`Expected`] value
	/// untouched.
	///
	/// This function can be used to pass through an expected result while
	/// handling an error.
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```
	/// use exun::*;
	///
	/// fn stringify(x: u32) -> String { format!("error code: {x}") }
	///
	/// let x: Expect<u32, u32> = Expected(2);
	/// assert_eq!(x.map_unexpected(stringify), Expected(2));
	///
	/// let x: Expect<u32, u32> = Unexpected(13);
	/// assert_eq!(x.map_unexpected(stringify), Unexpected("error code: 13".to_string()));
	/// ```
	pub fn map_unexpected<T, F: FnOnce(U) -> T>(self, op: F) -> Exun<E, T> {
		match self {
			Expected(e) => Expected(e),
			Unexpected(u) => Unexpected(op(u)),
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
	/// Basic usage:
	///
	/// ```
	/// use exun::*;
	///
	/// let x: Expect<u32, &str> = Expected(2);
	/// assert_eq!(x.unwrap(), 2);
	/// ```
	///
	/// ```should_panic
	/// use exun::*;
	///
	/// let x: Expect<u32, &str> = Unexpected("emergency failure");
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
	/// Basic usage:
	///
	/// ```should_panic
	/// use exun::*;
	///
	/// let x: Expect<u32, &str> = Expected(2);
	/// x.unwrap_unexpected(); // panics wirh `2`
	/// ```
	///
	/// ```
	/// use exun::*;
	///
	/// let x: Expect<u32, &str> = Unexpected("emergency failure");
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
