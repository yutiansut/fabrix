//! Fabrix Series
//!
//! This module contains the Series struct, which is used to store a series of same-typed values (nullable).
//!
//! Methods:
//! 1. new: auto gained by implemented `polars::prelude::NamedFrom` trait.
//! 1. from_integer
//! 1. from_range
//! 1. from_values
//! 1. from_values_default_name
//! 1. empty_series_from_field
//! 1. rechunk
//! 1. name
//! 1. rename
//! 1. data
//! 1. len
//! 1. dtype
//! 1. field
//! 1. is_empty
//! 1. has_null
//! 1. head
//! 1. tail
//! 1. get
//! 1. take
//! 1. slice
//! 1. contains
//! 1. find_index
//! 1. find_indices
//! 1. drop_nulls
//! 1. concat
//! 1. split
//! 1. push
//! 1. insert
//! 1. insert_many
//! 1. pop
//! 1. remove
//! 1. remove_slice

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use itertools::Itertools;
use polars::prelude::{
    BooleanChunked, BooleanType, DateChunked, DatetimeChunked, Float32Chunked, Float32Type,
    Float64Chunked, Float64Type, Int16Chunked, Int16Type, Int32Chunked, Int32Type, Int64Chunked,
    Int64Type, Int8Chunked, Int8Type, ObjectChunked, TakeRandom, TakeRandomUtf8, TimeChunked,
    UInt16Chunked, UInt16Type, UInt32Chunked, UInt32Type, UInt64Chunked, UInt64Type, UInt8Chunked,
    UInt8Type, Utf8Chunked, Utf8Type,
};
use polars::prelude::{
    DataType, Field, IntoSeries, NamedFrom, NewChunkedArray, Series as PolarsSeries, TimeUnit,
};

use super::{
    chunked_array_from_values, cis_err, impl_named_from_owned, impl_named_from_ref, oob_err,
    s_fn_next, sc_fn_next, sfv, si, sii, tms_err, FieldInfo, ObjectTypeBytes, ObjectTypeDecimal,
    ObjectTypeUuid, Stepper, BYTES, DAYS19700101, DECIMAL, IDX, NANO10E9, UUID,
};
use crate::{series, value, Bytes, CoreResult, Decimal, Uuid, Value, ValueType};

// ================================================================================================
// Series constructors
// ================================================================================================

impl_named_from_ref!([bool], BooleanType, from_slice);
impl_named_from_ref!([Option<bool>], BooleanType, from_slice_options);

impl_named_from_ref!([i8], Int8Type, from_slice);
impl_named_from_owned!(Vec<i8>, Int8Type);
impl_named_from_ref!([i16], Int16Type, from_slice);
impl_named_from_owned!(Vec<i16>, Int16Type);
impl_named_from_ref!([i32], Int32Type, from_slice);
impl_named_from_owned!(Vec<i32>, Int32Type);
impl_named_from_ref!([i64], Int64Type, from_slice);
impl_named_from_owned!(Vec<i64>, Int64Type);
impl_named_from_ref!([Option<i8>], Int8Type, from_slice_options);
impl_named_from_ref!([Option<i16>], Int16Type, from_slice_options);
impl_named_from_ref!([Option<i32>], Int32Type, from_slice_options);
impl_named_from_ref!([Option<i64>], Int64Type, from_slice_options);

impl_named_from_ref!([u8], UInt8Type, from_slice);
impl_named_from_owned!(Vec<u8>, UInt8Type);
impl_named_from_ref!([u16], UInt16Type, from_slice);
impl_named_from_owned!(Vec<u16>, UInt16Type);
impl_named_from_ref!([u32], UInt32Type, from_slice);
impl_named_from_owned!(Vec<u32>, UInt32Type);
impl_named_from_ref!([u64], UInt64Type, from_slice);
impl_named_from_owned!(Vec<u64>, UInt64Type);
impl_named_from_ref!([Option<u8>], UInt8Type, from_slice_options);
impl_named_from_ref!([Option<u16>], UInt16Type, from_slice_options);
impl_named_from_ref!([Option<u32>], UInt32Type, from_slice_options);
impl_named_from_ref!([Option<u64>], UInt64Type, from_slice_options);

impl_named_from_ref!([f32], Float32Type, from_slice);
impl_named_from_owned!(Vec<f32>, Float32Type);
impl_named_from_ref!([f64], Float64Type, from_slice);
impl_named_from_owned!(Vec<f64>, Float64Type);
impl_named_from_ref!([Option<f32>], Float32Type, from_slice_options);
impl_named_from_ref!([Option<f64>], Float64Type, from_slice_options);

impl_named_from_ref!([String], Utf8Type, from_slice);
impl_named_from_ref!([Option<String>], Utf8Type, from_slice_options);

