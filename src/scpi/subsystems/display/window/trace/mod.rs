mod y;

use std::fmt;
use std::fmt::{Display, Formatter};

use scpi::str_extensions::StrExtensions;
use super::WindowCommand;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScpiDisplayTrace {
    trace: usize,
    command: TraceCommand,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TraceCommand {
    GetYScaleReferenceLevel,
    SetYScaleReferenceLevel(isize),
}

impl Display for ScpiDisplayTrace {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let trace = self.trace;

        match self.command {
            TraceCommand::GetYScaleReferenceLevel => {
                write!(formatter, "TRAC{}:Y:SCAL:RLEV?", trace)
            }
            TraceCommand::SetYScaleReferenceLevel(value) => {
                write!(formatter, "TRAC{}:Y:SCAL:RLEV {}", trace, value)
            }
        }
    }
}

pub fn decode(message: &str) -> Option<WindowCommand> {
    let request_data = message.skip_expected_chars("TRACe");

    if let Some((trace, command)) = request_data.parse_integer() {
        if command.starts_with(":") {
            let command = command.skip_chars(1);

            let decoded_command = match command.view_first_chars(1) {
                "Y" => y::decode(command),
                _ => None,
            };

            if let Some(command) = decoded_command {
                return Some(
                    WindowCommand::Trace(ScpiDisplayTrace { trace, command }),
                );
            }
        }
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
    pub fn y(self) -> y::Builder {
        y::builder(self.window, self.trace)
    }
}