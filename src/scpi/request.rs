use std::fmt::Display;

use bytes::BytesMut;

use super::errors::{ErrorKind, Result};

pub trait ScpiRequest: Display {
    fn decode(message: &str) -> Option<Self>
    where
        Self: Sized;

    fn from(message: &str) -> Result<Self>
    where
        Self: Sized,
    {
        if let Some(decoded_message) = Self::decode(message) {
            Ok(decoded_message)
        } else {
            Err(ErrorKind::UnknownScpiRequest(String::from(message)).into())
        }
    }

    fn encode(&self, buffer: &mut BytesMut) {
        buffer.extend(self.to_string().as_bytes());
        buffer.extend("\n".as_bytes())
    }
}
