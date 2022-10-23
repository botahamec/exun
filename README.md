# Exun

There are many errors we don't expect to occur. But what if we're wrong? We
don't want our programs to panic because of that. We also don't want to spend
so much time handling unexpected errors. That's what this crate is for. You
keep your unexpected errors, and don't worry about them until later.

* This crate works in `no-std`, although most features (besides [`Exun`]) require
`alloc`.

* [`Exun`] is an error type. It'll hold on to your [`Unexpected`] error if you have
one, so you can figure out what to do with it later. If the error is
[`Expected`], then it'll hold onto that too.

* [`RawUnexpected`] bottles up all of your unexpected errors. There's also
[`UnexpectedError`], which implements [`Error`].

* [`Expect`] is a type alias for [`Exun<E, RawUnexpected>`].

* Clearly mark errors that you don't expect to occur by calling
`Result::unexpect`. If the error type doesn't implement `Error`, you can still
use `Result::unexpect_msg`, as long as it implements
`Debug + Display + Send + Sync + 'static`.

## Usage

The only pre-requisite is Rust 1.54.

For standard features:

```toml
[dependencies]
# ...
exun = "0.1"
```

The following features are enabled by default:

* `std`: This automatically enables `alloc`. It's used for the standard
library's [`Error`] type. Using this type allows more errors to be converted
into [`Exun`] and [`RawUnexpected`] errors automatically, and it's needed for
`Result::unexpect`.

* `alloc`: This is needed for [`Expect`], [`RawUnexpected`] and
[`UnexpectedError`], as well as `Result::unexpected_msg`.

To disable these features:

```toml
[dependencies]
# ...
exun = { version = "0.1", default-features = false }
```

If you'd like to use `alloc` but not `std`:

```toml
[dependencies]
# ...
exun = { version = "0.1", default-features = false, features = ["alloc"] }
```

## Examples

[`Error`]: https://doc.rust-lang.org/stable/std/fmt/struct.Error.html
