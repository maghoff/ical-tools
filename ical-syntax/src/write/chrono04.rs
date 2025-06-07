#![cfg(feature = "chrono04")]

use chrono::{Datelike, Timelike, Utc};

use crate::structure::value_types::{Date, DateTime, DateTimeUtc, Duration};
use crate::write::value_types::NeverValue;

use super::value_types::AsValueType;
use super::value_types::DateTimeOrDate;

// TODO: Document, at least for myself in the future, why I ended up with this
// intermediate type. Can we not simply implement AsValueType<DateTime>
// directly for the underlying types?

/// A concrete representation of the DATETIME type using types from [chrono].
///
/// [DateTime] values can be expressed in three distinct forms:
/// 1. Floating, corresponding to [chrono::NaiveDateTime]
/// 2. UTC, corresponding to [chrono::DateTime<chrono::Utc>]
/// 3. Tied to a timezone via the TZID parameter
///
/// This type models forms 1 and 2. To express times in form 3 you have to
/// explicitly write the TZID parameter and use [DateTimeForm::Floating].
/// In this case, you also have to write time zone specifications to the
/// ICalObject. See [DateTime] for details.
pub enum DateTimeForm {
    /// A floating DateTime, representing the given datetime in the timezone
    /// the user is in at any given time.
    Floating(chrono::NaiveDateTime),

    /// A DateTime in UTC.
    Utc(chrono::DateTime<Utc>),
}

impl AsValueType<DateTime> for DateTimeForm {
    fn fmt<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        match self {
            DateTimeForm::Floating(x) => {
                write!(
                    w,
                    "{:04}{:02}{:02}T{:02}{:02}{:02}",
                    x.year(),
                    x.month(),
                    x.day(),
                    x.hour(),
                    x.minute(),
                    x.second()
                )
            }
            DateTimeForm::Utc(x) => {
                write!(
                    w,
                    "{:04}{:02}{:02}T{:02}{:02}{:02}Z",
                    x.year(),
                    x.month(),
                    x.day(),
                    x.hour(),
                    x.minute(),
                    x.second()
                )
            }
        }
    }
}

impl AsValueType<Date> for chrono::NaiveDate {
    fn fmt<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{:04}{:02}{:02}", self.year(), self.month(), self.day())
    }
}

impl From<chrono::NaiveDateTime> for DateTimeForm {
    fn from(value: chrono::NaiveDateTime) -> Self {
        Self::Floating(value)
    }
}

impl From<chrono::DateTime<Utc>> for DateTimeForm {
    fn from(value: chrono::DateTime<Utc>) -> Self {
        Self::Utc(value)
    }
}

impl From<chrono::NaiveDateTime> for DateTimeOrDate<DateTimeForm, NeverValue> {
    fn from(value: chrono::NaiveDateTime) -> Self {
        Self::DateTime(value.into())
    }
}

impl From<chrono::DateTime<Utc>> for DateTimeOrDate<DateTimeForm, NeverValue> {
    fn from(value: chrono::DateTime<Utc>) -> Self {
        Self::DateTime(value.into())
    }
}

impl<T: AsValueType<Date>> From<T> for DateTimeOrDate<NeverValue, T> {
    fn from(value: T) -> Self {
        Self::Date(value)
    }
}

impl AsValueType<DateTimeUtc> for chrono::DateTime<Utc> {
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
        test_case::<DateTime>(DateTimeForm::from(datetime), "20240626T120000");
    }

    #[test]
    fn datetime_utc() {
        let datetime = chrono::DateTime::parse_from_rfc3339("2024-06-26T12:00:00Z")
            .unwrap()
            .to_utc();
        test_case::<DateTime>(DateTimeForm::from(datetime), "20240626T120000Z");
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
            PeriodOfTimeBuilder::start(DateTimeForm::from(start)).end(DateTimeForm::from(end)),
            "20240626T120000Z/20240626T130000Z",
        );

        test_case::<PeriodOfTime>(
            PeriodOfTimeBuilder::start(DateTimeForm::from(start)).duration(duration),
            "20240626T120000Z/PT3600S",
        );
    }
}
