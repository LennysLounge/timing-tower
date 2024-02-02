use std::ops::ControlFlow;

use backend::{
    style::{
        cell::FreeCell,
        component::Component,
        elements::{Element, FreeClipArea},
    },
    tree_iterator::{Method, TreeItem, TreeIterator, TreeIteratorMut},
};
use bevy_egui::egui::{self, vec2, Id, Ui};
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
    component: &mut Component,
    _reference_store: &ReferenceStore,
    undo_redo_manager: &mut UndoRedoManager,
) {
    let mut edit_result = EditResult::None;

    ui.label("Name:");
    edit_result |= ui.text_edit_singleline(&mut component.name).into();

    ui.separator();

    ui.label("Elements:");
    ui.group(|ui| {
        edit_result |= show_element_tree(ui, component);

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
                component
                    .elements
                    .elements
                    .push(Element::Cell(FreeCell::new()));
                edit_result = EditResult::FromId(ui.id());
                ui.close_menu();
            }
            if ui.selectable_label(false, "Clip Area").clicked() {
                component
                    .elements
                    .elements
                    .push(Element::ClipArea(FreeClipArea::new()));
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

fn show_element_tree(ui: &mut Ui, component: &mut Component) -> EditResult {
    let res = TreeView::new(ui.make_persistent_id("Component element tree"))
        .row_layout(RowLayout::AlignedIcons)
        .show(ui, |mut builder| {
            builder.set_root_id(component.id);
            component.elements.walk(&mut |element, method| {
                element_tree_node(&mut builder, element, method);
                ControlFlow::Continue::<()>(())
            });
        });

    for action in res.actions.iter() {
        if let Action::Move {
            source,
            target,
            position,
        } = action
        {
            if let Some(element) = remove_element(component, *source) {
                insert_element(component, *target, *position, element);
            }
        }
    }

    enum Command {
        Add {
            element: Element,
            target: Uuid,
            position: DropPosition<Uuid>,
        },
        Remove {
            id: Uuid,
        },
    }
    let mut commands = Vec::new();
    res.context_menu(ui, |ui, node_id| {
        component.elements.search_mut(node_id, |element| {
            if ui.button("delete").clicked() {
                commands.push(Command::Remove { id: node_id });
                ui.close_menu();
            }
            let (target, position) = match element {
                Element::Cell(_) => (
                    res.parent_of(node_id).unwrap_or_default(),
                    DropPosition::After(node_id),
                ),
                Element::ClipArea(_) => (node_id, DropPosition::Last),
            };
            if ui.button("add cell").clicked() {
                commands.push(Command::Add {
                    element: Element::Cell(FreeCell::new()),
                    target,
                    position,
                });
                ui.close_menu();
            }
            if ui.button("add clip area").clicked() {
                commands.push(Command::Add {
                    element: Element::ClipArea(FreeClipArea::new()),
                    target,
                    position,
                });
                ui.close_menu();
            }
        });
    });

    let mut edit_result = EditResult::None;
    for command in commands {
        match command {
            Command::Add {
                element,
                target,
                position,
            } => {
                insert_element(component, target, position, element);
                edit_result = EditResult::FromId(Id::new("Component element Tree view edit"));
            }
            Command::Remove { id } => {
                remove_element(component, id);
                edit_result = EditResult::FromId(Id::new("Component element Tree view edit"));
            }
        }
    }

    edit_result
}

fn element_tree_node(builder: &mut TreeViewBuilder<Uuid>, element: &Element, method: Method) {
    match (method, element) {
        (Method::Visit, Element::Cell(cell)) => {
            builder.node(
                NodeBuilder::leaf(cell.id).icon(|ui| {
                    egui::Image::new(egui::include_image!("../../../../images/article.png"))
                        .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                        .paint_at(ui, ui.max_rect());
                }),
                |ui| _ = ui.label(&cell.name),
            );
        }
        (Method::Leave, Element::Cell(_)) => (),
        (Method::Visit, Element::ClipArea(clip_area)) => {
            builder.node(
                NodeBuilder::dir(clip_area.id).icon(|ui| {
                    egui::Image::new(egui::include_image!("../../../../images/array.png"))
                        .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                        .paint_at(ui, ui.max_rect());
                }), // .closer(folder_closer)
                |ui| _ = ui.label("Clip Area"),
            );
        }
        (Method::Leave, Element::ClipArea(_)) => {
            builder.close_dir();
        }
    }
}

fn remove_element(component: &mut Component, id: Uuid) -> Option<Element> {
    if let Some(index) = component
        .elements
        .elements
        .iter()
        .position(|e| e.id() == id)
    {
        return Some(component.elements.elements.remove(index));
    }
    let r = component.elements.walk_mut(&mut |e, method| {
        if method != Method::Visit {
            return ControlFlow::Continue(());
        }
        match e {
            Element::Cell(_) => ControlFlow::Continue(()),
            Element::ClipArea(clip_area) => {
                if let Some(index) = clip_area.elements.iter().position(|e| e.id() == id) {
                    ControlFlow::Break(Some(clip_area.elements.remove(index)))
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
    component: &mut Component,
    target: Uuid,
    position: DropPosition<Uuid>,
    element: Element,
) {
    if target == component.id {
        insert_into_vec(&mut component.elements.elements, position, element);
    } else {
        component.elements.search_mut(target, |e| match e {
            Element::Cell(_) => (),
            Element::ClipArea(clip_area) => {
                insert_into_vec(&mut clip_area.elements, position, element);
            }
        });
    }
}

fn insert_into_vec(vec: &mut Vec<Element>, position: DropPosition<Uuid>, element: Element) {
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