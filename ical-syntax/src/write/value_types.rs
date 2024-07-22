use std::{borrow::Borrow, fmt::Write};

use crate::structure::{
    composite_value_types::{Any2, Any3, List},
    icalstream::parameters::Value,
    value_types::*,
    ValueType,
};

use super::{composite_value_types::AsCompositeValueType, PropertyValueWriter};

pub trait AsValueType<To: ValueType> {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result;
}

pub enum Never {}

impl<V: ValueType> AsValueType<V> for Never {
    fn fmt<W: std::fmt::Write>(&self, _w: &mut W) -> std::fmt::Result {
        unreachable!()
    }
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

pub enum PeriodOfTimeValue<
    StartT: AsValueType<DateTime>,
    EndT: AsValueType<DateTime>,
    DurationT: AsValueType<Duration>,
> {
    Explicit(StartT, EndT),
    Start(StartT, DurationT),
}

pub struct PeriodOfTimeBuilder<StartT: AsValueType<DateTime>> {
    start: StartT,
}

impl<StartT: AsValueType<DateTime>> PeriodOfTimeBuilder<StartT> {
    pub fn start(start: StartT) -> Self {
        Self { start }
    }

    pub fn end<EndT: AsValueType<DateTime>>(
        self,
        end: EndT,
    ) -> PeriodOfTimeValue<StartT, EndT, Never> {
        // TODO validate that start is earlier than end
        PeriodOfTimeValue::Explicit(self.start, end)
    }

    pub fn duration<DurationT: AsValueType<Duration>>(
        self,
        duration: DurationT,
    ) -> PeriodOfTimeValue<StartT, Never, DurationT> {
        // TODO validate that duration is non-negative (and non-zero?)
        PeriodOfTimeValue::Start(self.start, duration)
    }
}

impl<
        StartT: AsValueType<DateTime>,
        EndT: AsValueType<DateTime>,
        DurationT: AsValueType<Duration>,
    > AsValueType<PeriodOfTime> for PeriodOfTimeValue<StartT, EndT, DurationT>
{
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        match self {
            PeriodOfTimeValue::Explicit(start, end) => {
                start.fmt(w)?;
                write!(w, "/")?;
                end.fmt(w)?;
            }
            PeriodOfTimeValue::Start(start, duration) => {
                start.fmt(w)?;
                write!(w, "/")?;
                duration.fmt(w)?;
            }
        }

        Ok(())
    }
}

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

    #[cfg(feature = "chrono04")]
    #[test]
    fn period_of_time() {
        use crate::write::chrono04::DateTimeForm;

        let start = chrono::DateTime::parse_from_rfc3339("2024-06-26T12:00:00Z")
            .unwrap()
            .to_utc();

        let end = chrono::DateTime::parse_from_rfc3339("2024-06-26T13:00:00Z")
            .unwrap()
            .to_utc();

        let duration = end - start;

        test_case::<PeriodOfTime>(
            PeriodOfTimeBuilder::start(DateTimeForm::from(start)).end(DateTimeForm::from(end)),
            "20240626T120000Z/20240626T130000Z",
        );

        test_case::<PeriodOfTime>(
            PeriodOfTimeBuilder::start(DateTimeForm::from(start)).duration(duration),
            "20240626T120000Z/PT3600S",
        );
    }
}
