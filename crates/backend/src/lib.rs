use bevy::{app::Plugin, ecs::system::Resource};
use graphic::GraphicPlugin;
use savefile::SavefilePlugin;
use style_batcher::StyleBatcherPlugin;
use unified_sim_model::Adapter;
use value_store::ValueStorePlugin;

pub mod game_sources;
pub mod graphic;
pub mod savefile;
pub mod style;
pub mod style_batcher;
pub mod tree_iterator;
pub mod value_store;
pub mod value_types;
pub mod exact_variant;
pub mod ui;

pub struct BackendPlugin;
impl Plugin for BackendPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(StyleBatcherPlugin)
            .add_plugins(ValueStorePlugin)
            .add_plugins(SavefilePlugin)
            .add_plugins(GraphicPlugin);
    }
}

#[derive(Resource)]
#[allow(unused)]
pub struct GameAdapterResource {
    pub adapter: Adapter,
}
