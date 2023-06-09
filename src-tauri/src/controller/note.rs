use chrono::Utc;
use tauri::State;

use crate::{
    model::{
        note::{Note, UpdateNote},
        record::NoteRecord,
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
    let rec: NoteRecord = db
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
    let rec: Vec<NoteRecord> = db.select("note").await.unwrap();
    let res = serde_json::to_string(&rec).unwrap();
    Ok(format!("{}", res))
}

#[tauri::command]
pub async fn get_note(id: &str, store: State<'_, SurrealStore>) -> Result<String, ()> {
    let db = store.db.lock().await;
    let db_id = utils::id::to_surreal_id(id);
    let rec: Option<NoteRecord> = db.select(("note", db_id)).await.unwrap();
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
    id: Option<&str>,
    title: Option<&str>,
    text: Option<&str>,
    store: State<'_, SurrealStore>,
) -> Result<String, &'static str> {
    let db = store.db.lock().await;
    match id {
        Some(id) => {
            let rec: NoteRecord = db
                .update(("note", utils::id::to_surreal_id(id)))
                .merge(UpdateNote {
                    title,
                    text,
                    updated_at: Utc::now(),
                })
                .await
                .unwrap();
            let res = serde_json::to_string(&rec).unwrap();
            Ok(format!("{}", res))
        }
        None => Err("Mandatory ID"),
    }
}

#[tauri::command]
pub async fn delete_note(id: &str, store: State<'_, SurrealStore>) -> Result<String, ()> {
    let db = store.db.lock().await;
    let rec: NoteRecord = db
        .delete(("note", utils::id::to_surreal_id(id)))
        .await
        .unwrap();
    let res = serde_json::to_string(&rec).unwrap();
    Ok(format!("{}", res))
}
