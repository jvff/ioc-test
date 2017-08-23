use std::io;

use super::ioc_shell_codec;

error_chain! {
    foreign_links {
        Io(io::Error);
    }

    links {
        IocShellCodec(ioc_shell_codec::Error, ioc_shell_codec::ErrorKind);
    }

    errors {
        IocStdinAccessError {
            description("failed to access child IOC process standard input")
        }

        IocStdoutAccessError {
            description("failed to access child IOC process standard output")
        }

        IocWriteError {
            description("failed to write to child IOC process standard input")
        }

        SettingIocVariable {
            description("concurrent access while setting IOC variable")
        }

        IocInstancePolledAfterEnd {
            description("IOC instance Future was polled after it ended")
        }

        IocProcessPolledAfterEnd {
            description("IOC process Future was polled after it ended")
        }

        IocProcessPolledWhileCheckingForError {
            description("IOC process Future was polled while checking for \
                         error")
        }
    }
}
