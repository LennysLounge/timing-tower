use std::ops::ControlFlow;

use backend::{
    exact_variant::ExactVariant,
    style::{
        assets::{AssetDefinition, AssetFolder},
        graphic::{graphic_items::GraphicItemId, GraphicDefinition},
        variables::{VariableDefinition, VariableFolder},
        StyleDefinition, StyleId, StyleItem,
    },
    tree_iterator::{Method, TreeIterator, TreeIteratorMut},
};
use bevy_egui::egui::{self, ScrollArea, Ui};
use egui_ltreeview::{
    builder::{CloserState, NodeBuilder},
    Action, DropPosition, TreeViewBuilder, TreeViewResponse,
};

use crate::command::{
    insert_node::InsertNode, move_node::MoveNode, remove_node::RemoveNode, UndoRedoManager,
};

pub fn tree_view(
    ui: &mut Ui,
    selected_node: &mut Option<StyleId>,
    _secondary_selection: &mut Option<GraphicItemId>,
    base_node: &mut ExactVariant<StyleItem, StyleDefinition>,
    undo_redo_manager: &mut UndoRedoManager,
) -> bool {
    let mut changed = false;
    let response = ScrollArea::vertical()
        .show(ui, |ui| {
            show(ui, base_node.as_enum_mut(), undo_redo_manager)
        })
        .inner;

    for action in response.actions.iter() {
        match action {
            Action::SetSelected(id) => {
                *selected_node = *id;
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
                    .search(*source, |dragged| {
                        base_node.search(*target, |dropped| drop_allowed(dropped, dragged))
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

fn drop_allowed(target: &StyleItem, dragged: &StyleItem) -> bool {
    match (target, dragged) {
        (StyleItem::VariableFolder(_), StyleItem::VariableFolder(_)) => true,
        (StyleItem::VariableFolder(_), StyleItem::Variable(_)) => true,

        (StyleItem::AssetFolder(_), StyleItem::AssetFolder(_)) => true,
        (StyleItem::AssetFolder(_), StyleItem::Asset(_)) => true,

        (StyleItem::Scene(_), StyleItem::Graphic(_)) => true,
        _ => false,
    }
}

fn show(
    ui: &mut Ui,
    root: &mut StyleItem,
    undo_redo_manager: &mut UndoRedoManager,
) -> TreeViewResponse<StyleId> {
    let response = egui_ltreeview::TreeView::new(ui.make_persistent_id("element_tree_view"))
        .row_layout(egui_ltreeview::RowLayout::CompactAlignedLables)
        .fill_space_vertical(true)
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
    node: &mut StyleItem,
    method: Method,
    builder: &mut TreeViewBuilder<StyleId>,
) -> ControlFlow<()> {
    match (method, node) {
        (Method::Visit, StyleItem::Style(style)) => {
            builder.node(NodeBuilder::dir(style.id).flatten(true), |ui| {
                ui.add(egui::Label::new("Style").selectable(false));
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, StyleItem::Style(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItem::Asset(asset)) => {
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
                ui.add(egui::Label::new(&asset.name).selectable(false));
            });
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItem::AssetFolder(folder)) => {
            builder.node(
                NodeBuilder::dir(folder.id)
                    .closer(folder_closer)
                    .default_open(false),
                |ui| {
                    ui.add(egui::Label::new(&folder.name).selectable(false));
                },
            );
            ControlFlow::Continue(())
        }

        (Method::Leave, StyleItem::AssetFolder(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItem::Variable(variable)) => {
            builder.node(
                NodeBuilder::leaf(variable.id).icon(|ui| {
                    egui::Image::new(egui::include_image!("../../../images/object.png"))
                        .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                        .paint_at(ui, ui.max_rect());
                }),
                |ui| {
                    ui.add(egui::Label::new(&variable.name).selectable(false));
                },
            );
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItem::VariableFolder(folder)) => {
            builder.node(
                NodeBuilder::dir(folder.id)
                    .closer(folder_closer)
                    .default_open(false),
                |ui| {
                    ui.add(egui::Label::new(&folder.name).selectable(false));
                },
            );
            ControlFlow::Continue(())
        }

        (Method::Leave, StyleItem::VariableFolder(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItem::Scene(scene)) => {
            builder.node(NodeBuilder::dir(scene.id).closer(folder_closer), |ui| {
                ui.add(egui::Label::new("Scene").selectable(false));
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, StyleItem::Scene(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }
        (Method::Visit, StyleItem::Graphic(graphic)) => {
            builder.node(
                NodeBuilder::leaf(graphic.id).icon(|ui| {
                    egui::Image::new(egui::include_image!("../../../images/graphic.png"))
                        .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                        .paint_at(ui, ui.max_rect());
                }),
                |ui| {
                    ui.add(egui::Label::new(&graphic.name).selectable(false));
                },
            );
            ControlFlow::Continue(())
        }
        (Method::Visit, StyleItem::GraphicFolder(folder)) => {
            builder.node(NodeBuilder::dir(folder.id).closer(folder_closer), |ui| {
                ui.add(egui::Label::new(&folder.name).selectable(false));
            });
            ControlFlow::Continue(())
        }
        (Method::Leave, StyleItem::GraphicFolder(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Leave, StyleItem::Variable(_)) => ControlFlow::Continue(()),
        (Method::Leave, StyleItem::Asset(_)) => ControlFlow::Continue(()),
        (Method::Leave, StyleItem::Graphic(_)) => ControlFlow::Continue(()),
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
    node: &mut StyleItem,
    undo_redo_manager: &mut UndoRedoManager,
    tree_response: &TreeViewResponse<StyleId>,
) {
    match node {
        StyleItem::Style(_) => _ = ui.label("Style"),
        StyleItem::Variable(variable) => {
            if ui.button("add variable").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(variable.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(variable.id),
                    node: VariableDefinition::new().into(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(variable.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(variable.id),
                    node: VariableFolder::new().into(),
                });
                ui.close_menu();
            }
            if ui.button("delete").clicked() {
                undo_redo_manager.queue(RemoveNode { id: variable.id });
                ui.close_menu();
            }
        }
        StyleItem::VariableFolder(folder) => {
            if ui.button("add variable").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: VariableDefinition::new().into(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: VariableFolder::new().into(),
                });
                ui.close_menu();
            }
        }
        StyleItem::Asset(asset) => {
            if ui.button("add image").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(asset.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(asset.id),
                    node: AssetDefinition::new().into(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(asset.id)
                        .expect("Should have a parent"),
                    position: DropPosition::After(asset.id),
                    node: AssetFolder::new().into(),
                });
                ui.close_menu();
            }
            if ui.button("delete").clicked() {
                undo_redo_manager.queue(RemoveNode { id: asset.id });
                ui.close_menu();
            }
        }
        StyleItem::AssetFolder(folder) => {
            if ui.button("add image").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: AssetDefinition::new().into(),
                });
                ui.close_menu();
            }
            if ui.button("add group").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: AssetFolder::new().into(),
                });
                ui.close_menu();
            }
        }
        StyleItem::Scene(scene) => {
            if ui.button("add component").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: scene.id,
                    position: DropPosition::Last,
                    node: GraphicDefinition::new().into(),
                });
                ui.close_menu();
            }
        }
        StyleItem::Graphic(graphic) => {
            if ui.button("add graphic").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: tree_response
                        .parent_of(graphic.id)
                        .expect("Should have parent"),
                    position: DropPosition::After(graphic.id),
                    node: GraphicDefinition::new().into(),
                });
                ui.close_menu();
            }
            if ui.button("delete").clicked() {
                undo_redo_manager.queue(RemoveNode { id: graphic.id });
                ui.close_menu();
            }
        }
        StyleItem::GraphicFolder(folder) => {
            if ui.button("add graphic").clicked() {
                undo_redo_manager.queue(InsertNode {
                    target_node: folder.id,
                    position: DropPosition::Last,
                    node: GraphicDefinition::new().into(),
                });
                ui.close_menu();
            }
        }
    }
}
