use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub struct Phone {
    pub country_code: String,
    pub phone_number: String,
}

impl Default for Phone {
    fn default() -> Self {
        Phone {
            country_code: String::default(),
            phone_number: String::default(),
        }
    }
}

impl Phone {
    pub fn new(country_code: &str, phone_number: &str) -> Self {
        Phone {
            country_code: country_code.to_string(),
            phone_number: phone_number.to_string(),
        }
    }
}

impl Display for Phone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "+{} {}", &self.country_code, &self.phone_number)
    }
}
