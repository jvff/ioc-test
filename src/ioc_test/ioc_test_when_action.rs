use std::collections::HashMap;
use std::hash::Hash;

use super::super::instrumenting_service::{WhenAction, WhenVerifier};

pub struct IocTestWhenAction<'a, A, B>
where
    A: 'a,
    B: 'a,
{
    request: Option<A>,
    request_map: &'a mut HashMap<A, B>,
    verifiers: &'a mut Vec<WhenVerifier<A, B>>,
}

impl<'a, A, B> IocTestWhenAction<'a, A, B> {
    pub fn new(
        request_map: &'a mut HashMap<A, B>,
        verifiers: &'a mut Vec<WhenVerifier<A, B>>,
    ) -> Self {
        Self {
            request_map,
            verifiers,
            request: None,
        }
    }
}

impl<'a, A, B> WhenAction for IocTestWhenAction<'a, A, B>
where
    A: Clone + Eq + Hash,
    B: Clone,
{
    type Request = A;
    type Response = B;

    fn when(&mut self, request: &Self::Request) {
        self.request = Some(request.clone());
    }

    fn reply_with(&mut self, response: &Self::Response) {
        if let Some(ref request) = self.request {
            self.request_map.insert(request.clone(), response.clone());
        }
    }

    fn verify(
        &mut self,
        verifier: WhenVerifier<Self::Request, Self::Response>,
    ) {
        self.verifiers.push(verifier);
    }
}
