mod window;

use std::fmt;
use std::fmt::{Display, Formatter};

use scpi::str_extensions::StrExtensions;
use self::window::ScpiDisplayWindow;
use super::super::request::ScpiRequest;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ScpiDisplaySubsystem {
    Window(ScpiDisplayWindow),
}

impl From<ScpiDisplayWindow> for ScpiDisplaySubsystem {
    fn from(window: ScpiDisplayWindow) -> Self {
        ScpiDisplaySubsystem::Window(window)
    }
}

impl Display for ScpiDisplaySubsystem {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            ScpiDisplaySubsystem::Window(ref window) => {
                write!(formatter, "DISP:{}", window)
            }
        }
    }
}

impl ScpiRequest for ScpiDisplaySubsystem {
    fn decode(message: &str) -> Option<Self> {
        if message.view_first_chars(4) == "DISP" {
            let command = message.skip_expected_chars("DISPlay");

            if command.starts_with(":") {
                let command = command.skip_chars(1);

                return match command.view_first_chars(4) {
                    "WIND" => window::decode(command),
                    _ => None,
                };
            }
        }

        None
    }
}

pub type Subsystem = ScpiDisplaySubsystem;

pub struct Builder;

pub fn builder() -> Builder {
    Builder
}

impl Builder {
    pub fn window(self, window: usize) -> window::Builder {
        window::builder(window)
    }
}
