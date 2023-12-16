pub mod camera;
pub mod command;

use std::{fs::File, io::Write};

use backend::{
    savefile::{Savefile, SavefileChanged},
    style::{visitor::StyleNode, StyleDefinition},
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
    egui::{self, Rect, ScrollArea, Ui},
    EguiContexts,
};
use egui_dock::{DockArea, DockState, NodeIndex, TabViewer};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    reference_store::{ReferenceStore, ReferenceStorePlugin},
    style::visitors::{
        drop_allowed::DropAllowedVisitor,
        insert::InsertNodeVisitor,
        property_editor::PropertyEditorVisitor,
        remove::RemoveNodeVisitor,
        search::{SearchVisitor, SearchVisitorMut},
        tree_view::{TreeViewVisitor, TreeViewVisitorResult},
    },
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
    dock_state: DockState<Tab>,
    selected_node: Option<Uuid>,
    style: StyleDefinition,
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
        let [scene, _tree_view] = tree.split_right(scene, 0.8, vec![Tab::PropertyEditor]);
        let [_scene, _undo_redo] = tree.split_right(scene, 0.8, vec![Tab::UndoRedo]);

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
    let mut style_changed = false;
    DockArea::new(dock_state)
        .style(egui_dock::Style::from_egui(ctx.ctx_mut().style().as_ref()))
        .show(
            ctx.ctx_mut(),
            &mut EditorTabViewer {
                viewport,
                selected_node,
                style: style,
                style_changed: &mut style_changed,
                reference_store: &reference_store,
                undo_redo_manager: undo_redo_manager.as_mut(),
            },
        );

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

enum Tab {
    SceneView,
    Elements,
    Variables,
    Assets,
    PropertyEditor,
    UndoRedo,
}

struct EditorTabViewer<'a> {
    viewport: &'a mut Rect,
    selected_node: &'a mut Option<Uuid>,
    style: &'a mut StyleDefinition,
    style_changed: &'a mut bool,
    reference_store: &'a ReferenceStore,
    undo_redo_manager: &'a mut UndoRedoManager,
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
            Tab::UndoRedo => "Undo/Redo".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::SceneView => {
                *self.viewport = ui.clip_rect();
            }
            Tab::Elements => {
                *self.style_changed |= tree_view(ui, self.selected_node, &mut self.style.scene);
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
                *self.style_changed |= tree_view(ui, self.selected_node, &mut self.style.vars);
            }
            Tab::Assets => {
                *self.style_changed |= tree_view(ui, self.selected_node, &mut self.style.assets);
            }
            Tab::UndoRedo => {
                undo_redo(ui, &mut self.undo_redo_manager);
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

fn tree_view(
    ui: &mut Ui,
    _selected_node: &mut Option<Uuid>,
    base_node: &mut impl StyleNode,
) -> bool {
    let mut changed = false;
    let TreeViewVisitorResult {
        response,
        nodes_to_add,
        nodes_to_remove,
    } = ScrollArea::vertical()
        .show(ui, |ui| TreeViewVisitor::show(ui, base_node))
        .inner;

    // Add nodes
    for (id, position, node) in nodes_to_add {
        InsertNodeVisitor::new(id, position, node).insert_into(base_node);
    }
    // remove nodes
    for id in nodes_to_remove {
        RemoveNodeVisitor::new(id).remove_from(base_node);
    }

    if response.selected_node.is_some() {
        *_selected_node = response.selected_node;
    }

    if let Some(drop_action) = &response.drag_drop_action {
        let drop_allowed = SearchVisitor::new(drop_action.drag_id, |dragged| {
            SearchVisitor::new(drop_action.drop_id, |dropped| {
                DropAllowedVisitor::new(dragged.as_any()).test(dropped)
            })
            .search_in(base_node)
        })
        .search_in(base_node)
        .flatten()
        .unwrap_or(false);

        if !drop_allowed {
            response.remove_drop_marker(ui);
        }

        if response.dropped && drop_allowed {
            if let Some(removed_node) =
                RemoveNodeVisitor::new(drop_action.drag_id).remove_from(base_node)
            {
                InsertNodeVisitor::new(drop_action.drop_id, drop_action.position, removed_node)
                    .insert_into(base_node);
            } else {
                info!("No node was removed from the tree");
            }

            changed = true;
        }
    }
    changed
}

fn property_editor(
    ui: &mut Ui,
    selected_id: &mut Option<Uuid>,
    style: &mut StyleDefinition,
    changed: &mut bool,
    reference_store: &ReferenceStore,
) {
    let Some(selected_id) = selected_id else {
        return;
    };

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            *changed |= SearchVisitorMut::new(*selected_id, |selected_node| {
                PropertyEditorVisitor::new(ui, reference_store).apply_to(selected_node)
            })
            .search_in(style)
            .unwrap_or(false);
        });
}

fn undo_redo(ui: &mut Ui, undo_redo_manager: &mut UndoRedoManager) {
    ui.horizontal(|ui| {
        if ui.button("Undo").clicked() {
            undo_redo_manager.queue(command::EditorCommand::Undo);
        }
        if ui.button("Redo").clicked() {
            undo_redo_manager.queue(command::EditorCommand::Redo);
        }
    });
    for future_command in undo_redo_manager.future().iter() {
        ui.label(future_command.name());
    }
    ui.add_space(10.0);
    ui.label(">> Now <<");
    ui.add_space(10.0);
    for past_command in undo_redo_manager.past().iter() {
        ui.label(past_command.name());
    }
}
