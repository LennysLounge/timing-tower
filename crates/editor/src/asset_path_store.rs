use backend::savefile::{Savefile, SavefileChanged};
use bevy::{
    app::{First, Plugin},
    asset::AssetPath,
    ecs::{
        event::EventReader,
        system::{Res, ResMut, Resource},
    },
    utils::HashMap,
};
use frontend::AssetPathProvider;
use tracing::info;
use uuid::Uuid;

pub struct EditorAssetPathStorePlugin;
impl Plugin for EditorAssetPathStorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(EditorAssetPathStore::default())
            .add_systems(First, savefile_changed);
    }
}

#[derive(Resource, Default)]
pub struct EditorAssetPathStore {
    map: HashMap<Uuid, AssetPath<'static>>,
}
impl AssetPathProvider for EditorAssetPathStore {
    fn get(&self, id: &uuid::Uuid) -> Option<&bevy::asset::AssetPath> {
        self.map.get(id)
    }
}

fn savefile_changed(
    savefile: Res<Savefile>,
    mut store: ResMut<EditorAssetPathStore>,
    mut savefile_changed_event: EventReader<SavefileChanged>,
) {
    if savefile_changed_event.is_empty() {
        return;
    }
    savefile_changed_event.clear();

    info!("Reload asset path store");

    store.map.clear();
    for asset in savefile.style().assets.contained_assets() {
        let asset_path = savefile.base_path().join(&asset.path);
        store.map.insert(
            asset.id,
            AssetPath::from_path(&asset_path)
                .clone_owned()
                .with_source("savefile"),
        );
    }
}
