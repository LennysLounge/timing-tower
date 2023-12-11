pub mod camera;

use std::{fs::File, io::Write};

use backend::{
    savefile::{Savefile, SavefileChanged},
    style::{visitor::Visitable, StyleDefinition},
};
use bevy::{
    ecs::{event::EventWriter, system::Res},
    math::vec3,
    prelude::{
        resource_exists, IntoSystemConfigs, Plugin, Query, ResMut, Resource, Startup, Update, With,
    },
    transform::components::Transform,
};
use bevy_egui::{
    egui::{self, Rect, ScrollArea, Ui},
    EguiContexts,
};
use egui_dock::{DockArea, DockState, NodeIndex, TabViewer};
use tracing::error;
use tree_view::TreeViewBuilder;
use uuid::Uuid;

use crate::{
    reference_store::{ReferenceStore, ReferenceStorePlugin},
    style::{
        tree::{StyleTreeNode, TreeViewAction},
        tree_view_visitor::TreeViewVisitor,
        StyleDefinitionUiThings, StyleModel,
    },
    MainCamera,
};

use self::camera::{EditorCamera, EditorCameraPlugin};

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(EditorState::new())
            .add_plugins(ReferenceStorePlugin)
            .add_plugins(EditorCameraPlugin)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                ui.run_if(resource_exists::<EditorState>())
                    .before(camera::set_camera_viewport),
            );
    }
}

fn setup(mut ctx: EguiContexts) {
    dear_egui::set_theme(ctx.ctx_mut(), dear_egui::SKY);
}

#[derive(Resource)]
pub struct EditorState {
    dock_state: DockState<Tab>,
    selected_node: Option<Uuid>,
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
        let [_scene, _tree_view] = tree.split_right(scene, 0.8, vec![Tab::PropertyEditor]);

        Self {
            selected_node: None,
            dock_state: state,
        }
    }
}

enum Tab {
    SceneView,
    Elements,
    Variables,
    Assets,
    PropertyEditor,
}

struct EditorTabViewer<'a> {
    viewport: &'a mut Rect,
    selected_node: &'a mut Option<Uuid>,
    style: &'a mut StyleModel,
    style_changed: &'a mut bool,
    reference_store: &'a ReferenceStore,
}
impl<'a> TabViewer for EditorTabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::SceneView => "Scene view".into(),
            Tab::Elements => "Elements".into(),
            Tab::PropertyEditor => "Style".into(),
            Tab::Variables => "Variables".into(),
            Tab::Assets => "Assets".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::SceneView => {
                *self.viewport = ui.clip_rect();
            }
            Tab::Elements => {
                tree_view_elements(ui, self.selected_node, self.style);
            }
            Tab::PropertyEditor => {
                property_editor(
                    ui,
                    self.selected_node,
                    self.style,
                    self.style_changed,
                    self.reference_store,
                );
            }
            Tab::Variables => {
                tree_view_vars(ui, self.selected_node, self.style);
            }
            Tab::Assets => {
                tree_view_assets(ui, self.selected_node, self.style);
            }
        }
    }

    fn scroll_bars(&self, _tab: &Self::Tab) -> [bool; 2] {
        [false; 2]
    }

    fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
        false
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        !matches!(tab, Tab::SceneView)
    }
}

fn ui(
    reference_store: Res<ReferenceStore>,
    mut savefile: ResMut<Savefile>,
    mut ctx: EguiContexts,
    mut state: ResMut<EditorState>,
    mut editor_camera: Query<(&mut EditorCamera, &mut Transform), With<MainCamera>>,
    mut save_file_changed: EventWriter<SavefileChanged>,
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
    } = &mut *state;
    let viewport = &mut editor_camera.single_mut().0.raw_viewport;
    let mut style_changed = false;
    let mut style_model = StyleModel::new(savefile.style());
    DockArea::new(dock_state)
        .style(egui_dock::Style::from_egui(ctx.ctx_mut().style().as_ref()))
        .show(
            ctx.ctx_mut(),
            &mut EditorTabViewer {
                viewport,
                selected_node,
                style: &mut style_model,
                style_changed: &mut style_changed,
                reference_store: &reference_store,
            },
        );

    if style_changed {
        println!("style was changed.");
        savefile.set(&style_model.def, &mut save_file_changed);
    }
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

