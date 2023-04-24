use surrealdb::{
    engine::local::{Db, RocksDb},
    Surreal,
};
#[derive(Debug)]
pub struct SurrealStore {
    pub db: Surreal<Db>,
}

impl SurrealStore {
    pub async fn new(file_path: &str) -> Result<Self, surrealdb::Error> {
        let db = Surreal::new::<RocksDb>(file_path).await?;
        db.use_ns("test").use_db("test").await?;
        Ok(SurrealStore { db })
    }
}
