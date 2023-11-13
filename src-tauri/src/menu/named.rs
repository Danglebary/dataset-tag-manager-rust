use tauri::{Submenu, Menu, MenuItem, AboutMetadata};

pub fn get_named_submenu() -> Submenu {
    Submenu::new("Dataset Tag Manager", Menu::with_items([MenuItem::About("Dataset Tag Manager".to_string(), AboutMetadata::new().authors(vec!["Austin See <halfblowncontact@gmail.com>".to_string()]).comments("Wow, this is an app!")).into()]))
}