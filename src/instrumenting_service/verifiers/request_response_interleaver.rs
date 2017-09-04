use std::collections::VecDeque;

use super::verifier::Verifier;
use super::verifier_factory::VerifierFactory;

pub struct RequestResponseInterleaver<V>
where
    V: Verifier,
    V::Request: Clone,
    V::Response: Clone,
{
    verifier: V,
    status: Status,
    request_queue: VecDeque<V::Request>,
    response_queue: VecDeque<V::Response>,
}

#[derive(PartialEq)]
pub enum Status {
    ExpectingRequest,
    ExpectingResponse,
}

impl<V> RequestResponseInterleaver<V>
where
    V: Verifier,
    V::Request: Clone,
    V::Response: Clone,
{
    pub fn new(verifier: V) -> Self {
        Self {
            verifier,
            status: Status::ExpectingRequest,
            request_queue: VecDeque::new(),
            response_queue: VecDeque::new(),
        }
    }
}

impl<V> Verifier for RequestResponseInterleaver<V>
where
    V: Verifier,
    V::Request: Clone,
    V::Response: Clone,
{
    type Request = V::Request;
    type Response = V::Response;
    type Error = V::Error;

    fn request(&mut self, request: &Self::Request) {
        match self.status {
            Status::ExpectingRequest => {
                self.verifier.request(request);

                match self.response_queue.pop_front() {
                    Some(response) => self.verifier.response(&response),
                    None => self.status = Status::ExpectingResponse,
                };
            }
            Status::ExpectingResponse => {
                self.request_queue.push_back(request.clone())
            }
        }
    }

    fn response(&mut self, response: &Self::Response) {
        match self.status {
            Status::ExpectingRequest => {
                self.response_queue.push_back(response.clone())
            }
            Status::ExpectingResponse => {
                self.verifier.response(response);

                match self.response_queue.pop_front() {
                    Some(response) => self.verifier.response(&response),
                    None => self.status = Status::ExpectingResponse,
                };
            }
        }
    }

    fn has_finished(&self) -> Result<bool, Self::Error> {
        self.verifier.has_finished()
    }

    fn force_stop(&mut self) -> Result<(), Self::Error> {
        self.verifier.force_stop()
    }
}

impl<V> VerifierFactory for RequestResponseInterleaver<V>
where
    V: Verifier + VerifierFactory,
    V::Request: Clone,
    V::Response: Clone,
    <V::Verifier as Verifier>::Request: Clone,
    <V::Verifier as Verifier>::Response: Clone,
{
    type Verifier = RequestResponseInterleaver<V::Verifier>;

    fn create(&self) -> Self::Verifier {
        RequestResponseInterleaver::new(self.verifier.create())
    }
}
