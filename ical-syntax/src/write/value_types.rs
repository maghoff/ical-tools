use std::{borrow::Borrow, fmt::Write};

use crate::structure::{
    composite_value_types::Any2, icalstream::parameters::Value, value_types::*, ValueType,
};

use super::{composite_value_types::AsCompositeValueType, PropertyValueWriter};

pub trait AsValueType<To: ValueType> {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result;
}

// TODO impl<T: AsRef<[u8]>> AsValueType<Binary> for T {
// When using Binary, two parameters must be set: VALUE=BINARY and ENCODING=BASE64
// I _think_ BINARY is never the default type, so the VALUE parameter is taken
// care of. However, the ENCODING parameter probably means we must implement
// AsCompositeValueType instead, and this messes with type composition.

impl<T: Borrow<bool>> AsValueType<Boolean> for T {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        if *self.borrow() {
            write!(w, "TRUE")
        } else {
            write!(w, "FALSE")
        }
    }
}

// TODO: Find a relevant type for impling AsValueType<CalAddress>
// Maybe EmailAddress from https://crates.io/crates/email_address ?

// See the chrono04-module for AsValueType<DateTime> and AsValueType<Date>

/// A choice between the [DATETIME][DateTime] and [DATE][Date] value types.
pub enum DateTimeOrDate<T0: AsValueType<DateTime>, T1: AsValueType<Date>> {
    DateTime(T0),
    Date(T1),
}

impl<T0: AsValueType<DateTime>, T1: AsValueType<Date>> AsCompositeValueType<Any2<DateTime, Date>>
    for DateTimeOrDate<T0, T1>
{
    fn write_into<W: std::fmt::Write>(
        self,
        mut prop_value_writer: PropertyValueWriter<W>,
    ) -> std::fmt::Result {
        match self {
            DateTimeOrDate::DateTime(x) => x.write_into(prop_value_writer),
            DateTimeOrDate::Date(x) => {
                prop_value_writer.param(Value, Date::NAME)?;
                x.write_into(prop_value_writer)
            }
        }
    }
}

// TODO impl AsValueType<Float> for misc f-types

// TODO impl AsValueType<Integer>

// TODO impl AsValueType<Period>

// TODO impl AsValueType<RecurrenceRule>. And probably elaborate types for
// building recurrence rules

impl<T: std::fmt::Display> AsValueType<Text> for T {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{}", self)
    }
}

/// Helper type for the TRANSP property.
#[derive(PartialEq, Eq, Debug)]
pub enum TimeTransparency {
    Opaque,
    Transparent,
}

impl Default for TimeTransparency {
    fn default() -> Self {
        Self::Opaque
    }
}

impl std::fmt::Display for TimeTransparency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeTransparency::Opaque => write!(f, "OPAQUE"),
            TimeTransparency::Transparent => write!(f, "TRANSPARENT"),
        }
    }
}

// TODO impl AsValueType<Time>

// TODO impl AsValueType<Uri>

// TODO impl AsValueType<UtcOffset>

#[cfg(test)]
mod test {
    use crate::structure::ValueType;

    use super::*;

    fn test_case<V: ValueType>(v: impl AsValueType<V>, expected: &str) {
        let mut buf = String::new();
        AsValueType::<V>::fmt(&v, &mut buf).unwrap();
        assert_eq!(&buf, expected);
    }

    #[test]
    fn boolean() {
        test_case::<Boolean>(true, "TRUE");
        test_case::<Boolean>(false, "FALSE");
    }

    #[test]
    fn text() {
        test_case::<Text>("simple text", "simple text");
        test_case::<Text>("escaping is elsewhere;,\n", "escaping is elsewhere;,\n");
    }
}
