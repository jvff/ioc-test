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
            Ok(ScpiResponse::Empty)
        } else if let Ok(integer) = string.parse() {
            Ok(ScpiResponse::Integer(integer))
        } else if let Ok(double) = string.parse::<f64>() {
            Ok(ScpiResponse::Double(double.into()))
        } else {
            Ok(ScpiResponse::Utf8String(String::from(string)))
        }
    }
}
