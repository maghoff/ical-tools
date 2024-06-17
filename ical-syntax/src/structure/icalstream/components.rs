//! Calendar components, as defined in [RFC5545 3.6](https://www.rfc-editor.org/rfc/rfc5545.html#section-3.6)

use crate::structure::Component;

/// iCalendar Object
///
/// The Calendaring and Scheduling Core Object is a collection of
/// calendaring and scheduling information.  Typically, this information
/// will consist of an iCalendar stream with a single iCalendar object.
/// However, multiple iCalendar objects can be sequentially grouped
/// together in an iCalendar stream.  The first line and last line of the
/// iCalendar object MUST contain a pair of iCalendar object delimiter
/// strings.  The syntax for an iCalendar stream is as follows:
///
/// ```abnf
/// icalstream = 1*icalobject
///
/// icalobject = "BEGIN" ":" "VCALENDAR" CRLF
///              icalbody
///              "END" ":" "VCALENDAR" CRLF
/// ```
///
/// The following is a simple example of an iCalendar object:
///
/// ```ics
/// BEGIN:VCALENDAR
/// VERSION:2.0
/// PRODID:-//hacksw/handcal//NONSGML v1.0//EN
/// BEGIN:VEVENT
/// UID:19970610T172345Z-AF23B2@example.com
/// DTSTAMP:19970610T172345Z
/// DTSTART:19970714T170000Z
/// DTEND:19970715T040000Z
/// SUMMARY:Bastille Day Party
/// END:VEVENT
/// END:VCALENDAR
/// ```
///
/// Reference: [RFC5545 3.4](https://www.rfc-editor.org/rfc/rfc5545.html#section-3.4)
///
/// The body of the iCalendar object consists of a sequence of calendar
/// properties and one or more calendar components.  The calendar
/// properties are attributes that apply to the calendar object as a
/// whole.  The calendar components are collections of properties that
/// express a particular calendar semantic.  For example, the calendar
/// component can specify an event, a to-do, a journal entry, time zone
/// information, free/busy time information, or an alarm.
///
/// The body of the iCalendar object is defined by the following
/// notation:
///
/// ```abnf
/// icalbody   = calprops component
///
/// calprops   = *(
///            ;
///            ; The following are REQUIRED,
///            ; but MUST NOT occur more than once.
///            ;
///            prodid / version /
///            ;
///            ; The following are OPTIONAL,
///            ; but MUST NOT occur more than once.
///            ;
///            calscale / method /
///            ;
///            ; The following are OPTIONAL,
///            ; and MAY occur more than once.
///            ;
///            x-prop / iana-prop
///            ;
///            )
///
/// component  = 1*(eventc / todoc / journalc / freebusyc /
///              timezonec / iana-comp / x-comp)
///
/// iana-comp  = "BEGIN" ":" iana-token CRLF
///              1*contentline
///              "END" ":" iana-token CRLF
///
/// x-comp     = "BEGIN" ":" x-name CRLF
///              1*contentline
///              "END" ":" x-name CRLF
/// ```
///
/// An iCalendar object MUST include the "PRODID" and "VERSION" calendar
/// properties.  In addition, it MUST include at least one calendar
/// component.  Special forms of iCalendar objects are possible to
/// publish just busy time (i.e., only a "VFREEBUSY" calendar component)
/// or time zone (i.e., only a "VTIMEZONE" calendar component)
/// information.  In addition, a complex iCalendar object that is used to
/// capture a complete snapshot of the contents of a calendar is possible
/// (e.g., composite of many different calendar components).  More
/// commonly, an iCalendar object will consist of just a single "VEVENT",
/// "VTODO", or "VJOURNAL" calendar component.  Applications MUST ignore
/// x-comp and iana-comp values they don't recognize.  Applications that
/// support importing iCalendar objects SHOULD support all of the
/// component types defined in this document, and SHOULD NOT silently
/// drop any components as that can lead to user data loss.
///
/// Reference: [RFC5545 3.6](https://www.rfc-editor.org/rfc/rfc5545.html#section-3.6)
pub struct ICalObject;
impl Component for ICalObject {
    const NAME: &'static str = "VCALENDAR";
}

