use std::{
    fmt::Write,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use super::{content_line::ParamValueWriter, ContentLine, LineStream};

pub trait Component {
    const NAME: &'static str;
}

pub trait Property {
    const NAME: &'static str;

    type ValueType: ValueType;
}

pub trait Param {
    const NAME: &'static str;

    type ParamValueType: ParamValue;
}

pub trait ParamValueItem {
    const QUOTED: bool;
}

pub trait AsParamValueItem<To: ParamValueItem> {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result;
}

pub trait AsParamValue<To: ParamValue> {
    fn write_to<W: Write, P: ParamValueWriter<W>>(self, w: &mut P) -> std::fmt::Result;
}

pub struct One<Item: ParamValueItem> {
    _item: PhantomData<Item>,
}

pub struct SetOf<Item: ParamValueItem> {
    _item: PhantomData<Item>,
}

pub trait ParamValue {}

impl<Item: ParamValueItem> ParamValue for One<Item> {}

impl<Item: ParamValueItem> ParamValue for SetOf<Item> {}

impl<T, Inner: ParamValueItem> AsParamValue<One<Inner>> for T
where
    T: AsParamValueItem<Inner>,
{
    fn write_to<W: Write, P: ParamValueWriter<W>>(self, w: &mut P) -> std::fmt::Result {
        if Inner::QUOTED {
            let mut w = w.param_value_quoted_writer()?;
            self.fmt(&mut w)?;
            w.close()?;
        } else {
            let mut w = w.param_value_unquoted_writer()?;
            self.fmt(&mut w)?;
        }
        Ok(())
    }
}

impl<T, Inner: ParamValueItem, Item> AsParamValue<SetOf<Inner>> for T
where
    Item: AsParamValueItem<Inner>,
    T: IntoIterator<Item = Item>,
{
    fn write_to<W: Write, P: ParamValueWriter<W>>(self, w: &mut P) -> std::fmt::Result {
        for item in self {
            item.write_to(w)?;
        }
        Ok(())
    }
}

pub trait ValueType {}

pub trait AsValueType<To: ValueType> {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result;
}

pub struct Writer<W: Write> {
    inner: LineStream<W>,
}

impl<W: Write> Writer<W> {
    pub fn new(inner: W) -> Self {
        Self {
            inner: LineStream::new(inner),
        }
    }

    pub fn component<'x, 'y: 'x, C: Component>(
        &'y mut self,
        component: C,
    ) -> Result<ComponentWriter<'x, W, C>, std::fmt::Error> {
        ComponentWriter::new(self, component)
    }

    pub fn property<'x, 'y: 'x, P: Property>(
        &'y mut self,
        _property: P,
    ) -> Result<PropertyWriter<'x, W, P>, std::fmt::Error> {
        PropertyWriter::new(&mut self.inner)
    }

    pub fn simple_property<P: Property>(
        &mut self,
        property: P,
        value: impl AsValueType<P::ValueType>,
    ) -> std::fmt::Result {
        let mut p = self.property(property)?;
        p.value(value)?;
        p.end()
    }
}

const BEGIN_COMPONENT: &str = "BEGIN";
const END_COMPONENT: &str = "END";

pub struct ComponentWriter<'a, W: Write, C: Component> {
    inner: &'a mut Writer<W>,
    is_closed: bool,
    _component: PhantomData<C>,
}

impl<'a, W: Write, C: Component> ComponentWriter<'a, W, C> {
    pub fn new(inner: &'a mut Writer<W>, _component: C) -> Result<Self, std::fmt::Error> {
        // TODO Hmm... Should this be modeled as a property instead?
        inner.inner.simple_line(BEGIN_COMPONENT, C::NAME)?;

        Ok(Self {
            inner,
            is_closed: false,
            _component: PhantomData,
        })
    }

    pub fn end(mut self) -> std::fmt::Result {
        self.is_closed = true;

        // TODO Hmm... Should this be modeled as a property instead?
        self.inner.inner.simple_line(END_COMPONENT, C::NAME)
    }
}

impl<'a, W: Write, C: Component> Drop for ComponentWriter<'a, W, C> {
    fn drop(&mut self) {
        assert!(
            self.is_closed,
            "ComponentWriter::end() must be called before drop"
        );
    }
}

impl<W: Write, C: Component> Deref for ComponentWriter<'_, W, C> {
    type Target = Writer<W>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<W: Write, C: Component> DerefMut for ComponentWriter<'_, W, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

pub struct PropertyWriter<'a, W: Write, P: Property> {
    content_line: ContentLine<&'a mut W>,
    is_closed: bool,
    _property: PhantomData<P>,
}

