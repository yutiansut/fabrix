//! Fabrix sources: db error
//!
//! This module contains the error types for the database.

use std::error::Error as StdError;
use std::fmt::Display;

use nom::error::{ErrorKind, ParseError};
use thiserror::Error;

use crate::{CommonError, CoreError};

pub type SqlResult<T> = Result<T, SqlError>;

#[derive(Error, Debug)]
pub enum SqlError {
    #[error("common error {0}")]
    Common(CommonError),

    #[error(transparent)]
    CORE(#[from] CoreError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    SeqQuery(#[from] sea_query::error::Error),

    #[error("nom error {0}")]
    Nom(NomError),
}

impl SqlError {
    pub fn new_common_error<T>(msg: T) -> SqlError
    where
        T: Into<CommonError>,
    {
        SqlError::Common(msg.into())
    }

    pub fn turn_into_sqlx_decode_error(self) -> sqlx::Error {
        match self {
            SqlError::Sqlx(se) => se,
            _ => sqlx::Error::Decode(Box::new(SqlDecodeError::new("sql row decode error"))),
        }
    }
}

#[derive(Debug)]
pub struct NomError(pub(crate) String);

impl Display for NomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<(&'a str, ErrorKind)> for NomError {
    fn from(error: (&'a str, ErrorKind)) -> Self {
        NomError(format!("error code was: {:?}", error))
    }
}

impl<'a> ParseError<&'a str> for NomError {
    fn from_error_kind(_: &'a str, kind: ErrorKind) -> Self {
        NomError(format!("error code was: {:?}", kind))
    }

    fn append(_: &'a str, kind: ErrorKind, other: NomError) -> Self {
        NomError(format!("{:?}\nerror code was: {:?}", other, kind))
    }
}

#[derive(Debug)]
pub struct SqlDecodeError {
    pub err: String,
}

impl SqlDecodeError {
    pub fn new<T>(err: T) -> Self
    where
        T: AsRef<str>,
    {
        SqlDecodeError {
            err: err.as_ref().to_owned(),
        }
    }
}

impl Display for SqlDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl StdError for SqlDecodeError {}
