#[macro_export]
macro_rules! tests {
    (
        type Protocol = $protocol:ident;
        $( $test:ident ($name:expr) $body:tt )*
    ) => {
        pub fn add_tests<S>(scheduler: &mut TestScheduler<S>)
        where
            S: TestSpawner<TestSetup = IocTestSetup<$protocol>>,
        {
            $(scheduler.add(|mut $test| {
                $test.name($name);
                $body
            });)*
        }
    }
}
