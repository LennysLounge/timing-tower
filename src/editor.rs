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

use crate::{style_def::TimingTowerStyleDef, timing_tower::TimingTower, MainCamera};

use self::style_element_tree::StyleNode;

pub mod style_element_tree;

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
    pub elements: StyleNode,
    pub selected_element: Option<Uuid>,
}

pub fn setup(mut ctx: EguiContexts) {
    dear_egui::set_theme(ctx.ctx_mut(), dear_egui::SKY);
}

fn _save_style(style: &TimingTowerStyleDef) {
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
    mut _towers: Query<&mut TimingTower>,
) {
    let EditorState {
        elements,
        selected_element,
        ..
    } = &mut *state;

    occupied_space.0 = egui::SidePanel::left("Editor panel")
        .show(ctx.ctx_mut(), |ui| {
            if ui.button("Save").clicked() {
                //save_style(&state.style_def);
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

            if let Some(element) = selected_element.and_then(|id| elements.find_mut(&id)) {
                element.property_editor(ui);
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    // push new style to the towers
    // for mut tower in towers.iter_mut() {
    //     tower.style_def = state.style_def.clone();
    // }
}

// fn component_tree(ui: &mut Ui, state: &mut EditorState) {
//     ui.selectable_value(
//         &mut state.tower_selection,
//         TimingTowerSelection::None,
//         "None",
//     );

//     fn tree_node(
//         ui: &mut Ui,
//         state: &mut EditorState,
//         selection: TimingTowerSelection,
//         label: impl Into<WidgetText>,
//         add_body: impl FnOnce(&mut Ui, &mut EditorState),
//     ) {
//         let id = ui.next_auto_id();
//         let (_, _header_response, _) = CollapsingState::load_with_default_open(ui.ctx(), id, true)
//             .show_header(ui, |ui| {
//                 let res = ui.selectable_value(&mut state.tower_selection, selection, label);
//                 res.double_clicked()
//             })
//             .body(|ui| {
//                 add_body(ui, state);
//             });
//         // if header_response.inner {
//         //     if let Some(mut state) = CollapsingState::load(ui.ctx(), id) {
//         //         state.toggle(ui);
//         //         state.store(ui.ctx());
//         //     }
//         // }
//     }

//     tree_node(
//         ui,
//         state,
//         TimingTowerSelection::Tower,
//         "Timing tower",
//         |ui, state| {
//             let _ = ui.button("+ Add cell");
//             tree_node(
//                 ui,
//                 state,
//                 TimingTowerSelection::Table,
//                 "Table",
//                 |ui, state| {
//                     let _ = ui.button("+ Add cell");
//                     tree_node(ui, state, TimingTowerSelection::Row, "Row", |ui, state| {
//                         let _ = ui.button("+ Add cell");
//                         for column_name in state.style_def.table.row_style.columns.keys() {
//                             ui.selectable_value(
//                                 &mut state.tower_selection,
//                                 TimingTowerSelection::RowCell(column_name.to_string()),
//                                 column_name,
//                             );
//                         }
//                     });
//                 },
//             );
//         },
//     );
// }

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
