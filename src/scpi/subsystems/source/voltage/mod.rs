mod offset;

use super::ScpiSourceSubsystem;
use super::super::super::str_extensions::StrExtensions;

pub fn decode(string: &str, source: usize) -> Option<ScpiSourceSubsystem> {
    let command = string.skip_expected_chars("VOLTage");

    if command.starts_with("?") {
        return Some(ScpiSourceSubsystem::SourceVoltageGet(source));
    } else if command.starts_with(":") {
        let command = command.skip_chars(1);

        match command.view_first_chars(4) {
            "OFFS" => return offset::decode(command, source),
            _ => {}
        }
    }

    None
}