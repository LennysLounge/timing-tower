pub mod combo_box;
mod panels;
pub mod popup;
mod tabs;

use std::{collections::HashMap, fs::File, io::Write};

use backend::{
    exact_variant::ExactVariant,
    savefile::{Savefile, SavefileChanged},
    style::{
        graphic::{graphic_items::GraphicItemId, GraphicStateId},
        StyleDefinition, StyleId, StyleItem, TreePosition,
    },
    tree_iterator::TreeItem,
    GameAdapterResource,
};
use bevy::{
    app::{First, Update},
    ecs::{
        event::{EventReader, EventWriter},
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Local, Res, SystemState},
        world::World,
    },
    prelude::{Plugin, ResMut, Resource, Startup},
};
use bevy_egui::{
    egui::{self, Rect},
    EguiContexts,
};

use egui_ltreeview::TreeViewState;
use rand::{seq::IteratorRandom, thread_rng};
use tracing::error;
use unified_sim_model::Adapter;

use crate::camera::{AlignCamera, EditorCamera, ResetCamera};

pub struct EditorUiPlugin;
impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(EditorState::new())
            .insert_resource(EditorStyle::default())
            .insert_resource(UiMessages(Vec::new()))
            .insert_resource(tabs::TabArea::new())
            .add_systems(Startup, setup_egui_context)
            .add_systems(First, savefile_changed)
            .add_systems(
                Update,
                (
                    panels::top_panel,
                    panels::bottom_panel,
                    tabs::tab_area,
                    process_ui_messages,
                )
                    .chain()
                    .in_set(UiSystem),
            );
    }
}

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct UiSystem;

fn setup_egui_context(mut ctx: EguiContexts) {
    egui_extras::install_image_loaders(ctx.ctx_mut());
    dear_egui::set_theme(
        ctx.ctx_mut(),
        dear_egui::Theme::Sky,
        dear_egui::Font::OpenSans,
    );
}

#[derive(Resource)]
struct EditorStyle(ExactVariant<StyleItem, StyleDefinition>);
impl Default for EditorStyle {
    fn default() -> Self {
        Self(StyleDefinition::default().into())
    }
}

#[derive(Resource)]
struct EditorState {
    style_item_tree_state: TreeViewState<StyleId>,
    style_item_selection_data: HashMap<StyleId, StyleItemSelection>,
}
impl EditorState {
    fn new() -> Self {
        Self {
            style_item_tree_state: Default::default(),
            style_item_selection_data: HashMap::new(),
        }
    }
}

#[derive(Default)]
struct StyleItemSelection {
    graphic_item_tree_state: TreeViewState<GraphicItemId>,
    graphic_state_tree_state: TreeViewState<GraphicStateId>,
}

#[derive(Resource)]
struct UiMessages(Vec<UiMessage>);
impl UiMessages {
    fn push(&mut self, message: UiMessage) {
        self.0.push(message)
    }
}

fn savefile_changed(
    savefile: Res<Savefile>,
    mut editor_style: ResMut<EditorStyle>,
    mut savefile_changed_event: EventReader<SavefileChanged>,
) {
    if savefile_changed_event.is_empty() {
        return;
    }
    savefile_changed_event.clear();
    editor_style.0 = savefile.style().clone();
}

enum UiMessage {
    Undo,
    Redo,
    SceneViewport(Rect),
    SaveStyleDefinition,
    GameAdapterClose,
    GameAdapterConnectDummy,
    GameAdapterConnectACC,
    GameAdapterSelectRandomEntry,
    GameAdapterDummySetSessionType(unified_sim_model::model::SessionType),
    GameAdapterDummySetEntryAmount(usize),
    CameraReset,
    CameraAlign,
    StyleItemSelect(StyleId),
    StyleItemMove {
        source: StyleId,
        target: StyleId,
        position: TreePosition<StyleId>,
    },
    StyleItemInsert {
        target: StyleId,
        position: TreePosition<StyleId>,
        node: StyleItem,
        select_node: bool,
    },
    StyleItemRemove(StyleId),
    StyleItemEdit {
        _widget_id: egui::Id,
        _item: StyleItem,
    },
}

fn process_ui_messages(
    world: &mut World,
    mut undo_list: Local<Vec<UiMessage>>,
    mut redo_list: Local<Vec<UiMessage>>,
) {
    let messages = world
        .resource_mut::<UiMessages>()
        .0
        .drain(0..)
        .collect::<Vec<_>>();
    for message in messages {
        match message {
            UiMessage::Undo => {
                if let Some(redo_message) = undo_list
                    .pop()
                    .and_then(|message| process_message(message, world))
                {
                    redo_list.push(redo_message);
                }
            }
            UiMessage::Redo => {
                if let Some(redo_message) = redo_list
                    .pop()
                    .and_then(|message| process_message(message, world))
                {
                    undo_list.push(redo_message);
                }
            }
            _ => {
                if let Some(undo_message) = process_message(message, world) {
                    undo_list.push(undo_message);
                    redo_list.clear();
                }
            }
        }
    }
}

