use std::ops::ControlFlow;

use backend::{
    graphic::GraphicStates,
    style::graphic::{
        self,
        graphic_items::{
            cell::Cell, clip_area::ClipArea, driver_table::DriverTable, GraphicItem, GraphicItemId,
        },
        GraphicDefinition, GraphicStateId,
    },
    tree_iterator::{Method, TreeItem, TreeIterator, TreeIteratorMut},
};
use bevy_egui::egui::{self, vec2, Color32, Id, RichText, Ui};
use egui_ltreeview::{
    node::NodeBuilder, Action, DropPosition, RowLayout, TreeView, TreeViewBuilder,
};
use uuid::Uuid;

use crate::command::{
    edit_property::{EditProperty, EditResult},
    UndoRedoManager,
};

pub fn graphic_property_editor(
    ui: &mut Ui,
    component: &mut GraphicDefinition,
    graphic_item_selection: &mut Option<GraphicItemId>,
    undo_redo_manager: &mut UndoRedoManager,
    graphic_states: &mut GraphicStates,
) {
    let mut edit_result = EditResult::None;

    ui.label("Name:");
    let res = ui.text_edit_singleline(&mut component.name);
    if res.changed() {
        // Keep the graphic item root in sync with the graphic name itself.
        component.items.name = component.name.clone();
    };
    edit_result |= res.into();

    ui.separator();

    ui.label("Elements:");
    ui.group(|ui| {
        egui::ScrollArea::horizontal()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                edit_result |= show_element_tree(ui, graphic_item_selection, component);
            });

        ui.allocate_space(vec2(
            ui.available_width(),
            -ui.spacing().item_spacing.y + (100.0 - ui.min_rect().height()),
        ));
    });
    egui::ComboBox::from_id_source(ui.next_auto_id())
        .selected_text("add element")
        .width(ui.available_width())
        .show_ui(ui, |ui| {
            if ui.selectable_label(false, "Cell").clicked() {
                component.items.items.push(Cell::new().into());
                edit_result = EditResult::FromId(ui.id());
                ui.close_menu();
            }
            if ui.selectable_label(false, "Clip Area").clicked() {
                component.items.items.push(ClipArea::new().into());
                edit_result = EditResult::FromId(ui.id());
                ui.close_menu();
            }
            if ui.selectable_label(false, "Driver Table").clicked() {
                component.items.items.push(DriverTable::new().into());
                edit_result = EditResult::FromId(ui.id());
                ui.close_menu();
            }
        });

    ui.add_space(10.0);
    ui.label("States:");
    ui.group(|ui| {
        let tree_res = TreeView::new(ui.make_persistent_id("State tree"))
            .row_layout(RowLayout::Compact)
            .show(ui, |mut builder| {
                if let Some(state) = graphic_states.states.get(&component.id) {
                    builder.set_selected(*state);
                }
                builder.leaf(GraphicStateId(Uuid::default()), "Template");
                for state in component.states.iter_mut() {
                    builder.leaf(state.id, &state.name);
                }
            });
        for action in tree_res.actions {
            if let Action::SetSelected(Some(id)) = action {
                if id.0 == Uuid::default() {
                    graphic_states.states.remove(&component.id);
                } else {
                    graphic_states.states.insert(component.id, id);
                }
            }
        }

        ui.allocate_space(vec2(
            ui.available_width(),
            -ui.spacing().item_spacing.y + (100.0 - ui.min_rect().height()),
        ));
    });
    let add_button_res = ui.add_sized(vec2(ui.available_width(), 0.0), egui::Button::new("Add"));
    if add_button_res.clicked() {
        component.states.push(graphic::GraphicState {
            id: GraphicStateId::new(),
            name: String::from("new state"),
        });
    }

    if let EditResult::FromId(widget_id) = edit_result {
        undo_redo_manager.queue(EditProperty::new(
            component.id,
            component.clone(),
            widget_id,
        ));
    }
}

