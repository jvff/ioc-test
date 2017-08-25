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
    }
}
