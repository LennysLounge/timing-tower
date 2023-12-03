use std::collections::HashMap;

use backend::{style::{folder::Folder, assets::AssetDefinition}, value_types::ValueType};
use bevy::{
    asset::{AssetServer, Handle},
    render::texture::Image,
};
use common::asset_store::{AssetResolver, SpecializedAssetStore};
use uuid::Uuid;

/// The asset store holds a bevy handle to all assets that are
/// defined in the style.
pub struct AssetStoreImpl {
    assets: HashMap<Uuid, Handle<Image>>,
}

impl AssetStoreImpl {
    pub fn new(assets: &Folder<AssetDefinition>, asset_server: &AssetServer) -> Self {
        Self {
            assets: assets
                .all_t()
                .iter()
                .filter_map(|asset_def| match asset_def.value_type {
                    ValueType::Texture => Some((
                        asset_def.id,
                        asset_server.load(format!("savefile://{}", &asset_def.path)),
                    )),
                    _ => unreachable!(),
                })
                .collect(),
        }
    }
}

impl SpecializedAssetStore for AssetStoreImpl {}

impl AssetResolver<()> for AssetStoreImpl {
    fn get(&self, _id: &Uuid) -> Option<Handle<()>> {
        None
    }
}
impl AssetResolver<Image> for AssetStoreImpl {
    fn get(&self, id: &Uuid) -> Option<Handle<Image>> {
        self.assets.get(id).map(|h| h.clone())
    }
}
