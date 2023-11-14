mod file;
mod named;

use tauri::{ Menu, WindowMenuEvent, Manager };

use crate::{utils::logger::Logger, state::DatasetState};

use self::file::{open_dataset_handler, save_dataset_handler};

pub fn new() -> Menu {
    let named_submenu = named::get_named_submenu();
    let file_submenu = file::get_file_submenu();

    Menu::new().add_submenu(named_submenu).add_submenu(file_submenu)
}

pub fn app_menu_event_handler(event: WindowMenuEvent) {
    // TODO: we don't want to panic, remove any unwraps and replace with proper error handling
    let window = event.window();
    let app = window.app_handle();
    let state = app.state::<DatasetState>();
    let locked_state = state.dataset.lock().expect("Could not lock dataset");
    let dataset = locked_state.as_ref().clone();
    // we want to get the app state so we can get the dataset to pass to the save handler
    match event.menu_item_id() {
        // we pass the window to the handler so we can update the window title with the name of the dataset
        "open_dataset" => open_dataset_handler(window),
        "save_dataset" => save_dataset_handler(window, dataset),
        _ => {
            // error if none of the above passes
            Logger::debug(&format!("tauri event {:?}", event))
        }
    }
}