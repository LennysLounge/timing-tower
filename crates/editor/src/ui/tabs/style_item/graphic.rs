use std::ops::ControlFlow;

use backend::{
    style::{
        graphic::{
            self,
            graphic_items::{
                cell::Cell, clip_area::ClipArea, driver_table::DriverTable,
                entry_context::EntryContext, root::Root, GraphicItem, GraphicItemId,
            },
            GraphicDefinition, GraphicStateId,
        },
        StyleItem,
    },
    tree_iterator::{Method, TreeItem, TreeIterator, TreeIteratorMut},
};
use bevy_egui::egui::{self, vec2, Color32, Id, RichText, Ui};
use egui_ltreeview::{
    node::NodeBuilder, Action, DropPosition, RowLayout, TreeView, TreeViewBuilder, TreeViewState,
};

use crate::ui::{EditResult, StyleItemSelection, UiMessage, UiMessages};

pub(super) fn graphic_property_editor(
    ui: &mut Ui,
    graphic: &mut GraphicDefinition,
    messages: &mut UiMessages,
    selection_data: &mut StyleItemSelection,
) {
    let mut edit_result = EditResult::None;

    ui.label("Name:");
    let res = ui.text_edit_singleline(&mut graphic.name);
    if res.changed() {
        // Keep the graphic item root in sync with the graphic name itself.
        graphic.items.name = graphic.name.clone();
    };
    edit_result |= res.into();

    ui.separator();

    ui.label("Elements:");
    ui.group(|ui| {
        egui::ScrollArea::horizontal()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                edit_result |=
                    show_element_tree(ui, graphic, &mut selection_data.graphic_item_tree_state);
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
                graphic.items.items.push(Cell::new().into());
                edit_result = EditResult::FromId(ui.id());
                ui.close_menu();
            }
            if ui.selectable_label(false, "Clip Area").clicked() {
                graphic.items.items.push(ClipArea::new().into());
                edit_result = EditResult::FromId(ui.id());
                ui.close_menu();
            }
            if ui.selectable_label(false, "Driver Table").clicked() {
                graphic.items.items.push(DriverTable::new().into());
                edit_result = EditResult::FromId(ui.id());
                ui.close_menu();
            }
            if ui.selectable_label(false, "Entry Context").clicked() {
                graphic.items.items.push(EntryContext::new().into());
                edit_result = EditResult::FromId(ui.id());
                ui.close_menu();
            }
        });

    ui.add_space(10.0);
    ui.label("States:");
    ui.group(|ui| {
        edit_result |= show_states_tree(ui, graphic, &mut selection_data.graphic_state_tree_state);

        ui.allocate_space(vec2(
            ui.available_width(),
            -ui.spacing().item_spacing.y + (100.0 - ui.min_rect().height()),
        ));
    });
    let add_button_res = ui.add_sized(vec2(ui.available_width(), 0.0), egui::Button::new("Add"));
    if add_button_res.clicked() {
        graphic.states.push(graphic::GraphicState {
            id: GraphicStateId::new(),
            name: String::from("new state"),
        });
    }

    if let EditResult::FromId(widget_id) = edit_result {
        messages.push(UiMessage::StyleItemEdit {
            widget_id,
            item: StyleItem::Graphic(graphic.clone()),
        });
    }
}

enum GraphicItemCommand {
    Add {
        element: GraphicItem,
        target: GraphicItemId,
        position: DropPosition<GraphicItemId>,
    },
    Remove {
        id: GraphicItemId,
    },
}

