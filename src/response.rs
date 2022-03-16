use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Response<K: Serialize + Clone + Display, V: Serialize + Clone + Display> {
    Value { key: K, value: V },
    Error { msg: String },
}
