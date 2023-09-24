use std::collections::HashMap;

use bevy::prelude::{Color, Resource};
use uuid::Uuid;

use crate::editor::{
    properties::{ColorProperty, NumberProperty, TextProperty},
    variable_element::{VariableDefinition, VariableType, VariablesElement},
};

pub trait NumberSource {
    fn resolve(&self) -> Option<f32>;
}

pub trait TextSource {
    fn resolve(&self) -> Option<String>;
}

pub trait ColorSource {
    fn resolve(&self) -> Option<Color>;
}

pub struct Variable {
    pub def: VariableDefinition,
    pub source: VariableSource,
}

pub enum VariableSource {
    Number(Box<dyn NumberSource + Send + Sync>),
    Text(Box<dyn TextSource + Send + Sync>),
    Color(Box<dyn ColorSource + Send + Sync>),
}

#[derive(Resource)]
pub struct VariableRepo {
    pub vars: HashMap<Uuid, Variable>,
}

pub struct StaticNumber(f32);
impl NumberSource for StaticNumber {
    fn resolve(&self) -> Option<f32> {
        Some(self.0)
    }
}

pub struct StaticText(String);
impl TextSource for StaticText {
    fn resolve(&self) -> Option<String> {
        Some(self.0.clone())
    }
}

pub struct StaticColor(Color);
impl ColorSource for StaticColor {
    fn resolve(&self) -> Option<Color> {
        Some(self.0)
    }
}

impl VariableRepo {
    pub fn reload_repo(&mut self, var_defs: &VariablesElement) {
        self.vars.clear();

        for var_def in var_defs.vars.iter() {
            let var = match &var_def.var_type {
                VariableType::StaticNumber(n) => VariableSource::Number(Box::new(StaticNumber(*n))),
                VariableType::StaticText(t) => {
                    VariableSource::Text(Box::new(StaticText(t.clone())))
                }
                VariableType::StaticColor(c) => VariableSource::Color(Box::new(StaticColor(*c))),
            };
            self.vars.insert(
                var_def.id.clone(),
                Variable {
                    def: var_def.clone(),
                    source: var,
                },
            );
        }
    }

    pub fn get_var_def(&self, id: &Uuid) -> Option<&VariableDefinition> {
        self.vars.get(id).map(|v| &v.def)
    }

    pub fn get_number(&self, property: &NumberProperty) -> Option<f32> {
        match property {
            NumberProperty::Fixed(n) => Some(*n),
            NumberProperty::Ref(id) => self.vars.get(id).and_then(|v| v.source.resolve_number()),
        }
    }

    pub fn get_text(&self, property: &TextProperty) -> Option<String> {
        match property {
            TextProperty::Fixed(n) => Some(n.clone()),
            TextProperty::Ref(id) => self.vars.get(id).and_then(|v| v.source.resolve_text()),
        }
    }

    pub fn get_color(&self, property: &ColorProperty) -> Option<Color> {
        match property {
            ColorProperty::Fixed(n) => Some(n.clone()),
            ColorProperty::Ref(id) => self.vars.get(id).and_then(|v| v.source.resolve_color()),
        }
    }
}

impl VariableSource {
    pub fn resolve_number(&self) -> Option<f32> {
        match self {
            VariableSource::Number(s) => s.resolve(),
            _ => None,
        }
    }

    pub fn resolve_text(&self) -> Option<String> {
        match self {
            VariableSource::Text(s) => s.resolve(),
            VariableSource::Number(s) => s.resolve().map(|n| format!("{n}")),
            _ => None,
        }
    }

    pub fn resolve_color(&self) -> Option<Color> {
        match self {
            VariableSource::Color(s) => s.resolve(),
            _ => None,
        }
    }
}
