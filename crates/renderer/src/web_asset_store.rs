use bevy::render::texture::Image;
use common::asset_store::{AssetResolver, SpecializedAssetStore};

pub struct WebAssetStore;

impl SpecializedAssetStore for WebAssetStore {}

impl AssetResolver<()> for WebAssetStore {
    fn get(&self, _id: &bevy::utils::Uuid) -> Option<bevy::prelude::Handle<()>> {
        None
    }
}

impl AssetResolver<Image> for WebAssetStore {
    fn get(&self, _id: &bevy::utils::Uuid) -> Option<bevy::prelude::Handle<Image>> {
        None
    }
}
