// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use store::SurrealStore;
use tauri::async_runtime::block_on;

mod controller;
mod model;
mod store;
mod utils;

fn main() {
    let context = SurrealStore::new("../temp");
    let context = block_on(context);
    tauri::Builder::default()
        .manage(context)
        .invoke_handler(tauri::generate_handler![
            controller::note::add_note,
            controller::note::get_notes,
            controller::note::get_note,
            controller::note::update_note,
            controller::note::delete_note,
            controller::link::add_link,
            controller::link::get_link,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
