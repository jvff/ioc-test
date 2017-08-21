use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::{ClientProto, ServerProto};

use super::client_codec::ScpiClientCodec;
use super::errors::{Error, Result};
use super::requests::ScpiRequest;
use super::response::ScpiResponse;
use super::server_codec::ScpiServerCodec;

pub struct ScpiProtocol;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for ScpiProtocol {
    type Request = ScpiRequest;
    type Response = ScpiResponse;
    type Error = Error;
    type Transport = Framed<T, ScpiServerCodec>;
    type BindTransport = Result<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ScpiServerCodec))
    }
}

impl<T> ClientProto<T> for ScpiProtocol
where
    T: AsyncRead + AsyncWrite + 'static,
{
    type Request = ScpiRequest;
    type Response = ScpiResponse;
    type Error = Error;
    type Transport = Framed<T, ScpiClientCodec>;
    type BindTransport = Result<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ScpiClientCodec))
    }
}
