use std::ops::ControlFlow;

use backend::{
    style::{
        assets::{AssetDefinition, AssetFolder},
        graphic::GraphicDefinition,
        variables::{VariableDefinition, VariableFolder},
        StyleId, StyleItem, TreePosition,
    },
    tree_iterator::{Method, TreeIterator, TreeIteratorMut},
};
use bevy_egui::egui::{self, ScrollArea, Ui};
use egui_ltreeview::{
    node::{CloserState, NodeBuilder},
    Action, DropPosition, TreeViewBuilder, TreeViewResponse,
};

use crate::ui::{EditorState, EditorStyle, UiMessage, UiMessages};

pub(super) fn tree_view(
    ui: &mut Ui,
    messages: &mut UiMessages,
    style: &mut EditorStyle,
    state: &mut EditorState,
) {
    let response = ScrollArea::vertical()
        .show(ui, |ui| show(ui, style.0.as_enum_mut(), messages, state))
        .inner;

    for action in response.actions.iter() {
        match action {
            Action::SetSelected(Some(id)) => {
                messages.push(UiMessage::StyleItemSelect(*id));
            }
            Action::SetSelected(_) => (),
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
                let drop_allowed = style
                    .0
                    .search(*source, |dragged| {
                        style
                            .0
                            .search(*target, |dropped| drop_allowed(dropped, dragged))
                    })
                    .flatten()
                    .unwrap_or(false);
                if !drop_allowed {
                    response.remove_drop_marker(ui);
                }
                if let Action::Move { .. } = a {
                    messages.push(UiMessage::StyleItemMove {
                        source: *source,
                        target: *target,
                        position: match position {
                            DropPosition::First => TreePosition::First,
                            DropPosition::Last => TreePosition::Last,
                            DropPosition::After(id) => TreePosition::After(*id),
                            DropPosition::Before(id) => TreePosition::Before(*id),
                        },
                    });
                }
            }
        }
    }
}

fn drop_allowed(target: &StyleItem, dragged: &StyleItem) -> bool {
    match (target, dragged) {
        (StyleItem::VariableFolder(_), StyleItem::VariableFolder(_)) => true,
        (StyleItem::VariableFolder(_), StyleItem::Variable(_)) => true,

        (StyleItem::AssetFolder(_), StyleItem::AssetFolder(_)) => true,
        (StyleItem::AssetFolder(_), StyleItem::Asset(_)) => true,

        (StyleItem::GraphicFolder(_), StyleItem::GraphicFolder(_)) => true,
        (StyleItem::GraphicFolder(_), StyleItem::Graphic(_)) => true,

        _ => false,
    }
}

