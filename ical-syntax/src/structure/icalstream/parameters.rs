use crate::structure::{One, Param};

use super::parameter_value_items::Name;

/// Value Data Types
///
/// Parameter Name:  VALUE
///
/// Purpose:  To explicitly specify the value type format for a property
/// value.
///
/// Format Definition:  This property parameter is defined by the following
/// notation:
///
/// ```abnf
/// valuetypeparam = "VALUE" "=" valuetype
///
/// valuetype  = ("BINARY"
///            / "BOOLEAN"
///            / "CAL-ADDRESS"
///            / "DATE"
///            / "DATE-TIME"
///            / "DURATION"
///            / "FLOAT"
///            / "INTEGER"
///            / "PERIOD"
///            / "RECUR"
///            / "TEXT"
///            / "TIME"
///            / "URI"
///            / "UTC-OFFSET"
///            / x-name
///            ; Some experimental iCalendar value type.
///            / iana-token)
///            ; Some other IANA-registered iCalendar value type.
/// ```
///
/// Description:  This parameter specifies the value type and format of the
/// property value.  The property values MUST be of a single value type.  For
/// example, a "RDATE" property cannot have a combination of DATE-TIME and
/// TIME value types.
///
/// If the property's value is the default value type, then this parameter
/// need not be specified.  However, if the property's default value type is
/// overridden by some other allowable value type, then this parameter MUST be
/// specified.
///
/// Applications MUST preserve the value data for x-name and iana-token values
/// that they don't recognize without attempting to interpret or parse the
/// value data.
///
/// Reference: [RFC 5545
/// 3.2.20](https://www.rfc-editor.org/rfc/rfc5545#section-3.2.20)
pub struct Value;

impl Param for Value {
    const NAME: &'static str = "VALUE";

    type ParamValueType = One<Name>;
}
