//! [![github]](https://github.com/tjhardman/wallee)&ensp;[![crates-io]](https://crates.io/crates/wallee)&ensp;[![docs-rs]](https://docs.rs/wallee)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! This library provides [`wallee::Error`][Error], a trait object based error
//! type for easy idiomatic error handling in Rust applications.
//!
//! This crate is a fork of [`anyhow`] with support for
//! caller location tracking. This is useful when debug information is not included in the build.
//! The caller location attached to [`wallee::Error`][Error] includes the file, line and
//! column where the error originated.
//!
//! <br>
//!
//! # Details
//!
//! - Use `Result<T, wallee::Error>`, or equivalently `wallee::Result<T>`, as
//!   the return type of any fallible function.
//!
//!   Within the function, use `?` to easily propagate any error that implements
//!   the `std::error::Error` trait.
//!
//!   ```
//!   # pub trait Deserialize {}
//!   #
//!   # mod serde_json {
//!   #     use super::Deserialize;
//!   #     use std::io;
//!   #
//!   #     pub fn from_str<T: Deserialize>(json: &str) -> io::Result<T> {
//!   #         unimplemented!()
//!   #     }
//!   # }
//!   #
//!   # struct ClusterMap;
//!   #
//!   # impl Deserialize for ClusterMap {}
//!   #
//!   use wallee::Result;
//!
//!   fn get_cluster_info() -> Result<ClusterMap> {
//!       let config = std::fs::read_to_string("cluster.json")?;
//!       let map: ClusterMap = serde_json::from_str(&config)?;
//!       Ok(map)
//!   }
//!   #
//!   # fn main() {}
//!   ```
//!
//! - Attach context to help the person troubleshooting the error understand
//!   where things went wrong. A low-level error like "No such file or
//!   directory" can be annoying to debug without more context about what higher
//!   level step the application was in the middle of.
//!
//!   ```
//!   # struct It;
//!   #
//!   # impl It {
//!   #     fn detach(&self) -> Result<()> {
//!   #         unimplemented!()
//!   #     }
//!   # }
//!   #
//!   use wallee::{Context, Result};
//!
//!   fn main() -> Result<()> {
//!       # return Ok(());
//!       #
//!       # const _: &str = stringify! {
//!       ...
//!       # };
//!       #
//!       # let it = It;
//!       # let path = "./path/to/instrs.json";
//!       #
//!       it.detach().context("Failed to detach the important thing")?;
//!
//!       let content = std::fs::read(path)
//!           .with_context(|| format!("Failed to read instrs from {}", path))?;
//!       #
//!       # const _: &str = stringify! {
//!       ...
//!       # };
//!       #
//!       # Ok(())
//!   }
//!   ```
//!
//!   ```console
//!   Error: Failed to read instrs from ./path/to/instrs.json
//!
//!   Caused by:
//!       No such file or directory (os error 2)
//!   ```
//!
//! - Downcasting is supported and can be by value, by shared reference, or by
//!   mutable reference as needed.
//!
//!   ```
//!   # use wallee::wallee;
//!   # use std::fmt::{self, Display};
//!   # use std::task::Poll;
//!   #
//!   # #[derive(Debug)]
//!   # enum DataStoreError {
//!   #     Censored(()),
//!   # }
//!   #
//!   # impl Display for DataStoreError {
//!   #     fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//!   #         unimplemented!()
//!   #     }
//!   # }
//!   #
//!   # impl std::error::Error for DataStoreError {}
//!   #
//!   # const REDACTED_CONTENT: () = ();
//!   #
//!   # let error = wallee!("...");
//!   # let root_cause = &error;
//!   #
//!   # let ret =
//!   // If the error was caused by redaction, then return a
//!   // tombstone instead of the content.
//!   match root_cause.downcast_ref::<DataStoreError>() {
//!       Some(DataStoreError::Censored(_)) => Ok(Poll::Ready(REDACTED_CONTENT)),
//!       None => Err(error),
//!   }
//!   # ;
//!   ```
//!
//! - A backtrace is captured and printed with the error if the underlying error
//!   type does not already provide its own. In order to see backtraces, they must
//!   be enabled through the environment variables described in [`std::backtrace`]:
//!
//!   - If you want panics and errors to both have backtraces, set
//!     `RUST_BACKTRACE=1`;
//!   - If you want only errors to have backtraces, set `RUST_LIB_BACKTRACE=1`;
//!   - If you want only panics to have backtraces, set `RUST_BACKTRACE=1` and
//!     `RUST_LIB_BACKTRACE=0`.
//!
//!   [`std::backtrace`]: https://doc.rust-lang.org/std/backtrace/index.html#environment-variables
//!
//! - Wallee works with any error type that has an impl of `std::error::Error`,
//!   including ones defined in your crate. We do not bundle a `derive(Error)`
//!   macro but you can write the impls yourself or use a standalone macro like
//!   [thiserror].
//!
//!   [thiserror]: https://github.com/dtolnay/thiserror
//!   [`anyhow`]: https://github.com/dtolnay/anyhow
//!
//!   ```
//!   use thiserror::Error;
//!
//!   #[derive(Error, Debug)]
//!   pub enum FormatError {
//!       #[error("Invalid header (expected {expected:?}, got {found:?})")]
//!       InvalidHeader {
//!           expected: String,
//!           found: String,
//!       },
//!       #[error("Missing attribute: {0}")]
//!       MissingAttribute(String),
//!   }
//!   ```
//!
//! - One-off error messages can be constructed using the `wallee!` macro, which
//!   supports string interpolation and produces a `wallee::Error`.
//!
//!   ```
//!   # use wallee::{wallee, Result};
//!   #
//!   # fn demo() -> Result<()> {
//!   #     let missing = "...";
//!   return Err(wallee!("Missing attribute: {}", missing));
//!   #     Ok(())
//!   # }
//!   ```
//!
//!   A `bail!` macro is provided as a shorthand for the same early return.
//!
//!   ```
//!   # use wallee::{bail, Result};
//!   #
//!   # fn demo() -> Result<()> {
//!   #     let missing = "...";
//!   bail!("Missing attribute: {}", missing);
//!   #     Ok(())
//!   # }
//!   ```
//!

