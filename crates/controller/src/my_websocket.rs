use std::{
    net::{TcpListener, TcpStream},
    thread,
};

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query, ResMut, Resource},
    },
    utils::synccell::SyncCell,
};
use common::communication::{ToControllerMessage, ToRendererMessage};
use tracing::{error, info};
use websocket::{
    server::{InvalidConnection, NoTlsAcceptor, WsServer},
    sync::Client,
    Message, OwnedMessage,
};

pub struct WebsocketPlugin;
impl Plugin for WebsocketPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (accept_new_clients, read_clients));
    }
}

#[derive(Resource)]
struct WebsocketServer {
    server: WsServer<NoTlsAcceptor, TcpListener>,
}

fn setup(mut commands: Commands) {
    thread::spawn(|| {
        info!("Starting web server");
        rouille::start_server("0.0.0.0:8000", move |request| {
            println!("Requested: {}", request.url());
            rouille::match_assets(&request, concat!(file!(), "/../../web"))
        });
    });

    let server = websocket::sync::Server::bind("0.0.0.0:8001").unwrap();
    server.set_nonblocking(true).unwrap();

    commands.insert_resource(WebsocketServer { server });
}

fn accept_new_clients(mut commands: Commands, mut server: ResMut<WebsocketServer>) {
    match server.server.accept() {
        Ok(connection_request) => {
            let mut client = connection_request.accept().unwrap();
            if let Err(e) = client.set_nonblocking(true) {
                error!("Error setting websocket to non blocking: {e}");
                return;
            }
            if let Err(e) = client.set_nodelay(true) {
                error!("Error setting websocket to no delay: {e}");
                return;
            }
            commands.spawn(WebsocketClient {
                client: SyncCell::new(client),
                state: ClientState::Initializing,
            });
        }
        Err(InvalidConnection {
            error: websocket::server::sync::HyperIntoWsError::Io(ref error),
            ..
        }) if error.kind() == std::io::ErrorKind::WouldBlock => (),
        Err(e) => println!("Error accepting a websocket connection: {e:?}"),
    }
}

#[derive(Component)]
pub struct WebsocketClient {
    client: SyncCell<Client<TcpStream>>,
    state: ClientState,
}

impl WebsocketClient {
    pub fn send_message(&mut self, message: ToRendererMessage) {
        let data = postcard::to_allocvec(&message).expect("Cannot convert to postcard");
        if let Err(e) = self.client.get().send_message(&Message::binary(data)) {
            error!("Error trying to send on websocket: {e:?}");
        }
    }
    fn read_message(&mut self, data: Vec<u8>) {
        let message =
            postcard::from_bytes::<ToControllerMessage>(&data).expect("Cannot deserialize");

        match message {
            ToControllerMessage::Opened => {
                self.state = ClientState::Ready;
                //self.state = ClientState::ProcessingAssets;
                //self.send_message(ToRendererMessage::Assets { images: Vec::new() });
            }
            ToControllerMessage::AssetsLoaded => {
                self.state = ClientState::Ready;
            }
            ToControllerMessage::Debug(message) => {
                println!("Message from renderer: {message}");
            }
        }
    }
}

fn read_clients(mut commands: Commands, mut clients: Query<(&mut WebsocketClient, Entity)>) {
    for (mut client, entity) in clients.iter_mut() {
        match client.client.get().recv_message() {
            Ok(m) => match m {
                OwnedMessage::Binary(data) => {
                    client.read_message(data);
                }
                OwnedMessage::Close(_) => {
                    commands.entity(entity).despawn();
                }
                m @ _ => {
                    error!("Unexpected websocket message: {m:?}");
                    commands.entity(entity).despawn();
                }
            },
            Err(websocket::WebSocketError::IoError(ref error))
                if error.kind() == std::io::ErrorKind::WouldBlock =>
            {
                ()
            }
            Err(e) => {
                error!("Error in websocket: {e:?}");
                commands.entity(entity).despawn();
            }
        };
    }
}

enum ClientState {
    Initializing,
    Ready,
}
