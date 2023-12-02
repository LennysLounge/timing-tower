use std::collections::HashMap;

use bevy::{
    asset::{Asset, AssetServer, Handle},
    render::texture::Image,
};
use uuid::Uuid;

use crate::{
    style::{assets::AssetDefinition, folder::Folder},
    value_types::{Texture, ValueType},
};

/// The asset store holds a bevy handle to all assets that are
/// defined in the style.
pub struct AssetStoreImpl {
    assets: HashMap<Uuid, Handle<Image>>,
}

impl AssetStoreImpl {
    fn new(assets: &Folder<AssetDefinition>, asset_server: &AssetServer) -> Self {
        Self {
            assets: assets
                .all_t()
                .iter()
                .filter_map(|asset_def| match asset_def.value_type {
                    ValueType::Texture => Some((asset_def.id, asset_server.load(&asset_def.path))),
                    _ => unreachable!(),
                })
                .collect(),
        }
    }

    fn get<T, U>(&self, value: &T) -> Option<Handle<U>>
    where
        Self: AssetResolver<T, U>,
        U: Asset,
    {
        self.get_asset(value)
    }
}

pub trait AssetResolver<T, U>
where
    U: Asset,
{
    fn get_asset(&self, value: &T) -> Option<Handle<U>>;
}

impl AssetResolver<Texture, Image> for AssetStoreImpl {
    fn get_asset(&self, value: &Texture) -> Option<Handle<Image>> {
        None
        //self.assets.get(value.0)
    }
}
