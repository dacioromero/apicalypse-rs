use super::value::{Value, ValueSlice};
use derive_more::{Constructor, Deref, DerefMut, From};
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum ConditionKind<'a> {
    Eq(Value<'a>),
    Ne(Value<'a>),
    Gt(Value<'a>),
    Gte(Value<'a>),
    Lt(Value<'a>),
    Lte(Value<'a>),
    AllOf(ValueSlice<'a>),
    NoneOf(ValueSlice<'a>),
    OneOf(ValueSlice<'a>),
    Only(ValueSlice<'a>),
}

impl<'a> fmt::Display for ConditionKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eq(v) => write!(f, "={}", v),
            Self::Ne(v) => write!(f, "!={}", v),
            Self::Gt(v) => write!(f, ">{}", v),
            Self::Gte(v) => write!(f, ">={}", v),
            Self::Lt(v) => write!(f, "<{}", v),
            Self::Lte(v) => write!(f, "<={}", v),
            Self::AllOf(v) => write!(f, "=[{}]", v),
            Self::NoneOf(v) => write!(f, "=![{}]", v),
            Self::OneOf(v) => write!(f, "=({})", v),
            Self::Only(v) => write!(f, "={{{}}}", v),
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum ChainKind {
    And,
    Or,
}

impl fmt::Display for ChainKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::And => write!(f, "&"),
            Self::Or => write!(f, "|"),
        }
    }
}

#[derive(Debug, Clone, Deref, DerefMut, Constructor)]
struct Chain<'a> {
    kind: ChainKind,
    #[deref]
    #[deref_mut]
    condition: Box<Condition<'a>>,
}

impl<'a> fmt::Display for Chain<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.kind, self.condition)
    }
}

#[derive(Debug, Clone)]
pub struct Condition<'a> {
    field: &'a str,
    kind: ConditionKind<'a>,
    next: Option<Chain<'a>>,
}

impl<'a> Condition<'a> {
    pub fn new(field: &'a str, kind: ConditionKind<'a>) -> Self {
        Self {
            field,
            kind,
            next: None,
        }
    }

    pub fn and(&mut self, condition: Condition<'a>) -> &mut Self {
        if let Some(c) = &mut self.next {
            c.and(condition);
        } else {
            self.next = Chain::new(ChainKind::And, condition.into()).into();
        }

        self
    }

    pub fn or(&mut self, condition: Condition<'a>) -> &mut Self {
        if let Some(c) = &mut self.next {
            c.or(condition);
        } else {
            self.next = Chain::new(ChainKind::Or, condition.into()).into();
        }

        self
    }
}

impl<'a> fmt::Display for Condition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.field, self.kind)?;

        match &self.next {
            Some(c) => write!(f, "{}", c),
            None => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Deref, DerefMut, From)]
pub struct Where<'a>(Condition<'a>);

impl<'a> fmt::Display for Where<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "where {};", self.0)
    }
}
