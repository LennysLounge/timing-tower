pub mod camera;
pub mod command;
pub mod tab;

use std::{fs::File, io::Write};

use backend::{
    savefile::{Savefile, SavefileChanged},
    style::StyleDefinition,
};
use bevy::{
    app::First,
    ecs::{
        event::{EventReader, EventWriter},
        system::Res,
    },
    math::vec3,
    prelude::{
        resource_exists, IntoSystemConfigs, Plugin, Query, ResMut, Resource, Startup, Update, With,
    },
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
    reference_store::{ReferenceStore, ReferenceStorePlugin},
    MainCamera,
};

use self::{
    camera::{EditorCamera, EditorCameraPlugin},
    command::UndoRedoManager,
};

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(EditorState::new())
            .insert_resource(UndoRedoManager::default())
            .add_plugins(ReferenceStorePlugin)
            .add_plugins(EditorCameraPlugin)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                ui.run_if(resource_exists::<EditorState>())
                    .before(camera::set_camera_viewport),
            )
            .add_systems(First, savefile_changed);
    }
}

fn setup(mut ctx: EguiContexts) {
    dear_egui::set_theme(ctx.ctx_mut(), dear_egui::SKY);
}

#[derive(Resource)]
pub struct EditorState {
    dock_state: DockState<tab::Tab>,
    selected_node: Option<Uuid>,
    style: StyleDefinition,
}
impl EditorState {
    pub fn new() -> Self {
        let mut state = DockState::new(vec![tab::Tab::SceneView]);
        let tree = state.main_surface_mut();
        let [scene, _tree_view] = tree.split_left(
            NodeIndex::root(),
            0.15,
            vec![tab::Tab::Elements, tab::Tab::Variables, tab::Tab::Assets],
        );
        let [scene, _tree_view] = tree.split_right(scene, 0.8, vec![tab::Tab::PropertyEditor]);
        let [_scene, _undo_redo] = tree.split_right(scene, 0.8, vec![tab::Tab::UndoRedo]);

        Self {
            selected_node: None,
            dock_state: state,
            style: StyleDefinition::default(),
        }
    }
}

fn ui(
    reference_store: Res<ReferenceStore>,
    mut savefile: ResMut<Savefile>,
    mut ctx: EguiContexts,
    mut state: ResMut<EditorState>,
    mut editor_camera: Query<(&mut EditorCamera, &mut Transform), With<MainCamera>>,
    savefile_changed_event: EventWriter<SavefileChanged>,
    mut undo_redo_manager: ResMut<UndoRedoManager>,
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
                if ui.button("reset camera").clicked() {
                    let (mut camera, mut transform) = editor_camera.single_mut();
                    camera.scale = 1.0;
                    transform.translation = vec3(
                        transform.translation.x.round(),
                        transform.translation.y.round(),
                        transform.translation.z.round(),
                    );
                    ui.close_menu();
                }
            });
        });
    });
    egui::TopBottomPanel::bottom("Bottom panel").show(ctx.ctx_mut(), |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let (cam, trans) = editor_camera.single();
            let zoom = 1.0 / cam.scale * 100.0;
            ui.label(format!("Zoom: {:.0}%", zoom));
            ui.separator();

            ui.label(format!("Zoom raw: {:.10}", cam.scale));
            ui.separator();
            ui.label(format!("pos: {:?}", trans.translation));
        });
    });

    let EditorState {
        dock_state,
        selected_node,
        style,
    } = &mut *state;
    let viewport = &mut editor_camera.single_mut().0.raw_viewport;
    DockArea::new(dock_state)
        .style(egui_dock::Style::from_egui(ctx.ctx_mut().style().as_ref()))
        .show(
            ctx.ctx_mut(),
            &mut tab::EditorTabViewer {
                viewport,
                selected_node,
                style: style,
                reference_store: &reference_store,
                undo_redo_manager: undo_redo_manager.as_mut(),
            },
        );

    // if let Some(command) = extract_undo_redo_command(ctx.ctx_mut()) {
    //     undo_redo_manager.queue(command);
    // }
    undo_redo_manager.apply_queue(savefile.as_mut(), savefile_changed_event);
    // if style_changed {
    //     savefile.set(style.clone(), &mut save_file_changed);
    // }
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