/// Event Component
///
/// Component Name:  VEVENT
///
/// Purpose:  Provide a grouping of component properties that describe an event.
///
/// Format Definition:  A "VEVENT" calendar component is defined by the
/// following notation:
///
/// ```abnf
/// eventc     = "BEGIN" ":" "VEVENT" CRLF
///              eventprop *alarmc
///              "END" ":" "VEVENT" CRLF
///
/// eventprop  = *(
///            ;
///            ; The following are REQUIRED,
///            ; but MUST NOT occur more than once.
///            ;
///            dtstamp / uid /
///            ;
///            ; The following is REQUIRED if the component
///            ; appears in an iCalendar object that doesn't
///            ; specify the "METHOD" property; otherwise, it
///            ; is OPTIONAL; in any case, it MUST NOT occur
///            ; more than once.
///            ;
///            dtstart /
///            ;
///            ; The following are OPTIONAL,
///            ; but MUST NOT occur more than once.
///            ;
///            class / created / description / geo /
///            last-mod / location / organizer / priority /
///            seq / status / summary / transp /
///            url / recurid /
///            ;
///            ; The following is OPTIONAL,
///            ; but SHOULD NOT occur more than once.
///            ;
///            rrule /
///            ;
///            ; Either 'dtend' or 'duration' MAY appear in
///            ; a 'eventprop', but 'dtend' and 'duration'
///            ; MUST NOT occur in the same 'eventprop'.
///            ;
///            dtend / duration /
///            ;
///            ; The following are OPTIONAL,
///            ; and MAY occur more than once.
///            ;
///            attach / attendee / categories / comment /
///            contact / exdate / rstatus / related /
///            resources / rdate / x-prop / iana-prop
///            ;
///            )
/// ```
///
/// Description:  A "VEVENT" calendar component is a grouping of component
/// properties, possibly including "VALARM" calendar components, that
/// represents a scheduled amount of time on a calendar.  For example, it can
/// be an activity; such as a one-hour long, department meeting from 8:00 AM
/// to 9:00 AM, tomorrow. Generally, an event will take up time on an
/// individual calendar. Hence, the event will appear as an opaque interval
/// in a search for busy time.  Alternately, the event can have its Time
/// Transparency set to "TRANSPARENT" in order to prevent blocking of the
/// event in searches for busy time.
///
/// The "VEVENT" is also the calendar component used to specify an
/// anniversary or daily reminder within a calendar.  These events have a
/// DATE value type for the "DTSTART" property instead of the default value
/// type of DATE-TIME.  If such a "VEVENT" has a "DTEND" property, it MUST be
/// specified as a DATE value also.  The anniversary type of "VEVENT" can
/// span more than one date (i.e., "DTEND" property value is set to a
/// calendar date after the "DTSTART" property value).  If such a "VEVENT"
/// has a "DURATION" property, it MUST be specified as a "dur-day" or
/// "dur-week" value.
///
/// The "DTSTART" property for a "VEVENT" specifies the inclusive start of
/// the event.  For recurring events, it also specifies the very first
/// instance in the recurrence set.  The "DTEND" property for a "VEVENT"
/// calendar component specifies the non-inclusive end of the event.  For
/// cases where a "VEVENT" calendar component specifies a "DTSTART" property
/// with a DATE value type but no "DTEND" nor "DURATION" property, the
/// event's duration is taken to be one day.  For cases where a "VEVENT"
/// calendar component specifies a "DTSTART" property with a DATE-TIME value
/// type but no "DTEND" property, the event ends on the same calendar date
/// and time of day specified by the "DTSTART" property.
///
/// The "VEVENT" calendar component cannot be nested within another calendar
/// component.  However, "VEVENT" calendar components can be related to each
/// other or to a "VTODO" or to a "VJOURNAL" calendar component with the
/// "RELATED-TO" property.
///
/// Example:  The following is an example of the "VEVENT" calendar component
/// used to represent a meeting that will also be opaque to searches for busy
/// time:
///
/// ```ics
/// BEGIN:VEVENT
/// UID:19970901T130000Z-123401@example.com
/// DTSTAMP:19970901T130000Z
/// DTSTART:19970903T163000Z
/// DTEND:19970903T190000Z
/// SUMMARY:Annual Employee Review
/// CLASS:PRIVATE
/// CATEGORIES:BUSINESS,HUMAN RESOURCES
/// END:VEVENT
/// ```
///
/// The following is an example of the "VEVENT" calendar component used to
/// represent a reminder that will not be opaque, but rather transparent, to
/// searches for busy time:
///
/// ```ics
/// BEGIN:VEVENT
/// UID:19970901T130000Z-123402@example.com
/// DTSTAMP:19970901T130000Z
/// DTSTART:19970401T163000Z
/// DTEND:19970402T010000Z
/// SUMMARY:Laurel is in sensitivity awareness class.
/// CLASS:PUBLIC
/// CATEGORIES:BUSINESS,HUMAN RESOURCES
/// TRANSP:TRANSPARENT
/// END:VEVENT
/// ```
///
/// The following is an example of the "VEVENT" calendar component used to
/// represent an anniversary that will occur annually:
///
/// ```ics
/// BEGIN:VEVENT
/// UID:19970901T130000Z-123403@example.com
/// DTSTAMP:19970901T130000Z
/// DTSTART;VALUE=DATE:19971102
/// SUMMARY:Our Blissful Anniversary
/// TRANSP:TRANSPARENT
/// CLASS:CONFIDENTIAL
/// CATEGORIES:ANNIVERSARY,PERSONAL,SPECIAL OCCASION
/// RRULE:FREQ=YEARLY
/// END:VEVENT
/// ```
///
/// The following is an example of the "VEVENT" calendar component used to
/// represent a multi-day event scheduled from June 28th, 2007 to July 8th,
/// 2007 inclusively.  Note that the "DTEND" property is set to July 9th,
/// 2007, since the "DTEND" property specifies the non-inclusive end of the
/// event.
///
/// ```ics
/// BEGIN:VEVENT
/// UID:20070423T123432Z-541111@example.com
/// DTSTAMP:20070423T123432Z
/// DTSTART;VALUE=DATE:20070628
/// DTEND;VALUE=DATE:20070709
/// SUMMARY:Festival International de Jazz de Montreal
/// TRANSP:TRANSPARENT
/// END:VEVENT
/// ```
///
/// Reference: [RFC5545 3.6.1](https://www.rfc-editor.org/rfc/rfc5545.html#section-3.6.1)
pub struct EventC;
impl Component for EventC {
    const NAME: &'static str = "VEVENT";
}

/// To-Do Component
///
/// Component Name:  VTODO
///
/// Purpose:  Provide a grouping of calendar properties that describe a to-do.
///
/// Format Definition:  A "VTODO" calendar component is defined by the
/// following notation:
///
/// ```abnf
/// todoc      = "BEGIN" ":" "VTODO" CRLF
///              todoprop *alarmc
///              "END" ":" "VTODO" CRLF
///
/// todoprop   = *(
///            ;
///            ; The following are REQUIRED,
///            ; but MUST NOT occur more than once.
///            ;
///            dtstamp / uid /
///            ;
///            ; The following are OPTIONAL,
///            ; but MUST NOT occur more than once.
///            ;
///            class / completed / created / description /
///            dtstart / geo / last-mod / location / organizer /
///            percent / priority / recurid / seq / status /
///            summary / url /
///            ;
///            ; The following is OPTIONAL,
///            ; but SHOULD NOT occur more than once.
///            ;
///            rrule /
///            ;
///            ; Either 'due' or 'duration' MAY appear in
///            ; a 'todoprop', but 'due' and 'duration'
///            ; MUST NOT occur in the same 'todoprop'.
///            ; If 'duration' appear in a 'todoprop',
///            ; then 'dtstart' MUST also appear in
///            ; the same 'todoprop'.
///            ;
///            due / duration /
///            ;
///            ; The following are OPTIONAL,
///            ; and MAY occur more than once.
///            ;
///            attach / attendee / categories / comment / contact /
///            exdate / rstatus / related / resources /
///            rdate / x-prop / iana-prop
///            ;
///            )
/// ```
///
/// Description:  A "VTODO" calendar component is a grouping of component
/// properties and possibly "VALARM" calendar components that represent an
/// action-item or assignment.  For example, it can be used to represent an
/// item of work assigned to an individual; such as "turn in travel expense
/// today".
///
/// The "VTODO" calendar component cannot be nested within another calendar
/// component.  However, "VTODO" calendar components can be related to each
/// other or to a "VEVENT" or to a "VJOURNAL" calendar component with the
/// "RELATED-TO" property.
///
/// A "VTODO" calendar component without the "DTSTART" and "DUE" (or
/// "DURATION") properties specifies a to-do that will be associated with each
/// successive calendar date, until it is completed.
///
/// Examples:  The following is an example of a "VTODO" calendar component
/// that needs to be completed before May 1st, 2007.  On midnight May 1st,
/// 2007 this to-do would be considered overdue.
///
/// ```ics
/// BEGIN:VTODO
/// UID:20070313T123432Z-456553@example.com
/// DTSTAMP:20070313T123432Z
/// DUE;VALUE=DATE:20070501
/// SUMMARY:Submit Quebec Income Tax Return for 2006
/// CLASS:CONFIDENTIAL
/// CATEGORIES:FAMILY,FINANCE
/// STATUS:NEEDS-ACTION
/// END:VTODO
/// ```
///
/// The following is an example of a "VTODO" calendar component that was due
/// before 1:00 P.M. UTC on July 9th, 2007 and was completed on July 7th, 2007
/// at 10:00 A.M. UTC.
///
/// ```ics
/// BEGIN:VTODO
/// UID:20070514T103211Z-123404@example.com
/// DTSTAMP:20070514T103211Z
/// DTSTART:20070514T110000Z
/// DUE:20070709T130000Z
/// COMPLETED:20070707T100000Z
/// SUMMARY:Submit Revised Internet-Draft
/// PRIORITY:1
/// STATUS:NEEDS-ACTION
/// END:VTODO
/// ```
///
/// Reference: [RFC5545 3.6.2](https://www.rfc-editor.org/rfc/rfc5545.html#section-3.6.2)
pub struct TodoC;
impl Component for TodoC {
    const NAME: &'static str = "VTODO";
}