fn show_element_tree(
    ui: &mut Ui,
    secondary_selection: &mut Option<GraphicItemId>,
    graphic: &mut GraphicDefinition,
) -> EditResult {
    let mut edit_result = EditResult::None;
    let res = TreeView::new(ui.make_persistent_id("Component element tree"))
        .row_layout(RowLayout::AlignedIcons)
        .show(ui, |mut builder| {
            graphic.items.walk(&mut |item, method| {
                element_tree_node(&mut builder, item, method);
                ControlFlow::Continue::<()>(())
            });
        });

    for action in res.actions.iter() {
        match action {
            Action::SetSelected(id) => {
                *secondary_selection = *id;
            }
            Action::Move {
                source,
                target,
                position,
            } => {
                if let Some(element) = remove_element(graphic, *source) {
                    insert_element(graphic, *target, *position, element);
                    edit_result |= EditResult::FromId(Id::new("Graphic item moved"));
                }
            }
            Action::Drag { .. } => (),
        }
    }

    enum Command {
        Add {
            element: GraphicItem,
            target: GraphicItemId,
            position: DropPosition<GraphicItemId>,
        },
        Remove {
            id: GraphicItemId,
        },
    }
    let mut commands = Vec::new();
    res.context_menu(ui, |ui, node_id| {
        graphic.items.search_mut(node_id, |element| {
            let (target, position) = match element {
                GraphicItem::Cell(_) => (
                    res.parent_of(node_id).unwrap_or_default(),
                    DropPosition::After(node_id),
                ),
                GraphicItem::Root(_) | GraphicItem::ClipArea(_) | GraphicItem::DriverTable(_) => {
                    (node_id, DropPosition::Last)
                }
            };
            if ui.button("add cell").clicked() {
                commands.push(Command::Add {
                    element: Cell::new().into(),
                    target,
                    position,
                });
                ui.close_menu();
            }
            if ui.button("add clip area").clicked() {
                commands.push(Command::Add {
                    element: ClipArea::new().into(),
                    target,
                    position,
                });
                ui.close_menu();
            }
            if ui.button("add driver table").clicked() {
                commands.push(Command::Add {
                    element: DriverTable::new().into(),
                    target,
                    position,
                });
                ui.close_menu();
            }
            ui.separator();
            if ui.button("delete").clicked() {
                commands.push(Command::Remove { id: node_id });
                ui.close_menu();
            }
        });
    });
    for command in commands {
        match command {
            Command::Add {
                element,
                target,
                position,
            } => {
                insert_element(graphic, target, position, element);
                edit_result = EditResult::FromId(Id::new("Component element Tree view edit"));
            }
            Command::Remove { id } => {
                remove_element(graphic, id);
                edit_result = EditResult::FromId(Id::new("Component element Tree view edit"));
            }
        }
    }

    edit_result
}

