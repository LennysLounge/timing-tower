use std::any::{Any, TypeId};

use bevy_egui::egui::Ui;
use serde::{Deserialize, Serialize};
use tree_view::{DropPosition, TreeUi, TreeViewBuilder};
use uuid::Uuid;

use crate::reference_store::ReferenceStore;

use super::{StyleTreeNode, StyleTreeUi, TreeViewAction};

#[derive(Serialize, Deserialize, Clone)]
pub struct Folder<T: StyleTreeNode + FolderActions<FolderType = T>> {
    pub id: Uuid,
    pub name: String,
    pub content: Vec<FolderOrT<T>>,
    renameable: bool,
}

pub trait FolderActions {
    type FolderType: StyleTreeNode + FolderActions<FolderType = Self::FolderType>;
    #[allow(unused)]
    fn context_menu(
        ui: &mut Ui,
        folder: &Folder<Self::FolderType>,
        actions: &mut Vec<TreeViewAction>,
    ) {
    }
}

impl<T: StyleTreeNode + FolderActions<FolderType = T>> StyleTreeUi for Folder<T> {
    fn tree_view(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        let (header, _) = TreeViewBuilder::dir(self.id).show(
            ui,
            |ui| {
                ui.label(format!("🗀  {}", &self.name));
            },
            |ui| {
                for c in self.content.iter_mut() {
                    c.tree_view(ui, actions);
                }
            },
        );
        header.response.context_menu(|ui| {
            T::context_menu(ui, &*self, actions);
        });
    }

    fn property_editor(&mut self, ui: &mut Ui, _asset_repo: &ReferenceStore) -> bool {
        let mut changed = false;
        if self.renameable {
            ui.label("Name:");
            changed |= ui.text_edit_singleline(&mut self.name).changed();
        }
        changed
    }
}

impl<T: StyleTreeNode + FolderActions<FolderType = T>> StyleTreeNode for Folder<T> {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn chidren(&self) -> Vec<&dyn StyleTreeNode> {
        self.content
            .iter()
            .map(|c| c as &dyn StyleTreeNode)
            .collect()
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        self.content
            .iter_mut()
            .map(|c| c as &mut dyn StyleTreeNode)
            .collect()
    }

    fn can_insert(&self, node: &dyn Any) -> bool {
        if TypeId::of::<FolderOrT<T>>() == node.type_id() {
            true
        } else {
            false
        }
    }

    fn remove(&mut self, id: &Uuid) -> Option<Box<dyn Any>> {
        if let Some(pos) = self.content.iter().position(|c| c.id() == id) {
            Some(Box::new(self.content.remove(pos)))
        } else {
            None
        }
    }

    fn insert(&mut self, node: Box<dyn Any>, position: &DropPosition) {
        if node.is::<FolderOrT<T>>() {
            let node = node.downcast::<FolderOrT<T>>().expect("Type ids match");
            self.insert_at(*node, position);
        } else if node.is::<T>() {
            let node = node.downcast::<T>().expect("Type ids match");
            self.insert_at(FolderOrT::T(*node), position);
        } else if node.is::<Folder<T>>() {
            let node = node.downcast::<Folder<T>>().expect("Type ids match");
            self.insert_at(FolderOrT::Folder(*node), position);
        }
    }
}

impl<T: StyleTreeNode + FolderActions<FolderType = T>> Folder<T> {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "new group".to_string(),
            content: Vec::new(),
            renameable: true,
        }
    }
    /// Get a reference to all contained things of this folder.
    pub fn all_t(&self) -> Vec<&T> {
        self.content.iter().flat_map(|c| c.all_t()).collect()
    }

    /// Get a reference to all contained things of this folder.
    pub fn all_t_mut(&mut self) -> Vec<&mut T> {
        self.content
            .iter_mut()
            .flat_map(|c| c.all_t_mut())
            .collect()
    }

    /// Show the contents of this folder without a collapsing header.
    pub fn tree_view_flat(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        TreeViewBuilder::dir(self.id).headless().show(ui, |ui| {
            for c in self.content.iter_mut() {
                c.tree_view(ui, actions);
            }
        });
    }

    fn insert_at(&mut self, node: FolderOrT<T>, position: &DropPosition) {
        match position {
            DropPosition::First => self.content.insert(0, node),
            DropPosition::Last => self.content.push(node),
            DropPosition::After(id) => {
                let pos = self
                    .content
                    .iter()
                    .position(|c| c.id() == id)
                    .unwrap_or(self.content.len());
                self.content.insert(pos + 1, node);
            }
            DropPosition::Before(id) => {
                let pos = self
                    .content
                    .iter()
                    .position(|c| c.id() == id)
                    .unwrap_or(self.content.len());
                self.content.insert(pos, node);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum FolderOrT<T: StyleTreeNode + FolderActions<FolderType = T>> {
    T(T),
    Folder(Folder<T>),
}
impl<T: StyleTreeNode + FolderActions<FolderType = T>> StyleTreeUi for FolderOrT<T> {
    fn tree_view(&mut self, ui: &mut TreeUi, actions: &mut Vec<TreeViewAction>) {
        match self {
            FolderOrT::T(o) => o.tree_view(ui, actions),
            FolderOrT::Folder(o) => o.tree_view(ui, actions),
        }
    }

    fn property_editor(&mut self, ui: &mut Ui, asset_repo: &ReferenceStore) -> bool {
        match self {
            FolderOrT::T(o) => o.property_editor(ui, asset_repo),
            FolderOrT::Folder(o) => o.property_editor(ui, asset_repo),
        }
    }
}

impl<T: StyleTreeNode + FolderActions<FolderType = T>> StyleTreeNode for FolderOrT<T> {
    fn id(&self) -> &Uuid {
        match self {
            FolderOrT::T(o) => o.id(),
            FolderOrT::Folder(o) => o.id(),
        }
    }

    fn chidren(&self) -> Vec<&dyn StyleTreeNode> {
        match self {
            FolderOrT::T(o) => o.chidren(),
            FolderOrT::Folder(o) => o.chidren(),
        }
    }

    fn chidren_mut(&mut self) -> Vec<&mut dyn StyleTreeNode> {
        match self {
            FolderOrT::T(o) => o.chidren_mut(),
            FolderOrT::Folder(o) => o.chidren_mut(),
        }
    }

    fn can_insert(&self, node: &dyn Any) -> bool {
        match self {
            FolderOrT::T(o) => o.can_insert(node),
            FolderOrT::Folder(o) => o.can_insert(node),
        }
    }

    fn remove(&mut self, id: &Uuid) -> Option<Box<dyn Any>> {
        match self {
            FolderOrT::T(o) => o.remove(id),
            FolderOrT::Folder(o) => o.remove(id),
        }
    }

    fn insert(&mut self, node: Box<dyn Any>, position: &DropPosition) {
        match self {
            FolderOrT::T(o) => o.insert(node, position),
            FolderOrT::Folder(o) => o.insert(node, position),
        }
    }
}
impl<T: StyleTreeNode + FolderActions<FolderType = T>> FolderOrT<T> {
    pub fn all_t(&self) -> Vec<&T> {
        match self {
            FolderOrT::T(t) => vec![t],
            FolderOrT::Folder(f) => f.all_t(),
        }
    }
    pub fn all_t_mut(&mut self) -> Vec<&mut T> {
        match self {
            FolderOrT::T(t) => vec![t],
            FolderOrT::Folder(f) => f.all_t_mut(),
        }
    }
}
