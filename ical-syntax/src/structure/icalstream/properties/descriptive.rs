//! [RFC 5545 3.8.1: Descriptive Component Properties](https://www.rfc-editor.org/rfc/rfc5545#section-3.8.1)

use crate::structure::{
    icalstream::components::{AlarmCProperty, EventCProperty, JournalCProperty, TodoCProperty},
    value_types::{Float, Text},
    Property,
};

/// Geographic Position
///
/// Property Name:  GEO
///
/// Purpose:  This property specifies information related to the global
/// position for the activity specified by a calendar component.
///
/// Value Type:  FLOAT.  The value MUST be two SEMICOLON-separated FLOAT
/// values.
///
/// Property Parameters:  IANA and non-standard property parameters can be
/// specified on this property.
///
/// Conformance:  This property can be specified in "VEVENT" or "VTODO"
/// calendar components.
///
/// Description:  This property value specifies latitude and longitude, in
/// that order (i.e., "LAT LON" ordering).  The longitude represents the
/// location east or west of the prime meridian as a positive or negative real
/// number, respectively.  The longitude and latitude values MAY be specified
/// up to six decimal places, which will allow for accuracy to within one
/// meter of geographical position.  Receiving applications MUST accept values
/// of this precision and MAY truncate values of greater precision.
///
/// Values for latitude and longitude shall be expressed as decimal fractions
/// of degrees.  Whole degrees of latitude shall be represented by a two-digit
/// decimal number ranging from 0 through
/// 90.  Whole degrees of longitude shall be represented by a decimal number
/// ranging from 0 through 180.  When a decimal fraction of a degree is
/// specified, it shall be separated from the whole number of degrees by a
/// decimal point.
///
/// Latitudes north of the equator shall be specified by a plus sign (+), or
/// by the absence of a minus sign (-), preceding the digits designating
/// degrees.  Latitudes south of the Equator shall be designated by a minus
/// sign (-) preceding the digits designating degrees.  A point on the Equator
/// shall be assigned to the Northern Hemisphere.
///
/// Longitudes east of the prime meridian shall be specified by a plus sign
/// (+), or by the absence of a minus sign (-), preceding the digits
/// designating degrees.  Longitudes west of the meridian shall be designated
/// by minus sign (-) preceding the digits designating degrees.  A point on
/// the prime meridian shall be assigned to the Eastern Hemisphere.  A point
/// on the 180th meridian shall be assigned to the Western Hemisphere.  One
/// exception to this last convention is permitted.  For the special condition
/// of describing a band of latitude around the earth, the East Bounding
/// Coordinate data element shall be assigned the value +180 (180) degrees.
///
/// Any spatial address with a latitude of +90 (90) or -90 degrees will
/// specify the position at the North or South Pole, respectively.  The
/// component for longitude may have any legal value.
///
/// With the exception of the special condition described above, this form is
/// specified in [ANSI INCITS 61-1986].
///
/// The simple formula for converting degrees-minutes-seconds into decimal
/// degrees is:
///
/// ```text
/// decimal = degrees + minutes/60 + seconds/3600.
/// ```
///
/// Format Definition:  This property is defined by the following notation:
///
/// ```abnf
/// geo        = "GEO" geoparam ":" geovalue CRLF
///
/// geoparam   = *(";" other-param)
///
/// geovalue   = float ";" float
/// ;Latitude and Longitude components
/// ```
///
/// Example:  The following is an example of this property:
///
/// ```text
/// GEO:37.386013;-122.082932
/// ```
///
/// Reference: [RFC 5545
/// 3.8.1.6](https://www.rfc-editor.org/rfc/rfc5545#section-3.8.1.6)
pub struct Geo;

impl Property for Geo {
    const NAME: &'static str = "GEO";

    type CompositeValueType = (Float, Float); // Lat, long
}

/// Summary
///
/// Property Name:  SUMMARY
///
/// Purpose:  This property defines a short summary or subject for the
/// calendar component.
///
/// Value Type:  TEXT
///
/// Property Parameters:  IANA, non-standard, alternate text representation,
/// and language property parameters can be specified on this property.
///
/// Conformance:  The property can be specified in "VEVENT", "VTODO",
/// "VJOURNAL", or "VALARM" calendar components.
///
/// Description:  This property is used in the "VEVENT", "VTODO", and
/// "VJOURNAL" calendar components to capture a short, one-line summary about
/// the activity or journal entry.
///
/// This property is used in the "VALARM" calendar component to capture the
/// subject of an EMAIL category of alarm.
///
/// Format Definition:  This property is defined by the following notation:
///
/// ```abnf
/// summary    = "SUMMARY" summparam ":" text CRLF
///
/// summparam  = *(
///            ;
///            ; The following are OPTIONAL,
///            ; but MUST NOT occur more than once.
///            ;
///            (";" altrepparam) / (";" languageparam) /
///            ;
///            ; The following is OPTIONAL,
///            ; and MAY occur more than once.
///            ;
///            (";" other-param)
///            ;
///            )
/// ```
///
/// Example:  The following is an example of this property:
///
/// ```text
/// SUMMARY:Department Party
/// ```
///
/// Reference: [RFC 5545
/// 3.8.1.12](https://www.rfc-editor.org/rfc/rfc5545#section-3.8.1.12)
pub struct Summary;

impl Property for Summary {
    const NAME: &'static str = "SUMMARY";

    type CompositeValueType = Text;
}

impl EventCProperty for Summary {}
impl TodoCProperty for Summary {}
impl JournalCProperty for Summary {}
impl AlarmCProperty for Summary {}
