// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ctx::ctx::Ctx;
use model::record::Record;
use tauri::{async_runtime::block_on, State};

mod ctx;
mod model;
mod store;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(name: &str, ctx: State<'_, Ctx>) -> Result<String, ()> {
    let ctx = ctx.store.lock().await;
    let people: Vec<Record> = ctx.db.select("note").await.unwrap();
    let people = serde_json::to_string(&people).unwrap();
    println!("{}", name);
    Ok(format!("{}", people))
}

fn main() {
    let context = Ctx::new();
    let context = block_on(context);
    tauri::Builder::default()
        .manage(context)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
