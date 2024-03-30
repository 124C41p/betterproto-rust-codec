use chrono::Datelike;
use prost::Message;
use pyo3::{
    prelude::Bound,
    types::{PyAnyMethods, PyBytes},
    FromPyObject, PyAny, PyObject, PyResult, Python, ToPyObject,
};

use crate::{
    betterproto_interop::{InteropError, InteropResult},
    decode::{DecodeError, DecodeResult},
};

const NANOS_PER_SEC: u32 = 1_000_000_000;

#[derive(Message)]
pub struct BoolValue {
    #[prost(bool, tag = "1")]
    pub value: bool,
}

#[derive(Message)]
pub struct BytesValue {
    #[prost(bytes, tag = "1")]
    pub value: Vec<u8>,
}

#[derive(Message)]
pub struct DoubleValue {
    #[prost(double, tag = "1")]
    pub value: f64,
}

#[derive(Message)]
pub struct FloatValue {
    #[prost(float, tag = "1")]
    pub value: f32,
}

#[derive(Message)]
pub struct Int32Value {
    #[prost(int32, tag = "1")]
    pub value: i32,
}

#[derive(Message)]
pub struct Int64Value {
    #[prost(int64, tag = "1")]
    pub value: i64,
}

#[derive(Message)]
pub struct UInt32Value {
    #[prost(uint32, tag = "1")]
    pub value: u32,
}

#[derive(Message)]
pub struct UInt64Value {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}

#[derive(Message)]
pub struct StringValue {
    #[prost(string, tag = "1")]
    pub value: String,
}

#[derive(Message)]
pub struct Duration {
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}

#[derive(Message)]
pub struct Timestamp {
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}

impl<'py> FromPyObject<'py> for BoolValue {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let res = BoolValue {
            value: ob.extract()?,
        };
        Ok(res)
    }
}

impl<'py> FromPyObject<'py> for BytesValue {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let res = BytesValue {
            value: ob.extract()?,
        };
        Ok(res)
    }
}

impl<'py> FromPyObject<'py> for DoubleValue {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let res = DoubleValue {
            value: ob.extract()?,
        };
        Ok(res)
    }
}

impl<'py> FromPyObject<'py> for FloatValue {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let res = FloatValue {
            value: ob.extract()?,
        };
        Ok(res)
    }
}

impl<'py> FromPyObject<'py> for Int32Value {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let res = Int32Value {
            value: ob.extract()?,
        };
        Ok(res)
    }
}

impl<'py> FromPyObject<'py> for Int64Value {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let res = Int64Value {
            value: ob.extract()?,
        };
        Ok(res)
    }
}

impl<'py> FromPyObject<'py> for UInt32Value {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let res = UInt32Value {
            value: ob.extract()?,
        };
        Ok(res)
    }
}

impl<'py> FromPyObject<'py> for UInt64Value {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let res = UInt64Value {
            value: ob.extract()?,
        };
        Ok(res)
    }
}

impl<'py> FromPyObject<'py> for StringValue {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let res = StringValue {
            value: ob.extract()?,
        };
        Ok(res)
    }
}

impl From<chrono::TimeDelta> for Duration {
    fn from(value: chrono::TimeDelta) -> Self {
        Self {
            seconds: value.num_seconds(),
            nanos: value.subsec_nanos(),
        }
    }
}

impl<'py> FromPyObject<'py> for Duration {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(ob.extract::<chrono::TimeDelta>()?.into())
    }
}

impl TryFrom<&Duration> for chrono::Duration {
    type Error = DecodeError;

    fn try_from(value: &Duration) -> Result<Self, Self::Error> {
        let (secs, nanos) = if value.nanos < 0 {
            (value.seconds - 1, value.nanos + NANOS_PER_SEC as i32)
        } else {
            (value.seconds, value.nanos)
        };
        let nanos = u32::try_from(nanos).map_err(|_| DecodeError::InvalidData)?;
        chrono::Duration::new(secs, nanos).ok_or(DecodeError::InvalidData)
    }
}

impl Duration {
    pub fn try_to_object(&self, py: Python) -> DecodeResult<PyObject> {
        Ok(chrono::TimeDelta::try_from(self)?.to_object(py))
    }
}

impl Timestamp {
    fn try_from_any(ob: &Bound<PyAny>) -> InteropResult<Self> {
        // try to extract an offset-aware datetime object
        if let Ok(dt) = ob.extract::<chrono::DateTime<chrono::FixedOffset>>() {
            return Ok(dt.to_utc().into());
        }

        // fallback to extract an offset-naive datetime object, interpreted relative to the user's local timezone
        let dt = ob.extract::<chrono::NaiveDateTime>()?;
        Ok(dt
            .and_local_timezone(chrono::Local)
            .single()
            .ok_or(InteropError::OffsetNaiveDateTimeDoesNotMap(dt))?
            .to_utc()
            .into())
    }

    pub fn try_to_object(&self, py: Python) -> DecodeResult<PyObject> {
        let nanos = u32::try_from(self.nanos).map_err(|_| DecodeError::InvalidData)?;
        let dt = chrono::DateTime::from_timestamp(self.seconds, nanos)
            .ok_or(DecodeError::InvalidData)?;
        if !(1..=9999).contains(&dt.year()) {
            return Err(DecodeError::TimestampOutOfBounds(dt));
        }
        Ok(dt.to_object(py))
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Timestamp {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        let nanos = value.timestamp_subsec_nanos();
        debug_assert!(nanos < NANOS_PER_SEC, "Python datetimes do not have leap seconds, so this value should always satisfy the Protobuf specification.");

        Self {
            seconds: value.timestamp(),
            nanos: nanos as i32,
        }
    }
}

impl<'py> FromPyObject<'py> for Timestamp {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Self::try_from_any(ob)?)
    }
}

impl ToPyObject for BoolValue {
    fn to_object(&self, py: Python) -> PyObject {
        self.value.to_object(py)
    }
}

impl ToPyObject for BytesValue {
    fn to_object(&self, py: Python) -> PyObject {
        PyBytes::new_bound(py, &self.value).to_object(py)
    }
}

impl ToPyObject for DoubleValue {
    fn to_object(&self, py: Python) -> PyObject {
        self.value.to_object(py)
    }
}

impl ToPyObject for FloatValue {
    fn to_object(&self, py: Python) -> PyObject {
        self.value.to_object(py)
    }
}

impl ToPyObject for Int32Value {
    fn to_object(&self, py: Python) -> PyObject {
        self.value.to_object(py)
    }
}

impl ToPyObject for Int64Value {
    fn to_object(&self, py: Python) -> PyObject {
        self.value.to_object(py)
    }
}

impl ToPyObject for UInt32Value {
    fn to_object(&self, py: Python) -> PyObject {
        self.value.to_object(py)
    }
}

impl ToPyObject for UInt64Value {
    fn to_object(&self, py: Python) -> PyObject {
        self.value.to_object(py)
    }
}

impl ToPyObject for StringValue {
    fn to_object(&self, py: Python) -> PyObject {
        self.value.to_object(py)
    }
}
