use std::collections::HashMap;

use bevy::prelude::{Color, Resource};
use serde::{Deserialize, Serialize};
use unified_sim_model::model::Entry;
use uuid::{uuid, Uuid};

use crate::style::properties::{BooleanProperty, ColorProperty, NumberProperty, TextProperty};

pub trait NumberSource {
    fn resolve(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<f32>;
}

pub trait TextSource {
    fn resolve(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<String>;
}

pub trait ColorSource {
    fn resolve(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<Color>;
}

pub trait BooleanSource {
    fn resolve(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<bool>;
}

pub trait VariableDefinition {
    fn as_variable_source(&self) -> AssetSource;
    fn get_variable_id(&self) -> &AssetId;
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, Default)]
pub enum AssetType {
    #[default]
    Number,
    Text,
    Color,
    Boolean,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Reference {
    pub value_type: AssetType,
    pub key: Uuid,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AssetId {
    pub id: Uuid,
    pub name: String,
    pub asset_type: AssetType,
}

impl Default for AssetId {
    fn default() -> Self {
        Self {
            name: "Variable".to_string(),
            id: Uuid::new_v4(),
            asset_type: AssetType::default(),
        }
    }
}

pub struct Asset {
    pub id: AssetId,
    pub source: AssetSource,
}

pub enum AssetSource {
    Number(Box<dyn NumberSource + Send + Sync>),
    Text(Box<dyn TextSource + Send + Sync>),
    Color(Box<dyn ColorSource + Send + Sync>),
    Bool(Box<dyn BooleanSource + Send + Sync>),
}

#[derive(Resource)]
pub struct AssetRepo {
    pub assets: HashMap<Uuid, Asset>,
}

impl AssetRepo {
    pub fn reload_repo(&mut self, var_defs: Vec<&impl VariableDefinition>) {
        self.assets.clear();

        generate_game_sources(&mut self.assets);

        for var_def in var_defs {
            self.assets.insert(
                var_def.get_variable_id().id.clone(),
                Asset {
                    id: var_def.get_variable_id().clone(),
                    source: var_def.as_variable_source(),
                },
            );
        }
    }

    pub fn get_var_def(&self, reference: &Reference) -> Option<&AssetId> {
        self.assets.get(&reference.key).map(|v| &v.id)
    }

    pub fn get_number(&self, reference: &Reference, entry: Option<&Entry>) -> Option<f32> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.source.resolve_number(self, entry))
    }

    pub fn get_text(&self, reference: &Reference, entry: Option<&Entry>) -> Option<String> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.source.resolve_text(self, entry))
    }

    pub fn get_color(&self, reference: &Reference, entry: Option<&Entry>) -> Option<Color> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.source.resolve_color(self, entry))
    }
    pub fn get_bool(&self, reference: &Reference, entry: Option<&Entry>) -> Option<bool> {
        self.assets
            .get(&reference.key)
            .and_then(|v| v.source.resolve_bool(self, entry))
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

    pub fn get_bool_property(
        &self,
        property: &BooleanProperty,
        entry: Option<&Entry>,
    ) -> Option<bool> {
        match property {
            BooleanProperty::Fixed(b) => Some(*b),
            BooleanProperty::Ref(reference) => self.get_bool(reference, entry),
        }
    }
}

