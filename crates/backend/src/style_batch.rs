//! The style batch collects the style commands that are generate in a frame
//! and sends them to the renderer as one batch.
//!
//! The batcher only sends out styles that changed. It does so by comparing the incomming
//! style with the cell style from the last frame and only outputs the changes.

use std::sync::{Arc, Weak};

use bevy::ecs::system::Resource;
use common::communication::{CellStyle, StyleCommand};
use uuid::Uuid;

#[derive(Resource, Default)]
pub struct StyleBatcher {
    commands: Vec<StyleCommand>,
}
impl StyleBatcher {
    /// Add a style to the batch
    pub fn add(&mut self, cell_id: &CellId, style: CellStyle) {
        // TODO: implement the change detection
        self.commands.push(StyleCommand {
            id: cell_id.id,
            style,
        });
    }

    pub fn drain(&mut self) -> Vec<StyleCommand> {
        std::mem::replace(&mut self.commands, Vec::new())
    }
}

/// Identifies a cell by a unique id.
pub struct CellId {
    id: Uuid,
    alive: Arc<()>,
}
impl CellId {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            alive: Arc::new(()),
        }
    }
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns a weak pointer to keep track of the aliveness of this CellId.
    pub fn weak(&self) -> Weak<()> {
        Arc::downgrade(&self.alive)
    }
}
