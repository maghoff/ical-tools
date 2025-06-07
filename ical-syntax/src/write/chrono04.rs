#![cfg(feature = "chrono04")]

use chrono::{Datelike as _, Timelike as _};

use crate::structure::value_types::{Date, DateTime, DateTimeUtc, Duration};
use crate::write::value_types::NeverValue;

use super::value_types::AsValueType;
use super::value_types::DateTimeOrDate;

/// `chrono::NaiveDateTime` corresponds to the _floating_ form of a DateTime
impl AsValueType<DateTime> for chrono::NaiveDateTime {
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

/// `chrono::DateTime<chrono::Utc>` corresponds to the _UTC_ form of a DateTime
impl AsValueType<DateTime> for chrono::DateTime<chrono::Utc> {
    fn fmt<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(
            w,
            "{:04}{:02}{:02}T{:02}{:02}{:02}Z",
            self.year(),
            self.month(),
            self.day(),
            self.hour(),
            self.minute(),
            self.second()
        )
    }
}

impl AsValueType<Date> for chrono::NaiveDate {
    fn fmt<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{:04}{:02}{:02}", self.year(), self.month(), self.day())
    }
}

impl From<chrono::NaiveDateTime> for DateTimeOrDate<chrono::NaiveDateTime, NeverValue> {
    fn from(value: chrono::NaiveDateTime) -> Self {
        Self::DateTime(value)
    }
}

impl From<chrono::DateTime<chrono::Utc>>
    for DateTimeOrDate<chrono::DateTime<chrono::Utc>, NeverValue>
{
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self::DateTime(value)
    }
}

impl From<chrono::NaiveDate> for DateTimeOrDate<NeverValue, chrono::NaiveDate> {
    fn from(value: chrono::NaiveDate) -> Self {
        Self::Date(value)
    }
}

impl AsValueType<DateTimeUtc> for chrono::DateTime<chrono::Utc> {
    fn fmt<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(
            w,
            "{:04}{:02}{:02}T{:02}{:02}{:02}Z",
            self.year(),
            self.month(),
            self.day(),
            self.hour(),
            self.minute(),
            self.second()
        )
    }
}

impl AsValueType<Duration> for chrono::TimeDelta {
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
        let datetime = chrono::DateTime::parse_from_rfc3339("2024-06-26T12:00:00Z")
            .unwrap()
            .naive_utc();
        test_case::<DateTime>(datetime, "20240626T120000");
    }

    #[test]
    fn datetime_utc() {
        let datetime = chrono::DateTime::parse_from_rfc3339("2024-06-26T12:00:00Z")
            .unwrap()
            .to_utc();
        test_case::<DateTime>(datetime, "20240626T120000Z");
    }

    #[test]
    fn duration() {
        test_case::<Duration>(chrono::TimeDelta::hours(1), "PT3600S");
    }

    #[test]
    fn period_of_time() {
        use crate::{
            structure::value_types::PeriodOfTime, write::value_types::PeriodOfTimeBuilder,
        };

        let start = chrono::DateTime::parse_from_rfc3339("2024-06-26T12:00:00Z")
            .unwrap()
            .to_utc();

        let end = chrono::DateTime::parse_from_rfc3339("2024-06-26T13:00:00Z")
            .unwrap()
            .to_utc();

        let duration = end - start;

        test_case::<PeriodOfTime>(
            PeriodOfTimeBuilder::start(start).end(end),
            "20240626T120000Z/20240626T130000Z",
        );

        test_case::<PeriodOfTime>(
            PeriodOfTimeBuilder::start(start).duration(duration),
            "20240626T120000Z/PT3600S",
        );
    }
}
