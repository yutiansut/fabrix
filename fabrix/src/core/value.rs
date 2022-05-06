//! fabrix value
//!
//! This module contains the value type, which is the atomic unit of data in Fabrix.
//!
//! Custom types:
//! 1. Date
//! 1. Time
//! 1. DateTime
//! 1. Uuid
//! 1. Decimal

use std::any::Any;
use std::fmt::{Debug, Display};

use polars::chunked_array::object::PolarsObjectSafe;
use polars::prelude::{AnyValue, DataType, Field, ObjectType, PolarsObject};
use serde::{Deserialize, Serialize};

use super::{
    impl_custom_value, impl_custom_value2, impl_try_from_value, impl_value_from, BYTES, DATE,
    DATETIME, DECIMAL, TIME, UUID,
};

/// pub type D1<T>
pub type D1<T> = Vec<T>;
/// pub type D1<T>
pub type D2<T> = Vec<Vec<T>>;
/// pub type D1
pub type D1Value = D1<Value>;
/// pub type D2
pub type D2Value = D2<Value>;
/// pub type Date
pub type ObjectTypeDate = ObjectType<Date>;
/// pub type Time
pub type ObjectTypeTime = ObjectType<Time>;
/// pub type DateTime
pub type ObjectTypeDateTime = ObjectType<DateTime>;
/// pub type Uuid
pub type ObjectTypeUuid = ObjectType<Uuid>;
/// pub type Decimal
pub type ObjectTypeDecimal = ObjectType<Decimal>;
/// pub type Bytes
pub type ObjectTypeBytes = ObjectType<Bytes>;

/// Custom Value: Date
#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Date(pub chrono::NaiveDate);

impl Default for Date {
    fn default() -> Self {
        Date(chrono::NaiveDate::from_ymd(1970, 1, 1))
    }
}

impl_custom_value!(Date, DATE);

/// Custom Value: Time
#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Time(pub chrono::NaiveTime);

impl Default for Time {
    fn default() -> Self {
        Time(chrono::NaiveTime::from_hms(0, 0, 0))
    }
}

impl_custom_value!(Time, TIME);

/// Custom Value: DateTime
#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct DateTime(pub chrono::NaiveDateTime);

impl Default for DateTime {
    fn default() -> Self {
        DateTime(chrono::NaiveDateTime::from_timestamp(0, 0))
    }
}

impl_custom_value!(DateTime, DATETIME);

/// Custom Value: Decimal
#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Default)]
pub struct Decimal(pub rust_decimal::Decimal);

impl_custom_value!(Decimal, DECIMAL);

/// Custom Value: Uuid
#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Default)]
pub struct Uuid(pub uuid::Uuid);

impl_custom_value!(Uuid, UUID);

/// Custom Value: Bytes
#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Default)]
pub struct Bytes(pub Vec<u8>);

impl_custom_value2!(Bytes, BYTES);

/// Value is the fundamental element in Fabrix.
/// Providing type conversion between Rust/external type and polars `AnyValue`.
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum Value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    Date(Date),
    Time(Time),
    DateTime(DateTime),
    Decimal(Decimal),
    Uuid(Uuid),
    Bytes(Bytes),
    Null,
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Eq, Hash)]
pub enum ValueType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
    Date,
    Time,
    DateTime,
    Decimal,
    Uuid,
    Bytes,
    Null,
}

