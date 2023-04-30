use serde::{Deserialize, Serialize};
use surrealdb::sql::{Strand, Thing};

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteRecord {
    pub id: Thing,
    pub title: Strand,
    pub text: Strand,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkRecord {
    pub id: Thing,
    pub source: Strand,
    pub target: Strand,
}