impl AssetSource {
    pub fn resolve_number(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<f32> {
        match self {
            AssetSource::Number(s) => s.resolve(vars, entry),
            _ => None,
        }
    }

    pub fn resolve_text(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<String> {
        match self {
            AssetSource::Text(s) => s.resolve(vars, entry),
            AssetSource::Number(s) => s.resolve(vars, entry).map(|n| format!("{n}")),
            _ => None,
        }
    }

    pub fn resolve_color(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<Color> {
        match self {
            AssetSource::Color(s) => s.resolve(vars, entry),
            _ => None,
        }
    }
    pub fn resolve_bool(&self, vars: &AssetRepo, entry: Option<&Entry>) -> Option<bool> {
        match self {
            AssetSource::Bool(s) => s.resolve(vars, entry),
            _ => None,
        }
    }
}

impl Asset {
    pub fn get_ref(&self) -> Reference {
        Reference {
            value_type: self.id.asset_type.clone(),
            key: self.id.id.clone(),
        }
    }
}

impl AssetType {
    pub fn can_cast_to(&self, other: &AssetType) -> bool {
        match (self, other) {
            (ref a, ref b) if a == b => true,
            (AssetType::Number, AssetType::Text) => true,
            _ => false,
        }
    }
}

/*
*-------------------------------------------------------------------------------------------------------

*/

pub struct StaticNumber(pub f32);
impl NumberSource for StaticNumber {
    fn resolve(&self, _vars: &AssetRepo, _entry: Option<&Entry>) -> Option<f32> {
        Some(self.0)
    }
}

pub struct StaticText(pub String);
impl TextSource for StaticText {
    fn resolve(&self, _vars: &AssetRepo, _entry: Option<&Entry>) -> Option<String> {
        Some(self.0.clone())
    }
}

pub struct StaticColor(pub Color);
impl ColorSource for StaticColor {
    fn resolve(&self, _vars: &AssetRepo, _entry: Option<&Entry>) -> Option<Color> {
        Some(self.0)
    }
}
pub struct StaticBoolean(pub bool);
impl BooleanSource for StaticBoolean {
    fn resolve(&self, _vars: &AssetRepo, _entry: Option<&Entry>) -> Option<bool> {
        Some(self.0)
    }
}

pub struct GameNumberSource {
    extractor: fn(Option<&Entry>) -> Option<f32>,
}
impl NumberSource for GameNumberSource {
    fn resolve(&self, _vars: &AssetRepo, entry: Option<&Entry>) -> Option<f32> {
        (self.extractor)(entry)
    }
}

pub struct GameTextSource {
    extractor: fn(Option<&Entry>) -> Option<String>,
}
impl TextSource for GameTextSource {
    fn resolve(&self, _vars: &AssetRepo, entry: Option<&Entry>) -> Option<String> {
        (self.extractor)(entry)
    }
}

pub struct GameBooleanSource {
    extractor: fn(Option<&Entry>) -> Option<bool>,
}
impl BooleanSource for GameBooleanSource {
    fn resolve(&self, _vars: &AssetRepo, entry: Option<&Entry>) -> Option<bool> {
        (self.extractor)(entry)
    }
}

fn generate_game_sources(vars: &mut HashMap<Uuid, Asset>) {
    let mut make_source = |uuid: Uuid, name: &str, value_type: AssetType, source: AssetSource| {
        vars.insert(
            uuid.clone(),
            Asset {
                id: AssetId {
                    id: uuid,
                    name: name.to_string(),
                    asset_type: value_type,
                },
                source,
            },
        )
    };

    make_source(
        uuid!("6330a6bb-51d1-4af7-9bd0-efeb00b1ff52"),
        "Position",
        AssetType::Number,
        AssetSource::Number(Box::new(GameNumberSource {
            extractor: |entry| entry.map(|e| *e.position as f32),
        })),
    );

    make_source(
        uuid!("171d7438-3179-4c70-b818-811cf86d514e"),
        "Car number",
        AssetType::Number,
        AssetSource::Number(Box::new(GameNumberSource {
            extractor: |entry| entry.map(|e| *e.car_number as f32),
        })),
    );

    make_source(
        uuid!("8abcf9d5-60f7-4886-a716-139d62ad83ac"),
        "Driver name",
        AssetType::Text,
        AssetSource::Text(Box::new(GameTextSource {
            extractor: |entry| {
                entry.and_then(|e| {
                    e.drivers.get(&e.current_driver).map(|driver| {
                        let letter = if driver.first_name.is_empty() {
                            ""
                        } else {
                            &driver.first_name[0..1]
                        };
                        format!("{} {}", letter, driver.last_name)
                    })
                })
            },
        })),
    );
    make_source(
        uuid!("de909160-f54b-40cf-a987-6a8453df0914"),
        "Is focused",
        AssetType::Boolean,
        AssetSource::Bool(Box::new(GameBooleanSource {
            extractor: |entry| entry.map(|e| e.focused),
        })),
    );

    make_source(
        uuid!("c16f71b9-dcc9-4f04-9579-ea5211fa99be"),
        "Is in pits",
        AssetType::Boolean,
        AssetSource::Bool(Box::new(GameBooleanSource {
            extractor: |entry| entry.map(|e| *e.in_pits),
        })),
    );
}
