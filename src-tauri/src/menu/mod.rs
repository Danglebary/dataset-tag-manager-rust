mod file;
mod named;

use tauri::{ Menu, WindowMenuEvent };

use crate::utils::logger::Logger;

use self::file::{open_dataset_handler, save_dataset_handler};

pub fn new() -> Menu {
    let named_submenu = named::get_named_submenu();
    let file_submenu = file::get_file_submenu();

    Menu::new().add_submenu(named_submenu).add_submenu(file_submenu)
}

pub fn app_menu_event_handler(event: WindowMenuEvent) {
    let window = event.window();
    match event.menu_item_id() {
        // we pass the window to the handler so we can update the window title with the name of the dataset
        "open_dataset" => open_dataset_handler(window),
        "save_dataset" => save_dataset_handler(),
        _ => {
            // error if none of the above passes
            Logger::debug(&format!("tauri event {:?}", event))
        }
    }
}