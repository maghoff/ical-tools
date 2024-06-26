use std::marker::PhantomData;

use super::{CompositeValueType, ValueType};

// All the value types can be used directly as a CompositeValueType:
impl<T: ValueType> CompositeValueType for T {}

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
pub struct List<V: ValueType> {
    _phantom: PhantomData<V>,
}

impl<V: ValueType> CompositeValueType for List<V> {}

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
pub struct Any2<T0: ValueType, T1: ValueType> {
    _phantom0: T0,
    _phantom1: T1,
}

impl<T0: ValueType, T1: ValueType> CompositeValueType for Any2<T0, T1> {}
