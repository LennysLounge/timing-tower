use std::collections::HashMap;

use bevy::{
    app::{Plugin, Update},
    asset::AssetPath,
    ecs::{
        event::{EventReader, EventWriter},
        system::ResMut,
    },
};
use common::communication::{ToControllerMessage, ToRendererMessage};
use frontend::asset_path_store::{AssetPathProvider, AssetPathStore};
use uuid::Uuid;

use crate::websocket::{ReceivedMessage, SendMessage};

pub struct WebAssetPathStorePlugin;
impl Plugin for WebAssetPathStorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AssetPathStore::new(WebAssetPathStore::default()))
            .add_systems(Update, assets_received);
    }
}

#[derive(Default)]
pub struct WebAssetPathStore {
    map: HashMap<Uuid, AssetPath<'static>>,
}
impl AssetPathProvider for WebAssetPathStore {
    fn get(&self, id: &uuid::Uuid) -> Option<&bevy::asset::AssetPath> {
        self.map.get(id)
    }
}

fn assets_received(
    mut asset_path_store: ResMut<AssetPathStore>,
    mut received_messages: EventReader<ReceivedMessage>,
    mut send_message: EventWriter<SendMessage>,
) {
    let Some(images) = received_messages
        .read()
        .filter_map(|ReceivedMessage { message }| match message {
            ToRendererMessage::Assets { images } => Some(images),
            _ => None,
        })
        .last()
    else {
        return;
    };

    let mut map = HashMap::new();
    for (id, path) in images.into_iter() {
        map.insert(*id, AssetPath::from(path).clone_owned());
    }

    *asset_path_store = AssetPathStore::new(WebAssetPathStore { map });

    send_message.send(SendMessage {
        message: ToControllerMessage::AssetsLoaded,
    });
}
