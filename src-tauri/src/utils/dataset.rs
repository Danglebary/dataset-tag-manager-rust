use std::error::Error;
use std::path::Path;
use std::fs::{read_dir, read_to_string, write};

use serde::{ Serialize, Deserialize };

use super::logger::Logger;

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
    Name,
    Path,
    Read,
    Write,
    UnknownRead,
    ShouldBeImpossible,
}

#[derive(Debug)]
pub struct DatasetError {
    pub type_: DatasetErrorType,
    pub path: Option<String>,
}

impl DatasetError {
    pub fn new(type_: DatasetErrorType, path: Option<String>) -> DatasetError {
        DatasetError { type_, path }
    }
}

impl Error for DatasetError {}

impl std::fmt::Display for DatasetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let path = self.path.clone().unwrap_or("".to_string());
        match self.type_ {
            DatasetErrorType::Name => {
                let msg = format!("Error reading name from path '{}'", path);
                write!(f, "{msg}")
            },
            DatasetErrorType::Path => {
                let msg = format!("Error reading path '{}'", path);
                write!(f, "{msg}")
            },
            DatasetErrorType::Read => {
                let msg = format!("Error reading dataset from path '{}'", path);
                write!(f, "{msg}")
            },
            DatasetErrorType::Write => {
                let msg = format!("Error writing file to path '{}'", path);
                write!(f, "{msg}")
            },
            DatasetErrorType::UnknownRead => {
                let msg = format!("Unknown error occurred while reading dataset from path '{}'", path);
                write!(f, "{msg}")
            },
            DatasetErrorType::ShouldBeImpossible => {
                write!(f, "An error occurred that should be impossible to occur")
            },
        }
    }
}

impl Dataset {
    pub fn save_image_tags(&self) -> Result<Dataset, DatasetError> {
        for image in &self.data {
            let mut image_path = Path::new(&image.path).to_path_buf();
            image_path.set_extension("txt");

            let image_tags = image.tags.join(". ");

            let write_result = write(&image_path, image_tags);
            match write_result {
                Ok(_) => {},
                Err(_) => {
                    return Err(DatasetError::new(DatasetErrorType::Write, Some(image_path.to_string_lossy().to_string())));
                }
            }
        }

        Ok(self.clone())
    }

    pub fn delete_image_tag(&self, tag: String, image_name: String) -> Result<Dataset, DatasetError> {
        let mut dataset_data = self.data.clone();

        let mut image_index = None;
        for (index, dataset_image) in dataset_data.iter().enumerate() {
            if dataset_image.name == image_name {
                image_index = Some(index);
                break;
            }
        }

        if let Some(index) = image_index {
            let mut image = dataset_data[index].clone();

            let mut tag_index = None;
            for (index, image_tag) in image.tags.iter().enumerate() {
                if image_tag == &tag {
                    tag_index = Some(index);
                    break;
                }
            }

            if let Some(index) = tag_index {
                match Dataset::write_image_tags_for_file(&mut image) {
                    Ok(image) => {
                        // now we can remove the tag from the image in our local state
                        image.tags.remove(index);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            } else {
                // We shouldn't possibly be able to get here, but we'll handle it just in case
                return Err(DatasetError::new(DatasetErrorType::ShouldBeImpossible, None));
            }

            dataset_data[index] = image;
        }

        let dataset = Dataset {
            name: self.name.clone(),
            path: self.path.clone(),
            data: dataset_data
        };

        Ok(dataset)
    }

    pub fn update_image(&self, image: &mut DatasetImage) -> Result<Dataset, DatasetError> {
        let mut dataset_data = self.data.clone();

        let mut image_index = None;
        for (index, dataset_image) in dataset_data.iter().enumerate() {
            if dataset_image.name == image.name {
                image_index = Some(index);
                break;
            }
        }

        if let Some(index) = image_index {
            match Dataset::write_image_tags_for_file(image) {
                Ok(image) => {
                    // now we can update our local state
                    dataset_data[index] = image.clone();
                },
                Err(err) => {
                    return Err(err);
                }
            }
        } else {
            // We shouldn't possibly be able to get here, but we'll handle it just in case
            return Err(DatasetError::new(DatasetErrorType::ShouldBeImpossible, None));
        }

        let dataset = Dataset {
            name: self.name.clone(),
            path: self.path.clone(),
            data: dataset_data
        };

        Logger::info(&format!("updated dataset: {:?}", dataset));

        Ok(dataset)
    }

    pub fn write_image_tags_for_file(image: &mut DatasetImage) -> Result<&mut DatasetImage, DatasetError> {
        let mut image_path = Path::new(&image.path).to_path_buf();
        image_path.set_extension("txt");

        let image_tags = image.tags.join(". ");

        let write_result = write(&image_path, image_tags);
        match write_result {
            Ok(_) => {},
            Err(_) => {
                return Err(DatasetError::new(DatasetErrorType::Write, Some(image_path.to_string_lossy().to_string())));
            }
        }

        Ok(image)
    }

    
    // TODO: custom error types, will allow us to handle showing error dialogs to the user
    pub fn from_path(path: &Path) -> Result<Dataset, DatasetError> {
        let dataset_name = match path.file_name() {
            Some(name) => match name.to_str() {
                Some(name) => name.to_string(),
                None => return Err(DatasetError::new(DatasetErrorType::Name, Some(path.to_string_lossy().to_string())))
            },
            None => return Err(DatasetError::new(DatasetErrorType::Name, Some(path.to_string_lossy().to_string())))
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
                return Err(DatasetError::new(DatasetErrorType::Read, Some(path.to_string_lossy().to_string())));
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => {
                    return Err(DatasetError::new(DatasetErrorType::Read, Some(path.to_string_lossy().to_string())));
                }
            };

            let dataimage_path = entry.path();
            let dataimage_name = match dataimage_path.file_name() {
                Some(name) => match name.to_str() {
                    Some(name) => name.to_string(),
                    None => {
                        return Err(DatasetError::new(DatasetErrorType::Name, Some(dataimage_path.to_string_lossy().to_string())));
                    }
                },
                None => {
                    return Err(DatasetError::new(DatasetErrorType::Name, Some(dataimage_path.to_string_lossy().to_string())));
                }
            };

            if dataimage_path.is_file() && is_image_file(&dataimage_path) {
                let mut datatags_path = entry.path().clone();
                datatags_path.set_extension("txt");

                // we want to check if the datatags_path exists
                // if it does, we want to read the tags from the file
                // if it does not, we want to create the file and add it to the datasetImage
                let datatags_data: Vec<String> = match read_to_string(&datatags_path) {
                    Ok(contents) => contents.split('.').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
                    Err(_) => {
                        let write_result = write(&datatags_path, "");
                        match write_result {
                            Ok(_) => {},
                            Err(_) => {
                                return Err(DatasetError::new(DatasetErrorType::Write, Some(datatags_path.to_string_lossy().to_string())));
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

            matches!(ext, "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "tif" | "webp")
        },
        None => false
    }
}