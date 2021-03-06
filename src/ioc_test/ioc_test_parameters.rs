use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::io;

use tokio_core::net::TcpStream;
use tokio_proto::pipeline::ServerProto;
use tokio_service::NewService;

use super::errors::Error;
use super::super::async_server;
use super::super::async_server::FiniteService;
use super::super::instrumenting_service::verifiers;
use super::super::instrumenting_service::verifiers::BoxedVerifierFactory;

pub trait IocTestParameters {
    type Request: Clone + Debug + Display + Eq + Hash + 'static;
    type Response: Clone + Debug + Eq + 'static;
    type ProtocolError: From<io::Error> + Into<Error> + Into<async_server::Error>;
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

    fn create_protocol(&self) -> Self::Protocol;
    fn create_service_factory(
        &self,
        expected_requests: HashMap<Self::Request, Self::Response>,
        verifier_factory: BoxedVerifierFactory<
            'static,
            Self::Request,
            Self::Response,
            verifiers::Error,
        >,
    ) -> Self::ServiceFactory;
}
