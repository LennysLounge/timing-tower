use std::mem::discriminant;

use bevy_egui::egui::{Response, Ui};

pub mod condition;
pub mod fixed_value;
pub mod map;

trait EguiComboBoxExtension {
    /// Shows the combobox with one entry for each variant.
    /// Compares variants based on their discriminants and not PartialEq.
    fn choose<T>(self, ui: &mut Ui, current: &mut T, other: Vec<(T, &str)>) -> Response;
}
impl EguiComboBoxExtension for bevy_egui::egui::ComboBox {
    fn choose<T>(self, ui: &mut Ui, current: &mut T, other: Vec<(T, &str)>) -> Response {
        let mut changed = false;
        let mut res = self
            .selected_text({
                other
                    .iter()
                    .find_map(|(other, name)| {
                        (discriminant(current) == discriminant(other)).then_some(*name)
                    })
                    .unwrap_or("Not Found")
            })
            .show_ui(ui, |ui| {
                for (other, name) in other {
                    let is_same = discriminant(current) == discriminant(&other);
                    if ui.selectable_label(is_same, name).clicked() && !is_same {
                        *current = other;
                        changed = true;
                    }
                }
            })
            .response;
        if changed {
            res.mark_changed();
        }
        res
    }
}
