use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CellStyle {
    pub message: String,
    pub number: i32,
}
