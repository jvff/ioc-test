use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use tokio_core::net::TcpStream;
use tokio_proto::pipeline::ServerProto;

use super::errors::Error;
use super::ioc_test_parameters::IocTestParameters;
use super::super::{async_server, mock_service};
use super::super::instrumenting_service::{InstrumentingService,
                                          ServiceInstrumenter, WhenVerifier};
use super::super::instrumenting_service::verifiers;
use super::super::instrumenting_service::verifiers::{BoxedVerifier,
                                                     BoxedVerifierFactory,
                                                     EventuallyVerify,
                                                     VerifyAll};
use super::super::mock_service::{MockService, MockServiceFactory};

#[derive(Clone)]
pub struct MockTestParameters<P>
where
    P: ServerProto<TcpStream>,
{
    protocol: P,
}

impl<P> MockTestParameters<P>
where
    P: ServerProto<TcpStream>,
{
    pub fn new(protocol: P) -> Self {
        Self { protocol }
    }
}

impl<P> IocTestParameters for MockTestParameters<P>
where
    P: Clone + ServerProto<TcpStream>,
    P::Request: Clone + Debug + Display + Eq + Hash,
    P::Response: Clone + Debug + Eq,
    P::Error: Into<async_server::Error> + Into<Error>,
    mock_service::Error: From<P::Error>,
{
    type Request = P::Request;
    type Response = P::Response;
    type ProtocolError = P::Error;
    type Protocol = P;
    type ServiceError = mock_service::Error;
    type Service = InstrumentingService<
        MockService<P::Request, P::Response>,
        BoxedVerifier<'static, P::Request, P::Response, verifiers::Error>,
        mock_service::Error,
    >;
    type ServiceFactory = ServiceInstrumenter<
        MockServiceFactory<P::Request, P::Response>,
        BoxedVerifierFactory<
            'static,
            P::Request,
            P::Response,
            verifiers::Error,
        >,
        mock_service::Error,
    >;

    fn create_protocol(&self) -> Self::Protocol {
        self.protocol.clone()
    }

    fn create_service_factory(
        &self,
        expected_requests: HashMap<Self::Request, Self::Response>,
        verifier_factory: EventuallyVerify<
            VerifyAll<WhenVerifier<Self::Request, Self::Response>>,
        >,
    ) -> Self::ServiceFactory {
        let mock_service_factory =
            MockServiceFactory::new(Arc::new(Mutex::new(expected_requests)));
        let boxed_verifier_factory =
            BoxedVerifierFactory::new(verifier_factory);

        ServiceInstrumenter::new(mock_service_factory, boxed_verifier_factory)
    }
}