fn process_message(message: UiMessage, world: &mut World) -> Option<UiMessage> {
    match message {
        UiMessage::Undo => {}
        UiMessage::Redo => {}
        UiMessage::SceneViewport(viewport_rect) => {
            world
                .query::<&mut EditorCamera>()
                .single_mut(world)
                .raw_viewport = viewport_rect;
        }
        UiMessage::SaveStyleDefinition => {
            let savefile = world.resource::<Savefile>();
            let s = match serde_json::to_string_pretty(savefile.style()) {
                Ok(s) => s,
                Err(e) => {
                    error!("Error turning style into string: {e}");
                    return None;
                }
            };
            let mut file = match File::create("style.json") {
                Ok(f) => f,
                Err(e) => {
                    error!("Error opening file: {e}");
                    return None;
                }
            };
            if let Err(e) = file.write_all(s.as_bytes()) {
                error!("Cannot write to file: {e}");
                return None;
            }
        }
        UiMessage::GameAdapterClose => {
            let mut game_adapter = world.resource_mut::<GameAdapterResource>();
            if let Some(adapter) = game_adapter.adapter_mut() {
                adapter.send(unified_sim_model::AdapterCommand::Close);
            }
        }
        UiMessage::GameAdapterConnectDummy => {
            let mut game_adapter = world.resource_mut::<GameAdapterResource>();
            game_adapter.set(Adapter::new_dummy());
        }
        UiMessage::GameAdapterConnectACC => {
            let mut game_adapter = world.resource_mut::<GameAdapterResource>();
            game_adapter.set(Adapter::new_acc());
        }
        UiMessage::GameAdapterSelectRandomEntry => {
            let mut game_adapter = world.resource_mut::<GameAdapterResource>();
            if let Some(adapter) = game_adapter.adapter_mut() {
                let model = adapter.model.read_raw();
                if let Some(random_entry) = model
                    .current_session()
                    .and_then(|session| session.entries.values().choose(&mut thread_rng()))
                {
                    adapter.send(unified_sim_model::AdapterCommand::FocusOnCar(
                        random_entry.id,
                    ));
                }
            }
        }
        UiMessage::GameAdapterDummySetSessionType(session_type) => {
            let mut game_adapter = world.resource_mut::<GameAdapterResource>();
            if let Some(adapter) = game_adapter.adapter_mut() {
                adapter.send(unified_sim_model::AdapterCommand::Game(
                    unified_sim_model::GameAdapterCommand::Dummy(
                        unified_sim_model::games::dummy::DummyCommands::SetSessionType(
                            session_type,
                        ),
                    ),
                ))
            }
        }
        UiMessage::GameAdapterDummySetEntryAmount(amount) => {
            let mut game_adapter = world.resource_mut::<GameAdapterResource>();
            if let Some(adapter) = game_adapter.adapter_mut() {
                adapter.send(unified_sim_model::AdapterCommand::Game(
                    unified_sim_model::GameAdapterCommand::Dummy(
                        unified_sim_model::games::dummy::DummyCommands::SetEntryAmount(amount),
                    ),
                ));
            }
        }
        UiMessage::CameraReset => {
            world.send_event(ResetCamera);
        }
        UiMessage::CameraAlign => {
            world.send_event(AlignCamera);
        }
        UiMessage::StyleItemSelect(id) => {
            world
                .resource_mut::<EditorState>()
                .style_item_tree_state
                .set_selected(Some(id));
        }
        UiMessage::StyleItemMove {
            source,
            target,
            position,
        } => {
            let mut system_state: SystemState<(ResMut<EditorState>, ResMut<EditorStyle>)> =
                SystemState::new(world);
            let (mut editor_state, mut editor_style) = system_state.get_mut(world);

            if let Some((removed_node, removed_position)) =
                editor_style.0.as_enum_mut().remove(&source)
            {
                let removed_from = editor_state.style_item_tree_state.parent_id_of(source);

                editor_style
                    .0
                    .as_enum_mut()
                    .insert(removed_node, &target, position);
                editor_state
                    .style_item_tree_state
                    .expand_parents_of(target, true);

                if let Some(removed_from) = removed_from {
                    return Some(UiMessage::StyleItemMove {
                        source,
                        target: removed_from,
                        position: removed_position,
                    });
                }
            }
        }
        UiMessage::StyleItemInsert {
            target,
            position,
            node,
            select_node,
        } => {
            let mut system_state: SystemState<(
                ResMut<EditorState>,
                ResMut<EditorStyle>,
                ResMut<Savefile>,
                EventWriter<SavefileChanged>,
            )> = SystemState::new(world);
            let (mut editor_state, mut editor_style, mut savefile, mut savefile_changed_event) =
                system_state.get_mut(world);

            let node_id = node.id();
            if select_node {
                editor_state
                    .style_item_tree_state
                    .set_selected(Some(node.id()));
                editor_state
                    .style_item_tree_state
                    .expand_parents_of(target, true);
            }
            editor_style.0.as_enum_mut().insert(node, &target, position);
            savefile.set(editor_style.0.clone(), &mut savefile_changed_event);

            return Some(UiMessage::StyleItemRemove(node_id));
        }
        UiMessage::StyleItemRemove(id) => {
            let mut system_state: SystemState<(
                ResMut<EditorState>,
                ResMut<EditorStyle>,
                ResMut<Savefile>,
                EventWriter<SavefileChanged>,
            )> = SystemState::new(world);
            let (editor_state, mut editor_style, mut savefile, mut savefile_changed_event) =
                system_state.get_mut(world);

            let parent_id = editor_state.style_item_tree_state.parent_id_of(id);

            let remove_result = editor_style.0.as_enum_mut().remove(&id);
            savefile.set(editor_style.0.clone(), &mut savefile_changed_event);

            if let Some(((item, position), parent_id)) = remove_result.zip(parent_id) {
                return Some(UiMessage::StyleItemInsert {
                    target: parent_id,
                    position,
                    node: item,
                    select_node: false,
                });
            }
        }
        UiMessage::StyleItemEdit {
            _widget_id: _,
            _item: _,
        } => {
            let mut system_state: SystemState<(
                ResMut<EditorStyle>,
                ResMut<Savefile>,
                EventWriter<SavefileChanged>,
            )> = SystemState::new(world);
            let (editor_style, mut savefile, mut savefile_changed_event) =
                system_state.get_mut(world);

            savefile.set(editor_style.0.clone(), &mut savefile_changed_event);
        }
    }
    None
}