impl<'a, T: AsRef<[&'a str]>> NamedFrom<T, [&'a str]> for Series {
    fn new(name: &str, v: T) -> Self {
        let polars_series = Utf8Chunked::from_slice(name, v.as_ref()).into_series();
        Series(polars_series)
    }
}
impl<'a, T: AsRef<[Option<&'a str>]>> NamedFrom<T, [Option<&'a str>]> for Series {
    fn new(name: &str, v: T) -> Self {
        let polars_series = Utf8Chunked::from_slice_options(name, v.as_ref()).into_series();
        Series(polars_series)
    }
}

impl<T: AsRef<[NaiveDate]>> NamedFrom<T, [NaiveDate]> for Series {
    fn new(name: &str, v: T) -> Self {
        let v = v
            .as_ref()
            .iter()
            .map(|i| i.num_days_from_ce() - DAYS19700101)
            .collect::<Vec<_>>();
        let ca = Int32Chunked::from_slice(name, v.as_ref());
        let polars_series = DateChunked::from(ca).into_series();
        Series(polars_series)
    }
}

impl<T: AsRef<[Option<NaiveDate>]>> NamedFrom<T, [Option<NaiveDate>]> for Series {
    fn new(name: &str, v: T) -> Self {
        let v = v
            .as_ref()
            .iter()
            .map(|oi| oi.map(|i| i.num_days_from_ce() - DAYS19700101))
            .collect::<Vec<_>>();
        let ca = Int32Chunked::from_slice_options(name, v.as_ref());
        let polars_series = DateChunked::from(ca).into_series();
        Series(polars_series)
    }
}

impl<T: AsRef<[NaiveTime]>> NamedFrom<T, [NaiveTime]> for Series {
    fn new(name: &str, v: T) -> Self {
        let v = v
            .as_ref()
            .iter()
            .map(|i| i.num_seconds_from_midnight() as i64 * NANO10E9)
            .collect::<Vec<_>>();
        let ca = Int64Chunked::from_slice(name, v.as_ref());
        let polars_series = TimeChunked::from(ca).into_series();
        Series(polars_series)
    }
}

impl<T: AsRef<[Option<NaiveTime>]>> NamedFrom<T, [Option<NaiveTime>]> for Series {
    fn new(name: &str, v: T) -> Self {
        let v = v
            .as_ref()
            .iter()
            .map(|oi| oi.map(|i| i.num_seconds_from_midnight() as i64 * NANO10E9))
            .collect::<Vec<_>>();
        let ca = Int64Chunked::from_slice_options(name, v.as_ref());
        let polars_series = TimeChunked::from(ca).into_series();
        Series(polars_series)
    }
}

impl<T: AsRef<[NaiveDateTime]>> NamedFrom<T, [NaiveDateTime]> for Series {
    fn new(name: &str, v: T) -> Self {
        let v = v
            .as_ref()
            .iter()
            .map(|i| i.timestamp_nanos())
            .collect::<Vec<_>>();
        let ca = Int64Chunked::from_slice(name, v.as_ref());
        let polars_series = ca.into_datetime(TimeUnit::Nanoseconds, None).into_series();
        Series(polars_series)
    }
}

impl<T: AsRef<[Option<NaiveDateTime>]>> NamedFrom<T, [Option<NaiveDateTime>]> for Series {
    fn new(name: &str, v: T) -> Self {
        let v = v
            .as_ref()
            .iter()
            .map(|oi| oi.map(|i| i.timestamp_nanos()))
            .collect::<Vec<_>>();
        let ca = Int64Chunked::from_slice_options(name, v.as_ref());
        let polars_series = ca.into_datetime(TimeUnit::Nanoseconds, None).into_series();
        Series(polars_series)
    }
}

impl_named_from_ref!([Decimal], ObjectTypeDecimal, from_slice);
impl_named_from_ref!([Option<Decimal>], ObjectTypeDecimal, from_slice_options);

impl_named_from_ref!([Uuid], ObjectTypeUuid, from_slice);
impl_named_from_ref!([Option<Uuid>], ObjectTypeUuid, from_slice_options);

// ================================================================================================
// Series
// ================================================================================================

/// Series is a data structure used in Fabrix crate, it wrapped `polars` Series and provides
/// additional customized functionalities
#[derive(Clone)]
pub struct Series(pub PolarsSeries);

impl Series {
    /// new Series from an integer type (Rust standard type)
    pub fn from_integer<I, S>(value: I, name: S) -> CoreResult<Self>
    where
        I: Into<Value> + Copy,
        S: AsRef<str>,
    {
        from_integer(value.into(), name.as_ref())
    }

    /// new Series from an integer type (Rust standard type)
    pub fn from_integer_default_name<I>(value: I) -> CoreResult<Self>
    where
        I: Into<Value> + Copy,
    {
        from_integer(value.into(), IDX)
    }

