error_chain! {
    errors {
        RequestWasntVerified(request: String) {
            description("an expected request was not verified")
            display("an expected request was not verified: {}", request)
        }

        RequestAndResponseWerentVerified(request: String, response: String) {
            description(
                "an expected request and its response were not verified"
            )
            display(
                "an expected request ({}) and its response ({}) were not \
                 verified",
                request,
                response,
            )
        }

        RequestVerifiedButNotResponse(request: String, response: String) {
            description(
                "an expected request was verified, but its exected response \
                 was not verified"
            )
            display(
                "an expected request ({}) was verified, but its expected \
                 response ({}) was not verified",
                request,
                response,
            )
        }
    }
}