#![doc(html_root_url = "https://docs.rs/wallee/1.0.79")]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![deny(dead_code, unused_imports, unused_mut)]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(wallee_nightly_testing)]
compile_error!("Build script probe failed to compile.");

extern crate alloc;

#[macro_use]
mod backtrace;

#[macro_use]
mod location;

mod chain;
mod context;
mod ensure;
mod error;
mod fmt;
mod kind;
mod macros;
mod ptr;
mod wrapper;

use crate::error::ErrorImpl;
use crate::ptr::OwnPtr;
use core::fmt::Display;

use std::error::Error as StdError;

#[doc(no_inline)]
pub use wallee as format_err;

/// The `Error` type, a wrapper around a dynamic error type.
///
/// `Error` works a lot like `Box<dyn std::error::Error>`, but with these
/// differences:
///
/// - `Error` requires that the error is `Send`, `Sync`, and `'static`.
/// - `Error` guarantees that a backtrace is available, even if the underlying
///   error type does not provide one.
/// - `Error` is represented as a narrow pointer &mdash; exactly one word in
///   size instead of two.
///
/// <br>
///
/// # Display representations
///
/// When you print an error object using "{}" or to_string(), only the outermost
/// underlying error or context is printed, not any of the lower level causes.
/// This is exactly as if you had called the Display impl of the error from
/// which you constructed your wallee::Error.
///
/// ```console
/// Failed to read instrs from ./path/to/instrs.json
/// ```
///
/// To print causes as well using wallee's default formatting of causes, use the
/// alternate selector "{:#}".
///
/// ```console
/// Failed to read instrs from ./path/to/instrs.json: No such file or directory (os error 2)
/// ```
///
/// The Debug format "{:?}" includes the caller location. Note
/// that this is the representation you get by default if you return an error
/// from `fn main` instead of printing it explicitly yourself.
///
/// ```console
/// src/main.rs(5:8): Failed to read instrs from ./path/to/instrs.json
/// ```
///
/// The alternate Debug format "{:#?}" includes your backtrace if one was captured.
///
/// ```console
/// Error: src/main.rs(5:8): Failed to read instrs from ./path/to/instrs.json
///
/// Caused by:
///     No such file or directory (os error 2)
/// ```
///
/// and if there is a backtrace available:
///
/// ```console
/// Error: src/main.rs(5:8): Failed to read instrs from ./path/to/instrs.json
///
/// Caused by:
///     No such file or directory (os error 2)
///
/// Stack backtrace:
///    0: <E as wallee::context::ext::StdError>::ext_context
///              at /git/wallee/src/backtrace.rs:26
///    1: core::result::Result<T,E>::map_err
///              at /git/rustc/src/libcore/result.rs:596
///    2: wallee::context::<impl wallee::Context<T,E> for core::result::Result<T,E>>::with_context
///              at /git/wallee/src/context.rs:58
///    3: testing::main
///              at src/main.rs:5
///    4: std::rt::lang_start
///              at /git/rustc/src/libstd/rt.rs:61
///    5: main
///    6: __libc_start_main
///    7: _start
/// ```
///
/// If none of the built-in representations are appropriate and you would prefer
/// to render the error and its cause chain yourself, it can be done something
/// like this:
///
/// ```
/// use wallee::{Context, Result};
///
/// fn main() {
///     if let Err(err) = try_main() {
///         eprintln!("ERROR: {}", err);
///         err.chain().skip(1).for_each(|cause| eprintln!("because: {}", cause));
///         std::process::exit(1);
///     }
/// }
///
/// fn try_main() -> Result<()> {
///     # const IGNORE: &str = stringify! {
///     ...
///     # };
///     # Ok(())
/// }
/// ```
#[cfg_attr(not(doc), repr(transparent))]
pub struct Error {
    inner: OwnPtr<ErrorImpl>,
}

