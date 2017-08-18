use super::verifiers::BoxedVerifier;
use super::verifiers::VerifyRequest;
use super::verifiers::VerifyRequestResponse;
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

impl<A, B, W> When<A, B, W>
where
    A: Eq + 'static,
    B: Eq + 'static,
    W: WhenAction<Request = A, Response = B>,
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
                BoxedVerifier::from(
                    VerifyRequestResponse::new(self.request, response),
                )
            } else {
                BoxedVerifier::from(VerifyRequest::new(self.request))
            };

            action.verify(verifier);
        }
    }
}
