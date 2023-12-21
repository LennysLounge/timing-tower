use backend::style::cell::{Cell, TextAlignment};
use bevy_egui::egui::{ComboBox, Ui};

use crate::editor::{command::edit_property::EditResult, reference_store::ReferenceStore};

use super::property::PropertyEditor;

pub fn cell_property_editor(
    ui: &mut Ui,
    cell: &mut Cell,
    reference_store: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;

    ui.label("Cell:");
    ui.horizontal(|ui| {
        ui.label("Visible:");
        let res = ui.add(PropertyEditor::new(&mut cell.visible, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Text:");
        let res = ui.add(PropertyEditor::new(&mut cell.text, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Text color:");
        let res = ui.add(PropertyEditor::new(&mut cell.text_color, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Text size:");
        let res = ui.add(PropertyEditor::new(&mut cell.text_size, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Text alginment:");
        ComboBox::from_id_source("Text alginment combobox")
            .selected_text(match cell.text_alginment {
                TextAlignment::Left => "Left",
                TextAlignment::Center => "Center",
                TextAlignment::Right => "Right",
            })
            .show_ui(ui, |ui| {
                let res =
                    ui.selectable_value(&mut cell.text_alginment, TextAlignment::Left, "Left");
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
                let res =
                    ui.selectable_value(&mut cell.text_alginment, TextAlignment::Center, "Center");
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
                let res =
                    ui.selectable_value(&mut cell.text_alginment, TextAlignment::Right, "Right");
                if res.changed() {
                    edit_result = EditResult::FromId(res.id)
                }
            });
    });
    ui.horizontal(|ui| {
        ui.label("Text pos x:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.text_position.x,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Text pos y:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.text_position.y,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Background color:");
        let res = ui.add(PropertyEditor::new(&mut cell.color, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Background image:");
        let res = ui.add(PropertyEditor::new(&mut cell.image, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Pos x:");
        let res = ui.add(PropertyEditor::new(&mut cell.pos.x, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Pos y:");
        let res = ui.add(PropertyEditor::new(&mut cell.pos.y, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Pos z:");
        let res = ui.add(PropertyEditor::new(&mut cell.pos.z, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Width:");
        let res = ui.add(PropertyEditor::new(&mut cell.size.x, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Height:");
        let res = ui.add(PropertyEditor::new(&mut cell.size.y, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("Skew:");
        let res = ui.add(PropertyEditor::new(&mut cell.skew, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.label("Rounding:");
    ui.horizontal(|ui| {
        ui.label("top left:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.rounding.top_left,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("top right:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.rounding.top_right,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("bottom right:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.rounding.bot_right,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    ui.horizontal(|ui| {
        ui.label("bottom left:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.rounding.bot_left,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
    });
    edit_result
}
