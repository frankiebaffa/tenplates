#[cfg(test)]
mod test;

use {
    base64::prelude::{ BASE64_STANDARD, Engine, },
    crate::{
        context::Alias,
        error::{
            InternalError,
            InternalResult,
            IntoInternal,
        },
    },
    std::{
        cmp::Ordering,
        collections::HashMap,
    },
    rusqlite::{
        self,
        RowIndex,
        ToSql,
        types::{
            self,
            FromSql,
            FromSqlError,
            ValueRef,
            Null,
            Value as RusqliteValue,
        },
        Row as RusqliteRow,
    },
};

pub(crate) trait ToOutput {
    fn to_output(&self, formatters: ()) -> InternalResult<String>;
}

pub(crate) trait IsTruthy {
    fn is_truthy(&self) -> bool;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Integer(Option<i64>);

impl Integer {
    pub(crate) fn get(&self) -> &Option<i64> {
        &self.0
    }
}

impl PartialEq<Real> for Integer {
    fn eq(&self, other: &Real) -> bool {
        match self.0 {
            Some(a) => match other.0 {
                Some(b) => (a as f64) == b,
                None => false,
            },
            None => false
        }
    }
}

impl IsTruthy for Integer {
    fn is_truthy(&self) -> bool {
        match self.0 {
            Some(a) => a > 0,
            None => false,
        }
    }
}

impl ToSql for Integer {
    fn to_sql(&self) -> rusqlite::Result<types::ToSqlOutput> {
        Ok(types::ToSqlOutput::Borrowed(self.get().as_ref()
            .map_or(ValueRef::Null, |v| ValueRef::Integer(*v))
        ))
    }
}

impl FromSql for Integer {
    fn column_result(v: ValueRef) -> Result<Integer, FromSqlError> {
        match v {
            ValueRef::Null => Ok(Integer(None)),
            ValueRef::Integer(i) => Ok(Integer(Some(i))),
            ValueRef::Real(_) => Err(FromSqlError::InvalidType),
            ValueRef::Text(_) => Err(FromSqlError::InvalidType),
            ValueRef::Blob(_) => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToOutput for Integer {
    fn to_output(&self, _: ()) -> InternalResult<String> {
        match &self.0 {
            Some(t) => {
                // TODO: Format
                Ok(format!("{t}"))
            },
            None => {
                Ok(String::new())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Real(Option<f64>);

impl Real {
    pub(crate) fn get(&self) -> &Option<f64> {
        &self.0
    }
}

impl From<Integer> for Real {
    fn from(input: Integer) -> Real {
        Real(input.0.map(|i| i as f64))
    }
}

impl PartialEq<Integer> for Real {
    fn eq(&self, other: &Integer) -> bool {
        match self.0 {
            Some(a) => match other.0 {
                Some(b) => a == (b as f64),
                None => false,
            },
            None => false
        }
    }
}

impl IsTruthy for Real {
    fn is_truthy(&self) -> bool {
        match self.0 {
            Some(a) => a > 0_f64,
            None => false,
        }
    }
}

impl ToSql for Real {
    fn to_sql(&self) -> rusqlite::Result<types::ToSqlOutput> {
        Ok(types::ToSqlOutput::Borrowed(self.get().as_ref()
            .map_or(ValueRef::Null, |v| ValueRef::Real(*v))
        ))
    }
}

impl FromSql for Real {
    fn column_result(v: ValueRef) -> Result<Real, FromSqlError> {
        match v {
            ValueRef::Null => Ok(Real(None)),
            ValueRef::Real(r) => Ok(Real(Some(r))),
            ValueRef::Integer(_) => Err(FromSqlError::InvalidType),
            ValueRef::Text(_) => Err(FromSqlError::InvalidType),
            ValueRef::Blob(_) => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToOutput for Real {
    fn to_output(&self, _: ()) -> InternalResult<String> {
        match &self.0 {
            Some(t) => {
                // TODO: Format
                Ok(format!("{t}"))
            },
            None => {
                Ok(String::new())
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Text(Option<String>);

impl Text {
    pub(crate) fn get(&self) -> &Option<String> {
        &self.0
    }
}

impl IsTruthy for Text {
    fn is_truthy(&self) -> bool {
        match &self.0 {
            Some(t) => !t.is_empty(),
            None => false,
        }
    }
}

impl ToSql for Text {
    fn to_sql(&self) -> rusqlite::Result<types::ToSqlOutput> {
        Ok(types::ToSqlOutput::Borrowed(self.get().as_ref()
            .map_or(ValueRef::Null, |v| ValueRef::Text(v.as_bytes()))
        ))
    }
}

impl FromSql for Text {
    fn column_result(v: ValueRef) -> Result<Text, FromSqlError> {
        match v {
            ValueRef::Null => Ok(Text(None)),
            ValueRef::Text(t) => match String::from_utf8(t.to_owned()) {
                Ok(t) => Ok(Text(Some(t))),
                Err(_) => Err(FromSqlError::InvalidType),
            },
            ValueRef::Integer(_) => Err(FromSqlError::InvalidType),
            ValueRef::Real(_) => Err(FromSqlError::InvalidType),
            ValueRef::Blob(_) => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToOutput for Text {
    fn to_output(&self, _: ()) -> InternalResult<String> {
        match &self.0 {
            Some(t) => {
                // TODO: Format
                Ok(t.to_owned())
            },
            None => {
                Ok(String::new())
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Blob(Option<Vec<u8>>);

impl Blob {
    pub(crate) fn get(&self) -> &Option<Vec<u8>> {
        &self.0
    }
}

impl IsTruthy for Blob {
    fn is_truthy(&self) -> bool {
        match &self.0 {
            Some(t) => !t.is_empty(),
            None => false,
        }
    }
}

impl ToSql for Blob {
    fn to_sql(&self) -> rusqlite::Result<types::ToSqlOutput> {
        Ok(types::ToSqlOutput::Borrowed(self.get().as_ref()
            .map_or(ValueRef::Null, |v| ValueRef::Blob(v.as_slice()))
        ))
    }
}

impl FromSql for Blob {
    fn column_result(v: ValueRef) -> Result<Blob, FromSqlError> {
        match v {
            ValueRef::Null => Ok(Blob(None)),
            ValueRef::Blob(b) => Ok(Blob(Some(b.to_owned()))),
            ValueRef::Integer(_) => Err(FromSqlError::InvalidType),
            ValueRef::Real(_) => Err(FromSqlError::InvalidType),
            ValueRef::Text(_) => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToOutput for Blob {
    fn to_output(&self, _: ()) -> InternalResult<String> {
        match &self.0 {
            Some(t) => {
                // TODO: Format
                Ok(BASE64_STANDARD.encode(t))
            },
            None => {
                Ok(String::new())
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Row(HashMap<String, Value>);

impl IsTruthy for Row {
    fn is_truthy(&self) -> bool {
        !self.0.is_empty()
    }
}

impl ToOutput for Row {
    fn to_output(&self, _: ()) -> InternalResult<String> {
        Err(InternalError::new("Row cannot be output"))
    }
}

impl Row {
    pub(crate) fn get(&self, name: &str) -> Option<&Value> {
        self.0.get(name)
    }

    pub(crate) fn insert<S>(&mut self, name: S, value: Value) -> InternalResult<Option<Value>>
    where
        S: AsRef<str>,
    {
        let alias = Alias::from_str(name.as_ref())?;

        if alias.has_row() && !alias.has_column() {
            return Err(InternalError::new("Cannot insert a value at a specific row"));
        }
        else if !alias.has_row() && alias.has_column() {
            return Err(InternalError::new("Cannot insert a value at a specific column"));
        }
        else if alias.has_row() && alias.has_column() {
            return Err(InternalError::new("Cannot insert a value at a specific row's column"));
        }

        match value {
            Value::Row(_) => {
                Err(InternalError::new("A row cannot contain another row"))
            },
            Value::Rows(_) => {
                Err(InternalError::new("A row cannot contain a rows"))
            },
            _ => {
                Ok(self.0.insert(alias.into_variable(), value))
            },
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Rows(Vec<Value>);

impl Rows {
    pub(crate) fn get_row(&self, row: usize) -> Option<&Value> {
        self.0.get(row)
    }

    pub(crate) fn get_column(&self, column: &str) -> Option<&Value> {
        self.0.first().and_then(|v| v.as_row().and_then(|c| c.get(column)))
    }

    pub(crate) fn push<V>(&mut self, into_val: V)
    where
        V: Into<Value>,
    {
        self.0.push(into_val.into());
    }
}

impl IsTruthy for Rows {
    fn is_truthy(&self) -> bool {
        !self.0.is_empty()
    }
}

impl ToOutput for Rows {
    fn to_output(&self, _: ()) -> InternalResult<String> {
        Err(InternalError::new("Rows cannot be output"))
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Integer(Integer),
    Real(Real),
    Text(Text),
    Blob(Blob),
    Null,
    Row(Row),
    Rows(Rows),
}

impl IsTruthy for Value {
    fn is_truthy(&self) -> bool {
        match self {
            Value::Integer(i) => i.is_truthy(),
            Value::Real(r) => r.is_truthy(),
            Value::Text(t) => t.is_truthy(),
            Value::Blob(b) => b.is_truthy(),
            Value::Null => false,
            Value::Row(c) => c.is_truthy(),
            Value::Rows(m) => m.is_truthy(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match self {
            Value::Integer(a) => match other {
                Value::Integer(b) => a == b,
                Value::Real(b) => a == b,
                _ => false,
            },
            Value::Real(a) => match other {
                Value::Real(b) => a == b,
                Value::Integer(b) => a == b,
                _ => false,
            },
            Value::Text(a) => match other {
                Value::Text(b) => a == b,
                _ => false,
            },
            Value::Blob(a) => match other {
                Value::Blob(b) => a == b,
                _ => false,
            },
            Value::Null => false,
            Value::Row(_) => false,
            Value::Rows(_) => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match self {
            Value::Integer(a) => match other {
                Value::Integer(b) => a.partial_cmp(b),
                _ => None,
            },
            Value::Real(a) => match other {
                Value::Real(b) => a.partial_cmp(b),
                _ => None,
            },
            Value::Text(a) => match other {
                Value::Text(b) => a.partial_cmp(b),
                _ => None,
            },
            Value::Blob(a) => match other {
                Value::Blob(b) => a.partial_cmp(b),
                _ => None,
            },
            Value::Null => None,
            Value::Row(_) => None,
            Value::Rows(_) => None,
        }
    }
}

impl ToSql for Value {
    fn to_sql(&self) -> rusqlite::Result<types::ToSqlOutput> {
        match self {
            Self::Integer(i) => i.to_sql(),
            Self::Real(r) => r.to_sql(),
            Self::Text(t) => t.to_sql(),
            Self::Blob(b) => b.to_sql(),
            Self::Null => Null.to_sql(),
            Self::Row(_) => Err(rusqlite::Error::InvalidQuery),
            Self::Rows(_) => Err(rusqlite::Error::InvalidQuery),
        }
    }
}

impl ToOutput for Value {
    fn to_output(&self, formatters: ()) -> InternalResult<String> {
        match self {
            Self::Integer(i) => i.to_output(formatters),
            Self::Real(r) => r.to_output(formatters),
            Self::Text(t) => t.to_output(formatters),
            Self::Blob(b) => b.to_output(formatters),
            Self::Null => Ok(String::new()),
            Self::Row(c) => c.to_output(formatters),
            Self::Rows(m) => m.to_output(formatters),
        }
    }
}

impl From<Integer> for Value {
    fn from(input: Integer) -> Self {
        Self::Integer(input)
    }
}

impl From<Real> for Value {
    fn from(input: Real) -> Self {
        Self::Real(input)
    }
}

impl From<Text> for Value {
    fn from(input: Text) -> Self {
        Self::Text(input)
    }
}

impl From<Blob> for Value {
    fn from(input: Blob) -> Self {
        Self::Blob(input)
    }
}

impl From<Row> for Value {
    fn from(input: Row) -> Self {
        Self::Row(input)
    }
}

impl Value {
    #[cfg(test)]
    pub(crate) fn as_integer(&self) -> Option<&Integer> {
        match self {
            Self::Integer(i) => Some(i),
            _ => None,
        }
    }

    pub(crate) fn as_text(&self) -> Option<&Text> {
        match self {
            Self::Text(t) => Some(t),
            _ => None,
        }
    }

    pub(crate) fn as_row(&self) -> Option<&Row> {
        match self {
            Self::Row(c) => Some(c),
            _ => None,
        }
    }

    pub(crate) fn as_rows(&self) -> Option<&Rows> {
        match self {
            Self::Rows(m) => Some(m),
            _ => None,
        }
    }

    pub(crate) fn get_column(&self, column: &str) -> Option<&Value> {
        match self {
            Self::Rows(m) => m.get_column(column),
            Self::Row(c) => c.get(column),
            _ => None,
        }
    }

    pub(crate) fn into_vec(self) -> Vec<Value> {
        match self {
            Self::Rows(m) => m.0,
            _ => vec![ self ],
        }
    }

    #[cfg(test)]
    pub(crate) fn iter(&self) -> ValueIter<'_> {
        ValueIter::from(self)
    }
}

pub struct ValueIter<'value> {
    values: std::vec::IntoIter<&'value Value>,
}

impl<'value> From<&'value Value> for ValueIter<'value> {
    fn from(value: &'value Value) -> Self {
        match value {
            Value::Rows(m) => Self {
                values: m.0.iter().collect::<Vec<&Value>>().into_iter(),
            },
            _ => Self { values: vec![ value ].into_iter(), },
        }
    }
}

impl<'value> Iterator for ValueIter<'value> {
    type Item = &'value Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.values.next()
    }
}

impl From<Option<i64>> for Value {
    fn from(input: Option<i64>) -> Self {
        Self::Integer(Integer(input))
    }
}

impl From<i64> for Value {
    fn from(input: i64) -> Self {
        Some(input).into()
    }
}

impl From<Option<f64>> for Value {
    fn from(input: Option<f64>) -> Self {
        Self::Real(Real(input))
    }
}

impl From<f64> for Value {
    fn from(input: f64) -> Self {
        Some(input).into()
    }
}

impl From<Option<String>> for Value {
    fn from(input: Option<String>) -> Self {
        Self::Text(Text(input))
    }
}

impl From<String> for Value {
    fn from(input: String) -> Self {
        Some(input).into()
    }
}

impl<'from> From<Option<&'from str>> for Value {
    fn from(input: Option<&'from str>) -> Self {
        input.map(str::to_owned).into()
    }
}

impl<'from> From<&'from str> for Value {
    fn from(input: &'from str) -> Self {
        Some(input.to_owned()).into()
    }
}

impl From<Option<Vec<u8>>> for Value {
    fn from(input: Option<Vec<u8>>) -> Self {
        Self::Blob(Blob(input))
    }
}

impl From<Vec<u8>> for Value {
    fn from(input: Vec<u8>) -> Self {
        Some(input).into()
    }
}

impl Value {
    pub(crate) fn from_row<ColIdx>(row: &RusqliteRow, col_idx: ColIdx) -> InternalResult<Value>
    where
        ColIdx: RowIndex
    {
        let rf = row.get_ref::<ColIdx>(col_idx)
            .into_internal("Failed to load raw value from row")?;
        let sql_value = RusqliteValue::column_result(rf)
            .into_internal("Failed to convert the row's raw value into a known value")?;
        Ok(match sql_value {
            RusqliteValue::Null => Self::Null,
            RusqliteValue::Integer(i) => Value::from(i),
            RusqliteValue::Real(r) => Value::from(r),
            RusqliteValue::Text(t) => Value::from(t),
            RusqliteValue::Blob(b) => Value::from(b),
        })
    }
}

impl ToOutput for Option<&Value> {
    fn to_output(&self, formatters: ()) -> InternalResult<String> {
        match self {
            Some(v) => v.to_output(formatters),
            None => Ok(String::new()),
        }
    }
}
