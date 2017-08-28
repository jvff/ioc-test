mod sample_rate;

use super::super::SourceCommand;
use super::super::super::super::str_extensions::StrExtensions;

pub fn decode(string: &str) -> Option<SourceCommand> {
    let command = string.skip_expected_chars("ARBitrary");

    if command.starts_with("?") {
        return Some(SourceCommand::ArbitraryFunctionFileQuery);
    } else if command.starts_with(":") {
        let command = command.skip_chars(1);

        match command.view_first_chars(4) {
            "SRAT" => return sample_rate::decode(command),
            _ => {}
        }
    }

    None
}
