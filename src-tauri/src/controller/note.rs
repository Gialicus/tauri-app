use chrono::Utc;
use tauri::State;

use crate::{
    model::{
        note::{Note, UpdateNote},
        record::Record,
    },
    store::SurrealStore,
    utils,
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
        .content(Note {
            title,
            text,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
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
    let db_id = utils::id::to_surreal_id(id);
    let rec: Option<Record> = db.select(("note", db_id)).await.unwrap();
    match rec {
        Some(v) => {
            let v = serde_json::to_string(&v).unwrap();
            Ok(format!("{}", v))
        }
        None => Err(()),
    }
}

#[tauri::command]
pub async fn update_note(
    id: &str,
    title: &str,
    text: &str,
    store: State<'_, SurrealStore>,
) -> Result<String, ()> {
    let db = store.db.lock().await;
    let rec: Record = db
        .update(("note", utils::id::to_surreal_id(id)))
        .merge(UpdateNote {
            title: Some(title),
            text: Some(text),
            updated_at: Utc::now(),
        })
        .await
        .unwrap();
    let res = serde_json::to_string(&rec).unwrap();
    Ok(format!("{}", res))
}

#[tauri::command]
pub async fn delete_note(id: &str, store: State<'_, SurrealStore>) -> Result<String, ()> {
    let db = store.db.lock().await;
    let rec: Record = db
        .delete(("note", utils::id::to_surreal_id(id)))
        .await
        .unwrap();
    let res = serde_json::to_string(&rec).unwrap();
    Ok(format!("{}", res))
}
