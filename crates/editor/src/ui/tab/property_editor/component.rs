use std::ops::ControlFlow;

use backend::style::{
    cell::{ClipArea, FreeCell},
    component::{Component, Element, FreeClipArea},
};
use bevy_egui::egui::{self, vec2, Id, Pos2, Ui};
use egui_ltreeview::{
    builder::{CloserState, NodeBuilder},
    DropPosition, RowLayout, TreeView, TreeViewBuilder,
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
    reference_store: &ReferenceStore,
    undo_redo_manager: &mut UndoRedoManager,
) {
    let mut edit_result = EditResult::None;

    ui.label("Name:");
    edit_result |= ui.text_edit_singleline(&mut component.name).into();

    ui.separator();

    ui.label("Elements:");
    ui.group(|ui| {
        show_element_tree(ui, &mut component.elements);

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
                component.elements.push(Element::Cell(FreeCell::new()));
                edit_result = EditResult::FromId(ui.id());
                ui.close_menu();
            }
            if ui.selectable_label(false, "Clip Area").clicked() {
                component
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

fn show_element_tree(ui: &mut Ui, elements: &mut Vec<Element>) -> EditResult {
    let res = TreeView::new(ui.make_persistent_id("Component element tree"))
        .row_layout(RowLayout::AlignedIcons)
        .show(ui, |mut builder| {
            add_elements_to_tree(&mut builder, elements);
        });

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
        if let Some(element) = find_element(elements, &node_id) {
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
                Element::DriverTable => todo!(),
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
        }
        // match element {
        //     Element::Cell(cell) => {}
        //     Element::ClipArea(_) => todo!(),
        //     Element::DriverTable => todo!(),
        // }
    });

    let mut edit_result = EditResult::None;
    for command in commands {
        match command {
            Command::Add {
                element,
                target,
                position,
            } => {
                add(elements, element, target, position);
                edit_result = EditResult::FromId(Id::new("Component element Tree view edit"));
            }
            Command::Remove { id } => {
                remove(elements, id);
                edit_result = EditResult::FromId(Id::new("Component element Tree view edit"));
            }
        }
    }

    edit_result
}

fn add_elements_to_tree(builder: &mut TreeViewBuilder<Uuid>, elements: &Vec<Element>) {
    for element in elements.iter() {
        match element {
            Element::Cell(cell) => {
                builder.node(
                    NodeBuilder::leaf(cell.id).icon(|ui| {
                        egui::Image::new(egui::include_image!("../../../../images/article.png"))
                            .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                            .paint_at(ui, ui.max_rect());
                    }),
                    |ui| _ = ui.label(&cell.name),
                );
            }
            Element::ClipArea(clip_area) => {
                builder.node(
                    NodeBuilder::dir(clip_area.id).icon(|ui| {
                        egui::Image::new(egui::include_image!(
                            "../../../../images/array.png"
                        ))
                        .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                        .paint_at(ui, ui.max_rect());
                    }), // .closer(folder_closer)
                    |ui| _ = ui.label("Clip Area"),
                );
                add_elements_to_tree(builder, &clip_area.elements);
                builder.close_dir();
            }
            Element::DriverTable => todo!(),
        }
    }
}

fn folder_closer(ui: &mut Ui, state: CloserState) {
    let color = if state.is_hovered {
        ui.visuals().widgets.hovered.fg_stroke.color
    } else {
        ui.visuals().widgets.noninteractive.fg_stroke.color
    };
    if state.is_open {
        egui::Image::new(egui::include_image!("../../../../images/folder_open.png"))
            .tint(color)
            .paint_at(ui, ui.max_rect());
    } else {
        egui::Image::new(egui::include_image!("../../../../images/folder.png"))
            .tint(color)
            .paint_at(ui, ui.max_rect());
    }
}

fn find_element<'a>(elements: &'a [Element], id: &Uuid) -> Option<&'a Element> {
    for element in elements.iter() {
        if element.id() == id {
            return Some(element);
        }
        let x = match element {
            Element::ClipArea(clip_area) => find_element(&clip_area.elements, id),
            Element::DriverTable => todo!(),
            Element::Cell(_) => None,
        };
        if x.is_some() {
            return x;
        }
    }
    None
}
// fn add_element(elements: &mut Vec<Element>, target: Uuid, position: DropPosition<Uuid>) {

// }

fn remove(elements: &mut Vec<Element>, id: Uuid) -> Option<Element> {
    if let Some(index) = elements.iter().position(|e| *e.id() == id) {
        return Some(elements.remove(index));
    }
    for element in elements.iter_mut() {
        let x = match element {
            Element::Cell(_) => None,
            Element::ClipArea(clip_area) => remove(&mut clip_area.elements, id),
            Element::DriverTable => todo!(),
        };
        if x.is_some() {
            return x;
        }
    }
    return None;
}

fn add(
    elements: &mut Vec<Element>,
    mut element_to_add: Element,
    target: Uuid,
    position: DropPosition<Uuid>,
) -> ControlFlow<(), Element> {
    if target == Uuid::default() {
        add_element_to_list(elements, element_to_add, position);
        return ControlFlow::Break(());
    }

    for element in elements.iter_mut() {
        match element {
            Element::Cell(_) => (),
            Element::ClipArea(clip_area) => {
                if target == clip_area.id {
                    add_element_to_list(&mut clip_area.elements, element_to_add, position);
                    return ControlFlow::Break(());
                }
                element_to_add = add(&mut clip_area.elements, element_to_add, target, position)?;
            }
            Element::DriverTable => todo!(),
        }
    }
    ControlFlow::Continue(element_to_add)
}

fn add_element_to_list(
    elements: &mut Vec<Element>,
    element: Element,
    position: DropPosition<Uuid>,
) {
    match position {
        DropPosition::First => elements.insert(0, element),
        DropPosition::Last => elements.push(element),
        DropPosition::After(ref id) => {
            if let Some(index) = elements.iter().position(|e| e.id() == id) {
                elements.insert(index + 1, element);
            }
        }
        DropPosition::Before(ref id) => {
            if let Some(index) = elements.iter().position(|e| e.id() == id) {
                elements.insert(index + 1, element);
            }
        }
    }
}
