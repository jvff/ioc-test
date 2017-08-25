use std::fmt::Display;
use std::hash::Hash;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use futures::{Async, Future, Poll};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_core::reactor::Handle;
use tokio_proto::pipeline::ServerProto;
use tokio_service::NewService;

use super::errors::{Error, ErrorKind};
use super::finite_service::FiniteService;
use super::listening_server::ListeningServer;

pub struct StartServer<P, S>
where
    S: NewService,
    S::Request: 'static,
    S::Response: 'static,
    S::Instance: FiniteService,
    P: ServerProto<
        TcpStream,
        Request = <S as NewService>::Request,
        Response = <S as NewService>::Response,
    >,
{
    address: SocketAddr,
    service_factory: Option<S>,
    protocol: Arc<Mutex<P>>,
    handle: Handle,
}

impl<P, S> StartServer<P, S>
where
    S: NewService,
    S::Instance: FiniteService,
    S::Request: Clone + Display + Eq + Hash,
    S::Response: Clone,
    S::Error: Into<Error>,
    P: ServerProto<
        TcpStream,
        Request = <S as NewService>::Request,
        Response = <S as NewService>::Response,
    >,
    P::Error: Into<Error>,
{
    pub fn new(
        address: SocketAddr,
        service_factory: S,
        protocol: Arc<Mutex<P>>,
        handle: Handle,
    ) -> Self {
        Self {
            address,
            protocol,
            handle,
            service_factory: Some(service_factory),
        }
    }

    pub fn shutdown(&mut self) -> Poll<(), Error> {
        if let Some(service_factory) = self.service_factory.take() {
            let mut service = service_factory.new_service()?;

            match service.force_stop() {
                Ok(()) => Ok(Async::Ready(())),
                Err(error) => Err(error.into()),
            }
        } else {
            Err(
                ErrorKind::IncorrectShutDownAttempt(
                    String::from("StartServer"),
                ).into(),
            )
        }
    }

    fn start_server(&mut self) -> Poll<ListeningServer<P, S::Instance>, Error> {
        let listener = TcpListener::bind(&self.address, &self.handle)?;
        let protocol = self.protocol.clone();

        if let Some(service_factory) = self.service_factory.take() {
            Ok(Async::Ready(
                ListeningServer::new(listener, service_factory, protocol),
            ))
        } else {
            Err(ErrorKind::AttemptToStartServerTwice.into())
        }
    }
}

impl<P, S> Future for StartServer<P, S>
where
    S: NewService,
    S::Instance: FiniteService,
    S::Request: Clone + Display + Eq + Hash,
    S::Response: Clone,
    S::Error: Into<Error>,
    P: ServerProto<
        TcpStream,
        Request = <S as NewService>::Request,
        Response = <S as NewService>::Response,
    >,
    P::Error: Into<Error>,
{
    type Item = ListeningServer<P, S::Instance>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if self.service_factory.is_some() {
            self.start_server()
        } else {
            Err(ErrorKind::AttemptToStartServerTwice.into())
        }
    }
}
