mod bitrate;
mod polynomial;
mod transition;

use super::super::SourceCommand;
use super::super::super::super::str_extensions::StrExtensions;

pub fn decode(string: &str) -> Option<SourceCommand> {
    let command = string.skip_expected_chars("PRBS");

    if command.starts_with(":") {
        let command = command.skip_chars(1);

        match command.view_first_chars(4) {
            "BRAT" => return bitrate::decode(command),
            "DATA" => return polynomial::decode(command),
            "TRAN" => return transition::decode(command),
            _ => {}
        }
    }

    None
}
