use derive_more::Constructor;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Order {
    Ascending,
    Descending,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ascending => write!(f, "asc"),
            Self::Descending => write!(f, "desc"),
        }
    }
}

#[derive(Debug, Clone, Copy, Constructor)]
pub(super) struct Sort<'a> {
    field: &'a str,
    order: Order,
}

impl<'a> fmt::Display for Sort<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sort {} {};", self.field, self.order)
    }
}
