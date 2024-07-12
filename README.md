# Exun

There are many errors we don't expect to occur. But what if we're wrong? We
don't want our programs to panic because of that. We also don't want to spend
so much time handling unexpected errors. That's what this crate is for. You
keep your unexpected errors, and don't worry about them until later.

* This crate works in `no-std`. Some extra features come if `alloc` or `std` is
used.

* `Exun` is an error type. It'll hold on to your `Unexpected` error if you have
one, so you can figure out what to do with it later. If the error is
`Expected`, then it'll hold onto that too.

* `RawUnexpected` bottles up all of your unexpected errors. There's also
`UnexpectedError`, which implements `Error`.

* `Expect` is a type alias for `Exun<E, RawUnexpected>`.

* Clearly mark errors that you don't expect to occur by calling
`Result::unexpect`. If the error type doesn't implement `Error`, you can still
use `Result::unexpect_msg`, as long as it implements
`Debug + Display + Send + Sync + 'static`.

## Usage

The only pre-requisite is Rust 1.41.1.

For standard features:

```toml
[dependencies]
# ...
exun = "0.2"
```

The following features are enabled by default:

* `std`: This automatically enables `alloc`. It's used for the standard
library's `Error` type. Using this type allows more errors to be converted
into `Exun` and `RawUnexpected` errors automatically, and it's needed for
`Result::unexpect`.

* `alloc`: This is needed for `RawUnexpected` and `UnexpectedError` to hold
string messages. This can be done with `Result::unexpect_msg`. Without this,
only the equivalent of `Result::unexpect_none` can be constructed.

To disable these features:

```toml
[dependencies]
# ...
exun = { version = "0.2", default-features = false }
```

If you'd like to use `alloc` but not `std`:

```toml
[dependencies]
# ...
exun = { version = "0.2", default-features = false, features = ["alloc"] }
```

## Examples

```rust
use exun::*;

fn foo(num: &str) -> Result<i32, RawUnexpected> {
    // we use `unexpect` to indicate that we don't expect this error to occur
    let num = num.parse::<i32>().unexpect()?;
    Ok(num)
}
```

```rust
use exun::*;

fn first(list: &[i32]) -> Result<i32, RawUnexpected> {
    // for options, the `unexpect_none` method can be used
    let num = list.get(0).unexpect_none()?;
    Ok(num)
}
```

```rust
use std::error::Error;
use std::fmt::{self, Display};

use exun::*;

#[derive(Debug)]
struct NoNumberError;

impl Display for NoNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no number provided")
    }
}

impl Error for NoNumberError {}

fn foo(num: Option<&str>) -> Result<i32, Expect<NoNumberError>> {
    let num = num.ok_or(NoNumberError)?; // we expect that this may return an error
    let num = num.parse::<i32>().unexpect()?; // but we think the number is otherwise parsable
    Ok(num)
}
```

```rust
use std::error::Error;
use std::fmt::{self, Display};
use std::num::ParseIntError;

use exun::*;

#[derive(Debug)]
struct NoNumberError;

impl Display for NoNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no number provided")
    }
}

impl Error for NoNumberError {}

fn foo(num: Option<&str>) -> Result<i32, Exun<&str, ParseIntError>> {
    // we expect it possible to not get a number, so we handle it as such
    let num = match num {
        Some(num) => num,
        None => return Err(Expected("no number provided")),
    };

    // however, we expect that the number is otherwise parsable
    match num.parse() {
        Ok(int) => Ok(int),
        Err(e) => Err(Unexpected(e))
    }
}
```
