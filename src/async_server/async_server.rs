use std::fmt::Display;
use std::hash::Hash;
use std::mem;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use futures::{Async, Future, Poll, Stream, Sink};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::pipeline::ServerProto;
use tokio_service::NewService;

use super::active_server::ActiveServer;
use super::errors::{Error, ErrorKind};
use super::finite_service::FiniteService;
use super::listening_server::ListeningServer;
use super::start_server::StartServer;

pub enum AsyncServer<P, S>
where
    S: NewService,
    S::Request: 'static + Clone + Display + Eq + Hash,
    S::Response: 'static + Clone,
    S::Instance: FiniteService,
    P: ServerProto<
        TcpStream,
        Request = <S as NewService>::Request,
        Response = <S as NewService>::Response,
    >,
    P::Error: Into<Error>,
{
    Binding(StartServer<P, S>),
    BindCancelled(StartServer<P, S>),
    Listening(ListeningServer<P, S::Instance>),
    ListenCancelled(ListeningServer<P, S::Instance>),
    Active(ActiveServer<P::Transport, S::Instance>),
    Disconnecting(ActiveServer<P::Transport, S::Instance>),
    Dead,
}

impl<P, S> AsyncServer<P, S>
where
    S: NewService,
    S::Request: 'static + Clone + Display + Eq + Hash,
    S::Response: 'static + Clone,
    S::Instance: FiniteService,
    S::Error: Into<Error>,
    P: ServerProto<
        TcpStream,
        Request = <S as NewService>::Request,
        Response = <S as NewService>::Response,
    >,
    P::Error: Into<Error>,
    P::Transport: Stream<Item = S::Request> + Sink<SinkItem = S::Response>,
    <P::Transport as Stream>::Error: Into<S::Error>,
{
    pub fn new(
        address: SocketAddr,
        service_factory: S,
        protocol: Arc<Mutex<P>>,
        handle: Handle,
    ) -> Self {
        AsyncServer::Binding(
            StartServer::new(address, service_factory, protocol, handle),
        )
    }

    pub fn shutdown(&mut self) -> Poll<(), Error> {
        let shutdown_result = match *self {
            AsyncServer::Binding(ref mut handler) => handler.shutdown(),
            AsyncServer::BindCancelled(ref mut handler) => {
                return handler.shutdown();
            }
            AsyncServer::Listening(ref mut handler) => handler.shutdown(),
            AsyncServer::ListenCancelled(ref mut handler) => {
                return handler.shutdown();
            }
            AsyncServer::Active(ref mut handler) => handler.shutdown(),
            AsyncServer::Disconnecting(ref mut handler) => {
                return handler.shutdown();
            }
            AsyncServer::Dead => Ok(Async::Ready(())),
        };

        let new_state = match shutdown_result {
            Ok(Async::NotReady) => {
                match mem::replace(self, AsyncServer::Dead) {
                    AsyncServer::Binding(handler) => {
                        AsyncServer::BindCancelled(handler)
                    }
                    AsyncServer::Listening(handler) => {
                        AsyncServer::ListenCancelled(handler)
                    }
                    AsyncServer::Active(handler) => {
                        AsyncServer::Disconnecting(handler)
                    }
                    AsyncServer::Dead => AsyncServer::Dead,
                    shutting_down_state => shutting_down_state,
                }
            }
            _ => AsyncServer::Dead,
        };

        mem::replace(self, new_state);

        shutdown_result
    }
}

impl<P, S> From<StartServer<P, S>> for AsyncServer<P, S>
where
    S: NewService,
    S::Request: 'static + Clone + Display + Eq + Hash,
    S::Response: 'static + Clone,
    S::Instance: FiniteService,
    P: ServerProto<
        TcpStream,
        Request = <S as NewService>::Request,
        Response = <S as NewService>::Response,
    >,
    P::Error: Into<Error>,
{
    fn from(start_server: StartServer<P, S>) -> Self {
        AsyncServer::Binding(start_server)
    }
}

impl<P, S> From<ListeningServer<P, S::Instance>> for AsyncServer<P, S>
where
    S: NewService,
    S::Request: 'static + Clone + Display + Eq + Hash,
    S::Response: 'static + Clone,
    S::Instance: FiniteService,
    P: ServerProto<
        TcpStream,
        Request = <S as NewService>::Request,
        Response = <S as NewService>::Response,
    >,
    P::Error: Into<Error>,
{
    fn from(listening_server: ListeningServer<P, S::Instance>) -> Self {
        AsyncServer::Listening(listening_server)
    }
}

impl<P, S> From<ActiveServer<P::Transport, S::Instance>> for AsyncServer<P, S>
where
    S: NewService,
    S::Request: 'static + Clone + Display + Eq + Hash,
    S::Response: 'static + Clone,
    S::Instance: FiniteService,
    P: ServerProto<
        TcpStream,
        Request = <S as NewService>::Request,
        Response = <S as NewService>::Response,
    >,
    P::Error: Into<Error>,
{
    fn from(active_server: ActiveServer<P::Transport, S::Instance>) -> Self {
        AsyncServer::Active(active_server)
    }
}

impl<P, S> Future for AsyncServer<P, S>
where
    S: NewService,
    S::Request: 'static + Clone + Display + Eq + Hash,
    S::Response: 'static + Clone,
    S::Error: Into<Error>,
    S::Instance: FiniteService,
    P: ServerProto<
        TcpStream,
        Request = <S as NewService>::Request,
        Response = <S as NewService>::Response,
    >,
    P::Error: Into<Error> + Into<S::Error>,
{
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let maybe_new_state = match *self {
            AsyncServer::Binding(ref mut handler) => {
                Some(AsyncServer::Listening(try_ready!(handler.poll())))
            }
            AsyncServer::Listening(ref mut handler) => {
                Some(AsyncServer::Active(try_ready!(handler.poll())))
            }
            AsyncServer::Active(ref mut handler) => {
                try_ready!(handler.poll());
                None
            }
            AsyncServer::Dead => {
                return Err(ErrorKind::AsyncServerWasShutDown.into());
            }
            _ => return Err(ErrorKind::AsyncServerIsShuttingDown.into()),
        };

        if let Some(new_state) = maybe_new_state {
            mem::replace(self, new_state);
            self.poll()
        } else {
            Ok(Async::Ready(()))
        }
    }
}