/// Iterator of a chain of source errors.
///
/// This type is the iterator returned by [`Error::chain`].
///
/// # Example
///
/// ```
/// use wallee::Error;
/// use std::io;
///
/// pub fn underlying_io_error_kind(error: &Error) -> Option<io::ErrorKind> {
///     for cause in error.chain() {
///         if let Some(io_error) = cause.downcast_ref::<io::Error>() {
///             return Some(io_error.kind());
///         }
///     }
///     None
/// }
/// ```
#[derive(Clone)]
pub struct Chain<'a> {
    state: crate::chain::ChainState<'a>,
}

/// `Result<T, Error>`
///
/// This is a reasonable return type to use throughout your application but also
/// for `fn main`; if you do, failures will be printed along with any
/// [context][Context] and a backtrace if one was captured.
///
/// `wallee::Result` may be used with one *or* two type parameters.
///
/// ```rust
/// use wallee::Result;
///
/// # const IGNORE: &str = stringify! {
/// fn demo1() -> Result<T> {...}
///            // ^ equivalent to std::result::Result<T, wallee::Error>
///
/// fn demo2() -> Result<T, OtherError> {...}
///            // ^ equivalent to std::result::Result<T, OtherError>
/// # };
/// ```
///
/// # Example
///
/// ```
/// # pub trait Deserialize {}
/// #
/// # mod serde_json {
/// #     use super::Deserialize;
/// #     use std::io;
/// #
/// #     pub fn from_str<T: Deserialize>(json: &str) -> io::Result<T> {
/// #         unimplemented!()
/// #     }
/// # }
/// #
/// # #[derive(Debug)]
/// # struct ClusterMap;
/// #
/// # impl Deserialize for ClusterMap {}
/// #
/// use wallee::Result;
///
/// fn main() -> Result<()> {
///     # return Ok(());
///     let config = std::fs::read_to_string("cluster.json")?;
///     let map: ClusterMap = serde_json::from_str(&config)?;
///     println!("cluster info: {:#?}", map);
///     Ok(())
/// }
/// ```
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Provides the `context` method for `Result`.
///
/// This trait is sealed and cannot be implemented for types outside of
/// `wallee`.
///
/// <br>
///
/// # Example
///
/// ```
/// use wallee::{Context, Result};
/// use std::fs;
/// use std::path::PathBuf;
///
/// pub struct ImportantThing {
///     path: PathBuf,
/// }
///
/// impl ImportantThing {
///     # const IGNORE: &'static str = stringify! {
///     pub fn detach(&mut self) -> Result<()> {...}
///     # };
///     # fn detach(&mut self) -> Result<()> {
///     #     unimplemented!()
///     # }
/// }
///
/// pub fn do_it(mut it: ImportantThing) -> Result<Vec<u8>> {
///     it.detach().context("Failed to detach the important thing")?;
///
///     let path = &it.path;
///     let content = fs::read(path)
///         .with_context(|| format!("Failed to read instrs from {}", path.display()))?;
///
///     Ok(content)
/// }
/// ```
///
/// When printed, the outermost context would be printed first and the lower
/// level underlying causes would be enumerated below.
///
/// ```console
/// Error: src/main.rs(5:8): Failed to read instrs from ./path/to/instrs.json
///
/// Caused by:
///     No such file or directory (os error 2)
/// ```
///
/// Refer to the [Display representations] documentation for other forms in
/// which this context chain can be rendered.
///
/// [Display representations]: Error#display-representations
///
/// <br>
///
/// # Effect on downcasting
///
/// After attaching context of type `C` onto an error of type `E`, the resulting
/// `wallee::Error` may be downcast to `C` **or** to `E`.
///
/// That is, in codebases that rely on downcasting, Wallee's context supports
/// both of the following use cases:
///
///   - **Attaching context whose type is insignificant onto errors whose type
///     is used in downcasts.**
///
///     In other error libraries whose context is not designed this way, it can
///     be risky to introduce context to existing code because new context might
///     break existing working downcasts. In Wallee, any downcast that worked
///     before adding context will continue to work after you add a context, so
///     you should freely add human-readable context to errors wherever it would
///     be helpful.
///
///     ```
///     # use wallee::bail;
///     # use thiserror::Error;
///     #
///     # #[derive(Error, Debug)]
///     # #[error("???")]
///     # struct SuspiciousError;
///     #
///     # fn helper() -> Result<()> {
///     #     bail!(SuspiciousError);
///     # }
///     #
///     use wallee::{Context, Result};
///
///     fn do_it() -> Result<()> {
///         helper().context("Failed to complete the work")?;
///         # const IGNORE: &str = stringify! {
///         ...
///         # };
///         # unreachable!()
///     }
///
///     let err = do_it().unwrap_err();
///     if let Some(e) = err.downcast_ref::<SuspiciousError>() {
///         // If helper() returned SuspiciousError, this downcast will
///         // correctly succeed even with the context in between.
///         # return;
///     }
///     # panic!("expected downcast to succeed");
///     ```
///
///   - **Attaching context whose type is used in downcasts onto errors whose
///     type is insignificant.**
///
///     Some codebases prefer to use machine-readable context to categorize
///     lower level errors in a way that will be actionable to higher levels of
///     the application.
///
///     ```
///     # use wallee::bail;
///     # use thiserror::Error;
///     #
///     # #[derive(Error, Debug)]
///     # #[error("???")]
///     # struct HelperFailed;
///     #
///     # fn helper() -> Result<()> {
///     #     bail!("no such file or directory");
///     # }
///     #
///     use wallee::{Context, Result};
///
///     fn do_it() -> Result<()> {
///         helper().context(HelperFailed)?;
///         # const IGNORE: &str = stringify! {
///         ...
///         # };
///         # unreachable!()
///     }
///
///     let err = do_it().unwrap_err();
///     if let Some(e) = err.downcast_ref::<HelperFailed>() {
///         // If helper failed, this downcast will succeed because
///         // HelperFailed is the context that has been attached to
///         // that error.
///         # return;
///     }
///     # panic!("expected downcast to succeed");
///     ```
pub trait Context<T, E>: context::private::Sealed {
    /// Wrap the error value with additional context.
    #[track_caller]
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static;

