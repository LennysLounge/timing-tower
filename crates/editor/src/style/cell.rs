use backend::style::cell::{Cell, TextAlignment};
use bevy_egui::egui::{ComboBox, Ui};

use crate::{properties::PropertyEditor, reference_store::ReferenceStore};

use super::AttributeEditor;

impl AttributeEditor for Cell {
    fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;

        ui.label("Cell:");
        ui.horizontal(|ui| {
            ui.label("Visible:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.visible, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Text:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.text, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Text color:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.text_color, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Text size:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.text_size, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Text alginment:");
            ComboBox::from_id_source("Text alginment combobox")
                .selected_text(match self.text_alginment {
                    TextAlignment::Left => "Left",
                    TextAlignment::Center => "Center",
                    TextAlignment::Right => "Right",
                })
                .show_ui(ui, |ui| {
                    changed |= ui
                        .selectable_value(&mut self.text_alginment, TextAlignment::Left, "Left")
                        .changed();
                    changed |= ui
                        .selectable_value(&mut self.text_alginment, TextAlignment::Center, "Center")
                        .changed();
                    changed |= ui
                        .selectable_value(&mut self.text_alginment, TextAlignment::Right, "Right")
                        .changed();
                });
        });
        ui.horizontal(|ui| {
            ui.label("Text pos x:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.text_position.x, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Text pos y:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.text_position.y, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Background color:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.color, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Background image:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.image, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Pos x:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.pos.x, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Pos y:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.pos.y, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Pos z:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.pos.z, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Width:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.size.x, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Height:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.size.y, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("Skew:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.skew, asset_repo))
                .changed();
        });
        ui.label("Rounding:");
        ui.horizontal(|ui| {
            ui.label("top left:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.rounding.top_left, asset_repo))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("top right:");
            changed |= ui
                .add(PropertyEditor::new(
                    &mut self.rounding.top_right,
                    asset_repo,
                ))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("bottom right:");
            changed |= ui
                .add(PropertyEditor::new(
                    &mut self.rounding.bot_right,
                    asset_repo,
                ))
                .changed();
        });
        ui.horizontal(|ui| {
            ui.label("bottom left:");
            changed |= ui
                .add(PropertyEditor::new(&mut self.rounding.bot_left, asset_repo))
                .changed();
        });
        changed
    }
}
