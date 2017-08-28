use std::fmt;
use std::fmt::{Display, Formatter};

use super::super::request::ScpiRequest;
use super::super::str_extensions::StrExtensions;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScpiOutputSubsystem {
    channel: usize,
    command: OutputCommand,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum OutputCommand {
    On,
    Off,
    Status,
}

impl ScpiOutputSubsystem {
    pub fn new(channel: usize, command: OutputCommand) -> Self {
        Self { channel, command }
    }

    pub fn status(channel: usize) -> Self {
        Self::new(channel, OutputCommand::Status)
    }

    pub fn on(channel: usize) -> Self {
        Self::new(channel, OutputCommand::On)
    }

    pub fn off(channel: usize) -> Self {
        Self::new(channel, OutputCommand::Off)
    }
}

impl Display for ScpiOutputSubsystem {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let channel = self.channel;

        match self.command {
            OutputCommand::On => write!(formatter, "OUTP{} ON", channel),
            OutputCommand::Off => write!(formatter, "OUTP{} OFF", channel),
            OutputCommand::Status => write!(formatter, "OUTP{}?", channel),
        }
    }
}

impl ScpiRequest for ScpiOutputSubsystem {
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
            return Some(ScpiOutputSubsystem::status(channel));
        } else if command.chars().next() == Some(' ') {
            match command.trim() {
                "ON" => return Some(ScpiOutputSubsystem::on(channel)),
                "OFF" => return Some(ScpiOutputSubsystem::off(channel)),
                _ => {}
            }
        }
    }

    None
}

pub type Subsystem = ScpiOutputSubsystem;
