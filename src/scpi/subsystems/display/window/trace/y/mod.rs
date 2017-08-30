mod scale;

use scpi::str_extensions::StrExtensions;
use super::TraceCommand;

pub fn decode(message: &str) -> Option<TraceCommand> {
    let command = message.skip_expected_chars("Y");

    if command.starts_with(":") {
        let command = command.skip_chars(1);

        return match command.view_first_chars(4) {
            "RLEV" => scale::reference_level::decode(command),
            "SCAL" => scale::decode(command),
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
    pub fn scale(self) -> scale::Builder {
        scale::builder(self.window, self.trace)
    }
}
