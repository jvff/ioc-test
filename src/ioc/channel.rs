use std::io;
use std::io::{Read, Write};

use futures::Poll;
use tokio_io::{AsyncRead, AsyncWrite};

pub struct Channel<I, O>
where
    I: Read,
    O: Write,
{
    source: I,
    sink: O,
}

impl<I, O> Channel<I, O>
where
    I: Read,
    O: Write,
{
    pub fn new(source: I, sink: O) -> Channel<I, O> {
        Self { source, sink }
    }
}

impl<I, O> Read for Channel<I, O>
where
    I: Read,
    O: Write,
{
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.source.read(buffer)
    }
}

impl<I, O> Write for Channel<I, O>
where
    I: Read,
    O: Write,
{
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.sink.write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.sink.flush()
    }
}

impl<I, O> AsyncRead for Channel<I, O>
where
    I: AsyncRead,
    O: Write,
{
}

impl<I, O> AsyncWrite for Channel<I, O>
where
    I: Read,
    O: AsyncWrite,
{
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.sink.shutdown()
    }
}
