use tauri::State;

use crate::{
    model::{note::Note, record::Record},
    store::SurrealStore,
};

#[tauri::command]
pub async fn add_note(
    title: &str,
    text: &str,
    store: State<'_, SurrealStore>,
) -> Result<String, ()> {
    let db = store.db.lock().await;
    let rec: Record = db
        .create("note")
        .content(Note { title, text })
        .await
        .unwrap();
    let res = serde_json::to_string(&rec).unwrap();
    Ok(format!("{}", res))
}

#[tauri::command]
pub async fn get_notes(store: State<'_, SurrealStore>) -> Result<String, ()> {
    let db = store.db.lock().await;
    let rec: Vec<Record> = db.select("note").await.unwrap();
    let res = serde_json::to_string(&rec).unwrap();
    Ok(format!("{}", res))
}

#[tauri::command]
pub async fn get_note(id: &str, store: State<'_, SurrealStore>) -> Result<String, ()> {
    let db = store.db.lock().await;
    let rec: Option<Record> = db.select(("note", id)).await.unwrap();
    let res = serde_json::to_string(&rec).unwrap();
    Ok(format!("{}", res))
}
