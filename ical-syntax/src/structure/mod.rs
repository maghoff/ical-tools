pub mod icalstream;

pub trait Component {
    const NAME: &'static str;
}

pub trait Property {
    const NAME: &'static str;

    type ValueType: ValueType;
}

pub trait ValueType {}

pub trait Param {
    const NAME: &'static str;

    type ParamValueType: ParamValue;
}

pub trait ParamValueItem {
    const QUOTED: bool;
}

pub trait ParamValue {}
