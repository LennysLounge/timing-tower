use std::ops::ControlFlow;

use backend::style::{
    iterator::{Method, Node, NodeIterator, NodeIteratorMut, NodeMut},
    StyleNode,
};
use bevy_egui::egui::{ScrollArea, Ui};
use egui_ltreeview::{DropPosition, TreeViewBuilder, TreeViewResponse};
use uuid::Uuid;

use crate::command::{
    insert_node::InsertNode, move_node::MoveNode, remove_node::RemoveNode, UndoRedoManager,
};

pub fn tree_view(
    ui: &mut Ui,
    _selected_node: &mut Option<Uuid>,
    base_node: &mut impl StyleNode,
    undo_redo_manager: &mut UndoRedoManager,
) -> bool {
    let mut changed = false;
    let TreeViewVisitorResult {
        response,
        nodes_to_add,
        nodes_to_remove,
    } = ScrollArea::vertical()
        .show(ui, |ui| show(ui, base_node))
        .inner;

    // Add nodes
    for (target_node, position, node) in nodes_to_add {
        undo_redo_manager.queue(InsertNode {
            target_node,
            position,
            node,
        });
    }
    // remove nodes
    for id in nodes_to_remove {
        undo_redo_manager.queue(RemoveNode { id });
    }

    if response.selected_node.is_some() {
        *_selected_node = response.selected_node;
    }

    if let Some(drop_action) = &response.drag_drop_action {
        let drop_allowed = base_node
            .as_node()
            .search(&drop_action.drag_id, |dragged| {
                base_node.as_node().search(&drop_action.drop_id, |dropped| {
                    drop_allowed(dropped, dragged)
                })
            })
            .flatten()
            .unwrap_or(false);

        if !drop_allowed {
            response.remove_drop_marker(ui);
        }

        if response.dropped && drop_allowed {
            undo_redo_manager.queue(MoveNode {
                id: drop_action.drag_id,
                target_id: drop_action.drop_id,
                position: drop_action.position,
            });
            changed = true;
        }
    }
    changed
}

fn drop_allowed(target: Node, dragged: Node) -> bool {
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

        _ => false,
    }
}

