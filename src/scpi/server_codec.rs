use std::marker::PhantomData;
use std::str;

use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};

use super::errors::{Error, Result};
use super::extension::ScpiExtension;
use super::requests::ScpiRequest;
use super::response::ScpiResponse;

pub struct ScpiServerCodec<X: ScpiExtension> {
    _extension: PhantomData<X>,
}

impl<X> ScpiServerCodec<X>
where
    X: ScpiExtension,
{
    pub fn new() -> Self {
        Self {
            _extension: PhantomData,
        }
    }
}

impl<X> Encoder for ScpiServerCodec<X>
where
    X: ScpiExtension,
{
    type Item = ScpiResponse;
    type Error = Error;

    fn encode(
        &mut self,
        response: Self::Item,
        buffer: &mut BytesMut,
    ) -> Result<()> {
        response.encode(buffer);

        Ok(())
    }
}

impl<X> Decoder for ScpiServerCodec<X>
where
    X: ScpiExtension,
{
    type Item = ScpiRequest<X>;
    type Error = Error;

    fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>> {
        let message_end = buffer.iter().position(is_end_of_message);

        if let Some(message_end) = message_end {
            let message_bytes = buffer.split_to(message_end);
            let message = str::from_utf8(&message_bytes)?;

            buffer.split_to(1);

            Ok(Some(ScpiRequest::from(message)?))
        } else {
            Ok(None)
        }
    }
}

fn is_end_of_message(byte: &u8) -> bool {
    *byte == '\r' as u8 || *byte == '\n' as u8 || *byte == ';' as u8
}