impl Default for ValueType {
    fn default() -> Self {
        ValueType::Null
    }
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::Null => write!(f, "null"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<&Value> for ValueType {
    fn from(v: &Value) -> Self {
        match v {
            Value::Bool(_) => ValueType::Bool,
            Value::U8(_) => ValueType::U8,
            Value::U16(_) => ValueType::U16,
            Value::U32(_) => ValueType::U32,
            Value::U64(_) => ValueType::U64,
            Value::I8(_) => ValueType::I8,
            Value::I16(_) => ValueType::I16,
            Value::I32(_) => ValueType::I32,
            Value::I64(_) => ValueType::I64,
            Value::F32(_) => ValueType::F32,
            Value::F64(_) => ValueType::F64,
            Value::String(_) => ValueType::String,
            Value::Date(_) => ValueType::Date,
            Value::Time(_) => ValueType::Time,
            Value::DateTime(_) => ValueType::DateTime,
            Value::Decimal(_) => ValueType::Decimal,
            Value::Uuid(_) => ValueType::Uuid,
            Value::Bytes(_) => ValueType::Bytes,
            Value::Null => ValueType::Null,
        }
    }
}

impl AsRef<ValueType> for Value {
    fn as_ref(&self) -> &ValueType {
        match self {
            Value::Bool(_) => &ValueType::Bool,
            Value::U8(_) => &ValueType::U8,
            Value::U16(_) => &ValueType::U16,
            Value::U32(_) => &ValueType::U32,
            Value::U64(_) => &ValueType::U64,
            Value::I8(_) => &ValueType::I8,
            Value::I16(_) => &ValueType::I16,
            Value::I32(_) => &ValueType::I32,
            Value::I64(_) => &ValueType::I64,
            Value::F32(_) => &ValueType::F32,
            Value::F64(_) => &ValueType::F64,
            Value::String(_) => &ValueType::String,
            Value::Date(_) => &ValueType::Date,
            Value::Time(_) => &ValueType::Time,
            Value::DateTime(_) => &ValueType::DateTime,
            Value::Decimal(_) => &ValueType::Decimal,
            Value::Uuid(_) => &ValueType::Uuid,
            Value::Bytes(_) => &ValueType::Bytes,
            Value::Null => &ValueType::Null,
        }
    }
}

impl From<Value> for ValueType {
    fn from(value: Value) -> Self {
        ValueType::from(&value)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(v) => write!(f, "{:?}", v),
            Value::U8(v) => write!(f, "{:?}", v),
            Value::U16(v) => write!(f, "{:?}", v),
            Value::U32(v) => write!(f, "{:?}", v),
            Value::U64(v) => write!(f, "{:?}", v),
            Value::I8(v) => write!(f, "{:?}", v),
            Value::I16(v) => write!(f, "{:?}", v),
            Value::I32(v) => write!(f, "{:?}", v),
            Value::I64(v) => write!(f, "{:?}", v),
            Value::F32(v) => write!(f, "{:?}", v),
            Value::F64(v) => write!(f, "{:?}", v),
            Value::String(v) => f.write_str(v),
            Value::Date(v) => write!(f, "{:?}", v.0),
            Value::Time(v) => write!(f, "{:?}", v.0),
            Value::DateTime(v) => write!(f, "{:?}", v.0),
            Value::Decimal(v) => write!(f, "{:?}", v.0),
            Value::Uuid(v) => write!(f, "{:?}", v.0),
            Value::Bytes(v) => write!(f, "{:?}", v.0),
            Value::Null => write!(f, "null"),
        }
    }
}

impl From<&Value> for DataType {
    fn from(v: &Value) -> Self {
        match v {
            Value::Bool(_) => DataType::Boolean,
            Value::U8(_) => DataType::UInt8,
            Value::U16(_) => DataType::UInt32,
            Value::U32(_) => DataType::UInt32,
            Value::U64(_) => DataType::UInt64,
            Value::I8(_) => DataType::Int8,
            Value::I16(_) => DataType::Int32,
            Value::I32(_) => DataType::Int32,
            Value::I64(_) => DataType::Int64,
            Value::F32(_) => DataType::Float32,
            Value::F64(_) => DataType::Float64,
            Value::String(_) => DataType::Utf8,
            Value::Date(_) => DataType::Object(DATE),
            Value::Time(_) => DataType::Object(TIME),
            Value::DateTime(_) => DataType::Object(DATETIME),
            Value::Decimal(_) => DataType::Object(DECIMAL),
            Value::Uuid(_) => DataType::Object(UUID),
            Value::Bytes(_) => DataType::Object(BYTES),
            Value::Null => DataType::Null,
        }
    }
}

impl From<Value> for DataType {
    fn from(v: Value) -> Self {
        DataType::from(&v)
    }
}

impl From<&ValueType> for DataType {
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Bool => DataType::Boolean,
            ValueType::U8 => DataType::UInt8,
            ValueType::U16 => DataType::UInt32,
            ValueType::U32 => DataType::UInt32,
            ValueType::U64 => DataType::UInt64,
            ValueType::I8 => DataType::Int8,
            ValueType::I16 => DataType::Int32,
            ValueType::I32 => DataType::Int32,
            ValueType::I64 => DataType::Int64,
            ValueType::F32 => DataType::Float32,
            ValueType::F64 => DataType::Float64,
            ValueType::String => DataType::Utf8,
            ValueType::Date => DataType::Object(DATE),
            ValueType::Time => DataType::Object(TIME),
            ValueType::DateTime => DataType::Object(DATETIME),
            ValueType::Decimal => DataType::Object(DECIMAL),
            ValueType::Uuid => DataType::Object(UUID),
            ValueType::Bytes => DataType::Object(BYTES),
            ValueType::Null => DataType::Null,
        }
    }
}

