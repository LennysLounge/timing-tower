use std::collections::HashMap;

use backend::style::{
    graphic::{graphic_items::GraphicItemId, GraphicStateId},
    StyleId,
};

#[derive(Default)]
pub struct SelectionManager {
    states: HashMap<StyleId, SelectionState>,
    selected: Option<StyleId>,
}
impl SelectionManager {
    pub fn set_selected(&mut self, style_id: StyleId) {
        self.selected = Some(style_id);
        self.states.entry(style_id).or_default();
    }
    pub fn selected_state(&self) -> Option<&SelectionState> {
        self.selected.as_ref().and_then(|id| self.states.get(id))
    }
    pub fn selected_state_mut(&mut self) -> Option<&mut SelectionState> {
        self.selected
            .as_ref()
            .and_then(|id| self.states.get_mut(id))
    }
    pub fn selected(&self) -> Option<StyleId> {
        self.selected
    }
}

#[derive(Default)]
pub struct SelectionState {
    pub graphic_item: Option<GraphicItemId>,
    pub graphic_state: Option<GraphicStateId>,
}
