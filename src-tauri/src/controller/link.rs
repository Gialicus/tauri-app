use chrono::Utc;

use tauri::State;

use crate::{
    model::{
        link::Link,
        record::{LinkRecord, NoteRecord},
    },
    store::SurrealStore,
};

#[tauri::command]
pub async fn add_link(
    source: &str,
    target: &str,
    store: State<'_, SurrealStore>,
) -> Result<String, String> {
    let db = store.db.lock().await;
    let rec: LinkRecord = db
        .create("link")
        .content(Link {
            source,
            target,
            name: "Ciao",
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
        .await
        .unwrap();
    let rec = serde_json::to_string(&rec).unwrap();
    Ok(format!("{}", rec))
}

#[tauri::command]
pub async fn get_link(id: &str, store: State<'_, SurrealStore>) -> Result<String, String> {
    let db = store.db.lock().await;
    println!("{}", id);
    let mut result = db
        .query("SELECT * FROM link WHERE source = $id FETCH source;")
        .bind(("id", id))
        .await
        .unwrap();
    let result: Vec<LinkRecord> = result.take(0).unwrap();
    let mut sources = Vec::new();
    for item in result.iter() {
        println!("{}", item.source);
        let source: Option<NoteRecord> = db
            .query(format!("SELECT * FROM {}", item.source))
            .await
            .unwrap()
            .take(0)
            .unwrap();
        dbg!(&source);
        match source {
            Some(v) => sources.push(v),
            None => (),
        };
    }
    let response = serde_json::to_string(&sources).unwrap();
    Ok(format!("{}", response))
}
