use std::fmt::Write;

use crate::structure::{CompositeValueType, ValueType};

use super::{value_types::AsValueType, ContentLine};

pub trait AsCompositeValueType<To: CompositeValueType> {
    fn write_to<W: Write>(&self, content_line: &mut ContentLine<W>) -> std::fmt::Result;
}

// All the value types can be used directly as a CompositeValueType:
impl<T: AsValueType<V>, V: ValueType> AsCompositeValueType<V> for T {
    fn write_to<W: Write>(&self, content_line: &mut ContentLine<W>) -> std::fmt::Result {
        let mut tw = content_line.value_tuple_writer()?;
        let mut w = tw.next_value_writer()?;
        self.fmt(&mut w)
    }
}

// Tuples of value types can be used as CompositeValueType:
macro_rules! tuple_value_type {
    ($( $t:ident ),+ ; $( $v:ident ),+ ) => {
        impl<$( $t: AsValueType<$v> ),+, $( $v: ValueType ),+> AsCompositeValueType<($( $v, )+)> for ($( $t, )+) {
            #[allow(non_snake_case)]
            fn write_to<W: Write>(&self, content_line: &mut ContentLine<W>) -> std::fmt::Result {
                let mut tw = content_line.value_tuple_writer()?;

                let ($( $v, )+) = self;
                $( $v.fmt(&mut tw.next_value_writer()?)?; )+

                Ok(())
            }
        }

    };
}

tuple_value_type!(T0 ; V0);
tuple_value_type!(T0, T1 ; V0, V1);
tuple_value_type!(T0, T1, T2 ; V0, V1, V2);
tuple_value_type!(T0, T1, T2, T3 ; V0, V1, V2, V3);
