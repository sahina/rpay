use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub street2: String,
    pub city: String,
    pub region: String,
    pub zipcode: String,
    pub country: String,
}

impl Default for Address {
    fn default() -> Self {
        Address {
            street: String::default(),
            street2: String::default(),
            city: String::default(),
            region: String::default(),
            zipcode: String::default(),
            country: String::default(),
        }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let address = format!(
            "{} {} {} {} {} {}",
            &self.street, &self.street2, &self.city, &self.region, &self.zipcode, &self.country
        ).trim().to_string();

        write!(f, "{}", address)
    }
}

impl Address {
    pub fn new(street: &str) -> Self {
        Address {
            street: street.to_string(),
            street2: String::default(),
            city: String::default(),
            region: String::default(),
            zipcode: String::default(),
            country: String::default(),
        }
    }
}