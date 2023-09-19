use bevy::{
    prelude::{
        resource_exists, Camera, IntoSystemConfigs, Plugin, Query, Res, ResMut, Resource, Startup,
        UVec2, Update, With,
    },
    render::camera::Viewport,
    window::{PrimaryWindow, Window},
};
use bevy_egui::{
    egui::{self, DragValue, Ui},
    EguiContexts,
};

use crate::{
    style_def::{CellStyleDef, TimingTowerStyleDef, ValueSource},
    timing_tower::TimingTower,
    MainCamera,
};

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
    pub style_def: TimingTowerStyleDef,
}

pub fn setup(mut ctx: EguiContexts) {
    dear_egui::set_theme(ctx.ctx_mut(), dear_egui::SKY);
}

fn run_egui_main(
    mut ctx: EguiContexts,
    mut occupied_space: ResMut<OccupiedSpace>,
    mut state: ResMut<EditorState>,
    mut towers: Query<&mut TimingTower>,
) {
    occupied_space.0 = egui::SidePanel::left("Editor panel")
        .show(ctx.ctx_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.collapsing("Cell", |ui| {
                    cell_style_editor(ui, &mut state.style_def.cell);
                });
                ui.collapsing("Table", |ui| {
                    ui.collapsing("Cell", |ui| {
                        cell_style_editor(ui, &mut state.style_def.table.cell);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Row offset x:");
                        ui.add(egui::DragValue::new(
                            &mut state.style_def.table.row_offset.x,
                        ));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Row offset y:");
                        ui.add(egui::DragValue::new(
                            &mut state.style_def.table.row_offset.y,
                        ));
                    });
                    ui.collapsing("Row", |ui| {
                        ui.collapsing("Cell", |ui| {
                            cell_style_editor(ui, &mut state.style_def.table.row_style.cell);
                        });
                        for (column_name, column_style) in
                            state.style_def.table.row_style.columns.iter_mut()
                        {
                            ui.collapsing(column_name, |ui| {
                                cell_style_editor(ui, column_style);
                            });
                        }
                    });
                });
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    // push new style to the towers
    for mut tower in towers.iter_mut() {
        tower.style_def = state.style_def.clone();
    }
}

fn cell_style_editor(ui: &mut Ui, style: &mut CellStyleDef) {
    ui.horizontal(|ui| {
        ui.label("value source:");
        egui::ComboBox::from_id_source("cell value source")
            .selected_text(match &style.value_source {
                ValueSource::FixedValue(_) => "Fixed value",
                ValueSource::DriverName => "Driver name",
                ValueSource::Position => "Position",
            })
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(
                        matches!(style.value_source, ValueSource::FixedValue(_)),
                        "Fixed value",
                    )
                    .clicked()
                {
                    style.value_source = ValueSource::FixedValue("".to_string());
                };
                if ui
                    .selectable_label(
                        matches!(style.value_source, ValueSource::DriverName),
                        "Driver name",
                    )
                    .clicked()
                {
                    style.value_source = ValueSource::DriverName;
                };
                if ui
                    .selectable_label(
                        matches!(style.value_source, ValueSource::Position),
                        "Position",
                    )
                    .clicked()
                {
                    style.value_source = ValueSource::Position;
                };
            });
    });
    if let ValueSource::FixedValue(s) = &mut style.value_source {
        ui.horizontal(|ui| {
            ui.label("Text:");
            ui.text_edit_singleline(s);
        });
    }
    ui.horizontal(|ui| {
        ui.label("Background color:");
        let mut color = style.color.as_rgba_f32();
        ui.color_edit_button_rgba_unmultiplied(&mut color);
        style.color = color.into();
    });
    ui.horizontal(|ui| {
        ui.label("Pos x:");
        ui.add(DragValue::new(&mut style.pos.x));
    });
    ui.horizontal(|ui| {
        ui.label("Pos y:");
        ui.add(DragValue::new(&mut style.pos.y));
    });
    ui.horizontal(|ui| {
        ui.label("Pos z:");
        ui.add(DragValue::new(&mut style.pos.z));
    });
    ui.horizontal(|ui| {
        ui.label("Width:");
        ui.add(DragValue::new(&mut style.size.x).clamp_range(0.0..=f32::MAX));
    });
    ui.horizontal(|ui| {
        ui.label("Height:");
        ui.add(DragValue::new(&mut style.size.y).clamp_range(0.0..=f32::MAX));
    });
    ui.horizontal(|ui| {
        ui.label("Skew:");
        ui.add(DragValue::new(&mut style.skew));
    });
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
