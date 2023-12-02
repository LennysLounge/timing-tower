use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cell::style::CellStyle;

/// Messages that are send by the controller to renderers.
#[derive(Serialize, Deserialize)]
pub enum ToRendererMessage {
    Assets { images: Vec<(Uuid, String)> },
    CellStyle(Vec<CellStyle>),
}

/// Messages that are send by renderes to the controller.
#[derive(Serialize, Deserialize)]
pub enum ToControllerMessage {
    Opened,
    AssetsLoaded,
    Debug(String),
}
