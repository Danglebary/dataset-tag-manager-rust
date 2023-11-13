use std::error::Error;
use std::path::Path;
use std::fs::{read_dir, read_to_string, write};

use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatasetImage {
    pub name: String,
    pub path: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dataset {
    pub name: String,
    pub path: String,
    pub data: Vec<DatasetImage>
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum DatasetErrorType {
    NameError,
    PathError,
    ReadError,
    WriteError,
    UnknownError,
}

#[derive(Debug)]
pub struct DatasetError {
    pub type_: DatasetErrorType,
    pub path: String,
}

impl DatasetError {
    pub fn new(type_: DatasetErrorType, path: String) -> DatasetError {
        DatasetError { type_, path }
    }
}

impl Error for DatasetError {}

impl std::fmt::Display for DatasetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.type_ {
            DatasetErrorType::NameError => {
                let msg = format!("Error reading name from path '{}'", self.path);
                write!(f, "{msg}")
            },
            DatasetErrorType::PathError => {
                let msg = format!("Error reading path '{}'", self.path);
                write!(f, "{msg}")
            },
            DatasetErrorType::ReadError => {
                let msg = format!("Error reading dataset from path '{}'", self.path);
                write!(f, "{msg}")
            },
            DatasetErrorType::WriteError => {
                let msg = format!("Error writing file to path '{}'", self.path);
                write!(f, "{msg}")
            },
            DatasetErrorType::UnknownError => {
                let msg = format!("Unknown error occurred while reading dataset from path '{}'", self.path);
                write!(f, "{msg}")
            }
        }
    }
}

impl Dataset {
    // TODO: custom error types, will allow us to handle showing error dialogs to the user
    pub fn from_path(path: &Path) -> Result<Dataset, DatasetError> {
        let dataset_name = match path.file_name() {
            Some(name) => match name.to_str() {
                Some(name) => name.to_string(),
                None => return Err(DatasetError::new(DatasetErrorType::NameError, path.to_string_lossy().to_string()))
            },
            None => return Err(DatasetError::new(DatasetErrorType::NameError, path.to_string_lossy().to_string()))
        };

        let dataset_path = path.to_string_lossy().to_string();

        // we want to iterate through the files in the directory
        // we want to create a DatasetImage for each image file in the directory if that image file.
        // if the image file has a corresponding txt file, we want to read the tags from that file and add them to the DatasetImage as its tags
        // if the image file does not have a corresponding txt file, we want to create a new txt file with the same name as the image file and add it to the DatasetImage as its tags (which will be empty)
        // we want to add the DatasetImage to the Dataset's data vector
        // we want to return the Dataset

        let mut dataset_data: Vec<DatasetImage> = Vec::new();

        let entries = match read_dir(path) {
            Ok(entries) => entries,
            Err(_) => {
                return Err(DatasetError::new(DatasetErrorType::ReadError, path.to_string_lossy().to_string()));
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => {
                    return Err(DatasetError::new(DatasetErrorType::ReadError, path.to_string_lossy().to_string()));
                }
            };

            let dataimage_path = entry.path();
            let dataimage_name = match dataimage_path.file_name() {
                Some(name) => match name.to_str() {
                    Some(name) => name.to_string(),
                    None => {
                        return Err(DatasetError::new(DatasetErrorType::NameError, dataimage_path.to_string_lossy().to_string()));
                    }
                },
                None => {
                    return Err(DatasetError::new(DatasetErrorType::NameError, dataimage_path.to_string_lossy().to_string()));
                }
            };

            if dataimage_path.is_file() && is_image_file(&dataimage_path) {
                let mut datatags_path = entry.path().clone();
                datatags_path.set_extension("txt");

                // we want to check if the datatags_path exists
                // if it does, we want to read the tags from the file
                // if it does not, we want to create the file and add it to the datasetImage
                let datatags_data: Vec<String> = match read_to_string(&datatags_path) {
                    Ok(contents) => contents.split(".").map(|s| s.trim().to_string()).filter(|s| s.len() > 0).collect(),
                    Err(_) => {
                        let write_result = write(&datatags_path, "");
                        match write_result {
                            Ok(_) => {},
                            Err(_) => {
                                return Err(DatasetError::new(DatasetErrorType::WriteError, datatags_path.to_string_lossy().to_string()));
                            }
                        }
                        vec!["".to_string()]
                    }
                };

                let image = DatasetImage {
                    name: dataimage_name.clone(),
                    path: dataimage_path.to_string_lossy().to_string(),
                    tags: datatags_data
                };

                dataset_data.push(image);
            }
        };

        Ok(Dataset {
            name: dataset_name,
            path: dataset_path,
            data: dataset_data
        })
    }
}

fn is_image_file(path: &Path) -> bool {
    match path.extension() {
        Some(extension) => {
            let ext = match extension.to_str() {
                Some(extension) => extension,
                None => return false
            };

            match ext {
                "jpg" => true,
                "jpeg" => true,
                "png" => true,
                "gif" => true,
                "bmp" => true,
                "tiff" => true,
                "tif" => true,
                "webp" => true,
                _ => false
            }
        },
        None => false
    }
}