use bytes::BytesMut;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScpiResponse {
    Empty,
    Integer(isize),
    Utf8String(String),
}

impl ScpiResponse {
    pub fn from(string: &str) -> Self {
        if string.len() == 0 {
            ScpiResponse::Empty
        } else if let Ok(integer) = string.parse() {
            ScpiResponse::Integer(integer)
        } else {
            ScpiResponse::Utf8String(String::from(string))
        }
    }

    pub fn encode(&self, buffer: &mut BytesMut) {
        match *self {
            ScpiResponse::Empty => {}
            ScpiResponse::Integer(value) => {
                buffer.extend(value.to_string().as_bytes());
                buffer.extend("\n".as_bytes())
            }
            ScpiResponse::Utf8String(ref string) => {
                buffer.extend(string.as_bytes());
                buffer.extend("\n".as_bytes())
            }
        }
    }
}
