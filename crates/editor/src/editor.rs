pub mod command;
pub mod reference_store;

use bevy::prelude::Plugin;

use self::{command::UndoRedoManager, reference_store::ReferenceStorePlugin};

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(UndoRedoManager::default())
            .add_plugins(ReferenceStorePlugin);
    }
}
