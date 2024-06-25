use std::fmt::Display;

use crate::generate::text_writer::TextWriter;

use super::FoldingWriter;
use super::NameWriter;
use super::ParamtextWriter;
use super::QuotedStringWriter;

use std::fmt::Write;

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum State {
    Initial,
    AfterName,
    AfterParamName,
    AfterParamValue,
    Value,
}

pub struct ContentLine<W: Write> {
    pub(crate) inner: FoldingWriter<W>,
    pub(crate) state: State,
}

pub trait ParamValueWriter<W: Write> {
    fn param_value_unquoted_writer(
        &mut self,
    ) -> Result<ParamtextWriter<&'_ mut FoldingWriter<W>>, std::fmt::Error>;

    fn param_value_unquoted(&mut self, fmt: impl Display) -> std::fmt::Result {
        let mut pv = self.param_value_unquoted_writer()?;
        write!(&mut pv, "{}", fmt)
    }

    fn param_value_quoted_writer(
        &mut self,
    ) -> Result<QuotedStringWriter<&'_ mut FoldingWriter<W>>, std::fmt::Error>;

    fn param_value_quoted(&mut self, fmt: impl Display) -> std::fmt::Result {
        let mut pv = self.param_value_quoted_writer()?;
        write!(&mut pv, "{}", fmt)?;
        pv.close()
    }
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
        self.state = State::AfterName;

        NameWriter::new(&mut self.inner)
    }

    pub fn name(&mut self, fmt: impl Display) -> std::fmt::Result {
        let mut n = self.name_writer();
        write!(&mut n, "{}", fmt)
    }

    pub fn param_name_writer(
        &mut self,
    ) -> Result<NameWriter<&'_ mut FoldingWriter<W>>, std::fmt::Error> {
        assert!(self.state == State::AfterName || self.state == State::AfterParamValue);
        self.state = State::AfterParamName;

        self.inner.write_char(';')?;

        Ok(NameWriter::new(&mut self.inner))
    }

    pub fn param_name(&mut self, fmt: impl Display) -> std::fmt::Result {
        let mut pn = self.param_name_writer()?;
        write!(&mut pn, "{}", fmt)
    }

    fn goto_param_value_state(&mut self) -> std::fmt::Result {
        assert!(self.state == State::AfterParamName || self.state == State::AfterParamValue);

        if self.state == State::AfterParamName {
            self.inner.write_char('=')?;
        } else {
            self.inner.write_char(',')?;
        }

        self.state = State::AfterParamValue;

        Ok(())
    }

    pub fn param_unquoted(&mut self, name: impl Display, value: impl Display) -> std::fmt::Result {
        self.param_name(name)?;
        self.param_value_unquoted(value)
    }

    pub fn param_quoted(&mut self, name: impl Display, value: impl Display) -> std::fmt::Result {
        self.param_name(name)?;
        self.param_value_quoted(value)
    }

    fn value_writer(&mut self) -> Result<&'_ mut FoldingWriter<W>, std::fmt::Error> {
        assert!(self.state == State::AfterName || self.state == State::AfterParamValue);
        self.state = State::Value;

        self.inner.write_char(':')?;

        Ok(&mut self.inner)
    }

    pub fn value_tuple_writer<'x, 'y: 'x>(
        &'y mut self,
    ) -> Result<ValueTupleWriter<'x, W>, std::fmt::Error> {
        Ok(ValueTupleWriter::new(self.value_writer()?))
    }

    pub fn value(&mut self, fmt: impl Display) -> std::fmt::Result {
        let mut tw = self.value_tuple_writer()?;
        write!(&mut tw.next_value_writer()?, "{}", fmt)
    }

    pub fn eol(self) -> std::fmt::Result {
        assert_eq!(self.state, State::Value);
        self.inner.eol()
    }
}

impl<W: Write> ParamValueWriter<W> for ContentLine<W> {
    fn param_value_unquoted_writer(
        &mut self,
    ) -> Result<ParamtextWriter<&'_ mut FoldingWriter<W>>, std::fmt::Error> {
        self.goto_param_value_state()?;
        Ok(ParamtextWriter::new(&mut self.inner))
    }

    fn param_value_quoted_writer(
        &mut self,
    ) -> Result<QuotedStringWriter<&'_ mut FoldingWriter<W>>, std::fmt::Error> {
        self.goto_param_value_state()?;
        QuotedStringWriter::new(&mut self.inner)
    }
}

pub struct ValueTupleWriter<'a, W: Write> {
    inner: &'a mut FoldingWriter<W>,
    first_value: bool,
}

impl<'a, W: Write> ValueTupleWriter<'a, W> {
    pub fn new(inner: &'a mut FoldingWriter<W>) -> Self {
        Self {
            inner,
            first_value: true,
        }
    }

    pub fn next_value_writer<'x, 'y: 'x>(
        &'y mut self,
    ) -> Result<TextWriter<&'x mut FoldingWriter<W>>, std::fmt::Error> {
        if self.first_value {
            self.first_value = false;
        } else {
            self.inner.write_char(';')?;
        }

        Ok(TextWriter::new(&mut self.inner))
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

    #[test]
    fn content_line_param_value_list() -> std::fmt::Result {
        let mut buf = String::new();
        let mut cl = ContentLine::new(&mut buf);

        cl.name("X-PARAM-TEST")?;
        cl.param_name("LIST")?;
        cl.param_value_unquoted("unquoted text")?;
        cl.param_value_quoted("Quoted text, with comma and a ;")?;
        cl.value("value")?;

        cl.eol()?;

        assert_eq!(
            &buf,
            "X-PARAM-TEST;LIST=unquoted text,\"Quoted text, with comma and a ;\":value\r\n"
        );

        Ok(())
    }
}
