use std::ops::ControlFlow;

use backend::style::{
    definitions::*,
    visitor::{NodeVisitorMut, StyleNode},
};
use bevy_egui::egui::Ui;
use egui_ltreeview::{DropPosition, TreeViewBuilder, TreeViewResponse};
use uuid::Uuid;

pub struct TreeViewVisitorResult {
    pub response: TreeViewResponse,
    pub nodes_to_add: Vec<(Uuid, DropPosition, Box<dyn StyleNode + Sync + Send>)>,
    pub nodes_to_remove: Vec<Uuid>,
}
pub struct TreeViewVisitor<'a> {
    builder: TreeViewBuilder<'a>,
    nodes_to_add: &'a mut Vec<(Uuid, DropPosition, Box<dyn StyleNode + Sync + Send>)>,
    nodes_to_remove: &'a mut Vec<Uuid>,
    stack: Vec<Uuid>,
}
impl TreeViewVisitor<'_> {
    pub fn show(ui: &mut Ui, style_node: &mut dyn StyleNode) -> TreeViewVisitorResult {
        let mut nodes_to_add = Vec::new();
        let mut nodes_to_remove = Vec::new();
        let response = egui_ltreeview::TreeViewBuilder::new(
            ui,
            ui.make_persistent_id("element_tree_view"),
            |root| {
                style_node.walk_mut(&mut TreeViewVisitor {
                    builder: root,
                    nodes_to_add: &mut nodes_to_add,
                    nodes_to_remove: &mut nodes_to_remove,
                    stack: Vec::new(),
                });
            },
        );
        TreeViewVisitorResult {
            response,
            nodes_to_add,
            nodes_to_remove,
        }
    }
}
impl NodeVisitorMut for TreeViewVisitor<'_> {
    fn visit_style(&mut self, style: &mut StyleDefinition) -> ControlFlow<()> {
        self.stack.push(style.id);
        self.builder.dir(&style.id, |ui| {
            ui.label("Style");
        });
        ControlFlow::Continue(())
    }

    fn leave_style(&mut self, _style: &mut StyleDefinition) -> ControlFlow<()> {
        self.stack.pop();
        self.builder.close_dir();
        ControlFlow::Continue(())
    }

    fn visit_folder(&mut self, folder: &mut dyn FolderInfo) -> ControlFlow<()> {
        self.stack.push(*folder.id());
        let res = self.builder.dir(&folder.id(), |ui| {
            ui.label(folder.name());
        });
        if let Some(res) = res {
            res.context_menu(|ui| {
                if ui.button("delete").clicked() {
                    self.nodes_to_remove.push(*folder.id());
                    ui.close_menu();
                }
            });
        }

        ControlFlow::Continue(())
    }

    fn leave_folder(&mut self, _folder: &mut dyn FolderInfo) -> ControlFlow<()> {
        self.stack.pop();
        self.builder.close_dir();
        ControlFlow::Continue(())
    }

    fn visit_timing_tower(&mut self, tower: &mut TimingTower) -> ControlFlow<()> {
        self.stack.push(tower.id);
        self.builder.dir(&tower.id, |ui| {
            ui.label("Timing tower");
        });
        ControlFlow::Continue(())
    }

    fn leave_timing_tower(&mut self, _tower: &mut TimingTower) -> ControlFlow<()> {
        self.stack.pop();
        self.builder.close_dir();
        ControlFlow::Continue(())
    }

    fn visit_timing_tower_row(&mut self, row: &mut TimingTowerRow) -> ControlFlow<()> {
        self.stack.push(row.id);
        let res = self.builder.dir(&row.id, |ui| {
            ui.label("Row");
        });

        if let Some(res) = res {
            res.context_menu(|ui| {
                if ui.button("add column").clicked() {
                    self.nodes_to_add.push((
                        row.id,
                        DropPosition::Last,
                        Box::new(TimingTowerColumn::new()),
                    ));
                    ui.close_menu();
                }
                if ui.button("add group").clicked() {
                    self.nodes_to_add.push((
                        row.id,
                        DropPosition::Last,
                        Box::new(TimingTowerColumnFolder::new()),
                    ));
                    ui.close_menu();
                }
            });
        }

        ControlFlow::Continue(())
    }

    fn leave_timing_tower_row(&mut self, _row: &mut TimingTowerRow) -> ControlFlow<()> {
        self.stack.pop();
        self.builder.close_dir();
        ControlFlow::Continue(())
    }

    fn visit_timing_tower_column(&mut self, column: &mut TimingTowerColumn) -> ControlFlow<()> {
        let res = self.builder.leaf(&column.id, |ui| {
            ui.label(&column.name);
        });
        if let Some(res) = res {
            res.context_menu(|ui| {
                if ui.button("add column").clicked() {
                    self.nodes_to_add.push((
                        *self
                            .stack
                            .last()
                            .expect("There should always be a parent node"),
                        DropPosition::After(column.id),
                        Box::new(TimingTowerColumn::new()),
                    ));
                    ui.close_menu();
                }
                if ui.button("add group").clicked() {
                    self.nodes_to_add.push((
                        *self
                            .stack
                            .last()
                            .expect("There should always be a parent node"),
                        DropPosition::Last,
                        Box::new(TimingTowerColumnFolder::new()),
                    ));
                    ui.close_menu();
                }
                if ui.button("delete").clicked() {
                    self.nodes_to_remove.push(column.id);
                    ui.close_menu();
                }
            });
        }
        ControlFlow::Continue(())
    }

    fn visit_timing_tower_column_folder(
        &mut self,
        folder: &mut TimingTowerColumnFolder,
    ) -> ControlFlow<()> {
        let res = self.builder.dir(&folder.id, |ui| {
            ui.label(&folder.name);
        });
        if let Some(res) = res {
            res.context_menu(|ui| {
                if ui.button("add column").clicked() {
                    self.nodes_to_add.push((
                        *folder.id(),
                        DropPosition::Last,
                        Box::new(TimingTowerColumn::new()),
                    ));
                    ui.close_menu();
                }
                if ui.button("add group").clicked() {
                    self.nodes_to_add.push((
                        *folder.id(),
                        DropPosition::Last,
                        Box::new(TimingTowerColumnFolder::new()),
                    ));
                    ui.close_menu();
                }
            });
        }
        ControlFlow::Continue(())
    }

    fn leave_timing_tower_column_folder(
        &mut self,
        _folder: &mut TimingTowerColumnFolder,
    ) -> ControlFlow<()> {
        self.builder.close_dir();
        ControlFlow::Continue(())
    }

    fn visit_asset(&mut self, asset: &mut AssetDefinition) -> ControlFlow<()> {
        let res = self.builder.leaf(&asset.id, |ui| {
            ui.label(&asset.name);
        });
        if let Some(res) = res {
            res.context_menu(|ui| {
                if ui.button("add image").clicked() {
                    self.nodes_to_add.push((
                        *self
                            .stack
                            .last()
                            .expect("There should always be a parent node"),
                        DropPosition::After(asset.id),
                        Box::new(AssetDefinition::new()),
                    ));
                    ui.close_menu();
                }
                if ui.button("add group").clicked() {
                    self.nodes_to_add.push((
                        *self
                            .stack
                            .last()
                            .expect("There should always be a parent node"),
                        DropPosition::Last,
                        Box::new(Folder::<AssetDefinition>::new()),
                    ));
                    ui.close_menu();
                }
                if ui.button("delete").clicked() {
                    self.nodes_to_remove.push(asset.id);
                    ui.close_menu();
                }
            });
        }
        ControlFlow::Continue(())
    }

    fn visit_asset_folder(&mut self, folder: &mut AssetFolder) -> ControlFlow<()> {
        let res = self.builder.dir(&folder.id, |ui| {
            ui.label(&folder.name);
        });
        if let Some(res) = res {
            res.context_menu(|ui| {
                if ui.button("add image").clicked() {
                    self.nodes_to_add.push((
                        *folder.id(),
                        DropPosition::Last,
                        Box::new(AssetDefinition::new()),
                    ));
                    ui.close_menu();
                }
                if ui.button("add group").clicked() {
                    self.nodes_to_add.push((
                        *folder.id(),
                        DropPosition::Last,
                        Box::new(AssetFolder::new()),
                    ));
                    ui.close_menu();
                }
            });
        }
        ControlFlow::Continue(())
    }

    fn leave_asset_folder(&mut self, _folder: &mut AssetFolder) -> ControlFlow<()> {
        self.builder.close_dir();
        ControlFlow::Continue(())
    }

    fn visit_variable(&mut self, variable: &mut VariableDefinition) -> ControlFlow<()> {
        let res = self.builder.leaf(&variable.id, |ui| {
            ui.label(&variable.name);
        });
        if let Some(res) = res {
            res.context_menu(|ui| {
                if ui.button("add variable").clicked() {
                    self.nodes_to_add.push((
                        *self
                            .stack
                            .last()
                            .expect("There should always be a parent node"),
                        DropPosition::After(variable.id),
                        Box::new(VariableDefinition::new()),
                    ));
                    ui.close_menu();
                }
                if ui.button("add group").clicked() {
                    self.nodes_to_add.push((
                        *self
                            .stack
                            .last()
                            .expect("There should always be a parent node"),
                        DropPosition::Last,
                        Box::new(Folder::<VariableDefinition>::new()),
                    ));
                    ui.close_menu();
                }
                if ui.button("delete").clicked() {
                    self.nodes_to_remove.push(variable.id);
                    ui.close_menu();
                }
            });
        }
        ControlFlow::Continue(())
    }

    fn visit_variable_folder(&mut self, folder: &mut VariableFolder) -> ControlFlow<()> {
        let res = self.builder.dir(&folder.id, |ui| {
            ui.label(&folder.name);
        });
        if let Some(res) = res {
            res.context_menu(|ui| {
                if ui.button("add variable").clicked() {
                    self.nodes_to_add.push((
                        *folder.id(),
                        DropPosition::Last,
                        Box::new(VariableDefinition::new()),
                    ));
                    ui.close_menu();
                }
                if ui.button("add group").clicked() {
                    self.nodes_to_add.push((
                        *folder.id(),
                        DropPosition::Last,
                        Box::new(VariableFolder::new()),
                    ));
                    ui.close_menu();
                }
            });
        }
        ControlFlow::Continue(())
    }

    fn leave_variable_folder(&mut self, _folder: &mut VariableFolder) -> ControlFlow<()> {
        self.builder.close_dir();
        ControlFlow::Continue(())
    }

    fn visit_scene(&mut self, scene: &mut SceneDefinition) -> ControlFlow<()> {
        self.builder.dir(&scene.id, |ui| {
            ui.label("Scene");
        });
        ControlFlow::Continue(())
    }

    fn leave_scene(&mut self, _scene: &mut SceneDefinition) -> ControlFlow<()> {
        self.builder.close_dir();
        ControlFlow::Continue(())
    }
}
