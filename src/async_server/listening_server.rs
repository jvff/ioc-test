use std::fmt::Display;
use std::hash::Hash;
use std::{io, mem};
use std::sync::{Arc, Mutex};

use futures::{Async, Future, Poll};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_proto::pipeline::ServerProto;
use tokio_service::NewService;

use super::active_server::ActiveServer;
use super::bound_connection_future::BoundConnectionFuture;
use super::errors::Error;
use super::finite_service::FiniteService;

pub struct ListeningServer<P, S>
where
    P: ServerProto<TcpStream>,
    P::Error: Into<Error>,
    S: FiniteService<Request = P::Request, Response = P::Response>,
{
    connection: BoundConnectionFuture<P>,
    service: io::Result<S>,
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
        Self {
            service: service_factory.new_service(),
            connection: BoundConnectionFuture::from(listener, protocol),
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
        let connection = try_ready!(self.connection.poll());
        let service = mem::replace(
            &mut self.service,
            Err(io::Error::new(
                io::ErrorKind::Other,
                "server listening state can't be polled for two connections",
            )),
        );

        Ok(Async::Ready(ActiveServer::new(connection, service?)))
    }
}
