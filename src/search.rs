use super::Value;
use derive_more::Constructor;
use std::fmt;

#[derive(Debug, Clone, Copy, Constructor)]
pub struct Search<'a> {
    field: &'a str,
    value: Value<'a>,
}

impl<'a> fmt::Display for Search<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "search {} {};", self.field, self.value)
    }
}
