use std::marker::PhantomData;

use super::{CompositeValueType, ValueType};

// TODO: Maybe tuples should impl ValueType directly? This might allow
// encoding the value type of Version as Any2<Text, (Text, Text)>

// Tuples of value types can be used as CompositeValueType:
macro_rules! tuple_value_type {
    ($( $x:ident ),+) => {
        impl<$( $x ),+> CompositeValueType for ($( $x, )+) where $( $x: ValueType ),+ {}
    };
}

tuple_value_type!(T0);
tuple_value_type!(T0, T1);
tuple_value_type!(T0, T1, T2);
tuple_value_type!(T0, T1, T2, T3);

/// List of values, for property value types.
///
/// For example, the CATEGORIES property takes a list of TEXT values:
///
/// ```
/// # use ical_syntax::structure::{
/// #     composite_value_types::List,
/// #     value_types::{Text},
/// #     Property,
/// # };
///
/// pub struct Categories;
///
/// impl Property for Categories {
///     const NAME: &'static str = "CATEGORIES";
///
///     type CompositeValueType = List<Text>;
/// }
/// ```
pub struct List<V> {
    _phantom: PhantomData<V>,
}

impl<V: ValueTypeChoice> CompositeValueType for List<V> {}
impl<V: ValueTypeChoice> ValueTypeList for List<V> {}

impl<V: ValueTypeChoice> CompositeValueType for V {}

pub trait ValueTypeList {}

/// Choice of value types.
///
/// For example, the DTSTART property takes a DATETIME by default, or
/// alternatively a DATE:
///
/// ```
/// # use ical_syntax::structure::{
/// #     composite_value_types::Any2,
/// #     value_types::{DateTime, Date},
/// #     Property,
/// # };
///
/// pub struct DateTimeStart;
///
/// impl Property for DateTimeStart {
///     const NAME: &'static str = "DTSTART";
///
///     type CompositeValueType = Any2<DateTime, Date>;
/// }
/// ```
///
/// The first type argument is handled as the default value type, and no VALUE
/// parameter will be set when it is used. When the second type is used, the
/// VALUE parameter will be set accordingly.
pub trait ValueTypeChoice {
    type DefaultType: ValueType;
}

// A single ValueType is modeled as a predetermined choice of value:
impl<T: ValueType> ValueTypeChoice for T {
    type DefaultType = T;
}

pub struct Any2<T0, T1> {
    _phantom0: T0,
    _phantom1: T1,
}

impl<T0: ValueType, T1: ValueType> ValueTypeChoice for Any2<T0, T1> {
    type DefaultType = T0;
}

pub struct Any3<T0, T1, T2> {
    _phantom0: T0,
    _phantom1: T1,
    _phantom2: T2,
}

impl<T0: ValueType, T1: ValueType, T2: ValueType> ValueTypeChoice for Any3<T0, T1, T2> {
    type DefaultType = T0;
}

/// Marker-trait for ValueType-s to indicate that they are a valid choice for
/// a given ValueTypeChoice.
pub trait IsA<AnyT> {}
