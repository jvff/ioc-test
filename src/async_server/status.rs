use futures::{Async, AsyncSink, Poll, StartSend};

use super::errors::{Error, ErrorKind};

#[derive(Debug)]
pub enum Status<E> {
    Active,
    Finished,
    WouldBlock,
    Error(E),
}

impl<E> Status<E> {
    pub fn is_active(&self) -> bool {
        match *self {
            Status::Active => true,
            _ => false,
        }
    }

    pub fn is_running(&self) -> bool {
        match *self {
            Status::Active => true,
            Status::WouldBlock => true,
            _ => false,
        }
    }

    pub fn update<T: Into<Status<E>>>(&mut self, status_update: T) {
        let status_update = status_update.into();

        if status_update.is_more_severe_than(self) {
            *self = status_update;
        }
    }

    fn is_more_severe_than(&self, other: &Status<E>) -> bool {
        match (self, other) {
            (_, &Status::Error(_)) => false,
            (&Status::Error(_), _) => true,
            (_, &Status::WouldBlock) => false,
            (&Status::WouldBlock, _) => true,
            (_, &Status::Finished) => false,
            _ => true,
        }
    }
}

impl<E, U, V> From<Poll<U, V>> for Status<E>
where
    V: Into<E>,
{
    fn from(poll: Poll<U, V>) -> Status<E> {
        match poll {
            Ok(Async::Ready(_)) => Status::Active,
            Ok(Async::NotReady) => Status::WouldBlock,
            Err(error) => Status::Error(error.into()),
        }
    }
}

impl<E, U, V> From<StartSend<U, V>> for Status<E>
where
    V: Into<E>,
{
    fn from(start_send: StartSend<U, V>) -> Status<E> {
        match start_send {
            Ok(AsyncSink::Ready) => Status::Active,
            Ok(AsyncSink::NotReady(_)) => Status::WouldBlock,
            Err(error) => Status::Error(error.into()),
        }
    }
}

impl<E> Into<Poll<(), Error>> for Status<E>
where
    E: Into<Error>,
{
    fn into(self) -> Poll<(), Error> {
        match self {
            Status::Finished => Ok(Async::Ready(())),
            Status::WouldBlock => Ok(Async::NotReady),
            Status::Error(error) => Err(error.into()),
            Status::Active => {
                let error_type = ErrorKind::ActiveStatusHasNoPollEquivalent;
                let error: Error = error_type.into();

                Err(error.into())
            }
        }
    }
}
