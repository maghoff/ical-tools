//! [RFC 5545 3.8.2: Date and Time Component Properties](https://www.rfc-editor.org/rfc/rfc5545#section-3.8.2)

use crate::structure::{
    icalstream::components::{
        DaylightCProperty, EventCProperty, FreeBusyCProperty, StandardCProperty, TodoCProperty,
    },
    value_types::{DateTime, Text},
    Property,
};

/// Date-Time Start
///
/// Property Name:  DTSTART
///
/// Purpose:  This property specifies when the calendar component begins.
///
/// Value Type:  The default value type is DATE-TIME.  The time value MUST be
/// one of the forms defined for the DATE-TIME value type. The value type can
/// be set to a DATE value type.
///
/// Property Parameters:  IANA, non-standard, value data type, and time zone
/// identifier property parameters can be specified on this property.
///
/// Conformance:  This property can be specified once in the "VEVENT",
/// "VTODO", or "VFREEBUSY" calendar components as well as in the "STANDARD"
/// and "DAYLIGHT" sub-components.  This property is REQUIRED in all types of
/// recurring calendar components that specify the "RRULE" property.  This
/// property is also REQUIRED in "VEVENT" calendar components contained in
/// iCalendar objects that don't specify the "METHOD" property.
///
/// Description:  Within the "VEVENT" calendar component, this property
/// defines the start date and time for the event.
///
/// Within the "VFREEBUSY" calendar component, this property defines the start
/// date and time for the free or busy time information. The time MUST be
/// specified in UTC time.
///
/// Within the "STANDARD" and "DAYLIGHT" sub-components, this property defines
/// the effective start date and time for a time zone specification.  This
/// property is REQUIRED within each "STANDARD" and "DAYLIGHT" sub-components
/// included in "VTIMEZONE" calendar components and MUST be specified as a
/// date with local time without the "TZID" property parameter.
///
/// Format Definition:  This property is defined by the following notation:
///
/// ```abnf
/// dtstart    = "DTSTART" dtstparam ":" dtstval CRLF
///
/// dtstparam  = *(
///            ;
///            ; The following are OPTIONAL,
///            ; but MUST NOT occur more than once.
///            ;
///            (";" "VALUE" "=" ("DATE-TIME" / "DATE")) /
///            (";" tzidparam) /
///            ;
///            ; The following is OPTIONAL,
///            ; and MAY occur more than once.
///            ;
///            (";" other-param)
///            ;
///            )
///
/// dtstval    = date-time / date
/// ;Value MUST match value type
/// ```
///
/// Example:  The following is an example of this property:
///
/// ```text
/// DTSTART:19980118T073000Z
/// ```
///
/// Reference: [RFC 5545
/// 3.8.2.4](https://www.rfc-editor.org/rfc/rfc5545#section-3.8.2.4)
pub struct DateTimeStart;

impl Property for DateTimeStart {
    const NAME: &'static str = "DTSTART";

    // TODO: The value type is really a choice of Date or DateTime, with
    // DateTime as default
    type ValueType = DateTime;
}

impl EventCProperty for DateTimeStart {}
impl TodoCProperty for DateTimeStart {}
impl FreeBusyCProperty for DateTimeStart {}
impl StandardCProperty for DateTimeStart {}
impl DaylightCProperty for DateTimeStart {}

/// Time Transparency
///
/// Property Name:  TRANSP
///
/// Purpose:  This property defines whether or not an event is transparent to
/// busy time searches.
///
/// Value Type:  TEXT
///
/// Property Parameters:  IANA and non-standard property parameters can be
/// specified on this property.
///
/// Conformance:  This property can be specified once in a "VEVENT" calendar
/// component.
///
/// Description:  Time Transparency is the characteristic of an event that
/// determines whether it appears to consume time on a calendar. Events that
/// consume actual time for the individual or resource associated with the
/// calendar SHOULD be recorded as OPAQUE, allowing them to be detected by
/// free/busy time searches.  Other events, which do not take up the
/// individual's (or resource's) time SHOULD be recorded as TRANSPARENT,
/// making them invisible to free/ busy time searches.
///
/// Format Definition:  This property is defined by the following notation:
///
/// ```abnf
/// transp     = "TRANSP" transparam ":" transvalue CRLF
///
/// transparam = *(";" other-param)
///
/// transvalue = "OPAQUE"
///             ;Blocks or opaque on busy time searches.
///             / "TRANSPARENT"
///             ;Transparent on busy time searches.
/// ;Default value is OPAQUE
/// ```
///
/// Example:  The following is an example of this property for an event that
/// is transparent or does not block on free/busy time searches:
///
/// ```text
/// TRANSP:TRANSPARENT
/// ```
///
/// The following is an example of this property for an event that is opaque
/// or blocks on free/busy time searches:
///
/// ```text
/// TRANSP:OPAQUE
/// ```
///
/// Reference: [RFC 5545
/// 3.8.2.4](https://www.rfc-editor.org/rfc/rfc5545#section-3.8.2.4)
pub struct TimeTransparency;

impl Property for TimeTransparency {
    const NAME: &'static str = "TRANSP";

    type ValueType = Text;
}

impl EventCProperty for TimeTransparency {}
