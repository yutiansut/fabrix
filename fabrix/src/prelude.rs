//! Fabrix prelude

pub use crate::core::{
    DataFrame, Date, DateTime, Decimal, FieldInfo, Row, Series, Time, Uuid, Value, ValueType, D1,
    D2,
};

#[cfg(feature = "db")]
pub use crate::sources::db::sql_builder::adt;
#[cfg(feature = "db")]
pub use crate::sources::db::sql_executor::{ConnInfo, Executor as SqlExecutor};

#[cfg(feature = "file")]
pub use crate::sources::file::error::{XlError, XlResult};
