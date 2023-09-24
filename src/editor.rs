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
    egui::{self, Rect},
    EguiContexts,
};
use tracing::error;
use uuid::Uuid;

use crate::{variable_repo::VariableRepo, MainCamera};

use self::style_elements::{RootElement, StyleElement};

pub mod properties;
pub mod style_elements;
pub mod timing_tower_elements;
pub mod variable_element;

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(OccupiedSpace(0.0))
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
struct OccupiedSpace(f32);

#[derive(Resource)]
pub struct EditorState {
    pub selected_element: Option<Uuid>,
}

pub fn setup(mut ctx: EguiContexts) {
    dear_egui::set_theme(ctx.ctx_mut(), dear_egui::SKY);
}

fn save_style(style: &RootElement) {
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
    mut elements: ResMut<RootElement>,
    mut variable_repo: ResMut<VariableRepo>,
) {
    let EditorState {
        selected_element, ..
    } = &mut *state;

    occupied_space.0 = egui::SidePanel::left("Editor panel")
        .show(ctx.ctx_mut(), |ui| {
            if ui.button("Save").clicked() {
                save_style(&elements);
            }

            egui::Frame::group(ui.style()).show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    elements.element_tree(ui, selected_element);
                });
                ui.allocate_rect(
                    Rect::from_min_size(
                        ui.cursor().min,
                        egui::Vec2 {
                            x: ui.available_width(),
                            y: 0.0,
                        },
                    ),
                    egui::Sense::hover(),
                );
            });

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    egui::SidePanel::right("Property panel")
        .show(ctx.ctx_mut(), |ui| {
            if let Some(element) = selected_element.and_then(|id| elements.find_mut(&id)) {
                element.property_editor(ui, &variable_repo);
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    variable_repo.reload_repo(&elements.vars);
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
    viewport.physical_size.x = (window.width() - occupied_space.0) as u32;
    viewport.physical_size.y = window.height() as u32;
    viewport.physical_position.x = occupied_space.0 as u32;
}
