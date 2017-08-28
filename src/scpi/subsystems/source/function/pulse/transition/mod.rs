mod leading;
mod trailing;

use super::super::super::SourceCommand;
use super::super::super::super::super::str_extensions::StrExtensions;

pub fn decode(string: &str) -> Option<SourceCommand> {
    let command = string.skip_expected_chars("TRANsition");

    if command.starts_with(":") {
        let command = command.skip_chars(1);

        match command.view_first_chars(3) {
            "TRA" => return trailing::decode(command),
            _ => {}
        }

        match command.view_first_chars(4) {
            "LEAD" => return leading::decode(command),
            _ => {}
        }
    }

    None
}
