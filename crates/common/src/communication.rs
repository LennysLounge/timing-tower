use serde::{Deserialize, Serialize};

use crate::cell::style::CellStyleMessage;

/// Messages that are send by the controller to renderers.
#[derive(Serialize, Deserialize)]
pub enum ToRendererMessage {
    CellStyle(Vec<CellStyleMessage>),
}

/// Messages that are send by renderes to the controller.
#[derive(Serialize, Deserialize)]
pub enum ToControllerMessage {
    Opened,
}
