//! Sql builder

use std::str::FromStr;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use sea_query::Value as SValue;

use super::sv_2_v;
use crate::{
    core::{DAYS19700101, NANO10E9},
    Decimal, SqlError, SqlResult, Uuid, Value, ValueType,
};

#[derive(Debug, Clone)]
pub enum SqlBuilder {
    Mysql,
    Postgres,
    Sqlite,
}

impl std::fmt::Display for SqlBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mysql => write!(f, "mysql"),
            Self::Postgres => write!(f, "postgres"),
            Self::Sqlite => write!(f, "sqlite"),
        }
    }
}

impl FromStr for SqlBuilder {
    type Err = SqlError;

    fn from_str(s: &str) -> SqlResult<Self> {
        match s {
            "mysql" | "m" => Ok(SqlBuilder::Mysql),
            "postgres" | "p" => Ok(SqlBuilder::Postgres),
            "sqlite" | "s" => Ok(SqlBuilder::Sqlite),
            _ => Err(SqlError::new_common_error(format!(
                "{} is not a valid sql builder",
                s
            ))),
        }
    }
}

/// Don't use it in general type conversion, use `try_from_value_to_svalue` instead
impl From<Value> for SValue {
    fn from(v: Value) -> Self {
        match v {
            Value::Bool(v) => SValue::Bool(Some(v)),
            Value::U8(v) => SValue::TinyUnsigned(Some(v)),
            Value::U16(v) => SValue::SmallUnsigned(Some(v)),
            Value::U32(v) => SValue::Unsigned(Some(v)),
            Value::U64(v) => SValue::BigUnsigned(Some(v)),
            Value::I8(v) => SValue::TinyInt(Some(v)),
            Value::I16(v) => SValue::SmallInt(Some(v)),
            Value::I32(v) => SValue::Int(Some(v)),
            Value::I64(v) => SValue::BigInt(Some(v)),
            Value::F32(v) => SValue::Float(Some(v)),
            Value::F64(v) => SValue::Double(Some(v)),
            Value::String(v) => SValue::String(Some(Box::new(v))),
            Value::Date(v) => {
                let v = v + DAYS19700101;
                let d = NaiveDate::from_num_days_from_ce(v);
                SValue::ChronoDate(Some(Box::new(d)))
            }
            Value::Time(v) => {
                let v = (v / NANO10E9) as u32;
                let t = NaiveTime::from_num_seconds_from_midnight(v, 0);
                SValue::ChronoTime(Some(Box::new(t)))
            }
            Value::DateTime(v) => {
                let v = v / NANO10E9;
                let dt = NaiveDateTime::from_timestamp(v, 0);
                SValue::ChronoDateTime(Some(Box::new(dt)))
            }
            Value::Decimal(v) => SValue::Decimal(Some(Box::new(v.0))),
            Value::Uuid(v) => SValue::Uuid(Some(Box::new(v.0))),
            Value::Bytes(v) => SValue::Bytes(Some(Box::new(v.0))),
            // Temporary workaround
            Value::Null => SValue::Bool(None),
        }
    }
}

impl From<&Value> for SValue {
    fn from(v: &Value) -> Self {
        v.clone().into()
    }
}

/// Type conversion: from polars DataType to SeqQuery Value
fn from_data_type_to_null_svalue(dtype: &ValueType) -> SValue {
    match dtype {
        ValueType::Bool => SValue::Bool(None),
        ValueType::U8 => SValue::TinyUnsigned(None),
        ValueType::U16 => SValue::SmallUnsigned(None),
        ValueType::U32 => SValue::Unsigned(None),
        ValueType::U64 => SValue::BigUnsigned(None),
        ValueType::I8 => SValue::TinyInt(None),
        ValueType::I16 => SValue::SmallInt(None),
        ValueType::I32 => SValue::Int(None),
        ValueType::I64 => SValue::BigInt(None),
        ValueType::F32 => SValue::Float(None),
        ValueType::F64 => SValue::Double(None),
        ValueType::String => SValue::String(None),
        ValueType::Date => SValue::ChronoDate(None),
        ValueType::Time => SValue::ChronoTime(None),
        ValueType::DateTime => SValue::ChronoDateTime(None),
        ValueType::Decimal => SValue::Decimal(None),
        ValueType::Uuid => SValue::Uuid(None),
        ValueType::Bytes => SValue::Bytes(None),
        _ => panic!("unsupported data type conversion"),
    }
}

