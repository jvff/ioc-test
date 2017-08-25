use std::fmt;
use std::fmt::{Display, Formatter};

use super::extension::ScpiExtension;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NoScpiExtension;

impl ScpiExtension for NoScpiExtension {
    fn decode(_: &str) -> Option<Self> {
        None
    }
}

impl Display for NoScpiExtension {
    fn fmt(&self, _formatter: &mut Formatter) -> fmt::Result {
        panic!("invalid SCPI message");
    }
}
