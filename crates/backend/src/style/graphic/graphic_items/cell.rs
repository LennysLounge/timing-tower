use bevy::prelude::Color;
use common::communication::TextAlignment;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_types::{
    Boolean, Font, Number, Property, Text, Texture, Tint, Vec2Property, Vec3Property,
};

use super::{Attribute, GraphicItemId};

#[derive(Serialize, Deserialize, Clone)]
pub struct Cell {
    pub id: GraphicItemId,
    pub name: String,
    pub text: Attribute<Property<Text>>,
    pub text_color: Attribute<Property<Tint>>,
    pub text_size: Attribute<Property<Number>>,
    pub font: Attribute<Property<Font>>,
    pub color: Attribute<Property<Tint>>,
    pub image: Attribute<Property<Texture>>,
    pub pos: Attribute<Vec3Property>,
    pub size: Attribute<Vec2Property>,
    pub skew: Attribute<Property<Number>>,
    pub corner_offsets: Attribute<CornerOffsets>,
    pub visible: Attribute<Property<Boolean>>,
    pub rounding: Attribute<Rounding>,
    pub text_alginment: Attribute<TextAlignment>,
    pub text_position: Attribute<Vec2Property>,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            id: GraphicItemId::new(),
            name: String::from("Cell"),
            text: Property::Fixed(Text("Cell".to_string())).into(),
            text_color: Property::Fixed(Tint(Color::WHITE)).into(),
            text_size: Property::Fixed(Number(20.0)).into(),
            font: Property::Fixed(Font::Default).into(),
            color: Property::Fixed(Tint(Color::PURPLE)).into(),
            pos: Vec3Property {
                x: Property::Fixed(Number(0.0)),
                y: Property::Fixed(Number(0.0)),
                z: Property::Fixed(Number(0.0)),
            }
            .into(),
            size: Vec2Property {
                x: Property::Fixed(Number(100.0)),
                y: Property::Fixed(Number(100.0)),
            }
            .into(),
            skew: Property::Fixed(Number(0.0)).into(),
            corner_offsets: CornerOffsets::default().into(),
            visible: Property::Fixed(Boolean(true)).into(),
            rounding: Rounding {
                top_left: Property::Fixed(Number(0.0)),
                top_right: Property::Fixed(Number(0.0)),
                bot_left: Property::Fixed(Number(0.0)),
                bot_right: Property::Fixed(Number(0.0)),
            }
            .into(),
            text_alginment: TextAlignment::default().into(),
            text_position: Vec2Property {
                x: Property::Fixed(Number(5.0)),
                y: Property::Fixed(Number(15.0)),
            }
            .into(),
            image: Property::<Texture>::default().into(),
        }
    }

    pub fn compute_for_state(&self, state: Option<&Uuid>) -> ComputedCell {
        ComputedCell {
            id: self.id,
            text: self.text.get_state_or_template(state),
            text_color: self.text_color.get_state_or_template(state),
            text_size: self.text_size.get_state_or_template(state),
            font: self.font.get_state_or_template(state),
            color: self.color.get_state_or_template(state),
            image: self.image.get_state_or_template(state),
            pos: self.pos.get_state_or_template(state),
            size: self.size.get_state_or_template(state),
            skew: self.skew.get_state_or_template(state),
            corner_offsets: self.corner_offsets.get_state_or_template(state),
            visible: self.visible.get_state_or_template(state),
            rounding: self.rounding.get_state_or_template(state),
            text_alginment: self.text_alginment.get_state_or_template(state),
            text_position: self.text_position.get_state_or_template(state),
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

pub struct ComputedCell {
    pub id: GraphicItemId,
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
