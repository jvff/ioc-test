use super::verifiers::Error;
use super::when_action::WhenAction;
use super::when_verifier::WhenVerifier;

pub struct When<A, B, W>
where
    A: Eq,
    B: Eq,
    W: WhenAction<Request = A, Response = B>,
    W::Error: From<Error>,
{
    request: A,
    response: Option<B>,
    action: Option<W>,
}

impl<A, B, W> When<A, B, W>
where
    A: Eq,
    B: Eq,
    W: WhenAction<Request = A, Response = B>,
    W::Error: From<Error>,
{
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

    pub fn verify(self) {
        if let Some(mut action) = self.action {
            let verifier = if let Some(response) = self.response {
                WhenVerifier::for_request_response(self.request, response)
            } else {
                WhenVerifier::for_request(self.request)
            };

            action.verify(verifier);
        }
    }
}
