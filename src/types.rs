use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IndexResponse {
    pub message: String,
}
