use rusty_money::{iso, Money};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub currency_code: String,
    pub amount: i64,
}

impl Default for Currency {
    fn default() -> Self {
        Currency {
            currency_code: "USD".to_string(),
            amount: 0,
        }
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let country_code = iso::find(self.currency_code.as_str()).unwrap();
        let money = Money::from_minor(self.amount, country_code);

        write!(f, "{}", money.to_string())
    }
}

impl Currency {
    pub fn new(code: &str, amount: i64) -> Self {
        Self {
            currency_code: code.to_string(),
            amount,
        }
    }
}
