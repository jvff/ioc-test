use super::ScpiSourceSubsystem;

use super::super::super::str_extensions::StrExtensions;

pub fn decode(string: &str, source: usize) -> Option<ScpiSourceSubsystem> {
    let command = string.skip_expected_chars("PHASe");

    if command.starts_with("?") {
        return Some(ScpiSourceSubsystem::SourcePhaseGet(source));
    }

    None
}
