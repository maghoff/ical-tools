#![cfg(feature = "jiff02")]

use super::{AsValueType, DateTimeOrDate, NeverValue};
use crate::structure::value_types::{Date, DateTime, DateTimeUtc, Duration};

/// `jiff::civil::DateTime` corresponds to the _floating_ form of a DateTime
impl AsValueType<DateTime> for jiff::civil::DateTime {
    fn fmt<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(
            w,
            "{:04}{:02}{:02}T{:02}{:02}{:02}",
            self.year(),
            self.month(),
            self.day(),
            self.hour(),
            self.minute(),
            self.second()
        )
    }
}

impl From<jiff::civil::DateTime> for DateTimeOrDate<jiff::civil::DateTime, NeverValue> {
    fn from(value: jiff::civil::DateTime) -> Self {
        Self::DateTime(value)
    }
}

pub struct UtcForm {
    datetime: jiff::civil::DateTime,
}

impl UtcForm {
    pub fn from_civil(datetime: jiff::civil::DateTime) -> Self {
        Self { datetime }
    }

    pub fn from_zoned(datetime: jiff::Zoned) -> Result<Self, jiff::Error> {
        Ok(Self {
            datetime: datetime.in_tz("UTC")?.datetime(),
        })
    }
}

impl AsValueType<DateTime> for UtcForm {
    fn fmt<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        <jiff::civil::DateTime as AsValueType<DateTime>>::fmt(&self.datetime, w)?;
        write!(w, "Z")
    }
}

impl AsValueType<DateTimeUtc> for UtcForm {
    fn fmt<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        <Self as AsValueType<DateTime>>::fmt(self, w)
    }
}

impl From<UtcForm> for DateTimeOrDate<UtcForm, NeverValue> {
    fn from(value: UtcForm) -> Self {
        Self::DateTime(value)
    }
}

impl AsValueType<Date> for jiff::civil::Date {
    fn fmt<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{:04}{:02}{:02}", self.year(), self.month(), self.day())
    }
}

impl From<jiff::civil::Date> for DateTimeOrDate<NeverValue, jiff::civil::Date> {
    fn from(value: jiff::civil::Date) -> Self {
        Self::Date(value)
    }
}

impl AsValueType<Duration> for jiff::Span {
    fn fmt<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{}", self)
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
    fn datetime_floating() {
        let datetime: jiff::civil::DateTime = "2024-06-19 15:22:45".parse().unwrap();
        test_case::<DateTime>(datetime, "20240619T152245");
    }

    #[test]
    fn date() {
        let date = jiff::civil::date(2024, 6, 26);
        test_case::<Date>(date, "20240626");
    }

    #[test]
    fn duration() {
        test_case::<Duration>(jiff::Span::new().hours(1), "PT1H");
    }

    #[test]
    fn period_of_time() {
        use crate::{
            structure::value_types::PeriodOfTime, write::value_types::PeriodOfTimeBuilder,
        };

        let start: jiff::civil::DateTime = "2024-06-26T12:00:00".parse().unwrap();

        let end: jiff::civil::DateTime = "2024-06-26T13:00:00".parse().unwrap();

        let duration = end - start;

        test_case::<PeriodOfTime>(
            PeriodOfTimeBuilder::start(start).end(end),
            "20240626T120000/20240626T130000",
        );

        test_case::<PeriodOfTime>(
            PeriodOfTimeBuilder::start(start).duration(duration),
            "20240626T120000/PT1H",
        );
    }
}
