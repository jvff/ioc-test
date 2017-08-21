use std::str;

use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};

use super::errors::{Error, Result};
use super::requests::ScpiRequest;
use super::response::ScpiResponse;

pub struct ScpiClientCodec;

impl Encoder for ScpiClientCodec {
    type Item = ScpiRequest;
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

impl Decoder for ScpiClientCodec {
    type Item = ScpiResponse;
    type Error = Error;

    fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>> {
        let message_length = buffer.len();
        let message_bytes = buffer.split_to(message_length);
        let message = str::from_utf8(&message_bytes)?;

        Ok(Some(ScpiResponse::from(message)))
    }
}
