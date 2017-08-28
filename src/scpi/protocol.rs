use std::marker::PhantomData;

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::{ClientProto, ServerProto};

use super::client_codec::ScpiClientCodec;
use super::errors::{Error, Result};
use super::request::ScpiRequest;
use super::response::ScpiResponse;
use super::server_codec::ScpiServerCodec;

#[derive(Clone)]
pub struct ScpiProtocol<A: ScpiRequest> {
    _request: PhantomData<A>,
}

impl<A> ScpiProtocol<A>
where
    A: ScpiRequest + 'static,
{
    pub fn new() -> Self {
        Self {
            _request: PhantomData,
        }
    }
}

impl<T, A> ServerProto<T> for ScpiProtocol<A>
where
    T: AsyncRead + AsyncWrite + 'static,
    A: ScpiRequest + 'static,
{
    type Request = A;
    type Response = ScpiResponse;
    type Error = Error;
    type Transport = Framed<T, ScpiServerCodec<A>>;
    type BindTransport = Result<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ScpiServerCodec::new()))
    }
}

impl<T, A> ClientProto<T> for ScpiProtocol<A>
where
    T: AsyncRead + AsyncWrite + 'static,
    A: ScpiRequest + 'static,
{
    type Request = A;
    type Response = ScpiResponse;
    type Error = Error;
    type Transport = Framed<T, ScpiClientCodec<A>>;
    type BindTransport = Result<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ScpiClientCodec::new()))
    }
}
