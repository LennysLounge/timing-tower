pub mod combo_box;
mod tab;

use std::{fs::File, io::Write};

use backend::{
    exact_variant::ExactVariant,
    graphic::GraphicStates,
    savefile::{Savefile, SavefileChanged},
    style::{StyleDefinition, StyleItem},
};
use bevy::{
    app::{First, Update},
    ecs::{
        event::{EventReader, EventWriter},
        schedule::{IntoSystemConfigs, SystemSet},
        system::Res,
    },
    prelude::{Plugin, Query, ResMut, Resource, Startup, With},
    transform::components::Transform,
};
use bevy_egui::{
    egui::{self},
    EguiContexts,
};
use egui_dock::{DockArea, DockState, NodeIndex};
use tracing::error;
use uuid::Uuid;

use crate::{
    camera::{AlignCamera, EditorCamera, ResetCamera},
    command::{EditorCommand, UndoRedoManager},
    reference_store::ReferenceStore,
    GameAdapterResource, MainCamera,
};

use self::tab::Tab;

pub struct EditorUiPlugin;
impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(EditorState::new())
            .add_systems(Startup, setup_egui_context)
            .add_systems(First, savefile_changed)
            .add_systems(Update, ui.in_set(UiSystem));
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
struct EditorState {
    dock_state: DockState<Tab>,
    style_item_selection: Option<Uuid>,
    graphic_item_selection: Option<Uuid>,
    graphic_state_selection: Option<Uuid>,
    style: ExactVariant<StyleItem, StyleDefinition>,
}
impl EditorState {
    pub fn new() -> Self {
        let mut state = DockState::new(vec![Tab::SceneView]);
        let tree = state.main_surface_mut();
        let [scene, _tree_view] = tree.split_left(
            NodeIndex::root(),
            0.15,
            vec![Tab::Elements, Tab::Variables, Tab::Assets],
        );
        let [scene, _component_editor] =
            tree.split_right(scene, 0.8, vec![Tab::ComponentEditor, Tab::UndoRedo]);

        let [_scene, _element_editor] = tree.split_right(scene, 0.7, vec![Tab::ElementEditor]);

        Self {
            style_item_selection: None,
            graphic_item_selection: None,
            graphic_state_selection: None,
            dock_state: state,
            style: StyleDefinition::default().into(),
        }
    }
}

fn ui(
    reference_store: Res<ReferenceStore>,
    savefile_changed_event: EventWriter<SavefileChanged>,
    mut reset_camera_event: EventWriter<ResetCamera>,
    mut align_camera_event: EventWriter<AlignCamera>,
    mut savefile: ResMut<Savefile>,
    mut ctx: EguiContexts,
    mut state: ResMut<EditorState>,
    mut editor_camera: Query<(&mut EditorCamera, &mut Transform), With<MainCamera>>,
    mut undo_redo_manager: ResMut<UndoRedoManager>,
    mut game_adapter: ResMut<GameAdapterResource>,
    mut graphic_states: ResMut<GraphicStates>,
) {
    egui::TopBottomPanel::top("Top panel").show(ctx.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Save").clicked() {
                    save_style(savefile.style());
                    ui.close_menu();
                }
            });
            ui.menu_button("View", |ui| {
                if ui.button("Reset camera").clicked() {
                    reset_camera_event.send(ResetCamera);
                    ui.close_menu();
                }
                if ui.button("Align camera").clicked() {
                    align_camera_event.send(AlignCamera);
                    ui.close_menu();
                }
            });
            if ui.button("Undo").clicked() {
                undo_redo_manager.queue(EditorCommand::Undo);
            }
            if ui.button("Redo").clicked() {
                undo_redo_manager.queue(EditorCommand::Redo);
            }
        });
    });
    egui::TopBottomPanel::bottom("Bottom panel").show(ctx.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let (cam, trans) = editor_camera.single();
            let zoom = 1.0 / cam.scale * 100.0;
            ui.label(format!("Zoom: {:.0}%", zoom));
            ui.separator();
            ui.separator();
            ui.label(format!("Zoom exponent: {:.2}", cam.scale_exponent));
            ui.separator();
            ui.label(format!("pos: {:?}", trans.translation));
        });
    });

    let EditorState {
        dock_state,
        style_item_selection,
        graphic_item_selection,
        graphic_state_selection,
        style,
    } = &mut *state;
    let viewport = &mut editor_camera.single_mut().0.raw_viewport;
    DockArea::new(dock_state)
        .style(egui_dock::Style::from_egui(ctx.ctx_mut().style().as_ref()))
        .show(
            ctx.ctx_mut(),
            &mut tab::EditorTabViewer {
                viewport,
                style_item_selection,
                graphic_item_selection,
                graphic_state_selection,
                style: style,
                reference_store: &reference_store,
                undo_redo_manager: undo_redo_manager.as_mut(),
                game_adapter: &game_adapter.adapter,
            },
        );

    undo_redo_manager.apply_queue(
        savefile.as_mut(),
        savefile_changed_event,
        &mut game_adapter.adapter,
    );

    if let Some(graphic_item_selection) = state.style_item_selection {
        if let Some(state) = state.graphic_state_selection {
            graphic_states.states.insert(graphic_item_selection, state);
        } else {
            graphic_states.states.remove(&graphic_item_selection);
        }
    }
}

fn savefile_changed(
    savefile: Res<Savefile>,
    mut editor_state: ResMut<EditorState>,
    mut savefile_changed_event: EventReader<SavefileChanged>,
) {
    if savefile_changed_event.is_empty() {
        return;
    }
    savefile_changed_event.clear();
    editor_state.style = savefile.style().clone();
}

fn save_style(style: &StyleDefinition) {
    let s = match serde_json::to_string_pretty(style) {
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