impl From<&DataType> for &ValueType {
    fn from(v: &DataType) -> Self {
        match v {
            DataType::Boolean => &ValueType::Bool,
            DataType::UInt8 => &ValueType::U8,
            DataType::UInt32 => &ValueType::U16,
            DataType::UInt64 => &ValueType::U32,
            DataType::Int8 => &ValueType::I8,
            DataType::Int32 => &ValueType::I16,
            DataType::Int64 => &ValueType::I32,
            DataType::Float32 => &ValueType::F32,
            DataType::Float64 => &ValueType::F64,
            DataType::Utf8 => &ValueType::String,
            DataType::Object(DATE) => &ValueType::Date,
            DataType::Object(TIME) => &ValueType::Time,
            DataType::Object(DATETIME) => &ValueType::DateTime,
            DataType::Object(DECIMAL) => &ValueType::Decimal,
            DataType::Object(UUID) => &ValueType::Uuid,
            DataType::Object(BYTES) => &ValueType::Bytes,
            DataType::Null => &ValueType::Null,
            _ => unimplemented!(),
        }
    }
}

impl From<ValueType> for DataType {
    fn from(v: ValueType) -> Self {
        DataType::from(&v)
    }
}

impl AsRef<ValueType> for DataType {
    fn as_ref(&self) -> &ValueType {
        match &self {
            DataType::Boolean => &ValueType::Bool,
            DataType::UInt8 => &ValueType::U8,
            DataType::UInt32 => &ValueType::U16,
            DataType::UInt64 => &ValueType::U32,
            DataType::Int8 => &ValueType::I8,
            DataType::Int32 => &ValueType::I16,
            DataType::Int64 => &ValueType::I32,
            DataType::Float32 => &ValueType::F32,
            DataType::Float64 => &ValueType::F64,
            DataType::Utf8 => &ValueType::String,
            DataType::Object(DATE) => &ValueType::Date,
            DataType::Object(TIME) => &ValueType::Time,
            DataType::Object(DATETIME) => &ValueType::DateTime,
            DataType::Object(DECIMAL) => &ValueType::Decimal,
            DataType::Object(UUID) => &ValueType::Uuid,
            DataType::Object(BYTES) => &ValueType::Bytes,
            DataType::Null => &ValueType::Null,
            _ => unimplemented!(),
        }
    }
}

impl AsRef<DataType> for ValueType {
    fn as_ref(&self) -> &DataType {
        match &self {
            ValueType::Bool => &DataType::Boolean,
            ValueType::U8 => &DataType::UInt8,
            ValueType::U16 => &DataType::UInt16,
            ValueType::U32 => &DataType::UInt32,
            ValueType::U64 => &DataType::UInt64,
            ValueType::I8 => &DataType::Int8,
            ValueType::I16 => &DataType::Int16,
            ValueType::I32 => &DataType::Int32,
            ValueType::I64 => &DataType::Int64,
            ValueType::F32 => &DataType::Float32,
            ValueType::F64 => &DataType::Float64,
            ValueType::String => &DataType::Utf8,
            ValueType::Date => &DataType::Object(DATE),
            ValueType::Time => &DataType::Object(TIME),
            ValueType::DateTime => &DataType::Object(DATETIME),
            ValueType::Decimal => &DataType::Object(DECIMAL),
            ValueType::Uuid => &DataType::Object(UUID),
            ValueType::Bytes => &DataType::Object(BYTES),
            ValueType::Null => &DataType::Null,
        }
    }
}

