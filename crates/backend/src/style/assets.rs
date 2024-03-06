use std::ops::Deref;

use enumcapsulate::{VariantDiscriminant, VariantDowncast};
use serde::{Deserialize, Serialize};

use crate::{
    exact_variant::ExactVariant,
    style::StyleItemDiscriminant,
    value_store::{AnyValueProducer, ProducerId},
    value_types::{AnyProducerRef, Font, Texture, ValueType},
};

use super::{variables::StaticValueProducer, StyleId, StyleItem, TreePosition};

#[derive(Serialize, Deserialize, Clone)]
pub struct AssetDefinition {
    pub id: StyleId,
    pub name: String,
    pub value_type: ValueType,
    pub path: String,
}
impl AssetDefinition {
    pub fn new() -> Self {
        Self {
            id: StyleId::new(),
            name: String::from("image"),
            value_type: ValueType::Texture,
            path: String::new(),
        }
    }
    pub fn value_producer(&self) -> AnyValueProducer {
        match self.value_type {
            ValueType::Texture => StaticValueProducer(Texture::Handle(self.value_id().0)).into(),
            ValueType::Font => StaticValueProducer(Font::Handle(self.value_id().0)).into(),
            value_type @ _ => {
                unreachable!("An asset cannot have a value type of {}", value_type.name())
            }
        }
    }
    pub fn value_id(&self) -> ProducerId {
        ProducerId(self.id.0)
    }
    pub fn producer_ref(&self) -> AnyProducerRef {
        AnyProducerRef::new(self.value_id(), self.value_type)
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AssetFolder {
    pub id: StyleId,
    pub name: String,
    pub content: Vec<AssetOrFolder>,
}
impl AssetFolder {
    pub fn new() -> Self {
        Self {
            id: StyleId::new(),
            name: String::from("Group"),
            content: Vec::new(),
        }
    }
    pub fn contained_assets(&self) -> Vec<&AssetDefinition> {
        self.content
            .iter()
            .flat_map(|af| match af {
                AssetOrFolder::Asset(a) => vec![a.deref()],
                AssetOrFolder::Folder(f) => f.contained_assets(),
            })
            .collect()
    }
    pub fn remove_if_present(
        &mut self,
        id: &StyleId,
    ) -> Option<(StyleItem, TreePosition<StyleId>)> {
        if let Some(index) = self.content.iter().position(|c| c.id() == id) {
            Some((
                self.content.remove(index).to_enum(),
                if index == 0 {
                    TreePosition::First
                } else {
                    TreePosition::After(*self.content[index - 1].id())
                },
            ))
        } else {
            None
        }
    }
    pub fn insert(
        &mut self,
        item: StyleItem,
        position: TreePosition<StyleId>,
    ) -> Result<(), StyleItem> {
        let element = match item.variant_discriminant() {
            StyleItemDiscriminant::Asset => AssetOrFolder::Asset(
                item.as_variant_downcast::<AssetDefinition>()
                    .unwrap()
                    .into(),
            ),
            StyleItemDiscriminant::AssetFolder => {
                AssetOrFolder::Folder(item.as_variant_downcast::<AssetFolder>().unwrap().into())
            }
            _ => return Err(item),
        };
        match position {
            TreePosition::First => self.content.insert(0, element),
            TreePosition::Last => self.content.push(element),
            TreePosition::After(id) => {
                if let Some(index) = self.content.iter().position(|c| c.id() == &id) {
                    self.content.insert(index + 1, element);
                }
            }
            TreePosition::Before(id) => {
                if let Some(index) = self.content.iter().position(|c| c.id() == &id) {
                    self.content.insert(index, element);
                }
            }
        }
        Ok(())
    }
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "element_type")]
pub enum AssetOrFolder {
    Asset(ExactVariant<StyleItem, AssetDefinition>),
    Folder(ExactVariant<StyleItem, AssetFolder>),
}
impl AssetOrFolder {
    pub fn id(&self) -> &StyleId {
        match self {
            AssetOrFolder::Asset(o) => &o.id,
            AssetOrFolder::Folder(o) => &o.id,
        }
    }
    fn to_enum(self) -> StyleItem {
        match self {
            AssetOrFolder::Asset(a) => a.to_enum(),
            AssetOrFolder::Folder(f) => f.to_enum(),
        }
    }
}
