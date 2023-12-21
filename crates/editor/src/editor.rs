pub mod reference_store;

use bevy::prelude::Plugin;

use self::reference_store::ReferenceStorePlugin;

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ReferenceStorePlugin);
    }
}
