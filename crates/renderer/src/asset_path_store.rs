use std::collections::HashMap;

use bevy::{
    app::{Plugin, Update},
    asset::AssetPath,
    ecs::{event::EventReader, system::ResMut},
};
use common::communication::ToRendererMessage;
use frontend::asset_path_store::{AssetPathProvider, AssetPathStore};
use uuid::Uuid;

use crate::websocket::ReceivedMessages;

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
    mut received_messages: EventReader<ReceivedMessages>,
) {
    let Some(images) = received_messages
        .read()
        .flat_map(|received_message| received_message.messages.iter())
        .filter_map(|m| match m {
            ToRendererMessage::Assets { images } => Some(images),
            _ => None,
        })
        .last()
    else {
        return;
    };

    let mut map = HashMap::new();
    for (id, path_str) in images.into_iter() {
        let asset_path: AssetPath = path_str.clone().into();
        map.insert(*id, asset_path);
    }

    *asset_path_store = AssetPathStore::new(WebAssetPathStore { map });
}
