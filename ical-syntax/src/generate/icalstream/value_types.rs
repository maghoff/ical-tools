use std::fmt::Write;

use super::writer::{AsValueType, ValueType};

pub struct Text;

impl ValueType for Text {}

impl<T: std::fmt::Display> AsValueType<Text> for T {
    fn fmt<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{}", self)
    }
}
