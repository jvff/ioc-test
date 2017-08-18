use std::fmt::Display;
use std::hash::Hash;
use std::io;
use std::marker::PhantomData;

use tokio_core::net::TcpStream;
use tokio_proto::pipeline::ServerProto;

use super::errors::Error;
use super::ioc_test_protocol::IocTestProtocol;
use super::super::async_server;
use super::super::async_server::FiniteService;
use super::super::mock_service;
use super::super::mock_service::MockService;

pub trait IocTestParameters {
    type Request: Clone + Display + Eq + Hash + 'static;
    type Response: Clone + Eq + 'static;
    type ProtocolError: From<io::Error> + Into<Error> + Into<async_server::Error>;
    type Protocol: IocTestProtocol<Request = Self::Request, Response = Self::Response>
        + ServerProto<
        TcpStream,
        Request = Self::Request,
        Response = Self::Response,
        Error = Self::ProtocolError,
    >;
    type ServiceError: Into<Error>;
    type Service: FiniteService<Request = Self::Request, Response = Self::Response>;
}

pub struct MockTestParameters<P>
where
    P: IocTestProtocol,
{
    _protocol: PhantomData<P>,
}

impl<P> IocTestParameters for MockTestParameters<P>
where
    P: IocTestProtocol
        + ServerProto<
        TcpStream,
        Request = <P as IocTestProtocol>::Request,
        Response = <P as IocTestProtocol>::Response,
    >,
    <P as ServerProto<TcpStream>>::Error: Into<Error> + Into<async_server::Error>,
{
    type Request = <P as IocTestProtocol>::Request;
    type Response = <P as IocTestProtocol>::Response;
    type ProtocolError = <P as ServerProto<TcpStream>>::Error;
    type Protocol = P;
    type ServiceError = mock_service::Error;
    type Service = MockService<
        <P as IocTestProtocol>::Request,
        <P as IocTestProtocol>::Response,
    >;
}
