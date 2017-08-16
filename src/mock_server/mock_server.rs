use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::pipeline::ServerProto;

use super::errors::Error;
use super::mock_server_start::MockServerStart;
use super::super::mock_service::MockServiceFactory;

pub struct MockServer<P>
where
    P: ServerProto<TcpStream>,
{
    address: SocketAddr,
    protocol: Arc<Mutex<P>>,
}

impl<P> MockServer<P>
where
    P: ServerProto<TcpStream>,
    P::Request: Clone + Display + Eq + Hash,
    P::Response: Clone,
    P::Error: Into<Error>,
{
    pub fn new(address: SocketAddr, protocol: P) -> MockServer<P> {
        Self {
            address,
            protocol: Arc::new(Mutex::new(protocol)),
        }
    }

    pub fn start(
        self,
        handle: Handle,
        request_map: Arc<Mutex<HashMap<P::Request, P::Response>>>,
        requests_to_verify: Arc<Mutex<HashSet<P::Request>>>,
    ) -> MockServerStart<P> {
        let address = self.address;
        let protocol = self.protocol;
        let service_factory =
            MockServiceFactory::new(request_map, requests_to_verify);

        MockServerStart::new(address, service_factory, protocol, handle)
    }
}
