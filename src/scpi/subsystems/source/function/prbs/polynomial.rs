use super::super::super::ScpiSourceSubsystem;

use super::super::super::super::super::str_extensions::StrExtensions;

pub fn decode(string: &str, source: usize) -> Option<ScpiSourceSubsystem> {
    let command = string.skip_expected_chars("DATA");

    if command.starts_with("?") {
        return Some(ScpiSourceSubsystem::SourcePrbsFunctionPolynomialGet(source));
    }

    None
}
