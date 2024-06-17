pub mod typed_writers;

#[cfg(test)]
mod test {
    use crate::generate::Writer;

    #[test]
    fn writer() -> std::fmt::Result {
        use crate::structure::icalstream::{
            components::ICalObject,
            properties::calendar::{ProdId, Version},
        };

        let mut buf = String::new();
        let mut ics = Writer::new(&mut buf);

        let mut ico = ics.component(ICalObject)?;
        ico.simple_property(Version, "2.0")?;
        ico.simple_property(ProdId, "-//test//")?;
        ico.end()?;

        assert_eq!(
            &buf,
            "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//test//\r\nEND:VCALENDAR\r\n"
        );
        Ok(())
    }

    #[test]
    fn icalstream() -> std::fmt::Result {
        use super::typed_writers::ICalStreamWriter;

        let mut buf = String::new();
        let mut ics = ICalStreamWriter::new(&mut buf);

        let ico = ics.icalendar_object("-//test//")?;
        ico.end()?;

        assert_eq!(
            &buf,
            "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//test//\r\nEND:VCALENDAR\r\n"
        );
        Ok(())
    }
}
