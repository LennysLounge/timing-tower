use bevy::{
    app::{First, Last, Plugin, Startup},
    ecs::{
        event::{Event, EventReader, EventWriter},
        schedule::IntoSystemConfigs,
        system::NonSendMut,
        world::World,
    },
    time::TimeSystem,
};
use common::communication::{ToControllerMessage, ToRendererMessage};
use ewebsock::{WsMessage, WsReceiver, WsSender};
use tracing::info;

pub struct WebsocketPlugin;
impl Plugin for WebsocketPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SendMessage>()
            .add_event::<ReceivedMessages>()
            .add_systems(Startup, open_websocket)
            .add_systems(First, read_websocket.after(TimeSystem))
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
pub struct ReceivedMessages {
    pub messages: Vec<ToRendererMessage>,
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
    mut received_messages: EventWriter<ReceivedMessages>,
) {
    let mut messages = Vec::new();
    while let Some(event) = websocket.receiver.try_recv() {
        match event {
            ewebsock::WsEvent::Opened => {
                send_message.send(SendMessage {
                    message: ToControllerMessage::Opened,
                });
                websocket.connected = true;
            }
            ewebsock::WsEvent::Message(message) => {
                if let Some(message) = read_message(&message) {
                    messages.push(message);
                }
            }
            ewebsock::WsEvent::Error(e) => info!("socket error: {e}"),
            ewebsock::WsEvent::Closed => {
                info!("socket closed");
                websocket.connected = false;
            }
        }
    }

    if !websocket.connected {
        return;
    }

    //Remove all but the last CellStyle message
    let last_cell_style_index = messages
        .iter()
        .rev()
        .position(|m| matches!(m, ToRendererMessage::CellStyle(_)));
    let mut index = 0;
    messages.retain(|m| {
        let retain = if matches!(m, ToRendererMessage::CellStyle(_)) {
            last_cell_style_index.is_some_and(|keep_index| index == keep_index)
        } else {
            true
        };
        index += 1;
        retain
    });

    if !messages.is_empty() {
        received_messages.send(ReceivedMessages { messages });
    }
}

fn read_message(message: &WsMessage) -> Option<ToRendererMessage> {
    match message {
        WsMessage::Binary(b) => {
            Some(postcard::from_bytes::<ToRendererMessage>(b).expect("Cannot deserialize"))
        }
        WsMessage::Text(text) => {
            info!("Received message: {text}");
            None
        }
        _ => None,
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
