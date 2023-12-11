use std::any::Any;

use backend::style::StyleDefinition;
use bevy_egui::egui::Ui;
use tree_view::DropPosition;
use uuid::Uuid;

use crate::reference_store::ReferenceStore;

use self::tree::{StyleTreeNode, StyleTreeUi, TreeViewAction};

pub mod assets;
pub mod cell;
pub mod folder;
pub mod timing_tower;
pub mod tree;
pub mod variables;
pub mod visitors {
    pub mod drop_allowed;
    pub mod insert;
    pub mod remove;
    pub mod search;
    pub mod tree_view;
}

pub struct StyleModel {
    pub def: StyleDefinition,
}
impl StyleModel {
    pub fn new(style_def: &StyleDefinition) -> Self {
        Self {
            def: style_def.clone(),
        }
    }
}

trait AttributeEditor {
    fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool;
}

impl StyleTreeUi for StyleModel {}

impl StyleTreeNode for StyleModel {
    fn id(&self) -> &Uuid {
        &self.def.id
    }

    fn chidren(&self) -> Vec<&dyn StyleTreeNode> {
        vec![&self.def.vars, &self.def.timing_tower, &self.def.assets]
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        vec![
            &mut self.def.vars,
            &mut self.def.timing_tower,
            &mut self.def.assets,
        ]
    }

    fn can_insert(&self, _node: &dyn Any) -> bool {
        false
    }

    fn remove(&mut self, _id: &Uuid) -> Option<Box<dyn Any>> {
        None
    }

    fn insert(&mut self, _node: Box<dyn Any>, _position: &DropPosition) {}
}
