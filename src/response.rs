#[derive(Debug, PartialEq, Clone)]
pub enum Response {
    Value {
        key: String,
        value: String,
    },
    Set {
        key: String,
        value: String,
        previous: Option<String>,
    },
    Error {
        msg: String,
    },
}

impl Response {
    pub fn serialize(&self) -> String {
        match *self {
            Response::Value { ref key, ref value } => format!("k: {}, v: {}", key, value),
            Response::Set {
                ref key,
                ref value,
                ref previous,
            } => match previous {
                Some(previous) => format!("k: {}, v: {}, previous: {}", key, value, previous),
                None => format!("k: {}, v: {}", key, value),
            },
            Response::Error { ref msg } => format!("error: {}", msg),
        }
    }
}
