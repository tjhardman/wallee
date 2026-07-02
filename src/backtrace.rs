pub(crate) use std::backtrace::{Backtrace, BacktraceStatus};

macro_rules! impl_backtrace {
    () => {
        std::backtrace::Backtrace
    };
}

macro_rules! backtrace {
    () => {
        Some(crate::backtrace::Backtrace::capture())
    };
}

#[cfg(error_generic_member_access)]
macro_rules! backtrace_if_absent {
    ($err:expr) => {
        match std::error::request_ref::<std::backtrace::Backtrace>(
            $err as &dyn std::error::Error,
        ) {
            // The underlying error already carries a backtrace, so don't
            // capture a redundant one here.
            Some(_) => None,
            None => backtrace!(),
        }
    };
}

#[cfg(not(error_generic_member_access))]
macro_rules! backtrace_if_absent {
    ($err:expr) => {
        backtrace!()
    };
}

fn _assert_send_sync() {
    fn _assert<T: Send + Sync>() {}
    _assert::<Backtrace>();
}
