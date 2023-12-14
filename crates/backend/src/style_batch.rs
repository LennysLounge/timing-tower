//! The style batch collects the style commands that are generate in a frame
//! and sends them to the renderer as one batch.
//!
//! The batcher only sends out styles that changed. It does so by comparing the incomming
//! style with the cell style from the last frame and only outputs the changes.

use std::collections::HashMap;

use bevy::ecs::system::Resource;
use common::communication::{CellStyle, StyleCommand};
use uuid::Uuid;

#[derive(Resource, Default)]
pub struct StyleBatch {
    _known_styles: HashMap<Uuid, CellStyle>,
    commands: Vec<StyleCommand>,
}
impl StyleBatch {
    /// Add a style to the batch
    pub fn add(&mut self, id: Uuid, style: CellStyle) {
        // TODO: implement the change detection
        self.commands.push(StyleCommand { id, style });
    }

    pub fn drain(&mut self) -> Vec<StyleCommand> {
        std::mem::replace(&mut self.commands, Vec::new())
    }
}
