pub mod composite_value_types;
mod content_line;
mod folding_writer;
pub mod icalstream;
mod io_adapters;
mod line_stream;
mod parameter_value_items;
pub mod text_writer;
mod validating_writers;
pub mod value_types;
mod writer;

pub use content_line::ContentLine;
pub use folding_writer::FoldingWriter;
pub use line_stream::LineStream;
pub use validating_writers::{NameWriter, ParamtextWriter, QuotedStringWriter};
pub use writer::*;

#[cfg(test)]
mod test {
    use super::Writer;

    #[test]
    fn writer() -> std::fmt::Result {
        use crate::structure::icalstream::{
            components::ICalObject,
            properties::calendar::{ProdId, Version},
        };

        let mut buf = String::new();
        let mut ics = Writer::with_fmt(&mut buf);

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

    #[cfg(feature = "chrono04")]
    #[test]
    fn writer_event_chrono04() -> std::fmt::Result {
        use crate::{
            structure::icalstream::{
                components::{EventC, ICalObject},
                properties::{
                    calendar::{ProdId, Version},
                    change_management::DateTimeStamp,
                    date_and_time::{DateTimeStart, TimeTransparency},
                    descriptive::Summary,
                    relationship::Uid,
                },
            },
            write::value_types::TimeTransparency as TimeTransparencyValue,
        };

        let dtstamp = chrono::DateTime::parse_from_rfc3339("2024-06-26T12:00:00Z")
            .unwrap()
            .to_utc();

        let mut buf = String::new();
        let mut ics = Writer::with_fmt(&mut buf);

        let mut ico = ics.component(ICalObject)?;
        ico.simple_property(Version, "2.0")?;
        ico.simple_property(ProdId, "-//test//")?;

        let mut ev = ico.component(EventC)?;
        ev.simple_property(DateTimeStamp, dtstamp)?;
        ev.simple_property(Uid, "unique identifier")?;

        // Just test all the different options while we're at it:
        ev.simple_property(DateTimeStart, dtstamp.date_naive())?;
        ev.simple_property(DateTimeStart, dtstamp.naive_utc())?;
        ev.simple_property(DateTimeStart, dtstamp)?;

        ev.simple_property(Summary, "summary text")?;
        ev.simple_property(TimeTransparency, TimeTransparencyValue::Transparent)?;
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
            DTSTART:20240626T120000\r\n\
            DTSTART:20240626T120000Z\r\n\
            SUMMARY:summary text\r\n\
            TRANSP:TRANSPARENT\r\n\
            END:VEVENT\r\n\
            END:VCALENDAR\r\n"
        );
        Ok(())
    }

    #[cfg(feature = "jiff02")]
    #[test]
    fn writer_event_jiff02() -> std::fmt::Result {
        use crate::{
            structure::icalstream::{
                components::{EventC, ICalObject},
                properties::{
                    calendar::{ProdId, Version},
                    change_management::DateTimeStamp,
                    date_and_time::{DateTimeStart, TimeTransparency},
                    descriptive::Summary,
                    relationship::Uid,
                },
            },
            write::{
                value_types::jiff02::UtcForm,
                value_types::TimeTransparency as TimeTransparencyValue,
            },
        };

        let dtstamp: jiff::civil::DateTime = "2024-06-26 12:00:00".parse().unwrap();

        let mut buf = String::new();
        let mut ics = Writer::with_fmt(&mut buf);

        let mut ico = ics.component(ICalObject)?;
        ico.simple_property(Version, "2.0")?;
        ico.simple_property(ProdId, "-//test//")?;

        let mut ev = ico.component(EventC)?;
        ev.simple_property(DateTimeStamp, UtcForm::from_civil(dtstamp))?;
        ev.simple_property(Uid, "unique identifier")?;

        // Just test all the different options while we're at it:
        ev.simple_property(DateTimeStart, dtstamp.date())?;
        ev.simple_property(DateTimeStart, dtstamp)?;
        ev.simple_property(DateTimeStart, UtcForm::from_civil(dtstamp))?;

        ev.simple_property(Summary, "summary text")?;
        ev.simple_property(TimeTransparency, TimeTransparencyValue::Transparent)?;
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
            DTSTART:20240626T120000\r\n\
            DTSTART:20240626T120000Z\r\n\
            SUMMARY:summary text\r\n\
            TRANSP:TRANSPARENT\r\n\
            END:VEVENT\r\n\
            END:VCALENDAR\r\n"
        );
        Ok(())
    }
}
