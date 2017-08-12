use futures::{Async, Future, Poll};
use super::test::{IntoTest, Test};
use super::test_result::{TestResult, TestResultMethods};
use super::test_spawner::TestSpawner;

pub struct TestScheduler<S>
where
    S: TestSpawner,
{
    spawner: S,
    test_queue: Vec<Box<FnMut(&mut S::TestSetup)>>,
    test_executions: Vec<<S::TestSetup as IntoTest>::Test>,
    test_results:
        Vec<TestResult<<<S::TestSetup as IntoTest>::Test as Test>::Error>>,
}

impl<S> TestScheduler<S>
where
    S: TestSpawner,
{
    pub fn new(spawner: S) -> Self {
        Self {
            spawner,
            test_queue: Vec::new(),
            test_executions: Vec::new(),
            test_results: Vec::new(),
        }
    }

    pub fn add<F>(&mut self, test_setup: F)
    where
        F: FnMut(&mut S::TestSetup) + 'static,
    {
        self.test_queue.push(Box::new(test_setup));
    }

    fn start_queued_tests(&mut self) {
        for mut test_setup_function in self.test_queue.drain(0..) {
            let mut test_setup = self.spawner.spawn();

            test_setup_function(&mut test_setup);

            self.test_executions.push(test_setup.into_test());
        }
    }

    fn poll_tests(&mut self) {
        let test_executions_count = self.test_executions.len();
        let poll_results = self.test_executions
            .iter_mut()
            .map(|execution| execution.poll())
            .zip(0..test_executions_count)
            .rev()
            .collect::<Vec<_>>();

        for (poll_result, index) in poll_results {
            match poll_result {
                Ok(Async::NotReady) => {}
                poll_result => {
                    self.test_results.push(TestResult::from_poll(poll_result));
                    self.test_executions.remove(index);
                }
            }
        }
    }

    fn all_tests_finished(&self) -> bool {
        self.test_queue.is_empty() && self.test_executions.is_empty()
    }
}

impl<S> Future for TestScheduler<S>
where
    S: TestSpawner,
{
    type Item =
        Vec<TestResult<<<S::TestSetup as IntoTest>::Test as Test>::Error>>;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.start_queued_tests();
        self.poll_tests();

        if self.all_tests_finished() {
            Ok(Async::Ready(self.test_results.drain(..).collect()))
        } else {
            Ok(Async::NotReady)
        }
    }
}
