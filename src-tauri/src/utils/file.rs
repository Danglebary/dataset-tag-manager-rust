// use std::fs;
// use no_panic::no_panic;

// use super::error::{FileError, FileErrorType};
// use super::logger::Logger;
// use super::types::Dataset;

// #[no_panic]
// pub fn load_new_dataset(dir_path: String) -> Result<Dataset, FileError> {
//     // We are give a path to a directory
//     // using this, we need to grab all image files,
//     // and all .txt files associated with each image.
//     // if an image does not have a .txt file associated with it,
//     // we need to create one.
//     // file associations here are simple, they just need to have the same name.

//     // very first thing we need to do is save this dataset for use in "recent".
//     // saving a dataset is as simple as saving the name of the folder and the path to it
//     // in a json file. 
//     // we could make this more complicated and save our dataset structure, and add impls 
//     // for loading and saving from/to files, but for now we will 
//     // just re-parse everything when opening them as a starting point

//     // first, save the name and path of the dataset
//     let path_parts: Vec<&str> = dir_path.split("/").collect();

//     let dataset_name = match path_parts.last() {
//         Some(val) => *val,
//         None => 
//     }

//     let dataset_name = match dir_path.split("/").collect::<Vec<>>().last() {
//         Some(val) => val,
//     };
// }