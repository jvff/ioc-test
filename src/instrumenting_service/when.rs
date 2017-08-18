use super::verifiers::{Verifier, VerifyRequest, VerifyRequestResponse};
use super::when_action::WhenAction;

pub struct When<A, B, W>
where
    A: Eq,
    B: Eq,
    W: WhenAction<Request = A, Response = B>,
    W::Error: From<()>,
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
    W::Error: From<()> + 'static,
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
                VerifyRequestResponse::new(self.request, response)
                    .convert_error()
                    .boxed()
            } else {
                VerifyRequest::new(self.request).convert_error().boxed()
            };

            action.verify(verifier);
        }
    }
}
