use std::cmp::{Ordering, PartialOrd};

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
        *self > *other
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

impl<E> PartialEq for Status<E> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Status::Error(_), &Status::Error(_)) => false,
            (&Status::WouldBlock, &Status::WouldBlock) => true,
            (&Status::Finished, &Status::Finished) => true,
            (&Status::Active, &Status::Active) => true,
            _ => false,
        }
    }
}

impl<E> PartialOrd for Status<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = if self.eq(other) {
            Ordering::Equal
        } else {
            match (self, other) {
                (&Status::Error(_), _) => Ordering::Greater,
                (_, &Status::Error(_)) => Ordering::Less,
                (&Status::WouldBlock, _) => Ordering::Greater,
                (_, &Status::WouldBlock) => Ordering::Less,
                (&Status::Finished, _) => Ordering::Greater,
                _ => Ordering::Less,
            }
        };

        Some(ordering)
    }
}
