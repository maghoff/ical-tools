//! [RFC 5545 3.8.7: Change Management Component Properties](https://www.rfc-editor.org/rfc/rfc5545#section-3.8.7)

use crate::structure::{
    icalstream::components::{EventCProperty, FreeBusyCProperty, JournalCProperty, TodoCProperty},
    value_types::DateTimeUtc,
    Property,
};

/// Date-Time Stamp
///
/// Property Name:  DTSTAMP
///
/// Purpose:  In the case of an iCalendar object that specifies a "METHOD"
/// property, this property specifies the date and time that the instance of
/// the iCalendar object was created.  In the case of an iCalendar object that
/// doesn't specify a "METHOD" property, this property specifies the date and
/// time that the information associated with the calendar component was last
/// revised in the calendar store.
///
/// Value Type:  DATE-TIME
///
/// Property Parameters:  IANA and non-standard property parameters can be
/// specified on this property.
///
/// Conformance:  This property MUST be included in the "VEVENT", "VTODO",
/// "VJOURNAL", or "VFREEBUSY" calendar components.
///
/// Description:  The value MUST be specified in the UTC time format.
///
/// This property is also useful to protocols such as [2447bis] that have
/// inherent latency issues with the delivery of content.  This property will
/// assist in the proper sequencing of messages containing iCalendar objects.
///
/// In the case of an iCalendar object that specifies a "METHOD" property,
/// this property differs from the "CREATED" and "LAST- MODIFIED" properties.
/// These two properties are used to specify when the particular calendar data
/// in the calendar store was created and last modified.  This is different
/// than when the iCalendar object representation of the calendar service
/// information was created or last modified.
///
/// In the case of an iCalendar object that doesn't specify a "METHOD"
/// property, this property is equivalent to the "LAST-MODIFIED" property.
///
/// Format Definition:  This property is defined by the following notation:
///
/// ```abnf
/// dtstamp    = "DTSTAMP" stmparam ":" date-time CRLF
///
/// stmparam   = *(";" other-param)
/// ```
///
/// Example:
///
/// ```text
/// DTSTAMP:19971210T080000Z
/// ```
///
/// Reference: [RFC 5545
/// 3.8.7.2](https://www.rfc-editor.org/rfc/rfc5545#section-3.8.7.2)
pub struct DateTimeStamp;

impl Property for DateTimeStamp {
    const NAME: &'static str = "DTSTAMP";

    type CompositeValueType = DateTimeUtc;
}

impl EventCProperty for DateTimeStamp {}
impl TodoCProperty for DateTimeStamp {}
impl JournalCProperty for DateTimeStamp {}
impl FreeBusyCProperty for DateTimeStamp {}
