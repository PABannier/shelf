#[derive(Debug, PartialEq, Clone)]
pub enum Request {
    Get { key: String },
    Put { key: String, value: String },
    Delete { key: String },
}

impl Request {
    pub fn parse(input: &str) -> Result<Request, String> {
        let mut parts = input.splitn(3, ' ');
        match parts.next() {
            Some("GET") => {
                let key = parts.next().ok_or("GET must be followed by a key")?;
                if parts.next().is_some() {
                    return Err("Additional argument can't be parsed.".into());
                }
                Ok(Request::Get {
                    key: key.to_string(),
                })
            }
            Some("PUT") => {
                let key = match parts.next() {
                    Some(key) => key,
                    None => return Err("PUT must be followed by a key".into()),
                };
                let value = match parts.next() {
                    Some(value) => value,
                    None => return Err("A value must be provided with put".into()),
                };
                Ok(Request::Put {
                    key: key.to_string(),
                    value: value.to_string(),
                })
            }
            Some("DELETE") => {
                let key = parts.next().ok_or("DELETE must be followed by a key")?;
                if parts.next().is_some() {
                    return Err("Additional argument can't be parsed.".into());
                }
                Ok(Request::Delete {
                    key: key.to_string(),
                })
            }
            Some(cmd) => return Err(format!("Unknown command: {}", cmd)),
            None => Err("empty input".into()),
        }
    }
}
