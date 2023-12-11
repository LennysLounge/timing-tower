use std::mem::discriminant;

use bevy_egui::egui::{Response, Ui};
use unified_sim_model::model::Entry;

use crate::reference_store::{IntoProducerData, ProducerData};
use backend::{
    style::variables::{VariableBehavior, VariableDefinition},
    value_store::{ValueProducer, ValueStore},
};

pub mod condition;
pub mod fixed_value;
pub mod map;

impl IntoProducerData for VariableDefinition {
    fn producer_data(&self) -> ProducerData {
        ProducerData {
            id: self.id,
            name: self.name.clone(),
            value_type: match &self.behavior {
                VariableBehavior::FixedValue(o) => o.output_type(),
                VariableBehavior::Condition(o) => o.output_type(),
                VariableBehavior::Map(o) => o.output_type(),
            },
        }
    }
}

pub struct StaticValueProducer<T>(pub T);
impl<T> ValueProducer<T> for StaticValueProducer<T>
where
    T: Clone,
{
    fn get(&self, _value_store: &ValueStore, _entry: Option<&Entry>) -> Option<T> {
        Some(self.0.clone())
    }
}

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
