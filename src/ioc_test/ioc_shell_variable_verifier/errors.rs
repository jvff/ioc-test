use super::super::super::ioc::{EpicsDataType, IocShellCommand};

error_chain! {
    errors {
        UnexpectedIocShellCommand(command: IocShellCommand) {
            description("received an unexpected IOC shell command")
            display("received an unexpected IOC shell command: \"{}\"", command)
        }

        IncorrectIocShellCommand(
            received: IocShellCommand,
            expected: IocShellCommand
        ) {
            description("received an incorrect IOC shell command")
            display(
                "received an incorrect IOC shell command: received \"{}\" but \
                 expected \"{}\"",
                received,
                expected,
            )
        }

        UnexpectedIocShellVariableValue(value: EpicsDataType) {
            description("received unexpected PV value from IOC shell")
            display("received unexpected PV from IOC shell: \"{}\"", value)
        }

        IncorrectIocShellVariableValue(
            received: EpicsDataType,
            expected: EpicsDataType
        ) {
            description("received incorrect IOC shell PV value")
            display(
                "received incorrect IOC shell PV value: received \"{}\" but \
                 expected \"{}\"",
                received,
                expected,
            )
        }

        UnverifiedIocShellCommands(
            first_command: String,
            unverified_commands: usize
        ) {
            description("some expected IOC shell commands were not verified")
            display(
                "{} IOC shell commands were not verified, starting at {}",
                unverified_commands,
                first_command,
            )
        }

        UnverifiedIocShellCommandsAndVariableValues(
            first_command: String,
            unverified_commands: usize,
            unverified_results_of_verified_commands: usize
        ) {
            description(
                "some expected IOC shell commands were not verified, and some \
                 verified IOC shell commands did not have their resulting PV \
                 values verified"
            )
            display(
                "{} IOC shell commands were not verified, starting at {}, and \
                 the resulting PV values of {} verified commands were not \
                 verified",
                unverified_commands,
                first_command,
                unverified_results_of_verified_commands,
            )
        }

        UnverifiedIocShellVariableValues(remaining_results: usize) {
            description(
                "some expected IOC shell command PV values were not verified"
            )
            display(
                "{} IOC shell commands did not have their resulting PV values \
                 verified",
                remaining_results,
            )
        }
    }
}
