//! The style batch collects the style commands that are generate in a frame
//! and sends them to the renderer as one batch.
//!
//! The batcher only sends out styles that changed. It does so by comparing the incomming
//! style with the cell style from the last frame and only outputs the changes.

use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use bevy::{
    app::{First, Plugin, Update},
    ecs::{
        schedule::{IntoSystemConfigs, SystemSet},
        system::{ResMut, Resource},
    },
};
use common::communication::{CellStyle, ClipAreaStyle, StyleCommand};
use uuid::Uuid;

use crate::timing_tower::StyleElementUpdate;

pub struct StyleBatcherPlugin;
impl Plugin for StyleBatcherPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<StyleBatcher>()
            .add_systems(First, clear_style_batcher)
            .add_systems(
                Update,
                prepare_batcher
                    .in_set(PrepareBatcher)
                    .after(StyleElementUpdate),
            );
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct PrepareBatcher;

#[derive(Resource, Default)]
pub struct StyleBatcher {
    last_styles: HashMap<Uuid, RememberedStyle>,
    commands: Vec<StyleCommand>,
}
impl StyleBatcher {
    /// Add a style for a cell.
    pub fn add(&mut self, cell_id: &CellId, style: CellStyle) {
        // TODO: implement the change detection
        self.last_styles.insert(
            cell_id.id,
            RememberedStyle {
                id_ref: cell_id.weak(),
                _style: CellType::Cell(style.clone()),
            },
        );
        self.commands.push(StyleCommand::Style {
            id: cell_id.id,
            style,
        });
    }

    /// Add a style command for a clip area.
    pub fn add_clip_area(&mut self, cell_id: &CellId, style: ClipAreaStyle) {
        // TODO: implement change detection.
        self.last_styles.insert(
            cell_id.id,
            RememberedStyle {
                id_ref: cell_id.weak(),
                _style: CellType::ClipArea(style.clone()),
            },
        );
        self.commands.push(StyleCommand::ClipArea {
            id: cell_id.id,
            style,
        });
    }

    pub fn drain(&mut self) -> Vec<StyleCommand> {
        std::mem::replace(&mut self.commands, Vec::new())
    }
}

fn clear_style_batcher(mut batcher: ResMut<StyleBatcher>) {
    batcher.drain();
}

fn prepare_batcher(mut batcher: ResMut<StyleBatcher>) {
    // Test all known cells if they are still alive and remove the dead ones.
    let dead_cells: Vec<_> = batcher
        .last_styles
        .iter()
        .filter_map(|(id, remembered_style)| {
            (remembered_style.id_ref.strong_count() == 0).then_some(*id)
        })
        .collect();

    dead_cells.into_iter().for_each(|cell_id| {
        batcher.last_styles.remove(&cell_id);
        batcher.commands.push(StyleCommand::Remove { id: cell_id });
    });
}

struct RememberedStyle {
    id_ref: Weak<()>,
    _style: CellType,
}

enum CellType {
    Cell(CellStyle),
    ClipArea(ClipAreaStyle),
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
    /// Id of the cell.
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    /// Returns a weak pointer to keep track of the aliveness of this CellId.
    pub fn weak(&self) -> Weak<()> {
        Arc::downgrade(&self.alive)
    }

    // /// Returns a weak pointer to keep track of the aliveness of this CellId.
    // pub fn get_reference(&self) -> CellIdReference {
    //     CellIdReference {
    //         id: self.id,
    //         alive: Arc::downgrade(&self.alive),
    //     }
    // }
}
