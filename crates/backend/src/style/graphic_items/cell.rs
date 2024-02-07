use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

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
            id: Uuid::new_v4(),
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
pub struct Attribute<T> {
    template: T,
    states: HashMap<Uuid, T>,
}
impl<T> Attribute<T> {
    pub fn template(&self) -> &T {
        &self.template
    }
    pub fn template_mut(&mut self) -> &mut T {
        &mut self.template
    }
}
impl<T> Deref for Attribute<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.template()
    }
}
impl<T> DerefMut for Attribute<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.template_mut()
    }
}

impl<T> From<T> for Attribute<T> {
    fn from(value: T) -> Self {
        Self {
            template: value,
            states: HashMap::new(),
        }
    }
}