impl From<&DataType> for ValueType {
    fn from(v: &DataType) -> Self {
        match v {
            DataType::Boolean => ValueType::Bool,
            DataType::UInt8 => ValueType::U8,
            DataType::UInt16 => ValueType::U16,
            DataType::UInt32 => ValueType::U32,
            DataType::UInt64 => ValueType::U64,
            DataType::Int8 => ValueType::I8,
            DataType::Int16 => ValueType::I16,
            DataType::Int32 => ValueType::I32,
            DataType::Int64 => ValueType::I64,
            DataType::Float32 => ValueType::F32,
            DataType::Float64 => ValueType::F64,
            DataType::Utf8 => ValueType::String,
            DataType::Object(DATE) => ValueType::Date,
            DataType::Object(TIME) => ValueType::Time,
            DataType::Object(DATETIME) => ValueType::DateTime,
            DataType::Object(DECIMAL) => ValueType::Decimal,
            DataType::Object(UUID) => ValueType::Uuid,
            DataType::Object(BYTES) => ValueType::Bytes,
            DataType::Null => ValueType::Null,
            _ => unimplemented!(),
        }
    }
}

pub struct ValueTypes {
    inner: Vec<DataType>,
}

impl ValueTypes {
    pub fn new<I, V>(iter: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<DataType>,
    {
        Self {
            inner: iter.into_iter().map(|i| i.into()).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn polars_dtypes(&self) -> &[DataType] {
        &self.inner
    }
}

impl From<Vec<ValueType>> for ValueTypes {
    fn from(val: Vec<ValueType>) -> Self {
        ValueTypes::new(val)
    }
}

impl From<DataType> for ValueType {
    fn from(v: DataType) -> Self {
        ValueType::from(&v)
    }
}

impl From<&Value> for Field {
    fn from(v: &Value) -> Self {
        match v {
            Value::Bool(_) => Field::new("", DataType::Boolean),
            Value::U8(_) => Field::new("", DataType::UInt8),
            Value::U16(_) => Field::new("", DataType::UInt16),
            Value::U32(_) => Field::new("", DataType::UInt32),
            Value::U64(_) => Field::new("", DataType::UInt64),
            Value::I8(_) => Field::new("", DataType::Int8),
            Value::I16(_) => Field::new("", DataType::Int16),
            Value::I32(_) => Field::new("", DataType::Int32),
            Value::I64(_) => Field::new("", DataType::Int64),
            Value::F32(_) => Field::new("", DataType::Float32),
            Value::F64(_) => Field::new("", DataType::Float64),
            Value::String(_) => Field::new("", DataType::Utf8),
            Value::Date(_) => Field::new("", DataType::Object(DATE)),
            Value::Time(_) => Field::new("", DataType::Object(TIME)),
            Value::DateTime(_) => Field::new("", DataType::Object(DATETIME)),
            Value::Decimal(_) => Field::new("", DataType::Object(DECIMAL)),
            Value::Uuid(_) => Field::new("", DataType::Object(UUID)),
            Value::Bytes(_) => Field::new("", DataType::Object(BYTES)),
            Value::Null => Field::new("", DataType::Null),
        }
    }
}

impl From<Value> for Field {
    fn from(v: Value) -> Self {
        Field::from(&v)
    }
}

impl From<&ValueType> for Field {
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Bool => Field::new("", DataType::Boolean),
            ValueType::U8 => Field::new("", DataType::UInt8),
            ValueType::U16 => Field::new("", DataType::UInt16),
            ValueType::U32 => Field::new("", DataType::UInt32),
            ValueType::U64 => Field::new("", DataType::UInt64),
            ValueType::I8 => Field::new("", DataType::Int8),
            ValueType::I16 => Field::new("", DataType::Int16),
            ValueType::I32 => Field::new("", DataType::Int32),
            ValueType::I64 => Field::new("", DataType::Int64),
            ValueType::F32 => Field::new("", DataType::Float32),
            ValueType::F64 => Field::new("", DataType::Float64),
            ValueType::String => Field::new("", DataType::Utf8),
            ValueType::Date => Field::new("", DataType::Object(DATE)),
            ValueType::Time => Field::new("", DataType::Object(TIME)),
            ValueType::DateTime => Field::new("", DataType::Object(DATETIME)),
            ValueType::Decimal => Field::new("", DataType::Object(DECIMAL)),
            ValueType::Uuid => Field::new("", DataType::Object(UUID)),
            ValueType::Bytes => Field::new("", DataType::Object(BYTES)),
            ValueType::Null => Field::new("", DataType::Null),
        }
    }
}

impl From<ValueType> for Field {
    fn from(v: ValueType) -> Self {
        Field::from(&v)
    }
}

impl Value {
    pub fn is_dtype_match(&self, dtype: &DataType) -> bool {
        let vd = DataType::from(self);
        &vd == dtype
    }
}

impl ValueType {
    pub fn is_dtype_match(&self, dtype: &DataType) -> bool {
        let vd = DataType::from(self);
        &vd == dtype
    }
}

/// default value: null
impl Default for Value {
    fn default() -> Self {
        Value::Null
    }
}

/// from &dyn PolarsObjectSafe to Value, Beware performance.
impl From<&dyn PolarsObjectSafe> for Value {
    fn from(v: &dyn PolarsObjectSafe) -> Self {
        // Notice: this is not a safe way to convert from &dyn PolarsObjectSafe to Value.
        // Upcasting is an incomplete feature warned by Rust compiler.
        let any = v as &dyn Any;

        if any.is::<Date>() {
            Value::Date(any.downcast_ref::<Date>().unwrap().clone())
        } else if any.is::<Time>() {
            Value::Time(any.downcast_ref::<Time>().unwrap().clone())
        } else if any.is::<DateTime>() {
            Value::DateTime(any.downcast_ref::<DateTime>().unwrap().clone())
        } else if any.is::<Decimal>() {
            Value::Decimal(any.downcast_ref::<Decimal>().unwrap().clone())
        } else if any.is::<Uuid>() {
            Value::Uuid(any.downcast_ref::<Uuid>().unwrap().clone())
        } else if any.is::<Bytes>() {
            Value::Bytes(any.downcast_ref::<Bytes>().unwrap().clone())
        } else {
            Value::Null
        }
    }
}

/// Type conversion: polars' AnyValue -> Value. Beware performance, it usually used in getting values
/// from a Series, or getting a row from a DataFrame through iterating Vec<Series> it owned.
impl<'a> From<AnyValue<'a>> for Value {
    fn from(av: AnyValue<'a>) -> Self {
        match av {
            AnyValue::Null => Value::Null,
            AnyValue::Boolean(v) => Value::Bool(v),
            AnyValue::Utf8(v) => Value::String(v.to_owned()),
            AnyValue::UInt8(v) => Value::U8(v),
            AnyValue::UInt16(v) => Value::U16(v),
            AnyValue::UInt32(v) => Value::U32(v),
            AnyValue::UInt64(v) => Value::U64(v),
            AnyValue::Int8(v) => Value::I8(v),
            AnyValue::Int16(v) => Value::I16(v),
            AnyValue::Int32(v) => Value::I32(v),
            AnyValue::Int64(v) => Value::I64(v),
            AnyValue::Float32(v) => Value::F32(v),
            AnyValue::Float64(v) => Value::F64(v),
            AnyValue::Object(v) => v.into(),
            _ => unimplemented!(),
        }
    }
}

/// Type conversion: Value -> polars' AnyValue
impl<'a> From<&'a Value> for AnyValue<'a> {
    fn from(v: &'a Value) -> Self {
        match v {
            Value::Bool(v) => AnyValue::Boolean(*v),
            Value::U8(v) => AnyValue::UInt8(*v),
            Value::U16(v) => AnyValue::UInt16(*v),
            Value::U32(v) => AnyValue::UInt32(*v),
            Value::U64(v) => AnyValue::UInt64(*v),
            Value::I8(v) => AnyValue::Int8(*v),
            Value::I16(v) => AnyValue::Int16(*v),
            Value::I32(v) => AnyValue::Int32(*v),
            Value::I64(v) => AnyValue::Int64(*v),
            Value::F32(v) => AnyValue::Float32(*v),
            Value::F64(v) => AnyValue::Float64(*v),
            Value::String(v) => AnyValue::Utf8(v),
            Value::Date(v) => AnyValue::Object(v),
            Value::Time(v) => AnyValue::Object(v),
            Value::DateTime(v) => AnyValue::Object(v),
            Value::Decimal(v) => AnyValue::Object(v),
            Value::Uuid(v) => AnyValue::Object(v),
            Value::Bytes(v) => AnyValue::Object(v),
            Value::Null => AnyValue::Null,
        }
    }
}

