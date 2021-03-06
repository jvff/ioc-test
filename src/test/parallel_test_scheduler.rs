use futures::{Async, Poll, Stream};
use super::test::{IntoTest, Test};
use super::test_result::{TestResult, TestResultMethods};
use super::test_spawner::TestSpawner;

pub struct ParallelTestScheduler<S>
where
    S: TestSpawner,
{
    spawner: S,
    test_queue: Vec<Box<FnMut(&mut S::TestSetup)>>,
    test_executions: Vec<<S::TestSetup as IntoTest>::Test>,
}

impl<S> ParallelTestScheduler<S>
where
    S: TestSpawner,
{
    pub fn new(spawner: S) -> Self {
        Self {
            spawner,
            test_queue: Vec::new(),
            test_executions: Vec::new(),
        }
    }

    pub fn add<F>(&mut self, test_setup: F)
    where
        F: Into<Box<FnMut(&mut S::TestSetup)>>,
    {
        self.test_queue.push(test_setup.into());
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

    fn start_queued_tests(&mut self) {
        for mut test_setup_function in self.test_queue.drain(0..) {
            let mut test_setup = self.spawner.spawn();

            test_setup_function(&mut test_setup);

            self.test_executions.push(test_setup.into_test());
        }
    }

    fn next_test_result(&mut self) -> Poll<Option<<Self as Stream>::Item>, ()> {
        let next_ready_result = self.test_executions
            .iter_mut()
            .zip(0..)
            .filter_map(|(execution, index)| match execution.poll() {
                Ok(Async::NotReady) => None,
                poll_result => Some((poll_result, index)),
            })
            .next();

        if let Some((poll_result, index)) = next_ready_result {
            self.test_executions.remove(index);

            Ok(Async::Ready(Some(TestResult::from_poll(poll_result))))
        } else {
            if self.all_tests_finished() {
                Ok(Async::Ready(None))
            } else {
                Ok(Async::NotReady)
            }
        }
    }

    fn all_tests_finished(&self) -> bool {
        self.test_queue.is_empty() && self.test_executions.is_empty()
    }
}

impl<S> Stream for ParallelTestScheduler<S>
where
    S: TestSpawner,
{
    type Item = TestResult<<<S::TestSetup as IntoTest>::Test as Test>::Error>;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.start_queued_tests();

        self.next_test_result()
    }
}
