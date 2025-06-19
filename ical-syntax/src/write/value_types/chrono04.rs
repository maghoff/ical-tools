#![cfg(feature = "chrono04")]

use chrono::{Datelike as _, Timelike as _};

use super::AsValueType;
use crate::{
    structure::value_types::{Date, DateTime, DateTimeUtc, Duration},
    write::value_types::ToValueType,
};

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

impl ToValueType for chrono::NaiveDateTime {
    type ValueType = DateTime;
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

impl AsValueType<DateTimeUtc> for chrono::DateTime<chrono::Utc> {
    fn fmt<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        <Self as AsValueType<DateTime>>::fmt(self, w)
    }
}

impl ToValueType for chrono::DateTime<chrono::Utc> {
    type ValueType = DateTime;
}

impl AsValueType<Date> for chrono::NaiveDate {
    fn fmt<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{:04}{:02}{:02}", self.year(), self.month(), self.day())
    }
}

impl ToValueType for chrono::NaiveDate {
    type ValueType = Date;
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
    fn date() {
        let date = chrono::NaiveDate::from_ymd_opt(2024, 6, 26).unwrap();
        test_case::<Date>(date, "20240626");
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
