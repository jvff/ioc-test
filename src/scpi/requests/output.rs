use std::fmt;
use std::fmt::{Display, Formatter};

use super::super::extension::ScpiExtension;
use super::str_extensions::StrExtensions;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ScpiOutputSubsystem {
    OutputOn(usize),
    OutputOff(usize),
    OutputStatus(usize),
}

impl Display for ScpiOutputSubsystem {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            ScpiOutputSubsystem::OutputOn(channel) => {
                write!(formatter, "OUTP{} ON", channel)
            }
            ScpiOutputSubsystem::OutputOff(channel) => {
                write!(formatter, "OUTP{} OFF", channel)
            }
            ScpiOutputSubsystem::OutputStatus(channel) => {
                write!(formatter, "OUTP{}?", channel)
            }
        }
    }
}

impl ScpiExtension for ScpiOutputSubsystem {
    fn decode(message: &str) -> Option<Self> {
        if message.view_first_chars(4) == "OUTP" {
            return decode_output_message(message);
        }

        None
    }
}

pub fn decode_output_message(message: &str) -> Option<ScpiOutputSubsystem> {
    let request_data = message.skip_expected_chars("OUTPut");

    if let Some((channel, command)) = request_data.parse_integer() {
        if command == "?" {
            return Some(ScpiOutputSubsystem::OutputStatus(channel));
        } else if command.chars().next() == Some(' ') {
            match command.trim() {
                "ON" => return Some(ScpiOutputSubsystem::OutputOn(channel)),
                "OFF" => return Some(ScpiOutputSubsystem::OutputOff(channel)),
                _ => {}
            }
        }
    }

    None
}
