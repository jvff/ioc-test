#[macro_export]
macro_rules! tests {
    (
        generic type TestSetup < $($generic_parameter:ident),* $(,)* > =
            $test_setup:path
        where $($constrained:path : $constraint:path),* $(,)*;
        $( $test:ident ($name:expr) $body:tt )*
    ) => {
        mod tests {
            use super::*;

            pub fn create_tests<$($generic_parameter),*>(
            ) -> Vec<Box<FnMut(&mut $test_setup)>>
            where
                $($constrained : $constraint),*
            {
                let mut tests: Vec<Box<FnMut(&mut $test_setup)>> = Vec::new();

                $(tests.push(Box::new(|mut $test| {
                    $test.name($name);
                    $body
                }));)*

                tests
            }
        }

        pub use self::tests::create_tests;
    }
}
