use super::super::ScpiRequest;
use super::super::super::extension::ScpiExtension;
use super::super::str_extensions::StrExtensions;

pub fn decode<X>(string: &str, source: usize) -> Option<ScpiRequest<X>>
where
    X: ScpiExtension,
{
    let command = string.skip_expected_chars("PHASe");

    if command.starts_with("?") {
        return Some(ScpiRequest::SourcePhaseGet(source));
    }

    None
}
