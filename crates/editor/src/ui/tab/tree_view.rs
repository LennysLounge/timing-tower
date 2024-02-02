use std::ops::ControlFlow;

use backend::{
    style::{
        assets::{AssetDefinition, AssetFolder},
        cell::{FreeCell, FreeCellFolder},
        graphic::Graphic,
        variables::{VariableDefinition, VariableFolder},
        StyleDefinition, StyleItem, StyleItemMut, StyleItemRef,
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
    secondary_selection: &mut Option<Uuid>,
    base_node: &mut StyleDefinition,
    undo_redo_manager: &mut UndoRedoManager,
) -> bool {
    let mut changed = false;
    let response = ScrollArea::vertical()
        .show(ui, |ui| show(ui, base_node.as_mut(), undo_redo_manager))
        .inner;

    for action in response.actions.iter() {
        match action {
            Action::SetSelected(id) => {
                *selected_node = *id;
                *secondary_selection = None;
            }
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
                    .as_ref()
                    .search(*source, |dragged| {
                        base_node
                            .as_ref()
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

fn drop_allowed(target: &StyleItemRef, dragged: &StyleItemRef) -> bool {
    match (target, dragged) {
        (StyleItemRef::VariableFolder(_), StyleItemRef::VariableFolder(_)) => true,
        (StyleItemRef::VariableFolder(_), StyleItemRef::Variable(_)) => true,

        (StyleItemRef::AssetFolder(_), StyleItemRef::AssetFolder(_)) => true,
        (StyleItemRef::AssetFolder(_), StyleItemRef::Asset(_)) => true,

        (StyleItemRef::TimingTowerRow(_), StyleItemRef::FreeCellFolder(_)) => true,
        (StyleItemRef::TimingTowerRow(_), StyleItemRef::FreeCell(_)) => true,

        (StyleItemRef::FreeCellFolder(_), StyleItemRef::FreeCellFolder(_)) => true,
        (StyleItemRef::FreeCellFolder(_), StyleItemRef::FreeCell(_)) => true,

        (StyleItemRef::TimingTower(_), StyleItemRef::FreeCell(_)) => true,
        (StyleItemRef::TimingTower(_), StyleItemRef::FreeCellFolder(_)) => true,

        (StyleItemRef::Scene(_), StyleItemRef::Graphic(_)) => true,

        _ => false,
    }
}

fn show(
    ui: &mut Ui,
    mut root: StyleItemMut,
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
    node: &mut StyleItemMut,
    method: Method,
    builder: &mut TreeViewBuilder<Uuid>,
) -> ControlFlow<()> {
    match (method, node) {
        (Method::Visit, StyleItemMut::Style(style)) => {
            builder.node(NodeBuilder::dir(style.id), |ui| {
                ui.label("Style");
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, StyleItemMut::Style(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItemMut::TimingTower(tower)) => {
            builder.node(NodeBuilder::dir(tower.id).closer(folder_closer), |ui| {
                ui.label("Timing tower");
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, StyleItemMut::TimingTower(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItemMut::TimingTowerRow(row)) => {
            builder.node(NodeBuilder::dir(row.id).closer(folder_closer), |ui| {
                ui.label("Row");
            });
            ControlFlow::Continue(())
        }

        (Method::Leave, StyleItemMut::TimingTowerRow(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItemMut::FreeCell(cell)) => {
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

        (Method::Visit, StyleItemMut::FreeCellFolder(folder)) => {
            builder.node(NodeBuilder::dir(folder.id).closer(folder_closer), |ui| {
                ui.label(&folder.name);
            });
            ControlFlow::Continue(())
        }

        (Method::Leave, StyleItemMut::FreeCellFolder(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItemMut::Asset(asset)) => {
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

        (Method::Visit, StyleItemMut::AssetFolder(folder)) => {
            builder.node(
                NodeBuilder::dir(folder.id)
                    .closer(folder_closer)
                    .default_open(false),
                |ui| {
                    ui.label(&folder.name);
                },
            );
            ControlFlow::Continue(())
        }

        (Method::Leave, StyleItemMut::AssetFolder(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItemMut::Variable(variable)) => {
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

        (Method::Visit, StyleItemMut::VariableFolder(folder)) => {
            builder.node(
                NodeBuilder::dir(folder.id)
                    .closer(folder_closer)
                    .default_open(false),
                |ui| {
                    ui.label(&folder.name);
                },
            );
            ControlFlow::Continue(())
        }

        (Method::Leave, StyleItemMut::VariableFolder(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItemMut::Scene(scene)) => {
            builder.node(NodeBuilder::dir(scene.id).closer(folder_closer), |ui| {
                ui.label("Scene");
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, StyleItemMut::Scene(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }
        (Method::Visit, StyleItemMut::Graphic(comp)) => {
            builder.leaf(comp.id, |ui| {
                ui.label(&comp.name);
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, StyleItemMut::Variable(_)) => ControlFlow::Continue(()),
        (Method::Leave, StyleItemMut::Asset(_)) => ControlFlow::Continue(()),
        (Method::Leave, StyleItemMut::FreeCell(_)) => ControlFlow::Continue(()),
        (Method::Leave, StyleItemMut::Graphic(_)) => ControlFlow::Continue(()),
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
    node: &mut StyleItemMut,
    undo_redo_manager: &mut UndoRedoManager,
    tree_response: &TreeViewResponse<Uuid>,
) {
    match node {
        StyleItemMut::Style(_) => _ = ui.label("Style"),
        StyleItemMut::Variable(variable) => {
            if ui.button("add variable").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(variable.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(variable.id),
                    node: VariableDefinition::new().to_owned(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(variable.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(variable.id),
                    node: VariableFolder::new().to_owned(),
                });
                ui.close_menu();
            }
            if ui.button("delete").clicked() {
                undo_redo_manager.queue(RemoveNode { id: variable.id });
                ui.close_menu();
            }
        }
        StyleItemMut::VariableFolder(folder) => {
            if ui.button("add variable").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: VariableDefinition::new().to_owned(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: VariableFolder::new().to_owned(),
                });
                ui.close_menu();
            }
        }
        StyleItemMut::Asset(asset) => {
            if ui.button("add image").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(asset.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(asset.id),
                    node: AssetDefinition::new().to_owned(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(asset.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(asset.id),
                    node: AssetFolder::new().to_owned(),
                });
                ui.close_menu();
            }
            if ui.button("delete").clicked() {
                undo_redo_manager.queue(RemoveNode { id: asset.id });
                ui.close_menu();
            }
        }
        StyleItemMut::AssetFolder(folder) => {
            if ui.button("add image").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: AssetDefinition::new().to_owned(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: AssetFolder::new().to_owned(),
                });
                ui.close_menu();
            }
        }
        StyleItemMut::Scene(scene) => {
            if ui.button("add component").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: scene.id,
                    position: DropPosition::Last,
                    node: Graphic::new().to_owned(),
                });
                ui.close_menu();
            }
        }
        StyleItemMut::TimingTower(tower) => {
            if ui.button("add column").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tower.id,
                    position: DropPosition::Last,
                    node: FreeCell::new().to_owned(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tower.id,
                    position: DropPosition::Last,
                    node: FreeCellFolder::new().to_owned(),
                });
                ui.close_menu();
            }
        }
        StyleItemMut::TimingTowerRow(row) => {
            if ui.button("add column").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: row.id,
                    position: DropPosition::Last,
                    node: FreeCell::new().to_owned(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: row.id,
                    position: DropPosition::Last,
                    node: FreeCellFolder::new().to_owned(),
                });
                ui.close_menu();
            }
        }
        StyleItemMut::FreeCellFolder(folder) => {
            if ui.button("add column").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: FreeCell::new().to_owned(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: FreeCellFolder::new().to_owned(),
                });
                ui.close_menu();
            }
            if ui.button("delete").clicked() {
                undo_redo_manager.queue(RemoveNode { id: folder.id });
                ui.close_menu();
            }
        }
        StyleItemMut::FreeCell(cell) => {
            if ui.button("add column").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(cell.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(cell.id),
                    node: FreeCell::new().to_owned(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(cell.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(cell.id),
                    node: FreeCellFolder::new().to_owned(),
                });
                ui.close_menu();
            }
            if ui.button("delete").clicked() {
                undo_redo_manager.queue(RemoveNode { id: cell.id });
                ui.close_menu();
            }
        }
        StyleItemMut::Graphic(comp) => {
            ui.label(&comp.name);
        }
    }
}
