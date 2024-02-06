use std::marker::PhantomData;

use enumcapsulate::{AsVariantMut, AsVariantRef, FromVariant};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ExactVariant<E, V> {
    value: E,
    #[serde(skip)]
    #[serde(default)]
    _variant_type: PhantomData<V>,
}

impl<E, V> ExactVariant<E, V>
where
    E: AsVariantRef<V>,
{
    pub fn new(value: E) -> Option<ExactVariant<E, V>> {
        if value.as_variant_ref().is_some() {
            Some(Self {
                value,
                _variant_type: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn as_enum_ref(&self) -> &E {
        &self.value
    }

    pub fn as_enum_mut(&mut self) -> &mut E {
        &mut self.value
    }
}

impl<E, V> AsRef<V> for ExactVariant<E, V>
where
    E: AsVariantRef<V>,
{
    fn as_ref(&self) -> &V {
        self.value
            .as_variant_ref()
            .expect("Variant should always match")
    }
}

impl<E, V> AsMut<V> for ExactVariant<E, V>
where
    E: AsVariantMut<V>,
{
    fn as_mut(&mut self) -> &mut V {
        self.value
            .as_variant_mut()
            .expect("Variant should always match")
    }
}

impl<E, V> From<V> for ExactVariant<E, V>
where
    E: FromVariant<V>,
{
    fn from(value: V) -> Self {
        Self {
            value: E::from_variant(value),
            _variant_type: PhantomData,
        }
    }
}
