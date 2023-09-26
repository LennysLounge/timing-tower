use bevy_egui::egui::Ui;
use serde::{Deserialize, Serialize};

use crate::{
    editor::properties::{ColorProperty, NumberProperty, TextProperty},
    variable_repo::{StaticNumber, VariableRepo, VariableSource},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Condition {
    pub reference: AnyProperty,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AnyProperty {
    Number(NumberProperty),
    Text(TextProperty),
    Color(ColorProperty),
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            reference: AnyProperty::Number(NumberProperty::Fixed(0.0)),
        }
    }
}

impl Condition {
    pub fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {
        // // Change the reference type to match the output type
        // match (output_type, &self.reference) {
        //     (VariableOutputType::Number, AnyProperty::Number(_)) => (),
        //     (VariableOutputType::Text, AnyProperty::Text(_)) => (),
        //     (VariableOutputType::Color, AnyProperty::Color(_)) => (),
        //     (VariableOutputType::Number, _) => {
        //         self.reference = AnyProperty::Number(NumberProperty::Fixed(0.0))
        //     }
        //     (VariableOutputType::Text, _) => {
        //         self.reference = AnyProperty::Text(TextProperty::Fixed(String::new()))
        //     }
        //     (VariableOutputType::Color, _) => {
        //         self.reference = AnyProperty::Color(ColorProperty::Fixed(Color::WHITE))
        //     }
        // }

        ui.horizontal(|ui| {
            ui.label("Ref:");
            match &mut self.reference {
                AnyProperty::Number(n) => n.editor(ui, vars),
                AnyProperty::Text(t) => t.editor(ui, vars),
                AnyProperty::Color(c) => c.editor(ui, vars),
            }
        });
    }

    pub fn as_variable_source(&self) -> VariableSource {
        VariableSource::Number(Box::new(StaticNumber(12.0)))
    }
}
