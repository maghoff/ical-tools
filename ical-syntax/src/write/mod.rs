pub mod composite_value_types;
mod content_line;
mod folding_writer;
pub mod icalstream;
mod line_stream;
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
    use crate::structure::icalstream::{
        components::EventC,
        properties::{date_and_time::TimeTransparency, descriptive::Summary, relationship::Uid},
    };

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
}
