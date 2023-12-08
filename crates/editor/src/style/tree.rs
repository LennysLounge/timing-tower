use std::any::Any;

use bevy_egui::egui::Ui;
use tree_view::{DropPosition, TreeUi};
use uuid::Uuid;

use crate::reference_store::ReferenceStore;

/// Actions that a tree view can produce.
pub enum TreeViewAction {
    Insert {
        target: Uuid,
        node: Box<dyn Any>,
        position: DropPosition,
    },
    Remove {
        node: Uuid,
    },
    Select {
        node: Uuid,
    },
}

pub trait StyleTreeUi {
    #[allow(unused)]
    /// Display the property editor for this tree node and return true if any fields where changed
    /// Returns falls if no changes were made to the node.
    fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        false
    }
    /// Display the tree view node of this node.
    fn tree_view(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>);
}

/// Trait to upcast or change the trait object type.
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

/// Actions a node in the tree view must be able to perform
pub trait StyleTreeNode: StyleTreeNodeConversions + StyleTreeUi {
    fn id(&self) -> &Uuid;
    fn chidren(&self) -> Vec<&dyn StyleTreeNode>;
    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode>;

    fn can_insert(&self, node: &dyn Any) -> bool;
    fn remove(&mut self, id: &Uuid) -> Option<Box<dyn Any>>;
    fn insert(&mut self, node: Box<dyn Any>, position: &DropPosition);

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