    /// new Series from a range
    pub fn from_range<I, S>(range: [I; 2], name: S) -> CoreResult<Self>
    where
        I: Into<Value> + Copy,
        S: AsRef<str>,
    {
        from_range([range[0].into(), range[1].into()], name.as_ref())
    }

    /// new Series from a range
    pub fn from_range_default_name<I>(range: [I; 2]) -> CoreResult<Self>
    where
        I: Into<Value> + Copy,
    {
        from_range([range[0].into(), range[1].into()], IDX)
    }

    /// new Series from Vec<Value> and name
    pub fn from_values<S>(values: Vec<Value>, name: S, nullable: bool) -> CoreResult<Self>
    where
        S: AsRef<str>,
    {
        from_values(values, name.as_ref(), nullable)
    }

    /// new Series from Vec<Value>
    pub fn from_values_default_name(values: Vec<Value>, nullable: bool) -> CoreResult<Self> {
        from_values(values, IDX, nullable)
    }

    /// new empty Series from field
    pub fn empty_series_from_field(field: &Field, nullable: bool) -> CoreResult<Self> {
        empty_series_from_field(field, nullable)
    }

    // TODO:
    // pub fn from_chunked_array<A>(array: ChunkedArray<A>) -> CoreResult<Self> {
    //     Ok(Self(array.into_series()))
    // }

    /// rechunk: aggregate all chunks to a contiguous array of memory
    pub fn rechunk(&mut self) {
        self.0 = self.0.rechunk();
    }

    /// get Series' name
    pub fn name(&self) -> &str {
        self.0.name()
    }

    /// rename Series
    pub fn rename<S>(&mut self, name: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.0.rename(name.as_ref());
        self
    }

    /// show data
    pub fn data(&self) -> &PolarsSeries {
        &self.0
    }

    /// show data length
    pub fn len(&self) -> usize {
        self.data().len()
    }

    /// show Series type
    pub fn dtype(&self) -> &ValueType {
        self.0.dtype().into()
    }

    /// get series field
    pub fn field(&self) -> FieldInfo {
        FieldInfo::new(self.0.name().to_owned(), self.0.dtype().into())
    }

    /// check whether the series is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// check if contains null value
    /// WARNING: object column will cause panic, since `polars` hasn't implemented yet
    pub fn has_null(&self) -> bool {
        !self.0.is_not_null().all()
    }

    /// head, if length is `None`, return a series only contains the first element
    pub fn head(&self, length: Option<usize>) -> CoreResult<Series> {
        let len = self.len();

        match length {
            Some(l) => {
                if l >= self.len() {
                    Err(oob_err(l, len))
                } else {
                    Ok(self.0.head(length).into())
                }
            }
            None => Ok(self.0.head(Some(1)).into()),
        }
    }

    /// tail, if length is `None`, return a series only contains the last element
    pub fn tail(&self, length: Option<usize>) -> CoreResult<Series> {
        let len = self.len();

        match length {
            Some(l) => {
                if l >= len {
                    Err(oob_err(l, len))
                } else {
                    Ok(self.0.tail(length).into())
                }
            }
            None => Ok(self.0.tail(Some(1)).into()),
        }
    }

    /// get a cloned value by idx
    pub fn get(&self, idx: usize) -> CoreResult<Value> {
        let len = self.len();

        if idx >= len {
            Err(oob_err(idx, len))
        } else {
            let v = self.0.get(idx);
            Ok(value!(v))
        }
    }

    /// take a cloned slice by an indices array
    pub fn take(&self, indices: &[usize]) -> CoreResult<Series> {
        let mut iter = indices.iter().copied();
        Ok(Series(self.0.take_iter(&mut iter)?))
    }

    /// slice the Series
    #[must_use]
    pub fn slice(&self, offset: i64, length: usize) -> Series {
        self.0.slice(offset, length).into()
    }

    /// check Series whether contains a value (`self.into_iter` is not zero copy)
    pub fn contains(&self, val: &Value) -> bool {
        self.into_iter().contains(val)
    }

    /// find idx by a Value (`self.into_iter` is not zero copy)
    pub fn find_index(&self, val: &Value) -> Option<usize> {
        self.into_iter().position(|ref e| e == val)
    }

    /// find idx vector by a Series (`self.into_iter` is not zero copy)
    pub fn find_indices(&self, series: &Series) -> Vec<usize> {
        self.into_iter()
            .enumerate()
            .fold(vec![], |mut accum, (idx, e)| {
                if series.contains(&e) {
                    accum.push(idx);
                }
                accum
            })
    }

    /// drop nulls
    pub fn drop_nulls(&mut self) -> &mut Self {
        self.0 = self.0.drop_nulls();
        self
    }

    /// concat another series to current series
    pub fn concat(&mut self, series: Series) -> CoreResult<&mut Self> {
        self.0.append(&series.0)?;
        Ok(self)
    }

