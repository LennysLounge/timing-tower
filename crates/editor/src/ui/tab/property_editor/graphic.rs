use std::ops::ControlFlow;

use backend::{
    style::{
        graphic::GraphicDefinition,
        graphic_items::{cell::Cell, clip_area::ClipArea, driver_table::DriverTable, GraphicItem},
    },
    tree_iterator::{Method, TreeItem, TreeIterator, TreeIteratorMut},
};
use bevy_egui::egui::{self, vec2, Color32, Id, Ui};
use egui_ltreeview::{
    builder::NodeBuilder, Action, DropPosition, RowLayout, TreeView, TreeViewBuilder,
};
use uuid::Uuid;

use crate::{
    command::{
        edit_property::{EditProperty, EditResult},
        UndoRedoManager,
    },
    reference_store::ReferenceStore,
};

pub fn component_property_editor(
    ui: &mut Ui,
    component: &mut GraphicDefinition,
    secondary_selection: &mut Option<Uuid>,
    _reference_store: &ReferenceStore,
    undo_redo_manager: &mut UndoRedoManager,
) {
    let mut edit_result = EditResult::None;

    ui.label("Name:");
    edit_result |= ui.text_edit_singleline(&mut component.name).into();

    ui.separator();

    ui.label("Elements:");
    ui.group(|ui| {
        egui::ScrollArea::horizontal()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                edit_result |= show_element_tree(ui, secondary_selection, component);
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
                component.items.items.push(GraphicItem::Cell(Cell::new()));
                edit_result = EditResult::FromId(ui.id());
                ui.close_menu();
            }
            if ui.selectable_label(false, "Clip Area").clicked() {
                component
                    .items
                    .items
                    .push(GraphicItem::ClipArea(ClipArea::new()));
                edit_result = EditResult::FromId(ui.id());
                ui.close_menu();
            }
            if ui.selectable_label(false, "Driver Table").clicked() {
                component
                    .items
                    .items
                    .push(GraphicItem::DriverTable(DriverTable::new()));
                edit_result = EditResult::FromId(ui.id());
                ui.close_menu();
            }
        });

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
    secondary_selection: &mut Option<Uuid>,
    graphic: &mut GraphicDefinition,
) -> EditResult {
    let mut edit_result = EditResult::None;
    let res = TreeView::new(ui.make_persistent_id("Component element tree"))
        .row_layout(RowLayout::AlignedIcons)
        .show(ui, |mut builder| {
            builder.node(NodeBuilder::dir(graphic.id), |ui| {
                _ = ui.label(&graphic.name)
            });
            graphic.items.walk(&mut |element, method| {
                element_tree_node(&mut builder, element, method);
                ControlFlow::Continue::<()>(())
            });
            builder.close_dir();
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
            target: Uuid,
            position: DropPosition<Uuid>,
        },
        Remove {
            id: Uuid,
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
                GraphicItem::ClipArea(_) | GraphicItem::DriverTable(_) => {
                    (node_id, DropPosition::Last)
                }
            };
            if ui.button("add cell").clicked() {
                commands.push(Command::Add {
                    element: GraphicItem::Cell(Cell::new()),
                    target,
                    position,
                });
                ui.close_menu();
            }
            if ui.button("add clip area").clicked() {
                commands.push(Command::Add {
                    element: GraphicItem::ClipArea(ClipArea::new()),
                    target,
                    position,
                });
                ui.close_menu();
            }
            if ui.button("add driver table").clicked() {
                commands.push(Command::Add {
                    element: GraphicItem::DriverTable(DriverTable::new()),
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

fn element_tree_node(builder: &mut TreeViewBuilder<Uuid>, element: &GraphicItem, method: Method) {
    match (method, element) {
        (Method::Visit, GraphicItem::Cell(cell)) => {
            builder.node(
                NodeBuilder::leaf(cell.id).icon(|ui| {
                    egui::Image::new(egui::include_image!("../../../../images/cell.png"))
                        .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                        .paint_at(ui, ui.max_rect());
                }),
                |ui| {
                    ui.horizontal(|ui| {
                        ui.colored_label(Color32::from_gray(120), "Cell");
                        ui.label(&cell.name);
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
                        ui.colored_label(Color32::from_gray(120), "Clip area");
                        ui.label(&clip_area.name);
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
                        ui.colored_label(Color32::from_gray(120), "Driver table");
                        ui.label(&driver_table.name);
                    });
                },
            );
        }
        (Method::Leave, GraphicItem::DriverTable(_)) => {
            builder.close_dir();
        }
    }
}

fn remove_element(component: &mut GraphicDefinition, id: Uuid) -> Option<GraphicItem> {
    if let Some(index) = component.items.items.iter().position(|e| e.id() == id) {
        return Some(component.items.items.remove(index));
    }
    let r = component.items.walk_mut(&mut |e, method| {
        if method != Method::Visit {
            return ControlFlow::Continue(());
        }
        match e {
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
    target: Uuid,
    position: DropPosition<Uuid>,
    element: GraphicItem,
) {
    if target == component.id {
        insert_into_vec(&mut component.items.items, position, element);
    } else {
        component.items.search_mut(target, |e| match e {
            GraphicItem::Cell(_) => (),
            GraphicItem::ClipArea(clip_area) => {
                insert_into_vec(&mut clip_area.items, position, element);
            }
            GraphicItem::DriverTable(driver_table) => {
                insert_into_vec(&mut driver_table.columns, position, element);
            }
        });
    }
}

fn insert_into_vec(vec: &mut Vec<GraphicItem>, position: DropPosition<Uuid>, element: GraphicItem) {
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
