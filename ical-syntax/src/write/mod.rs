pub mod chrono04;
pub mod composite_value_types;
mod content_line;
mod folding_writer;
pub mod icalstream;
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

    #[cfg(feature = "chrono04")]
    #[test]
    fn writer_event() -> std::fmt::Result {
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
                chrono04::{DateTimeForm, DateTimeOrDate},
                value_types::TimeTransparency as TimeTransparencyValue,
            },
        };

        let dtstamp = chrono::DateTime::parse_from_rfc3339("2024-06-26T12:00:00Z")
            .unwrap()
            .to_utc();

        let mut buf = String::new();
        let mut ics = Writer::new(&mut buf);

        let mut ico = ics.component(ICalObject)?;
        ico.simple_property(Version, "2.0")?;
        ico.simple_property(ProdId, "-//test//")?;

        let mut ev = ico.component(EventC)?;
        ev.simple_property(DateTimeStamp, DateTimeForm::from(dtstamp))?;
        ev.simple_property(Uid, "unique identifier")?;
        ev.simple_property(DateTimeStart, DateTimeOrDate::from(dtstamp.date_naive()))?;
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
            SUMMARY:summary text\r\n\
            TRANSP:TRANSPARENT\r\n\
            END:VEVENT\r\n\
            END:VCALENDAR\r\n"
        );
        Ok(())
    }
}
