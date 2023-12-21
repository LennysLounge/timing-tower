mod property_editor;
mod tree_view;
mod undo_redo;

use backend::style::StyleDefinition;
use bevy_egui::egui::{self, Rect};
use egui_dock::TabViewer;
use uuid::Uuid;

use crate::{command::UndoRedoManager, editor::reference_store::ReferenceStore};

pub enum Tab {
    SceneView,
    Elements,
    Variables,
    Assets,
    PropertyEditor,
    UndoRedo,
}

pub struct EditorTabViewer<'a> {
    pub viewport: &'a mut Rect,
    pub selected_node: &'a mut Option<Uuid>,
    pub style: &'a mut StyleDefinition,
    pub reference_store: &'a ReferenceStore,
    pub undo_redo_manager: &'a mut UndoRedoManager,
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
                tree_view::tree_view(
                    ui,
                    self.selected_node,
                    &mut self.style.scene,
                    self.undo_redo_manager,
                );
            }
            Tab::PropertyEditor => {
                property_editor::property_editor(
                    ui,
                    self.selected_node,
                    self.style,
                    self.reference_store,
                    self.undo_redo_manager,
                );
            }
            Tab::Variables => {
                tree_view::tree_view(
                    ui,
                    self.selected_node,
                    &mut self.style.vars,
                    self.undo_redo_manager,
                );
            }
            Tab::Assets => {
                tree_view::tree_view(
                    ui,
                    self.selected_node,
                    &mut self.style.assets,
                    self.undo_redo_manager,
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
