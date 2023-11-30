use bevy::{
    asset::Handle,
    render::{color::Color, texture::Image},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Number(pub f32);

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Text(pub String);

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Tint(pub Color);

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Boolean(pub bool);

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum Texture {
    #[default]
    None,
    #[serde(skip)]
    Handle(Handle<Image>),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, Default, Copy)]
pub enum ValueType {
    #[default]
    Number,
    Text,
    Tint,
    Boolean,
    Texture,
}
impl ValueType {
    pub fn can_cast_to(&self, target: &ValueType) -> bool {
        use ValueType::*;

        match (self, target) {
            (ref a, ref b) if a == b => true,
            (Number, Text) => true,
            (Boolean, Text) => true,
            _ => false,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            ValueType::Number => "Number",
            ValueType::Text => "Text",
            ValueType::Tint => "Color",
            ValueType::Boolean => "Boolean",
            ValueType::Texture => "Image",
        }
    }
}

pub trait ValueTypeOf<T> {
    fn get() -> Self;
}

impl ValueTypeOf<Number> for ValueType {
    fn get() -> Self {
        ValueType::Number
    }
}
impl ValueTypeOf<Text> for ValueType {
    fn get() -> Self {
        ValueType::Text
    }
}
impl ValueTypeOf<Tint> for ValueType {
    fn get() -> Self {
        ValueType::Tint
    }
}
impl ValueTypeOf<Boolean> for ValueType {
    fn get() -> Self {
        ValueType::Boolean
    }
}
impl ValueTypeOf<Texture> for ValueType {
    fn get() -> Self {
        ValueType::Texture
    }
}
