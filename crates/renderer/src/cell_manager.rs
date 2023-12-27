use bevy::{
    app::{Plugin, Update},
    asset::{AssetServer, Assets},
    ecs::{
        event::{EventReader, EventWriter},
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Local, Query, Res, ResMut},
    },
    render::{camera::Camera, texture::Image},
    transform::components::Transform,
};
use common::communication::ToRendererMessage;
use frontend::{
    cell::{CellSystem, SetStyle},
    cell_manager::CellManager,
};

use crate::{
    asset_path_store::WebAssetPathStore, framerate::FrameCounter, websocket::ReceivedMessage,
};

pub struct CellManagerPlugin;
impl Plugin for CellManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, spawn_cells.before(CellSystem));
    }
}

fn spawn_cells(
    mut cell_manager: Local<CellManager>,
    mut received_messages: EventReader<ReceivedMessage>,
    mut frame_counter: ResMut<FrameCounter>,
    commands: Commands,
    set_style: EventWriter<SetStyle>,
    images: ResMut<Assets<Image>>,
    cameras: Query<&mut Transform, With<Camera>>,
    asset_server: Res<AssetServer>,
    asset_path_store: ResMut<WebAssetPathStore>,
) {
    let style_commands: Vec<_> = received_messages
        .read()
        .filter_map(|ReceivedMessage { message }| match message {
            ToRendererMessage::Style(styles) => Some(styles),
            _ => None,
        })
        .flat_map(|styles| styles.iter())
        .map(|x| x.clone())
        .map(|x| {
            frame_counter.inc();
            x
        })
        .collect();

    cell_manager.apply_commands(
        style_commands,
        set_style,
        commands,
        images,
        cameras,
        asset_server,
        asset_path_store.as_ref(),
    );
}
