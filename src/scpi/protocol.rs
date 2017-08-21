use std::marker::PhantomData;

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::{ClientProto, ServerProto};

use super::client_codec::ScpiClientCodec;
use super::errors::{Error, Result};
use super::extension::ScpiExtension;
use super::requests::ScpiRequest;
use super::response::ScpiResponse;
use super::server_codec::ScpiServerCodec;

#[derive(Clone)]
pub struct ScpiProtocol<X: ScpiExtension> {
    _extension: PhantomData<X>,
}

impl<X> ScpiProtocol<X>
where
    X: ScpiExtension + 'static,
{
    pub fn new() -> Self {
        Self {
            _extension: PhantomData,
        }
    }
}

impl<T, X> ServerProto<T> for ScpiProtocol<X>
where
    T: AsyncRead + AsyncWrite + 'static,
    X: ScpiExtension + 'static,
{
    type Request = ScpiRequest<X>;
    type Response = ScpiResponse;
    type Error = Error;
    type Transport = Framed<T, ScpiServerCodec<X>>;
    type BindTransport = Result<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ScpiServerCodec::new()))
    }
}

impl<T, X> ClientProto<T> for ScpiProtocol<X>
where
    T: AsyncRead + AsyncWrite + 'static,
    X: ScpiExtension + 'static,
{
    type Request = ScpiRequest<X>;
    type Response = ScpiResponse;
    type Error = Error;
    type Transport = Framed<T, ScpiClientCodec<X>>;
    type BindTransport = Result<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ScpiClientCodec::new()))
    }
}
