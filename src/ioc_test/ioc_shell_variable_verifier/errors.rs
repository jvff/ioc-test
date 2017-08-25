use super::super::super::ioc::IocShellCommand;

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

        UnexpectedIocShellOutput(output: String) {
            description("received unexpected output from IOC shell")
            display("received unexpected output from IOC shell: \"{}\"", output)
        }

        IncorrectIocShellOutput(received: String, expected: String) {
            description("received incorrect IOC shell output")
            display(
                "received incorrect IOC shell output: received \"{}\" but \
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

        UnverifiedIocShellCommandsAndOutputs(
            first_command: String,
            unverified_commands: usize,
            unverified_outputs_of_verified_commands: usize
        ) {
            description(
                "some expected IOC shell commands were not verified, and some \
                 verified IOC shell commands did not have their output verified"
            )
            display(
                "{} IOC shell commands were not verified, starting at {}, and \
                 the output of {} verified commands were not verified",
                unverified_commands,
                first_command,
                unverified_outputs_of_verified_commands,
            )
        }

        UnverifiedIocShellOutputs(remaining_outputs: usize) {
            description(
                "some expected IOC shell command outputs were not verified"
            )
            display(
                "{} IOC shell commands did not have their output verified",
                remaining_outputs,
            )
        }
    }
}