pub struct TreeViewVisitorResult {
    pub response: TreeViewResponse<Uuid>,
    pub nodes_to_add: Vec<(Uuid, DropPosition<Uuid>, Box<dyn StyleNode + Sync + Send>)>,
    pub nodes_to_remove: Vec<Uuid>,
}
pub fn show(ui: &mut Ui, style_node: &mut dyn StyleNode) -> TreeViewVisitorResult {
    let mut nodes_to_add = Vec::new();
    let mut nodes_to_remove = Vec::new();
    let mut stack: Vec<Uuid> = Vec::new();

    let response = egui_ltreeview::TreeView::new(ui.make_persistent_id("element_tree_view")).show(
        ui,
        |mut builder| {
            style_node.as_node_mut().walk_mut(&mut |node, method| {
                show_node(
                    node,
                    method,
                    &mut builder,
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
fn show_node(
    node: NodeMut,
    method: Method,
    builder: &mut TreeViewBuilder<Uuid>,
    _nodes_to_add: &mut Vec<(Uuid, DropPosition<Uuid>, Box<dyn StyleNode + Sync + Send>)>,
    _nodes_to_remove: &mut Vec<Uuid>,
    stack: &mut Vec<Uuid>,
) -> ControlFlow<()> {
    match (method, node) {
        (Method::Visit, NodeMut::Style(style)) => {
            stack.push(style.id);
            builder.dir(style.id, |ui| {
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
            builder.dir(tower.id, |ui| {
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
            builder.dir(row.id, |ui| {
                ui.label("Row");
            });

            // if let Some(res) = res {
            //     res.context_menu(|ui| {
            //         if ui.button("add column").clicked() {
            //             nodes_to_add.push((row.id, DropPosition::Last, Box::new(FreeCell::new())));
            //             ui.close_menu();
            //         }
            //         if ui.button("add group").clicked() {
            //             nodes_to_add.push((
            //                 row.id,
            //                 DropPosition::Last,
            //                 Box::new(FreeCellFolder::new()),
            //             ));
            //             ui.close_menu();
            //         }
            //     });
            // }

            ControlFlow::Continue(())
        }
        (Method::Leave, NodeMut::TimingTowerRow(_)) => {
            stack.pop();
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::FreeCell(cell)) => {
            builder.leaf(cell.id, |ui| {
                ui.label(&cell.name);
            });
            // if let Some(res) = res {
            //     res.context_menu(|ui| {
            //         if ui.button("add column").clicked() {
            //             nodes_to_add.push((
            //                 *stack.last().expect("There should always be a parent node"),
            //                 DropPosition::After(cell.id),
            //                 Box::new(FreeCell::new()),
            //             ));
            //             ui.close_menu();
            //         }
            //         if ui.button("add group").clicked() {
            //             nodes_to_add.push((
            //                 *stack.last().expect("There should always be a parent node"),
            //                 DropPosition::Last,
            //                 Box::new(FreeCellFolder::new()),
            //             ));
            //             ui.close_menu();
            //         }
            //         if ui.button("delete").clicked() {
            //             nodes_to_remove.push(cell.id);
            //             ui.close_menu();
            //         }
            //     });
            // }
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::FreeCellFolderMut(folder)) => {
            builder.dir(folder.id, |ui| {
                ui.label(&folder.name);
            });
            // if let Some(res) = res {
            //     res.context_menu(|ui| {
            //         if ui.button("add column").clicked() {
            //             nodes_to_add.push((
            //                 *folder.id(),
            //                 DropPosition::Last,
            //                 Box::new(FreeCell::new()),
            //             ));
            //             ui.close_menu();
            //         }
            //         if ui.button("add group").clicked() {
            //             nodes_to_add.push((
            //                 *folder.id(),
            //                 DropPosition::Last,
            //                 Box::new(FreeCellFolder::new()),
            //             ));
            //             ui.close_menu();
            //         }
            //     });
            // }
            ControlFlow::Continue(())
        }
        (Method::Leave, NodeMut::FreeCellFolderMut(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::Asset(asset)) => {
            builder.leaf(asset.id, |ui| {
                ui.label(&asset.name);
            });
            // if let Some(res) = res {
            //     res.context_menu(|ui| {
            //         if ui.button("add image").clicked() {
            //             nodes_to_add.push((
            //                 *stack.last().expect("There should always be a parent node"),
            //                 DropPosition::After(asset.id),
            //                 Box::new(AssetDefinition::new()),
            //             ));
            //             ui.close_menu();
            //         }
            //         if ui.button("add group").clicked() {
            //             nodes_to_add.push((
            //                 *stack.last().expect("There should always be a parent node"),
            //                 DropPosition::Last,
            //                 Box::new(AssetFolder::new()),
            //             ));
            //             ui.close_menu();
            //         }
            //         if ui.button("delete").clicked() {
            //             nodes_to_remove.push(asset.id);
            //             ui.close_menu();
            //         }
            //     });
            // }
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::AssetFolder(folder)) => {
            builder.dir(folder.id, |ui| {
                ui.label(&folder.name);
            });
            // if let Some(res) = res {
            //     res.context_menu(|ui| {
            //         if ui.button("add image").clicked() {
            //             nodes_to_add.push((
            //                 *folder.id(),
            //                 DropPosition::Last,
            //                 Box::new(AssetDefinition::new()),
            //             ));
            //             ui.close_menu();
            //         }
            //         if ui.button("add group").clicked() {
            //             nodes_to_add.push((
            //                 *folder.id(),
            //                 DropPosition::Last,
            //                 Box::new(AssetFolder::new()),
            //             ));
            //             ui.close_menu();
            //         }
            //     });
            // }
            ControlFlow::Continue(())
        }
        (Method::Leave, NodeMut::AssetFolder(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::Variable(variable)) => {
            builder.leaf(variable.id, |ui| {
                ui.label(&variable.name);
            });
            // if let Some(res) = res {
            //     res.context_menu(|ui| {
            //         if ui.button("add variable").clicked() {
            //             nodes_to_add.push((
            //                 *stack.last().expect("There should always be a parent node"),
            //                 DropPosition::After(variable.id),
            //                 Box::new(VariableDefinition::new()),
            //             ));
            //             ui.close_menu();
            //         }
            //         if ui.button("add group").clicked() {
            //             nodes_to_add.push((
            //                 *stack.last().expect("There should always be a parent node"),
            //                 DropPosition::Last,
            //                 Box::new(VariableFolder::new()),
            //             ));
            //             ui.close_menu();
            //         }
            //         if ui.button("delete").clicked() {
            //             nodes_to_remove.push(variable.id);
            //             ui.close_menu();
            //         }
            //     });
            // }
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::VariableFolder(folder)) => {
            builder.dir(folder.id, |ui| {
                ui.label(&folder.name);
            });
            // if let Some(res) = res {
            //     res.context_menu(|ui| {
            //         if ui.button("add variable").clicked() {
            //             nodes_to_add.push((
            //                 *folder.id(),
            //                 DropPosition::Last,
            //                 Box::new(VariableDefinition::new()),
            //             ));
            //             ui.close_menu();
            //         }
            //         if ui.button("add group").clicked() {
            //             nodes_to_add.push((
            //                 *folder.id(),
            //                 DropPosition::Last,
            //                 Box::new(VariableFolder::new()),
            //             ));
            //             ui.close_menu();
            //         }
            //     });
            // }
            ControlFlow::Continue(())
        }
        (Method::Leave, NodeMut::VariableFolder(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, NodeMut::Scene(scene)) => {
            builder.dir(scene.id, |ui| {
                ui.label("Scene");
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, NodeMut::Scene(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        _ => ControlFlow::Continue(()),
    }
}
