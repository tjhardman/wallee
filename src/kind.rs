// Tagged dispatch mechanism for resolving the behavior of `wallee!($expr)`.
//
// When wallee! is given a single expr argument to turn into wallee::Error, we
// want the resulting Error to pick up the input's implementation of source()
// and backtrace() if it has a std::error::Error impl, otherwise require nothing
// more than Display and Debug.
//
// Expressed in terms of specialization, we want something like:
//
//     trait WalleeNew {
//         fn new(self) -> Error;
//     }
//
//     impl<T> WalleeNew for T
//     where
//         T: Display + Debug + Send + Sync + 'static,
//     {
//         default fn new(self) -> Error {
//             /* no std error impl */
//         }
//     }
//
//     impl<T> WalleeNew for T
//     where
//         T: std::error::Error + Send + Sync + 'static,
//     {
//         fn new(self) -> Error {
//             /* use std error's source() and backtrace() */
//         }
//     }
//
// Since specialization is not stable yet, instead we rely on autoref behavior
// of method resolution to perform tagged dispatch. Here we have two traits
// AdhocKind and TraitKind that both have an wallee_kind() method. AdhocKind is
// implemented whether or not the caller's type has a std error impl, while
// TraitKind is implemented only when a std error impl does exist. The ambiguity
// is resolved by AdhocKind requiring an extra autoref so that it has lower
// precedence.
//
// The wallee! macro will set up the call in this form:
//
//     #[allow(unused_imports)]
//     use $crate::__private::{AdhocKind, TraitKind};
//     let error = $msg;
//     (&error).wallee_kind().new(error)

use crate::Error;
use core::fmt::{Debug, Display};

use crate::StdError;

pub struct MakeDebug;

#[doc(hidden)]
pub trait DebugKind: Sized {
    #[inline]
    fn wallee_kind(&self) -> MakeDebug {
        MakeDebug
    }
}

impl<T> DebugKind for &T where T: ?Sized + Display + Debug + Send + Sync + 'static {}

impl MakeDebug {
    #[cold]
    #[track_caller]
    pub fn make<M>(self, message: M) -> Error
    where
        M: Display + Debug + Send + Sync + 'static,
    {
        Error::from_debug(message, backtrace!())
    }
}

pub struct MakeStd;

#[doc(hidden)]
pub trait StdKind: Sized {
    #[inline]
    fn wallee_kind(&self) -> MakeStd {
        MakeStd
    }
}

impl<E> StdKind for E where E: StdError + Send + Sync + 'static {}

impl MakeStd {
    #[cold]
    #[track_caller]
    pub fn make<E>(self, error: E) -> Error
    where
        E: StdError + Send + Sync + 'static,
    {
        Error::new(error)
    }
}

pub struct MakeError;

#[doc(hidden)]
pub trait ErrorKind: Sized {
    #[inline]
    fn wallee_kind(&self) -> MakeError {
        MakeError
    }
}

impl ErrorKind for Error {}

impl MakeError {
    #[cold]
    #[track_caller]
    pub fn make(self, error: Error) -> Error {
        error
    }
}

pub struct Boxed;

#[doc(hidden)]
pub trait BoxedKind: Sized {
    #[inline]
    fn wallee_kind(&self) -> Boxed {
        Boxed
    }
}

impl BoxedKind for Box<dyn StdError + Send + Sync> {}

impl Boxed {
    #[cold]
    #[track_caller]
    pub fn make(self, error: Box<dyn StdError + Send + Sync>) -> Error {
        let backtrace = backtrace_if_absent!(&*error);
        Error::construct_from_boxed(error, backtrace)
    }
}
