mod arbitrary;
mod noise;
mod prbs;
mod pulse;
mod ramp;
mod square;

use super::SourceCommand;
use super::super::super::str_extensions::StrExtensions;

pub fn decode(string: &str) -> Option<SourceCommand> {
    let command = string.skip_expected_chars("FUNCtion");

    if command.starts_with("?") {
        return Some(SourceCommand::FunctionQuery);
    } else if command.starts_with(":") {
        let command = command.skip_chars(1);

        match command.view_first_chars(3) {
            "ARB" => return arbitrary::decode(command),
            "SQU" => return square::decode(command),
            _ => {}
        }

        match command.view_first_chars(4) {
            "NOIS" => return noise::decode(command),
            "PRBS" => return prbs::decode(command),
            "PULS" => return pulse::decode(command),
            "RAMP" => return ramp::decode(command),
            _ => {}
        }
    }

    None
}
