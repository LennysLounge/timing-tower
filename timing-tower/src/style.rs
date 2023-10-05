use std::any::Any;

use bevy::prelude::Resource;
use bevy_egui::egui::Ui;
use serde::{Deserialize, Serialize};
use tree_view::{DropPosition, TreeUi};
use uuid::Uuid;

use crate::variable_repo::VariableRepo;

use self::{timing_tower::TimingTower, variables::Variables};

pub mod cell;
pub mod properties;
pub mod timing_tower;
pub mod variables;

pub trait StyleTreeUi {
    #[allow(unused)]
    fn property_editor(&mut self, ui: &mut Ui, vars: &VariableRepo) {}
    fn tree_view(&mut self, ui: &mut TreeUi);
}

pub trait StyleTreeNodeConversions: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_dyn_mut(&mut self) -> &mut dyn StyleTreeNode;
}
impl<T: StyleTreeNode + Any> StyleTreeNodeConversions for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_dyn_mut(&mut self) -> &mut dyn StyleTreeNode {
        self
    }
}

pub trait StyleTreeNode: StyleTreeNodeConversions + StyleTreeUi {
    fn id(&self) -> &Uuid;
    fn chidren(&self) -> Vec<&dyn StyleTreeNode>;
    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode>;

    #[allow(unused)]
    fn can_insert(&self, node: &dyn Any) -> bool {
        false
    }
    #[allow(unused)]
    fn remove(&mut self, id: &Uuid) -> Option<Box<dyn Any>> {
        None
    }
    #[allow(unused)]
    fn insert(&mut self, node: Box<dyn Any>, position: DropPosition) {}

    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn StyleTreeNode> {
        self.chidren_mut().into_iter().find_map(|c| {
            if c.id() == id {
                Some(c)
            } else {
                c.find_mut(id)
            }
        })
    }
    fn find(&self, id: &Uuid) -> Option<&dyn StyleTreeNode> {
        self.chidren()
            .into_iter()
            .find_map(|c| if c.id() == id { Some(c) } else { c.find(id) })
    }
    fn find_parent_of(&mut self, id: &Uuid) -> Option<&mut dyn StyleTreeNode> {
        if self.chidren().into_iter().any(|c| c.id() == id) {
            Some(self.as_dyn_mut())
        } else {
            self.chidren_mut()
                .into_iter()
                .find_map(|c| c.find_parent_of(id))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Resource)]
pub struct StyleDefinition {
    pub id: Uuid,
    pub vars: Variables,
    pub timing_tower: TimingTower,
}

impl StyleTreeUi for StyleDefinition {
    fn tree_view(&mut self, ui: &mut TreeUi) {
        self.vars.tree_view(ui);
        self.timing_tower.tree_view(ui);
    }
}

impl StyleTreeNode for StyleDefinition {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn chidren(&self) -> Vec<&dyn StyleTreeNode> {
        vec![&self.vars, &self.timing_tower]
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        vec![&mut self.vars, &mut self.timing_tower]
    }
}
