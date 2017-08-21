use std::fmt::Display;

pub trait ScpiExtension: Display {
    fn decode(message: &str) -> Option<Self>
    where
        Self: Sized;
}
