use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::io;

use tokio_core::net::TcpStream;
use tokio_proto::pipeline::ServerProto;
use tokio_service::NewService;

use super::errors::Error;
use super::super::async_server;
use super::super::async_server::FiniteService;
use super::super::instrumenting_service::WhenVerifier;
use super::super::instrumenting_service::verifiers::VerifyAll;

pub trait IocTestParameters {
    type Request: Clone + Display + Eq + Hash + 'static;
    type Response: Clone + Eq + 'static;
    type ProtocolError: From<io::Error>
        + Into<Error>
        + Into<async_server::Error>;
    type Protocol: ServerProto<
        TcpStream,
        Request = Self::Request,
        Response = Self::Response,
        Error = Self::ProtocolError,
    >;
    type ServiceError: From<Self::ProtocolError> + Into<Error> + Into<async_server::Error>;
    type Service: FiniteService<
        Request = Self::Request,
        Response = Self::Response,
        Error = Self::ServiceError,
    >;
    type ServiceFactory: NewService<
        Request = Self::Request,
        Response = Self::Response,
        Error = Self::ServiceError,
        Instance = Self::Service,
    >;

    fn create_service_factory(
        expected_requests: HashMap<Self::Request, Self::Response>,
        verifier: VerifyAll<WhenVerifier<Self::Request, Self::Response>>,
    ) -> Self::ServiceFactory;
}
