use std::fmt::Write;

/// A writer for the quoted-string grammar:
///
///     quoted-string = DQUOTE *QSAFE-CHAR DQUOTE
///
///     QSAFE-CHAR    = WSP / %x21 / %x23-7E / NON-US-ASCII
///     ; Any character except CONTROL and DQUOTE
///
///     CONTROL       = %x00-08 / %x0A-1F / %x7F
///     ; All the controls except HTAB
///
/// There is no escaping mechanism inside the quoted string, so if you want to
/// write, say, a URI containing the DQUOTE character, it must be escaped using
/// URI encoding: `%22`
///
/// Writing text containing forbidden characters will yield `Result::Err`.
///
/// This writer inserts the leading and trailing DQUOTEs for the quoted string.
///
/// Use [`ParamtextWriter`] for writing unquoted param values.
pub struct QuotedStringWriter<W: Write> {
    inner: W,
}

impl<W: Write> Write for QuotedStringWriter<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        // Validate as QSAFE-CHAR from the BNF.
        let is_valid = s
            .bytes()
            .all(|x| x == b'\t' || ((0x20..=0x7e).contains(&x) && x != b'"') || x >= 0x80);
        if !is_valid {
            return Err(std::fmt::Error);
        }

        self.inner.write_str(s)
    }
}

impl<W: Write> QuotedStringWriter<W> {
    pub fn new(mut inner: W) -> Result<Self, std::fmt::Error> {
        inner.write_char('"')?;
        Ok(Self { inner })
    }

    pub fn close(mut self) -> Result<W, std::fmt::Error> {
        self.inner.write_char('"')?;
        Ok(self.inner)
    }
}

/// A writer for the paramtext grammar:
///
///     paramtext     = *SAFE-CHAR
///
///     SAFE-CHAR     = WSP / %x21 / %x23-2B / %x2D-39 / %x3C-7E
///                    / NON-US-ASCII
///     ; Any character except CONTROL, DQUOTE, ";", ":", ","
///
///     CONTROL       = %x00-08 / %x0A-1F / %x7F
///     ; All the controls except HTAB
///
/// Writing text containing forbidden characters will yield `Result::Err`.
///
/// Use [`QuotedStringWriter`] for writing quoted param values.
pub struct ParamtextWriter<W: Write> {
    inner: W,
}

impl<W: Write> Write for ParamtextWriter<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        // Validate as SAFE-CHAR from the BNF.
        let is_valid = s.bytes().all(|x| {
            x == b'\t'
                || ((0x20..=0x7e).contains(&x) && x != b'"' && x != b';' && x != b':' && x != b',')
                || x >= 0x80
        });
        if !is_valid {
            return Err(std::fmt::Error);
        }

        self.inner.write_str(s)
    }
}

impl<W: Write> ParamtextWriter<W> {
    pub fn new(inner: W) -> Self {
        Self { inner }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quoted_string_writer_positive() {
        let mut buf = String::new();
        let mut w = QuotedStringWriter::new(&mut buf).unwrap();
        write!(w, "I am a string. Quote me on that!").unwrap();
        w.close().unwrap();
        assert_eq!(&buf, "\"I am a string. Quote me on that!\"");
    }

    #[test]
    fn quoted_string_writer_reserved_chars() {
        let mut buf = String::new();
        let mut w = QuotedStringWriter::new(&mut buf).unwrap();
        write!(w, "I can contain :, ; and , no problem!").unwrap();
        w.close().unwrap();
        assert_eq!(&buf, "\"I can contain :, ; and , no problem!\"");
    }

    #[test]
    fn quoted_string_writer_unicode() {
        let mut buf = String::new();
        let mut w = QuotedStringWriter::new(&mut buf).unwrap();
        write!(w, "\u{1F92A}").unwrap();
        w.close().unwrap();
        assert_eq!(&buf, "\"\u{1F92A}\"");
    }

    #[test]
    fn quoted_string_writer_negative() {
        let mut buf = String::new();
        let mut w = QuotedStringWriter::new(&mut buf).unwrap();
        assert!(writeln!(w, "I accidentally contain a newline").is_err());
        assert!(write!(w, "I also may not contain \"").is_err());
        w.close().unwrap();
    }

    #[test]
    fn paramtext_writer_positive() {
        let mut buf = String::new();
        let mut w = ParamtextWriter::new(&mut buf);
        write!(w, "I am a string \u{1F92A}").unwrap();
        assert_eq!(&buf, "I am a string \u{1F92A}");
    }

    #[test]
    fn paramtext_writer_negative() {
        let mut buf = String::new();
        let mut w = ParamtextWriter::new(&mut buf);
        assert!(writeln!(w, "I accidentally contain a newline").is_err());
        assert!(write!(w, "\x7f").is_err());
        assert!(write!(w, "\x05").is_err());
        assert!(write!(w, "\"").is_err());
        assert!(write!(w, ":").is_err());
        assert!(write!(w, ";").is_err());
        assert!(write!(w, ",").is_err());
    }
}
