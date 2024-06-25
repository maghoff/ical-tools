use std::fmt::Write;

/// A writer implementing escaping for TEXT values.
///
/// This is simple backslash-escaping of the characters `\`, `\n`, `;` and `,`.
///
/// No other value types may contain any of these characters, which have
/// syntactic significance. Hence, this writer is applicable to any value type,
/// even if it isn't neccessary for types other than TEXT.
///
/// Reference: [RFC 5545
/// 3.3.11](https://www.rfc-editor.org/rfc/rfc5545#section-3.3.11)
pub struct TextWriter<W: Write> {
    inner: W,
}

impl<W: Write> TextWriter<W> {
    pub fn new(inner: W) -> Self {
        Self { inner }
    }
}

impl<W: Write> Write for TextWriter<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mut s = s;
        while !s.is_empty() {
            match s.find(['\\', '\n', ';', ',']) {
                Some(idx) => {
                    let (head, tail) = s.split_at(idx);
                    let (separator, tail) = tail.split_at(1);
                    let separator = separator.chars().next().unwrap();
                    s = tail;

                    write!(
                        self.inner,
                        "{}\\{}",
                        head,
                        if separator == '\n' { 'n' } else { separator }
                    )?;
                }
                None => {
                    write!(self.inner, "{}", s)?;
                    break;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Write;

    use super::TextWriter;

    #[test]
    fn text_noescaping() -> std::fmt::Result {
        let mut buf = String::new();
        let mut w = TextWriter::new(&mut buf);
        write!(&mut w, "Basic test with no escaping")?;
        assert_eq!(&buf, "Basic test with no escaping");
        Ok(())
    }

    #[test]
    fn text_escaping() -> std::fmt::Result {
        let mut buf = String::new();
        let mut w = TextWriter::new(&mut buf);
        write!(&mut w, "\n;,\\")?;
        assert_eq!(&buf, "\\n\\;\\,\\\\");
        Ok(())
    }

    #[test]
    fn text_escaping2() -> std::fmt::Result {
        let mut buf = String::new();
        let mut w = TextWriter::new(&mut buf);
        write!(&mut w, "Mix\nwith different; yet equivalent, parts")?;
        assert_eq!(&buf, "Mix\\nwith different\\; yet equivalent\\, parts");
        Ok(())
    }
}
