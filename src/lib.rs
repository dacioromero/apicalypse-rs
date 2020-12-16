mod search;
pub mod sort;
pub mod value;
pub mod r#where;

use derive_more::{Deref, DerefMut, From};
use r#where::{Condition, Where};
use search::Search;
use sort::{Order, Sort};
use std::fmt;
use value::Value;

#[derive(Debug, Clone, Default, Deref, DerefMut)]
struct FieldVec<'a>(Vec<&'a str>);

impl<'a> fmt::Display for FieldVec<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.join(","))
    }
}

#[derive(Debug, Clone, Default, Deref, DerefMut, From)]
struct Fields<'a>(FieldVec<'a>);

impl<'a> fmt::Display for Fields<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.is_empty() {
            true => write!(f, "fields *;"),
            false => write!(f, "fields {};", self.0),
        }
    }
}

#[derive(Debug, Clone, Default, Deref, DerefMut, From)]
struct Exclude<'a>(FieldVec<'a>);

impl<'a> fmt::Display for Exclude<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.is_empty() {
            true => Ok(()),
            false => write!(f, "exclude {};", self.0),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Deref, DerefMut, From)]
struct Limit(i32);

impl fmt::Display for Limit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "limit {};", self.0)
    }
}

#[derive(Debug, Clone, Copy, Default, Deref, DerefMut, From)]
struct Offset(i32);

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "offset {};", self.0)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Apicalypse<'a> {
    fields: Fields<'a>,
    exclude: Exclude<'a>,
    r#where: Option<Where<'a>>,
    limit: Option<Limit>,
    offset: Option<Offset>,
    sort: Option<Sort<'a>>,
    search: Option<Search<'a>>,
}

impl<'a> Apicalypse<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn fields(mut self, fields: &'a [&str]) -> Self {
        self.fields.extend_from_slice(fields);

        self
    }

    pub fn exclude(mut self, fields: &'a [&str]) -> Self {
        self.exclude.extend_from_slice(fields);

        self
    }

    pub fn and(mut self, condition: Condition<'a>) -> Self {
        if let Some(w) = &mut self.r#where {
            w.and(condition);
        } else {
            self.r#where = Some(condition.into());
        }

        self
    }

    pub fn or(mut self, condition: Condition<'a>) -> Self {
        if let Some(w) = &mut self.r#where {
            w.or(condition);
        } else {
            self.r#where = Some(condition.into());
        }

        self
    }

    pub fn limit(mut self, limit: Option<i32>) -> Self {
        self.limit = limit.map(Into::into);

        self
    }

    pub fn offset(mut self, offset: Option<i32>) -> Self {
        self.offset = offset.map(Into::into);

        self
    }

    pub fn sort(mut self, field: &'a str, order: Order) -> Self {
        self.sort = Sort::new(field, order).into();

        self
    }

    pub fn search<T: Into<Value<'a>>>(mut self, field: &'a str, value: T) -> Self {
        self.search = Search::new(field, value.into()).into();

        self
    }
}

impl<'a> fmt::Display for Apicalypse<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.fields, self.exclude)?;

        if let Some(w) = &self.r#where {
            write!(f, "{}", w)?;
        }

        if let Some(l) = &self.limit {
            write!(f, "{}", l)?;
        }

        if let Some(o) = &self.offset {
            write!(f, "{}", o)?;
        }

        if let Some(s) = &self.sort {
            write!(f, "{}", s)?;
        }

        if let Some(s) = &self.search {
            write!(f, "{}", s)?;
        }

        Ok(())
    }
}
