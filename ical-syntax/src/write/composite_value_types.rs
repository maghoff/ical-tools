use std::fmt::Write;

use crate::structure::{
    composite_value_types::{Any2, Any3, List, ValueTypeChoice},
    CompositeValueType, ValueType,
};

use super::{
    value_types::{AsValueType, AsValueTypeChoice},
    PropertyValueWriter,
};

pub trait AsCompositeValueType<To: CompositeValueType> {
    fn write_into<W: Write>(self, prop_value_writer: PropertyValueWriter<W>) -> std::fmt::Result;
}

// All the value types can be used directly as a CompositeValueType:
impl<T: AsValueType<V>, V: ValueType> AsCompositeValueType<V> for T {
    fn write_into<W: Write>(
        self,
        mut prop_value_writer: PropertyValueWriter<W>,
    ) -> std::fmt::Result {
        let mut tw = prop_value_writer.value_tuple_writer()?;
        let mut w = tw.next_value_writer()?;
        self.fmt(&mut w)
    }
}

impl<T: AsValueType<V>, V: ValueType> AsValueTypeChoice<V> for T {
    type Type = V;
}

// Tuples of value types can be used as CompositeValueType:
macro_rules! tuple_value_type {
    ($( $t:ident ),+ ; $( $v:ident ),+ ) => {
        impl<$( $t: AsValueType<$v> ),+, $( $v: ValueType ),+> AsCompositeValueType<($( $v, )+)> for ($( $t, )+) {
            #[allow(non_snake_case)]
            fn write_into<W: Write>(self, mut prop_value_writer: PropertyValueWriter<W>) -> std::fmt::Result {
                let mut tw = prop_value_writer.value_tuple_writer()?;

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

// Choice of value types
impl<RustType, T0, T1> AsCompositeValueType<Any2<T0, T1>> for RustType
where
    T0: ValueType,
    T1: ValueType,
    RustType: AsValueTypeChoice<Any2<T0, T1>>
        + AsValueType<<RustType as AsValueTypeChoice<Any2<T0, T1>>>::Type>,
{
    fn write_into<W: Write>(
        self,
        mut prop_value_writer: PropertyValueWriter<W>,
    ) -> std::fmt::Result {
        if <RustType as AsValueTypeChoice<Any2<T0, T1>>>::Type::NAME
            != <Any2<T0, T1> as ValueTypeChoice>::DefaultType::NAME
        {
            prop_value_writer.param(
                crate::structure::icalstream::parameters::Value,
                <RustType as AsValueTypeChoice<Any2<T0, T1>>>::Type::NAME,
            )?;
        }

        let mut tw = prop_value_writer.value_tuple_writer()?;
        let mut w = tw.next_value_writer()?;
        self.fmt(&mut w)
    }
}

impl<RustType, T0, T1, T2> AsCompositeValueType<Any3<T0, T1, T2>> for RustType
where
    T0: ValueType,
    T1: ValueType,
    T2: ValueType,
    RustType: AsValueTypeChoice<Any3<T0, T1, T2>>
        + AsValueType<<RustType as AsValueTypeChoice<Any3<T0, T1, T2>>>::Type>,
{
    fn write_into<W: Write>(
        self,
        mut prop_value_writer: PropertyValueWriter<W>,
    ) -> std::fmt::Result {
        if <RustType as AsValueTypeChoice<Any3<T0, T1, T2>>>::Type::NAME
            != <Any2<T0, T1> as ValueTypeChoice>::DefaultType::NAME
        {
            prop_value_writer.param(
                crate::structure::icalstream::parameters::Value,
                <RustType as AsValueTypeChoice<Any3<T0, T1, T2>>>::Type::NAME,
            )?;
        }

        let mut tw = prop_value_writer.value_tuple_writer()?;
        let mut w = tw.next_value_writer()?;
        self.fmt(&mut w)
    }
}

// Lists of value types:
impl<VT, RustType, I> AsCompositeValueType<List<VT>> for I
where
    VT: ValueTypeChoice,
    I: IntoIterator<Item = RustType>,
    RustType: AsValueTypeChoice<VT> + AsValueType<<RustType as AsValueTypeChoice<VT>>::Type>,
{
    fn write_into<W: Write>(
        self,
        mut prop_value_writer: PropertyValueWriter<W>,
    ) -> std::fmt::Result {
        if <RustType as AsValueTypeChoice<VT>>::Type::NAME != VT::DefaultType::NAME {
            prop_value_writer.param(
                crate::structure::icalstream::parameters::Value,
                <RustType as AsValueTypeChoice<VT>>::Type::NAME,
            )?;
        }

        let mut lw = prop_value_writer.value_list_writer()?;

        for item in self {
            item.fmt(&mut lw.next_value_writer()?)?;
        }

        Ok(())
    }
}
