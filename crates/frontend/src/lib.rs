use asset_path_store::AssetPathStorePlugin;
use bevy::app::Plugin;
use cell::CellPlugin;
use cell_material::CellMaterialPlugin;

pub mod asset_path_store;
pub mod cell;
pub mod cell_material;
pub mod gradient_material;

pub struct FrontendPlugin;
impl Plugin for FrontendPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(CellPlugin)
            .add_plugins(CellMaterialPlugin)
            .add_plugins(AssetPathStorePlugin);
    }
}
