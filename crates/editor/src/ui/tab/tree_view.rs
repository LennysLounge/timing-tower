use std::ops::ControlFlow;

use backend::{
    style::{
        assets::{AssetDefinition, AssetFolder},
        cell::{FreeCell, FreeCellFolder},
        component::Component,
        variables::{VariableDefinition, VariableFolder},
        Node, NodeMut, StyleNode,
    },
    tree_iterator::{Method, TreeIterator, TreeIteratorMut},
};
use bevy_egui::egui::{self, ScrollArea, Ui};
use egui_ltreeview::{
    builder::{CloserState, NodeBuilder},
    Action, DropPosition, TreeViewBuilder, TreeViewResponse,
};
use uuid::Uuid;

use crate::command::{
    insert_node::InsertNode, move_node::MoveNode, remove_node::RemoveNode, UndoRedoManager,
};

pub fn tree_view(
    ui: &mut Ui,
    selected_node: &mut Option<Uuid>,
    base_node: &mut impl StyleNode,
    undo_redo_manager: &mut UndoRedoManager,
) -> bool {
    let mut changed = false;
    let response = ScrollArea::vertical()
        .show(ui, |ui| {
            show(ui, base_node.as_node_mut(), undo_redo_manager)
        })
        .inner;

    for action in response.actions.iter() {
        match action {
            Action::SetSelected(id) => *selected_node = *id,
            a @ Action::Move {
                source,
                target,
                position,
            }
            | a @ Action::Drag {
                source,
                target,
                position,
            } => {
                let drop_allowed = base_node
                    .as_node()
                    .search(*source, |dragged| {
                        base_node
                            .as_node()
                            .search(*target, |dropped| drop_allowed(dropped, dragged))
                    })
                    .flatten()
                    .unwrap_or(false);
                if !drop_allowed {
                    response.remove_drop_marker(ui);
                }
                if let Action::Move { .. } = a {
                    undo_redo_manager.queue(MoveNode {
                        id: *source,
                        target_id: *target,
                        position: *position,
                    });
                    changed = true;
                }
            }
        }
    }
    changed
}

fn drop_allowed(target: &Node, dragged: &Node) -> bool {
    match (target, dragged) {
        (Node::VariableFolder(_), Node::VariableFolder(_)) => true,
        (Node::VariableFolder(_), Node::Variable(_)) => true,

        (Node::AssetFolder(_), Node::AssetFolder(_)) => true,
        (Node::AssetFolder(_), Node::Asset(_)) => true,

        (Node::TimingTowerRow(_), Node::FreeCellFolder(_)) => true,
        (Node::TimingTowerRow(_), Node::FreeCell(_)) => true,

        (Node::FreeCellFolder(_), Node::FreeCellFolder(_)) => true,
        (Node::FreeCellFolder(_), Node::FreeCell(_)) => true,

        (Node::TimingTower(_), Node::FreeCell(_)) => true,
        (Node::TimingTower(_), Node::FreeCellFolder(_)) => true,

        (Node::Scene(_), Node::Component(_)) => true,

        _ => false,
    }
}

