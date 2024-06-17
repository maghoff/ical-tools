use std::marker::PhantomData;

pub mod icalstream;
pub mod value_types;

pub trait Component {
    const NAME: &'static str;
}

pub trait Property {
    const NAME: &'static str;

    type ValueType: ValueType;
}

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
