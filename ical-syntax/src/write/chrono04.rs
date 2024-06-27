#![cfg(feature = "chrono04")]

use chrono::{Datelike, Timelike, Utc};

use crate::structure::value_types::{Date, DateTime};

use super::value_types::AsValueType;

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

pub type DateTimeOrDate = super::value_types::DateTimeOrDate<DateTimeForm, chrono::NaiveDate>;

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

impl From<chrono::NaiveDateTime> for DateTimeOrDate {
    fn from(value: chrono::NaiveDateTime) -> Self {
        Self::DateTime(value.into())
    }
}

impl From<chrono::DateTime<Utc>> for DateTimeOrDate {
    fn from(value: chrono::DateTime<Utc>) -> Self {
        Self::DateTime(value.into())
    }
}

impl From<chrono::NaiveDate> for DateTimeOrDate {
    fn from(value: chrono::NaiveDate) -> Self {
        Self::Date(value)
    }
}
