//! [RFC 5545 3.8.2: Date and Time Component Properties](https://www.rfc-editor.org/rfc/rfc5545#section-3.8.2)

use crate::structure::{
    composite_value_types::{Any2, Any3, List},
    icalstream::components::{
        DaylightCProperty, EventCProperty, FreeBusyCProperty, StandardCProperty, TodoCProperty,
    },
    value_types::{Date, DateTime, PeriodOfTime, Text},
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

    type CompositeValueType = Any2<DateTime, Date>;
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

    type CompositeValueType = Text;
}

impl EventCProperty for TimeTransparency {}

/// Recurrence Date-Times
///
/// Property Name:  RDATE
///
/// Purpose:  This property defines the list of DATE-TIME values for recurring
/// events, to-dos, journal entries, or time zone definitions.
///
/// Value Type:  The default value type for this property is DATE-TIME. The
/// value type can be set to DATE or PERIOD.
///
/// Property Parameters:  IANA, non-standard, value data type, and time zone
/// identifier property parameters can be specified on this property.
///
/// Conformance:  This property can be specified in recurring "VEVENT",
/// "VTODO", and "VJOURNAL" calendar components as well as in the "STANDARD"
/// and "DAYLIGHT" sub-components of the "VTIMEZONE" calendar component.
///
/// Description:  This property can appear along with the "RRULE" property to
/// define an aggregate set of repeating occurrences. When they both appear in
/// a recurring component, the recurrence instances are defined by the union
/// of occurrences defined by both the "RDATE" and "RRULE".
///
/// The recurrence dates, if specified, are used in computing the recurrence
/// set.  The recurrence set is the complete set of recurrence instances for a
/// calendar component.  The recurrence set is generated by considering the
/// initial "DTSTART" property along with the "RRULE", "RDATE", and "EXDATE"
/// properties contained within the recurring component.  The "DTSTART"
/// property defines the first instance in the recurrence set.  The "DTSTART"
/// property value SHOULD match the pattern of the recurrence rule, if
/// specified.  The recurrence set generated with a "DTSTART" property value
/// that doesn't match the pattern of the rule is undefined. The final
/// recurrence set is generated by gathering all of the start DATE-TIME values
/// generated by any of the specified "RRULE" and "RDATE" properties, and then
/// excluding any start DATE-TIME values specified by "EXDATE" properties.
/// This implies that start DATE-TIME values specified by "EXDATE" properties
/// take precedence over those specified by inclusion properties (i.e.,
/// "RDATE" and "RRULE").  Where duplicate instances are generated by the
/// "RRULE" and "RDATE" properties, only one recurrence is considered.
/// Duplicate instances are ignored.
///
/// Format Definition:  This property is defined by the following notation:
///
/// ```abnf
/// rdate      = "RDATE" rdtparam ":" rdtval *("," rdtval) CRLF
///
/// rdtparam   = *(
///            ;
///            ; The following are OPTIONAL,
///            ; but MUST NOT occur more than once.
///            ;
///            (";" "VALUE" "=" ("DATE-TIME" / "DATE" / "PERIOD")) /
///            (";" tzidparam) /
///            ;
///            ; The following is OPTIONAL,
///            ; and MAY occur more than once.
///            ;
///            (";" other-param)
///            ;
///            )
///
/// rdtval     = date-time / date / period
/// ;Value MUST match value type
/// ```
///
/// Example:  The following are examples of this property:
///
/// ```ics
/// RDATE:19970714T123000Z
/// RDATE;TZID=America/New_York:19970714T083000
/// ```
///
/// ```ics
/// RDATE;VALUE=PERIOD:19960403T020000Z/19960403T040000Z,
///  19960404T010000Z/PT3H
/// ```
///
/// ```ics
/// RDATE;VALUE=DATE:19970101,19970120,19970217,19970421
///  19970526,19970704,19970901,19971014,19971128,19971129,19971225
/// ```
///
/// Reference: [RFC 5545
/// 3.8.5.2](https://www.rfc-editor.org/rfc/rfc5545#section-3.8.5.2)
pub struct RecurrenceDateTimes;

impl Property for RecurrenceDateTimes {
    const NAME: &'static str = "RDATE";

    type CompositeValueType = Any3<List<DateTime>, List<Date>, List<PeriodOfTime>>;
}

impl EventCProperty for RecurrenceDateTimes {}
