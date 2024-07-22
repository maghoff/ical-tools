use std::fmt::{Display, Error, Write};

use crate::{
    structure::{
        icalstream::{
            components::*,
            properties::{
                calendar::*, date_and_time::RecurrenceDateTimes, descriptive::Summary,
                relationship::Uid,
            },
        },
        value_types::*,
        Property,
    },
    write::{
        composite_value_types::AsCompositeValueType, value_types::AsValueType, ComponentWriter,
        PropertyWriter, Writer,
    },
};

/// Reference: [RFC5545 3.4](https://www.rfc-editor.org/rfc/rfc5545.html#section-3.4)
pub struct ICalStreamWriter<W: Write> {
    inner: Writer<W>,
}

impl<W: Write> ICalStreamWriter<W> {
    pub fn with_fmt(inner: W) -> Self {
        Self {
            inner: Writer::with_fmt(inner),
        }
    }

    pub fn component<'x, 'y: 'x, NC: ICalStreamComponent>(
        &'y mut self,
        component: NC,
    ) -> Result<ComponentWriter<'x, W, NC>, Error> {
        ComponentWriter::new(&mut self.inner, component)
    }

    pub fn icalendar_object<'a, 'b: 'a>(
        &'b mut self,
        prod_id: impl AsValueType<Text>,
    ) -> Result<ICalObjectWriter<'a, W>, Error> {
        ICalObjectWriter::new(self.component(ICalObject)?, prod_id)
    }
}

impl<W: std::io::Write> ICalStreamWriter<crate::write::io_adapters::FmtToIo<W>> {
    pub fn new(inner: W) -> Self {
        use crate::write::io_adapters::WriteExtension;

        Self::with_fmt(inner.write_adapter())
    }
}

pub struct ICalObjectWriter<'a, W: Write> {
    inner: ComponentWriter<'a, W, ICalObject>,
}

impl<'a, W: Write> ICalObjectWriter<'a, W> {
    pub fn new(
        inner: ComponentWriter<'a, W, ICalObject>,
        prod_id: impl AsValueType<Text>,
    ) -> Result<Self, Error> {
        let mut new = Self { inner };
        new.simple_property(Version, "2.0")?;
        new.simple_property(ProdId, prod_id)?;

        Ok(new)
    }

    pub fn property<'x, 'y: 'x, P: ICalObjectProperty>(
        &'y mut self,
        property: P,
    ) -> Result<PropertyWriter<'x, W, P>, Error> {
        self.inner.property(property)
    }

    pub fn simple_property<P: ICalObjectProperty>(
        &mut self,
        property: P,
        value: impl AsCompositeValueType<P::CompositeValueType>,
    ) -> std::fmt::Result {
        self.inner.simple_property(property, value)
    }

    pub fn component<'x, 'y: 'x, NC: ICalObjectComponent>(
        &'y mut self,
        component: NC,
    ) -> Result<ComponentWriter<'x, W, NC>, Error> {
        ComponentWriter::new(&mut self.inner, component)
    }

    pub fn event<'x, 'y: 'x>(&'y mut self) -> Result<EventWriter<'x, W>, Error> {
        Ok(EventWriter::new(self.component(EventC)?))
    }

    pub fn end(self) -> Result<(), Error> {
        self.inner.end()
    }
}

pub struct EventWriter<'a, W: Write> {
    inner: ComponentWriter<'a, W, EventC>,
}

impl<'a, W: Write> EventWriter<'a, W> {
    pub fn new(inner: ComponentWriter<'a, W, EventC>) -> Self {
        Self { inner }
    }

    pub fn property<'x, 'y: 'x, P: EventCProperty>(
        &'y mut self,
        property: P,
    ) -> Result<PropertyWriter<'x, W, P>, Error> {
        self.inner.property(property)
    }

    pub fn simple_property<P: EventCProperty>(
        &mut self,
        property: P,
        value: impl AsCompositeValueType<P::CompositeValueType>,
    ) -> std::fmt::Result {
        self.inner.simple_property(property, value)
    }

    pub fn dtstamp(&mut self, dtstamp: impl AsCompositeValueType<DateTimeUtc>) -> std::fmt::Result {
        self.simple_property(
            crate::structure::icalstream::properties::change_management::DateTimeStamp,
            dtstamp,
        )
    }

    pub fn uid(&mut self, value: impl Display) -> std::fmt::Result {
        self.simple_property(Uid, value)
    }

    #[cfg(feature = "chrono04")]
    pub fn dtstart(
        &mut self,
        dtstart: impl Into<crate::write::chrono04::DateTimeOrDate>,
    ) -> std::fmt::Result {
        self.simple_property(
            crate::structure::icalstream::properties::date_and_time::DateTimeStart,
            dtstart.into(),
        )
    }

    pub fn summary(&mut self, value: impl Display) -> std::fmt::Result {
        self.simple_property(Summary, value)
    }

    pub fn time_transparency(
        &mut self,
        value: crate::write::value_types::TimeTransparency,
    ) -> std::fmt::Result {
        if value != Default::default() {
            self.simple_property(
                crate::structure::icalstream::properties::date_and_time::TimeTransparency,
                value,
            )
        } else {
            Ok(())
        }
    }

    pub fn recurrence_datetimes(
        &mut self,
        value: impl AsCompositeValueType<<RecurrenceDateTimes as Property>::CompositeValueType>,
    ) -> std::fmt::Result {
        self.simple_property(RecurrenceDateTimes, value)
    }

    pub fn end(self) -> Result<(), Error> {
        self.inner.end()
    }
}
