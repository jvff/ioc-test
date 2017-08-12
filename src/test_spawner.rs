use futures::IntoFuture;

pub trait TestSpawner {
    type Test: IntoFuture;

    fn spawn(&mut self) -> Self::Test;
}
