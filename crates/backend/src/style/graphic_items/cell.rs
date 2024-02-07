use std::collections::HashMap;

use bevy::prelude::Color;
use common::communication::TextAlignment;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::{
    Boolean, Font, Number, Property, Text, Texture, Tint, Vec2Property, Vec3Property,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Cell {
    pub id: Uuid,
    pub name: String,
    #[serde(flatten)]
    pub style: CellStyleDefinition,
    #[serde(default)]
    pub states: HashMap<Uuid, CellAttributes>,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Cell"),
            style: CellStyleDefinition {
                text: Property::Fixed(Text("Cell".to_string())),
                text_color: Property::Fixed(Tint(Color::WHITE)),
                text_size: Property::Fixed(Number(20.0)),
                font: Property::Fixed(Font::Default),
                color: Property::Fixed(Tint(Color::PURPLE)),
                pos: Vec3Property {
                    x: Property::Fixed(Number(0.0)),
                    y: Property::Fixed(Number(0.0)),
                    z: Property::Fixed(Number(0.0)),
                },
                size: Vec2Property {
                    x: Property::Fixed(Number(100.0)),
                    y: Property::Fixed(Number(100.0)),
                },
                skew: Property::Fixed(Number(0.0)),
                corner_offsets: CornerOffsets::default(),
                visible: Property::Fixed(Boolean(true)),
                rounding: Rounding {
                    top_left: Property::Fixed(Number(0.0)),
                    top_right: Property::Fixed(Number(0.0)),
                    bot_left: Property::Fixed(Number(0.0)),
                    bot_right: Property::Fixed(Number(0.0)),
                },
                text_alginment: TextAlignment::default(),
                text_position: Vec2Property {
                    x: Property::Fixed(Number(5.0)),
                    y: Property::Fixed(Number(15.0)),
                },
                image: Property::<Texture>::default(),
            },
            states: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Rounding {
    pub top_left: Property<Number>,
    pub top_right: Property<Number>,
    pub bot_left: Property<Number>,
    pub bot_right: Property<Number>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CornerOffsets {
    pub top_left: Vec2Property,
    pub top_right: Vec2Property,
    pub bot_left: Vec2Property,
    pub bot_right: Vec2Property,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CellStyleDefinition {
    pub text: Property<Text>,
    pub text_color: Property<Tint>,
    pub text_size: Property<Number>,
    pub font: Property<Font>,
    pub color: Property<Tint>,
    pub image: Property<Texture>,
    pub pos: Vec3Property,
    pub size: Vec2Property,
    pub skew: Property<Number>,
    pub corner_offsets: CornerOffsets,
    pub visible: Property<Boolean>,
    pub rounding: Rounding,
    pub text_alginment: TextAlignment,
    pub text_position: Vec2Property,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CellAttributes {
    #[serde(default)]
    #[serde(skip_serializing_if = "Override::is_disabled")]
    pub text: Override<Property<Text>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Override::is_disabled")]
    pub text_color: Override<Property<Tint>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Override::is_disabled")]
    pub text_size: Override<Property<Number>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Override::is_disabled")]
    pub font: Override<Property<Font>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Override::is_disabled")]
    pub color: Override<Property<Tint>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Override::is_disabled")]
    pub image: Override<Property<Texture>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Override::is_disabled")]
    pub pos: Override<Vec3Property>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Override::is_disabled")]
    pub size: Override<Vec2Property>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Override::is_disabled")]
    pub skew: Override<Property<Number>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Override::is_disabled")]
    pub corner_offsets: Override<CornerOffsets>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Override::is_disabled")]
    pub visible: Override<Property<Boolean>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Override::is_disabled")]
    pub rounding: Override<Rounding>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Override::is_disabled")]
    pub text_alginment: Override<TextAlignment>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Override::is_disabled")]
    pub text_position: Override<Vec2Property>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Override<T> {
    enabled: bool,
    value: T,
}

impl<T> Override<T> {
    pub fn is_disabled(&self) -> bool {
        !self.enabled
    }
}
