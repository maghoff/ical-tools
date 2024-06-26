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
}
