use bevy::{app::Plugin, ecs::system::Resource};
use graphic::GraphicPlugin;
use savefile::SavefilePlugin;
use style_batcher::StyleBatcherPlugin;
use unified_sim_model::Adapter;
use value_store::ValueStorePlugin;

pub mod exact_variant;
pub mod game_sources;
pub mod graphic;
pub mod savefile;
pub mod style;
pub mod style_batcher;
pub mod tree_iterator;
pub mod ui;
pub mod value_store;
pub mod value_types;

pub struct BackendPlugin;
impl Plugin for BackendPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(GameAdapterResource { adapter: None })
            .add_plugins(StyleBatcherPlugin)
            .add_plugins(ValueStorePlugin)
            .add_plugins(SavefilePlugin)
            .add_plugins(GraphicPlugin);
    }
}

#[derive(Resource)]
pub struct GameAdapterResource {
    adapter: Option<Adapter>,
}
impl GameAdapterResource {
    pub fn adapter(&self) -> Option<&Adapter> {
        self.adapter.as_ref()
    }
    pub fn adapter_mut(&mut self) -> Option<&mut Adapter> {
        self.adapter.as_mut()
    }
    pub fn set(&mut self, adapter: Adapter) {
        self.adapter = Some(adapter);
    }
}
