mod dashboard;
mod secondary_editor;
mod style_item;
mod style_item_tree;
mod undo_redo;

use backend::{graphic::GraphicStates, savefile::Savefile, GameAdapterResource};
use bevy::ecs::system::{Res, ResMut, Resource};
use bevy_egui::{egui, EguiContexts};
use egui_dock::{DockArea, DockState, NodeIndex, TabViewer};
use unified_sim_model::Adapter;

use crate::reference_store::ReferenceStore;

use super::{EditorState, EditorStyle, UiMessage, UiMessages};

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
            tree.split_right(scene, 0.75, vec![Tab::StyleItemEditor, Tab::UndoRedo]);

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
    reference_store: Res<ReferenceStore>,
    game_adapter: Res<GameAdapterResource>,
    mut graphic_states: ResMut<GraphicStates>,
    savefile: ResMut<Savefile>,
) {
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
                reference_store: &reference_store,
                game_adapter: game_adapter.adapter(),
                graphic_states: &mut graphic_states,
                savefile: &savefile,
            },
        );
}

enum Tab {
    SceneView,
    Dashboard,
    StyleItems,
    StyleItemEditor,
    GraphicItemEditor,
    UndoRedo,
}

struct EditorTabViewer<'a> {
    messages: &'a mut UiMessages,
    editor_style: &'a mut EditorStyle,
    editor_state: &'a mut EditorState,
    reference_store: &'a ReferenceStore,
    game_adapter: Option<&'a Adapter>,
    graphic_states: &'a mut GraphicStates,
    savefile: &'a Savefile,
}
impl<'a> TabViewer for EditorTabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::SceneView => "Scene view".into(),
            Tab::Dashboard => "Dashboard".into(),
            Tab::StyleItems => "Style".into(),
            Tab::StyleItemEditor => "Component".into(),
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
                dashboard::dashboard(
                    ui,
                    self.game_adapter,
                    self.savefile.style(),
                    self.graphic_states,
                );
            }
            Tab::StyleItems => {
                style_item_tree::tree_view(ui, self.messages, self.editor_style, self.editor_state);
            }
            Tab::StyleItemEditor => {
                style_item::editor(
                    ui,
                    self.messages,
                    self.editor_style,
                    self.editor_state,
                    self.reference_store,
                    self.game_adapter,
                );
            }
            Tab::GraphicItemEditor => {
                secondary_editor::editor(
                    ui,
                    self.messages,
                    self.editor_state,
                    self.editor_style,
                    self.reference_store,
                );
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
