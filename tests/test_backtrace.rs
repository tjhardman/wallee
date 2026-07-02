#![cfg_attr(error_generic_member_access, feature(error_generic_member_access))]
#![allow(clippy::let_underscore_untyped)]

#[rustversion::not(nightly)]
#[ignore]
#[test]
fn test_backtrace() {}

#[rustversion::nightly]
#[test]
fn test_backtrace() {
    use wallee::wallee;

    let error = wallee!("oh no!");
    let _ = error.backtrace();
}

// On a toolchain with the generic member access API, a wallee::Error used as a
// thiserror source forwards its backtrace through the standard library's
// provider API (via Error::thiserror_provide). This exercises the
// error_generic_member_access code paths wired up in build.rs.
#[cfg(error_generic_member_access)]
#[test]
fn test_provide_backtrace_through_thiserror() {
    use std::backtrace::Backtrace;
    use std::error::{request_ref, Error as StdError};
    use wallee::wallee;

    #[derive(thiserror::Error, Debug)]
    #[error("outer")]
    struct Outer {
        #[from]
        #[backtrace]
        source: wallee::Error,
    }

    let outer: Outer = wallee!("oh no!").into();
    let dyn_error: &(dyn StdError + 'static) = &outer;
    assert!(request_ref::<Backtrace>(dyn_error).is_some());
}