fn show_element_tree(
    ui: &mut Ui,
    graphic: &mut GraphicDefinition,
    tree_view_state: &mut TreeViewState<GraphicItemId>,
) -> EditResult {
    let mut edit_result = EditResult::None;
    let mut commands = Vec::new();
    let res = TreeView::new(ui.make_persistent_id("Component element tree"))
        .row_layout(RowLayout::AlignedIcons)
        .show_state(ui, tree_view_state, |mut builder| {
            graphic.items.walk(&mut |item, method| {
                element_tree_node(&mut builder, item, method, &mut commands);
                ControlFlow::Continue::<()>(())
            });
        });

    for action in res.actions.iter() {
        match action {
            Action::SetSelected(_) => (),
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
    for command in commands {
        match command {
            GraphicItemCommand::Add {
                element,
                target,
                position,
            } => {
                tree_view_state.set_selected(Some(element.id()));
                tree_view_state.expand_parents_of(target, true);
                insert_element(graphic, target, position, element);
                edit_result = EditResult::FromId(Id::new("Component element Tree view edit"));
            }
            GraphicItemCommand::Remove { id } => {
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
    commands: &mut Vec<GraphicItemCommand>,
) {
    match (method, element) {
        (Method::Visit, GraphicItem::Root(root)) => {
            let parent_id = builder.parent_id();
            builder.node(
                NodeBuilder::dir(root.id)
                    .label(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::Label::new(
                                    RichText::new("Graphic").color(Color32::from_gray(120)),
                                )
                                .selectable(false),
                            );
                            ui.add(egui::Label::new(&root.name).selectable(false));
                        });
                    })
                    .context_menu(|ui| graphic_item_context_menu(ui, element, commands, parent_id)),
            );
        }
        (Method::Leave, GraphicItem::Root(_)) => {
            builder.close_dir();
        }

        (Method::Visit, GraphicItem::Cell(cell)) => {
            let parent_id = builder.parent_id();
            builder.node(
                NodeBuilder::leaf(cell.id)
                    .icon(|ui| {
                        egui::Image::new(egui::include_image!("../../../../images/cell.png"))
                            .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                            .paint_at(ui, ui.max_rect());
                    })
                    .label(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::Label::new(
                                    RichText::new("Cell").color(Color32::from_gray(120)),
                                )
                                .selectable(false),
                            );
                            ui.add(egui::Label::new(&cell.name).selectable(false));
                        });
                    })
                    .context_menu(|ui| graphic_item_context_menu(ui, element, commands, parent_id)),
            );
        }
        (Method::Leave, GraphicItem::Cell(_)) => (),
        (Method::Visit, GraphicItem::ClipArea(clip_area)) => {
            let parent_id = builder.parent_id();
            builder.node(
                NodeBuilder::dir(clip_area.id)
                    .icon(|ui| {
                        egui::Image::new(egui::include_image!("../../../../images/clip_area.png"))
                            .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                            .paint_at(ui, ui.max_rect());
                    })
                    .label(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::Label::new(
                                    RichText::new("Clip area").color(Color32::from_gray(120)),
                                )
                                .selectable(false),
                            );
                            ui.add(egui::Label::new(&clip_area.name).selectable(false));
                        });
                    })
                    .context_menu(|ui| graphic_item_context_menu(ui, element, commands, parent_id)),
            );
        }
        (Method::Leave, GraphicItem::ClipArea(_)) => {
            builder.close_dir();
        }
        (Method::Visit, GraphicItem::DriverTable(driver_table)) => {
            let parent_id = builder.parent_id();
            builder.node(
                NodeBuilder::dir(driver_table.id)
                    .icon(|ui| {
                        egui::Image::new(egui::include_image!(
                            "../../../../images/driver_table.png"
                        ))
                        .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                        .paint_at(ui, ui.max_rect());
                    })
                    .label(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::Label::new(
                                    RichText::new("Driver table").color(Color32::from_gray(120)),
                                )
                                .selectable(false),
                            );
                            ui.add(egui::Label::new(&driver_table.name).selectable(false));
                        });
                    })
                    .context_menu(|ui| graphic_item_context_menu(ui, element, commands, parent_id)),
            );
        }
        (Method::Leave, GraphicItem::DriverTable(_)) => {
            builder.close_dir();
        }
        (Method::Visit, GraphicItem::EntryContext(entry_context)) => {
            let parent_id = builder.parent_id();
            builder.node(
                NodeBuilder::dir(entry_context.id)
                    .icon(|ui| {
                        egui::Image::new(egui::include_image!(
                            "../../../../images/entry_circle.png"
                        ))
                        .tint(ui.visuals().widgets.noninteractive.fg_stroke.color)
                        .paint_at(ui, ui.max_rect());
                    })
                    .label(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::Label::new(
                                    RichText::new("Entry context").color(Color32::from_gray(120)),
                                )
                                .selectable(false),
                            );
                            ui.add(egui::Label::new(&entry_context.name).selectable(false));
                        });
                    })
                    .context_menu(|ui| graphic_item_context_menu(ui, element, commands, parent_id)),
            )
        }
        (Method::Leave, GraphicItem::EntryContext(_)) => {
            builder.close_dir();
        }
    }
}

