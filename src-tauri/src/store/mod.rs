use std::sync::Arc;

use surrealdb::{
    engine::local::{Db, RocksDb},
    Surreal,
};
use tauri::async_runtime::Mutex;
#[derive(Debug)]
pub struct SurrealStore {
    pub db: Arc<Mutex<Surreal<Db>>>,
}

impl SurrealStore {
    pub async fn new(file_path: &str) -> Self {
        let db = Surreal::new::<RocksDb>(file_path)
            .await
            .expect("Connessionel al db fallita");
        db.use_ns("test").use_db("test").await.unwrap();
        let db = Mutex::new(db);
        let db = Arc::new(db);
        SurrealStore { db }
    }
}
