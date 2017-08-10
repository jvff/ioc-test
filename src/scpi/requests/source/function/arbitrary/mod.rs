mod sample_rate;

use super::super::super::ScpiRequest;
use super::super::super::str_extensions::StrExtensions;

pub fn decode(string: &str, source: usize) -> Option<ScpiRequest> {
    let command = string.skip_expected_chars("ARBitrary");

    if command.starts_with(":") {
        let command = command.skip_chars(1);
        let first_four_chars = command.view_first_chars(4);

        match first_four_chars {
            "SRAT" => return sample_rate::decode(command, source),
            _ => {}
        }
    }

    None
}
