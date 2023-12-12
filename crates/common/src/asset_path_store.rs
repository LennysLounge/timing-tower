use bevy::{
    app::{Plugin, Startup},
    asset::AssetPath,
    ecs::system::{Res, Resource},
};
use uuid::Uuid;

pub struct AssetPathStorePlugin;
impl Plugin for AssetPathStorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, |store: Option<Res<AssetPathStore>>| {
            if store.is_none() {
                panic!("No implementation of `AssetPathStore` was registered");
            }
        });
    }
}

pub trait AssetPathProvider {
    fn get(&self, id: &Uuid) -> Option<&AssetPath>;
}

#[derive(Resource)]
pub struct AssetPathStore {
    inner: Box<dyn AssetPathProvider + Sync + Send>,
}
impl AssetPathStore {
    pub fn new(provider: impl AssetPathProvider + Send + Sync + 'static) -> Self {
        Self {
            inner: Box::new(provider),
        }
    }
}
impl AssetPathProvider for AssetPathStore {
    fn get(&self, id: &Uuid) -> Option<&AssetPath> {
        self.inner.get(id)
    }
}
