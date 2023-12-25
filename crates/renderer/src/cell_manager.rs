use std::collections::HashMap;

use bevy::{
    app::{Plugin, Update},
    asset::AssetServer,
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        schedule::IntoSystemConfigs,
        system::{Commands, Local, Res, ResMut},
    },
};
use common::communication::{StyleCommand, ToRendererMessage};
use frontend::{
    asset_path_store::{AssetPathProvider, AssetPathStore},
    cell::{CellSystem, CreateCell, SetStyle},
};
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
    asset_server: Res<AssetServer>,
    asset_path_store: ResMut<AssetPathStore>,
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
                    .or_insert_with(|| commands.spawn_empty().add(CreateCell).id());

                frame_counter.inc();
                set_style.send(SetStyle {
                    entity: *cell_id,
                    style: frontend::cell::CellStyle {
                        text: style.text.clone(),
                        text_color: style.text_color,
                        text_size: style.text_size,
                        text_alignment: style.text_alignment,
                        text_position: style.text_position,
                        color: style.color,
                        texture: style
                            .texture
                            .as_ref()
                            .and_then(|id| asset_path_store.get(id))
                            .and_then(|path| Some(asset_server.load(path))),
                        pos: style.pos,
                        size: style.size,
                        skew: style.skew,
                        visible: style.visible,
                        rounding: style.rounding,
                        render_layer: style.render_layer,
                    },
                });
            }
            StyleCommand::ClipArea { id: _, style: _ } => {
                todo!()
            }
            StyleCommand::Remove { id } => {
                if let Some(cell_id) = known_cells.remove(&id) {
                    commands.entity(cell_id).despawn();
                }
            }
        }
    }
}