/// Journal Component
///
/// Component Name:  VJOURNAL
///
/// Purpose:  Provide a grouping of component properties that describe a
/// journal entry.
///
/// Format Definition:  A "VJOURNAL" calendar component is defined by the
/// following notation:
///
/// ```abnf
/// journalc   = "BEGIN" ":" "VJOURNAL" CRLF
///              jourprop
///              "END" ":" "VJOURNAL" CRLF
///
/// jourprop   = *(
///            ;
///            ; The following are REQUIRED,
///            ; but MUST NOT occur more than once.
///            ;
///            dtstamp / uid /
///            ;
///            ; The following are OPTIONAL,
///            ; but MUST NOT occur more than once.
///            ;
///            class / created / dtstart /
///            last-mod / organizer / recurid / seq /
///            status / summary / url /
///            ;
///            ; The following is OPTIONAL,
///            ; but SHOULD NOT occur more than once.
///            ;
///            rrule /
///            ;
///            ; The following are OPTIONAL,
///            ; and MAY occur more than once.
///            ;
///            attach / attendee / categories / comment /
///            contact / description / exdate / related / rdate /
///            rstatus / x-prop / iana-prop
///            ;
///            )
/// ```
///
/// Description:  A "VJOURNAL" calendar component is a grouping of component
/// properties that represent one or more descriptive text notes associated
/// with a particular calendar date.  The "DTSTART" property is used to
/// specify the calendar date with which the journal entry is associated.
/// Generally, it will have a DATE value data type, but it can also be used to
/// specify a DATE-TIME value data type.  Examples of a journal entry include
/// a daily record of a legislative body or a journal entry of individual
/// telephone contacts for the day or an ordered list of accomplishments for
/// the day.  The "VJOURNAL" calendar component can also be used to associate
/// a document with a calendar date.
///
/// The "VJOURNAL" calendar component does not take up time on a calendar.
/// Hence, it does not play a role in free or busy time searches -- it is as
/// though it has a time transparency value of TRANSPARENT.  It is transparent
/// to any such searches.
///
/// The "VJOURNAL" calendar component cannot be nested within another calendar
/// component.  However, "VJOURNAL" calendar components can be related to each
/// other or to a "VEVENT" or to a "VTODO" calendar component, with the
/// "RELATED-TO" property.
///
/// Example:  The following is an example of the "VJOURNAL" calendar
/// component:
///
/// ```ics
/// BEGIN:VJOURNAL
/// UID:19970901T130000Z-123405@example.com
/// DTSTAMP:19970901T130000Z
/// DTSTART;VALUE=DATE:19970317
/// SUMMARY:Staff meeting minutes
/// DESCRIPTION:1. Staff meeting: Participants include Joe\,
///   Lisa\, and Bob. Aurora project plans were reviewed.
///   There is currently no budget reserves for this project.
///   Lisa will escalate to management. Next meeting on Tuesday.\n
///  2. Telephone Conference: ABC Corp. sales representative
///   called to discuss new printer. Promised to get us a demo by
///   Friday.\n3. Henry Miller (Handsoff Insurance): Car was
///   totaled by tree. Is looking into a loaner car. 555-2323
///   (tel).
/// END:VJOURNAL
/// ```
///
/// Reference: [RFC5545 3.6.3](https://www.rfc-editor.org/rfc/rfc5545.html#section-3.6.3)
pub struct JournalC;
impl Component for JournalC {
    const NAME: &'static str = "VJOURNAL";
}