    /// split into two series
    pub fn split(&self, idx: usize) -> CoreResult<(Series, Series)> {
        let len = self.len();

        if idx >= len {
            Err(oob_err(idx, len))
        } else {
            let (len1, len2) = (idx, len - idx);
            Ok((self.slice(0, len1), self.slice(idx as i64, len2)))
        }
    }

    /// push a value at the end of the series, self mutation
    pub fn push(&mut self, value: Value) -> CoreResult<&mut Self> {
        let s = from_values(vec![value], IDX, true)?;
        self.concat(s)?;
        Ok(self)
    }

    /// insert a value into the series by idx, self mutation
    pub fn insert(&mut self, idx: usize, value: Value) -> CoreResult<&mut Self> {
        let (mut s1, s2) = self.split(idx)?;

        s1.push(value)?.concat(s2)?;
        *self = s1;

        Ok(self)
    }

    /// insert a series at a specified idx, self mutation
    pub fn insert_many(&mut self, idx: usize, series: Series) -> CoreResult<&mut Self> {
        let (mut s1, s2) = self.split(idx)?;

        s1.concat(series)?.concat(s2)?;
        *self = s1;

        Ok(self)
    }

    /// pop the last element from the series, self mutation
    pub fn pop(&mut self) -> CoreResult<&mut Self> {
        let len = self.len();
        if len == 0 {
            return Err(cis_err("series"));
        }

        *self = self.slice(0, len - 1);

        Ok(self)
    }

    /// remove a value from the series, self mutation
    pub fn remove(&mut self, idx: usize) -> CoreResult<&mut Self> {
        let len = self.len();
        if idx >= len {
            return Err(oob_err(idx, len));
        }
        let (mut s1, s2) = (self.slice(0, idx), self.slice(idx as i64 + 1, len));

        s1.concat(s2)?;
        *self = s1;

        Ok(self)
    }

    /// remove a slice from the series, self mutation
    pub fn remove_slice(&mut self, offset: i64, length: usize) -> CoreResult<&mut Self> {
        let len = self.len();
        let offset = if offset >= 0 {
            offset
        } else {
            len as i64 + offset
        };
        let (mut s1, s2) = (
            self.slice(0, offset as usize),
            self.slice(offset + length as i64, len),
        );

        s1.concat(s2)?;
        *self = s1;

        Ok(self)
    }

    // TODO: apply methods

    pub fn apply() {
        unimplemented!()
    }

    pub fn apply_at_idx() {
        unimplemented!()
    }
}

/// new Series from an AnyValue (integer specific)
fn from_integer(val: Value, name: &str) -> CoreResult<Series> {
    match val {
        Value::U8(v) => Ok(series!(name => (0..v).collect::<Vec<_>>())),
        Value::U16(v) => Ok(series!(name => (0..v).collect::<Vec<_>>())),
        Value::U32(v) => Ok(series!(name => (0..v).collect::<Vec<_>>())),
        Value::U64(v) => Ok(series!(name => (0..v).collect::<Vec<_>>())),
        Value::I8(v) => Ok(series!(name => (0..v).collect::<Vec<_>>())),
        Value::I16(v) => Ok(series!(name => (0..v).collect::<Vec<_>>())),
        Value::I32(v) => Ok(series!(name => (0..v).collect::<Vec<_>>())),
        Value::I64(v) => Ok(series!(name => (0..v).collect::<Vec<_>>())),
        _ => Err(tms_err("val")),
    }
}

/// new Series from a range of AnyValue (integer specific)
fn from_range(rng: [Value; 2], name: &str) -> CoreResult<Series> {
    let [r0, r1] = rng;
    match [r0, r1] {
        [Value::U8(s), Value::U8(e)] => Ok(series!(name => (s..e).collect::<Vec<_>>())),
        [Value::U16(s), Value::U16(e)] => Ok(series!(name => (s..e).collect::<Vec<_>>())),
        [Value::U32(s), Value::U32(e)] => Ok(series!(name => (s..e).collect::<Vec<_>>())),
        [Value::U64(s), Value::U64(e)] => Ok(series!(name => (s..e).collect::<Vec<_>>())),
        [Value::I8(s), Value::I8(e)] => Ok(series!(name => (s..e).collect::<Vec<_>>())),
        [Value::I16(s), Value::I16(e)] => Ok(series!(name => (s..e).collect::<Vec<_>>())),
        [Value::I32(s), Value::I32(e)] => Ok(series!(name => (s..e).collect::<Vec<_>>())),
        [Value::I64(s), Value::I64(e)] => Ok(series!(name => (s..e).collect::<Vec<_>>())),
        _ => Err(tms_err("rng")),
    }
}

// Simple conversion
impl From<PolarsSeries> for Series {
    fn from(s: PolarsSeries) -> Self {
        Series(s)
    }
}

