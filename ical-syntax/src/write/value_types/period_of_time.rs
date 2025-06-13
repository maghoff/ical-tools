use std::fmt::Write;

use crate::structure::value_types::{DateTime, Duration, PeriodOfTime};

use super::AsValueType;

pub struct PeriodOfTimeStartEndValue<StartT, EndT> {
    start: StartT,
    end: EndT,
}

impl<StartT: AsValueType<DateTime>, EndT: AsValueType<DateTime>> AsValueType<PeriodOfTime>
    for PeriodOfTimeStartEndValue<StartT, EndT>
{
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        self.start.fmt(w)?;
        write!(w, "/")?;
        self.end.fmt(w)?;

        Ok(())
    }
}

pub struct PeriodOfTimeDurationValue<StartT, DurationT> {
    start: StartT,
    duration: DurationT,
}

impl<StartT: AsValueType<DateTime>, DurationT: AsValueType<Duration>> AsValueType<PeriodOfTime>
    for PeriodOfTimeDurationValue<StartT, DurationT>
{
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        self.start.fmt(w)?;
        write!(w, "/")?;
        self.duration.fmt(w)?;

        Ok(())
    }
}

pub struct PeriodOfTimeBuilder<StartT> {
    start: StartT,
}

impl<StartT: AsValueType<DateTime>> PeriodOfTimeBuilder<StartT> {
    pub fn start(start: StartT) -> Self {
        Self { start }
    }

    pub fn end<EndT: AsValueType<DateTime>>(
        self,
        end: EndT,
    ) -> PeriodOfTimeStartEndValue<StartT, EndT> {
        // TODO validate that start is earlier than end
        PeriodOfTimeStartEndValue {
            start: self.start,
            end,
        }
    }

    pub fn duration<DurationT: AsValueType<Duration>>(
        self,
        duration: DurationT,
    ) -> PeriodOfTimeDurationValue<StartT, DurationT> {
        // TODO validate that duration is non-negative (and non-zero?)
        PeriodOfTimeDurationValue {
            start: self.start,
            duration,
        }
    }
}
