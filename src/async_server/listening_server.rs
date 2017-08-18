use std::fmt::Display;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use futures::{Async, Future, Poll};
use futures::future::{FutureResult, Join};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_proto::pipeline::ServerProto;
use tokio_service::NewService;

use super::active_server::ActiveServer;
use super::bound_connection_future::BoundConnectionFuture;
use super::errors::{Error, NormalizeError};
use super::finite_service::FiniteService;

pub struct ListeningServer<P, S>
where
    P: ServerProto<TcpStream>,
    P::Error: Into<Error>,
    S: FiniteService<Request = P::Request, Response = P::Response>,
{
    connection_and_service:
        Join<BoundConnectionFuture<P>, FutureResult<S, Error>>,
}

impl<P, S> ListeningServer<P, S>
where
    P: ServerProto<TcpStream>,
    P::Request: Clone + Display + Eq + Hash,
    P::Response: Clone,
    P::Error: Into<Error>,
    S: FiniteService<Request = P::Request, Response = P::Response>,
{
    pub fn new<F>(
        listener: TcpListener,
        service_factory: F,
        protocol: Arc<Mutex<P>>,
    ) -> Self
    where
        F: NewService<
            Request = S::Request,
            Response = S::Response,
            Error = S::Error,
            Instance = S,
        >,
    {
        let service = service_factory.new_service();
        let connection = BoundConnectionFuture::from(listener, protocol);

        Self {
            connection_and_service: connection.join(service.normalize_error()),
        }
    }
}

impl<P, S> Future for ListeningServer<P, S>
where
    P: ServerProto<TcpStream>,
    P::Request: Clone + Display + Eq + Hash,
    P::Response: Clone,
    P::Error: Into<Error> + Into<S::Error>,
    S: FiniteService<Request = P::Request, Response = P::Response>,
    S::Error: Into<Error>,
{
    type Item = ActiveServer<P::Transport, S>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let (connection, service) =
            try_ready!(self.connection_and_service.poll());

        Ok(Async::Ready(ActiveServer::new(connection, service)))
    }
}
