use std::collections::HashMap;

use bevy::{asset::AssetPath, prelude::*};
use common::communication::{ToControllerMessage, ToRendererMessage};
use frontend::AssetPathProvider;
use uuid::Uuid;

use crate::websocket::{ReceivedMessage, SendMessage};

pub struct WebAssetPathStorePlugin;
impl Plugin for WebAssetPathStorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(WebAssetPathStore::default())
            .add_systems(Update, assets_received);
    }
}

#[derive(Resource, Default)]
pub struct WebAssetPathStore {
    map: HashMap<Uuid, AssetPath<'static>>,
}
impl AssetPathProvider for WebAssetPathStore {
    fn get(&self, id: &uuid::Uuid) -> Option<&bevy::asset::AssetPath> {
        self.map.get(id)
    }
}

fn assets_received(
    mut store: ResMut<WebAssetPathStore>,
    mut received_messages: EventReader<ReceivedMessage>,
    mut send_message: EventWriter<SendMessage>,
) {
    let Some(images) = received_messages
        .read()
        .filter_map(|ReceivedMessage { message }| match message {
            ToRendererMessage::Init { images, .. } => Some(images),
            _ => None,
        })
        .last()
    else {
        return;
    };

    store.map.clear();
    for (id, path) in images.into_iter() {
        store.map.insert(*id, AssetPath::from(path).clone_owned());
    }

    send_message.send(SendMessage {
        message: ToControllerMessage::AssetsLoaded,
    });
}
