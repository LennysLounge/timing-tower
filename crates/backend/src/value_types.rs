use std::marker::PhantomData;

use bevy::render::color::Color;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    Handle(String),
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

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(transparent)]
pub struct ValueRef<T> {
    pub id: Uuid,
    #[serde(skip)]
    pub phantom: PhantomData<T>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UntypedValueRef {
    pub id: Uuid,
    pub value_type: ValueType,
}

impl UntypedValueRef {
    pub fn typed<T>(self) -> ValueRef<T> {
        ValueRef {
            id: self.id,
            phantom: PhantomData,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Property<T> {
    ValueRef(ValueRef<T>),
    #[serde(untagged)]
    Fixed(T),
}

impl<T: Default> Default for Property<T> {
    fn default() -> Self {
        Property::Fixed(T::default())
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Vec2Property {
    pub x: Property<Number>,
    pub y: Property<Number>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Vec3Property {
    pub x: Property<Number>,
    pub y: Property<Number>,
    pub z: Property<Number>,
}
