use futures::{Async, Poll};

pub type TestResult<E> = Result<String, (String, E)>;

pub trait TestResultMethods<E> {
    fn from_poll(poll_result: Poll<String, (String, E)>) -> Self;
    fn success(test: String) -> Self;
    fn failure<F: Into<E>>(test: String, error: F) -> Self;
    fn name(&self) -> &str;
}

impl<E> TestResultMethods<E> for TestResult<E> {
    fn from_poll(poll_result: Poll<String, (String, E)>) -> Self {
        match poll_result {
            Err(error) => Err(error),
            Ok(Async::Ready(test)) => Ok(test),
            Ok(Async::NotReady) => {
                panic!("Attempt to get test result of unfinished test");
            }
        }
    }

    fn success(test: String) -> Self {
        Ok(test)
    }

    fn failure<F>(test: String, error: F) -> Self
    where
        F: Into<E>,
    {
        Err((test, error.into()))
    }

    fn name(&self) -> &str {
        match *self {
            Ok(ref test) => test.as_str(),
            Err((ref test, _)) => test.as_str(),
        }
    }
}
