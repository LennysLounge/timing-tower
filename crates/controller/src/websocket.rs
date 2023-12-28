use std::net::{TcpListener, TcpStream};

use backend::{
    savefile::{Savefile, SavefileChanged},
    style::definitions::AssetDefinition,
};
use bevy::{
    app::{First, Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    utils::synccell::SyncCell,
};
use common::communication::{ToControllerMessage, ToRendererMessage};
use tracing::{debug, error};
use uuid::Uuid;
use websocket::{
    server::{InvalidConnection, NoTlsAcceptor, WsServer},
    sync::Client,
    Message, OwnedMessage,
};

pub struct WebsocketPlugin;
impl Plugin for WebsocketPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let server = websocket::sync::Server::bind("0.0.0.0:8001").unwrap();
        server.set_nonblocking(true).unwrap();

        app.insert_resource(WebsocketServer { server })
            .add_systems(Update, (accept_new_clients, read_clients))
            .add_systems(First, savefile_changed);
    }
}

#[derive(Resource)]
struct WebsocketServer {
    server: WsServer<NoTlsAcceptor, TcpListener>,
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

#[derive(PartialEq, Eq)]
pub enum ClientState {
    Initializing,
    ProcessingAssets,
    Ready,
    Closed,
}

impl WebsocketClient {
    pub fn send_message(&mut self, message: ToRendererMessage) {
        let data = postcard::to_allocvec(&message).expect("Cannot convert to postcard");
        if let Err(e) = self.client.get().send_message(&Message::binary(data)) {
            error!("Error trying to send on websocket: {e:?}");
        }
    }
    pub fn state(&self) -> &ClientState {
        &self.state
    }
    fn read_websocket(&mut self) -> Option<ToControllerMessage> {
        match self.client.get().recv_message() {
            Ok(OwnedMessage::Binary(data)) => Some(
                postcard::from_bytes::<ToControllerMessage>(&data).expect("Cannot deserialize"),
            ),
            Ok(OwnedMessage::Close(_)) => {
                self.state = ClientState::Closed;
                None
            }
            Ok(m) => {
                error!("Unexpected websocket message: {m:?}");
                self.state = ClientState::Closed;
                None
            }
            Err(websocket::WebSocketError::IoError(ref error))
                if error.kind() == std::io::ErrorKind::WouldBlock =>
            {
                None
            }
            Err(e) => {
                error!("Error in websocket: {e:?}");
                self.state = ClientState::Closed;
                None
            }
        }
    }
}

fn read_clients(
    mut commands: Commands,
    savefile: Res<Savefile>,
    mut clients: Query<(&mut WebsocketClient, Entity)>,
) {
    for (mut client, entity) in clients.iter_mut() {
        match client.read_websocket() {
            Some(ToControllerMessage::Opened) => {
                client.state = ClientState::ProcessingAssets;
                client.send_message(make_assets_message(&*savefile));
            }
            Some(ToControllerMessage::AssetsLoaded) => {
                client.state = ClientState::Ready;
            }
            Some(ToControllerMessage::Debug(m)) => debug!("Websocket message: {m}"),
            None => (),
        }
        if client.state() == &ClientState::Closed {
            commands.entity(entity).despawn();
        }
    }
}

fn make_assets_message(savefile: &Savefile) -> ToRendererMessage {
    let images: Vec<_> = savefile
        .style()
        .assets
        .contained_assets()
        .into_iter()
        .map(|asset| (asset.id, asset_to_uuid_asset_path(asset).into()))
        .collect();
    ToRendererMessage::Init { images }
}

/// Turn an asset into an String representing an `AssetPath`
pub fn asset_to_uuid_asset_path(asset: &AssetDefinition) -> String {
    let parts = asset.path.splitn(2, ".").collect::<Vec<_>>();
    let extension = parts.get(1).expect("Asset paths must have an extension");

    let mut uuid_string = asset
        .id
        .as_hyphenated()
        .encode_lower(&mut Uuid::encode_buffer())
        .to_owned();
    uuid_string.push('.');
    uuid_string.push_str(extension);
    uuid_string
}

fn savefile_changed(
    savefile: Res<Savefile>,
    mut savefile_changed_event: EventReader<SavefileChanged>,
    mut clients: Query<&mut WebsocketClient>,
) {
    if savefile_changed_event.is_empty() {
        return;
    }
    savefile_changed_event.clear();

    for mut client in clients.iter_mut() {
        client.state = ClientState::ProcessingAssets;
        client.send_message(make_assets_message(&*savefile));
    }
}
