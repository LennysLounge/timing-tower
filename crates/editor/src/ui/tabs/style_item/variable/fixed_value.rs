use backend::{
    style::variables::fixed_value::FixedValue,
    value_types::{Boolean, Number, Text, Tint},
};
use bevy_egui::egui::{ComboBox, DragValue, Ui};

use crate::{
    command::edit_property::EditResult,
    reference_store::ReferenceStore,
    ui::{combo_box::LComboBox, tabs::secondary_editor::ui_split},
};

pub fn property_editor(
    ui: &mut Ui,
    value: &mut FixedValue,
    _reference_store: &ReferenceStore,
) -> EditResult {
    let mut edit_result = EditResult::None;

    ui_split(ui, "Output type", |ui| {
        edit_result |= ui
            .add(
                LComboBox::new_comparable(value, |a, b| {
                    std::mem::discriminant(a) == std::mem::discriminant(b)
                })
                .add_option(FixedValue::Number(Number::default()), "Number")
                .add_option(FixedValue::Text(Text::default()), "Text")
                .add_option(FixedValue::Tint(Tint::default()), "Color")
                .add_option(FixedValue::Boolean(Boolean::default()), "Yes/No"),
            )
            .into();
    });

    ui.separator();

    match value {
        FixedValue::Number(Number(number)) => ui_split(ui, "Value", |ui| {
            edit_result |= ui.add(DragValue::new(number)).into();
        }),
        FixedValue::Text(Text(text)) => {
            ui_split(ui, "Text", |ui| {
                edit_result |= ui.text_edit_singleline(text).into();
            });
        }
        FixedValue::Tint(Tint(tint)) => {
            ui_split(ui, "Color", |ui| {
                let mut color_local = tint.as_rgba_f32();
                edit_result |= ui
                    .color_edit_button_rgba_unmultiplied(&mut color_local)
                    .into();
                *tint = color_local.into();
            });
        }
        FixedValue::Boolean(Boolean(boolean)) => {
            ui_split(ui, "Value", |ui| {
                ComboBox::from_id_source(ui.next_auto_id())
                    .width(ui.available_width())
                    .selected_text(match boolean {
                        true => "Yes",
                        false => "No",
                    })
                    .show_ui(ui, |ui| {
                        edit_result |= ui.selectable_value(boolean, true, "Yes").into();
                        edit_result |= ui.selectable_value(boolean, false, "No").into();
                    });
            });
        }
    }
    edit_result
}
