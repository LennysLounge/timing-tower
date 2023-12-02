use bevy::{
    asset::{Asset, Handle},
    ecs::system::Resource,
};
use uuid::Uuid;

#[derive(Resource)]
pub struct AssetStore;

impl AssetStore {
    pub fn get<T>(&self, _id: &Uuid) -> Option<Handle<T>>
    where
        T: Asset,
    {
        None
    }
}
