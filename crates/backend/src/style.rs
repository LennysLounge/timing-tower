use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::{
    assets::AssetDefinition, folder::Folder, timing_tower::TimingTower,
    variables::VariableDefinition,
};

pub mod assets;
pub mod cell;
pub mod folder;
pub mod timing_tower;
pub mod variables;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct StyleDefinition {
    pub id: Uuid,
    pub assets: Folder<AssetDefinition>,
    pub vars: Folder<VariableDefinition>,
    pub timing_tower: TimingTower,
}