/// Free/Busy Component
///
/// Component Name:  VFREEBUSY
///
/// Purpose:  Provide a grouping of component properties that describe either
/// a request for free/busy time, describe a response to a request for
/// free/busy time, or describe a published set of busy time.
///
/// Format Definition:  A "VFREEBUSY" calendar component is defined by the
/// following notation:
///
/// ```abnf
/// freebusyc  = "BEGIN" ":" "VFREEBUSY" CRLF
///              fbprop
///              "END" ":" "VFREEBUSY" CRLF
///
/// fbprop     = *(
///            ;
///            ; The following are REQUIRED,
///            ; but MUST NOT occur more than once.
///            ;
///            dtstamp / uid /
///            ;
///            ; The following are OPTIONAL,
///            ; but MUST NOT occur more than once.
///            ;
///            contact / dtstart / dtend /
///            organizer / url /
///            ;
///            ; The following are OPTIONAL,
///            ; and MAY occur more than once.
///            ;
///            attendee / comment / freebusy / rstatus / x-prop /
///            iana-prop
///            ;
///            )
/// ```
///
/// Description:  A "VFREEBUSY" calendar component is a grouping of component
/// properties that represents either a request for free or busy time
/// information, a reply to a request for free or busy time information, or a
/// published set of busy time information.
///
/// When used to request free/busy time information, the "ATTENDEE" property
/// specifies the calendar users whose free/busy time is being requested; the
/// "ORGANIZER" property specifies the calendar user who is requesting the
/// free/busy time; the "DTSTART" and "DTEND" properties specify the window of
/// time for which the free/ busy time is being requested; the "UID" and
/// "DTSTAMP" properties are specified to assist in proper sequencing of
/// multiple free/busy time requests.
///
/// When used to reply to a request for free/busy time, the "ATTENDEE"
/// property specifies the calendar user responding to the free/busy time
/// request; the "ORGANIZER" property specifies the calendar user that
/// originally requested the free/busy time; the "FREEBUSY" property specifies
/// the free/busy time information (if it exists); and the "UID" and "DTSTAMP"
/// properties are specified to assist in proper sequencing of multiple
/// free/busy time replies.
///
/// When used to publish busy time, the "ORGANIZER" property specifies the
/// calendar user associated with the published busy time; the "DTSTART" and
/// "DTEND" properties specify an inclusive time window that surrounds the
/// busy time information; the "FREEBUSY" property specifies the published
/// busy time information; and the "DTSTAMP" property specifies the DATE-TIME
/// that iCalendar object was created.
///
/// The "VFREEBUSY" calendar component cannot be nested within another
/// calendar component.  Multiple "VFREEBUSY" calendar components can be
/// specified within an iCalendar object.  This permits the grouping of
/// free/busy information into logical collections, such as monthly groups of
/// busy time information.
///
/// The "VFREEBUSY" calendar component is intended for use in iCalendar object
/// methods involving requests for free time, requests for busy time, requests
/// for both free and busy, and the associated replies.
///
/// Free/Busy information is represented with the "FREEBUSY" property. This
/// property provides a terse representation of time periods. One or more
/// "FREEBUSY" properties can be specified in the "VFREEBUSY" calendar
/// component.
///
/// When present in a "VFREEBUSY" calendar component, the "DTSTART" and
/// "DTEND" properties SHOULD be specified prior to any "FREEBUSY" properties.
///
/// The recurrence properties ("RRULE", "RDATE", "EXDATE") are not permitted
/// within a "VFREEBUSY" calendar component.  Any recurring events are
/// resolved into their individual busy time periods using the "FREEBUSY"
/// property.
///
/// Example:  The following is an example of a "VFREEBUSY" calendar component
/// used to request free or busy time information:
///
/// ```ics
/// BEGIN:VFREEBUSY
/// UID:19970901T082949Z-FA43EF@example.com
/// ORGANIZER:mailto:jane_doe@example.com
/// ATTENDEE:mailto:john_public@example.com
/// DTSTART:19971015T050000Z
/// DTEND:19971016T050000Z
/// DTSTAMP:19970901T083000Z
/// END:VFREEBUSY
/// ```
///
/// The following is an example of a "VFREEBUSY" calendar component used to
/// reply to the request with busy time information:
///
/// ```ics
/// BEGIN:VFREEBUSY
/// UID:19970901T095957Z-76A912@example.com
/// ORGANIZER:mailto:jane_doe@example.com
/// ATTENDEE:mailto:john_public@example.com
/// DTSTAMP:19970901T100000Z
/// FREEBUSY:19971015T050000Z/PT8H30M,
///  19971015T160000Z/PT5H30M,19971015T223000Z/PT6H30M
/// URL:http://example.com/pub/busy/jpublic-01.ifb
/// COMMENT:This iCalendar file contains busy time information for
///   the next three months.
/// END:VFREEBUSY
/// ```
///
/// The following is an example of a "VFREEBUSY" calendar component used to
/// publish busy time information:
///
/// ```ics
/// BEGIN:VFREEBUSY
/// UID:19970901T115957Z-76A912@example.com
/// DTSTAMP:19970901T120000Z
/// ORGANIZER:jsmith@example.com
/// DTSTART:19980313T141711Z
/// DTEND:19980410T141711Z
/// FREEBUSY:19980314T233000Z/19980315T003000Z
/// FREEBUSY:19980316T153000Z/19980316T163000Z
/// FREEBUSY:19980318T030000Z/19980318T040000Z
/// URL:http://www.example.com/calendar/busytime/jsmith.ifb
/// END:VFREEBUSY
/// ```
///
/// Reference: [RFC5545 3.6.4](https://www.rfc-editor.org/rfc/rfc5545.html#section-3.6.4)
pub struct FreeBusyC;
impl Component for FreeBusyC {
    const NAME: &'static str = "VFREEBUSY";
}

