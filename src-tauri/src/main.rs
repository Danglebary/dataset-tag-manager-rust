// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod utils;
mod menu;

fn main() {
    let app_menu = menu::new();


    tauri::Builder::default()
        .plugin(tauri_plugin_persisted_scope::init())
        .menu(app_menu)
        .on_menu_event(menu::app_menu_event_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
