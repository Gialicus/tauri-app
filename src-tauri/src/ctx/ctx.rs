use std::sync::Arc;

use tauri::async_runtime::Mutex;

use crate::store::SurrealStore;

pub struct Ctx {
    pub store: Arc<Mutex<SurrealStore>>,
}

impl Ctx {
    pub async fn new() -> Self {
        let store = SurrealStore::new("../temp").await;
        let store = Mutex::new(store.unwrap());
        Ctx {
            store: Arc::new(store),
        }
    }
}
