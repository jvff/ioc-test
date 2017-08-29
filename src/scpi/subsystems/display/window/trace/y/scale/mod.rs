mod reference_level;

use scpi::str_extensions::StrExtensions;
use super::super::TraceCommand;

pub fn decode(message: &str) -> Option<TraceCommand> {
    let command = message.skip_expected_chars("SCALe");

    if command.starts_with(":") {
        let command = command.skip_chars(1);

        return match command.view_first_chars(4) {
            "RLEV" => reference_level::decode(command),
            _ => None,
        };
    }

    None
}

#[derive(Clone, Copy)]
pub struct Builder {
    window: usize,
    trace: usize,
}

pub fn builder(window: usize, trace: usize) -> Builder {
    Builder { window, trace }
}

impl Builder {
    pub fn reference_level(self) -> reference_level::Builder {
        reference_level::builder(self.window, self.trace)
    }
}