impl_value_from!(bool, Bool);
impl_value_from!(String, String);
impl_value_from!(u8, U8);
impl_value_from!(u16, U16);
impl_value_from!(u32, U32);
impl_value_from!(u64, U64);
impl_value_from!(i8, I8);
impl_value_from!(i16, I16);
impl_value_from!(i32, I32);
impl_value_from!(i64, I64);
impl_value_from!(f32, F32);
impl_value_from!(f64, F64);
impl_value_from!(Date, Date);
impl_value_from!(chrono::NaiveDate, Date, Date);
impl_value_from!(Time, Time);
impl_value_from!(chrono::NaiveTime, Time, Time);
impl_value_from!(DateTime, DateTime);
impl_value_from!(chrono::NaiveDateTime, DateTime, DateTime);
impl_value_from!(Decimal, Decimal);
impl_value_from!(rust_decimal::Decimal, Decimal, Decimal);
impl_value_from!(Uuid, Uuid);
impl_value_from!(uuid::Uuid, Uuid, Uuid);
impl_value_from!(Bytes, Bytes);
impl_value_from!(Vec<u8>, Bytes, Bytes);

impl_value_from!(Option<bool>, Bool);
impl_value_from!(Option<String>, String);
impl_value_from!(Option<u8>, U8);
impl_value_from!(Option<u16>, U16);
impl_value_from!(Option<u32>, U32);
impl_value_from!(Option<u64>, U64);
impl_value_from!(Option<i8>, I8);
impl_value_from!(Option<i16>, I16);
impl_value_from!(Option<i32>, I32);
impl_value_from!(Option<i64>, I64);
impl_value_from!(Option<f32>, F32);
impl_value_from!(Option<f64>, F64);
impl_value_from!(Option<Date>, Date);
impl_value_from!(Option<chrono::NaiveDate>, Date, Date);
impl_value_from!(Option<Time>, Time);
impl_value_from!(Option<chrono::NaiveTime>, Time, Time);
impl_value_from!(Option<DateTime>, DateTime);
impl_value_from!(Option<chrono::NaiveDateTime>, DateTime, DateTime);
impl_value_from!(Option<Decimal>, Decimal);
impl_value_from!(Option<rust_decimal::Decimal>, Decimal, Decimal);
impl_value_from!(Option<Uuid>, Uuid);
impl_value_from!(Option<uuid::Uuid>, Uuid, Uuid);
impl_value_from!(Option<Bytes>, Bytes);
impl_value_from!(Option<Vec<u8>>, Bytes, Bytes);

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::String(v.to_owned())
    }
}