fn tree_view_elements(ui: &mut Ui, _selected_node: &mut Option<Uuid>, style: &mut StyleModel) {
    ScrollArea::vertical().show(ui, |ui| {
        egui_ltreeview::TreeViewBuilder::new(
            ui,
            ui.make_persistent_id("element_tree_view"),
            |root| {
                style.def.walk(&mut TreeViewVisitor { builder: root });
            },
        );
    });

    // let mut actions = Vec::new();
    // let res = TreeViewBuilder::new()
    //     .selected(*selected_node)
    //     .show(ui, |ui| {
    //         style.tree_view_elements(ui, &mut actions);
    //     });
    // *selected_node = res.selected;

    // // Set the curso to no drop to show if the drop is not allowed
    // if let Some(hovered_action) = &res.hovered {
    //     if !style.can_drop(hovered_action) {
    //         ui.ctx().set_cursor_icon(egui::CursorIcon::NoDrop);
    //     }
    // }

    // // perform the drop action.
    // if let Some(drop_action) = &res.dropped {
    //     style.perform_drop(drop_action);
    // }

    // for action in actions {
    //     match action {
    //         TreeViewAction::Insert {
    //             target,
    //             node,
    //             position,
    //         } => StyleDefinitionUiThings::insert(style, &target, node, position),
    //         TreeViewAction::Remove { node } => StyleDefinitionUiThings::remove(style, &node),
    //         TreeViewAction::Select { node } => *selected_node = Some(node),
    //     }
    // }
}

fn tree_view_vars(ui: &mut Ui, selected_node: &mut Option<Uuid>, style: &mut StyleModel) {
    let mut actions = Vec::new();
    let res = TreeViewBuilder::new()
        .selected(*selected_node)
        .show(ui, |ui| {
            style.tree_view_variables(ui, &mut actions);
        });
    *selected_node = res.selected;

    // Set the curso to no drop to show if the drop is not allowed
    if let Some(hovered_action) = &res.hovered {
        if !style.can_drop(hovered_action) {
            ui.ctx().set_cursor_icon(egui::CursorIcon::NoDrop);
        }
    }

    // perform the drop action.
    if let Some(drop_action) = &res.dropped {
        style.perform_drop(drop_action);
    }

    for action in actions {
        match action {
            TreeViewAction::Insert {
                target,
                node,
                position,
            } => StyleDefinitionUiThings::insert(style, &target, node, position),
            TreeViewAction::Remove { node } => StyleDefinitionUiThings::remove(style, &node),
            TreeViewAction::Select { node } => *selected_node = Some(node),
        }
    }
}

fn tree_view_assets(ui: &mut Ui, selected_node: &mut Option<Uuid>, style: &mut StyleModel) {
    let mut actions = Vec::new();
    let res = TreeViewBuilder::new()
        .selected(*selected_node)
        .show(ui, |ui| {
            style.tree_view_assets(ui, &mut actions);
        });
    *selected_node = res.selected;

    // Set the curso to no drop to show if the drop is not allowed
    if let Some(hovered_action) = &res.hovered {
        if !style.can_drop(hovered_action) {
            ui.ctx().set_cursor_icon(egui::CursorIcon::NoDrop);
        }
    }

    // perform the drop action.
    if let Some(drop_action) = &res.dropped {
        style.perform_drop(drop_action);
    }

    for action in actions {
        match action {
            TreeViewAction::Insert {
                target,
                node,
                position,
            } => StyleDefinitionUiThings::insert(style, &target, node, position),
            TreeViewAction::Remove { node } => StyleDefinitionUiThings::remove(style, &node),
            TreeViewAction::Select { node } => *selected_node = Some(node),
        }
    }
}

fn property_editor(
    ui: &mut Ui,
    selected_node: &mut Option<Uuid>,
    style: &mut StyleModel,
    changed: &mut bool,
    reference_store: &ReferenceStore,
) {
    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if let Some(selected_node) = selected_node.as_ref().and_then(|id| style.find_mut(id)) {
                *changed |= selected_node.property_editor(ui, &reference_store);
            }
        });
}
