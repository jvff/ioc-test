use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use super::super::instrumenting_service::{WhenAction, WhenVerifier};

pub struct IocTestWhenAction<A, B> {
    request: Option<A>,
    request_map: Arc<Mutex<HashMap<A, B>>>,
    requests_to_verify: Arc<Mutex<HashSet<A>>>,
    verifiers: Arc<Mutex<Vec<WhenVerifier<A, B>>>>,
}

impl<A, B> IocTestWhenAction<A, B> {
    pub fn new(
        request_map: Arc<Mutex<HashMap<A, B>>>,
        requests_to_verify: Arc<Mutex<HashSet<A>>>,
        verifiers: Arc<Mutex<Vec<WhenVerifier<A, B>>>>,
    ) -> Self {
        Self {
            request_map,
            requests_to_verify,
            verifiers,
            request: None,
        }
    }
}

impl<A, B> WhenAction for IocTestWhenAction<A, B>
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
            let mut request_map = self.request_map.lock().expect(
                "another thread panicked while holding a lock to the mock \
                 request map",
            );

            request_map.insert(request.clone(), response.clone());
        }
    }

    fn verify(
        &mut self,
        verifier: WhenVerifier<Self::Request, Self::Response>,
    ) {
        if let Some(ref request) = self.request {
            let mut requests_to_verify =
                self.requests_to_verify.lock().expect(
                    "another thread panicked while holding a lock to the mock \
                     request verification set",
                );

            requests_to_verify.insert(request.clone());

            let mut verifiers = self.verifiers.lock().expect(
                "another thread panicked while holding a lock to the list of \
                 verifiers",
            );

            verifiers.push(verifier);
        }
    }
}
