use futures::{Async, Poll};

pub trait Test {
    type Error;

    fn name(&self) -> &str;
    fn poll_test(&mut self) -> Poll<(), Self::Error>;

    fn poll(&mut self) -> Poll<String, (String, Self::Error)> {
        match self.poll_test() {
            Ok(Async::Ready(())) => Ok(Async::Ready(String::from(self.name()))),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(error) => Err((String::from(self.name()), error)),
        }
    }
}

pub trait IntoTest {
    type Test: Test;

    fn into_test(self) -> Self::Test;
}
