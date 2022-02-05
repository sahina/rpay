use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub key: String,
    pub value: String,
}

impl Display for MetaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", &self.key, &self.value)
    }
}

impl MetaData {
    pub fn new(key: &str, value: &str) -> Self {
        MetaData {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}
