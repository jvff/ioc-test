use std::{io, str};

use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};

use super::ioc_shell_command::IocShellCommand;

error_chain! {
    foreign_links {
        Io(io::Error);
        InvalidIocShellOutput(str::Utf8Error);
    }
}

pub struct IocShellCodec;

impl Encoder for IocShellCodec {
    type Item = IocShellCommand;
    type Error = Error;

    fn encode(
        &mut self,
        item: Self::Item,
        buffer: &mut BytesMut,
    ) -> Result<()> {
        buffer.extend(item.to_string().as_bytes());
        buffer.extend("\n".as_bytes());

        Ok(())
    }
}

impl Decoder for IocShellCodec {
    type Item = String;
    type Error = Error;

    fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>> {
        let prompt = "epics> ";

        if let Some(prompt_start) = buffer.find(prompt)? {
            let prompt_line_length =
                buffer.iter().skip(prompt_start).position(is_newline);

            if let Some(prompt_line_length) = prompt_line_length {
                buffer.split_to(prompt_start + prompt_line_length + 1);

                if let Some(next_prompt_start) = buffer.find(prompt)? {
                    let command_output = buffer.split_to(next_prompt_start);
                    let command_output_string =
                        str::from_utf8(&command_output)?;

                    return Ok(Some(String::from(command_output_string)));
                }
            }
        }

        Ok(None)
    }
}

trait FindStringInBuffer {
    fn find(&self, string: &str) -> Result<Option<usize>>;
}

impl FindStringInBuffer for BytesMut {
    fn find(&self, string: &str) -> Result<Option<usize>> {
        let buffer = str::from_utf8(self)?;

        Ok(buffer.find(string))
    }
}

fn is_newline(character: &u8) -> bool {
    *character == '\n' as u8
}
