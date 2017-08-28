use super::super::super::SourceCommand;

use super::super::super::super::super::str_extensions::StrExtensions;

pub fn decode(string: &str) -> Option<SourceCommand> {
    let command = string.skip_expected_chars("SRATe");

    if command.starts_with("?") {
        return Some(SourceCommand::ArbitraryFunctionSampleRateGet);
    }

    None
}
