use super::*;

pub mod components;
pub mod properties;
pub mod typed_writers;
pub mod value_types;

#[cfg(test)]
mod test {
    use components::ICalObject;
    use properties::{ProdId, Version};
    use typed_writers::ICalStreamWriter;

    use super::*;

    #[test]
    fn writer() -> std::fmt::Result {
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
