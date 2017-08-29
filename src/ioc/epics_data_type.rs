use super::errors::{Error, ErrorKind, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum EpicsDataType {
    DbrString(String),
}

impl EpicsDataType {
    pub fn from<S>(string: S) -> Result<EpicsDataType>
    where
        S: AsRef<str>,
    {
        let string = string.as_ref().trim();

        if string.starts_with("DBR_") {
            let mut parts = string.split_whitespace();

            if let Some(mut data_type) = parts.next() {
                if data_type.ends_with(":") {
                    data_type = &data_type[0..data_type.len() - 1];
                }

                match data_type {
                    "DBR_STRING" => {
                        let value = parts
                            .next()
                            .ok_or::<Error>(
                                ErrorKind::MissingEpicsDataParameter.into(),
                            )?;

                        Ok(EpicsDataType::dbr_string_from(value))
                    }
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
}
