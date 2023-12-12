use backend::savefile::{Savefile, SavefileChanged};
use bevy::{
    app::{First, Plugin},
    asset::AssetPath,
    ecs::{
        event::EventReader,
        system::{Res, ResMut},
    },
    utils::HashMap,
};
use common::asset_path_store::{AssetPathProvider, AssetPathStore};
use tracing::info;
use uuid::Uuid;

pub struct EditorAssetPathStorePlugin;
impl Plugin for EditorAssetPathStorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AssetPathStore::new(EditorAssetPathStore::default()))
            .add_systems(First, savefile_changed);
    }
}

#[derive(Default)]
pub struct EditorAssetPathStore {
    map: HashMap<Uuid, AssetPath<'static>>,
}
impl AssetPathProvider for EditorAssetPathStore {
    fn get(&self, id: &uuid::Uuid) -> Option<&bevy::asset::AssetPath> {
        self.map.get(id)
    }
}

fn savefile_changed(
    _savefile: Res<Savefile>,
    mut asset_path_store: ResMut<AssetPathStore>,
    mut savefile_changed_event: EventReader<SavefileChanged>,
) {
    if savefile_changed_event.is_empty() {
        return;
    }
    savefile_changed_event.clear();

    info!("Reload asset path store");
    *asset_path_store = AssetPathStore::new(EditorAssetPathStore::default());
}
