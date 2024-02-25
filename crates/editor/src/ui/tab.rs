mod dashboard;
mod element_editor;
mod property_editor;
mod tree_view;
mod undo_redo;

use backend::{
    exact_variant::ExactVariant,
    graphic::GraphicStates,
    style::{StyleDefinition, StyleItem},
};
use bevy_egui::egui::{self, Rect};
use egui_dock::TabViewer;
use unified_sim_model::Adapter;

use crate::{command::UndoRedoManager, reference_store::ReferenceStore};

use super::selection_manager::SelectionManager;

pub enum Tab {
    SceneView,
    Dashboard,
    StyleItems,
    GraphicEditor,
    GraphicItemEditor,
    UndoRedo,
}

pub struct EditorTabViewer<'a> {
    pub viewport: &'a mut Rect,
    pub selection_manager: &'a mut SelectionManager,
    pub style: &'a mut ExactVariant<StyleItem, StyleDefinition>,
    pub reference_store: &'a ReferenceStore,
    pub undo_redo_manager: &'a mut UndoRedoManager,
    pub game_adapter: Option<&'a Adapter>,
    pub graphic_states: &'a mut GraphicStates,
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
                *self.viewport = ui.clip_rect();
            }
            Tab::Dashboard => {
                dashboard::dashboard(ui, self.game_adapter, &self.style, self.graphic_states);
            }
            Tab::StyleItems => {
                tree_view::tree_view(
                    ui,
                    self.selection_manager,
                    self.style,
                    self.undo_redo_manager,
                );
            }
            Tab::GraphicEditor => {
                property_editor::property_editor(
                    ui,
                    self.selection_manager,
                    self.style,
                    self.reference_store,
                    self.undo_redo_manager,
                    self.game_adapter,
                    self.graphic_states,
                );
            }
            Tab::GraphicItemEditor => {
                element_editor::element_editor(
                    ui,
                    self.selection_manager,
                    self.style,
                    self.reference_store,
                    self.undo_redo_manager,
                    self.graphic_states,
                );
            }
            Tab::UndoRedo => {
                undo_redo::undo_redo(ui, &mut self.undo_redo_manager);
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
