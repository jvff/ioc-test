use super::setup::Protocol;
use super::super::ioc_test::IocTestSetup;
use super::super::scpi::ScpiRequest;
use super::super::scpi::ScpiResponse;
use super::super::test_scheduler::TestScheduler;
use super::super::test_spawner::TestSpawner;

tests! {
    test("enable channel output") {
        test.set_variable("channelOutput-Sel", "ON");

        test.when(ScpiRequest::OutputOn(1))
            .reply_with(ScpiResponse::Empty)
            .verify();
    }

    test("disable channel output") {
        test.set_variable("channelOutput-Sel", "OFF");

        test.when(ScpiRequest::OutputOff(1))
            .reply_with(ScpiResponse::Empty)
            .verify();
    }

    test("read channel output status") {
        test.set_variable("channelOutput-Sts", "");

        test.when(ScpiRequest::OutputStatus(1))
            .reply_with(ScpiResponse::Utf8String(String::from("ON")))
            .verify();
    }
}
