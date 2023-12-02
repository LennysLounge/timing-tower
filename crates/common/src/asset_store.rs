use bevy::{
    asset::{Asset, Handle},
    ecs::system::Resource,
    render::texture::Image,
};
use uuid::Uuid;

#[derive(Resource)]
pub struct AssetStore {
    specialized: Box<dyn SpecializedAssetStore + Send + Sync>,
}

impl AssetStore {
    pub fn new(specialized: Box<dyn SpecializedAssetStore + Send + Sync>) -> Self {
        Self { specialized }
    }

    pub fn get<T>(&self, id: &Uuid) -> Option<Handle<T>>
    where
        T: Asset,
        dyn SpecializedAssetStore + Send + Sync: AssetResolver<T>,
    {
        self.specialized.get(id)
    }
}

pub trait SpecializedAssetStore: AssetResolver<Image> + AssetResolver<()> {}

pub trait AssetResolver<T>
where
    T: Asset,
{
    fn get(&self, id: &Uuid) -> Option<Handle<T>>;
}
