use std::collections::HashMap;

use bevy::prelude::{Color, Resource};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::{uuid, Uuid};

use crate::editor::{
    properties::{ColorProperty, NumberProperty, TextProperty},
    variable_element::VariablesElement,
};

pub trait NumberSource {
    fn resolve(&self, vars: &VariableRepo, entry: Option<&Entry>) -> Option<f32>;
}

pub trait TextSource {
    fn resolve(&self, vars: &VariableRepo, entry: Option<&Entry>) -> Option<String>;
}

pub trait ColorSource {
    fn resolve(&self, vars: &VariableRepo, entry: Option<&Entry>) -> Option<Color>;
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, Default)]
pub enum ValueType {
    #[default]
    Number,
    Text,
    Color,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Reference {
    pub value_type: ValueType,
    pub key: Uuid,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct VariableId {
    pub id: Uuid,
    pub name: String,
    pub value_type: ValueType,
}

pub struct Variable {
    pub id: VariableId,
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

impl VariableRepo {
    pub fn reload_repo(&mut self, var_defs: &VariablesElement) {
        self.vars.clear();

        generate_game_sources(&mut self.vars);

        for var_def in var_defs.vars.iter() {
            self.vars.insert(
                var_def.get_id().id.clone(),
                Variable {
                    id: var_def.get_id().clone(),
                    source: var_def.as_variable_source(),
                },
            );
        }
    }

    pub fn get_var_def(&self, reference: &Reference) -> Option<&VariableId> {
        self.vars.get(&reference.key).map(|v| &v.id)
    }

    pub fn get_number(&self, reference: &Reference, entry: Option<&Entry>) -> Option<f32> {
        self.vars
            .get(&reference.key)
            .and_then(|v| v.source.resolve_number(self, entry))
    }

    pub fn get_text(&self, reference: &Reference, entry: Option<&Entry>) -> Option<String> {
        self.vars
            .get(&reference.key)
            .and_then(|v| v.source.resolve_text(self, entry))
    }

    pub fn get_color(&self, reference: &Reference, entry: Option<&Entry>) -> Option<Color> {
        self.vars
            .get(&reference.key)
            .and_then(|v| v.source.resolve_color(self, entry))
    }

    pub fn get_number_property(
        &self,
        property: &NumberProperty,
        entry: Option<&Entry>,
    ) -> Option<f32> {
        match property {
            NumberProperty::Fixed(n) => Some(*n),
            NumberProperty::Ref(reference) => self.get_number(reference, entry),
        }
    }

    pub fn get_text_property(
        &self,
        property: &TextProperty,
        entry: Option<&Entry>,
    ) -> Option<String> {
        match property {
            TextProperty::Fixed(n) => Some(n.clone()),
            TextProperty::Ref(reference) => self.get_text(reference, entry),
        }
    }

    pub fn get_color_property(
        &self,
        property: &ColorProperty,
        entry: Option<&Entry>,
    ) -> Option<Color> {
        match property {
            ColorProperty::Fixed(n) => Some(n.clone()),
            ColorProperty::Ref(reference) => self.get_color(reference, entry),
        }
    }
}

impl VariableSource {
    pub fn resolve_number(&self, vars: &VariableRepo, entry: Option<&Entry>) -> Option<f32> {
        match self {
            VariableSource::Number(s) => s.resolve(vars, entry),
            _ => None,
        }
    }

    pub fn resolve_text(&self, vars: &VariableRepo, entry: Option<&Entry>) -> Option<String> {
        match self {
            VariableSource::Text(s) => s.resolve(vars, entry),
            VariableSource::Number(s) => s.resolve(vars, entry).map(|n| format!("{n}")),
            _ => None,
        }
    }

    pub fn resolve_color(&self, vars: &VariableRepo, entry: Option<&Entry>) -> Option<Color> {
        match self {
            VariableSource::Color(s) => s.resolve(vars, entry),
            _ => None,
        }
    }
}

impl Variable {
    pub fn get_ref(&self) -> Reference {
        Reference {
            value_type: self.id.value_type.clone(),
            key: self.id.id.clone(),
        }
    }
}

impl ValueType {
    pub fn can_cast_to(&self, other: &ValueType) -> bool {
        match (self, other) {
            (ref a, ref b) if a == b => true,
            (ValueType::Number, ValueType::Text) => true,
            _ => false,
        }
    }
}

/*
*-------------------------------------------------------------------------------------------------------

*/

pub struct StaticNumber(pub f32);
impl NumberSource for StaticNumber {
    fn resolve(&self, _vars: &VariableRepo, _entry: Option<&Entry>) -> Option<f32> {
        Some(self.0)
    }
}

pub struct StaticText(pub String);
impl TextSource for StaticText {
    fn resolve(&self, _vars: &VariableRepo, _entry: Option<&Entry>) -> Option<String> {
        Some(self.0.clone())
    }
}

pub struct StaticColor(pub Color);
impl ColorSource for StaticColor {
    fn resolve(&self, _vars: &VariableRepo, _entry: Option<&Entry>) -> Option<Color> {
        Some(self.0)
    }
}

pub struct GameNumberSource {
    extractor: fn(Option<&Entry>) -> Option<f32>,
}
impl NumberSource for GameNumberSource {
    fn resolve(&self, _vars: &VariableRepo, entry: Option<&Entry>) -> Option<f32> {
        (self.extractor)(entry)
    }
}

pub struct GameTextSource {
    extractor: fn(Option<&Entry>) -> Option<String>,
}
impl TextSource for GameTextSource {
    fn resolve(&self, _vars: &VariableRepo, entry: Option<&Entry>) -> Option<String> {
        (self.extractor)(entry)
    }
}

fn generate_game_sources(vars: &mut HashMap<Uuid, Variable>) {
    let mut make_source =
        |uuid: Uuid, name: &str, value_type: ValueType, source: VariableSource| {
            vars.insert(
                uuid.clone(),
                Variable {
                    id: VariableId {
                        id: uuid,
                        name: name.to_string(),
                        value_type,
                    },
                    source,
                },
            )
        };

    make_source(
        uuid!("6330a6bb-51d1-4af7-9bd0-efeb00b1ff52"),
        "Position",
        ValueType::Number,
        VariableSource::Number(Box::new(GameNumberSource {
            extractor: |entry| entry.map(|e| *e.position as f32),
        })),
    );

    make_source(
        uuid!("171d7438-3179-4c70-b818-811cf86d514e"),
        "Car number",
        ValueType::Number,
        VariableSource::Number(Box::new(GameNumberSource {
            extractor: |entry| entry.map(|e| *e.car_number as f32),
        })),
    );

    make_source(
        uuid!("8abcf9d5-60f7-4886-a716-139d62ad83ac"),
        "Driver name",
        ValueType::Text,
        VariableSource::Text(Box::new(GameTextSource {
            extractor: |entry| {
                entry.and_then(|e| {
                    e.drivers
                        .get(&e.current_driver)
                        .map(|driver| format!("{} {}", driver.first_name, driver.last_name))
                })
            },
        })),
    );
}
