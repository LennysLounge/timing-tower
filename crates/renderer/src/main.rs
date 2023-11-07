use bevy::{
    input::mouse::MouseButtonInput,
    prelude::{
        App, Camera2dBundle, ClearColor, Color, EventReader, NonSendMut, Startup, Update, World,
    },
    utils::{info, synccell::SyncCell},
    DefaultPlugins,
};
use ewebsock::{WsMessage, WsReceiver, WsSender};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, send_message)
        .run();
}

struct Websocket {
    sender: SyncCell<WsSender>,
    _receiver: SyncCell<WsReceiver>,
}

fn setup(world: &mut World) {
    info("Connecting to websocket");
    let (mut sender, receiver) = ewebsock::connect("ws://127.0.0.1:8001").unwrap();
    sender.send(ewebsock::WsMessage::Text("Hello Server!".into()));

    world.insert_non_send_resource(Websocket {
        sender: SyncCell::new(sender),
        _receiver: SyncCell::new(receiver),
    });

    world.spawn(Camera2dBundle::default());
}

fn send_message(
    mut websocket: NonSendMut<Websocket>,
    mut mouse_event: EventReader<MouseButtonInput>,
) {
    for ev in mouse_event.read() {
        if let bevy::input::ButtonState::Pressed = ev.state {
            websocket
                .sender
                .get()
                .send(WsMessage::Text("Mouse pressed".into()));
        }
    }
}
