use backend::{
    style::{elements::GraphicItem, StyleItemMut, StyleDefinition, StyleItem},
    tree_iterator::TreeIteratorMut,
};
use bevy_egui::egui::Ui;
use unified_sim_model::Adapter;
use uuid::Uuid;

use crate::{
    command::{
        edit_property::{EditProperty, EditResult},
        UndoRedoManager,
    },
    reference_store::ReferenceStore,
};

use super::property_editor::cell;

pub fn element_editor(
    ui: &mut Ui,
    selected_id: &mut Option<Uuid>,
    secondary_selection: &mut Option<Uuid>,
    style: &mut StyleDefinition,
    reference_store: &ReferenceStore,
    undo_redo_manager: &mut UndoRedoManager,
    _game_adapter: &Adapter,
) {
    let Some(selection) = selected_id else {
        return;
    };
    let Some(secondary_selection) = secondary_selection else {
        return;
    };
    style.as_mut().search_mut(*selection, |node| {
        if let StyleItemMut::Graphic(component) = node {
            let edit_result = component
                .items
                .search_mut(*secondary_selection, |element| {
                    editor(ui, element, reference_store)
                });
            if let Some(EditResult::FromId(widget_id)) = edit_result {
                undo_redo_manager.queue(EditProperty::new(
                    component.id,
                    component.clone(),
                    widget_id,
                ));
            }
        }
    });
}

fn editor(ui: &mut Ui, element: &mut GraphicItem, reference_store: &ReferenceStore) -> EditResult {
    match element {
        GraphicItem::Cell(cell) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut cell.name).into();
            ui.separator();
            edit_result |= cell::cell_property_editor(ui, &mut cell.cell, reference_store).into();
            edit_result
        }
        GraphicItem::ClipArea(clip_area) => {
            let mut edit_result = EditResult::None;

            ui.label("Name:");
            edit_result |= ui.text_edit_singleline(&mut clip_area.name).into();
            ui.separator();
            edit_result |= cell::clip_area_editor(ui, &mut clip_area.clip_area, reference_store);
            edit_result
        }
    }
}