/// Time Zone Component
///
/// Component Name:  VTIMEZONE
///
/// Purpose:  Provide a grouping of component properties that defines a time
/// zone.
///
/// Format Definition:  A "VTIMEZONE" calendar component is defined by the
/// following notation:
///
/// ```abnf
/// timezonec  = "BEGIN" ":" "VTIMEZONE" CRLF
///              *(
///              ;
///              ; 'tzid' is REQUIRED, but MUST NOT occur more
///              ; than once.
///              ;
///              tzid /
///              ;
///              ; 'last-mod' and 'tzurl' are OPTIONAL,
///              ; but MUST NOT occur more than once.
///              ;
///              last-mod / tzurl /
///              ;
///              ; One of 'standardc' or 'daylightc' MUST occur
///              ; and each MAY occur more than once.
///              ;
///              standardc / daylightc /
///              ;
///              ; The following are OPTIONAL,
///              ; and MAY occur more than once.
///              ;
///              x-prop / iana-prop
///              ;
///              )
///              "END" ":" "VTIMEZONE" CRLF
///
/// standardc  = "BEGIN" ":" "STANDARD" CRLF
///              tzprop
///              "END" ":" "STANDARD" CRLF
///
/// daylightc  = "BEGIN" ":" "DAYLIGHT" CRLF
///              tzprop
///              "END" ":" "DAYLIGHT" CRLF
///
/// tzprop     = *(
///              ;
///              ; The following are REQUIRED,
///              ; but MUST NOT occur more than once.
///              ;
///              dtstart / tzoffsetto / tzoffsetfrom /
///              ;
///              ; The following is OPTIONAL,
///              ; but SHOULD NOT occur more than once.
///              ;
///              rrule /
///              ;
///              ; The following are OPTIONAL,
///              ; and MAY occur more than once.
///              ;
///              comment / rdate / tzname / x-prop / iana-prop
///              ;
///              )
/// ```
///
/// Description:  A time zone is unambiguously defined by the set of time
/// measurement rules determined by the governing body for a given geographic
/// area.  These rules describe, at a minimum, the base offset from UTC for the
/// time zone, often referred to as the Standard Time offset.  Many locations
/// adjust their Standard Time forward or backward by one hour, in order to
/// accommodate seasonal changes in number of daylight hours, often referred to
/// as Daylight Saving Time.  Some locations adjust their time by a fraction of
/// an hour.  Standard Time is also known as Winter Time.  Daylight Saving Time
/// is also known as Advanced Time, Summer Time, or Legal Time in certain
/// countries.  The following table shows the changes in time zone rules in
/// effect for New York City starting from 1967. Each line represents a
/// description or rule for a particular observance.
///
/// ## Effective Observance Rule ##
///
/// | Date      | (Date-Time)              | Offset | Abbreviation |
/// |-----------|--------------------------|--------|--------------|
/// | 1967-1973 | last Sun in Apr, 02:00   | -0400  | EDT          |
/// | 1967-2006 | last Sun in Oct, 02:00   | -0500  | EST          |
/// | 1974-1974 | Jan 6, 02:00             | -0400  | EDT          |
/// | 1975-1975 | Feb 23, 02:00            | -0400  | EDT          |
/// | 1976-1986 | last Sun in Apr, 02:00   | -0400  | EDT          |
/// | 1987-2006 | first Sun in Apr, 02:00  | -0400  | EDT          |
/// | 2007-*    | second Sun in Mar, 02:00 | -0400  | EDT          |
/// | 2007-*    | first Sun in Nov, 02:00  | -0500  | EST          |
///
/// Note: The specification of a global time zone registry is not addressed by
/// this document and is left for future study. However, implementers may find
/// the TZ database TZDB a useful reference.  It is an informal, public-domain
/// collection of time zone information, which is currently being maintained by
/// volunteer Internet participants, and is used in several operating systems.
/// This database contains current and historical time zone information for a
/// wide variety of locations around the globe; it provides a time zone
/// identifier for every unique time zone rule set in actual use since 1970,
/// with historical data going back to the introduction of standard time.
///
/// Interoperability between two calendaring and scheduling applications,
/// especially for recurring events, to-dos or journal entries, is dependent on
/// the ability to capture and convey date and time information in an
/// unambiguous format.  The specification of current time zone information is
/// integral to this behavior.
///
/// If present, the "VTIMEZONE" calendar component defines the set of Standard
/// Time and Daylight Saving Time observances (or rules) for a particular time
/// zone for a given interval of time.  The "VTIMEZONE" calendar component
/// cannot be nested within other calendar components.  Multiple "VTIMEZONE"
/// calendar components can exist in an iCalendar object.  In this situation,
/// each "VTIMEZONE" MUST represent a unique time zone definition.  This is
/// necessary for some classes of events, such as airline flights, that start in
/// one time zone and end in another.
///
/// The "VTIMEZONE" calendar component MUST include the "TZID" property and at
/// least one definition of a "STANDARD" or "DAYLIGHT" sub-component.  The
/// "STANDARD" or "DAYLIGHT" sub-component MUST include the "DTSTART",
/// "TZOFFSETFROM", and "TZOFFSETTO" properties.
///
/// An individual "VTIMEZONE" calendar component MUST be specified for each
/// unique "TZID" parameter value specified in the iCalendar object.  In
/// addition, a "VTIMEZONE" calendar component, referred to by a recurring
/// calendar component, MUST provide valid time zone information for all
/// recurrence instances.
///
/// Each "VTIMEZONE" calendar component consists of a collection of one or more
/// sub-components that describe the rule for a particular observance (either a
/// Standard Time or a Daylight Saving Time observance).  The "STANDARD"
/// sub-component consists of a collection of properties that describe Standard
/// Time.  The "DAYLIGHT" sub-component consists of a collection of properties
/// that describe Daylight Saving Time.  In general, this collection of
/// properties consists of:
///
/// *  the first onset DATE-TIME for the observance;
///
/// *  the last onset DATE-TIME for the observance, if a last onset is known;
///
/// *  the offset to be applied for the observance;
///
/// *  a rule that describes the day and time when the observance takes effect;
///
/// *  an optional name for the observance.
///
/// For a given time zone, there may be multiple unique definitions of the
/// observances over a period of time.  Each observance is described using
/// either a "STANDARD" or "DAYLIGHT" sub-component. The collection of these
/// sub-components is used to describe the time zone for a given period of time.
/// The offset to apply at any given time is found by locating the observance
/// that has the last onset date and time before the time in question, and using
/// the offset value from that observance.
///
/// The top-level properties in a "VTIMEZONE" calendar component are:
///
/// The mandatory "TZID" property is a text value that uniquely identifies the
/// "VTIMEZONE" calendar component within the scope of an iCalendar object.
///
/// The optional "LAST-MODIFIED" property is a UTC value that specifies the date
/// and time that this time zone definition was last updated.
///
/// The optional "TZURL" property is a url value that points to a published
/// "VTIMEZONE" definition.  "TZURL" SHOULD refer to a resource that is
/// accessible by anyone who might need to interpret the object.  This SHOULD
/// NOT normally be a "file" URL or other URL that is not widely accessible.
///
/// The collection of properties that are used to define the "STANDARD" and
/// "DAYLIGHT" sub-components include:
///
/// The mandatory "DTSTART" property gives the effective onset date and local
/// time for the time zone sub-component definition. "DTSTART" in this usage
/// MUST be specified as a date with a local time value.
///
/// The mandatory "TZOFFSETFROM" property gives the UTC offset that is in use
/// when the onset of this time zone observance begins. "TZOFFSETFROM" is
/// combined with "DTSTART" to define the effective onset for the time zone
/// sub-component definition.  For example, the following represents the time at
/// which the observance of Standard Time took effect in Fall 1967 for New York
/// City:
///
/// ```ics
/// DTSTART:19671029T020000
///
/// TZOFFSETFROM:-0400
/// ```
///
/// The mandatory "TZOFFSETTO" property gives the UTC offset for the time zone
/// sub-component (Standard Time or Daylight Saving Time) when this observance
/// is in use.
///
/// The optional "TZNAME" property is the customary name for the time zone.
/// This could be used for displaying dates.
///
/// The onset DATE-TIME values for the observance defined by the time zone
/// sub-component is defined by the "DTSTART", "RRULE", and "RDATE" properties.
///
/// The "RRULE" property defines the recurrence rule for the onset of the
/// observance defined by this time zone sub-component.  Some specific
/// requirements for the usage of "RRULE" for this purpose include:
///
/// *  If observance is known to have an effective end date, the "UNTIL"
///     recurrence rule parameter MUST be used to specify the last valid onset
///     of this observance (i.e., the UNTIL DATE-TIME will be equal to the last
///     instance generated by the recurrence pattern).  It MUST be specified in
///     UTC time.
///
/// *  The "DTSTART" and the "TZOFFSETFROM" properties MUST be used when
///     generating the onset DATE-TIME values (instances) from the "RRULE".
///
/// The "RDATE" property can also be used to define the onset of the observance
/// by giving the individual onset date and times.  "RDATE" in this usage MUST
/// be specified as a date with local time value, relative to the UTC offset
/// specified in the "TZOFFSETFROM" property.
///
/// The optional "COMMENT" property is also allowed for descriptive explanatory
/// text.
///
/// Example:  The following are examples of the "VTIMEZONE" calendar component:
///
/// This is an example showing all the time zone rules for New York City since
/// April 30, 1967 at 03:00:00 EDT.
///
/// ```ics
/// BEGIN:VTIMEZONE
/// TZID:America/New_York
/// LAST-MODIFIED:20050809T050000Z
/// BEGIN:DAYLIGHT
/// DTSTART:19670430T020000
/// RRULE:FREQ=YEARLY;BYMONTH=4;BYDAY=-1SU;UNTIL=19730429T070000Z
/// TZOFFSETFROM:-0500
/// TZOFFSETTO:-0400
/// TZNAME:EDT
/// END:DAYLIGHT
/// BEGIN:STANDARD
/// DTSTART:19671029T020000
/// RRULE:FREQ=YEARLY;BYMONTH=10;BYDAY=-1SU;UNTIL=20061029T060000Z
/// TZOFFSETFROM:-0400
/// TZOFFSETTO:-0500
/// TZNAME:EST
/// END:STANDARD
/// BEGIN:DAYLIGHT
/// DTSTART:19740106T020000
/// RDATE:19750223T020000
/// TZOFFSETFROM:-0500
/// TZOFFSETTO:-0400
/// TZNAME:EDT
/// END:DAYLIGHT
/// BEGIN:DAYLIGHT
/// DTSTART:19760425T020000
/// RRULE:FREQ=YEARLY;BYMONTH=4;BYDAY=-1SU;UNTIL=19860427T070000Z
/// TZOFFSETFROM:-0500
/// TZOFFSETTO:-0400
/// TZNAME:EDT
/// END:DAYLIGHT
/// BEGIN:DAYLIGHT
/// DTSTART:19870405T020000
/// RRULE:FREQ=YEARLY;BYMONTH=4;BYDAY=1SU;UNTIL=20060402T070000Z
/// TZOFFSETFROM:-0500
/// TZOFFSETTO:-0400
/// TZNAME:EDT
/// END:DAYLIGHT
/// BEGIN:DAYLIGHT
/// DTSTART:20070311T020000
/// RRULE:FREQ=YEARLY;BYMONTH=3;BYDAY=2SU
/// TZOFFSETFROM:-0500
/// TZOFFSETTO:-0400
/// TZNAME:EDT
/// END:DAYLIGHT
/// BEGIN:STANDARD
/// DTSTART:20071104T020000
/// RRULE:FREQ=YEARLY;BYMONTH=11;BYDAY=1SU
/// TZOFFSETFROM:-0400
/// TZOFFSETTO:-0500
/// TZNAME:EST
/// END:STANDARD
/// END:VTIMEZONE
/// ```
///
/// This is an example showing time zone information for New York City using
/// only the "DTSTART" property.  Note that this is only suitable for a
/// recurring event that starts on or later than March 11, 2007 at 03:00:00 EDT
/// (i.e., the earliest effective transition date and time) and ends no later
/// than March 9, 2008 at 01:59:59 EST (i.e., latest valid date and time for EST
/// in this scenario). For example, this can be used for a recurring event that
/// occurs every Friday, 8:00 A.M.-9:00 A.M., starting June 1, 2007, ending
/// December 31, 2007,
///
/// ```ics
/// BEGIN:VTIMEZONE
/// TZID:America/New_York
/// LAST-MODIFIED:20050809T050000Z
/// BEGIN:STANDARD
/// DTSTART:20071104T020000
/// TZOFFSETFROM:-0400
/// TZOFFSETTO:-0500
/// TZNAME:EST
/// END:STANDARD
/// BEGIN:DAYLIGHT
/// DTSTART:20070311T020000
/// TZOFFSETFROM:-0500
/// TZOFFSETTO:-0400
/// TZNAME:EDT
/// END:DAYLIGHT
/// END:VTIMEZONE
/// ```
///
/// This is a simple example showing the current time zone rules for New York
/// City using a "RRULE" recurrence pattern.  Note that there is no effective
/// end date to either of the Standard Time or Daylight Time rules.  This
/// information would be valid for a recurring event starting today and
/// continuing indefinitely.
///
/// ```ics
/// BEGIN:VTIMEZONE
/// TZID:America/New_York
/// LAST-MODIFIED:20050809T050000Z
/// TZURL:http://zones.example.com/tz/America-New_York.ics
/// BEGIN:STANDARD
/// DTSTART:20071104T020000
/// RRULE:FREQ=YEARLY;BYMONTH=11;BYDAY=1SU
/// TZOFFSETFROM:-0400
/// TZOFFSETTO:-0500
/// TZNAME:EST
/// END:STANDARD
/// BEGIN:DAYLIGHT
/// DTSTART:20070311T020000
/// RRULE:FREQ=YEARLY;BYMONTH=3;BYDAY=2SU
/// TZOFFSETFROM:-0500
/// TZOFFSETTO:-0400
/// TZNAME:EDT
/// END:DAYLIGHT
/// END:VTIMEZONE
/// ```
///
/// This is an example showing a set of rules for a fictitious time zone where
/// the Daylight Time rule has an effective end date (i.e., after that date,
/// Daylight Time is no longer observed).
///
/// ```ics
/// BEGIN:VTIMEZONE
/// TZID:Fictitious
/// LAST-MODIFIED:19870101T000000Z
/// BEGIN:STANDARD
/// DTSTART:19671029T020000
/// RRULE:FREQ=YEARLY;BYDAY=-1SU;BYMONTH=10
/// TZOFFSETFROM:-0400
/// TZOFFSETTO:-0500
/// TZNAME:EST
/// END:STANDARD
/// BEGIN:DAYLIGHT
/// DTSTART:19870405T020000
/// RRULE:FREQ=YEARLY;BYDAY=1SU;BYMONTH=4;UNTIL=19980404T070000Z
/// TZOFFSETFROM:-0500
/// TZOFFSETTO:-0400
/// TZNAME:EDT
/// END:DAYLIGHT
/// END:VTIMEZONE
/// ```
///
/// This is an example showing a set of rules for a fictitious time zone where
/// the first Daylight Time rule has an effective end date. There is a second
/// Daylight Time rule that picks up where the other left off.
///
/// ```ics
/// BEGIN:VTIMEZONE
/// TZID:Fictitious
/// LAST-MODIFIED:19870101T000000Z
/// BEGIN:STANDARD
/// DTSTART:19671029T020000
/// RRULE:FREQ=YEARLY;BYDAY=-1SU;BYMONTH=10
/// TZOFFSETFROM:-0400
/// TZOFFSETTO:-0500
/// TZNAME:EST
/// END:STANDARD
/// BEGIN:DAYLIGHT
/// DTSTART:19870405T020000
/// RRULE:FREQ=YEARLY;BYDAY=1SU;BYMONTH=4;UNTIL=19980404T070000Z
/// TZOFFSETFROM:-0500
/// TZOFFSETTO:-0400
/// TZNAME:EDT
/// END:DAYLIGHT
/// BEGIN:DAYLIGHT
/// DTSTART:19990424T020000
/// RRULE:FREQ=YEARLY;BYDAY=-1SU;BYMONTH=4
/// TZOFFSETFROM:-0500
/// TZOFFSETTO:-0400
/// TZNAME:EDT
/// END:DAYLIGHT
/// END:VTIMEZONE
/// ```
///
/// Reference: [RFC5545 3.6.5](https://www.rfc-editor.org/rfc/rfc5545.html#section-3.6.5)
pub struct TimeZoneC;
impl Component for TimeZoneC {
    const NAME: &'static str = "VTIMEZONE";
}

