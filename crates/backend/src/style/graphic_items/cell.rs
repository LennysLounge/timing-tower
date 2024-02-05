use std::collections::HashMap;

use bevy::prelude::Color;
use common::communication::TextAlignment;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::{
    Boolean, Font, Number, Property, Text, Texture, Tint, Vec2Property, Vec3Property,
};

use super::EnumSet;

#[derive(Serialize, Deserialize, Clone)]
pub struct Cell {
    pub id: Uuid,
    pub name: String,
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
    pub attributes: HashMap<Uuid, EnumSet<CellAttributes>>,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Cell"),
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
            attributes: HashMap::new(),
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

#[derive(Serialize, Deserialize, Clone)]
pub enum CellAttributes {
    Text(Property<Text>),
    TextColor(Property<Tint>),
    TextSize(Property<Number>),
    Font(Property<Font>),
    Color(Property<Tint>),
    Image(Property<Texture>),
    Pos(Vec3Property),
    Size(Vec2Property),
    Skew(Property<Number>),
    CornerOffsets(CornerOffsets),
    Visible(Property<Boolean>),
    Rounding(Rounding),
    TextAlginment(TextAlignment),
    TextPosition(Vec2Property),
}
impl ToString for CellAttributes {
    fn to_string(&self) -> String {
        String::from(match self {
            CellAttributes::Text(_) => "Text",
            CellAttributes::TextColor(_) => "TextColor",
            CellAttributes::TextSize(_) => "TextSize",
            CellAttributes::Font(_) => "Font",
            CellAttributes::Color(_) => "Color",
            CellAttributes::Image(_) => "Image",
            CellAttributes::Pos(_) => "Pos",
            CellAttributes::Size(_) => "Size",
            CellAttributes::Skew(_) => "Skew",
            CellAttributes::CornerOffsets(_) => "CornerOffsets",
            CellAttributes::Visible(_) => "Visible",
            CellAttributes::Rounding(_) => "Rounding",
            CellAttributes::TextAlginment(_) => "TextAlginment",
            CellAttributes::TextPosition(_) => "TextPosition",
        })
    }
}
