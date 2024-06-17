use std::{borrow::Borrow, fmt::Write};

use crate::{generate::AsValueType, structure::icalstream::value_types::*};

// TODO impl<T: AsRef<[u8]>> AsValueType<Binary> for T {

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

// TODO impl AsValueType<Date> for... some chrono type, I suppose?

// TODO impl AsValueType<DateTime> for some chrono type, I guess?

// TODO impl AsValueType<Duration> for a signed duration type in chrono?

// TODO impl AsValueType<Float> for misc f-types

// TODO impl AsValueType<Integer>

// TODO impl AsValueType<Period>

// TODO impl AsValueType<RecurrenceRule>. And probably elaborate types for
// building recurrence rules

impl<T: std::fmt::Display> AsValueType<Text> for T {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        // TODO Implement escaping according to grammar above
        write!(w, "{}", self)
    }
}

// TODO impl AsValueType<Time>

// TODO impl AsValueType<Uri>

// TODO impl AsValueType<UtcOffset>