fn show(
    ui: &mut Ui,
    root: &mut StyleItem,
    messages: &mut UiMessages,
    state: &mut EditorState,
) -> TreeViewResponse<StyleId> {
    let response = egui_ltreeview::TreeView::new(ui.make_persistent_id("element_tree_view"))
        .row_layout(egui_ltreeview::RowLayout::CompactAlignedLables)
        .fill_space_vertical(true)
        .show_state(ui, &mut state.style_item_tree_state, |mut builder| {
            root.walk_mut(&mut |node, method| show_node(node, method, &mut builder, messages));
        });

    response
}
fn show_node(
    node: &mut StyleItem,
    method: Method,
    builder: &mut TreeViewBuilder<StyleId>,
    messages: &mut UiMessages,
) -> ControlFlow<()> {
    match (method, node) {
        (Method::Visit, StyleItem::Style(style)) => {
            builder.node(NodeBuilder::dir(style.id).flatten(true).label(|ui| {
                ui.add(egui::Label::new("Style").selectable(false));
            }));
            ControlFlow::Continue(())
        }
        (Method::Leave, StyleItem::Style(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItem::Asset(asset)) => {
            let parent_id = builder.parent_id();
            builder.node(
                NodeBuilder::leaf(asset.id)
                    .icon(|ui| {
                        match asset.value_type {
                            backend::value_types::ValueType::Texture => {
                                egui::Image::new(egui::include_image!("../../../images/image.png"))
                                    .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                                    .paint_at(ui, ui.max_rect());
                            }
                            backend::value_types::ValueType::Font => {
                                egui::Image::new(egui::include_image!(
                                    "../../../images/match_case.png"
                                ))
                                .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                                .paint_at(ui, ui.max_rect());
                            }
                            _ => (),
                        };
                    })
                    .label(|ui| {
                        ui.add(egui::Label::new(&asset.name).selectable(false));
                    })
                    .context_menu(|ui| {
                        if ui.button("add image").clicked() {
                            messages.push(UiMessage::StyleItemInsert {
                                target: parent_id.expect("Should have a parent"),
                                position: TreePosition::After(asset.id),
                                node: AssetDefinition::new_image().into(),
                                select_node: true,
                            });
                            ui.close_menu();
                        }
                        if ui.button("add font").clicked() {
                            messages.push(UiMessage::StyleItemInsert {
                                target: parent_id.expect("Should have a parent"),
                                position: TreePosition::After(asset.id),
                                node: AssetDefinition::new_font().into(),
                                select_node: true,
                            });
                            ui.close_menu();
                        }
                        if ui.button("add group").clicked() {
                            messages.push(UiMessage::StyleItemInsert {
                                target: parent_id.expect("Should have a parent"),
                                position: TreePosition::After(asset.id),
                                node: AssetFolder::new().into(),
                                select_node: true,
                            });
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("delete").clicked() {
                            messages.push(UiMessage::StyleItemRemove(asset.id));
                            ui.close_menu();
                        }
                    }),
            );
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItem::AssetFolder(folder)) => {
            builder.node(
                NodeBuilder::dir(folder.id)
                    .closer(folder_closer)
                    .default_open(false)
                    .label(|ui| {
                        ui.add(egui::Label::new(&folder.name).selectable(false));
                    })
                    .context_menu(|ui| {
                        if ui.button("add image").clicked() {
                            messages.push(UiMessage::StyleItemInsert {
                                target: folder.id,
                                position: TreePosition::Last,
                                node: AssetDefinition::new_image().into(),
                                select_node: true,
                            });
                            ui.close_menu();
                        }
                        if ui.button("add group").clicked() {
                            messages.push(UiMessage::StyleItemInsert {
                                target: folder.id,
                                position: TreePosition::Last,
                                node: AssetFolder::new().into(),
                                select_node: true,
                            });
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("delete").clicked() {
                            messages.push(UiMessage::StyleItemRemove(folder.id));
                            ui.close_menu();
                        }
                    }),
            );
            ControlFlow::Continue(())
        }

        (Method::Leave, StyleItem::AssetFolder(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItem::Variable(variable)) => {
            let parent_id = builder.parent_id();
            builder.node(
                NodeBuilder::leaf(variable.id)
                    .icon(|ui| {
                        egui::Image::new(egui::include_image!("../../../images/object.png"))
                            .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                            .paint_at(ui, ui.max_rect());
                    })
                    .label(|ui| {
                        ui.add(egui::Label::new(&variable.name).selectable(false));
                    })
                    .context_menu(|ui| {
                        if ui.button("add variable").clicked() {
                            messages.push(UiMessage::StyleItemInsert {
                                target: parent_id.expect("Should have a parent"),
                                position: TreePosition::After(variable.id),
                                node: VariableDefinition::new().into(),
                                select_node: true,
                            });
                            ui.close_menu();
                        }
                        if ui.button("add group").clicked() {
                            messages.push(UiMessage::StyleItemInsert {
                                target: parent_id.expect("Should have a parent"),
                                position: TreePosition::After(variable.id),
                                node: VariableDefinition::new().into(),
                                select_node: true,
                            });
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("delete").clicked() {
                            messages.push(UiMessage::StyleItemRemove(variable.id));
                            ui.close_menu();
                        }
                    }),
            );
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItem::VariableFolder(folder)) => {
            builder.node(
                NodeBuilder::dir(folder.id)
                    .closer(folder_closer)
                    .default_open(false)
                    .label(|ui| {
                        ui.add(egui::Label::new(&folder.name).selectable(false));
                    })
                    .context_menu(|ui| {
                        if ui.button("add variable").clicked() {
                            messages.push(UiMessage::StyleItemInsert {
                                target: folder.id,
                                position: TreePosition::Last,
                                node: VariableDefinition::new().into(),
                                select_node: true,
                            });
                            ui.close_menu();
                        }
                        if ui.button("add group").clicked() {
                            messages.push(UiMessage::StyleItemInsert {
                                target: folder.id,
                                position: TreePosition::Last,
                                node: VariableFolder::new().into(),
                                select_node: true,
                            });
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("delete").clicked() {
                            messages.push(UiMessage::StyleItemRemove(folder.id));
                            ui.close_menu();
                        }
                    }),
            );
            ControlFlow::Continue(())
        }

        (Method::Leave, StyleItem::VariableFolder(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }

        (Method::Visit, StyleItem::Scene(scene)) => {
            builder.node(
                NodeBuilder::dir(scene.id)
                    .closer(folder_closer)
                    .label(|ui| {
                        ui.add(egui::Label::new("Scene").selectable(false));
                    }),
            );
            ControlFlow::Continue(())
        }
        (Method::Leave, StyleItem::Scene(_)) => {
            builder.close_dir();
            ControlFlow::Continue(())
        }
        (Method::Visit, StyleItem::Graphic(graphic)) => {
            let parent_id = builder.parent_id();
            builder.node(
                NodeBuilder::leaf(graphic.id)
                    .icon(|ui| {
                        egui::Image::new(egui::include_image!("../../../images/graphic.png"))
                            .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                            .paint_at(ui, ui.max_rect());
                    })
                    .label(|ui| {
                        ui.add(egui::Label::new(&graphic.name).selectable(false));
                    })
                    .context_menu(|ui| {
                        if ui.button("add graphic").clicked() {
                            messages.push(UiMessage::StyleItemInsert {
                                target: parent_id.expect("Should have parent"),
                                position: TreePosition::After(graphic.id),
                                node: GraphicDefinition::new().into(),
                                select_node: true,
                            });
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("delete").clicked() {
                            messages.push(UiMessage::StyleItemRemove(graphic.id));
                            ui.close_menu();
                        }
                    }),
            );
            ControlFlow::Continue(())
        }
        (Method::Visit, StyleItem::GraphicFolder(folder)) => {
            builder.node(
                NodeBuilder::dir(folder.id)
                    .closer(folder_closer)
                    .label(|ui| {
                        ui.add(egui::Label::new(&folder.name).selectable(false));
                    })
                    .context_menu(|ui| {
                        if ui.button("add graphic").clicked() {
                            messages.push(UiMessage::StyleItemInsert {
                                target: folder.id,
                                position: TreePosition::Last,
                                node: GraphicDefinition::new().into(),
                                select_node: true,
                            });
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("delete").clicked() {
                            messages.push(UiMessage::StyleItemRemove(folder.id));
                            ui.close_menu();
                        }
                    }),
            );
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
