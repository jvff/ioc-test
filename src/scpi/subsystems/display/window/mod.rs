mod trace;

use std::fmt;
use std::fmt::{Display, Formatter};

use scpi::str_extensions::StrExtensions;
use self::trace::ScpiDisplayTrace;
use super::ScpiDisplaySubsystem;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScpiDisplayWindow {
    window: usize,
    command: WindowCommand,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum WindowCommand {
    Trace(ScpiDisplayTrace),
}

impl Display for ScpiDisplayWindow {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let window = self.window;

        match self.command {
            WindowCommand::Trace(ref trace) => {
                write!(formatter, "WIND{}:{}", window, trace)
            }
        }
    }
}

pub fn decode(message: &str) -> Option<ScpiDisplaySubsystem> {
    let request_data = message.skip_expected_chars("WINDow");

    if let Some((window, command)) = request_data.parse_integer() {
        if command.starts_with(":") {
            let command = command.skip_chars(1);

            let decoded_command = match command.view_first_chars(4) {
                "TRAC" => trace::decode(command),
                _ => None,
            };

            if let Some(command) = decoded_command {
                return Some(
                    ScpiDisplaySubsystem::Window(
                        ScpiDisplayWindow { window, command },
                    ),
                );
            }
        }
    }

    None
}

#[derive(Clone, Copy)]
pub struct Builder {
    window: usize,
}

pub fn builder(window: usize) -> Builder {
    Builder { window }
}

impl Builder {
    pub fn trace(self, trace: usize) -> trace::Builder {
        trace::builder(self.window, trace)
    }
}
