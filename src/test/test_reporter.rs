use std::fmt::Display;

use futures::{Async, Future, Poll, Stream};
use termion::color::{Fg, Green, Red, Yellow};
use termion::style::{Bold, Reset};

use super::test_result::TestResult;

pub struct TestReporter<S, E>
where
    S: Stream<Item = TestResult<E>>,
    E: Display,
{
    successful_tests: usize,
    failed_tests: usize,
    test_results: S,
}

impl<S, E> TestReporter<S, E>
where
    S: Stream<Item = TestResult<E>>,
    E: Display,
{
    pub fn new(test_results: S) -> Self {
        TestReporter {
            successful_tests: 0,
            failed_tests: 0,
            test_results,
        }
    }

    fn report(&mut self, result: TestResult<E>) {
        match result {
            Ok(_) => self.successful_tests += 1,
            Err((ref test, ref error)) => {
                println!(
                    "{bold}{red}Fail: {reset}{yellow}{name}{reset}: {message}",
                    bold = Bold,
                    name = test,
                    message = error,
                    red = Fg(Red),
                    yellow = Fg(Yellow),
                    reset = Reset,
                );

                self.failed_tests += 1;
            }
        }
    }

    fn report_summary(&self) -> Poll<(), S::Error> {
        if self.failed_tests > 0 {
            println!("");
        }

        if self.successful_tests == 1 {
            println!(
                "{bold}1 test {green}succeeded{reset}",
                bold = Bold,
                green = Fg(Green),
                reset = Reset
            );
        } else if self.successful_tests > 1 {
            println!(
                "{bold}{count} tests {green}succeeded{reset}",
                bold = Bold,
                count = self.successful_tests,
                green = Fg(Green),
                reset = Reset
            );
        }

        if self.failed_tests == 1 {
            println!(
                "{bold}1 test {red}failed{reset}",
                bold = Bold,
                red = Fg(Red),
                reset = Reset
            );
        } else if self.failed_tests > 1 {
            println!(
                "{bold}{count} tests {red}failed{reset}",
                bold = Bold,
                count = self.failed_tests,
                red = Fg(Red),
                reset = Reset
            );
        }

        Ok(Async::Ready(()))
    }
}

impl<S, E> Future for TestReporter<S, E>
where
    S: Stream<Item = TestResult<E>>,
    E: Display,
{
    type Item = ();
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut poll_result = self.test_results.poll();

        while let Ok(Async::Ready(Some(result))) = poll_result {
            self.report(result);

            poll_result = self.test_results.poll();
        }

        match poll_result {
            Ok(Async::Ready(Some(_))) => {
                unreachable!(
                    "All available test results should have been reported"
                );
            }
            Ok(Async::Ready(None)) => self.report_summary(),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(error) => Err(error),
        }
    }
}
