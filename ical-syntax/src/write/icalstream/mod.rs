pub mod typed_writers;

#[cfg(test)]
mod test {
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

    #[cfg(feature = "chrono04")]
    #[test]
    fn writer_event() -> std::fmt::Result {
        use crate::write::value_types::TimeTransparency;

        use super::typed_writers::ICalStreamWriter;

        let dtstamp = chrono::DateTime::parse_from_rfc3339("2024-06-26T12:00:00Z")
            .unwrap()
            .to_utc();

        let mut buf = String::new();
        let mut ics = ICalStreamWriter::new(&mut buf);

        let mut ico = ics.icalendar_object("-//test//")?;

        let mut ev = ico.event()?;
        ev.dtstamp(dtstamp)?;
        ev.uid("unique identifier")?;
        ev.dtstart(dtstamp.date_naive())?;
        ev.summary("summary text")?;
        ev.time_transparency(TimeTransparency::Transparent)?;
        ev.end()?;

        ico.end()?;

        assert_eq!(
            &buf,
            "BEGIN:VCALENDAR\r\n\
            VERSION:2.0\r\n\
            PRODID:-//test//\r\n\
            BEGIN:VEVENT\r\n\
            DTSTAMP:20240626T120000Z\r\n\
            UID:unique identifier\r\n\
            DTSTART;VALUE=DATE:20240626\r\n\
            SUMMARY:summary text\r\n\
            TRANSP:TRANSPARENT\r\n\
            END:VEVENT\r\n\
            END:VCALENDAR\r\n"
        );
        Ok(())
    }
}
