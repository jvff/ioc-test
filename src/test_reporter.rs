use futures::{Async, Future, Poll, Stream};
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
    successful_tests: usize,
    failed_tests: usize,
    scheduler: TestScheduler<S>,
}

impl<S> TestReporter<S>
where
    S: TestSpawner,
{
    pub fn new(scheduler: TestScheduler<S>) -> Self {
        TestReporter {
            successful_tests: 0,
            failed_tests: 0,
            scheduler,
        }
    }

    fn report(
        &mut self,
        result: TestResult<<<S::TestSetup as IntoTest>::Test as Test>::Error>,
){
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

    fn report_summary(&self) -> Poll<(), ()> {
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

impl<S> Future for TestReporter<S>
where
    S: TestSpawner,
{
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut poll_result = self.scheduler.poll();

        while let Ok(Async::Ready(Some(result))) = poll_result {
            self.report(result);

            poll_result = self.scheduler.poll();
        }

        match self.scheduler.poll() {
            Ok(Async::Ready(Some(_))) => {
                unreachable!(
                    "All available test results should have been reported"
                );
            }
            Ok(Async::Ready(None)) => self.report_summary(),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(()) => Err(()),
        }
    }
}
