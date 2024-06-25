use std::marker::PhantomData;

pub mod composite_value_types;
pub mod icalstream;
pub mod value_types;

pub trait Component {
    const NAME: &'static str;
}

// TODO: Modelling of choice of value type, via VALUE= parameter
// TODO: Modelling of comma-separated lists of values (eg RDATE)
// TODO: Modelling of semicolon-separated parts for a value (eg VERSION, GEO)
// --> see https://www.rfc-editor.org/rfc/rfc5545#section-3.1.1

pub trait Property {
    const NAME: &'static str;

    type CompositeValueType: CompositeValueType;
}

pub trait CompositeValueType {}

pub trait ValueType {
    const NAME: &'static str;
}

pub trait Param {
    const NAME: &'static str;

    type ParamValueType: ParamValue;
}

pub trait ParamValueItem {
    const QUOTED: bool;
}

pub trait ParamValue {}

pub struct One<Item: ParamValueItem> {
    _item: PhantomData<Item>,
}

pub struct SetOf<Item: ParamValueItem> {
    _item: PhantomData<Item>,
}

impl<Item: ParamValueItem> ParamValue for One<Item> {}

impl<Item: ParamValueItem> ParamValue for SetOf<Item> {}
