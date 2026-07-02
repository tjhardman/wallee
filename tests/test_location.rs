use std::io;

use wallee::{bail, wallee, Context, Error, Result};

#[test]
fn test_new() {
    let error = io::Error::new(io::ErrorKind::PermissionDenied, "oh no!");
    let (err, file, line) = (Error::new(error), file!(), line!());
    assert_eq!(err.file(), file);
    assert_eq!(err.line(), line);
}

#[test]
fn test_msg() {
    let (err, file, line) = (Error::msg("oh no!"), file!(), line!());
    assert_eq!(err.file(), file);
    assert_eq!(err.line(), line);
}

#[test]
fn test_from_boxed() {
    let boxed: Box<dyn std::error::Error + Send + Sync> =
        Box::new(io::Error::new(io::ErrorKind::PermissionDenied, "oh no!"));
    let (err, file, line) = (Error::from_boxed(boxed), file!(), line!());
    assert_eq!(err.file(), file);
    assert_eq!(err.line(), line);
}

#[test]
fn test_wallee_macro() {
    let (err, file, line) = (wallee!("oh no!"), file!(), line!());
    assert_eq!(err.file(), file);
    assert_eq!(err.line(), line);

    let (err, file, line) = (wallee!("oh no!: {}", 33), file!(), line!());
    assert_eq!(err.file(), file);
    assert_eq!(err.line(), line);

    let (err, file) = (
        wallee!(io::Error::new(io::ErrorKind::PermissionDenied, "oh no!")),
        file!(),
    );
    assert_eq!(err.file(), file);
}

#[test]
fn test_bail_macro() {
    let err = || -> Result<()> { bail!("oh no!") }().unwrap_err();
    assert_eq!(err.file(), file!());

    let err = || -> Result<()> { bail!("oh no!: {}", 33) }().unwrap_err();
    assert_eq!(err.file(), file!());

    let err =
        || -> Result<()> { bail!(io::Error::new(io::ErrorKind::PermissionDenied, "oh no!")) }()
            .unwrap_err();
    assert_eq!(err.file(), file!());
}

#[test]
fn test_context() {
    let (err, file, line) = (wallee!("oh no!").context("it failed"), file!(), line!());
    assert_eq!(err.file(), file);
    assert_eq!(err.line(), line);
}

#[test]
fn test_result_context() {
    let result: io::Result<()> = Err(io::Error::new(io::ErrorKind::PermissionDenied, "oh no!"));
    let (err, file, line) = (result.context("it failed").unwrap_err(), file!(), line!());
    assert_eq!(err.file(), file);
    assert_eq!(err.line(), line);

    let result: io::Result<()> = Err(io::Error::new(io::ErrorKind::PermissionDenied, "oh no!"));
    let (err, file) = (result.with_context(|| "it failed").unwrap_err(), file!());
    assert_eq!(err.file(), file);
}

#[test]
fn test_option_context() {
    let option = None::<()>;
    let (err, file, line) = (option.context("it's none").unwrap_err(), file!(), line!());
    assert_eq!(err.file(), file);
    assert_eq!(err.line(), line);

    let option = None::<()>;
    let (err, file) = (option.with_context(|| "it's none").unwrap_err(), file!());
    assert_eq!(err.file(), file);
}
