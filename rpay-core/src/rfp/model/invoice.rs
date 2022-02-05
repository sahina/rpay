use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::rfp::model::currency::Currency;

//
// Invoice

#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {
    pub invoice_id: String,
    pub items: Vec<InvoiceItem>,
}

impl Display for Invoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invoice_id: {}, {} items", &self.invoice_id, &self.items.len())
    }
}

impl Default for Invoice {
    fn default() -> Self {
        Invoice {
            invoice_id: "n/a".to_string(),
            items: Vec::new(),
        }
    }
}

impl Invoice {
    pub fn new(id: &str) -> Self {
        Invoice {
            invoice_id: id.to_string(),
            items: Vec::new(),
        }
    }
}


//
// InvoiceItem

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub description: String,
    pub currency: Currency,
}

impl Display for InvoiceItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", &self.description, &self.currency.to_string())
    }
}

impl Default for InvoiceItem {
    fn default() -> Self {
        InvoiceItem {
            description: "n/a".to_string(),
            currency: Currency::default(),
        }
    }
}

impl InvoiceItem {
    pub fn new(desc: &str, currency: Currency) -> Self {
        InvoiceItem {
            description: desc.to_string(),
            currency,
        }
    }
}
