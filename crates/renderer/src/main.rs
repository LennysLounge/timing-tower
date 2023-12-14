mod asset_path_store;
mod framerate;
mod websocket;

use asset_path_store::WebAssetPathStorePlugin;
use bevy::{
    app::Update,
    ecs::{
        entity::Entity,
        event::EventWriter,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, ResMut},
    },
    input::mouse::MouseButtonInput,
    math::{vec2, vec3},
    prelude::{App, Camera2dBundle, ClearColor, Color, EventReader, Startup},
    transform::components::Transform,
    window::{PrimaryWindow, Window},
    DefaultPlugins,
};
use common::communication::{CellStyle, ToControllerMessage, ToRendererMessage};
use framerate::{FrameCounter, FrameratePlugin};
use frontend::{
    cell::{init_cell, CellMarker, CellSystem, SetStyle},
    FrontendPlugin,
};

use tracing::info;
use websocket::{ReceivedMessage, SendMessage, WebsocketPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_plugins(FrontendPlugin)
        .add_plugins(WebAssetPathStorePlugin)
        .add_plugins((WebsocketPlugin, FrameratePlugin))
        .add_systems(Startup, setup_camera)
        //.add_systems(Startup, setup_cell)
        .add_systems(
            Update,
            (mouse_click_send_message, spawn_cells.before(CellSystem)),
        )
        .run();
}

#[allow(unused)]
fn setup_cell(mut commands: Commands, mut set_style: EventWriter<SetStyle>) {
    let cell_id = commands.spawn_empty().add(init_cell).id();
    set_style.send(SetStyle {
        entity: cell_id,
        style: CellStyle {
            text: String::from(""),
            text_color: Color::BLACK,
            text_size: 40.0,
            text_alignment: common::communication::TextAlignment::Center,
            text_position: vec2(0.0, 0.0),
            color: Color::BLUE,
            texture: None,
            pos: vec3(320.0, 540.0, 0.0),
            size: vec2(640.0, 360.0),
            skew: 0.0,
            visible: true,
            rounding: [100.0, 200.0, 50.0, 150.0],
        },
    });
}

fn setup_camera(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.single();
    let x = window.physical_width() / 2;
    let y = window.physical_height() / 2;
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(vec3(x as f32, y as f32, 0.0))
            .with_scale(vec3(2.0, 2.0, 1.0)),
        ..Default::default()
    });
}

fn mouse_click_send_message(
    mut mouse_event: EventReader<MouseButtonInput>,
    mut send_message: EventWriter<SendMessage>,
) {
    for ev in mouse_event.read() {
        if let bevy::input::ButtonState::Pressed = ev.state {
            send_message.send(SendMessage {
                message: ToControllerMessage::Debug("Mouse pressed".to_owned()),
            });
        }
    }
}

fn spawn_cells(
    mut commands: Commands,
    cells: Query<Entity, With<CellMarker>>,
    mut received_messages: EventReader<ReceivedMessage>,
    mut set_style: EventWriter<SetStyle>,
    mut frame_counter: ResMut<FrameCounter>,
) {
    let cell_ids: Vec<Entity> = cells.iter().collect();
    let style_commands = received_messages
        .read()
        .filter_map(|ReceivedMessage { message }| match message {
            ToRendererMessage::Style(styles) => {
                frame_counter.inc();
                Some(styles)
            }
            _ => None,
        })
        .flat_map(|styles| styles.iter());
    for (index, command) in style_commands.enumerate() {
        let cell_id = cell_ids.get(index).map(|e| *e).unwrap_or_else(|| {
            info!("Spawn entity");
            commands.spawn_empty().add(init_cell).id()
        });
        set_style.send(SetStyle {
            entity: cell_id,
            style: command.style.clone(),
        });
    }
}