    /// Wrap the error value with additional context that is evaluated lazily
    /// only once an error does occur.
    #[track_caller]
    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

/// Equivalent to Ok::<_, wallee::Error>(value).
///
/// This simplifies creation of an wallee::Result in places where type inference
/// cannot deduce the `E` type of the result &mdash; without needing to write
/// `Ok::<_, wallee::Error>(value)`.
///
/// One might think that `wallee::Result::Ok(value)` would work in such cases
/// but it does not.
///
/// ```console
/// error[E0282]: type annotations needed for `std::result::Result<i32, E>`
///   --> src/main.rs:11:13
///    |
/// 11 |     let _ = wallee::Result::Ok(1);
///    |         -   ^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `E` declared on the enum `Result`
///    |         |
///    |         consider giving this pattern the explicit type `std::result::Result<i32, E>`, where the type parameter `E` is specified
/// ```
#[allow(non_snake_case)]
pub fn Ok<T>(t: T) -> Result<T> {
    Result::Ok(t)
}

// Not public API. Referenced by macro-generated code.
#[doc(hidden)]
pub mod __private {
    use crate::Error;
    use alloc::fmt;
    use core::fmt::Arguments;

    #[doc(hidden)]
    pub use crate::ensure::{BothDebug, NotBothDebug};
    #[doc(hidden)]
    pub use alloc::format;
    #[doc(hidden)]
    pub use core::result::Result::Err;
    #[doc(hidden)]
    pub use core::{concat, format_args, stringify};

    #[doc(hidden)]
    pub mod kind {
        #[doc(hidden)]
        pub use crate::kind::{BoxedKind, DebugKind, ErrorKind, StdKind};
    }

    #[doc(hidden)]
    #[inline]
    #[cold]
    #[track_caller]
    pub fn format_err(args: Arguments) -> Error {
        let fmt_arguments_as_str = args.as_str();

        if let Some(message) = fmt_arguments_as_str {
            // wallee!("literal"), can downcast to &'static str
            Error::msg(message)
        } else {
            // wallee!("interpolate {var}"), can downcast to String
            Error::msg(fmt::format(args))
        }
    }

    #[doc(hidden)]
    #[inline]
    #[cold]
    #[track_caller]
    pub fn format_context(error: Error, args: Arguments) -> Error {
        let fmt_arguments_as_str = args.as_str();

        if let Some(message) = fmt_arguments_as_str {
            // wallee!("literal"), can downcast to &'static str
            error.context(message)
        } else {
            // wallee!("interpolate {var}"), can downcast to String
            error.context(fmt::format(args))
        }
    }

    #[doc(hidden)]
    #[inline]
    #[cold]
    #[must_use]
    pub fn must_use(error: Error) -> Error {
        error
    }
}
