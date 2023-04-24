use serde::{Deserialize, Serialize};
use surrealdb::sql::{Strand, Thing};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: Thing,
    pub title: Strand,
    pub text: Strand,
}
