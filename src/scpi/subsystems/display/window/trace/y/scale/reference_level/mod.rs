use scpi::str_extensions::StrExtensions;
use super::super::super::super::super::ScpiDisplaySubsystem;
use super::super::super::super::{ScpiDisplayWindow, WindowCommand};
use super::super::super::{ScpiDisplayTrace, TraceCommand};

pub fn decode(message: &str) -> Option<TraceCommand> {
    let command = message.skip_expected_chars("RLEVel");

    if command.starts_with("?") {
        return Some(TraceCommand::GetYScaleReferenceLevel);
    } else if command.starts_with(" ") {
        if let Ok(level) = command.trim().parse::<f64>() {
            return Some(TraceCommand::SetYScaleReferenceLevel(level.into()));
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
    pub fn get(self) -> ScpiDisplaySubsystem {
        let trace = ScpiDisplayTrace {
            trace: self.trace,
            command: TraceCommand::GetYScaleReferenceLevel,
        };

        let window = ScpiDisplayWindow {
            window: self.window,
            command: WindowCommand::Trace(trace),
        };

        ScpiDisplaySubsystem::Window(window)
    }

    pub fn set(self, value: f64) -> ScpiDisplaySubsystem {
        let trace = ScpiDisplayTrace {
            trace: self.trace,
            command: TraceCommand::SetYScaleReferenceLevel(value.into()),
        };

        let window = ScpiDisplayWindow {
            window: self.window,
            command: WindowCommand::Trace(trace),
        };

        ScpiDisplaySubsystem::Window(window)
    }
}
