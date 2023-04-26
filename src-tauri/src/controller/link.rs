use tauri::State;

use crate::{model::record::Record, store::SurrealStore, utils};

#[tauri::command]
pub async fn add_link(
    source: &str,
    target: &str,
    store: State<'_, SurrealStore>,
) -> Result<String, String> {
    let db = store.db.lock().await;
    let rec = db
        .query(format!(
            "RELATE note:{}->knows->note:{} SET time.written = time::now();",
            utils::id::to_surreal_id(source),
            utils::id::to_surreal_id(target)
        ))
        .await;
    match rec {
        Ok(r) => {
            println!("{}", source);
            println!("{}", target);
            println!("{:?}", r);
            Ok(format!("{}", "OK"))
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[tauri::command]
pub async fn get_link(id: &str, store: State<'_, SurrealStore>) -> Result<String, String> {
    let db = store.db.lock().await;
    println!("ID: {}", id);
    let query = format!("SELECT *,->knows->note FROM note WHERE id = {};", id);
    println!("QUERY: {}", query);
    let rec = db.query(query).await;
    match rec {
        Ok(mut r) => {
            let result: Result<Vec<Record>, surrealdb::Error> = r.take(0);
            match result {
                Ok(v) => {
                    println!("{:?}", v);
                    Ok(format!("{:?}", serde_json::to_string(&v).unwrap()))
                }
                Err(e) => Err(format!("{:?}", e)),
            }
        }
        Err(e) => Err(format!("{:?}", e)),
    }
}
