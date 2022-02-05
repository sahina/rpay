use serde::{Deserialize, Serialize};
use crate::rfp::model::contact_info::ContactInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Consumer {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub contact_info: ContactInfo,
}

impl Consumer {
    pub fn new() -> Self {
        Consumer {
            first_name: None,
            middle_name: None,
            last_name: None,
            contact_info: ContactInfo::default(),
        }
    }
}
