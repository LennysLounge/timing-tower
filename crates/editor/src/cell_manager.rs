use backend::style_batcher::{PrepareBatcher, StyleBatcher};
use bevy::prelude::*;
use frontend::{asset_path_store::AssetPathStore, cell::SetStyle, cell_manager::CellManager};

pub struct CellManagerPlugin;
impl Plugin for CellManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, execute_style_commands.after(PrepareBatcher));
    }
}

fn execute_style_commands(
    mut style_batcher: ResMut<StyleBatcher>,
    mut cell_manager: Local<CellManager>,
    set_style: EventWriter<SetStyle>,
    commands: Commands,
    images: ResMut<Assets<Image>>,
    cameras: Query<&mut Transform, With<Camera>>,
    asset_server: Res<AssetServer>,
    asset_path_store: ResMut<AssetPathStore>,
) {
    let style_commands = style_batcher.drain();
    cell_manager.apply_commands(
        style_commands,
        set_style,
        commands,
        images,
        cameras,
        asset_server,
        asset_path_store,
    );
}
