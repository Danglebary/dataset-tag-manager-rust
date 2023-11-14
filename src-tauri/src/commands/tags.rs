use tauri::State;

use crate::{utils::{dataset::DatasetImage, logger::Logger}, state::DatasetState};


#[tauri::command]
pub fn save_dataset_image_tags(image: DatasetImage, state: State<DatasetState>, window: tauri::Window) -> bool {
    let mut dataset = state.dataset.lock().unwrap();
    if let Some(dataset) = &mut *dataset {
        let mut clone = image.clone();
        match dataset.update_image(&mut clone) {
            Ok(new) => {
                *dataset = new;

                // now that we know the dataset is updated, we want to enable the save button
                // if it is currently disabled
                let _ = window.menu_handle().get_item("save_dataset").set_enabled(false).map_err(|err| Logger::error(&format!("Error disabling save menu item: {}", err)));
                Logger::info(&format!("Saved image tags for image '{}'", image.name));
                true
            },
            Err(err) => {
                Logger::error(&format!("Could not save image tags for image '{}': {}", image.name, err));
                false
            }
        }
    } else {
        Logger::error(&format!("Could not save image tags for image '{}': dataset is None", image.name));
        false
    }
}

#[tauri::command]
pub fn delete_dataset_image_tag(tag: String, image_name: String, state: State<DatasetState>) -> bool {
    let mut dataset = state.dataset.lock().unwrap();
    if let Some(dataset) = &mut *dataset {
        match dataset.delete_image_tag(tag.clone(), image_name) {
            Ok(new) => {
                *dataset = new;
                Logger::info(&format!("Deleted image tag '{}'", tag));
                true
            },
            Err(err) => {
                Logger::error(&format!("Could not delete image tag '{}': {}", tag, err));
                false
            }
        }
    } else {
        Logger::error("Could not delete image tag: dataset is None");
        false
    }
}