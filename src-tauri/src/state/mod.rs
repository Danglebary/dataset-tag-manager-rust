use std::sync::Mutex;

use crate::utils::dataset::Dataset;

pub struct DatasetState {
    pub dataset: Mutex<Option<Dataset>>
}