fn show(
    ui: &mut Ui,
    mut root: NodeMut,
    undo_redo_manager: &mut UndoRedoManager,
) -> TreeViewResponse<Uuid> {
    let response = egui_ltreeview::TreeView::new(ui.make_persistent_id("element_tree_view"))
        .row_layout(egui_ltreeview::RowLayout::CompactAlignedLables)
        .show(ui, |mut builder| {
            root.walk_mut(&mut |node, method| show_node(node, method, &mut builder));
        });

    response.context_menu(ui, |ui, node_id| {
        root.search_mut(node_id, |node| {
            context_menu(ui, node, undo_redo_manager, &response);
        });
    });

    response
}
fn show_node(
    node: &mut NodeMut,
    method: Method,
    builder: &mut TreeViewBuilder<Uuid>,
) -> ControlFlow<()> {
    match (method, node) {
        (Method::Visit, NodeMut::Style(style)) => {
            builder.node(NodeBuilder::dir(style.id), |ui| {
                ui.label("Style");
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, NodeMut::Style(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::TimingTower(tower)) => {
            builder.node(NodeBuilder::dir(tower.id).closer(folder_closer), |ui| {
                ui.label("Timing tower");
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, NodeMut::TimingTower(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::TimingTowerRow(row)) => {
            builder.node(NodeBuilder::dir(row.id).closer(folder_closer), |ui| {
                ui.label("Row");
            });
            ControlFlow::Continue(())
        }

        (Method::Leave, NodeMut::TimingTowerRow(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::FreeCell(cell)) => {
            builder.node(
                NodeBuilder::leaf(cell.id).icon(|ui| {
                    egui::Image::new(egui::include_image!("../../../images/article.png"))
                        .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                        .paint_at(ui, ui.max_rect());
                }),
                |ui| {
                    ui.label(&cell.name);
                },
            );
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::FreeCellFolder(folder)) => {
            builder.node(NodeBuilder::dir(folder.id).closer(folder_closer), |ui| {
                ui.label(&folder.name);
            });
            ControlFlow::Continue(())
        }

        (Method::Leave, NodeMut::FreeCellFolder(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::Asset(asset)) => {
            let node_config = NodeBuilder::leaf(asset.id).icon(|ui| {
                match asset.value_type {
                    backend::value_types::ValueType::Texture => {
                        egui::Image::new(egui::include_image!("../../../images/image.png"))
                            .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                            .paint_at(ui, ui.max_rect());
                    }
                    backend::value_types::ValueType::Font => {
                        egui::Image::new(egui::include_image!("../../../images/match_case.png"))
                            .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                            .paint_at(ui, ui.max_rect());
                    }
                    _ => (),
                };
            });
            builder.node(node_config, |ui| {
                ui.label(&asset.name);
            });
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::AssetFolder(folder)) => {
            builder.node(NodeBuilder::dir(folder.id).closer(folder_closer), |ui| {
                ui.label(&folder.name);
            });
            ControlFlow::Continue(())
        }

        (Method::Leave, NodeMut::AssetFolder(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::Variable(variable)) => {
            builder.node(
                NodeBuilder::leaf(variable.id).icon(|ui| {
                    egui::Image::new(egui::include_image!("../../../images/object.png"))
                        .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                        .paint_at(ui, ui.max_rect());
                }),
                |ui| {
                    ui.label(&variable.name);
                },
            );
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::VariableFolder(folder)) => {
            builder.node(NodeBuilder::dir(folder.id).closer(folder_closer), |ui| {
                ui.label(&folder.name);
            });
            ControlFlow::Continue(())
        }

        (Method::Leave, NodeMut::VariableFolder(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::Scene(scene)) => {
            builder.node(NodeBuilder::dir(scene.id).closer(folder_closer), |ui| {
                ui.label("Scene");
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, NodeMut::Scene(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }
        (Method::Visit, NodeMut::Component(comp)) => {
            builder.leaf(comp.id, |ui| {
                ui.label(&comp.name);
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, NodeMut::Variable(_)) => ControlFlow::Continue(()),
        (Method::Leave, NodeMut::Asset(_)) => ControlFlow::Continue(()),
        (Method::Leave, NodeMut::FreeCell(_)) => ControlFlow::Continue(()),
        (Method::Leave, NodeMut::Component(_)) => ControlFlow::Continue(()),
    }
}

fn folder_closer(ui: &mut Ui, state: CloserState) {
    let color = if state.is_hovered {
        ui.visuals().widgets.hovered.fg_stroke.color
    } else {
        ui.visuals().widgets.noninteractive.fg_stroke.color
    };
    if state.is_open {
        egui::Image::new(egui::include_image!("../../../images/folder_open.png"))
            .tint(color)
            .paint_at(ui, ui.max_rect());
    } else {
        egui::Image::new(egui::include_image!("../../../images/folder.png"))
            .tint(color)
            .paint_at(ui, ui.max_rect());
    }
}

fn context_menu(
    ui: &mut Ui,
    node: &mut NodeMut,
    undo_redo_manager: &mut UndoRedoManager,
    tree_response: &TreeViewResponse<Uuid>,
) {
    match node {
        NodeMut::Style(_) => _ = ui.label("Style"),
        NodeMut::Variable(variable) => {
            if ui.button("add variable").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(variable.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(variable.id),
                    node: VariableDefinition::new().to_node(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(variable.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(variable.id),
                    node: VariableFolder::new().to_node(),
                });
                ui.close_menu();
            }
            if ui.button("delete").clicked() {
                undo_redo_manager.queue(RemoveNode { id: variable.id });
                ui.close_menu();
            }
        }
        NodeMut::VariableFolder(folder) => {
            if ui.button("add variable").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: VariableDefinition::new().to_node(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: VariableFolder::new().to_node(),
                });
                ui.close_menu();
            }
        }
        NodeMut::Asset(asset) => {
            if ui.button("add image").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(asset.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(asset.id),
                    node: AssetDefinition::new().to_node(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(asset.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(asset.id),
                    node: AssetFolder::new().to_node(),
                });
                ui.close_menu();
            }
            if ui.button("delete").clicked() {
                undo_redo_manager.queue(RemoveNode { id: asset.id });
                ui.close_menu();
            }
        }
        NodeMut::AssetFolder(folder) => {
            if ui.button("add image").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: AssetDefinition::new().to_node(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: AssetFolder::new().to_node(),
                });
                ui.close_menu();
            }
        }
        NodeMut::Scene(scene) => {
            if ui.button("add component").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: scene.id,
                    position: DropPosition::Last,
                    node: Component::new().to_node(),
                });
                ui.close_menu();
            }
        }
        NodeMut::TimingTower(tower) => {
            if ui.button("add column").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tower.id,
                    position: DropPosition::Last,
                    node: FreeCell::new().to_node(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tower.id,
                    position: DropPosition::Last,
                    node: FreeCellFolder::new().to_node(),
                });
                ui.close_menu();
            }
        }
        NodeMut::TimingTowerRow(row) => {
            if ui.button("add column").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: row.id,
                    position: DropPosition::Last,
                    node: FreeCell::new().to_node(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: row.id,
                    position: DropPosition::Last,
                    node: FreeCellFolder::new().to_node(),
                });
                ui.close_menu();
            }
        }
        NodeMut::FreeCellFolder(folder) => {
            if ui.button("add column").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: FreeCell::new().to_node(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: FreeCellFolder::new().to_node(),
                });
                ui.close_menu();
            }
            if ui.button("delete").clicked() {
                undo_redo_manager.queue(RemoveNode { id: folder.id });
                ui.close_menu();
            }
        }
        NodeMut::FreeCell(cell) => {
            if ui.button("add column").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(cell.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(cell.id),
                    node: FreeCell::new().to_node(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(cell.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(cell.id),
                    node: FreeCellFolder::new().to_node(),
                });
                ui.close_menu();
            }
            if ui.button("delete").clicked() {
                undo_redo_manager.queue(RemoveNode { id: cell.id });
                ui.close_menu();
            }
        }
        NodeMut::Component(comp) => {
            ui.label(&comp.name);
        }
    }
}
