use std::collections::HashMap;

use backend::style_batcher::{PrepareBatcher, StyleBatcher};
use bevy::{
    app::{Plugin, PostUpdate},
    ecs::{
        entity::Entity,
        event::EventWriter,
        schedule::IntoSystemConfigs,
        system::{Commands, Local, ResMut},
    },
};
use common::communication::StyleCommand;
use frontend::cell::{CreateCell, SetStyle};
use uuid::Uuid;

pub struct CellManagerPlugin;
impl Plugin for CellManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostUpdate, execute_style_commands.after(PrepareBatcher));
    }
}

fn execute_style_commands(
    mut style_batcher: ResMut<StyleBatcher>,
    mut known_cells: Local<HashMap<Uuid, Entity>>,
    mut set_style: EventWriter<SetStyle>,
    mut commands: Commands,
) {
    let style_commands = style_batcher.drain();
    for command in style_commands.into_iter() {
        match command {
            StyleCommand::Style { id, style } => {
                let cell_id = known_cells
                    .entry(id)
                    .or_insert_with(|| commands.spawn_empty().add(CreateCell).id());

                set_style.send(SetStyle {
                    entity: *cell_id,
                    style,
                });
            }
            StyleCommand::Remove { id } => {
                if let Some(cell_id) = known_cells.remove(&id) {
                    commands.entity(cell_id).despawn();
                }
            }
        }
    }
}
