use derive_more::{Deref, DerefMut, From};
use std::fmt;

#[derive(Debug, Copy, Clone, From)]
pub enum Value<'a> {
    Str(&'a str),
    Int(i32),
    Float(f32),
    Bool(bool),
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Str(v) => write!(f, "\"{}\"", v),
            Self::Int(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
            Self::Bool(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Clone, Copy, Deref, DerefMut, From)]
pub struct ValueSlice<'a>(&'a [Value<'a>]);

impl<'a> fmt::Display for ValueSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}
