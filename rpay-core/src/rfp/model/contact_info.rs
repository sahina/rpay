use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::rfp::model::address::Address;
use crate::rfp::model::email::Email;
use crate::rfp::model::phone::Phone;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInfo {
    pub emails: Vec<Email>,
    pub phone_numbers: Option<Vec<Phone>>,
    pub sms_numbers: Option<Vec<Phone>>,
    pub addresses: Option<Vec<Address>>,
}

impl Display for ContactInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "Email: {:?}, Sms: {:?}, Phone: {:?}, Address: {:?}",
            &self.emails, &self.sms_numbers, &self.phone_numbers, &self.addresses
        )
    }
}

impl Default for ContactInfo {
    fn default() -> Self {
        ContactInfo {
            emails: vec![],
            sms_numbers: Some(vec![]),
            phone_numbers: Some(vec![]),
            addresses: Some(vec![]),
        }
    }
}

impl ContactInfo {
    pub fn new(email: &str) -> Self {
        ContactInfo {
            emails: vec![Email::new("default", email)],
            phone_numbers: None,
            sms_numbers: None,
            addresses: None,
        }
    }
}