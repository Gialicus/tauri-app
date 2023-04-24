use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note<'a> {
    pub title: &'a str,
    pub text: &'a str,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
