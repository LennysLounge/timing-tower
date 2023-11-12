use bevy::{
    input::mouse::MouseButtonInput,
    prelude::{
        App, Camera2dBundle, ClearColor, Color, EventReader, NonSendMut, Startup, Update, World,
    },
    DefaultPlugins,
};
use ewebsock::{WsMessage, WsReceiver, WsSender};
use tracing::info;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
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

    world.spawn(Camera2dBundle::default());
}

fn read_cell_render(websocket: NonSendMut<Websocket>) {
    if let Some(event) = websocket.receiver.try_recv() {
        match event {
            ewebsock::WsEvent::Opened => info!("socket opened"),
            ewebsock::WsEvent::Message(message) => info!("message: {message:?}"),
            ewebsock::WsEvent::Error(e) => info!("socket error: {e}"),
            ewebsock::WsEvent::Closed => info!("socket closed"),
        }
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
