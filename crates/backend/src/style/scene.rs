use bevy::math::Vec2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SceneDefinition {
    pub id: Uuid,
    pub prefered_size: Vec2,
}
impl SceneDefinition {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            prefered_size: Vec2::new(1920.0, 1080.0),
        }
    }
}
