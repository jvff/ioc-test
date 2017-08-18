use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::io;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use tokio_core::net::TcpStream;
use tokio_proto::pipeline::ServerProto;
use tokio_service::NewService;

use super::errors::Error;
use super::super::async_server;
use super::super::async_server::FiniteService;
use super::super::mock_service;
use super::super::mock_service::{MockService, MockServiceFactory};

pub trait IocTestParameters {
    type Request: Clone + Display + Eq + Hash + 'static;
    type Response: Clone + Eq + 'static;
    type ProtocolError: From<io::Error>
        + Into<Error>
        + Into<async_server::Error>
        + Into<mock_service::Error>;
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
        expected_requests: Arc<Mutex<HashMap<Self::Request, Self::Response>>>,
        requests_to_verify: Arc<Mutex<HashSet<Self::Request>>>,
    ) -> Self::ServiceFactory;
}

pub struct MockTestParameters<P>
where
    P: ServerProto<TcpStream>,
{
    _protocol: PhantomData<P>,
}

impl<P> IocTestParameters for MockTestParameters<P>
where
    P: ServerProto<TcpStream>,
    P::Request: Clone + Display + Eq + Hash,
    P::Response: Clone + Eq,
    P::Error: Into<async_server::Error> + Into<Error>,
    mock_service::Error: From<P::Error>,
{
    type Request = P::Request;
    type Response = P::Response;
    type ProtocolError = P::Error;
    type Protocol = P;
    type ServiceError = mock_service::Error;
    type Service = MockService<P::Request, P::Response>;
    type ServiceFactory = MockServiceFactory<P::Request, P::Response>;

    fn create_service_factory(
        expected_requests: Arc<Mutex<HashMap<Self::Request, Self::Response>>>,
        requests_to_verify: Arc<Mutex<HashSet<Self::Request>>>,
    ) -> Self::ServiceFactory {
        MockServiceFactory::new(expected_requests, requests_to_verify)
    }
}