fn graphic_item_context_menu(
    ui: &mut Ui,
    graphic_item: &GraphicItem,
    commands: &mut Vec<GraphicItemCommand>,
    parent_id: Option<GraphicItemId>,
) {
    let (target, position) = match graphic_item {
        GraphicItem::Cell(cell) => (parent_id.unwrap_or_default(), DropPosition::After(cell.id)),
        GraphicItem::Root(Root { id, .. })
        | GraphicItem::ClipArea(ClipArea { id, .. })
        | GraphicItem::DriverTable(DriverTable { id, .. })
        | GraphicItem::EntryContext(EntryContext { id, .. }) => (*id, DropPosition::Last),
    };
    if ui.button("add cell").clicked() {
        commands.push(GraphicItemCommand::Add {
            element: Cell::new().into(),
            target,
            position,
        });
        ui.close_menu();
    }
    if ui.button("add clip area").clicked() {
        commands.push(GraphicItemCommand::Add {
            element: ClipArea::new().into(),
            target,
            position,
        });
        ui.close_menu();
    }
    if ui.button("add driver table").clicked() {
        commands.push(GraphicItemCommand::Add {
            element: DriverTable::new().into(),
            target,
            position,
        });
        ui.close_menu();
    }
    if ui.button("add entry context").clicked() {
        commands.push(GraphicItemCommand::Add {
            element: EntryContext::new().into(),
            target,
            position,
        });
        ui.close_menu();
    }
    ui.separator();
    if ui.button("delete").clicked() {
        commands.push(GraphicItemCommand::Remove {
            id: graphic_item.id(),
        });
        ui.close_menu();
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
            GraphicItem::EntryContext(entry_context) => {
                if let Some(index) = entry_context.items.iter().position(|e| e.id() == id) {
                    ControlFlow::Break(Some(entry_context.items.remove(index)))
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
        GraphicItem::EntryContext(entry_context) => {
            insert_into_vec(&mut entry_context.items, position, element);
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

fn show_states_tree(
    ui: &mut Ui,
    graphic: &mut GraphicDefinition,
    tree_view_state: &mut TreeViewState<GraphicStateId>,
) -> EditResult {
    let mut edit_result = EditResult::None;

    // Graphic states are really only a list but for the tree view it still needs a root
    // element. This is the fixed id of that element.
    const TREE_ROOT_ID: GraphicStateId =
        GraphicStateId(uuid::uuid!("65f0d4ef-4057-415d-99b3-eadb158d0d27"));
    const TEMPLATE_ID: GraphicStateId =
        GraphicStateId(uuid::uuid!("3bd691e6-4a87-4082-89da-31a7cfb3967c"));

    let tree_res = TreeView::new(ui.make_persistent_id("State tree"))
        .row_layout(RowLayout::Compact)
        .show_state(ui, tree_view_state, |mut builder| {
            builder.node(NodeBuilder::dir(TREE_ROOT_ID).flatten(true));
            builder.leaf(TEMPLATE_ID, "Template");
            for state in graphic.states.iter_mut() {
                builder.leaf(state.id, &state.name);
            }
        });
    for action in &tree_res.actions {
        match action {
            Action::SetSelected(_) => (),
            Action::Move {
                source,
                target: _,
                position,
            } => {
                if source == &TEMPLATE_ID {
                    tree_res.remove_drop_marker(ui);
                } else {
                    let source_idx = graphic.states.iter().position(|s| &s.id == source);
                    if let Some(source_idx) = source_idx {
                        let state = graphic.states.remove(source_idx);
                        match position {
                            DropPosition::First => graphic.states.insert(0, state),
                            DropPosition::Last => graphic.states.push(state),
                            DropPosition::After(id) | DropPosition::Before(id)
                                if id == &TEMPLATE_ID =>
                            {
                                graphic.states.insert(0, state);
                            }
                            DropPosition::After(id) => {
                                if let Some(index) = graphic.states.iter().position(|e| &e.id == id)
                                {
                                    graphic.states.insert(index + 1, state);
                                } else {
                                    graphic.states.push(state);
                                }
                            }
                            DropPosition::Before(id) => {
                                if let Some(index) = graphic.states.iter().position(|e| &e.id == id)
                                {
                                    graphic.states.insert(index, state);
                                } else {
                                    graphic.states.push(state);
                                }
                            }
                        }
                        edit_result = EditResult::FromId(tree_res.response.id)
                    }
                }
            }
            Action::Drag { source, .. } => {
                if source == &TEMPLATE_ID {
                    tree_res.remove_drop_marker(ui);
                }
            }
        }
    }

    edit_result
}