// Simple conversion
impl From<Series> for PolarsSeries {
    fn from(s: Series) -> Self {
        s.0
    }
}

impl AsRef<PolarsSeries> for Series {
    fn as_ref(&self) -> &PolarsSeries {
        &self.0
    }
}

/// Series from values, series type is determined by the first not-null value,
/// if the who vectors are null then use u64 as the default type.
///
/// if nullable is true, mismatched types will be converted to null.
fn from_values(values: Vec<Value>, name: &str, nullable: bool) -> CoreResult<Series> {
    if values.is_empty() {
        return Err(cis_err("values"));
    }

    // iterate until get the first non-null value
    // let opt_dtype = values.iter().skip_while(|v| v.is_null()).next();
    let opt_dtype = values.iter().find(|v| !v.is_null());

    match opt_dtype {
        Some(v) => match ValueType::from(v) {
            ValueType::Bool => sfv!(nullable; name, values; bool, BooleanType),
            ValueType::String => sfv!(nullable; name, values; String, Utf8Type),
            ValueType::U8 => sfv!(nullable; name, values; u8, UInt8Type),
            ValueType::U16 => sfv!(nullable; name, values; u16, UInt16Type),
            ValueType::U32 => sfv!(nullable; name, values; u32, UInt32Type),
            ValueType::U64 => sfv!(nullable; name, values; u64, UInt64Type),
            ValueType::I8 => sfv!(nullable; name, values; i8, Int8Type),
            ValueType::I16 => sfv!(nullable; name, values; i16, Int16Type),
            ValueType::I32 => sfv!(nullable; name, values; i32, Int32Type),
            ValueType::I64 => sfv!(nullable; name, values; i64, Int64Type),
            ValueType::F32 => sfv!(nullable; name, values; f32, Float32Type),
            ValueType::F64 => sfv!(nullable; name, values; f64, Float64Type),
            ValueType::Date => {
                dbg!(&values);
                let ca = if nullable {
                    chunked_array_from_values!(name, values; i32, Int32Type)
                } else {
                    chunked_array_from_values!(name, values; Option<i32>, Int32Type)
                };
                Ok(Series(ca.into_date().into_series()))
            }
            ValueType::Time => {
                let ca = if nullable {
                    chunked_array_from_values!(name, values; i64, Int64Type)
                } else {
                    chunked_array_from_values!(name, values; Option<i64>, Int64Type)
                };
                Ok(Series(ca.into_time().into_series()))
            }
            ValueType::DateTime => {
                let ca = if nullable {
                    chunked_array_from_values!(name, values; i64, Int64Type)
                } else {
                    chunked_array_from_values!(name, values; Option<i64>, Int64Type)
                };
                let s = ca.into_datetime(TimeUnit::Nanoseconds, None).into_series();
                Ok(Series(s))
            }
            ValueType::Decimal => sfv!(nullable; name, values; Decimal, ObjectTypeDecimal),
            ValueType::Uuid => sfv!(nullable; name, values; Uuid, ObjectTypeUuid),
            ValueType::Bytes => sfv!(nullable; name, values; Bytes, ObjectTypeBytes),
            ValueType::Null => Ok(Series::from_integer(values.len() as u64, name)?),
        },
        None => Ok(Series::from_integer(values.len() as u64, name)?),
    }
}

/// empty series from field
fn empty_series_from_field(field: &Field, nullable: bool) -> CoreResult<Series> {
    match field.data_type() {
        DataType::Boolean => sfv!(nullable; field.name(); bool, BooleanType),
        DataType::Utf8 => sfv!(nullable; field.name(); String, Utf8Type),
        DataType::UInt8 => sfv!(nullable; field.name(); u8, UInt8Type),
        DataType::UInt16 => sfv!(nullable; field.name(); u16, UInt16Type),
        DataType::UInt32 => sfv!(nullable; field.name(); u32, UInt32Type),
        DataType::UInt64 => sfv!(nullable; field.name(); u64, UInt64Type),
        DataType::Int8 => sfv!(nullable; field.name(); i8, Int8Type),
        DataType::Int16 => sfv!(nullable; field.name(); i16, Int16Type),
        DataType::Int32 => sfv!(nullable; field.name(); i32, Int32Type),
        DataType::Int64 => sfv!(nullable; field.name(); i64, Int64Type),
        DataType::Float32 => sfv!(nullable; field.name(); f32, Float32Type),
        DataType::Float64 => sfv!(nullable; field.name(); f64, Float64Type),
        DataType::Date => {
            let ca = Int32Chunked::from_vec(field.name(), vec![]);
            let s = DateChunked::from(ca).into_series();
            Ok(Series(s))
        }
        DataType::Time => {
            let ca = Int64Chunked::from_vec(field.name(), vec![]);
            let s = TimeChunked::from(ca).into_series();
            Ok(Series(s))
        }
        DataType::Datetime(TimeUnit::Nanoseconds, None) => {
            let ca = Int64Chunked::from_vec(field.name(), vec![]);
            let s = ca.into_datetime(TimeUnit::Nanoseconds, None).into_series();
            Ok(Series(s))
        }
        DataType::Object(DECIMAL) => sfv!(nullable; field.name(); Decimal, ObjectTypeDecimal),
        DataType::Object(UUID) => sfv!(nullable; field.name(); Uuid, ObjectTypeUuid),
        DataType::Object(BYTES) => sfv!(nullable; field.name(); Bytes, ObjectTypeBytes),
        DataType::Null => sfv!(nullable; field.name(); u64, UInt64Type),
        _ => unimplemented!(),
    }
}

