use std::marker::PhantomData;

use bevy::render::color::Color;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::value_store::ProducerId;

/// Base trait for style value type in the application.
pub trait Value {
    /// Return the type of this value.
    fn ty() -> ValueType;
}

/// A number. Contains a f32.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Number(pub f32);
impl Value for Number {
    fn ty() -> ValueType {
        ValueType::Number
    }
}

/// A type that represents some text. Contains a owned String.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Text(pub String);
impl Value for Text {
    fn ty() -> ValueType {
        ValueType::Text
    }
}

/// A type that represents a color. Contains a [`bevy::render::color::Color`].
/// The name "tint" was chosen to avoid ambiguity with the bevy color type.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Tint(pub Color);
impl Value for Tint {
    fn ty() -> ValueType {
        ValueType::Tint
    }
}

/// A boolean type.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Boolean(pub bool);
impl Value for Boolean {
    fn ty() -> ValueType {
        ValueType::Boolean
    }
}

/// A type that represents a texture. Stores a reference to an asset
/// that contains the actual texture data. Textures are generally optional
/// Which is why this type contains the "None" variant.
#[derive(Serialize, Deserialize, Clone, Default)]
pub enum Texture {
    #[default]
    None,
    Handle(Uuid),
}
impl Value for Texture {
    fn ty() -> ValueType {
        ValueType::Texture
    }
}

/// A type that represents a font. Stores a reference to an asset
/// that contains the actual font data. Fonts can be unspecified
/// in which case a default font is used.
#[derive(Serialize, Deserialize, Clone, Default)]
pub enum Font {
    #[default]
    Default,
    Handle(Uuid),
}
impl Value for Font {
    fn ty() -> ValueType {
        ValueType::Font
    }
}

/// Enumerates the different available value types.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, Default, Copy)]
pub enum ValueType {
    #[default]
    Number,
    Text,
    Tint,
    Boolean,
    Texture,
    Font,
}
impl ValueType {
    /// Test if this type can be cast to the target type.
    pub fn can_cast_to(&self, target: &ValueType) -> bool {
        match (self, target) {
            (ref a, ref b) if a == b => true,
            (ValueType::Number, ValueType::Text) => true,
            (ValueType::Boolean, ValueType::Text) => true,
            _ => false,
        }
    }
    /// Get the name of this value type.
    pub fn name(&self) -> &str {
        match self {
            ValueType::Number => "Number",
            ValueType::Text => "Text",
            ValueType::Tint => "Color",
            ValueType::Boolean => "Boolean",
            ValueType::Texture => "Image",
            ValueType::Font => "Font",
        }
    }
}

/// References a [`ValueProducer`](crate::value_store::ValueProducer) in the
/// [`ValueStore`](crate::value_store::ValueStore).  
/// Carries the type of the `ValueProducer` in the generic type `T`.
#[derive(Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct ProducerRef<T> {
    pub id: ProducerId,
    #[serde(skip)]
    pub phantom: PhantomData<T>,
}
impl<T: Value> ProducerRef<T> {
    pub fn to_any_producer_ref(self) -> AnyProducerRef {
        AnyProducerRef {
            id: self.id,
            value_type: T::ty(),
        }
    }
}
impl<T> Clone for ProducerRef<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            phantom: self.phantom.clone(),
        }
    }
}

/// References a [`ValueProducer`](crate::value_store::ValueProducer) in the
/// [`ValueStore`](crate::value_store::ValueStore).  
/// Carries the type of the `ValueProducer` as a field and can therefore represents
/// any value producer of any type.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AnyProducerRef {
    pub id: ProducerId,
    pub value_type: ValueType,
}

impl AnyProducerRef {
    pub fn typed<T>(self) -> ProducerRef<T> {
        ProducerRef {
            id: self.id,
            phantom: PhantomData,
        }
    }
    pub fn to_typed_2<T: Value>(&self) -> Option<ProducerRef<T>> {
        if T::ty() == self.value_type {
            Some(ProducerRef {
                id: self.id,
                phantom: PhantomData,
            })
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Property<T> {
    Producer(ProducerId),
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
