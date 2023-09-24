use std::collections::HashMap;

use bevy::prelude::{Color, Resource};
use uuid::Uuid;

use crate::editor::{
    properties::{ColorProperty, NumberProperty, TextProperty},
    variable_element::{Variable, VariableType, VariablesElement},
};

#[derive(Resource)]
pub struct VariableRepo {
    pub vars: HashMap<Uuid, Variable>,
}

impl VariableRepo {
    pub fn reload_repo(&mut self, vars: &VariablesElement) {
        self.vars.clear();

        for var in vars.vars.iter() {
            self.vars.insert(var.id.clone(), var.clone());
        }
    }

    pub fn get_var(&self, id: &Uuid) -> Option<&Variable> {
        self.vars.get(id)
    }

    pub fn get_number(&self, property: &NumberProperty) -> Option<f32> {
        match property {
            NumberProperty::Fixed(n) => Some(*n),
            NumberProperty::Ref(var_ref) => {
                self.get_var(&var_ref.id)
                    .and_then(|var| match &var.var_type {
                        VariableType::Number(n) => Some(*n),
                        VariableType::Text(_) => None,
                        VariableType::Color(_) => None,
                    })
            }
        }
    }

    pub fn get_text(&self, property: &TextProperty) -> Option<String> {
        match property {
            TextProperty::Fixed(n) => Some(n.clone()),
            TextProperty::Ref(id) => self.get_var(&id.id).and_then(|var| match &var.var_type {
                VariableType::Number(n) => Some(format!("{n}")),
                VariableType::Text(s) => Some(s.clone()),
                VariableType::Color(_) => None,
            }),
        }
    }

    pub fn get_color(&self, property: &ColorProperty) -> Option<Color> {
        match property {
            ColorProperty::Fixed(n) => Some(n.clone()),
            ColorProperty::Ref(id) => self.get_var(&id.id).and_then(|var| match &var.var_type {
                VariableType::Number(_) => None,
                VariableType::Text(_) => None,
                VariableType::Color(c) => Some(c.clone()),
            }),
        }
    }
}
