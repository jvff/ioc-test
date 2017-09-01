use scpi::str_extensions::StrExtensions;
use super::super::super::super::super::super::ScpiDisplaySubsystem;
use super::super::super::super::super::{ScpiDisplayWindow, WindowCommand};
use super::super::super::super::{ScpiDisplayTrace, TraceCommand};

pub fn decode(message: &str) -> Option<TraceCommand> {
    let command = message.skip_expected_chars("OFFSet");

    if command.starts_with("?") {
        return Some(TraceCommand::GetYScaleReferenceLevelOffset);
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
    pub fn get(self) -> ScpiDisplaySubsystem {
        let trace = ScpiDisplayTrace {
            trace: self.trace,
            command: TraceCommand::GetYScaleReferenceLevelOffset,
        };

        let window = ScpiDisplayWindow {
            window: self.window,
            command: WindowCommand::Trace(trace),
        };

        ScpiDisplaySubsystem::Window(window)
    }
}
