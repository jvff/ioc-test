use std::iter::{IntoIterator, Iterator};

use futures::{Async, Future, Poll};
use tokio_service::Service;

use super::ioc_test_variable_action::IocTestVariableAction;

pub struct IocVariableActionService<I, S>
where
    I: Iterator<Item = IocTestVariableAction>,
    S: Service,
    IocTestVariableAction: Into<S::Request>,
{
    variable_actions: I,
    ioc_variable_service: S,
    active_request: Option<S::Future>,
}

impl<I, S> IocVariableActionService<I, S>
where
    I: Iterator<Item = IocTestVariableAction>,
    S: Service,
    IocTestVariableAction: Into<S::Request>,
{
    pub fn new<T>(variable_actions: T, ioc_variable_service: S) -> Self
    where
        T: IntoIterator<IntoIter = I, Item = IocTestVariableAction>,
    {
        Self {
            ioc_variable_service,
            variable_actions: variable_actions.into_iter(),
            active_request: None,
        }
    }

    fn send_next_request(&mut self) {
        if let Some(variable_action) = self.variable_actions.next() {
            let request = variable_action.into();
            let active_request = self.ioc_variable_service.call(request);

            self.active_request = Some(active_request);
        }
    }

    fn poll_active_request(&mut self) -> Poll<(), S::Error> {
        if let Some(ref mut active_request) = self.active_request {
            try_ready!(active_request.poll());
        }

        Ok(Async::Ready(()))
    }
}

impl<I, S> Future for IocVariableActionService<I, S>
where
    I: Iterator<Item = IocTestVariableAction>,
    S: Service,
    IocTestVariableAction: Into<S::Request>,
{
    type Item = ();
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if self.active_request.is_none() {
            self.send_next_request();
        }

        while self.active_request.is_some() {
            try_ready!(self.poll_active_request());

            self.send_next_request();
        }

        Ok(Async::Ready(()))
    }
}
