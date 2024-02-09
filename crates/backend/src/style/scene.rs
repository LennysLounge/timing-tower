use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

use super::StyleId;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SceneDefinition {
    pub id: StyleId,
    pub prefered_size: Vec2,
}
impl SceneDefinition {
    pub fn new() -> Self {
        Self {
            id: StyleId::new(),
            prefered_size: Vec2::new(1920.0, 1080.0),
        }
    }
}
