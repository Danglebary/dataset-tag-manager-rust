use tauri::{Submenu, CustomMenuItem, Menu, api::dialog, Window};

use crate::utils::logger::Logger;
use crate::utils::dataset::Dataset;

pub fn get_file_submenu() -> Submenu {
    let open_item = CustomMenuItem::new("open_dataset".to_string(), "Open Dataset...").accelerator("Cmd+o").into();
    let save_item = CustomMenuItem::new("save_dataset".to_string(), "Save Dataset...").accelerator("Cmd+s").disabled().into();

    Submenu::new("File", Menu::with_items([open_item, save_item]))
}

pub fn open_dataset_handler(main_window: &Window) {
    // we need to clone the main window so we can use it in the callback
    let window = main_window.clone();
    dialog::FileDialogBuilder::default().pick_folder(move  |path_buf| {
        // The only way for path_buf to be None is if the user canceled the dialog.
        // We can just ignore it in that case.
        if let Some(buf) = path_buf {
            let path = buf.as_path();

            // we want to now load the dataset from the path
            let dataset = match Dataset::from_path(path) {
                Ok(dataset) => dataset,
                Err(err) => {
                    // if the dataset is an error, we want to show an error dialog to the user and return
                    dialog::message(Some(&window), "Error loading Dataset", &format!("An error occurred while loading the Dataset. Please try again.\n\n{}", err));
                    return;
                }
            };

            // if the dataset was successfully loaded, we want to do a couple things:
            // 1. pass the dataset to the main window
            let _ = window.emit("dataset_loaded", dataset.clone()).map_err(|err| Logger::error(&format!("Error sending dataset to main window: {}", err)));
            // 2. set the window title to the name of the dataset
            let _ = window.set_title(dataset.name.as_str()).map_err(|err| Logger::error(&format!("Error setting window title: {}", err)));
            // 3. enable the save menu item
            let _ = window.menu_handle().get_item("save_dataset").set_enabled(true).map_err(|err| Logger::error(&format!("Error enabling save menu item: {}", err)));
        }
    });
}

pub fn save_dataset_handler() {
    println!("save dataset");
}