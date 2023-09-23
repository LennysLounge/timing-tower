use std::collections::HashMap;

use bevy::prelude::Color;

use super::style_elements::{ColorProperty, NumberProperty, TextProperty};

pub enum Variable {
    Number(f32),
    Text(String),
    Color(Color),
}

pub struct Scope {
    pub vars: HashMap<String, Variable>,
}

impl Scope {
    pub fn get_number(&self, property: &NumberProperty) -> Option<f32> {
        match property {
            NumberProperty::Fixed(n) => Some(*n),
            NumberProperty::Ref(name) => self.vars.get(name).and_then(|s| match s {
                Variable::Number(n) => Some(*n),
                Variable::Text(_) => None,
                Variable::Color(_) => None,
            }),
        }
    }

    pub fn get_text(&self, property: &TextProperty) -> Option<String> {
        match property {
            TextProperty::Fixed(n) => Some(n.clone()),
            TextProperty::Ref(name) => self.vars.get(name).and_then(|s| match s {
                Variable::Number(n) => Some(format!("{n}")),
                Variable::Text(s) => Some(s.clone()),
                Variable::Color(_) => None,
            }),
        }
    }

    pub fn get_color(&self, property: &ColorProperty) -> Option<Color> {
        match property {
            ColorProperty::Fixed(n) => Some(n.clone()),
            ColorProperty::Ref(name) => self.vars.get(name).and_then(|s| match s {
                Variable::Number(_) => None,
                Variable::Text(_) => None,
                Variable::Color(c) => Some(c.clone()),
            }),
        }
    }
}
