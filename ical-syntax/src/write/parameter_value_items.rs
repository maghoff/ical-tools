use crate::structure::parameter_value_items::Name;

use super::AsParamValueItem;

impl<T: std::fmt::Display> AsParamValueItem<Name> for T {
    fn fmt<W: std::fmt::Write>(&self, w: &mut W) -> std::fmt::Result {
        // TODO, validate that the format adheres to the name grammar?
        write!(w, "{}", self)
    }
}
