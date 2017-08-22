#[macro_export]
macro_rules! tests {
    (
        type Protocol = $protocol:ty;
        $( $test:ident ($name:expr) $body:tt )*
    ) => {
        pub fn add_tests<S, P>(scheduler: &mut TestScheduler<S>)
        where
            P: IocTestParameters<
                Protocol = $protocol,
                Request = <$protocol as ServerProto<TcpStream>>::Request,
                Response = <$protocol as ServerProto<TcpStream>>::Response,
                ProtocolError = <$protocol as ServerProto<TcpStream>>::Error,
            >,
            P::ServiceError: From<<$protocol as ServerProto<TcpStream>>::Error>,
            S: TestSpawner<TestSetup = IocTestSetup<P>>,
        {
            $(scheduler.add(|mut $test| {
                $test.name($name);
                $body
            });)*
        }
    }
}

#[macro_export]
macro_rules! request_response_map {
    ( $object:expr , $($request:expr => $response:expr),+ $(,)* ) => {
        $($object.when($request).reply_with($response));+
    }
}
