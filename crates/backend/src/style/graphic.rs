use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{graphic_items::root::Root, OwnedStyleItem, StyleItem, StyleItemMut, StyleItemRef};

#[derive(Serialize, Deserialize, Clone)]
pub struct State {
    pub id: Uuid,
    pub name: String,
}

/// A visual graphic component in the scene.
#[derive(Serialize, Deserialize, Clone)]
pub struct GraphicDefinition {
    pub id: Uuid,
    pub name: String,
    pub items: Root,
    pub states: Vec<State>,
}
impl GraphicDefinition {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Graphic"),
            items: Root::new(),
            states: Vec::new(),
        }
    }
}

impl StyleItem for GraphicDefinition {
    fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    fn as_ref<'a>(&'a self) -> StyleItemRef<'a> {
        StyleItemRef::Graphic(self)
    }

    fn as_mut<'a>(&'a mut self) -> StyleItemMut<'a> {
        StyleItemMut::Graphic(self)
    }
    fn to_owned(self) -> OwnedStyleItem {
        OwnedStyleItem::Graphic(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GraphicFolder {
    pub id: Uuid,
    pub name: String,
    pub content: Vec<GraphicOrFolder>,
}
impl GraphicFolder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Graphics"),
            content: Vec::new(),
        }
    }
    pub fn contained_graphics(&self) -> Vec<&GraphicDefinition> {
        self.content
            .iter()
            .flat_map(|af| match af {
                GraphicOrFolder::Graphic(a) => vec![a],
                GraphicOrFolder::Folder(f) => f.contained_graphics(),
            })
            .collect()
    }
}
impl StyleItem for GraphicFolder {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn as_ref<'a>(&'a self) -> StyleItemRef<'a> {
        StyleItemRef::GraphicFolder(self)
    }
    fn as_mut<'a>(&'a mut self) -> StyleItemMut<'a> {
        StyleItemMut::GraphicFolder(self)
    }
    fn to_owned(self) -> OwnedStyleItem {
        OwnedStyleItem::GraphicFolder(self)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "element_type")]
pub enum GraphicOrFolder {
    Graphic(GraphicDefinition),
    Folder(GraphicFolder),
}
impl GraphicOrFolder {
    pub fn id(&self) -> &Uuid {
        match self {
            GraphicOrFolder::Graphic(o) => &o.id,
            GraphicOrFolder::Folder(o) => &o.id,
        }
    }
}
