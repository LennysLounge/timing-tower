use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{elements::GraphicItems, StyleItemRef, StyleItemMut, OwnedStyleItem, StyleItem};

/// A visual graphic component in the scene.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Graphic {
    pub id: Uuid,
    pub name: String,
    pub items: GraphicItems,
}
impl Graphic {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Graphic"),
            items: GraphicItems::default(),
        }
    }
}

impl StyleItem for Graphic {
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
