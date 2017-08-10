mod bitrate;
mod polynomial;
mod transition;

use super::super::super::ScpiRequest;
use super::super::super::str_extensions::StrExtensions;

pub fn decode(string: &str, source: usize) -> Option<ScpiRequest> {
    let command = string.skip_expected_chars("PRBS");

    if command.starts_with(":") {
        let command = command.skip_chars(1);

        match command.view_first_chars(4) {
            "BRAT" => return bitrate::decode(command, source),
            "DATA" => return polynomial::decode(command, source),
            "TRAN" => return transition::decode(command, source),
            _ => {}
        }
    }

    None
}
