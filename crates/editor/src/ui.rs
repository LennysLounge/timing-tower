pub mod combo_box;
mod panels;
pub mod popup;
mod selection_manager;
mod tabs;

use std::{fs::File, io::Write};

use backend::{
    exact_variant::ExactVariant,
    savefile::{Savefile, SavefileChanged},
    style::{StyleDefinition, StyleId, StyleItem, TreePosition},
    tree_iterator::TreeItem,
};
use bevy::{
    app::{First, Update},
    ecs::{
        event::{EventReader, EventWriter},
        query::With,
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Query, Res},
    },
    prelude::{Plugin, ResMut, Resource, Startup},
};
use bevy_egui::{egui::Rect, EguiContexts};

use egui_ltreeview::{DropPosition, TreeViewState};
use tracing::error;
use unified_sim_model::Adapter;

use crate::{
    camera::{AlignCamera, EditorCamera, ResetCamera},
    GameAdapterResource, MainCamera,
};

use self::selection_manager::SelectionManager;

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
                    process_messages,
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
    selection_manager: SelectionManager,
}
impl EditorState {
    fn new() -> Self {
        Self {
            selection_manager: Default::default(),
            style_item_tree_state: Default::default(),
        }
    }
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
    GameAdaperConnectDummy,
    GameAdaperConnectACC,
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
}

fn process_messages(
    mut messages: ResMut<UiMessages>,
    mut savefile: ResMut<Savefile>,
    mut savefile_changed_event: EventWriter<SavefileChanged>,
    mut editor_style: ResMut<EditorStyle>,
    mut editor_state: ResMut<EditorState>,
    mut game_adapter: ResMut<GameAdapterResource>,
    mut reset_camera_event: EventWriter<ResetCamera>,
    mut align_camera_event: EventWriter<AlignCamera>,
    mut editor_camera: Query<&mut EditorCamera, With<MainCamera>>,
) {
    for message in messages.0.drain(0..) {
        match message {
            UiMessage::Undo => todo!(),
            UiMessage::Redo => todo!(),
            UiMessage::SceneViewport(viewport_rect) => {
                editor_camera.single_mut().raw_viewport = viewport_rect;
            }
            UiMessage::SaveStyleDefinition => {
                let s = match serde_json::to_string_pretty(savefile.style()) {
                    Ok(s) => s,
                    Err(e) => {
                        error!("Error turning style into string: {e}");
                        return;
                    }
                };
                let mut file = match File::create("style.json") {
                    Ok(f) => f,
                    Err(e) => {
                        error!("Error opening file: {e}");
                        return;
                    }
                };
                if let Err(e) = file.write_all(s.as_bytes()) {
                    error!("Cannot write to file: {e}");
                    return;
                }
            }
            UiMessage::GameAdapterClose => {
                if let Some(adapter) = game_adapter.adapter_mut() {
                    adapter.send(unified_sim_model::AdapterCommand::Close);
                }
            }
            UiMessage::GameAdaperConnectDummy => {
                game_adapter.set(Adapter::new_dummy());
            }
            UiMessage::GameAdaperConnectACC => {
                game_adapter.set(Adapter::new_acc());
            }
            UiMessage::CameraReset => {
                reset_camera_event.send(ResetCamera);
            }
            UiMessage::CameraAlign => {
                align_camera_event.send(AlignCamera);
            }
            UiMessage::StyleItemSelect(id) => {
                editor_state.style_item_tree_state.set_selected(Some(id));
            }
            UiMessage::StyleItemMove {
                source,
                target,
                position,
            } => todo!(),
            UiMessage::StyleItemInsert {
                target,
                position,
                node,
                select_node,
            } => {
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
            }
            UiMessage::StyleItemRemove(id) => {
                let _result = editor_style.0.as_enum_mut().remove(&id);
                savefile.set(editor_style.0.clone(), &mut savefile_changed_event);
            }
        }
    }
}
