use crate::models::apod::ApodId;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserGallery {
    pub email: String,
    pub apod_id : i32,
}