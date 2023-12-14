use bevy::{
    app::{First, Last, Plugin, Startup},
    ecs::{
        event::{Event, EventReader, EventWriter},
        system::NonSendMut,
        world::World,
    },
};
use common::communication::{ToControllerMessage, ToRendererMessage};
use ewebsock::{WsMessage, WsReceiver, WsSender};
use tracing::info;

pub struct WebsocketPlugin;
impl Plugin for WebsocketPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SendMessage>()
            .add_event::<ReceivedMessage>()
            .add_systems(Startup, open_websocket)
            .add_systems(First, read_websocket)
            .add_systems(Last, send_messages);
    }
}

/// Event for sending messages to the controller.
#[derive(Event)]
pub struct SendMessage {
    pub message: ToControllerMessage,
}

/// Event contains all messages that were received from the controller.
#[derive(Event)]
pub struct ReceivedMessage {
    pub message: ToRendererMessage,
}

struct Websocket {
    sender: WsSender,
    receiver: WsReceiver,
    connected: bool,
}

fn open_websocket(world: &mut World) {
    info!("Connecting to websocket");
    let (sender, receiver) = ewebsock::connect("ws://127.0.0.1:8001").unwrap();

    world.insert_non_send_resource(Websocket {
        sender: sender,
        receiver: receiver,
        connected: false,
    });
}

fn read_websocket(
    mut websocket: NonSendMut<Websocket>,
    mut send_message: EventWriter<SendMessage>,
    mut received_messages: EventWriter<ReceivedMessage>,
) {
    while let Some(event) = websocket.receiver.try_recv() {
        match event {
            ewebsock::WsEvent::Opened => {
                send_message.send(SendMessage {
                    message: ToControllerMessage::Opened,
                });
                websocket.connected = true;
            }
            ewebsock::WsEvent::Message(WsMessage::Binary(b)) => {
                received_messages.send(ReceivedMessage {
                    message: postcard::from_bytes::<ToRendererMessage>(b.as_slice())
                        .expect("Cannot deserialize"),
                });
            }
            ewebsock::WsEvent::Message(message) => {
                info!("Unexpected message on websocket: {message:?}");
            }
            ewebsock::WsEvent::Error(e) => info!("socket error: {e}"),
            ewebsock::WsEvent::Closed => {
                info!("socket closed");
                websocket.connected = false;
            }
        }
    }
}

fn send_messages(
    mut websocket: NonSendMut<Websocket>,
    mut send_messages: EventReader<SendMessage>,
) {
    for message in send_messages.read() {
        let data = postcard::to_allocvec(&message.message).expect("Cannot serialize");
        websocket.sender.send(WsMessage::Binary(data));
    }
}
