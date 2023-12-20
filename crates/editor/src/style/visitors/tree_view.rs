use std::ops::ControlFlow;

use backend::style::{
    definitions::*,
    visitor::{Method, NodeIteratorMut, NodeMut},
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
pub fn show(ui: &mut Ui, style_node: &mut dyn StyleNode) -> TreeViewVisitorResult {
    let mut nodes_to_add = Vec::new();
    let mut nodes_to_remove = Vec::new();
    let mut stack: Vec<Uuid> = Vec::new();
    let response = egui_ltreeview::TreeViewBuilder::new(
        ui,
        ui.make_persistent_id("element_tree_view"),
        |mut root| {
            style_node.as_node_mut().walk_mut(&mut |node, method| {
                tree_view(
                    node,
                    method,
                    &mut root,
                    &mut nodes_to_add,
                    &mut nodes_to_remove,
                    &mut stack,
                )
            });
        },
    );
    TreeViewVisitorResult {
        response,
        nodes_to_add,
        nodes_to_remove,
    }
}
fn tree_view(
    node: NodeMut,
    method: Method,
    builder: &mut TreeViewBuilder,
    nodes_to_add: &mut Vec<(Uuid, DropPosition, Box<dyn StyleNode + Sync + Send>)>,
    nodes_to_remove: &mut Vec<Uuid>,
    stack: &mut Vec<Uuid>,
) -> ControlFlow<()> {
    match (method, node) {
        (Method::Visit, NodeMut::Style(style)) => {
            stack.push(style.id);
            builder.dir(&style.id, |ui| {
                ui.label("Style");
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, NodeMut::Style(_)) => {
            stack.pop();
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::TimingTower(tower)) => {
            stack.push(tower.id);
            builder.dir(&tower.id, |ui| {
                ui.label("Timing tower");
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, NodeMut::TimingTower(_)) => {
            stack.pop();
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::TimingTowerRow(row)) => {
            stack.push(row.id);
            let res = builder.dir(&row.id, |ui| {
                ui.label("Row");
            });

            if let Some(res) = res {
                res.context_menu(|ui| {
                    if ui.button("add column").clicked() {
                        nodes_to_add.push((
                            row.id,
                            DropPosition::Last,
                            Box::new(TimingTowerColumn::new()),
                        ));
                        ui.close_menu();
                    }
                    if ui.button("add group").clicked() {
                        nodes_to_add.push((
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
            stack.pop();
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::TimingTowerColumn(column)) => {
            let res = builder.leaf(&column.id, |ui| {
                ui.label(&column.name);
            });
            if let Some(res) = res {
                res.context_menu(|ui| {
                    if ui.button("add column").clicked() {
                        nodes_to_add.push((
                            *stack.last().expect("There should always be a parent node"),
                            DropPosition::After(column.id),
                            Box::new(TimingTowerColumn::new()),
                        ));
                        ui.close_menu();
                    }
                    if ui.button("add group").clicked() {
                        nodes_to_add.push((
                            *stack.last().expect("There should always be a parent node"),
                            DropPosition::Last,
                            Box::new(TimingTowerColumnFolder::new()),
                        ));
                        ui.close_menu();
                    }
                    if ui.button("delete").clicked() {
                        nodes_to_remove.push(column.id);
                        ui.close_menu();
                    }
                });
            }
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::TimingTowerColumnFolder(folder)) => {
            let res = builder.dir(&folder.id, |ui| {
                ui.label(&folder.name);
            });
            if let Some(res) = res {
                res.context_menu(|ui| {
                    if ui.button("add column").clicked() {
                        nodes_to_add.push((
                            *folder.id(),
                            DropPosition::Last,
                            Box::new(TimingTowerColumn::new()),
                        ));
                        ui.close_menu();
                    }
                    if ui.button("add group").clicked() {
                        nodes_to_add.push((
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
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::Asset(asset)) => {
            let res = builder.leaf(&asset.id, |ui| {
                ui.label(&asset.name);
            });
            if let Some(res) = res {
                res.context_menu(|ui| {
                    if ui.button("add image").clicked() {
                        nodes_to_add.push((
                            *stack.last().expect("There should always be a parent node"),
                            DropPosition::After(asset.id),
                            Box::new(AssetDefinition::new()),
                        ));
                        ui.close_menu();
                    }
                    if ui.button("add group").clicked() {
                        nodes_to_add.push((
                            *stack.last().expect("There should always be a parent node"),
                            DropPosition::Last,
                            Box::new(AssetFolder::new()),
                        ));
                        ui.close_menu();
                    }
                    if ui.button("delete").clicked() {
                        nodes_to_remove.push(asset.id);
                        ui.close_menu();
                    }
                });
            }
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::AssetFolder(folder)) => {
            let res = builder.dir(&folder.id, |ui| {
                ui.label(&folder.name);
            });
            if let Some(res) = res {
                res.context_menu(|ui| {
                    if ui.button("add image").clicked() {
                        nodes_to_add.push((
                            *folder.id(),
                            DropPosition::Last,
                            Box::new(AssetDefinition::new()),
                        ));
                        ui.close_menu();
                    }
                    if ui.button("add group").clicked() {
                        nodes_to_add.push((
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
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::Variable(variable)) => {
            let res = builder.leaf(&variable.id, |ui| {
                ui.label(&variable.name);
            });
            if let Some(res) = res {
                res.context_menu(|ui| {
                    if ui.button("add variable").clicked() {
                        nodes_to_add.push((
                            *stack.last().expect("There should always be a parent node"),
                            DropPosition::After(variable.id),
                            Box::new(VariableDefinition::new()),
                        ));
                        ui.close_menu();
                    }
                    if ui.button("add group").clicked() {
                        nodes_to_add.push((
                            *stack.last().expect("There should always be a parent node"),
                            DropPosition::Last,
                            Box::new(VariableFolder::new()),
                        ));
                        ui.close_menu();
                    }
                    if ui.button("delete").clicked() {
                        nodes_to_remove.push(variable.id);
                        ui.close_menu();
                    }
                });
            }
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::VariableFolder(folder)) => {
            let res = builder.dir(&folder.id, |ui| {
                ui.label(&folder.name);
            });
            if let Some(res) = res {
                res.context_menu(|ui| {
                    if ui.button("add variable").clicked() {
                        nodes_to_add.push((
                            *folder.id(),
                            DropPosition::Last,
                            Box::new(VariableDefinition::new()),
                        ));
                        ui.close_menu();
                    }
                    if ui.button("add group").clicked() {
                        nodes_to_add.push((
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
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::Scene(scene)) => {
            builder.dir(&scene.id, |ui| {
                ui.label("Scene");
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, NodeMut::Scene(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::ClipArea(clip_area)) => {
            builder.dir(clip_area.id(), |ui| {
                ui.label("Clip area");
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, NodeMut::ClipArea(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        _ => ControlFlow::Continue(()),
    }
}