fn element_tree_node(
    builder: &mut TreeViewBuilder<GraphicItemId>,
    element: &GraphicItem,
    method: Method,
) {
    match (method, element) {
        (Method::Visit, GraphicItem::Root(root)) => {
            builder.node(NodeBuilder::dir(root.id), |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Label::new(RichText::new("Graphic").color(Color32::from_gray(120)))
                            .selectable(false),
                    );
                    ui.add(egui::Label::new(&root.name).selectable(false));
                });
            });
        }
        (Method::Leave, GraphicItem::Root(_)) => {
            builder.close_dir();
        }

        (Method::Visit, GraphicItem::Cell(cell)) => {
            builder.node(
                NodeBuilder::leaf(cell.id).icon(|ui| {
                    egui::Image::new(egui::include_image!("../../../../images/cell.png"))
                        .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                        .paint_at(ui, ui.max_rect());
                }),
                |ui| {
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Label::new(RichText::new("Cell").color(Color32::from_gray(120)))
                                .selectable(false),
                        );
                        ui.add(egui::Label::new(&cell.name).selectable(false));
                    });
                },
            );
        }
        (Method::Leave, GraphicItem::Cell(_)) => (),
        (Method::Visit, GraphicItem::ClipArea(clip_area)) => {
            builder.node(
                NodeBuilder::dir(clip_area.id).icon(|ui| {
                    egui::Image::new(egui::include_image!("../../../../images/clip_area.png"))
                        .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                        .paint_at(ui, ui.max_rect());
                }),
                |ui| {
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Label::new(
                                RichText::new("Clip area").color(Color32::from_gray(120)),
                            )
                            .selectable(false),
                        );
                        ui.add(egui::Label::new(&clip_area.name).selectable(false));
                    });
                },
            );
        }
        (Method::Leave, GraphicItem::ClipArea(_)) => {
            builder.close_dir();
        }
        (Method::Visit, GraphicItem::DriverTable(driver_table)) => {
            builder.node(
                NodeBuilder::dir(driver_table.id).icon(|ui| {
                    egui::Image::new(egui::include_image!("../../../../images/driver_table.png"))
                        .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                        .paint_at(ui, ui.max_rect());
                }),
                |ui| {
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Label::new(
                                RichText::new("Driver table").color(Color32::from_gray(120)),
                            )
                            .selectable(false),
                        );
                        ui.add(egui::Label::new(&driver_table.name).selectable(false));
                    });
                },
            );
        }
        (Method::Leave, GraphicItem::DriverTable(_)) => {
            builder.close_dir();
        }
    }
}

fn remove_element(component: &mut GraphicDefinition, id: GraphicItemId) -> Option<GraphicItem> {
    if let Some(index) = component.items.items.iter().position(|e| e.id() == id) {
        return Some(component.items.items.remove(index));
    }
    let r = component.items.walk_mut(&mut |e, method| {
        if method != Method::Visit {
            return ControlFlow::Continue(());
        }
        match e {
            GraphicItem::Root(root) => {
                if let Some(index) = root.items.iter().position(|e| e.id() == id) {
                    ControlFlow::Break(Some(root.items.remove(index)))
                } else {
                    ControlFlow::Continue(())
                }
            }
            GraphicItem::Cell(_) => ControlFlow::Continue(()),
            GraphicItem::ClipArea(clip_area) => {
                if let Some(index) = clip_area.items.iter().position(|e| e.id() == id) {
                    ControlFlow::Break(Some(clip_area.items.remove(index)))
                } else {
                    ControlFlow::Continue(())
                }
            }
            GraphicItem::DriverTable(driver_table) => {
                if let Some(index) = driver_table.columns.iter().position(|e| e.id() == id) {
                    ControlFlow::Break(Some(driver_table.columns.remove(index)))
                } else {
                    ControlFlow::Continue(())
                }
            }
        }
    });
    match r {
        ControlFlow::Continue(_) => None,
        ControlFlow::Break(x) => x,
    }
}

fn insert_element(
    component: &mut GraphicDefinition,
    target: GraphicItemId,
    position: DropPosition<GraphicItemId>,
    element: GraphicItem,
) {
    component.items.search_mut(target, |e| match e {
        GraphicItem::Root(root) => {
            insert_into_vec(&mut root.items, position, element);
        }
        GraphicItem::Cell(_) => (),
        GraphicItem::ClipArea(clip_area) => {
            insert_into_vec(&mut clip_area.items, position, element);
        }
        GraphicItem::DriverTable(driver_table) => {
            insert_into_vec(&mut driver_table.columns, position, element);
        }
    });
}

fn insert_into_vec(
    vec: &mut Vec<GraphicItem>,
    position: DropPosition<GraphicItemId>,
    element: GraphicItem,
) {
    match position {
        DropPosition::First => vec.insert(0, element),
        DropPosition::Last => vec.push(element),
        DropPosition::After(id) => {
            if let Some(index) = vec.iter().position(|e| e.id() == id) {
                vec.insert(index + 1, element);
            } else {
                vec.push(element);
            }
        }
        DropPosition::Before(id) => {
            if let Some(index) = vec.iter().position(|e| e.id() == id) {
                vec.insert(index, element);
            } else {
                vec.push(element);
            }
        }
    }
}
