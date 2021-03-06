use std::io;

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;

use super::line_codec::LineCodec;

#[derive(Clone, Copy, Debug)]
pub struct LineProtocol {
    separator: u8,
}

impl LineProtocol {
    pub fn with_separator(separator: u8) -> Self {
        Self { separator }
    }
}

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProtocol {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Transport = Framed<T, LineCodec>;
    type BindTransport = io::Result<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec::with_separator(self.separator)))
    }
}
