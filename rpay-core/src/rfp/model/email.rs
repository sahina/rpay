use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    pub description: String,
    pub email_address: String,
}

impl Default for Email {
    fn default() -> Self {
        Email {
            description: String::default(),
            email_address: String::default(),
        }
    }
}

impl Email {
    pub fn new(description: &str, email_address: &str) -> Self {
        Email {
            description: description.to_string(),
            email_address: email_address.to_string(),
        }
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", &self.description, &self.email_address)
    }
}
