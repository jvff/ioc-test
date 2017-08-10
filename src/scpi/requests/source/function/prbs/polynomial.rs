use super::super::super::super::ScpiRequest;
use super::super::super::super::str_extensions::StrExtensions;

pub fn decode(string: &str, source: usize) -> Option<ScpiRequest> {
    let command = string.skip_expected_chars("DATA");

    if command.starts_with("?") {
        return Some(ScpiRequest::SourcePrbsFunctionPolynomialGet(source));
    }

    None
}
