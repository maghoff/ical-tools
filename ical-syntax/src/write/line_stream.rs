use std::fmt::Display;
use std::fmt::Write;

use super::ContentLine;

pub struct LineStream<W> {
    inner: W,
}

impl<W: Write> LineStream<W> {
    pub fn new(inner: W) -> Self {
        Self { inner }
    }

    pub fn content_line(&mut self) -> ContentLine<&mut W> {
        ContentLine::new(&mut self.inner)
    }

    pub fn simple_line(&mut self, name: impl Display, value: impl Display) -> std::fmt::Result {
        let mut cl = self.content_line();
        cl.name(name)?;
        cl.value(value)?;
        cl.eol()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn line_stream() -> std::fmt::Result {
        let mut buf = String::new();
        let mut ls = LineStream::new(&mut buf);

        ls.simple_line("BEGIN", "VCALENDAR")?;

        let mut cl = ls.content_line();
        cl.name("X-PARAM-TEST")?;
        cl.param_unquoted("UNQUOTED", "unquoted text")?;
        cl.param_quoted("QUOTED", "Quoted text, with comma and a ;")?;
        cl.value("value")?;
        cl.eol()?;

        ls.simple_line("END", "VCALENDAR")?;

        assert_eq!(&buf, "BEGIN:VCALENDAR\r\nX-PARAM-TEST;UNQUOTED=unquoted text;QUOTED=\"Quoted text, with comma and a ;\r\n \":value\r\nEND:VCALENDAR\r\n");

        Ok(())
    }
}
