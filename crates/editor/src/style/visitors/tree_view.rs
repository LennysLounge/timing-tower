use std::ops::ControlFlow;

use backend::style::{
    definitions::*,
    visitor::{Method, NodeIteratorMut, NodeMut, NodeVisitorMut},
    StyleNode,
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
                style_node.as_node_mut().walk_mut(&mut TreeViewVisitor {
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
    fn visit(&mut self, node: NodeMut, method: Method) -> ControlFlow<()> {
        match (method, node) {
            (Method::Visit, NodeMut::Style(style)) => {
                self.stack.push(style.id);
                self.builder.dir(&style.id, |ui| {
                    ui.label("Style");
                });
                ControlFlow::Continue(())
            }
            (Method::Leave, NodeMut::Style(_)) => {
                self.stack.pop();
                self.builder.close_dir();
                ControlFlow::Continue(())
            }

            (Method::Visit, NodeMut::TimingTower(tower)) => {
                self.stack.push(tower.id);
                self.builder.dir(&tower.id, |ui| {
                    ui.label("Timing tower");
                });
                ControlFlow::Continue(())
            }
            (Method::Leave, NodeMut::TimingTower(_)) => {
                self.stack.pop();
                self.builder.close_dir();
                ControlFlow::Continue(())
            }

            (Method::Visit, NodeMut::TimingTowerRow(row)) => {
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
            (Method::Leave, NodeMut::TimingTowerRow(_)) => {
                self.stack.pop();
                self.builder.close_dir();
                ControlFlow::Continue(())
            }

            (Method::Visit, NodeMut::TimingTowerColumn(column)) => {
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

            (Method::Visit, NodeMut::TimingTowerColumnFolder(folder)) => {
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
            (Method::Leave, NodeMut::TimingTowerColumnFolder(_)) => {
                self.builder.close_dir();
                ControlFlow::Continue(())
            }

            (Method::Visit, NodeMut::Asset(asset)) => {
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
                                Box::new(AssetFolder::new()),
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

            (Method::Visit, NodeMut::AssetFolder(folder)) => {
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
            (Method::Leave, NodeMut::AssetFolder(_)) => {
                self.builder.close_dir();
                ControlFlow::Continue(())
            }

            (Method::Visit, NodeMut::Variable(variable)) => {
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
                                Box::new(VariableFolder::new()),
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

            (Method::Visit, NodeMut::VariableFolder(folder)) => {
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
            (Method::Leave, NodeMut::VariableFolder(_)) => {
                self.builder.close_dir();
                ControlFlow::Continue(())
            }

            (Method::Visit, NodeMut::Scene(scene)) => {
                self.builder.dir(&scene.id, |ui| {
                    ui.label("Scene");
                });
                ControlFlow::Continue(())
            }
            (Method::Leave, NodeMut::Scene(_)) => {
                self.builder.close_dir();
                ControlFlow::Continue(())
            }

            (Method::Visit, NodeMut::ClipArea(clip_area)) => {
                self.builder.dir(clip_area.id(), |ui| {
                    ui.label("Clip area");
                });
                ControlFlow::Continue(())
            }
            (Method::Leave, NodeMut::ClipArea(_)) => {
                self.builder.close_dir();
                ControlFlow::Continue(())
            }

            _ => ControlFlow::Continue(()),
        }
    }
}
