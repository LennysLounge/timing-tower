use bevy::math::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{OwnedStyleItem, StyleItem, StyleItemMut, StyleItemRef};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SceneDefinition {
    pub id: Uuid,
    pub prefered_size: Vec2,
}
impl StyleItem for SceneDefinition {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn as_ref<'a>(&'a self) -> StyleItemRef<'a> {
        StyleItemRef::Scene(self)
    }
    fn as_mut<'a>(&'a mut self) -> StyleItemMut<'a> {
        StyleItemMut::Scene(self)
    }
    fn to_owned(self) -> OwnedStyleItem {
        OwnedStyleItem::Scene(self)
    }
}