impl From<Option<&str>> for Value {
    fn from(ov: Option<&str>) -> Self {
        match ov {
            Some(v) => Value::String(v.to_owned()),
            None => Value::Null,
        }
    }
}

impl_try_from_value!(Bool, bool, "bool");
impl_try_from_value!(String, String, "String");
impl_try_from_value!(U8, u8, "u8");
impl_try_from_value!(U16, u16, "u16");
impl_try_from_value!(U32, u32, "u32");
impl_try_from_value!(U64, u64, "u64");
impl_try_from_value!(I8, i8, "i8");
impl_try_from_value!(I16, i16, "i16");
impl_try_from_value!(I32, i32, "i32");
impl_try_from_value!(I64, i64, "i64");
impl_try_from_value!(F32, f32, "f32");
impl_try_from_value!(F64, f64, "f64");
impl_try_from_value!(Date, Date, "Date");
impl_try_from_value!(Time, Time, "Time");
impl_try_from_value!(DateTime, DateTime, "DateTime");
impl_try_from_value!(Decimal, Decimal, "Decimal");
impl_try_from_value!(Uuid, Uuid, "Uuid");
impl_try_from_value!(Bytes, Bytes, "Bytes");

impl_try_from_value!(Bool, Option<bool>, "Option<bool>");
impl_try_from_value!(String, Option<String>, "Option<String>");
impl_try_from_value!(U8, Option<u8>, "Option<u8>");
impl_try_from_value!(U16, Option<u16>, "Option<u16>");
impl_try_from_value!(U32, Option<u32>, "Option<u32>");
impl_try_from_value!(U64, Option<u64>, "Option<u64>");
impl_try_from_value!(I8, Option<i8>, "Option<i8>");
impl_try_from_value!(I16, Option<i16>, "Option<i16>");
impl_try_from_value!(I32, Option<i32>, "Option<i32>");
impl_try_from_value!(I64, Option<i64>, "Option<i64>");
impl_try_from_value!(F32, Option<f32>, "Option<f32>");
impl_try_from_value!(F64, Option<f64>, "Option<f64>");
impl_try_from_value!(Date, Option<Date>, "Option<Date>");
impl_try_from_value!(Time, Option<Time>, "Option<Time>");
impl_try_from_value!(DateTime, Option<DateTime>, "Option<DateTime>");
impl_try_from_value!(Decimal, Option<Decimal>, "Option<Decimal>");
impl_try_from_value!(Uuid, Option<Uuid>, "Option<Uuid>");
impl_try_from_value!(Bytes, Option<Bytes>, "Option<Bytes>");

