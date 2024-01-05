use bevy::{asset::AssetPath, prelude::*};
use cell::CellPlugin;
use cell_material::CellMaterialPlugin;
use uuid::Uuid;

pub mod cell;
pub mod cell_manager;
pub mod cell_material;

pub struct FrontendPlugin;
impl Plugin for FrontendPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(CellPlugin).add_plugins(CellMaterialPlugin);
    }
}

pub trait AssetPathProvider {
    fn get(&self, id: &Uuid) -> Option<&AssetPath>;
}
