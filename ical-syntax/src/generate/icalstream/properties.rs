use super::writer::Property;

/// Property Name:  VERSION
///
/// Purpose:  This property specifies the identifier corresponding to the
/// highest version number or the minimum and maximum range of the
/// iCalendar specification that is required in order to interpret the
/// iCalendar object.
///
/// Value Type:  TEXT
///
/// Property Parameters:  IANA and non-standard property parameters can
/// be specified on this property.
///
/// Conformance:  This property MUST be specified once in an iCalendar
/// object.
///
/// Description:  A value of "2.0" corresponds to this memo.
///
/// Format Definition:  This property is defined by the following
/// notation:
///
/// ```abnf
/// version    = "VERSION" verparam ":" vervalue CRLF
///
/// verparam   = *(";" other-param)
///
/// vervalue   = "2.0"         ;This memo
///             / maxver
///             / (minver ";" maxver)
///
/// minver     = <A IANA-registered iCalendar version identifier>
/// ;Minimum iCalendar version needed to parse the iCalendar object.
///
/// maxver     = <A IANA-registered iCalendar version identifier>
/// ;Maximum iCalendar version needed to parse the iCalendar object.
/// ```
///
/// Example:  The following is an example of this property:
///
/// ```text
/// VERSION:2.0
/// ```
///
/// Reference: [RFC5545 3.7.4](https://www.rfc-editor.org/rfc/rfc5545.html#section-3.7.4)
pub struct Version;

impl Property for Version {
    const NAME: &'static str = "VERSION";

    type ValueType = super::value_types::Text;
}

/// Property Name:  PRODID
///
/// Purpose:  This property specifies the identifier for the product that
/// created the iCalendar object.
///
/// Value Type:  TEXT
///
/// Property Parameters:  IANA and non-standard property parameters can
/// be specified on this property.
///
/// Conformance:  The property MUST be specified once in an iCalendar
/// object.
///
/// Description:  The vendor of the implementation SHOULD assure that
/// this is a globally unique identifier; using some technique such as
/// an FPI value, as defined in [ISO.9070.1991].
///
/// This property SHOULD NOT be used to alter the interpretation of an
/// iCalendar object beyond the semantics specified in this memo.  For
/// example, it is not to be used to further the understanding of non-
/// standard properties.
///
/// Format Definition:  This property is defined by the following notation:
///
/// ```abnf
/// prodid     = "PRODID" pidparam ":" pidvalue CRLF
///
/// pidparam   = *(";" other-param)
///
/// pidvalue   = text
/// ;Any text that describes the product and version
/// ;and that is generally assured of being unique.
/// ```
///
/// Example:  The following is an example of this property.  It does not
/// imply that English is the default language.
///
/// ```text
/// PRODID:-//ABC Corporation//NONSGML My Product//EN
/// ```
///
/// Reference: [RFC5545 3.7.3](https://www.rfc-editor.org/rfc/rfc5545.html#section-3.7.3)
pub struct ProdId;

impl Property for ProdId {
    const NAME: &'static str = "PRODID";

    type ValueType = super::value_types::Text;
}