// ================================================================================================
// IntoIterator impls
// ================================================================================================

/// Series IntoIterator implementation
impl IntoIterator for Series {
    type Item = Value;
    type IntoIter = SeriesIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        match self.dtype() {
            ValueType::Bool => sii!(self.0.bool(), Bool),
            ValueType::U8 => sii!(self.0.u8(), U8),
            ValueType::U16 => sii!(self.0.u16(), U16),
            ValueType::U32 => sii!(self.0.u32(), U32),
            ValueType::U64 => sii!(self.0.u64(), U64),
            ValueType::I8 => sii!(self.0.i8(), I8),
            ValueType::I16 => sii!(self.0.i16(), I16),
            ValueType::I32 => sii!(self.0.i32(), I32),
            ValueType::I64 => sii!(self.0.i64(), I64),
            ValueType::F32 => sii!(self.0.f32(), F32),
            ValueType::F64 => sii!(self.0.f64(), F64),
            ValueType::String => sii!(self.0.utf8(), String),
            ValueType::Date => sii!(self.0.date(), Date),
            ValueType::Time => sii!(self.0.time(), Time),
            ValueType::DateTime => sii!(self.0.datetime(), DateTime),
            ValueType::Decimal => sii!(self.0.as_any(), Decimal, Decimal),
            ValueType::Uuid => sii!(self.0.as_any(), Uuid, Uuid),
            ValueType::Bytes => sii!(self.0.as_any(), Bytes, Bytes),
            ValueType::Null => panic!("Null value series"),
        }
    }
}

/// IntoIterator
pub enum SeriesIntoIterator {
    Id(UInt64Chunked, Stepper),
    Bool(BooleanChunked, Stepper),
    U8(UInt8Chunked, Stepper),
    U16(UInt16Chunked, Stepper),
    U32(UInt32Chunked, Stepper),
    U64(UInt64Chunked, Stepper),
    I8(Int8Chunked, Stepper),
    I16(Int16Chunked, Stepper),
    I32(Int32Chunked, Stepper),
    I64(Int64Chunked, Stepper),
    F32(Float32Chunked, Stepper),
    F64(Float64Chunked, Stepper),
    String(Utf8Chunked, Stepper),
    Date(DateChunked, Stepper),
    Time(TimeChunked, Stepper),
    DateTime(DatetimeChunked, Stepper),
    Decimal(ObjectChunked<Decimal>, Stepper),
    Uuid(ObjectChunked<Uuid>, Stepper),
    Bytes(ObjectChunked<Bytes>, Stepper),
}

