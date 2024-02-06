use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use enumcapsulate::{AsVariantMut, AsVariantRef, FromVariant};
use serde::{Deserialize, Serialize};

use crate::tree_iterator::{TreeIterator, TreeIteratorMut};

#[derive(Clone, Serialize, Deserialize)]
pub struct ExactVariant<E, V> {
    #[serde(flatten)]
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
}

impl<E, V> Deref for ExactVariant<E, V>
where
    E: AsVariantRef<V>,
{
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.value
            .as_variant_ref()
            .expect("Variant should always match")
    }
}

impl<E, V> DerefMut for ExactVariant<E, V>
where
    E: AsVariantRef<V> + AsVariantMut<V>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
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

impl<E, V> TreeIterator for ExactVariant<E, V>
where
    E: TreeIterator,
{
    type Item<'item> = <E as TreeIterator>::Item<'item>;

    fn walk<F, R>(&self, f: &mut F) -> std::ops::ControlFlow<R>
    where
        F: FnMut(&Self::Item<'_>, crate::tree_iterator::Method) -> std::ops::ControlFlow<R>,
    {
        self.value.walk(f)
    }
}

impl<E, V> TreeIteratorMut for ExactVariant<E, V>
where
    E: TreeIteratorMut,
{
    type Item<'item> = <E as TreeIteratorMut>::Item<'item>;

    fn walk_mut<F, R>(&mut self, f: &mut F) -> std::ops::ControlFlow<R>
    where
        F: FnMut(&mut Self::Item<'_>, crate::tree_iterator::Method) -> std::ops::ControlFlow<R>,
    {
        self.value.walk_mut(f)
    }
}