impl<'a, W: Write, P: Property> PropertyWriter<'a, W, P> {
    pub fn new(inner: &'a mut LineStream<W>) -> Result<Self, std::fmt::Error> {
        let mut content_line = inner.content_line();
        content_line.name(P::NAME)?;

        Ok(Self {
            content_line,
            is_closed: false,
            _property: PhantomData,
        })
    }

    pub fn param<'x, 'y: 'x, PP: Param>(
        &'y mut self,
        _param: PP,
        value: impl AsParamValue<PP::ParamValueType>,
    ) -> std::fmt::Result {
        self.content_line.param_name(PP::NAME)?;
        value.write_to(&mut self.content_line)
    }

    pub fn value(&mut self, value: impl AsValueType<P::ValueType>) -> std::fmt::Result {
        let w = self.content_line.value_writer()?;
        value.fmt(w)
    }

    pub fn end(mut self) -> std::fmt::Result {
        self.is_closed = true;
        self.content_line.eol()
    }
}

// TODO figure out what to do with Drop here and in ContentLine
//
// impl<'a, W: Write, P: GenericProperty> Drop for Property<'a, W, P> {
//     fn drop(&mut self) {
//         assert!(self.is_closed, "Property::end() must be called before drop");
//     }
// }

#[cfg(test)]
mod test {
    use std::{borrow::Borrow, fmt::Display};

    use crate::generate::LineStream;

    use super::*;

    struct TestProp;
    impl Property for TestProp {
        const NAME: &'static str = "TEST";

        type ValueType = TextValue;
    }

    struct TextValue;
    impl ValueType for TextValue {}

    impl<T: Display> AsValueType<TextValue> for T {
        fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
            write!(w, "{}", self)
        }
    }

    struct TextParamItem;
    impl ParamValueItem for TextParamItem {
        const QUOTED: bool = true;
    }

    impl<T: Display> AsParamValueItem<TextParamItem> for T {
        fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
            write!(w, "{}", self)
        }
    }

    #[test]
    fn singular_parameter() -> std::fmt::Result {
        struct SingularParam;
        impl Param for SingularParam {
            const NAME: &'static str = "SINGULAR";

            type ParamValueType = One<TextParamItem>;
        }

        let mut buf = String::new();
        let mut line_stream = LineStream::new(&mut buf);
        let mut prop = PropertyWriter::<_, TestProp>::new(&mut line_stream)?;

        prop.param(SingularParam, "kake")?;
        prop.param(SingularParam, "kake".to_owned())?;
        prop.value("brille")?;
        prop.end()?;

        assert_eq!(&buf, "TEST;SINGULAR=\"kake\";SINGULAR=\"kake\":brille\r\n");

        Ok(())
    }

    #[test]
    fn parameter_set() -> std::fmt::Result {
        struct PluralParam;
        impl Param for PluralParam {
            const NAME: &'static str = "PLURAL";

            type ParamValueType = SetOf<TextParamItem>;
        }

        let mut buf = String::new();
        let mut line_stream = LineStream::new(&mut buf);
        let mut prop = PropertyWriter::<_, TestProp>::new(&mut line_stream)?;

        prop.param(PluralParam, ["kake", "hatt"])?;
        prop.param(PluralParam, &["kake", "hatt"])?;
        prop.value("brille")?;
        prop.end()?;

        assert_eq!(
            &buf,
            "TEST;PLURAL=\"kake\",\"hatt\";PLURAL=\"kake\",\"hatt\":brille\r\n"
        );

        Ok(())
    }

    #[test]
    fn custom_param_type() -> std::fmt::Result {
        struct CustomParam;
        impl Param for CustomParam {
            const NAME: &'static str = "C";

            type ParamValueType = One<CustomValue>;
        }

        struct CustomValue;
        impl ParamValueItem for CustomValue {
            const QUOTED: bool = false;
        }

        struct Custom;

        // TODO Hmm... Letting the user choose between using a borrow or move
        // when writing the parameter value requires us to implement this trait
        // for Borrow<Custom> instead of just Custom. Is this a bit weird? Could
        // we move the mechanism elsewhere so the implementer of Custom does not
        // need to deal with this?
        impl<T: Borrow<Custom>> AsParamValueItem<CustomValue> for T {
            fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
                write!(w, "CUSTOM")
            }
        }

        let mut buf = String::new();
        let mut line_stream = LineStream::new(&mut buf);
        let mut prop = PropertyWriter::<_, TestProp>::new(&mut line_stream)?;

        let value = Custom;
        prop.param(CustomParam, &value)?;
        prop.param(CustomParam, value)?;
        prop.value("brille")?;
        prop.end()?;

        assert_eq!(&buf, "TEST;C=CUSTOM;C=CUSTOM:brille\r\n");

        Ok(())
    }
}
