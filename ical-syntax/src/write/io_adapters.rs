//! Adapted from [io-adapters](https://crates.io/crates/io-adapters)
//!
//! See also [rust-lang/libs-team#113](https://github.com/rust-lang/libs-team/issues/133)
//! for the issue of getting this into std.
use std::{fmt, io};

#[derive(Debug)]
pub struct FmtToIo<W> {
    inner: W,
    pub error: Option<io::Error>,
}

impl<W: io::Write> fmt::Write for FmtToIo<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.inner.write_all(s.as_bytes()) {
            Ok(()) => {
                self.error = None;
                Ok(())
            }
            Err(e) => {
                self.error = Some(e);
                Err(fmt::Error)
            }
        }
    }
}

pub trait WriteExtension<T> {
    type Adapter;

    fn write_adapter(self) -> Self::Adapter;
}

impl<W: io::Write> WriteExtension<FmtToIo<W>> for W {
    type Adapter = FmtToIo<W>;

    fn write_adapter(self) -> FmtToIo<W> {
        FmtToIo {
            inner: self,
            error: None,
        }
    }
}
