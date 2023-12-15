use std::collections::HashMap;

use bevy::{
    app::{Plugin, Update},
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        schedule::IntoSystemConfigs,
        system::{Commands, Local, ResMut},
    },
};
use common::communication::{StyleCommand, ToRendererMessage};
use frontend::cell::{init_cell, CellSystem, SetStyle};
use uuid::Uuid;

use crate::{framerate::FrameCounter, websocket::ReceivedMessage};

pub struct CellManagerPlugin;
impl Plugin for CellManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, spawn_cells.before(CellSystem));
    }
}

fn spawn_cells(
    mut commands: Commands,
    mut received_messages: EventReader<ReceivedMessage>,
    mut set_style: EventWriter<SetStyle>,
    mut known_cells: Local<HashMap<Uuid, Entity>>,
    mut frame_counter: ResMut<FrameCounter>,
) {
    let style_commands: Vec<&StyleCommand> = received_messages
        .read()
        .filter_map(|ReceivedMessage { message }| match message {
            ToRendererMessage::Style(styles) => Some(styles),
            _ => None,
        })
        .flat_map(|styles| styles.iter())
        .collect();

    for command in style_commands.into_iter() {
        match command {
            StyleCommand::Style { id, style } => {
                let cell_id = known_cells
                    .entry(*id)
                    .or_insert_with(|| commands.spawn_empty().add(init_cell).id());

                frame_counter.inc();
                set_style.send(SetStyle {
                    entity: *cell_id,
                    style: style.clone(),
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
