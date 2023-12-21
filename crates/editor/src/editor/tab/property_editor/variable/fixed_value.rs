use backend::{
    style::variables::fixed_value::FixedValue,
    value_types::{Boolean, Number, Text, Tint},
};
use bevy_egui::egui::{ComboBox, DragValue, Ui};

use crate::editor::reference_store::ReferenceStore;

pub fn property_editor(
    ui: &mut Ui,
    value: &mut FixedValue,
    _reference_store: &ReferenceStore,
) -> bool {
    let mut changed = false;

    ui.horizontal(|ui| {
        ui.label("Type:");
        ComboBox::new(ui.next_auto_id(), "")
            .selected_text(match value {
                FixedValue::Number(_) => "Number",
                FixedValue::Text(_) => "Text",
                FixedValue::Tint(_) => "Color",
                FixedValue::Boolean(_) => "Yes/No",
            })
            .show_ui(ui, |ui| {
                let is_number = matches!(value, FixedValue::Number(_));
                if ui.selectable_label(is_number, "Number").clicked() && !is_number {
                    *value = FixedValue::Number(Number::default());
                    changed |= true;
                }
                let is_text = matches!(value, FixedValue::Text(_));
                if ui.selectable_label(is_text, "Text").clicked() && !is_text {
                    *value = FixedValue::Text(Text::default());
                    changed |= true;
                }
                let is_color = matches!(value, FixedValue::Tint(_));
                if ui.selectable_label(is_color, "Color").clicked() && !is_color {
                    *value = FixedValue::Tint(Tint::default());
                    changed |= true;
                }
                let is_boolean = matches!(value, FixedValue::Boolean(_));
                if ui.selectable_label(is_boolean, "Yes/No").clicked() && !is_boolean {
                    *value = FixedValue::Boolean(Boolean::default());
                    changed |= true;
                }
            });
    });

    match value {
        FixedValue::Number(Number(number)) => {
            ui.horizontal(|ui| {
                ui.label("Value");
                changed |= ui.add(DragValue::new(number)).changed();
            });
        }
        FixedValue::Text(Text(text)) => {
            ui.horizontal(|ui| {
                ui.label("Text");
                changed |= ui.text_edit_singleline(text).changed();
            });
        }
        FixedValue::Tint(Tint(tint)) => {
            ui.horizontal(|ui| {
                ui.label("Color:");
                let mut color_local = tint.as_rgba_f32();
                changed |= ui
                    .color_edit_button_rgba_unmultiplied(&mut color_local)
                    .changed();
                *tint = color_local.into();
            });
        }
        FixedValue::Boolean(Boolean(boolean)) => {
            ui.horizontal(|ui| {
                ui.label("Value:");
                ComboBox::from_id_source(ui.next_auto_id())
                    .width(50.0)
                    .selected_text(match boolean {
                        true => "Yes",
                        false => "No",
                    })
                    .show_ui(ui, |ui| {
                        changed |= ui.selectable_value(boolean, true, "Yes").changed();
                        changed |= ui.selectable_value(boolean, false, "No").changed();
                    });
            });
        }
    }
    changed
}