impl Iterator for SeriesIntoIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SeriesIntoIterator::Id(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::Bool(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::U8(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::U16(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::U32(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::U64(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::I8(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::I16(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::I32(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::I64(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::F32(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::F64(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::String(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::Date(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::Time(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::DateTime(arr, s) => s_fn_next!(arr, s),
            SeriesIntoIterator::Decimal(ref arr, s) => sc_fn_next!(arr, s),
            SeriesIntoIterator::Uuid(ref arr, s) => sc_fn_next!(arr, s),
            SeriesIntoIterator::Bytes(ref arr, s) => sc_fn_next!(arr, s),
        }
    }
}

impl<'a> IntoIterator for &'a Series {
    type Item = Value;
    type IntoIter = SeriesIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self.dtype() {
            ValueType::Bool => si!(self.0.bool(), Bool),
            ValueType::U8 => si!(self.0.u8(), U8),
            ValueType::U16 => si!(self.0.u16(), U16),
            ValueType::U32 => si!(self.0.u32(), U32),
            ValueType::U64 => si!(self.0.u64(), U64),
            ValueType::I8 => si!(self.0.i8(), I8),
            ValueType::I16 => si!(self.0.i16(), I16),
            ValueType::I32 => si!(self.0.i32(), I32),
            ValueType::I64 => si!(self.0.i64(), I64),
            ValueType::F32 => si!(self.0.f32(), F32),
            ValueType::F64 => si!(self.0.f64(), F64),
            ValueType::String => si!(self.0.utf8(), String),
            ValueType::Date => si!(self.0.date(), Date),
            ValueType::Time => si!(self.0.time(), Time),
            ValueType::DateTime => si!(self.0.datetime(), DateTime),
            ValueType::Decimal => si!(self.0.as_any(), Decimal, Decimal),
            ValueType::Uuid => si!(self.0.as_any(), Uuid, Uuid),
            ValueType::Bytes => si!(self.0.as_any(), Bytes, Bytes),
            // temporary ignore the rest of DataType variants
            _ => unimplemented!(),
        }
    }
}

pub enum SeriesIterator<'a> {
    Id(&'a UInt64Chunked, Stepper),
    Bool(&'a BooleanChunked, Stepper),
    U8(&'a UInt8Chunked, Stepper),
    U16(&'a UInt16Chunked, Stepper),
    U32(&'a UInt32Chunked, Stepper),
    U64(&'a UInt64Chunked, Stepper),
    I8(&'a Int8Chunked, Stepper),
    I16(&'a Int16Chunked, Stepper),
    I32(&'a Int32Chunked, Stepper),
    I64(&'a Int64Chunked, Stepper),
    F32(&'a Float32Chunked, Stepper),
    F64(&'a Float64Chunked, Stepper),
    String(&'a Utf8Chunked, Stepper),
    Date(&'a DateChunked, Stepper),
    Time(&'a TimeChunked, Stepper),
    DateTime(&'a DatetimeChunked, Stepper),
    Decimal(&'a ObjectChunked<Decimal>, Stepper),
    Uuid(&'a ObjectChunked<Uuid>, Stepper),
    Bytes(&'a ObjectChunked<Bytes>, Stepper),
}

impl<'a> Iterator for SeriesIterator<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SeriesIterator::Id(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::Bool(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::U8(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::U16(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::U32(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::U64(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::I8(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::I16(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::I32(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::I64(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::F32(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::F64(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::String(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::Date(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::Time(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::DateTime(arr, s) => s_fn_next!(arr, s),
            SeriesIterator::Decimal(ref arr, s) => sc_fn_next!(arr, s),
            SeriesIterator::Uuid(ref arr, s) => sc_fn_next!(arr, s),
            SeriesIterator::Bytes(ref arr, s) => sc_fn_next!(arr, s),
        }
    }
}

#[cfg(test)]
mod test_fabrix_series {

    use super::*;
    use crate::{date, series, value};

    #[test]
    fn test_series_creation() {
        let s = Series::from_integer_default_name(10u32);
        assert!(s.is_ok());

        let s = s.unwrap();
        assert_eq!(s.dtype(), &ValueType::U32);
        assert_eq!(s.get(9).unwrap(), value!(9u32));
        assert_eq!(s.take(&[0, 3, 9]).unwrap().len(), 3);

        let s = Series::from_range_default_name([3u8, 9]);
        assert!(s.is_ok());

        let s = s.unwrap();
        assert_eq!(s.dtype(), &ValueType::U8);
        assert!(s.get(100).is_err());
        assert_eq!(s.take(&[0, 4]).unwrap().len(), 2);

        let s = Series::from_values_default_name(
            vec![
                value!(Some("Jacob")),
                value!(Some("Jamie")),
                value!(None::<&str>),
            ],
            true,
        );
        assert!(s.is_ok());

        let s = s.unwrap();
        assert_eq!(s.dtype(), &ValueType::String);
    }

    #[test]
    fn test_series_creation2() {
        // `Series::from_values` & `Series::from_values_default_name`
        // if nullable is false, it will be in a strict mode, any type who mismatched will
        // return an error.
        let s = Series::from_values_default_name(
            vec![
                // this will turn to be `Value::Null`, where `&str` is no longer a type annotation
                value!(None::<&str>),
                value!(Some(10)),
                value!(Some(20)),
                value!(Some(30)),
            ],
            true,
        );
        assert!(s.is_ok());

        let s = Series::from_values_default_name(
            vec![
                // this will turn to be `Value::Null`, where `&str` is no longer a type annotation
                Value::Null,
                Value::I16(8),
                Value::I16(5),
                Value::I16(3),
            ],
            true,
        );
        assert!(s.is_ok());
    }

    #[test]
    fn test_series_new() {
        let s = Series::new(
            "date",
            &[
                date!(2019, 1, 1),
                date!(2019, 1, 2),
                date!(2019, 1, 3),
                date!(2019, 1, 4),
            ],
        );
        assert_eq!(s.dtype(), &ValueType::Date);

        let s = Series::new("num", &[1u8, 3, 5, 7, 9]);
        assert_eq!(s.dtype(), &ValueType::U8);
    }

    #[test]
    fn test_series_props() {
        let s = series!("yes" => [Some(1), None, Some(2)]);
        assert!(s.has_null());

        let s = series!("no" => [Some(1), Some(3), Some(2)]);
        assert!(!s.has_null());

        let s = series!("no" => [1, 3, 2]);
        assert!(!s.has_null());

        let s = series!("no" => [
            date!(2019, 1, 1),
            date!(2019, 1, 2),
            date!(2019, 1, 3),
            date!(2019, 1, 4),
        ]);
        assert!(!s.has_null());
    }

    #[test]
    fn test_series_get() {
        let s = series!("dollars" => ["Jacob", "Sam", "James", "April", "Julia", "Jack", "Henry"]);

        assert_eq!(s.head(None).unwrap().get(0).unwrap(), value!("Jacob"));
        assert_eq!(s.head(Some(2)).unwrap().len(), 2);
        assert!(s.head(Some(10)).is_err());

        assert_eq!(s.tail(None).unwrap().get(0).unwrap(), value!("Henry"));
        assert_eq!(s.tail(Some(2)).unwrap().len(), 2);
        assert!(s.tail(Some(10)).is_err());

        let (s1, s2) = s.split(4).unwrap();
        assert_eq!(s1.len(), 4);
        assert_eq!(s2.len(), 3);
    }

    #[test]
    fn test_series_op() {
        let s = series!("dollars" => ["Jacob", "Sam", "James", "April"]);

        let flt = series!("cmp" => ["Jacob", "Bob"]);
        assert_eq!(s.find_indices(&flt), vec![0]);

        let flt = value!("April");
        assert_eq!(s.find_index(&flt), Some(3));
    }

    #[test]
    fn test_series_concat() {
        let mut s1 = series!("dollars" => ["Jacob", "Sam", "James", "April"]);
        let s2 = series!("other" => ["Julia", "Jack", "John"]);

        s1.concat(s2).unwrap();
        assert_eq!(s1.len(), 7);
    }

    #[test]
    fn test_series_op1() {
        let mut s1 = series!("dollars" => ["Jacob", "Sam", "James", "April"]);

        let v1 = value!("Julia");
        s1.push(v1).unwrap();
        assert_eq!(s1.len(), 5);

        let s2 = series!(["Jackson", "Jan"]);
        s1.concat(s2).unwrap();
        assert_eq!(s1.len(), 7);

        let v2 = value!("Merry");
        s1.insert(2, v2).unwrap();
        assert_eq!(s1.len(), 8);

        let s3 = series!(["Jasmine", "Justin"]);
        s1.insert_many(3, s3).unwrap();
        assert_eq!(s1.len(), 10);

        s1.pop().unwrap();
        assert_eq!(s1.len(), 9);

        s1.remove(3).unwrap();
        assert_eq!(s1.len(), 8);
    }

    #[test]
    fn test_series_op2() {
        let mut s1 = series!("dollars" => ["Jacob", "Sam", "James", "April", "Julia", "Jack", "Merry", "Justin"]);

        assert_eq!(s1.slice(3, 4).len(), 4);

        s1.remove_slice(3, 4).unwrap();
        assert_eq!(s1.len(), 4);

        assert_eq!(s1.slice(-3, 4).len(), 3);

        s1.remove_slice(-3, 4).unwrap();
        assert_eq!(s1.len(), 1);
    }

    #[test]
    fn test_series_iteration() {
        let s = series!("dollars" => ["Jacob", "Sam", "James", "April", "Julia", "Jack", "Henry"]);
        let mut iter = s.into_iter();

        assert_eq!(iter.next().unwrap(), value!("Jacob"));
        assert_eq!(iter.next().unwrap(), value!("Sam"));
        assert_eq!(iter.next().unwrap(), value!("James"));
        assert_eq!(iter.next().unwrap(), value!("April"));
        assert_eq!(iter.next().unwrap(), value!("Julia"));
        assert_eq!(iter.next().unwrap(), value!("Jack"));
        assert_eq!(iter.next().unwrap(), value!("Henry"));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_series_apply() {
        use polars::prelude::ChunkApply;
        use std::borrow::Cow;

        let s1 = series!("dollars" => ["Jacob", "Sam", "James", "April", "Julia", "Jack", "Henry"]);

        println!("{:?}", s1);

        let c =
            s1.0.utf8()
                .unwrap()
                .apply(|s| Cow::Owned(s.to_owned() + "!"));

        let s2 = Series(c.into_series());

        println!("{:?}", s2);

        let mut iter = s2.into_iter();

        assert_eq!(iter.next().unwrap(), value!("Jacob!"));
        assert_eq!(iter.next().unwrap(), value!("Sam!"));
        assert_eq!(iter.next().unwrap(), value!("James!"));
        assert_eq!(iter.next().unwrap(), value!("April!"));
        assert_eq!(iter.next().unwrap(), value!("Julia!"));
        assert_eq!(iter.next().unwrap(), value!("Jack!"));
        assert_eq!(iter.next().unwrap(), value!("Henry!"));
        assert!(iter.next().is_none());
    }
}
