use std::str::FromStr;

use bytes::BytesMut;
use ordered_float::OrderedFloat;

use super::errors::{Error, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScpiResponse {
    Empty,
    Integer(isize),
    Double(OrderedFloat<f64>),
    Utf8String(String),
}

impl ScpiResponse {
    pub fn encode(&self, buffer: &mut BytesMut) {
        match *self {
            ScpiResponse::Empty => {}
            ScpiResponse::Integer(value) => {
                buffer.extend(value.to_string().as_bytes());
                buffer.extend("\n".as_bytes())
            }
            ScpiResponse::Double(value) => {
                buffer.extend(value.to_string().as_bytes());
                buffer.extend("\n".as_bytes())
            }
            ScpiResponse::Utf8String(ref string) => {
                buffer.extend(string.as_bytes());
                buffer.extend("\n".as_bytes())
            }
        }
    }
}

impl FromStr for ScpiResponse {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self> {
        if string.len() == 0 {
            Ok(().into())
        } else if let Ok(integer) = string.parse::<isize>() {
            Ok(integer.into())
        } else if let Ok(double) = string.parse::<f64>() {
            Ok(double.into())
        } else {
            Ok(string.into())
        }
    }
}

impl From<()> for ScpiResponse {
    fn from(_nothing: ()) -> ScpiResponse {
        ScpiResponse::Empty
    }
}

impl From<isize> for ScpiResponse {
    fn from(value: isize) -> ScpiResponse {
        ScpiResponse::Integer(value)
    }
}

impl From<f64> for ScpiResponse {
    fn from(value: f64) -> ScpiResponse {
        ScpiResponse::Double(value.into())
    }
}

impl<'a> From<&'a str> for ScpiResponse {
    fn from(string: &str) -> ScpiResponse {
        ScpiResponse::Utf8String(string.to_string())
    }
}
