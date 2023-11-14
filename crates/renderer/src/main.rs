use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventWriter,
        query::With,
        system::{Commands, Query, Res},
    },
    hierarchy::DespawnRecursiveExt,
    input::mouse::MouseButtonInput,
    math::vec3,
    prelude::{
        App, Camera2dBundle, ClearColor, Color, EventReader, NonSendMut, Startup, Update, World,
    },
    text::{Text, TextSection, TextStyle},
    transform::components::Transform,
    ui::{node_bundles::TextBundle, Style, Val},
    window::{PrimaryWindow, Window},
    DefaultPlugins,
};
use common::{
    cell::{
        init_cell,
        style::{CellStyle, SetStyle},
        CellMarker, CellPlugin,
    },
    communication::{ToControllerMessage, ToRendererMessage},
    gradient_material::CustomMaterialPlugin,
};
use ewebsock::{WsMessage, WsReceiver, WsSender};
use tracing::info;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .add_plugins(CellPlugin)
        .add_plugins(CustomMaterialPlugin)
        .add_systems(Startup, (setup, setup_camera))
        .add_systems(Update, (send_message, read_cell_render, text_update_system))
        .run();
}

struct Websocket {
    sender: WsSender,
    receiver: WsReceiver,
}

fn setup(world: &mut World) {
    info!("Connecting to websocket");
    let (sender, receiver) = ewebsock::connect("ws://127.0.0.1:8001").unwrap();

    world.insert_non_send_resource(Websocket {
        sender: sender,
        receiver: receiver,
    });

    // Text with multiple sections
    world.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 60.0,
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 60.0,
                color: Color::GOLD,
                // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                ..Default::default()
            }),
            TextSection::new(
                "cell count: ",
                TextStyle {
                    font_size: 60.0,
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 60.0,
                color: Color::GOLD,
                // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                ..Default::default()
            }),
        ])
        .with_style(Style {
            position_type: bevy::ui::PositionType::Absolute,
            left: Val::Px(5.0),
            top: Val::Px(5.0),
            ..Default::default()
        }),
        FpsText,
    ));
}
fn setup_camera(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.single();
    let x = window.physical_width() / 2;
    let y = window.physical_height() / 2;
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(vec3(x as f32, y as f32, 0.0)),
        ..Default::default()
    });
}

fn read_cell_render(
    mut commands: Commands,
    set_style_event: EventWriter<SetStyle>,
    mut websocket: NonSendMut<Websocket>,
    cells: Query<Entity, With<CellMarker>>,
) {
    for entity_id in cells.iter() {
        commands.entity(entity_id).despawn_recursive();
    }

    if let Some(event) = websocket.receiver.try_recv() {
        match event {
            ewebsock::WsEvent::Opened => {
                let data =
                    postcard::to_allocvec(&ToControllerMessage::Opened).expect("Cannot serialize");
                websocket.sender.send(WsMessage::Binary(data));
            }
            ewebsock::WsEvent::Message(message) => {
                read_message(&message, commands, set_style_event)
            }
            ewebsock::WsEvent::Error(e) => info!("socket error: {e}"),
            ewebsock::WsEvent::Closed => info!("socket closed"),
        }
    }
}

fn read_message(
    message: &WsMessage,
    mut commands: Commands,
    mut set_style_event: EventWriter<SetStyle>,
) {
    match message {
        WsMessage::Binary(b) => {
            let message: ToRendererMessage = postcard::from_bytes(b).expect("Cannot deserialize");
            match message {
                ToRendererMessage::CellStyle(styles) => {
                    info!("styles received: {}", styles.len());
                    for style in styles.into_iter() {
                        let cell_id = commands.spawn_empty().add(init_cell).id();
                        set_style_event.send(SetStyle {
                            entity: cell_id,
                            style: CellStyle {
                                text: style.text,
                                text_color: style.text_color,
                                text_size: style.text_size,
                                text_alignment: style.text_alignment,
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
                }
            }
        }
        WsMessage::Text(text) => info!("Received message: {text}"),
        _ => (),
    }
}

fn send_message(
    mut websocket: NonSendMut<Websocket>,
    mut mouse_event: EventReader<MouseButtonInput>,
) {
    for ev in mouse_event.read() {
        if let bevy::input::ButtonState::Pressed = ev.state {
            websocket
                .sender
                .send(WsMessage::Text("Mouse pressed".into()));
        }
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
    cells: Query<&CellMarker>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
        text.sections[3].value = format!("{}", cells.iter().count());
    }
}
