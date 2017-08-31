use std::collections::VecDeque;

use futures::{Async, Poll, Stream};
use super::test::{IntoTest, Test};
use super::test_result::{TestResult, TestResultMethods};
use super::test_spawner::TestSpawner;

pub struct SequentialTestScheduler<S>
where
    S: TestSpawner,
{
    spawner: S,
    test_queue: VecDeque<Box<FnMut(&mut S::TestSetup)>>,
    test_execution: Option<<S::TestSetup as IntoTest>::Test>,
}

impl<S> SequentialTestScheduler<S>
where
    S: TestSpawner,
{
    pub fn new(spawner: S) -> Self {
        Self {
            spawner,
            test_queue: VecDeque::new(),
            test_execution: None,
        }
    }

    pub fn add<F>(&mut self, test_setup: F)
    where
        F: Into<Box<FnMut(&mut S::TestSetup)>>,
    {
        self.test_queue.push_back(test_setup.into());
    }

    pub fn add_all<F, I>(&mut self, test_setup_functions: I)
    where
        F: Into<Box<FnMut(&mut S::TestSetup)>>,
        I: IntoIterator<Item = F>,
    {
        let mut new_tests = test_setup_functions
            .into_iter()
            .map(|test| test.into())
            .collect();

        self.test_queue.append(&mut new_tests)
    }

    fn start_next_test(&mut self) {
        self.test_execution =
            self.test_queue.pop_front().map(|mut test_setup_function| {
                let mut test_setup = self.spawner.spawn();

                test_setup_function(&mut test_setup);

                test_setup.into_test()
            });
    }

    fn all_tests_finished(&self) -> bool {
        self.test_execution.is_none() && self.test_queue.is_empty()
    }
}

impl<S> Stream for SequentialTestScheduler<S>
where
    S: TestSpawner,
{
    type Item = TestResult<<<S::TestSetup as IntoTest>::Test as Test>::Error>;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        while !self.all_tests_finished() {
            if let Some(mut test_execution) = self.test_execution.take() {
                match test_execution.poll() {
                    Ok(Async::NotReady) => {
                        self.test_execution = Some(test_execution);

                        return Ok(Async::NotReady);
                    }
                    poll_result => {
                        let test_result = TestResult::from_poll(poll_result);

                        return Ok(Async::Ready(Some(test_result)));
                    }
                }
            }

            self.start_next_test();
        }

        Ok(Async::Ready(None))
    }
}
