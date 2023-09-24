use std::collections::HashMap;

use bevy::prelude::{Color, Resource};
use unified_sim_model::model::Entry;
use uuid::{uuid, Uuid};

use crate::editor::{
    properties::{ColorProperty, NumberProperty, TextProperty},
    variable_element::{VariableDefinition, VariableType, VariablesElement},
};

pub trait NumberSource {
    fn resolve(&self, entry: Option<&Entry>) -> Option<f32>;
}

pub trait TextSource {
    fn resolve(&self, entry: Option<&Entry>) -> Option<String>;
}

pub trait ColorSource {
    fn resolve(&self, entry: Option<&Entry>) -> Option<Color>;
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
    fn resolve(&self, _entry: Option<&Entry>) -> Option<f32> {
        Some(self.0)
    }
}

pub struct StaticText(String);
impl TextSource for StaticText {
    fn resolve(&self, _entry: Option<&Entry>) -> Option<String> {
        Some(self.0.clone())
    }
}

pub struct StaticColor(Color);
impl ColorSource for StaticColor {
    fn resolve(&self, _entry: Option<&Entry>) -> Option<Color> {
        Some(self.0)
    }
}

impl VariableRepo {
    pub fn reload_repo(&mut self, var_defs: &VariablesElement) {
        self.vars.clear();

        generate_game_sources(&mut self.vars);

        for var_def in var_defs.vars.iter() {
            let var = match &var_def.var_type {
                VariableType::StaticNumber(n) => VariableSource::Number(Box::new(StaticNumber(*n))),
                VariableType::StaticText(t) => {
                    VariableSource::Text(Box::new(StaticText(t.clone())))
                }
                VariableType::StaticColor(c) => VariableSource::Color(Box::new(StaticColor(*c))),
                _ => unreachable!(),
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

    pub fn get_number(&self, property: &NumberProperty, entry: Option<&Entry>) -> Option<f32> {
        match property {
            NumberProperty::Fixed(n) => Some(*n),
            NumberProperty::Ref(id) => self
                .vars
                .get(id)
                .and_then(|v| v.source.resolve_number(entry)),
        }
    }

    pub fn get_text(&self, property: &TextProperty, entry: Option<&Entry>) -> Option<String> {
        match property {
            TextProperty::Fixed(n) => Some(n.clone()),
            TextProperty::Ref(id) => self.vars.get(id).and_then(|v| v.source.resolve_text(entry)),
        }
    }

    pub fn get_color(&self, property: &ColorProperty, entry: Option<&Entry>) -> Option<Color> {
        match property {
            ColorProperty::Fixed(n) => Some(n.clone()),
            ColorProperty::Ref(id) => self
                .vars
                .get(id)
                .and_then(|v| v.source.resolve_color(entry)),
        }
    }
}

impl VariableSource {
    pub fn resolve_number(&self, entry: Option<&Entry>) -> Option<f32> {
        match self {
            VariableSource::Number(s) => s.resolve(entry),
            _ => None,
        }
    }

    pub fn resolve_text(&self, entry: Option<&Entry>) -> Option<String> {
        match self {
            VariableSource::Text(s) => s.resolve(entry),
            VariableSource::Number(s) => s.resolve(entry).map(|n| format!("{n}")),
            _ => None,
        }
    }

    pub fn resolve_color(&self, entry: Option<&Entry>) -> Option<Color> {
        match self {
            VariableSource::Color(s) => s.resolve(entry),
            _ => None,
        }
    }
}

pub struct GameNumberSource {
    extractor: fn(Option<&Entry>) -> Option<f32>,
}
impl NumberSource for GameNumberSource {
    fn resolve(&self, entry: Option<&Entry>) -> Option<f32> {
        (self.extractor)(entry)
    }
}

pub struct GameTextSource {
    extractor: fn(Option<&Entry>) -> Option<String>,
}
impl TextSource for GameTextSource {
    fn resolve(&self, entry: Option<&Entry>) -> Option<String> {
        (self.extractor)(entry)
    }
}

fn generate_game_sources(vars: &mut HashMap<Uuid, Variable>) {
    let uuid = uuid!("6330a6bb-51d1-4af7-9bd0-efeb00b1ff52");
    vars.insert(
        uuid.clone(),
        Variable {
            def: VariableDefinition {
                id: uuid,
                name: "Position".to_string(),
                var_type: VariableType::Game,
            },
            source: VariableSource::Number(Box::new(GameNumberSource {
                extractor: |entry| entry.map(|e| *e.position as f32),
            })),
        },
    );

    let uuid = uuid!("171d7438-3179-4c70-b818-811cf86d514e");
    vars.insert(
        uuid.clone(),
        Variable {
            def: VariableDefinition {
                id: uuid,
                name: "Car number".to_string(),
                var_type: VariableType::Game,
            },
            source: VariableSource::Number(Box::new(GameNumberSource {
                extractor: |entry| entry.map(|e| *e.car_number as f32),
            })),
        },
    );

    let uuid = uuid!("8abcf9d5-60f7-4886-a716-139d62ad83ac");
    vars.insert(
        uuid.clone(),
        Variable {
            def: VariableDefinition {
                id: uuid,
                name: "Driver name".to_string(),
                var_type: VariableType::Game,
            },
            source: VariableSource::Text(Box::new(GameTextSource {
                extractor: |entry| {
                    entry.and_then(|e| {
                        e.drivers
                            .get(&e.current_driver)
                            .map(|driver| format!("{} {}", driver.first_name, driver.last_name))
                    })
                },
            })),
        },
    );
}
