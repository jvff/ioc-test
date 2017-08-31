#[macro_export]
macro_rules! ioc_tests {
    (
        type Protocol = $protocol:path;
        $( $test:ident ($name:expr) $body:tt )*
    ) => {
        mod ioc_tests {
            use ioc_test::{IocTestParameters, IocTestSetup};
            use tokio_core::net::TcpStream;
            use tokio_proto::pipeline::ServerProto;

            use super::*;

            tests! {
                generic type TestSetup<P> = IocTestSetup<P>
                where
                    P: IocTestParameters<
                        Protocol = $protocol,
                        Request =
                            <$protocol as ServerProto<TcpStream>>::Request,
                        Response =
                            <$protocol as ServerProto<TcpStream>>::Response,
                        ProtocolError =
                            <$protocol as ServerProto<TcpStream>>::Error,
                    >,
                    P::ServiceError: From<P::ProtocolError>;

                $( $test($name) $body )*
            }
        }

        pub use self::ioc_tests::create_tests;
    }
}

#[macro_export]
macro_rules! request_response_map {
    ( $object:expr , $($request:expr => $response:expr),+ $(,)* ) => {
        $($object.when($request).reply_with($response));+
    }
}
