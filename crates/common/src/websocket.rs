use serde::{Deserialize, Serialize};

use crate::cell::style::CellStyleMessage;

#[derive(Serialize, Deserialize)]
pub enum Message {
    Opened,
    CellStyle(CellStyleMessage),
}
