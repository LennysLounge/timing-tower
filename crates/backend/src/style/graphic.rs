use std::ops::Deref;

use enumcapsulate::{VariantDiscriminant, VariantDowncast};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::exact_variant::ExactVariant;

use graphic_items::{root::Root, ComputedGraphicItem, GraphicItem};

use super::{
    StyleId, StyleItem, StyleItemDiscriminant, TreePosition,
};

pub mod graphic_items;

#[derive(Serialize, Deserialize, Clone)]
pub struct GraphicState {
    pub id: GraphicStateId,
    pub name: String,
}

pub const GRAPHIC_STATE_HIDDEN: GraphicStateId =
    GraphicStateId(uuid::uuid!("5d493243-8787-4b4c-a06f-ceb17a35f10a"));

/// Id that identifies a graphic state item.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub struct GraphicStateId(pub Uuid);
impl GraphicStateId {
    pub fn new() -> Self {
        GraphicStateId(Uuid::new_v4())
    }
}

pub struct ComputedGraphic {
    pub graphic_id: Uuid,
    pub root: ComputedGraphicItem,
}

/// A visual graphic component in the scene.
#[derive(Serialize, Deserialize, Clone)]
pub struct GraphicDefinition {
    pub id: StyleId,
    pub name: String,
    pub items: ExactVariant<GraphicItem, Root>,
    pub states: Vec<GraphicState>,
}
impl GraphicDefinition {
    pub fn new() -> Self {
        Self {
            id: StyleId::new(),
            name: String::from("Graphic"),
            items: Root::new().into(),
            states: Vec::new(),
        }
    }

    pub fn compute_style(&self, state: Option<&GraphicStateId>) -> ComputedGraphic {
        ComputedGraphic {
            graphic_id: self.id.0,
            root: self.items.as_enum_ref().compute_for_state(state),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GraphicFolder {
    pub id: StyleId,
    pub name: String,
    pub content: Vec<GraphicOrFolder>,
}
impl GraphicFolder {
    pub fn new() -> Self {
        Self {
            id: StyleId::new(),
            name: String::from("Graphics"),
            content: Vec::new(),
        }
    }
    pub fn contained_graphics(&self) -> Vec<&GraphicDefinition> {
        self.content
            .iter()
            .flat_map(|af| match af {
                GraphicOrFolder::Graphic(a) => vec![a.deref()],
                GraphicOrFolder::Folder(f) => f.contained_graphics(),
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
        println!("insert graphic");
        let element = match item.variant_discriminant() {
            StyleItemDiscriminant::Graphic => GraphicOrFolder::Graphic(
                item.as_variant_downcast::<GraphicDefinition>()
                    .unwrap()
                    .into(),
            ),
            StyleItemDiscriminant::GraphicFolder => {
                GraphicOrFolder::Folder(item.as_variant_downcast::<GraphicFolder>().unwrap().into())
            }
            _ => return Err(item),
        };
        println!("element is correct");
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
pub enum GraphicOrFolder {
    Graphic(ExactVariant<StyleItem, GraphicDefinition>),
    Folder(ExactVariant<StyleItem, GraphicFolder>),
}
impl GraphicOrFolder {
    pub fn id(&self) -> &StyleId {
        match self {
            GraphicOrFolder::Graphic(o) => &o.id,
            GraphicOrFolder::Folder(o) => &o.id,
        }
    }
    fn to_enum(self) -> StyleItem {
        match self {
            GraphicOrFolder::Graphic(g) => g.to_enum(),
            GraphicOrFolder::Folder(f) => f.to_enum(),
        }
    }
}
