use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use futures::stream::{SplitSink, SplitStream};
use tokio_core::net::TcpStream;
use tokio_proto::pipeline::{ClientProto, ServerProto};

use super::errors::Error;
use super::ioc_test_parameters::IocTestParameters;
use super::super::{async_server, proxy_service};
use super::super::instrumenting_service::{InstrumentingService,
                                          ServiceInstrumenter, WhenVerifier};
use super::super::instrumenting_service::verifiers;
use super::super::instrumenting_service::verifiers::{BoxedVerifier,
                                                     BoxedVerifierFactory,
                                                     EventuallyVerify,
                                                     VerifyAll};
use super::super::proxy_service::{ProxyService, ProxyServiceFactory};

#[derive(Clone)]
pub struct ProxyTestParameters<P>
where
    P: Clone + ClientProto<TcpStream> + ServerProto<TcpStream>,
{
    protocol: P,
    source: Arc<Mutex<SplitStream<<P as ClientProto<TcpStream>>::Transport>>>,
    sink: Arc<Mutex<SplitSink<<P as ClientProto<TcpStream>>::Transport>>>,
}

impl<P> ProxyTestParameters<P>
where
    P: Clone + ClientProto<TcpStream> + ServerProto<TcpStream>,
{
    pub fn new(
        protocol: P,
        source: SplitStream<<P as ClientProto<TcpStream>>::Transport>,
        sink: SplitSink<<P as ClientProto<TcpStream>>::Transport>,
    ) -> Self {
        Self {
            protocol,
            source: Arc::new(Mutex::new(source)),
            sink: Arc::new(Mutex::new(sink)),
        }
    }
}

impl<P> IocTestParameters for ProxyTestParameters<P>
where
    P: Clone
        + ServerProto<TcpStream>
        + ClientProto<
        TcpStream,
        Request = <P as ServerProto<TcpStream>>::Request,
        Response = <P as ServerProto<TcpStream>>::Response,
        Error = <P as ServerProto<TcpStream>>::Error,
    >,
    <P as ServerProto<TcpStream>>::Request: Clone + Debug + Display + Eq + Hash,
    <P as ServerProto<TcpStream>>::Response: Clone + Debug + Eq,
    <P as ServerProto<TcpStream>>::Error: Into<async_server::Error> + Into<Error>,
    proxy_service::Error: From<<P as ServerProto<TcpStream>>::Error>,
{
    type Request = <P as ServerProto<TcpStream>>::Request;
    type Response = <P as ServerProto<TcpStream>>::Response;
    type ProtocolError = <P as ServerProto<TcpStream>>::Error;
    type Protocol = P;
    type ServiceError = proxy_service::Error;
    type Service = InstrumentingService<
        ProxyService<
            SplitStream<<P as ClientProto<TcpStream>>::Transport>,
            SplitSink<<P as ClientProto<TcpStream>>::Transport>,
        >,
        BoxedVerifier<'static, Self::Request, Self::Response, verifiers::Error>,
        proxy_service::Error,
    >;
    type ServiceFactory = ServiceInstrumenter<
        ProxyServiceFactory<
            SplitStream<<P as ClientProto<TcpStream>>::Transport>,
            SplitSink<<P as ClientProto<TcpStream>>::Transport>,
        >,
        BoxedVerifierFactory<
            'static,
            Self::Request,
            Self::Response,
            verifiers::Error,
        >,
        proxy_service::Error,
    >;

    fn create_protocol(&self) -> Self::Protocol {
        self.protocol.clone()
    }

    fn create_service_factory(
        &self,
        _expected_requests: HashMap<Self::Request, Self::Response>,
        verifier_factory: EventuallyVerify<
            VerifyAll<WhenVerifier<Self::Request, Self::Response>>,
        >,
    ) -> Self::ServiceFactory {
        let proxy_service_factory =
            ProxyServiceFactory::new(self.source.clone(), self.sink.clone());
        let boxed_verifier_factory =
            BoxedVerifierFactory::new(verifier_factory);

        ServiceInstrumenter::new(proxy_service_factory, boxed_verifier_factory)
    }
}
