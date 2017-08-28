mod symmetry;

use super::super::SourceCommand;
use super::super::super::super::str_extensions::StrExtensions;

pub fn decode(string: &str) -> Option<SourceCommand> {
    let command = string.skip_expected_chars("RAMP");

    if command.starts_with(":") {
        let command = command.skip_chars(1);

        match command.view_first_chars(4) {
            "SYMM" => return symmetry::decode(command),
            _ => {}
        }
    }

    None
}
