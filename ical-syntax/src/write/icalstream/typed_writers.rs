use std::fmt::{Error, Write};

use crate::{
    structure::{
        icalstream::{components::*, properties::calendar::*},
        value_types::*,
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
    pub fn new(inner: W) -> Self {
        Self {
            inner: Writer::new(inner),
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

    pub fn end(self) -> Result<(), Error> {
        self.inner.end()
    }
}
