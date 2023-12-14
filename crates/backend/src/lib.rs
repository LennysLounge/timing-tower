use bevy::app::Plugin;
use savefile::SavefilePlugin;
use style_batch::StyleBatcher;
use value_store::ValueStorePlugin;

pub mod game_sources;
pub mod savefile;
pub mod style;
pub mod style_batch;
pub mod value_store;
pub mod value_types;

pub struct BackendPlugin;
impl Plugin for BackendPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<StyleBatcher>()
            .add_plugins(ValueStorePlugin)
            .add_plugins(SavefilePlugin);
    }
}
