use std::marker::PhantomData;
use std::str;

use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};

use super::errors::{Error, Result};
use super::request::ScpiRequest;
use super::response::ScpiResponse;

pub struct ScpiServerCodec<A: ScpiRequest> {
    _request: PhantomData<A>,
}

impl<A> ScpiServerCodec<A>
where
    A: ScpiRequest,
{
    pub fn new() -> Self {
        Self {
            _request: PhantomData,
        }
    }
}

impl<A> Encoder for ScpiServerCodec<A>
where
    A: ScpiRequest,
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

impl<A> Decoder for ScpiServerCodec<A>
where
    A: ScpiRequest,
{
    type Item = A;
    type Error = Error;

    fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>> {
        let message_end = buffer.iter().position(is_end_of_message);

        if let Some(message_end) = message_end {
            let message_bytes = buffer.split_to(message_end);
            let message = str::from_utf8(&message_bytes)?;

            remove_trailing_bytes(buffer);

            Ok(Some(A::from(message)?))
        } else {
            Ok(None)
        }
    }
}

fn remove_trailing_bytes(buffer: &mut BytesMut) {
    let start_of_next_message = buffer.iter().position(is_not_terminator);

    if let Some(next_message_byte) = start_of_next_message {
        if next_message_byte > 0 {
            buffer.split_to(next_message_byte);
        }
    } else {
        let trailing_bytes = buffer.len();

        buffer.split_to(trailing_bytes);
    }
}

fn is_not_terminator(byte: &u8) -> bool {
    !is_end_of_message(byte)
}

fn is_end_of_message(byte: &u8) -> bool {
    *byte == '\r' as u8 || *byte == '\n' as u8 || *byte == ';' as u8
}
