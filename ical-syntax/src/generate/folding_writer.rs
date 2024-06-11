use std::fmt::Write;

const MAX_LINE_LENGTH: u32 = 75;
const CONTINUATION: &str = "\r\n ";
const CRLF: &str = "\r\n";

/// A generator for "content lines", which are subject to folding at line
/// lengths of at most 75 octets.
///
/// To maintain some human readability, this implementation folds at UTF-8
/// codepoint boundaries. Grapheme clusters may be split.
pub struct FoldingWriter<W: Write> {
    inner: W,
    rem_line_len: u32,
    passed_eol: bool,
}

impl<W: Write> FoldingWriter<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            rem_line_len: MAX_LINE_LENGTH,
            passed_eol: false,
        }
    }

    pub fn eol(mut self) -> std::fmt::Result {
        self.passed_eol = true;
        self.inner.write_str(CRLF)
    }
}

impl<W: Write> Drop for FoldingWriter<W> {
    fn drop(&mut self) {
        assert!(
            self.passed_eol,
            "FoldingWriter::eol() must be called before dropping the value"
        );
    }
}

impl<W: Write> Write for FoldingWriter<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mut b = s.as_bytes();

        // Validate as VALUE-CHAR from the BNF.
        if b.iter().any(|&x| x == 0x7f || (x < 0x20 && x != b'\t')) {
            // Control chars are not valid at this level of the syntax. Some
            // value types can transport newlines as "\\n".
            return Err(std::fmt::Error);
        }

        while b.len() > self.rem_line_len as usize {
            let mut end = self.rem_line_len as usize;

            // This is guaranteed to terminate after at most three iterations
            // since the input is valid UTF-8
            while b[end] & 0xc0 == 0x80 {
                end -= 1;
            }

            // Consider using from_utf8_unchecked instead. We have maintained
            // the UTF-8 invariants.
            self.inner
                .write_str(std::str::from_utf8(&b[0..end]).unwrap())?;
            self.inner.write_str(CONTINUATION)?;
            b = &b[end..];

            // Subtract one to account for the leading SPACE due to the
            // continuation sequence
            self.rem_line_len = MAX_LINE_LENGTH - 1;
        }

        self.rem_line_len -= b.len() as u32;
        // Consider using from_utf8_unchecked instead. We have maintained
        // the UTF-8 invariants.
        self.inner.write_str(std::str::from_utf8(b).unwrap())?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let mut buf = String::new();
        let mut line_writer = FoldingWriter::new(&mut buf);

        write!(line_writer, "simple test").unwrap();
        line_writer.eol().unwrap();

        assert_eq!(&buf, "simple test\r\n");
    }

    #[test]
    fn ascii_folding() {
        let mut buf = String::new();
        let mut line_writer = FoldingWriter::new(&mut buf);

        write!(
            line_writer,
            "test string exceeding 75 chars, all ASCII, to see that it does indeed get folded"
        )
        .unwrap();
        line_writer.eol().unwrap();

        assert_eq!(
            &buf,
            "test string exceeding 75 chars, all ASCII, to see that it does indeed get f\r\n olded\r\n"
        );
    }

    #[test]
    fn utf8_folding() {
        let mut buf = String::new();
        let mut line_writer = FoldingWriter::new(&mut buf);

        write!(
            line_writer,
            "test string exceeding 75 chars, with a multi-byte UTF-8 character juust \u{1F90F} at the fold"
        )
        .unwrap();
        line_writer.eol().unwrap();

        assert_eq!(
            &buf,
            "test string exceeding 75 chars, with a multi-byte UTF-8 character juust \r\n \u{1F90F} at the fold\r\n"
        );
    }

    #[test]
    fn chunked_folding() {
        let mut buf = String::new();
        let mut line_writer = FoldingWriter::new(&mut buf);

        write!(line_writer, "test line exceeding 75 chars, all ASCII, ").unwrap();
        write!(line_writer, "to see that it does indeed get folded").unwrap();
        line_writer.eol().unwrap();

        assert_eq!(
            &buf,
            "test line exceeding 75 chars, all ASCII, to see that it does indeed get fol\r\n ded\r\n"
        );
    }
}
