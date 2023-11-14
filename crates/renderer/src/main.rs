use bevy::{
    ecs::{
        event::EventWriter,
        query::With,
        system::{Commands, Query},
    },
    input::mouse::MouseButtonInput,
    math::vec3,
    prelude::{
        App, Camera2dBundle, ClearColor, Color, EventReader, NonSendMut, Startup, Update, World,
    },
    transform::components::Transform,
    window::{PrimaryWindow, Window},
    DefaultPlugins,
};
use common::{
    cell::{
        init_cell,
        style::{CellStyle, CellStyleMessage, SetStyle},
        CellPlugin,
    },
    gradient_material::CustomMaterialPlugin,
    websocket::Message,
};
use ewebsock::{WsMessage, WsReceiver, WsSender};
use tracing::info;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_plugins(CellPlugin)
        .add_plugins(CustomMaterialPlugin)
        .add_systems(Startup, (setup, setup_camera))
        .add_systems(Update, (send_message, read_cell_render))
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
    commands: Commands,
    set_style_event: EventWriter<SetStyle>,
    mut websocket: NonSendMut<Websocket>,
) {
    if let Some(event) = websocket.receiver.try_recv() {
        match event {
            ewebsock::WsEvent::Opened => {
                let data = postcard::to_allocvec(&Message::Opened).expect("Cannot serialize");
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
            info!("cell style received");
            let cell_style: CellStyleMessage = postcard::from_bytes(b).expect("Cannot deserialize");

            let cell_id = commands.spawn_empty().add(init_cell).id();
            set_style_event.send(SetStyle {
                entity: cell_id,
                style: CellStyle {
                    text: cell_style.text,
                    text_color: cell_style.color,
                    text_size: cell_style.text_size,
                    text_alignment: cell_style.text_alignment,
                    text_position: cell_style.text_position,
                    color: cell_style.color,
                    texture: None,
                    pos: cell_style.pos,
                    size: cell_style.size,
                    skew: cell_style.skew,
                    visible: cell_style.visible,
                    rounding: cell_style.rounding,
                },
            });
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
