use bevy_egui::egui::Ui;
use tree_view::TreeViewBuilder;
use uuid::Uuid;

use crate::variable_repo::VariableRepo;

use super::TreeNode;

struct Folder<T: TreeNode> {
    id: Uuid,
    name: String,
    content: Vec<FolderOrT<T>>,
}
enum FolderOrT<T: TreeNode> {
    T(T),
    Folder(Folder<T>),
}

impl<T: TreeNode> TreeNode for Folder<T> {
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn TreeNode> {
        if &self.id == id {
            Some(self)
        } else {
            self.content.iter_mut().find_map(|c| c.find_mut(id))
        }
    }

    fn tree_view(&mut self, ui: &mut tree_view::TreeUi) {
        TreeViewBuilder::dir(self.id).show(
            ui,
            |ui| {
                ui.label(&self.name);
            },
            |ui| {
                for c in self.content.iter_mut() {
                    c.tree_view(ui);
                }
            },
        );
    }

    fn property_editor(&mut self, ui: &mut Ui, _vars: &VariableRepo) {
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.name);
        });
    }
}

impl<T: TreeNode> TreeNode for FolderOrT<T> {
    fn find_mut(&mut self, id: &Uuid) -> Option<&mut dyn TreeNode> {
        match self {
            FolderOrT::T(o) => o.find_mut(id),
            FolderOrT::Folder(o) => o.find_mut(id),
        }
    }

    fn property_editor(
        &mut self,
        ui: &mut bevy_egui::egui::Ui,
        vars: &crate::variable_repo::VariableRepo,
    ) {
        match self {
            FolderOrT::T(o) => o.property_editor(ui, vars),
            FolderOrT::Folder(o) => o.property_editor(ui, vars),
        }
    }

    fn tree_view(&mut self, ui: &mut tree_view::TreeUi) {
        match self {
            FolderOrT::T(o) => o.tree_view(ui),
            FolderOrT::Folder(o) => o.tree_view(ui),
        }
    }
}
