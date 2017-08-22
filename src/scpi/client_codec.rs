use std::marker::PhantomData;
use std::str;

use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};

use super::errors::{Error, Result};
use super::extension::ScpiExtension;
use super::requests::ScpiRequest;
use super::response::ScpiResponse;

pub struct ScpiClientCodec<X: ScpiExtension> {
    _extension: PhantomData<X>,
}

impl<X> ScpiClientCodec<X>
where
    X: ScpiExtension,
{
    pub fn new() -> Self {
        Self {
            _extension: PhantomData,
        }
    }
}

impl<X> Encoder for ScpiClientCodec<X>
where
    X: ScpiExtension,
{
    type Item = ScpiRequest<X>;
    type Error = Error;

    fn encode(
        &mut self,
        request: Self::Item,
        buffer: &mut BytesMut,
    ) -> Result<()> {
        request.encode(buffer);

        Ok(())
    }
}

impl<X> Decoder for ScpiClientCodec<X>
where
    X: ScpiExtension,
{
    type Item = ScpiResponse;
    type Error = Error;

    fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>> {
        let message_length = buffer.len();
        let message_bytes = buffer.split_to(message_length);
        let message = str::from_utf8(&message_bytes)?;

        Ok(Some(ScpiResponse::from(message.trim())))
    }
}
