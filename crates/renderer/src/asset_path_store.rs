use bevy::{app::Plugin, asset::AssetPath, utils::HashMap};
use frontend::asset_path_store::{AssetPathProvider, AssetPathStore};
use uuid::Uuid;

pub struct WebAssetPathStorePlugin;
impl Plugin for WebAssetPathStorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AssetPathStore::new(WebAssetPathStore::default()));
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
