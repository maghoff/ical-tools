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
