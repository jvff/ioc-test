mod transition;
mod width;

use super::super::super::ScpiRequest;
use super::super::super::super::extension::ScpiExtension;
use super::super::super::str_extensions::StrExtensions;

pub fn decode<X>(string: &str, source: usize) -> Option<ScpiRequest<X>>
where
    X: ScpiExtension,
{
    let command = string.skip_expected_chars("PULSe");

    if command.starts_with(":") {
        let command = command.skip_chars(1);

        match command.view_first_chars(4) {
            "TRAN" => return transition::decode(command, source),
            "WIDT" => return width::decode(command, source),
            _ => {}
        }
    }

    None
}
