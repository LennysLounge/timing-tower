use std::{fs::File, io::Write};

use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseWheel},
        ButtonState,
    },
    math::{vec2, vec3},
    prelude::{
        resource_exists, AssetEvent, AssetServer, Camera, Camera2dBundle, Commands, Component,
        EventReader, Image, IntoSystemConfigs, Plugin, Query, Res, ResMut, Resource, Startup,
        Transform, UVec2, Update, Vec2, With,
    },
    render::camera::Viewport,
    window::{PrimaryWindow, Window},
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
    asset_reference_repo::AssetReferenceRepo,
    asset_repo::AssetRepo,
    style::{StyleDefinition, StyleTreeNode, TreeViewAction},
    MainCamera,
};

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    ui.run_if(resource_exists::<EditorState>()),
                    set_camera_viewport,
                )
                    .chain(),
            )
            .add_systems(Update, camera_drag)
            .add_systems(Update, update_asset_load_state);
    }
}

fn setup(mut commands: Commands, mut ctx: EguiContexts) {
    dear_egui::set_theme(ctx.ctx_mut(), dear_egui::SKY);

    commands.spawn((Camera2dBundle::default(), MainCamera, CameraDrag::new()));
}

#[derive(Component)]
struct CameraDrag {
    drag_position: Option<Vec2>,
    scale: f32,
}
impl CameraDrag {
    fn new() -> Self {
        Self {
            drag_position: None,
            scale: 1.0,
        }
    }
}

fn camera_drag(
    mut mouse_events: EventReader<MouseButtonInput>,
    mut scroll_events: EventReader<MouseWheel>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut camera: Query<(&mut CameraDrag, &mut Transform, &Camera), With<MainCamera>>,
) {
    let window = window.single();

    let (mut camera_drag, mut camera_transform, camera) = camera.single_mut();

    for ev in mouse_events.iter() {
        match ev.state {
            ButtonState::Pressed => {
                let is_cursor_inside_viewport = |cursor_pos: &Vec2| {
                    camera.viewport.as_ref().is_some_and(|viewport| {
                        let viewport_max = (viewport.physical_position + viewport.physical_size)
                            .as_vec2()
                            - vec2(5.0, 5.0);
                        let viewport_min = viewport.physical_position.as_vec2() + vec2(5.0, 5.0);

                        viewport_max.x > cursor_pos.x
                            && viewport_max.y > cursor_pos.y
                            && viewport_min.x < cursor_pos.x
                            && viewport_min.y < cursor_pos.y
                    })
                };
                camera_drag.drag_position =
                    window.cursor_position().filter(is_cursor_inside_viewport);
            }
            ButtonState::Released => {
                camera_drag.drag_position = None;
            }
        }
    }

    for ev in scroll_events.iter() {
        if ev.y > 0.0 {
            camera_drag.scale *= 0.9;
        }
        if ev.y < 0.0 {
            camera_drag.scale /= 0.9;
        }
        camera_transform.scale = vec3(camera_drag.scale, camera_drag.scale, 1.0);
    }

    if let Some(drag_position) = camera_drag.drag_position {
        if let Some(cursor_position) = window.cursor_position() {
            let delta = drag_position - cursor_position;
            camera_transform.translation += vec3(delta.x, -delta.y, 0.0) * camera_drag.scale;
        }
        camera_drag.drag_position = window.cursor_position();
    }
}

#[derive(Resource)]
pub struct EditorState {
    dock_state: DockState<Tab>,
    viewport: Rect,
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
            viewport: Rect::NOTHING,
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
    style: &'a mut StyleDefinition,
    asset_server: &'a AssetServer,
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
                property_editor(ui, self.selected_node, self.style, self.asset_server);
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
    mut ctx: EguiContexts,
    mut state: ResMut<EditorState>,
    mut style: ResMut<StyleDefinition>,
    mut variable_repo: ResMut<AssetRepo>,
    asset_server: Res<AssetServer>,
) {
    egui::TopBottomPanel::top("Top panel")
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

    let EditorState {
        dock_state,
        viewport,
        selected_node,
    } = &mut *state;
    DockArea::new(dock_state)
        .style(egui_dock::Style::from_egui(ctx.ctx_mut().style().as_ref()))
        .show(
            ctx.ctx_mut(),
            &mut EditorTabViewer {
                viewport,
                selected_node,
                style: &mut *style,
                asset_server: &asset_server,
            },
        );

    variable_repo.reload_repo(style.vars.all_t(), style.assets.all_t());
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

fn tree_view_elements(ui: &mut Ui, selected_node: &mut Option<Uuid>, style: &mut StyleDefinition) {
    let mut actions = Vec::new();
    let res = TreeViewBuilder::new()
        .selected(*selected_node)
        .show(ui, |ui| {
            style.tree_view_elements(ui, &mut actions);
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
            } => style.insert(&target, node, position),
            TreeViewAction::Remove { node } => style.remove(&node),
            TreeViewAction::Select { node } => *selected_node = Some(node),
        }
    }
}

fn tree_view_vars(ui: &mut Ui, selected_node: &mut Option<Uuid>, style: &mut StyleDefinition) {
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
            } => style.insert(&target, node, position),
            TreeViewAction::Remove { node } => style.remove(&node),
            TreeViewAction::Select { node } => *selected_node = Some(node),
        }
    }
}

fn tree_view_assets(ui: &mut Ui, selected_node: &mut Option<Uuid>, style: &mut StyleDefinition) {
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
            } => style.insert(&target, node, position),
            TreeViewAction::Remove { node } => style.remove(&node),
            TreeViewAction::Select { node } => *selected_node = Some(node),
        }
    }
}

fn property_editor(
    ui: &mut Ui,
    selected_node: &mut Option<Uuid>,
    style: &mut StyleDefinition,
    asset_server: &AssetServer,
) {
    let mut changed = false;

    let asset_reference_repo = AssetReferenceRepo::new(&style.vars, &style.assets);
    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            changed |= selected_node
                .as_ref()
                .and_then(|id| style.find_mut(id))
                .map(|selected_node| selected_node.property_editor(ui, &asset_reference_repo))
                .is_some_and(|b| b);
        });

    if changed {
        style
            .assets
            .all_t_mut()
            .into_iter()
            .for_each(|a| a.load_asset(&*asset_server));
    }
}

// make camera only render to view not obstructed by UI
fn set_camera_viewport(
    state: Res<EditorState>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    egui_settings: Res<bevy_egui::EguiSettings>,
    mut cameras: Query<&mut Camera, With<MainCamera>>,
) {
    let mut cam = cameras.single_mut();

    let Ok(window) = primary_window.get_single() else {
        return;
    };

    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let viewport_pos = state.viewport.left_top().to_vec2() * scale_factor as f32;
    let viewport_size = state.viewport.size() * scale_factor as f32;

    cam.viewport = Some(Viewport {
        physical_position: UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32),
        physical_size: UVec2::new(viewport_size.x as u32, viewport_size.y as u32),
        depth: 0.0..1.0,
    });
}

fn update_asset_load_state(
    event: EventReader<AssetEvent<Image>>,
    asset_server: Res<AssetServer>,
    mut style: ResMut<StyleDefinition>,
) {
    if event.is_empty() {
        return;
    }
    style
        .assets
        .all_t_mut()
        .into_iter()
        .for_each(|a| a.load_asset(&*asset_server));
}
