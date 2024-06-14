use super::writer::Component;

/// Reference: [RFC5545 3.4](https://www.rfc-editor.org/rfc/rfc5545.html#section-3.4)
pub struct ICalObject;
impl Component for ICalObject {
    const NAME: &'static str = "VCALENDAR";
}
