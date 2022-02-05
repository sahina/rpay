use chrono;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Confirmation {
    pub id: Uuid,
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}

// any type that implements `Into<String>` can be used to create a `Confirmation`
impl<T> From<T> for Confirmation
    where T: Into<String> {
    fn from(email: T) -> Self {
        Confirmation {
            id: Uuid::new_v4(),
            email: email.into(),
            expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
        }
    }
}
