mod asset_path_store;
mod cell_manager;
mod framerate;
mod websocket;

use asset_path_store::WebAssetPathStorePlugin;
use bevy::{
    app::{PluginGroup, Update},
    ecs::{
        event::EventWriter,
        query::With,
        system::{Commands, Query},
    },
    input::mouse::MouseButtonInput,
    math::{vec2, vec3},
    prelude::{App, Camera2dBundle, ClearColor, Color, EventReader, Startup},
    transform::components::Transform,
    window::{PrimaryWindow, Window, WindowPlugin},
    DefaultPlugins, asset::AssetMetaCheck,
};
use cell_manager::CellManagerPlugin;
use common::communication::ToControllerMessage;
use framerate::FrameratePlugin;
use frontend::{
    cell::{CreateCell, SetStyle},
    FrontendPlugin,
};

use websocket::{SendMessage, WebsocketPlugin};

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(FrontendPlugin)
        .add_plugins((
            WebsocketPlugin,
            FrameratePlugin,
            WebAssetPathStorePlugin,
            CellManagerPlugin,
        ))
        .add_systems(Startup, setup_camera)
        //.add_systems(Startup, setup_cell)
        .add_systems(Update, mouse_click_send_message)
        .run();
}

#[allow(unused)]
fn setup_cell(mut commands: Commands, mut set_style: EventWriter<SetStyle>) {
    let cell_id = commands.spawn_empty().add(CreateCell).id();
    set_style.send(SetStyle {
        entity: cell_id,
        style: frontend::cell::CellStyle {
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
            render_layer: 0,
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
