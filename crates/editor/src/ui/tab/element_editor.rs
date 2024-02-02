use backend::{
    style::{elements::Element, NodeMut, StyleDefinition, StyleNode},
    tree_iterator::TreeIteratorMut,
};
use bevy_egui::egui::Ui;
use unified_sim_model::Adapter;
use uuid::Uuid;

use crate::{command::UndoRedoManager, reference_store::ReferenceStore};

pub fn element_editor(
    ui: &mut Ui,
    selected_id: &mut Option<Uuid>,
    secondary_selection: &mut Option<Uuid>,
    style: &mut StyleDefinition,
    _reference_store: &ReferenceStore,
    _undo_redo_manager: &mut UndoRedoManager,
    _game_adapter: &Adapter,
) {
    let Some(selection) = selected_id else {
        return;
    };
    let Some(secondary_selection) = secondary_selection else {
        return;
    };
    style.as_node_mut().search_mut(*selection, |node| {
        if let NodeMut::Component(component) = node {
            component
                .elements
                .search_mut(*secondary_selection, |element| {
                    editor(ui, element);
                });
        }
    });
}

fn editor(ui: &mut Ui, element: &mut Element) {
    match element {
        Element::Cell(_) => ui.label("Cell"),
        Element::ClipArea(_) => ui.label("Clip area"),
    };
}
