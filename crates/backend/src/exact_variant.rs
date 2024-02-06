use std::marker::PhantomData;

use enumcapsulate::{AsVariantMut, AsVariantRef};

pub struct ExactVariant<E, V> {
    value: E,
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
