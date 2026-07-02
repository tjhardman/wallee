#![allow(
    // Clippy bug: https://github.com/rust-lang/rust-clippy/issues/7422
    clippy::nonstandard_macro_braces,
)]

use std::error::Error as StdError;
use std::io;
use thiserror::Error;
use wallee::{wallee, Error};

#[derive(Error, Debug)]
#[error("outer")]
struct MyError {
    source: io::Error,
}

#[test]
fn test_boxed_str() {
    let error = Box::<dyn StdError + Send + Sync>::from("oh no!");
    let error = wallee!(error);
    assert_eq!("oh no!", error.to_string());
    assert_eq!(
        "oh no!",
        error
            .downcast_ref::<Box<dyn StdError + Send + Sync>>()
            .unwrap()
            .to_string()
    );
}

#[test]
fn test_boxed_thiserror() {
    let error = MyError {
        source: io::Error::other("oh no!"),
    };
    let error = wallee!(error);
    assert_eq!("oh no!", error.source().unwrap().to_string());
}

#[test]
fn test_boxed_wallee() {
    let error = wallee!("oh no!").context("it failed");
    let error = wallee!(error);
    assert_eq!("oh no!", error.source().unwrap().to_string());
}

#[test]
fn test_from_boxed() {
    // A type-erased boxed error becomes downcastable to the boxed trait object
    // (not the concrete type), while Display and the source chain are preserved.
    let boxed: Box<dyn StdError + Send + Sync> = Box::new(MyError {
        source: io::Error::other("oh no!"),
    });
    let error = Error::from_boxed(boxed);
    assert_eq!("outer", error.to_string());
    assert_eq!("oh no!", error.source().unwrap().to_string());
    assert!(error
        .downcast_ref::<Box<dyn StdError + Send + Sync>>()
        .is_some());
}

#[test]
fn test_into_boxed_dyn_error() {
    // The cheap pointer-cast conversion preserves Display but not downcasting.
    let error = wallee!(MyError {
        source: io::Error::other("oh no!"),
    });
    let boxed = error.into_boxed_dyn_error();
    assert_eq!("outer", boxed.to_string());
    assert!(boxed.downcast_ref::<MyError>().is_none());
}

#[test]
fn test_reallocate_into_boxed_dyn_error() {
    // The reallocating conversion relocates E so it stays downcastable.
    let error = wallee!(MyError {
        source: io::Error::other("oh no!"),
    });
    let boxed = error.reallocate_into_boxed_dyn_error_without_backtrace();
    assert_eq!("outer", boxed.to_string());
    assert!(boxed.downcast_ref::<MyError>().is_some());
    assert!(boxed.downcast::<MyError>().is_ok());
}
