use bevy_egui::egui::Ui;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    editor::style_elements::reference_editor,
    variable_repo::{Reference, StaticNumber, ValueType, VariableRepo, VariableSource},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Condition {
    left: Reference,
    right: Reference,
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            left: Reference {
                value_type: ValueType::Number,
                key: Uuid::nil(),
            },
            right: Reference {
                value_type: ValueType::Number,
                key: Uuid::nil(),
            },
        }
    }
}

impl Condition {
    pub fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        let new_ref = reference_editor(ui, vars, &mut self.left, |v| match v.value_type {
            ValueType::Number => true,
            ValueType::Text => true,
            ValueType::Color => false,
        });
        if let Some(reference) = new_ref {
            self.left = reference;
        }

        match self.left.value_type {
            ValueType::Number => ui.label("Number reference"),
            ValueType::Text => ui.label("Text reference"),
            ValueType::Color => ui.label("Color reference"),
        };

        // Make right side the same type as left
        if !self.right.value_type.can_cast_to(&self.left.value_type) {
            self.right = Reference {
                value_type: self.left.value_type.clone(),
                key: Uuid::nil(),
            };
        }

        // show select for right side
        let new_ref = reference_editor(ui, vars, &mut self.right, |v| {
            v.value_type.can_cast_to(&self.left.value_type)
        });
        if let Some(reference) = new_ref {
            self.right = reference;
        };
    }

    pub fn as_variable_source(&self) -> VariableSource {
        VariableSource::Number(Box::new(StaticNumber(0.0)))
    }
}
