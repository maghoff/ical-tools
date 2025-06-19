mod chrono04;
pub mod jiff02;
mod period_of_time;

use std::{borrow::Borrow, fmt::Write};

use super::{composite_value_types::AsCompositeValueType, PropertyValueWriter};
use crate::structure::{
    composite_value_types::{Any2, Any3, IsA, List},
    icalstream::parameters::Value,
    value_types::*,
    ValueType,
};

pub use period_of_time::{
    PeriodOfTimeBuilder, PeriodOfTimeDurationValue, PeriodOfTimeStartEndValue,
};

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

// See the chrono04 and jiff02 modules for AsValueType<DateTime>,
// AsValueType<DateTimeUtc> and AsValueType<Date>

trait ToValueType {
    type ValueType;
}

impl<RustType, VT, T1, T2> AsCompositeValueType<Any2<T1, T2>> for RustType
where
    T1: ValueType,
    T2: ValueType,
    VT: ValueType + IsA<Any2<T1, T2>>,
    RustType: AsCompositeValueType<VT> + ToValueType<ValueType = VT>,
{
    fn write_into<W: std::fmt::Write>(
        self,
        mut prop_value_writer: PropertyValueWriter<W>,
    ) -> std::fmt::Result {
        if VT::NAME != T1::NAME {
            prop_value_writer.param(Value, VT::NAME)?;
        }
        <Self as AsCompositeValueType<VT>>::write_into(self, prop_value_writer)
    }
}

// TODO impl AsValueType<Float> for misc f-types

// TODO impl AsValueType<Integer>

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

pub struct DateTimeList<V: AsCompositeValueType<List<DateTime>>>(pub V);

impl<V: AsCompositeValueType<List<DateTime>>>
    AsCompositeValueType<Any3<List<DateTime>, List<Date>, List<PeriodOfTime>>> for DateTimeList<V>
{
    fn write_into<W: Write>(self, prop_value_writer: PropertyValueWriter<W>) -> std::fmt::Result {
        self.0.write_into(prop_value_writer)
    }
}

pub struct DateList<V: AsCompositeValueType<List<Date>>>(pub V);

impl<V: AsCompositeValueType<List<Date>>>
    AsCompositeValueType<Any3<List<DateTime>, List<Date>, List<PeriodOfTime>>> for DateList<V>
{
    fn write_into<W: Write>(
        self,
        mut prop_value_writer: PropertyValueWriter<W>,
    ) -> std::fmt::Result {
        prop_value_writer.param(Value, Date::NAME)?;
        self.0.write_into(prop_value_writer)
    }
}

pub struct PeriodOfTimeList<V: AsCompositeValueType<List<PeriodOfTime>>>(pub V);

impl<V: AsCompositeValueType<List<PeriodOfTime>>>
    AsCompositeValueType<Any3<List<DateTime>, List<Date>, List<PeriodOfTime>>>
    for PeriodOfTimeList<V>
{
    fn write_into<W: Write>(
        self,
        mut prop_value_writer: PropertyValueWriter<W>,
    ) -> std::fmt::Result {
        prop_value_writer.param(Value, PeriodOfTime::NAME)?;
        self.0.write_into(prop_value_writer)
    }
}

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
