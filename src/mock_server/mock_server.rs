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
    service_factory: MockServiceFactory<P::Request, P::Response>,
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
            service_factory: MockServiceFactory::new(),
            protocol: Arc::new(Mutex::new(protocol)),
        }
    }

    pub fn start(
        mut self,
        handle: Handle,
        request_map: Arc<Mutex<HashMap<P::Request, P::Response>>>,
        requests_to_verify: Arc<Mutex<HashSet<P::Request>>>,
    ) -> MockServerStart<P> {
        let address = self.address;
        let protocol = self.protocol;
        let mut service_factory = self.service_factory;

        let mut request_map = request_map.lock().expect(
            "another thread panicked while holding a lock to the mock request \
             map",
        );
        let mut requests_to_verify = requests_to_verify.lock().expect(
            "another thread panicked while holding a lock to the mock request \
             verification set",
        );

        for (request, response) in request_map.drain() {
            service_factory.when(request).reply_with(response);
        }

        for request in requests_to_verify.drain() {
            service_factory.verify(request);
        }

        MockServerStart::new(address, service_factory, protocol, handle)
    }
}
