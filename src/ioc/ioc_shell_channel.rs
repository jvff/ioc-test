use futures::{Poll, Sink, StartSend, Stream};
use tokio_io::AsyncRead;
use tokio_io::codec::Framed;
use tokio_process::{Child, ChildStdin, ChildStdout};

use super::channel::Channel;
use super::errors::Error;
use super::ioc_shell_codec::IocShellCodec;
use super::ioc_shell_command::IocShellCommand;

pub struct IocShellChannel {
    channel: Framed<Channel<ChildStdout, ChildStdin>, IocShellCodec>,
}

impl IocShellChannel {
    pub fn from(ioc_process: &mut Child) -> Option<IocShellChannel> {
        if let Some(stdin) = ioc_process.stdin().take() {
            if let Some(stdout) = ioc_process.stdout().take() {
                let channel = Channel::new(stdout, stdin).framed(IocShellCodec);

                return Some(IocShellChannel { channel });
            }
        }

        None
    }
}

impl Stream for IocShellChannel {
    type Item = String;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.poll_complete()?;
        Ok(self.channel.poll()?)
    }
}

impl Sink for IocShellChannel {
    type SinkItem = IocShellCommand;
    type SinkError = Error;

    fn start_send(
        &mut self,
        command: Self::SinkItem,
    ) -> StartSend<Self::SinkItem, Self::SinkError> {
        Ok(self.channel.start_send(command)?)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(self.channel.poll_complete()?)
    }
}
