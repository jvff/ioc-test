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
        IocShellAccessError {
            description("failed to access child IOC process shell")
        }

        IocShellReadError {
            description("no more input could be read from IOC shell process \
                         stdout")
        }

        IocShellWriteError {
            description("failed to write to child IOC shell process stdin")
        }

        UnexpectedIocShellOutput {
            description("unexpected IOC shell output block was read")
        }

        IocShellServiceLockError {
            description("another thread panicked while holding a lock to the \
                         IOC shell service scheduler")
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

        IocShellCommandOutputPolledAfterError {
            description("IOC shell command output future polled after it had \
                         already returned an error")
        }
    }
}
