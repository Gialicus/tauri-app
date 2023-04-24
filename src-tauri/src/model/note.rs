use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note<'a> {
    pub title: &'a str,
    pub text: &'a str,
}
