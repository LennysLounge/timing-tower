use backend::{
    style::{
        elements::GraphicItem, graphic::GraphicDefinition, StyleDefinition, StyleItem, StyleItemMut,
    },
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

use super::property_editor::{cell, property::PropertyEditor};

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
        if let StyleItemMut::Graphic(graphic) = node {
            let mut edit_result = EditResult::None;
            if *secondary_selection == graphic.id {
                edit_result |= graphic_editor(ui, graphic, reference_store);
            } else {
                edit_result |= graphic
                    .items
                    .search_mut(*secondary_selection, |element| {
                        editor(ui, element, reference_store)
                    })
                    .unwrap_or(EditResult::None);
            }
            if let EditResult::FromId(widget_id) = edit_result {
                undo_redo_manager.queue(EditProperty::new(graphic.id, graphic.clone(), widget_id));
            }
        }
    });
}

fn graphic_editor(
    ui: &mut Ui,
    graphic: &mut GraphicDefinition,
    reference_store: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;
    ui.label("Position:");
    ui.horizontal(|ui| {
        ui.label("X:");
        edit_result |= ui
            .add(PropertyEditor::new(
                &mut graphic.items.position.x,
                reference_store,
            ))
            .into();
    });
    ui.horizontal(|ui| {
        ui.label("Y:");
        edit_result |= ui
            .add(PropertyEditor::new(
                &mut graphic.items.position.y,
                reference_store,
            ))
            .into();
    });
    edit_result
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