/// A subcomponent of [TimeZoneC].
pub struct StandardC;
impl Component for StandardC {
    const NAME: &'static str = "STANDARD";
}

/// A subcomponent of [TimeZoneC].
pub struct DaylightC;
impl Component for DaylightC {
    const NAME: &'static str = "DAYLIGHT";
}

/// Alarm Component
///
/// Component Name:  VALARM
///
/// Purpose:  Provide a grouping of component properties that define an alarm.
///
/// Format Definition:  A "VALARM" calendar component is defined by the
/// following notation:
///
/// ```abnf
/// alarmc     = "BEGIN" ":" "VALARM" CRLF
///              (audioprop / dispprop / emailprop)
///              "END" ":" "VALARM" CRLF
///
/// audioprop  = *(
///            ;
///            ; 'action' and 'trigger' are both REQUIRED,
///            ; but MUST NOT occur more than once.
///            ;
///            action / trigger /
///            ;
///            ; 'duration' and 'repeat' are both OPTIONAL,
///            ; and MUST NOT occur more than once each;
///            ; but if one occurs, so MUST the other.
///            ;
///            duration / repeat /
///            ;
///            ; The following is OPTIONAL,
///            ; but MUST NOT occur more than once.
///            ;
///            attach /
///            ;
///            ; The following is OPTIONAL,
///            ; and MAY occur more than once.
///            ;
///            x-prop / iana-prop
///            ;
///            )
///
/// dispprop   = *(
///            ;
///            ; The following are REQUIRED,
///            ; but MUST NOT occur more than once.
///            ;
///            action / description / trigger /
///            ;
///            ; 'duration' and 'repeat' are both OPTIONAL,
///            ; and MUST NOT occur more than once each;
///            ; but if one occurs, so MUST the other.
///            ;
///            duration / repeat /
///            ;
///            ; The following is OPTIONAL,
///            ; and MAY occur more than once.
///            ;
///            x-prop / iana-prop
///            ;
///            )
///
/// emailprop  = *(
///            ;
///            ; The following are all REQUIRED,
///            ; but MUST NOT occur more than once.
///            ;
///            action / description / trigger / summary /
///            ;
///            ; The following is REQUIRED,
///            ; and MAY occur more than once.
///            ;
///            attendee /
///            ;
///            ; 'duration' and 'repeat' are both OPTIONAL,
///            ; and MUST NOT occur more than once each;
///            ; but if one occurs, so MUST the other.
///            ;
///            duration / repeat /
///            ;
///            ; The following are OPTIONAL,
///            ; and MAY occur more than once.
///            ;
///            attach / x-prop / iana-prop
///            ;
///            )
/// ```
///
/// Description:  A "VALARM" calendar component is a grouping of component
/// properties that is a reminder or alarm for an event or a to-do.  For
/// example, it may be used to define a reminder for a pending event or an
/// overdue to-do.
///
/// The "VALARM" calendar component MUST include the "ACTION" and "TRIGGER"
/// properties.  The "ACTION" property further constrains the "VALARM" calendar
/// component in the following ways:
///
/// When the action is "AUDIO", the alarm can also include one and only one
/// "ATTACH" property, which MUST point to a sound resource, which is rendered
/// when the alarm is triggered.
///
/// When the action is "DISPLAY", the alarm MUST also include a "DESCRIPTION"
/// property, which contains the text to be displayed when the alarm is
/// triggered.
///
/// When the action is "EMAIL", the alarm MUST include a "DESCRIPTION" property,
/// which contains the text to be used as the message body, a "SUMMARY"
/// property, which contains the text to be used as the message subject, and one
/// or more "ATTENDEE" properties, which contain the email address of attendees
/// to receive the message.  It can also include one or more "ATTACH"
/// properties, which are intended to be sent as message attachments.  When the
/// alarm is triggered, the email message is sent.
///
/// The "VALARM" calendar component MUST only appear within either a "VEVENT" or
/// "VTODO" calendar component.  "VALARM" calendar components cannot be nested.
/// Multiple mutually independent "VALARM" calendar components can be specified
/// for a single "VEVENT" or "VTODO" calendar component.
///
/// The "TRIGGER" property specifies when the alarm will be triggered. The
/// "TRIGGER" property specifies a duration prior to the start of an event or a
/// to-do.  The "TRIGGER" edge may be explicitly set to be relative to the
/// "START" or "END" of the event or to-do with the "RELATED" parameter of the
/// "TRIGGER" property.  The "TRIGGER" property value type can alternatively be
/// set to an absolute calendar date with UTC time.
///
/// In an alarm set to trigger on the "START" of an event or to-do, the
/// "DTSTART" property MUST be present in the associated event or to-do.  In an
/// alarm in a "VEVENT" calendar component set to trigger on the "END" of the
/// event, either the "DTEND" property MUST be present, or the "DTSTART" and
/// "DURATION" properties MUST both be present.  In an alarm in a "VTODO"
/// calendar component set to trigger on the "END" of the to-do, either the
/// "DUE" property MUST be present, or the "DTSTART" and "DURATION" properties
/// MUST both be present.
///
/// The alarm can be defined such that it triggers repeatedly.  A definition of
/// an alarm with a repeating trigger MUST include both the "DURATION" and
/// "REPEAT" properties.  The "DURATION" property specifies the delay period,
/// after which the alarm will repeat. The "REPEAT" property specifies the
/// number of additional repetitions that the alarm will be triggered.  This
/// repetition count is in addition to the initial triggering of the alarm.
/// Both of these properties MUST be present in order to specify a repeating
/// alarm.  If one of these two properties is absent, then the alarm will not
/// repeat beyond the initial trigger.
///
/// The "ACTION" property is used within the "VALARM" calendar component to
/// specify the type of action invoked when the alarm is triggered.  The
/// "VALARM" properties provide enough information for a specific action to be
/// invoked.  It is typically the responsibility of a "Calendar User Agent"
/// (CUA) to deliver the alarm in the specified fashion.  An "ACTION" property
/// value of AUDIO specifies an alarm that causes a sound to be played to alert
/// the user; DISPLAY specifies an alarm that causes a text message to be
/// displayed to the user; and EMAIL specifies an alarm that causes an
/// electronic email message to be delivered to one or more email addresses.
///
/// In an AUDIO alarm, if the optional "ATTACH" property is included, it MUST
/// specify an audio sound resource.  The intention is that the sound will be
/// played as the alarm effect.  If an "ATTACH" property is specified that does
/// not refer to a sound resource, or if the specified sound resource cannot be
/// rendered (because its format is unsupported, or because it cannot be
/// retrieved), then the CUA or other entity responsible for playing the sound
/// may choose a fallback action, such as playing a built-in default sound, or
/// playing no sound at all.
///
/// In a DISPLAY alarm, the intended alarm effect is for the text value of the
/// "DESCRIPTION" property to be displayed to the user.
///
/// In an EMAIL alarm, the intended alarm effect is for an email message to be
/// composed and delivered to all the addresses specified by the "ATTENDEE"
/// properties in the "VALARM" calendar component.  The "DESCRIPTION" property
/// of the "VALARM" calendar component MUST be used as the body text of the
/// message, and the "SUMMARY" property MUST be used as the subject text.  Any
/// "ATTACH" properties in the "VALARM" calendar component SHOULD be sent as
/// attachments to the message.
///
/// Note: Implementations should carefully consider whether they accept alarm
/// components from untrusted sources, e.g., when importing calendar objects
/// from external sources.  One reasonable policy is to always ignore alarm
/// components that the calendar user has not set herself, or at least ask for
/// confirmation in such a case.
///
/// Example:  The following example is for a "VALARM" calendar component that
/// specifies an audio alarm that will sound at a precise time and repeat 4 more
/// times at 15-minute intervals:
///
/// ```ics
/// BEGIN:VALARM
/// TRIGGER;VALUE=DATE-TIME:19970317T133000Z
/// REPEAT:4
/// DURATION:PT15M
/// ACTION:AUDIO
/// ATTACH;FMTTYPE=audio/basic:ftp://example.com/pub/
///  sounds/bell-01.aud
/// END:VALARM
/// ```
///
/// The following example is for a "VALARM" calendar component that specifies a
/// display alarm that will trigger 30 minutes before the scheduled start of the
/// event or of the to-do it is associated with and will repeat 2 more times at
/// 15-minute intervals:
///
/// ```ics
/// BEGIN:VALARM
/// TRIGGER:-PT30M
/// REPEAT:2
/// DURATION:PT15M
/// ACTION:DISPLAY
/// DESCRIPTION:Breakfast meeting with executive\n
///  team at 8:30 AM EST.
/// END:VALARM
/// ```
///
/// The following example is for a "VALARM" calendar component that specifies an
/// email alarm that will trigger 2 days before the scheduled due DATE-TIME of a
/// to-do with which it is associated. It does not repeat.  The email has a
/// subject, body, and attachment link.
///
/// ```ics
/// BEGIN:VALARM
/// TRIGGER;RELATED=END:-P2D
/// ACTION:EMAIL
/// ATTENDEE:mailto:john_doe@example.com
/// SUMMARY:*** REMINDER: SEND AGENDA FOR WEEKLY STAFF MEETING ***
/// DESCRIPTION:A draft agenda needs to be sent out to the attendees
///   to the weekly managers meeting (MGR-LIST). Attached is a
///   pointer the document template for the agenda file.
/// ATTACH;FMTTYPE=application/msword:http://example.com/
///  templates/agenda.doc
/// END:VALARM
/// ```
///
/// Reference: [RFC5545 3.6.6](https://www.rfc-editor.org/rfc/rfc5545.html#section-3.6.6)
pub struct AlarmC;
impl Component for AlarmC {
    const NAME: &'static str = "VALARM";
}
