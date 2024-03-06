mod dashboard;
mod element_editor;
mod property_editor;
mod style_items;
mod undo_redo;

use std::marker::PhantomData;

use backend::{
    exact_variant::ExactVariant,
    graphic::GraphicStates,
    style::{StyleDefinition, StyleItem},
    GameAdapterResource,
};
use bevy::ecs::system::{Res, ResMut, Resource};
use bevy_egui::{
    egui::{self, Rect},
    EguiContexts,
};
use egui_dock::{DockArea, DockState, NodeIndex, TabViewer};
use unified_sim_model::Adapter;

use crate::{command::UndoRedoManager, reference_store::ReferenceStore};

use super::{selection_manager::SelectionManager, EditorState, EditorStyle, UiMessage, UiMessages};

#[derive(Resource)]
pub struct TabArea {
    dock_state: DockState<Tab>,
}
impl TabArea {
    pub fn new() -> Self {
        let mut state = DockState::new(vec![Tab::SceneView, Tab::Dashboard]);
        let tree = state.main_surface_mut();
        let [scene, _tree_view] = tree.split_left(NodeIndex::root(), 0.15, vec![Tab::StyleItems]);
        let [scene, _component_editor] =
            tree.split_right(scene, 0.75, vec![Tab::GraphicEditor, Tab::UndoRedo]);

        let [_scene, _element_editor] = tree.split_right(scene, 0.7, vec![Tab::GraphicItemEditor]);

        Self { dock_state: state }
    }
}

pub(super) fn tab_area(
    mut ctx: EguiContexts,
    mut messages: ResMut<UiMessages>,
    mut tab_area: ResMut<TabArea>,
    mut editor_style: ResMut<EditorStyle>,
    mut editor_state: ResMut<EditorState>,
) {
    //let viewport = &mut editor_camera.single_mut().0.raw_viewport;
    DockArea::new(&mut tab_area.dock_state)
        .style({
            let mut style = egui_dock::Style::from_egui(ctx.ctx_mut().style().as_ref());
            style.separator.width = 3.0;
            style.separator.color_idle = ctx.ctx_mut().style().visuals.panel_fill;
            style.separator.color_dragged = ctx.ctx_mut().style().visuals.panel_fill;
            style.separator.color_hovered = ctx.ctx_mut().style().visuals.panel_fill;

            style.tab_bar.rounding = egui::Rounding::ZERO;
            style.tab_bar.bg_fill = ctx.ctx_mut().style().visuals.panel_fill;

            style.tab.hline_below_active_tab_name = false;

            style.tab.inactive.text_color = style.tab.inactive.text_color.linear_multiply(0.3);
            style
        })
        .show(
            ctx.ctx_mut(),
            &mut EditorTabViewer {
                messages: &mut messages,
                editor_style: &mut editor_style,
                editor_state: &mut editor_state,
                //editor_style: &mut editor_state.style,
                // viewport,
                // selection_manager,
                //style,
                // reference_store: &reference_store,
                // undo_redo_manager: undo_redo_manager.as_mut(),
                //game_adapter: game_adapter.adapter(),
                //graphic_states: &mut graphic_states,
            },
        );
}

enum Tab {
    SceneView,
    Dashboard,
    StyleItems,
    GraphicEditor,
    GraphicItemEditor,
    UndoRedo,
}

struct EditorTabViewer<'a> {
    messages: &'a mut UiMessages,
    editor_style: &'a mut EditorStyle,
    editor_state: &'a mut EditorState,
    //editor_style: &'a mut ExactVariant<StyleItem, StyleDefinition>,
    // pub viewport: &'a mut Rect,
    // pub selection_manager: &'a mut SelectionManager,
    //pub style: &'a mut ExactVariant<StyleItem, StyleDefinition>,
    // pub reference_store: &'a ReferenceStore,
    // pub undo_redo_manager: &'a mut UndoRedoManager,
    // pub game_adapter: Option<&'a Adapter>,
    // pub graphic_states: &'a mut GraphicStates,
}
impl<'a> TabViewer for EditorTabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::SceneView => "Scene view".into(),
            Tab::Dashboard => "Dashboard".into(),
            Tab::StyleItems => "Style".into(),
            Tab::GraphicEditor => "Component".into(),
            Tab::GraphicItemEditor => "Element".into(),
            Tab::UndoRedo => "Undo/Redo".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::SceneView => {
                self.messages.push(UiMessage::SceneViewport(ui.clip_rect()));
            }
            Tab::Dashboard => {
                //dashboard::dashboard(ui, self.game_adapter, &self.style, self.graphic_states);
            }
            Tab::StyleItems => {
                style_items::tree_view(ui, self.messages, self.editor_style, self.editor_state);
            }
            Tab::GraphicEditor => {
                // property_editor::property_editor(
                //     ui,
                //     self.selection_manager,
                //     self.style,
                //     self.reference_store,
                //     self.undo_redo_manager,
                //     self.game_adapter,
                //     self.graphic_states,
                // );
            }
            Tab::GraphicItemEditor => {
                // element_editor::element_editor(
                //     ui,
                //     self.selection_manager,
                //     self.style,
                //     self.reference_store,
                //     self.undo_redo_manager,
                //     self.graphic_states,
                // );
            }
            Tab::UndoRedo => {
                //undo_redo::undo_redo(ui, &mut self.undo_redo_manager);
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
