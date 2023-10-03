use std::{fs::File, io::Write};

use bevy::{
    prelude::{
        resource_exists, Camera, IntoSystemConfigs, Plugin, Query, Res, ResMut, Resource, Startup,
        UVec2, Update, With,
    },
    render::camera::Viewport,
    window::{PrimaryWindow, Window},
};
use bevy_egui::{
    egui::{self},
    EguiContexts,
};
use tracing::error;
use tree_view::{TreeNodeConverstions, TreeView};

use crate::{style::StyleDefinition, variable_repo::VariableRepo, MainCamera};

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(OccupiedSpace {
            top: 0.0,
            left: 0.0,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                run_egui_main.run_if(resource_exists::<EditorState>()),
                update_camera,
            )
                .chain(),
        );
    }
}

#[derive(Resource)]
struct OccupiedSpace {
    top: f32,
    left: f32,
}

#[derive(Resource)]
pub struct EditorState {
    pub tree: TreeView,
}

pub fn setup(mut ctx: EguiContexts) {
    dear_egui::set_theme(ctx.ctx_mut(), dear_egui::SKY);
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

fn run_egui_main(
    mut ctx: EguiContexts,
    mut occupied_space: ResMut<OccupiedSpace>,
    mut state: ResMut<EditorState>,
    mut style: ResMut<StyleDefinition>,
    mut variable_repo: ResMut<VariableRepo>,
) {
    occupied_space.top = egui::TopBottomPanel::top("Top panel")
        .show(ctx.ctx_mut(), |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        save_style(&style);
                        ui.close_menu();
                    }
                });
            });
        })
        .response
        .rect
        .height();

    occupied_space.left = egui::SidePanel::left("Editor panel")
        .show(ctx.ctx_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                state.tree.show(ui, style.as_dyn_mut());
            });

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    egui::SidePanel::right("Property panel")
        .show(ctx.ctx_mut(), |ui| {
            state
                .tree
                .selected
                .map(|id| style.property_editor(ui, &variable_repo, &id));

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    variable_repo.reload_repo(&style.vars.vars);
}

fn update_camera(
    mut cameras: Query<&mut Camera, With<MainCamera>>,
    occupied_space: Res<OccupiedSpace>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let mut camera = cameras.single_mut();
    let viewport = camera.viewport.get_or_insert_with(|| Viewport {
        physical_position: UVec2::new(0, 0),
        physical_size: UVec2::new(window.width() as u32, window.height() as u32),
        depth: 0.0..1.0,
    });
    viewport.physical_size.x = (window.width() - occupied_space.left) as u32;
    viewport.physical_size.y = (window.height() - occupied_space.top) as u32;
    viewport.physical_position.x = occupied_space.left as u32;
    viewport.physical_position.y = occupied_space.top as u32;
}
