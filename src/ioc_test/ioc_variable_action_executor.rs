use std::iter::{IntoIterator, Iterator};

use futures::{Async, Future, Poll};

use super::super::async_server::FiniteService;
use super::ioc_test_variable_action::IocTestVariableAction;

pub struct IocVariableActionExecutor<I, S>
where
    I: IntoIterator<Item = IocTestVariableAction>,
    S: FiniteService,
    IocTestVariableAction: Into<S::Request>,
{
    variable_actions: I::IntoIter,
    ioc_variable_service: S,
    active_request: Option<S::Future>,
}

impl<I, S> IocVariableActionExecutor<I, S>
where
    I: IntoIterator<Item = IocTestVariableAction>,
    S: FiniteService,
    IocTestVariableAction: Into<S::Request>,
{
    pub fn new(variable_actions: I, ioc_variable_service: S) -> Self {
        Self {
            ioc_variable_service,
            variable_actions: variable_actions.into_iter(),
            active_request: None,
        }
    }

    pub fn force_stop(&mut self) -> Result<(), S::Error> {
        self.ioc_variable_service.force_stop()
    }

    fn send_next_request(&mut self) {
        if let Some(variable_action) = self.variable_actions.next() {
            let request = variable_action.into();
            let active_request = self.ioc_variable_service.call(request);

            self.active_request = Some(active_request);
        } else {
            self.active_request = None;
        }
    }

    fn poll_active_request(&mut self) -> Poll<(), S::Error> {
        if let Some(ref mut active_request) = self.active_request {
            try_ready!(active_request.poll());
        }

        Ok(Async::Ready(()))
    }
}

impl<I, S> Future for IocVariableActionExecutor<I, S>
where
    I: IntoIterator<Item = IocTestVariableAction>,
    S: FiniteService,
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

        self.force_stop()?;

        Ok(Async::Ready(()))
    }
}
