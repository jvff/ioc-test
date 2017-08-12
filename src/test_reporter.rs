use futures::{Async, Future, Poll};
use termion::color::{Fg, Green, Red, Yellow};
use termion::style::{Bold, Reset};

use super::test::{IntoTest, Test};
use super::test_result::TestResult;
use super::test_scheduler::TestScheduler;
use super::test_spawner::TestSpawner;

pub struct TestReporter<S>
where
    S: TestSpawner,
{
    scheduler: TestScheduler<S>,
}

impl<S> TestReporter<S>
where
    S: TestSpawner,
{
    pub fn new(scheduler: TestScheduler<S>) -> Self {
        TestReporter { scheduler }
    }

    fn report(
        &self,
        test_results: Vec<
            TestResult<<<S::TestSetup as IntoTest>::Test as Test>::Error>
        >,
) -> Poll<(), ()>{
        let mut successful_tests = 0;
        let mut failed_tests = 0;

        for test_result in test_results {
            match test_result {
                Ok(_) => successful_tests += 1,
                Err((ref test, ref error)) => {
                    println!(
                        "{bold}{red}Fail: {reset}{yellow}{name}{reset}: \
                         {message}",
                        bold = Bold,
                        name = test,
                        message = error,
                        red = Fg(Red),
                        yellow = Fg(Yellow),
                        reset = Reset,
                    );
                    failed_tests += 1;
                }
            }
        }

        if failed_tests > 0 {
            println!("");
        }

        if successful_tests == 1 {
            println!(
                "{bold}1 test {green}succeeded{reset}",
                bold = Bold,
                green = Fg(Green),
                reset = Reset
            );
        } else if successful_tests > 1 {
            println!(
                "{bold}{count} tests {green}succeeded{reset}",
                bold = Bold,
                count = successful_tests,
                green = Fg(Green),
                reset = Reset
            );
        }

        if failed_tests == 1 {
            println!(
                "{bold}1 test {red}failed{reset}",
                bold = Bold,
                red = Fg(Red),
                reset = Reset
            );
        } else if failed_tests > 1 {
            println!(
                "{bold}{count} tests {red}failed{reset}",
                bold = Bold,
                count = failed_tests,
                red = Fg(Red),
                reset = Reset
            );
        }

        Ok(Async::Ready(()))
    }
}

impl<S> Future for TestReporter<S>
where
    S: TestSpawner,
{
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.scheduler.poll() {
            Ok(Async::Ready(results)) => self.report(results),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(()) => Err(()),
        }
    }
}
