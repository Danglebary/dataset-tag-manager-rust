use tauri::Manager;
use tauri::{Submenu, CustomMenuItem, Menu, api::dialog, Window};

use crate::state;
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
                    dialog::message(Some(&window), "Error loading Dataset", format!("An error occurred while loading the Dataset. Please try again.\n\n{}", err));
                    return;
                }
            };

            // if the dataset was successfully loaded, we want to do a couple things:
            // 1. pass the dataset to the main window
            let _ = window.emit("dataset_loaded", dataset.clone()).map_err(|err| Logger::error(&format!("Error sending dataset to main window: {}", err)));
            // 2. set the window title to the name of the dataset
            let _ = window.set_title(dataset.name.as_str()).map_err(|err| Logger::error(&format!("Error setting window title: {}", err)));
            // TODO: do we want to enable the save menu right away? or only after the user has made changes?
            // It's a good question, since we technically make changes to the dataset when we load it, since we trim the tags.
            // For now, we'll keep enabling it right away.
            // 3. enable the save menu item
            let _ = window.menu_handle().get_item("save_dataset").set_enabled(true).map_err(|err| Logger::error(&format!("Error enabling save menu item: {}", err)));

            // now that we've done all that, we want to set the dataset in the app state
            let _ = window.app_handle().state::<state::DatasetState>().dataset.lock().map(|mut dataset_state| {
                *dataset_state = Some(dataset);
            }).map_err(|err| Logger::error(&format!("Error setting dataset in app state: {}", err)));
        }
    });
}

pub fn save_dataset_handler(main_window: &Window, dataset: Option<&Dataset>) {
    if dataset.is_none() {
        // if the dataset is None, we want to show an error dialog to the user and return
        dialog::message(Some(main_window), "Error saving Dataset", "An error occurred while saving the Dataset. Please try again.");
        return;
    }

    // we know the dataset is Some, so we can unwrap it
    match dataset.unwrap().save_image_tags() {
        Ok(dataset) => {
            // if the dataset was successfully saved, we want to do a couple things:
            // 1. update the dataset in the app state
            let _ = main_window.app_handle().state::<state::DatasetState>().dataset.lock().map(|mut dataset_state| {
                *dataset_state = Some(dataset);
            }).map_err(|err| Logger::error(&format!("Error setting dataset in app state: {}", err)));

            dialog::message(Some(main_window), "Dataset Saved", "The Dataset was successfully saved.");
            // Now that we've saved the dataset, we want to disable the save menu item again, until the user makes changes
            let _ = main_window.menu_handle().get_item("save_dataset").set_enabled(false).map_err(|err| Logger::error(&format!("Error disabling save menu item: {}", err)));
        },
        Err(err) => {
            dialog::message(Some(main_window), "Error Saving Dataset", format!("An error occurred while saving the Dataset. Please try again.\n\n{}", err));
        }
    }
}