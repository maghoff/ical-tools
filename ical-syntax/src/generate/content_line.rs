use std::fmt::Display;

use super::FoldingWriter;
use super::NameWriter;
use super::ParamtextWriter;
use super::QuotedStringWriter;

use std::fmt::Write;

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum State {
    Initial,
    Name,
    Param,
}

pub struct ContentLine<W: Write> {
    pub(crate) inner: FoldingWriter<W>,
    pub(crate) state: State,
}

impl<W: Write> ContentLine<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner: FoldingWriter::new(inner),
            state: State::Initial,
        }
    }

    pub fn name_writer(&mut self) -> NameWriter<&'_ mut FoldingWriter<W>> {
        assert_eq!(self.state, State::Initial);
        self.state = State::Name;

        NameWriter::new(&mut self.inner)
    }

    pub fn name(&mut self, fmt: impl Display) -> std::fmt::Result {
        let mut n = self.name_writer();
        write!(&mut n, "{}", fmt)
    }

    pub fn param_name_writer(
        &mut self,
    ) -> Result<NameWriter<&'_ mut FoldingWriter<W>>, std::fmt::Error> {
        assert_eq!(self.state, State::Name);
        self.state = State::Param;

        self.inner.write_char(';')?;

        Ok(NameWriter::new(&mut self.inner))
    }

    pub fn param_name(&mut self, fmt: impl Display) -> std::fmt::Result {
        let mut pn = self.param_name_writer()?;
        write!(&mut pn, "{}", fmt)
    }

    pub fn param_value_unquoted_writer(
        &mut self,
    ) -> Result<ParamtextWriter<&'_ mut FoldingWriter<W>>, std::fmt::Error> {
        assert_eq!(self.state, State::Param);
        self.state = State::Name;

        self.inner.write_char('=')?;

        Ok(ParamtextWriter::new(&mut self.inner))
    }

    pub fn param_value_unquoted(&mut self, fmt: impl Display) -> std::fmt::Result {
        let mut pv = self.param_value_unquoted_writer()?;
        write!(&mut pv, "{}", fmt)
    }

    pub fn param_value_quoted_writer(
        &mut self,
    ) -> Result<QuotedStringWriter<&'_ mut FoldingWriter<W>>, std::fmt::Error> {
        assert_eq!(self.state, State::Param);
        self.state = State::Name;

        self.inner.write_char('=')?;

        QuotedStringWriter::new(&mut self.inner)
    }

    pub fn param_value_quoted(&mut self, fmt: impl Display) -> std::fmt::Result {
        let mut pv = self.param_value_quoted_writer()?;
        write!(&mut pv, "{}", fmt)?;
        pv.close()
    }

    pub fn param_unquoted(&mut self, name: impl Display, value: impl Display) -> std::fmt::Result {
        self.param_name(name)?;
        self.param_value_unquoted(value)
    }

    pub fn param_quoted(&mut self, name: impl Display, value: impl Display) -> std::fmt::Result {
        self.param_name(name)?;
        self.param_value_quoted(value)
    }

    pub fn value_writer(&mut self) -> Result<&'_ mut FoldingWriter<W>, std::fmt::Error> {
        assert_eq!(self.state, State::Name);

        self.inner.write_char(':')?;

        Ok(&mut self.inner)
    }

    pub fn value(&mut self, fmt: impl Display) -> std::fmt::Result {
        let mut v = self.value_writer()?;
        write!(&mut v, "{}", fmt)
    }

    pub fn eol(self) -> std::fmt::Result {
        self.inner.eol()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn content_line_writer() -> std::fmt::Result {
        let mut buf = String::new();
        let mut cl = ContentLine::new(&mut buf);

        let mut n = cl.name_writer();
        write!(&mut n, "BEGIN")?;

        let mut v = cl.value_writer()?;
        write!(&mut v, "VCALENDAR")?;

        cl.eol()?;

        assert_eq!(&buf, "BEGIN:VCALENDAR\r\n");

        Ok(())
    }

    #[test]
    fn content_line_params_writer() -> std::fmt::Result {
        let mut buf = String::new();
        let mut cl = ContentLine::new(&mut buf);

        let mut n = cl.name_writer();
        write!(&mut n, "X-PARAM-TEST")?;

        let mut pn = cl.param_name_writer()?;
        write!(&mut pn, "UNQUOTED")?;

        let mut pv = cl.param_value_unquoted_writer()?;
        write!(&mut pv, "unquoted text")?;

        let mut pn = cl.param_name_writer()?;
        write!(&mut pn, "QUOTED")?;

        let mut pv = cl.param_value_quoted_writer()?;
        write!(&mut pv, "Quoted text, with comma and a ;")?;
        pv.close()?;

        let mut v = cl.value_writer()?;
        write!(&mut v, "value")?;

        cl.eol()?;

        assert_eq!(
            &buf,
            "X-PARAM-TEST;UNQUOTED=unquoted text;QUOTED=\"Quoted text, with comma and a ;\r\n \":value\r\n"
        );

        Ok(())
    }

    #[test]
    fn content_line_params() -> std::fmt::Result {
        let mut buf = String::new();
        let mut cl = ContentLine::new(&mut buf);

        cl.name("X-PARAM-TEST")?;

        cl.param_name("UNQUOTED")?;
        cl.param_value_unquoted("unquoted text")?;

        cl.param_name("QUOTED")?;
        cl.param_value_quoted("Quoted text, with comma and a ;")?;

        cl.value("value")?;

        cl.eol()?;

        assert_eq!(
            &buf,
            "X-PARAM-TEST;UNQUOTED=unquoted text;QUOTED=\"Quoted text, with comma and a ;\r\n \":value\r\n"
        );

        Ok(())
    }

    #[test]
    fn content_line_params_pairs() -> std::fmt::Result {
        let mut buf = String::new();
        let mut cl = ContentLine::new(&mut buf);

        cl.name("X-PARAM-TEST")?;
        cl.param_unquoted("UNQUOTED", "unquoted text")?;
        cl.param_quoted("QUOTED", "Quoted text, with comma and a ;")?;
        cl.value("value")?;

        cl.eol()?;

        assert_eq!(
            &buf,
            "X-PARAM-TEST;UNQUOTED=unquoted text;QUOTED=\"Quoted text, with comma and a ;\r\n \":value\r\n"
        );

        Ok(())
    }
}
