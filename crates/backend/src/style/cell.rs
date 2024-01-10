use bevy::prelude::Color;
use common::communication::TextAlignment;
use serde::{Deserialize, Serialize};

use crate::value_types::{
    Boolean, Font, Number, Property, Text, Texture, Tint, Vec2Property, Vec3Property,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Cell {
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

impl Default for Cell {
    fn default() -> Self {
        Self {
            text: Property::Fixed(Text("Column".to_string())),
            text_color: Property::Fixed(Tint(Color::BLACK)),
            text_size: Property::Fixed(Number(20.0)),
            font: Property::Fixed(Font::Default),
            color: Property::Fixed(Tint(Color::PURPLE)),
            pos: Vec3Property {
                x: Property::Fixed(Number(10.0)),
                y: Property::Fixed(Number(10.0)),
                z: Property::Fixed(Number(0.0)),
            },
            size: Vec2Property {
                x: Property::Fixed(Number(30.0)),
                y: Property::Fixed(Number(30.0)),
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
pub struct ClipArea {
    pub pos: Vec3Property,
    pub size: Vec2Property,
    pub skew: Property<Number>,
    pub rounding: Rounding,
    pub render_layer: u8,
}
