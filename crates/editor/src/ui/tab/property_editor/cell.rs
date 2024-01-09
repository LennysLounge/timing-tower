use backend::style::cell::Cell;
use bevy_egui::egui::{Grid, Ui};
use common::communication::TextAlignment;

use crate::{
    command::edit_property::EditResult, reference_store::ReferenceStore, ui::combo_box::LComboBox,
};

use super::property::PropertyEditor;

pub fn cell_property_editor(
    ui: &mut Ui,
    cell: &mut Cell,
    reference_store: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;

    Grid::new("Cell property editor grid").show(ui, |ui| {
        ui.label("Cell:");
        ui.end_row();

        ui.label("Visible:");
        let res = ui.add(PropertyEditor::new(&mut cell.visible, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.strong("Text");
        ui.end_row();

        ui.label("Text:");
        let res = ui.add(PropertyEditor::new(&mut cell.text, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("Text color:");
        let res = ui.add(PropertyEditor::new(&mut cell.text_color, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("Text size:");
        let res = ui.add(PropertyEditor::new(&mut cell.text_size, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("Text alginment:");
        let res = ui.add(
            LComboBox::new(&mut cell.text_alginment)
                .with_id(ui.make_persistent_id("Text alginment combobox"))
                .add_option(TextAlignment::Left, "Left")
                .add_option(TextAlignment::Center, "Center")
                .add_option(TextAlignment::Right, "Right"),
        );
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("Text pos x:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.text_position.x,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("Text pos y:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.text_position.y,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("Font:");
        let res = ui.add(PropertyEditor::new(&mut cell.font, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("Color");
        ui.end_row();

        ui.label("Background color:");
        let res = ui.add(PropertyEditor::new(&mut cell.color, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("Background image:");
        let res = ui.add(PropertyEditor::new(&mut cell.image, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("Position");
        ui.end_row();

        ui.label("Pos x:");
        let res = ui.add(PropertyEditor::new(&mut cell.pos.x, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("Pos y:");
        let res = ui.add(PropertyEditor::new(&mut cell.pos.y, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("Pos z:");
        let res = ui.add(PropertyEditor::new(&mut cell.pos.z, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("Shape");
        ui.end_row();

        ui.label("Width:");
        let res = ui.add(PropertyEditor::new(&mut cell.size.x, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("Height:");
        let res = ui.add(PropertyEditor::new(&mut cell.size.y, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("Skew:");
        let res = ui.add(PropertyEditor::new(&mut cell.skew, reference_store));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();
        ui.label("offset top left:");
        ui.scope(|ui| {
            edit_result |= ui
                .add(PropertyEditor::new(
                    &mut cell.corner_offsets.top_left.x,
                    reference_store,
                ))
                .into();
            edit_result |= ui
                .add(PropertyEditor::new(
                    &mut cell.corner_offsets.top_left.y,
                    reference_store,
                ))
                .into();
        });
        ui.end_row();
        ui.label("offset top right:");
        ui.scope(|ui| {
            edit_result |= ui
                .add(PropertyEditor::new(
                    &mut cell.corner_offsets.top_right.x,
                    reference_store,
                ))
                .into();
            edit_result |= ui
                .add(PropertyEditor::new(
                    &mut cell.corner_offsets.top_right.y,
                    reference_store,
                ))
                .into();
        });
        ui.end_row();
        ui.label("offset bottom left:");
        ui.scope(|ui| {
            edit_result |= ui
                .add(PropertyEditor::new(
                    &mut cell.corner_offsets.bot_left.x,
                    reference_store,
                ))
                .into();
            edit_result |= ui
                .add(PropertyEditor::new(
                    &mut cell.corner_offsets.bot_left.y,
                    reference_store,
                ))
                .into();
        });
        ui.end_row();
        ui.label("offset bottom right:");
        ui.scope(|ui| {
            edit_result |= ui
                .add(PropertyEditor::new(
                    &mut cell.corner_offsets.bot_right.x,
                    reference_store,
                ))
                .into();
            edit_result |= ui
                .add(PropertyEditor::new(
                    &mut cell.corner_offsets.bot_right.y,
                    reference_store,
                ))
                .into();
        });
        ui.end_row();

        ui.label("Rounding:");
        ui.end_row();

        ui.label("top left:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.rounding.top_left,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("top right:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.rounding.top_right,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("bottom right:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.rounding.bot_right,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();

        ui.label("bottom left:");
        let res = ui.add(PropertyEditor::new(
            &mut cell.rounding.bot_left,
            reference_store,
        ));
        if res.changed() {
            edit_result = EditResult::FromId(res.id)
        }
        ui.end_row();
    });
    edit_result
}
