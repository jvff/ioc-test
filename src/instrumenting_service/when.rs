use super::verifier::Verifier;
use super::verify_request::VerifyRequest;
use super::verify_request_response::VerifyRequestResponse;
use super::when_action::WhenAction;

pub struct When<A, B, W>
where
    A: Eq,
    B: Eq,
    W: WhenAction<Request = A, Response = B>,
{
    request: A,
    response: Option<B>,
    action: Option<W>,
}

impl<'a, A, B, W> When<A, B, W>
where
    A: 'a + Eq,
    B: 'a + Eq,
    W: WhenAction<Request = A, Response = B>,
{
    pub fn new(request: A) -> Self {
        Self {
            request,
            action: None,
            response: None,
        }
    }

    pub fn with_action(request: A, mut action: W) -> Self {
        action.when(&request);

        Self {
            request,
            action: Some(action),
            response: None,
        }
    }

    pub fn reply_with<D>(mut self, response: D) -> Self
    where
        D: Into<B>,
    {
        let response = response.into();

        if let Some(ref mut action) = self.action {
            action.reply_with(&response);
        }

        self.response = Some(response);

        self
    }

    pub fn verify(
        self,
    ) -> Box<Verifier<Request = A, Response = B, Error = ()> + 'a> {
        if let Some(mut action) = self.action {
            action.verify();
        }

        if let Some(response) = self.response {
            Box::new(VerifyRequestResponse::new(self.request, response))
        } else {
            Box::new(VerifyRequest::new(self.request))
        }
    }
}
