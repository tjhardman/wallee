use crate::chain::Chain;
use crate::error::ErrorImpl;
use crate::ptr::RefPtr;
use core::fmt::{self, Write};

impl ErrorImpl {
    pub(crate) unsafe fn display(this: RefPtr<Self>, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", unsafe { Self::error(this) })?;

        if f.alternate() {
            let chain = unsafe { Self::chain(this) };
            for cause in chain.skip(1) {
                write!(f, ": {}", cause)?;
            }
        }

        Ok(())
    }

    pub(crate) unsafe fn debug(this: RefPtr<Self>, f: &mut fmt::Formatter) -> fmt::Result {
        let error = unsafe { Self::error(this) };
        let location = unsafe { Self::location(this) };

        if f.alternate() {
            return f
                .debug_struct("Wallee")
                .field(
                    "location",
                    &format!(
                        "{}({}:{})",
                        location.file(),
                        location.line(),
                        location.column()
                    ),
                )
                .field("error", &error)
                .finish();
        }

        write!(
            f,
            "{}({}:{}): {}",
            location.file(),
            location.line(),
            location.column(),
            error
        )?;

        if let Some(cause) = error.source() {
            write!(f, "\n\nCaused by:")?;
            let multiple = cause.source().is_some();
            for (n, error) in Chain::new(cause).enumerate() {
                writeln!(f)?;
                let mut indented = Indented {
                    inner: f,
                    number: if multiple { Some(n) } else { None },
                    started: false,
                };
                write!(indented, "{}", error)?;
            }
        }

        use crate::backtrace::BacktraceStatus;

        let backtrace = unsafe { Self::backtrace(this) };
        if let BacktraceStatus::Captured = backtrace.status() {
            let mut backtrace = backtrace.to_string();
            write!(f, "\n\n")?;
            if backtrace.starts_with("stack backtrace:") {
                // Capitalize to match "Caused by:"
                backtrace.replace_range(0..1, "S");
            } else {
                // "stack backtrace:" prefix was removed in
                // https://github.com/rust-lang/backtrace-rs/pull/286
                writeln!(f, "Stack backtrace:")?;
            }
            backtrace.truncate(backtrace.trim_end().len());
            write!(f, "{}", backtrace)?;
        }

        Ok(())
    }
}

struct Indented<'a, D> {
    inner: &'a mut D,
    number: Option<usize>,
    started: bool,
}

impl<T> Write for Indented<'_, T>
where
    T: Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for (i, line) in s.split('\n').enumerate() {
            if !self.started {
                self.started = true;
                match self.number {
                    Some(number) => write!(self.inner, "{: >5}: ", number)?,
                    None => self.inner.write_str("    ")?,
                }
            } else if i > 0 {
                self.inner.write_char('\n')?;
                if self.number.is_some() {
                    self.inner.write_str("       ")?;
                } else {
                    self.inner.write_str("    ")?;
                }
            }

            self.inner.write_str(line)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_digit() {
        let input = "verify\nthis";
        let expected = "    2: verify\n       this";
        let mut output = String::new();

        Indented {
            inner: &mut output,
            number: Some(2),
            started: false,
        }
        .write_str(input)
        .unwrap();

        assert_eq!(expected, output);
    }

    #[test]
    fn two_digits() {
        let input = "verify\nthis";
        let expected = "   12: verify\n       this";
        let mut output = String::new();

        Indented {
            inner: &mut output,
            number: Some(12),
            started: false,
        }
        .write_str(input)
        .unwrap();

        assert_eq!(expected, output);
    }

    #[test]
    fn no_digits() {
        let input = "verify\nthis";
        let expected = "    verify\n    this";
        let mut output = String::new();

        Indented {
            inner: &mut output,
            number: None,
            started: false,
        }
        .write_str(input)
        .unwrap();

        assert_eq!(expected, output);
    }
}
