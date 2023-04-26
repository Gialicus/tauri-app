use chrono::Utc;
use tauri::State;

use crate::{
    model::{link::Link, record::Record},
    store::SurrealStore,
};

#[tauri::command]
pub async fn add_link(
    source: &str,
    target: &str,
    name: &str,
    store: State<'_, SurrealStore>,
) -> Result<String, ()> {
    let db = store.db.lock().await;
    let rec: Record = db
        .create("note")
        .content(Link {
            source,
            target,
            name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
        .await
        .unwrap();
    let res = serde_json::to_string(&rec).unwrap();
    Ok(format!("{}", res))
}