/// Type conversion: from Value to `sea-query` Value
pub(crate) fn try_from_value_to_svalue(
    value: Value,
    dtype: &ValueType,
    nullable: bool,
) -> SqlResult<SValue> {
    match value {
        Value::Bool(v) => Ok(SValue::Bool(Some(v))),
        Value::U8(v) => Ok(SValue::TinyUnsigned(Some(v))),
        Value::U16(v) => Ok(SValue::SmallUnsigned(Some(v))),
        Value::U32(v) => Ok(SValue::Unsigned(Some(v))),
        Value::U64(v) => Ok(SValue::BigUnsigned(Some(v))),
        Value::I8(v) => Ok(SValue::TinyInt(Some(v))),
        Value::I16(v) => Ok(SValue::SmallInt(Some(v))),
        Value::I32(v) => Ok(SValue::Int(Some(v))),
        Value::I64(v) => Ok(SValue::BigInt(Some(v))),
        Value::F32(v) => Ok(SValue::Float(Some(v))),
        Value::F64(v) => Ok(SValue::Double(Some(v))),
        Value::String(v) => Ok(SValue::String(Some(Box::new(v)))),
        Value::Date(v) => {
            let v = v + DAYS19700101;
            let d = NaiveDate::from_num_days_from_ce(v);
            Ok(SValue::ChronoDate(Some(Box::new(d))))
        }
        Value::Time(v) => {
            let v = (v / NANO10E9) as u32;
            let t = NaiveTime::from_num_seconds_from_midnight(v, 0);
            Ok(SValue::ChronoTime(Some(Box::new(t))))
        }
        Value::DateTime(v) => {
            let v = v / NANO10E9;
            let dt = NaiveDateTime::from_timestamp(v, 0);
            Ok(SValue::ChronoDateTime(Some(Box::new(dt))))
        }
        Value::Decimal(v) => Ok(SValue::Decimal(Some(Box::new(v.0)))),
        Value::Uuid(v) => Ok(SValue::Uuid(Some(Box::new(v.0)))),
        Value::Bytes(v) => Ok(SValue::Bytes(Some(Box::new(v.0)))),
        Value::Null => {
            if nullable {
                Ok(from_data_type_to_null_svalue(dtype))
            } else {
                Err(SqlError::new_common_error(format!(
                    "convert {:?} to {:?} error",
                    value, dtype
                )))
            }
        }
    }
}

#[allow(dead_code)]
/// Type conversion: from `SeaQuery` Value to Value
pub(crate) fn from_svalue_to_value(svalue: SValue, nullable: bool) -> SqlResult<Value> {
    match svalue {
        SValue::Bool(ov) => sv_2_v!(ov, nullable),
        SValue::TinyInt(ov) => sv_2_v!(ov, nullable),
        SValue::SmallInt(ov) => sv_2_v!(ov, nullable),
        SValue::Int(ov) => sv_2_v!(ov, nullable),
        SValue::BigInt(ov) => sv_2_v!(ov, nullable),
        SValue::TinyUnsigned(ov) => sv_2_v!(ov, nullable),
        SValue::SmallUnsigned(ov) => sv_2_v!(ov, nullable),
        SValue::Unsigned(ov) => sv_2_v!(ov, nullable),
        SValue::BigUnsigned(ov) => sv_2_v!(ov, nullable),
        SValue::Float(ov) => sv_2_v!(ov, nullable),
        SValue::Double(ov) => sv_2_v!(ov, nullable),
        SValue::String(ov) => sv_2_v!(ov, String, nullable),
        SValue::ChronoDate(ov) => sv_2_v!(ov, NaiveDate, nullable),
        SValue::ChronoTime(ov) => sv_2_v!(ov, NaiveTime, nullable),
        SValue::ChronoDateTime(ov) => sv_2_v!(ov, NaiveDateTime, nullable),
        SValue::Decimal(ov) => sv_2_v!(ov, Decimal, nullable),
        SValue::Uuid(ov) => sv_2_v!(ov, Uuid, nullable),
        _ => Err(SqlError::new_common_error("unsupported type")),
    }
}
