use super::test::IntoTest;

pub trait TestSpawner {
    type TestSetup: IntoTest;

    fn spawn(&mut self) -> Self::TestSetup;
}
