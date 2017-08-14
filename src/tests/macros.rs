#[macro_export]
macro_rules! tests {
    ( $( $test:ident ($name:expr) $body:tt )* ) => {
        pub fn add_tests<S, P>(scheduler: &mut TestScheduler<S>)
        where
            S: TestSpawner<TestSetup = IocTestSetup<P>>,
            P: IocTestProtocol,
        {
            $(scheduler.add(|mut $test| {
                $test.name($name);
                $body
            });)*
        }
    }
}