#[cfg(test)]
mod test_value {

    use crate::{value, Date, Decimal, Uuid, Value, ValueType};
    use chrono::{NaiveDate, NaiveTime};
    use rust_decimal::Decimal as RDecimal;
    use uuid::Uuid as UUuid;

    #[test]
    fn test_conversion() {
        let v = 123;
        let i = Value::from(v);
        assert_eq!(ValueType::from(&i), ValueType::I32);

        let v = value!(123);
        let i = i32::try_from(v);
        assert!(i.is_ok());

        let v = value!(Some(123));
        let i = Option::<i32>::try_from(v);
        assert!(i.is_ok());

        let v = value!(None::<i32>);
        let i = Option::<i32>::try_from(v);
        assert!(i.is_ok());
    }

    #[test]
    fn test_custom_type_conversion() {
        // test case: external crate type (rust_decimal::Decimal)
        let v = RDecimal::new(123, 0);
        let v = Some(Decimal(v));
        let v: Value = v.into();
        assert_eq!(ValueType::from(&v), ValueType::Decimal);

        // test case: external crate type (uuid::Uuid)
        let v = UUuid::new_v4();
        let v = Some(Uuid(v));
        let v: Value = v.into();
        assert_eq!(ValueType::from(&v), ValueType::Uuid);

        // test case: external crate type (chrono::NaiveTime)
        let v = NaiveTime::from_hms(1, 2, 3);
        let v = Some(v);
        let v: Value = v.into();
        assert_eq!(ValueType::from(&v), ValueType::Time);

        // test case: custom type (crate::Date), which is a wrapper of chrono::NaiveDate
        let v = NaiveDate::from_ymd(2019, 1, 1);
        let v = Some(Date(v));
        let v: Value = v.into();
        assert_eq!(ValueType::from(&v), ValueType::Date);
    }
}
