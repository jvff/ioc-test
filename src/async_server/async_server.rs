use std::fmt::Display;
use std::hash::Hash;
use std::mem;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use futures::{Async, Future, Poll};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::pipeline::ServerProto;
use tokio_service::NewService;

use super::active_server::ActiveServer;
use super::errors::Error;
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
    Listening(ListeningServer<P, S::Instance>),
    Active(ActiveServer<P::Transport, S::Instance>),
}

impl<P, S> AsyncServer<P, S>
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
        };

        if let Some(new_state) = maybe_new_state {
            mem::replace(self, new_state);
            self.poll()
        } else {
            Ok(Async::Ready(()))
        }
    }
}
