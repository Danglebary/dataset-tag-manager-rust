// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

mod commands;
mod utils;
mod menu;
mod state;

fn main() {
    let app_menu = menu::new();


    tauri::Builder::default()
        .plugin(tauri_plugin_persisted_scope::init())
        .manage(state::DatasetState { dataset: Mutex::new(None) })
        .invoke_handler(tauri::generate_handler![commands::tags::save_dataset_image_tags, commands::tags::delete_dataset_image_tag])
        .menu(app_menu)
        .on_menu_event(menu::app_menu_event_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
