#[macro_export]
macro_rules! tests {
    (
        type Protocol = $protocol:ty;
        $( $test:ident ($name:expr) $body:tt )*
    ) => {
        mod tests {
            use tokio_core::net::TcpStream;
            use tokio_proto::pipeline::ServerProto;

            use ioc_test::{IocTestParameters, IocTestSetup, TestScheduler,
                           TestSpawner};

            use super::*;

            pub fn add_tests<S, P>(scheduler: &mut TestScheduler<S>)
            where
                P: IocTestParameters<
                    Protocol = $protocol,
                    Request = <$protocol as ServerProto<TcpStream>>::Request,
                    Response = <$protocol as ServerProto<TcpStream>>::Response,
                    ProtocolError =
                        <$protocol as ServerProto<TcpStream>>::Error,
                >,
                P::ServiceError: From<
                    <$protocol as ServerProto<TcpStream>>::Error
                >,
                S: TestSpawner<TestSetup = IocTestSetup<P>>,
            {
                $(scheduler.add(|mut $test| {
                    $test.name($name);
                    $body
                });)*
            }
        }

        pub use self::tests::add_tests;
    }
}

#[macro_export]
macro_rules! request_response_map {
    ( $object:expr , $($request:expr => $response:expr),+ $(,)* ) => {
        $($object.when($request).reply_with($response));+
    }
}
