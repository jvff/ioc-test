use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use super::errors::{Error, ErrorKind, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum EpicsDataType {
    DbrDouble(f64),
    DbrString(String),
}

impl EpicsDataType {
    pub fn dbr_double_from<S>(string: S) -> Result<EpicsDataType>
    where
        S: AsRef<str>,
    {
        Ok(EpicsDataType::DbrDouble(string.as_ref().parse()?))
    }

    pub fn dbr_string_from<S>(string: S) -> EpicsDataType
    where
        S: AsRef<str>,
    {
        let mut string = string.as_ref().trim().to_string();

        if string.starts_with("\"") && string.ends_with("\"") {
            let length = string.len();

            string = string[1..length - 1]
                .replace("\\n", "\n")
                .replace("\\r", "\r")
                .replace("\\t", "\t")
                .replace("\\\\", "\\");
        }

        EpicsDataType::DbrString(string)
    }

    pub fn type_name(&self) -> &'static str {
        match *self {
            EpicsDataType::DbrDouble(_) => "DBR_DOUBLE",
            EpicsDataType::DbrString(_) => "DBR_STRING",
        }
    }
}

impl FromStr for EpicsDataType {
    type Err = Error;

    fn from_str(string: &str) -> Result<EpicsDataType> {
        let string = string.trim();

        if string.starts_with("DBR_") {
            let mut parts = string.split_whitespace();

            if let Some(mut data_type) = parts.next() {
                if data_type.ends_with(":") {
                    data_type = &data_type[0..data_type.len() - 1];
                }

                let value =
                    parts
                        .next()
                        .ok_or::<Error>(
                            ErrorKind::MissingEpicsDataParameter.into(),
                        )?;

                match data_type {
                    "DBR_DOUBLE" => EpicsDataType::dbr_double_from(value),
                    "DBR_STRING" => Ok(EpicsDataType::dbr_string_from(value)),
                    data_type => {
                        Err(
                            ErrorKind::UnknownEpicsDataType(
                                data_type.to_string(),
                            ).into(),
                        )
                    }
                }
            } else {
                unreachable!("string was verified to not be empty");
            }

        } else {
            Err(ErrorKind::InvalidEpicsDataString(string.to_string()).into())
        }
    }
}

impl<'a> From<&'a str> for EpicsDataType {
    fn from(string: &'a str) -> EpicsDataType {
        EpicsDataType::DbrString(string.to_string())
    }
}

impl From<f64> for EpicsDataType {
    fn from(double: f64) -> EpicsDataType {
        EpicsDataType::DbrDouble(double.into())
    }
}

impl Display for EpicsDataType {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            EpicsDataType::DbrString(ref value) => {
                write!(formatter, "{:?}", value)
            }
            EpicsDataType::DbrDouble(ref value) => value.fmt(formatter),
        }
    }
}
