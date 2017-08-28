use std::fmt;
use std::fmt::{Display, Formatter};

use bytes::BytesMut;

use super::super::errors::{ErrorKind, Result};
use super::super::extension::ScpiExtension;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ScpiRequest<X>
where
    X: ScpiExtension,
{
    Other(X),
}

impl<X> ScpiRequest<X>
where
    X: ScpiExtension,
{
    pub fn from(string: &str) -> Result<Self> {
        if let Some(extended_request) = X::decode(string) {
            Ok(ScpiRequest::Other(extended_request))
        } else {
            Err(ErrorKind::UnknownScpiRequest(String::from(string)).into())
        }
    }

    pub fn encode(&self, buffer: &mut BytesMut) {
        buffer.extend(self.to_string().as_bytes());
        buffer.extend("\n".as_bytes())
    }
}

impl<X> Display for ScpiRequest<X>
where
    X: ScpiExtension,
{
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            ScpiRequest::Other(ref request_extension) => {
                request_extension.fmt(formatter)
            }
        }
    }
}
