mod framerate;
mod websocket;

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
    math::vec3,
    prelude::{App, Camera2dBundle, ClearColor, Color, EventReader, Startup},
    transform::components::Transform,
    window::{PrimaryWindow, Window},
    DefaultPlugins,
};
use common::{
    cell::{
        init_cell,
        style::{CellStyle, SetStyle},
        CellMarker, CellPlugin, CellSystem,
    },
    communication::{ToControllerMessage, ToRendererMessage},
    gradient_material::CustomMaterialPlugin,
};
use framerate::{FrameCounter, FrameratePlugin};
use websocket::{ReceivedMessages, SendMessage, WebsocketPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_plugins((CellPlugin, CustomMaterialPlugin))
        .add_plugins((WebsocketPlugin, FrameratePlugin))
        .add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            (mouse_click_send_message, spawn_cells.before(CellSystem)),
        )
        .run();
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
    mut received_messages: EventReader<ReceivedMessages>,
    mut set_style: EventWriter<SetStyle>,
    mut frame_counter: ResMut<FrameCounter>,
) {
    for messages in received_messages.read() {
        for message in messages.messages.iter() {
            match message {
                ToRendererMessage::CellStyle(styles) => {
                    let cell_ids: Vec<Entity> = cells.iter().collect();
                    for (index, style) in styles.iter().enumerate() {
                        let cell_id = cell_ids
                            .get(index)
                            .map(|e| *e)
                            .unwrap_or_else(|| commands.spawn_empty().add(init_cell).id());
                        set_style.send(SetStyle {
                            entity: cell_id,
                            style: CellStyle {
                                text: style.text.clone(),
                                text_color: style.text_color,
                                text_size: style.text_size,
                                text_alignment: style.text_alignment.clone(),
                                text_position: style.text_position,
                                color: style.color,
                                texture: None,
                                pos: style.pos,
                                size: style.size,
                                skew: style.skew,
                                visible: style.visible,
                                rounding: style.rounding,
                            },
                        });
                    }
                    frame_counter.inc();
                }
            }
        }
    }
}
