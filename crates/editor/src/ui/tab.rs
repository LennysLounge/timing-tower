mod element_editor;
mod property_editor;
mod tree_view;
mod undo_redo;

use backend::{
    exact_variant::ExactVariant,
    style::{StyleDefinition, StyleItem},
};
use bevy_egui::egui::{self, Rect};
use egui_dock::TabViewer;
use unified_sim_model::Adapter;
use uuid::Uuid;

use crate::{command::UndoRedoManager, reference_store::ReferenceStore};

pub enum Tab {
    SceneView,
    Elements,
    Variables,
    Assets,
    ComponentEditor,
    ElementEditor,
    UndoRedo,
}

pub struct EditorTabViewer<'a> {
    pub viewport: &'a mut Rect,
    pub style_item_selection: &'a mut Option<Uuid>,
    pub graphic_item_selection: &'a mut Option<Uuid>,
    pub graphic_state_selection: &'a mut Option<Uuid>,
    pub style: &'a mut ExactVariant<StyleItem, StyleDefinition>,
    pub reference_store: &'a ReferenceStore,
    pub undo_redo_manager: &'a mut UndoRedoManager,
    pub game_adapter: &'a Adapter,
}
impl<'a> TabViewer for EditorTabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::SceneView => "Scene view".into(),
            Tab::Elements => "Elements".into(),
            Tab::ComponentEditor => "Component".into(),
            Tab::ElementEditor => "Element".into(),
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
                    self.style_item_selection,
                    self.graphic_item_selection,
                    self.style,
                    self.undo_redo_manager,
                );
            }
            Tab::ComponentEditor => {
                property_editor::property_editor(
                    ui,
                    self.style_item_selection,
                    self.graphic_item_selection,
                    self.graphic_state_selection,
                    self.style,
                    self.reference_store,
                    self.undo_redo_manager,
                    self.game_adapter,
                );
            }
            Tab::ElementEditor => {
                element_editor::element_editor(
                    ui,
                    self.style_item_selection,
                    self.graphic_item_selection,
                    self.graphic_state_selection,
                    self.style,
                    self.reference_store,
                    self.undo_redo_manager,
                    self.game_adapter,
                );
            }
            Tab::Variables => {
                // tree_view::tree_view(
                //     ui,
                //     self.selected_node,
                //     self.secondary_selection,
                //     &mut self.style.vars,
                //     self.undo_redo_manager,
                // );
            }
            Tab::Assets => {
                // tree_view::tree_view(
                //     ui,
                //     self.selected_node,
                //     self.secondary_selection,
                //     &mut self.style.assets,
                //     self.undo_redo_manager,
                // );
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
