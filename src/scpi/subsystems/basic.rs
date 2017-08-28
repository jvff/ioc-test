use std::fmt;
use std::fmt::{Display, Formatter};

use super::super::request::ScpiRequest;
use super::super::str_extensions::StrExtensions;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ScpiBasicSubsystem {
    CalibrationQuery,
}

impl Display for ScpiBasicSubsystem {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            ScpiBasicSubsystem::CalibrationQuery => write!(formatter, "*CAL?"),
        }
    }
}

impl ScpiRequest for ScpiBasicSubsystem {
    fn decode(message: &str) -> Option<Self> {
        if message.view_first_chars(4) == "*CAL?" {
            return Some(ScpiBasicSubsystem::CalibrationQuery)
        }

        None
    }
}
